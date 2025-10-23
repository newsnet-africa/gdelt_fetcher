//! High-level GDELT data fetcher with time-based queries
//!
//! This module provides the main API for fetching GDELT data with support for:
//! - Latest data (automatically rounded to nearest 15-minute interval)
//! - Specific timestamp queries
//! - Full day queries (all 15-minute intervals)
//! - Date range queries
//! - Optional persistent storage via netabase
//! - Configurable storage modes (in-memory, tmp directory, or custom path)

use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use models::types::{
    event_table::EventTable,
    gkg_table::GKGTable,
    masterlist::{EventTableDay, GkgTableDay, MasterlistEntry, MentionTableDay, TableType},
    mention_table::MentionTable,
};

#[cfg(not(feature = "netabase"))]
use crate::masterlist_manager::MasterlistManager;

use crate::data_processor;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

/// Storage configuration for downloaded data
#[derive(Debug, Clone)]
pub enum StorageMode {
    /// Process data entirely in memory without saving to disk (default, WASM-compatible)
    InMemory,
    /// Save CSV files to system tmp directory (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    TmpDirectory,
    /// Save CSV files to a custom path (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    CustomPath(PathBuf),
}

impl Default for StorageMode {
    fn default() -> Self {
        StorageMode::InMemory
    }
}

/// Main GDELT fetcher with time-based query capabilities
pub struct GdeltFetcher {
    #[cfg(not(feature = "netabase"))]
    masterlist: MasterlistManager,
    auto_update: bool,
    storage_mode: StorageMode,
}

impl GdeltFetcher {
    /// Create a new fetcher with in-memory processing (default, WASM-compatible)
    pub fn new() -> Self {
        Self {
            #[cfg(not(feature = "netabase"))]
            masterlist: MasterlistManager::new_in_memory(),
            auto_update: true,
            storage_mode: StorageMode::InMemory,
        }
    }

