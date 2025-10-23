//! Masterlist manager with netabase storage
//!
//! This module provides a manager for GDELT master file lists with persistent storage
//! using netabase, supporting both native and WASM environments.

use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use models::types::masterlist::{MasterlistEntry, TableType};

#[cfg(feature = "netabase")]
use netabase_store::{
    backends::Backend,
    model::NetabaseModelTrait,
    sled_backend::SledBackend,
    tree::NetabaseSledTree,
};

use crate::fetchers::gdelt::http_fetcher;

/// URLs for GDELT master file lists
const GDELT_V2_MASTERLIST: &str = "http://data.gdeltproject.org/gdeltv2/masterfilelist.txt";
const GDELT_V2_LASTUPDATE: &str = "http://data.gdeltproject.org/gdeltv2/lastupdate.txt";
const GDELT_V3_GEG_MASTERLIST: &str =
    "http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/MASTERFILELIST.TXT";

/// Manager for GDELT master file lists with persistence
pub struct MasterlistManager {
    #[cfg(feature = "netabase")]
    tree: Option<NetabaseSledTree<MasterlistEntry, models::types::masterlist::MasterlistEntryKey>>,

    #[cfg(not(feature = "netabase"))]
    in_memory_cache: std::collections::HashMap<String, MasterlistEntry>,

    v2_initialized: bool,
    v3_initialized: bool,
}

impl MasterlistManager {
    /// Create a new masterlist manager without persistence
    pub fn new_in_memory() -> Self {
        Self {
            #[cfg(feature = "netabase")]
            tree: None,
            #[cfg(not(feature = "netabase"))]
            in_memory_cache: std::collections::HashMap::new(),
            v2_initialized: false,
            v3_initialized: false,
        }
    }

    /// Create a new masterlist manager with netabase persistence
    #[cfg(feature = "netabase")]
    pub fn with_storage(storage_path: &str) -> Result<Self> {
        let backend = SledBackend::new(storage_path)
            .with_context(|| format!("Failed to create storage backend at: {}", storage_path))?;

        let tree = backend
            .get_tree("gdelt_masterlist")
            .with_context(|| "Failed to open masterlist tree")?;

        Ok(Self {
            tree: Some(tree),
            v2_initialized: false,
            v3_initialized: false,
        })
    }

    /// Initialize the masterlist from GDELT v2 and v3
    /// This should be called once on first use or periodically to refresh
    pub async fn initialize(&mut self) -> Result<()> {
        // Initialize v2 masterlist (this is large, so only do it if needed)
        if !self.v2_initialized {
            log::info!("Fetching GDELT v2 masterlist...");
            let content = http_fetcher::fetch_text(GDELT_V2_MASTERLIST).await?;
            self.parse_and_store(&content, "v2").await?;
            self.v2_initialized = true;
            log::info!("GDELT v2 masterlist initialized");
        }

        // Initialize v3 GEG masterlist
        if !self.v3_initialized {
            log::info!("Fetching GDELT v3 GEG masterlist...");
            let content = http_fetcher::fetch_text(GDELT_V3_GEG_MASTERLIST).await?;
            self.parse_and_store(&content, "v3_geg").await?;
            self.v3_initialized = true;
            log::info!("GDELT v3 GEG masterlist initialized");
        }

        Ok(())
    }

    /// Update with latest entries from lastupdate.txt (v2 only)
    pub async fn update_latest(&mut self) -> Result<usize> {
        log::info!("Fetching latest updates from lastupdate.txt...");
        let content = http_fetcher::fetch_text(GDELT_V2_LASTUPDATE).await?;
        let added = self.parse_and_store(&content, "v2").await?;
        log::info!("Added {} new entries from lastupdate.txt", added);
        Ok(added)
    }

    /// Parse content and store entries
    async fn parse_and_store(&mut self, content: &str, version: &str) -> Result<usize> {
        let mut added_count = 0;

        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match MasterlistEntry::from_line(line, version) {
                Ok(entry) => {
                    if self.insert_entry(entry).await? {
                        added_count += 1;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to parse line {}: {} - Error: {}", i + 1, line, e);
                }
            }
        }

        Ok(added_count)
    }