    /// Create a new fetcher with custom storage mode (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_storage_mode(storage_mode: StorageMode) -> Self {
        Self {
            masterlist: MasterlistManager::new_in_memory(),
            auto_update: true,
            storage_mode,
        }
    }

    /// Create a new fetcher that saves CSV files to tmp directory (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_tmp_storage() -> Self {
        Self::with_storage_mode(StorageMode::TmpDirectory)
    }

    /// Create a new fetcher that saves CSV files to a custom path (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_custom_path<P: Into<PathBuf>>(path: P) -> Self {
        Self::with_storage_mode(StorageMode::CustomPath(path.into()))
    }

    /// Create a new fetcher with persistent masterlist storage at the specified path (netabase feature required)
    #[cfg(feature = "netabase")]
    pub fn with_masterlist_storage(storage_path: &str) -> Result<Self> {
        Ok(Self {
            masterlist: MasterlistManager::with_storage(storage_path)?,
            auto_update: true,
            storage_mode: StorageMode::InMemory,
        })
    }

    /// Set the storage mode for downloaded CSV data (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_storage_mode(&mut self, storage_mode: StorageMode) {
        self.storage_mode = storage_mode;
    }

    /// Get the current storage mode
    pub fn storage_mode(&self) -> &StorageMode {
        &self.storage_mode
    }

    /// Initialize the fetcher (downloads master file lists)
    pub async fn initialize(&mut self) -> Result<()> {
        self.masterlist.initialize().await
    }

    /// Update the masterlist with latest entries from lastupdate.txt
    pub async fn update_latest(&mut self) -> Result<usize> {
        self.masterlist.update_latest().await
    }

    /// Fetch the latest data for a table type (rounded to nearest 15-minute interval)
    pub async fn fetch_latest_events(&mut self) -> Result<Vec<EventTable>> {
        if self.auto_update {
            self.update_latest().await?;
        }

        let entry = self
            .masterlist
            .find_latest(TableType::Export, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No latest export data found"))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let save_path = self.get_save_path(&entry, "events");
            let result =
                data_processor::fetch_and_parse_events_with_save(&entry.url, save_path.as_deref())
                    .await?;
            return Ok(result.data);
        }

        #[cfg(target_arch = "wasm32")]
        {
            data_processor::fetch_and_parse_events(&entry.url).await
        }
    }

    /// Fetch the latest mentions
    pub async fn fetch_latest_mentions(&mut self) -> Result<Vec<MentionTable>> {
        if self.auto_update {
            self.update_latest().await?;
        }

        let entry = self
            .masterlist
            .find_latest(TableType::Mentions, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No latest mentions data found"))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let save_path = self.get_save_path(&entry, "mentions");
            let result = data_processor::fetch_and_parse_mentions_with_save(
                &entry.url,
                save_path.as_deref(),
            )
            .await?;
            return Ok(result.data);
        }

        #[cfg(target_arch = "wasm32")]
        {
            data_processor::fetch_and_parse_mentions(&entry.url).await
        }
    }

    /// Fetch the latest GKG data
    pub async fn fetch_latest_gkg(&mut self) -> Result<Vec<GKGTable>> {
        if self.auto_update {
            self.update_latest().await?;
        }

        let entry = self
            .masterlist
            .find_latest(TableType::Gkg, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No latest GKG data found"))?;

        #[cfg(not(target_arch = "wasm32"))]
        {
            let save_path = self.get_save_path(&entry, "gkg");
            let result =
                data_processor::fetch_and_parse_gkg_with_save(&entry.url, save_path.as_deref())
                    .await?;
            return Ok(result.data);
        }

        #[cfg(target_arch = "wasm32")]
        {
            data_processor::fetch_and_parse_gkg(&entry.url).await
        }
    }

    /// Fetch events at a specific timestamp (must be on a 15-minute boundary)
    pub async fn fetch_events_at(&self, timestamp: NaiveDateTime) -> Result<Vec<EventTable>> {
        let rounded = MasterlistEntry::round_to_15min(timestamp);

        let entry = self
            .masterlist
            .find_by_timestamp(rounded, TableType::Export, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No export data found for timestamp: {}", rounded))?;

        data_processor::fetch_and_parse_events(&entry.url).await
    }

    /// Fetch mentions at a specific timestamp
    pub async fn fetch_mentions_at(&self, timestamp: NaiveDateTime) -> Result<Vec<MentionTable>> {
        let rounded = MasterlistEntry::round_to_15min(timestamp);

        let entry = self
            .masterlist
            .find_by_timestamp(rounded, TableType::Mentions, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No mentions data found for timestamp: {}", rounded))?;

        data_processor::fetch_and_parse_mentions(&entry.url).await
    }

    /// Fetch GKG data at a specific timestamp
    pub async fn fetch_gkg_at(&self, timestamp: NaiveDateTime) -> Result<Vec<GKGTable>> {
        let rounded = MasterlistEntry::round_to_15min(timestamp);

        let entry = self
            .masterlist
            .find_by_timestamp(rounded, TableType::Gkg, false)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No GKG data found for timestamp: {}", rounded))?;

        data_processor::fetch_and_parse_gkg(&entry.url).await
    }

    /// Fetch all event data for a specific day (returns Vec of Vec, one per 15-min interval)
    pub async fn fetch_events_day(&self, date: NaiveDate) -> Result<EventTableDay> {
        let entries = self
            .masterlist
            .find_by_day(date, TableType::Export, false)
            .await?;

        let mut day_data = Vec::new();

        for entry in entries {
            match data_processor::fetch_and_parse_events(&entry.url).await {
                Ok(events) => day_data.push(events),
                Err(e) => {
                    log::warn!(
                        "Failed to fetch events for {}: {}",
                        entry.get_timestamp(),
                        e
                    );
                }
            }
        }

        Ok(day_data)
    }

    /// Fetch all mention data for a specific day
    pub async fn fetch_mentions_day(&self, date: NaiveDate) -> Result<MentionTableDay> {
        let entries = self
            .masterlist
            .find_by_day(date, TableType::Mentions, false)
            .await?;

        let mut day_data = Vec::new();

        for entry in entries {
            match data_processor::fetch_and_parse_mentions(&entry.url).await {
                Ok(mentions) => day_data.push(mentions),
                Err(e) => {
                    log::warn!(
                        "Failed to fetch mentions for {}: {}",
                        entry.get_timestamp(),
                        e
                    );
                }
            }
        }

        Ok(day_data)
    }

    /// Fetch all GKG data for a specific day
    pub async fn fetch_gkg_day(&self, date: NaiveDate) -> Result<GkgTableDay> {
        let entries = self
            .masterlist
            .find_by_day(date, TableType::Gkg, false)
            .await?;

        let mut day_data = Vec::new();

        for entry in entries {
            match data_processor::fetch_and_parse_gkg(&entry.url).await {
                Ok(gkg) => day_data.push(gkg),
                Err(e) => {
                    log::warn!("Failed to fetch GKG for {}: {}", entry.get_timestamp(), e);
                }
            }
        }

        Ok(day_data)
    }

    /// Fetch events for a date range
    pub async fn fetch_events_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<EventTableDay>> {
        let entries = self
            .masterlist
            .find_by_range(start_date, end_date, TableType::Export, false)
            .await?;

        // Group by day
        let mut range_data: Vec<EventTableDay> = Vec::new();
        let mut current_day: Option<NaiveDate> = None;
        let mut current_day_data: EventTableDay = Vec::new();

        for entry in entries {
            let entry_date = entry.get_timestamp().date();

            if current_day.is_none() || current_day.unwrap() != entry_date {
                if !current_day_data.is_empty() {
                    range_data.push(current_day_data);
                    current_day_data = Vec::new();
                }
                current_day = Some(entry_date);
            }

            match data_processor::fetch_and_parse_events(&entry.url).await {
                Ok(events) => current_day_data.push(events),
                Err(e) => {
                    log::warn!(
                        "Failed to fetch events for {}: {}",
                        entry.get_timestamp(),
                        e
                    );
                }
            }
        }

        // Push the last day
        if !current_day_data.is_empty() {
            range_data.push(current_day_data);
        }

        Ok(range_data)
    }

    /// Fetch mentions for a date range
    pub async fn fetch_mentions_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<MentionTableDay>> {
        let entries = self
            .masterlist
            .find_by_range(start_date, end_date, TableType::Mentions, false)
            .await?;

        let mut range_data: Vec<MentionTableDay> = Vec::new();
        let mut current_day: Option<NaiveDate> = None;
        let mut current_day_data: MentionTableDay = Vec::new();

        for entry in entries {
            let entry_date = entry.get_timestamp().date();

            if current_day.is_none() || current_day.unwrap() != entry_date {
                if !current_day_data.is_empty() {
                    range_data.push(current_day_data);
                    current_day_data = Vec::new();
                }
                current_day = Some(entry_date);
            }

            match data_processor::fetch_and_parse_mentions(&entry.url).await {
                Ok(mentions) => current_day_data.push(mentions),
                Err(e) => {
                    log::warn!(
                        "Failed to fetch mentions for {}: {}",
                        entry.get_timestamp(),
                        e
                    );
                }
            }
        }

        if !current_day_data.is_empty() {
            range_data.push(current_day_data);
        }

        Ok(range_data)
    }

    /// Fetch GKG data for a date range
    pub async fn fetch_gkg_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<GkgTableDay>> {
        let entries = self
            .masterlist
            .find_by_range(start_date, end_date, TableType::Gkg, false)
            .await?;

        let mut range_data: Vec<GkgTableDay> = Vec::new();
        let mut current_day: Option<NaiveDate> = None;
        let mut current_day_data: GkgTableDay = Vec::new();

        for entry in entries {
            let entry_date = entry.get_timestamp().date();

            if current_day.is_none() || current_day.unwrap() != entry_date {
                if !current_day_data.is_empty() {
                    range_data.push(current_day_data);
                    current_day_data = Vec::new();
                }
                current_day = Some(entry_date);
            }

            match data_processor::fetch_and_parse_gkg(&entry.url).await {
                Ok(gkg) => current_day_data.push(gkg),
                Err(e) => {
                    log::warn!("Failed to fetch GKG for {}: {}", entry.get_timestamp(), e);
                }
            }
        }

        if !current_day_data.is_empty() {
            range_data.push(current_day_data);
        }

        Ok(range_data)
    }

    /// Get the count of stored masterlist entries
    pub async fn masterlist_count(&self) -> Result<usize> {
        self.masterlist.count().await
    }

    /// Set whether to automatically update the masterlist on latest queries
    pub fn set_auto_update(&mut self, auto_update: bool) {
        self.auto_update = auto_update;
    }

    /// Get the save path for a CSV file based on storage mode (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    fn get_save_path(&self, entry: &MasterlistEntry, data_type: &str) -> Option<PathBuf> {
        match &self.storage_mode {
            StorageMode::InMemory => None,
            StorageMode::TmpDirectory => {
                let filename = format!(
                    "{}_{}.csv",
                    data_type,
                    entry.get_timestamp().format("%Y%m%d%H%M%S")
                );
                Some(std::env::temp_dir().join("gdelt").join(filename))
            }
            StorageMode::CustomPath(base_path) => {
                let filename = format!(
                    "{}_{}.csv",
                    data_type,
                    entry.get_timestamp().format("%Y%m%d%H%M%S")
                );
                Some(base_path.join(filename))
            }
        }
    }
}

impl Default for GdeltFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_fetcher() {
        let fetcher = GdeltFetcher::new();
        assert!(fetcher.auto_update);
    }

    #[tokio::test]
    async fn test_round_to_15min() {
        let dt = NaiveDateTime::parse_from_str("2024-01-15 14:23:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let rounded = MasterlistEntry::round_to_15min(dt);
        assert_eq!(
            rounded.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2024-01-15 14:15:00"
        );
    }
}