    /// Insert an entry into storage (returns true if new entry was added)
    async fn insert_entry(&mut self, entry: MasterlistEntry) -> Result<bool> {
        #[cfg(feature = "netabase")]
        {
            if let Some(tree) = &self.tree {
                let key = entry.key();
                // Check if entry already exists
                if tree.get(&key)?.is_none() {
                    tree.insert(entry.key(), entry)?;
                    return Ok(true);
                }
                return Ok(false);
            }
        }

        #[cfg(not(feature = "netabase"))]
        {
            let existed = self.in_memory_cache.insert(entry.id.clone(), entry).is_some();
            return Ok(!existed);
        }

        #[cfg(feature = "netabase")]
        Ok(false)
    }

    /// Find the latest entry for a specific table type
    pub async fn find_latest(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterlistEntry>> {
        let now = Utc::now().naive_utc();
        let rounded = MasterlistEntry::round_to_15min(now);

        // Search backward from current time
        for minutes_back in (0..=60).step_by(15) {
            let target_time = rounded - chrono::Duration::minutes(minutes_back);
            if let Some(entry) = self.find_by_timestamp(target_time, table_type, is_translation).await? {
                return Ok(Some(entry));
            }
        }

        Ok(None)
    }

    /// Find an entry by timestamp
    pub async fn find_by_timestamp(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterlistEntry>> {
        let id = MasterlistEntry::generate_id(&timestamp, table_type, is_translation);

        #[cfg(feature = "netabase")]
        {
            if let Some(tree) = &self.tree {
                return tree.get(&id);
            }
        }

        #[cfg(not(feature = "netabase"))]
        {
            return Ok(self.in_memory_cache.get(&id).cloned());
        }

        #[cfg(feature = "netabase")]
        Ok(None)
    }

    /// Find all entries for a specific day
    pub async fn find_by_day(
        &self,
        date: NaiveDate,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Vec<MasterlistEntry>> {
        let mut entries = Vec::new();
        let start_time = date.and_hms_opt(0, 0, 0).unwrap();

        // GDELT updates every 15 minutes, so 96 intervals per day
        for interval in 0..96 {
            let timestamp = start_time + chrono::Duration::minutes(interval * 15);
            if let Some(entry) = self.find_by_timestamp(timestamp, table_type, is_translation).await? {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    /// Find all entries within a date range
    pub async fn find_by_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Vec<MasterlistEntry>> {
        let mut entries = Vec::new();
        let mut current_date = start_date;

        while current_date <= end_date {
            let day_entries = self.find_by_day(current_date, table_type, is_translation).await?;
            entries.extend(day_entries);
            current_date = current_date.succ_opt().unwrap();
        }

        Ok(entries)
    }

    /// Get the total number of entries stored
    pub async fn count(&self) -> Result<usize> {
        #[cfg(feature = "netabase")]
        {
            if let Some(tree) = &self.tree {
                return Ok(tree.len());
            }
        }

        #[cfg(not(feature = "netabase"))]
        {
            return Ok(self.in_memory_cache.len());
        }

        #[cfg(feature = "netabase")]
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_manager() {
        let mut manager = MasterlistManager::new_in_memory();

        let entry = MasterlistEntry::from_line(
            "277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip",
            "v2"
        ).unwrap();

        let was_new = manager.insert_entry(entry.clone()).await.unwrap();
        assert!(was_new);

        let was_new_again = manager.insert_entry(entry.clone()).await.unwrap();
        assert!(!was_new_again);

        let count = manager.count().await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_find_by_timestamp() {
        let mut manager = MasterlistManager::new_in_memory();

        let entry = MasterlistEntry::from_line(
            "277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip",
            "v2"
        ).unwrap();

        manager.insert_entry(entry.clone()).await.unwrap();

        let timestamp = entry.get_timestamp();
        let found = manager.find_by_timestamp(timestamp, TableType::Mentions, false)
            .await
            .unwrap();

        assert!(found.is_some());
        assert_eq!(found.unwrap().url, entry.url);
    }
}
