use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use log::info;
use models::types::geg_table::GEGTable;
use std::path::Path;

use super::masterfilelist_fetcher::{GdeltVersionForMasterfilelist, MasterfilelistFetcher};
use models::types::masterfilelist::TableType;

/// Fetcher for GDELT 3.0 GEG (GDELT Event Graph) data
///
/// GEG data is in newline-delimited JSON format and contains entity extraction
/// and sentiment analysis from Google's Cloud Natural Language API.
pub struct GEGFetcher {
    masterfilelist_fetcher: MasterfilelistFetcher,
}

impl GEGFetcher {
    /// Create a new GEG fetcher
    pub fn new<P: AsRef<Path>>(store_path: P) -> Result<Self> {
        let masterfilelist_fetcher =
            MasterfilelistFetcher::new(store_path, GdeltVersionForMasterfilelist::V3Geg)?;

        Ok(Self {
            masterfilelist_fetcher,
        })
    }

    /// Initialize the masterfilelist store (if needed)
    pub async fn initialize(&self) -> Result<usize> {
        self.masterfilelist_fetcher.initialize_store().await
    }

    /// Update the masterfilelist with the latest entries
    pub async fn update(&self) -> Result<usize> {
        self.masterfilelist_fetcher.update_store().await
    }

    /// Fetch the latest GEG data
    ///
    /// This returns all GEG entries from the latest available file.
    /// Note: GEG files use the GKG table type in the masterfilelist.
    pub async fn fetch_latest(&self) -> Result<Vec<GEGTable>> {
        // GEG files are stored as "gkg" type in the masterfilelist
        let entry = self
            .masterfilelist_fetcher
            .get_latest(TableType::Gkg, false)?
            .ok_or_else(|| anyhow::anyhow!("No GEG entry found in masterfilelist"))?;

        info!("Fetching GEG data from: {}", entry.url);

        // Fetch and decompress the file
        let data = self
            .masterfilelist_fetcher
            .fetch_and_decompress(&entry)
            .await?;

        // Parse JSON
        let content = String::from_utf8(data).context("Failed to convert to UTF-8")?;

        // Parse newline-delimited JSON
        GEGTable::from_ndjson(&content).map_err(|e| anyhow::anyhow!("Failed to parse GEG data: {}", e))
    }

    /// Fetch GEG data for a specific timestamp
    pub async fn fetch_by_timestamp(&self, timestamp: NaiveDateTime) -> Result<Vec<GEGTable>> {
        let entry = self
            .masterfilelist_fetcher
            .get_entry(timestamp, TableType::Gkg, false)?
            .ok_or_else(|| {
                anyhow::anyhow!("No GEG entry found for timestamp: {}", timestamp)
            })?;

        info!("Fetching GEG data from: {}", entry.url);

        // Fetch and decompress the file
        let data = self
            .masterfilelist_fetcher
            .fetch_and_decompress(&entry)
            .await?;

        // Parse JSON
        let content = String::from_utf8(data).context("Failed to convert to UTF-8")?;

        // Parse newline-delimited JSON
        GEGTable::from_ndjson(&content).map_err(|e| anyhow::anyhow!("Failed to parse GEG data: {}", e))
    }

    /// Get all available timestamps
    pub fn available_timestamps(&self) -> Result<Vec<NaiveDateTime>> {
        self.masterfilelist_fetcher.all_timestamps()
    }

    /// Get the count of available GEG files
    pub fn available_files_count(&self) -> Result<usize> {
        Ok(self
            .masterfilelist_fetcher
            .get_by_table_type(TableType::Gkg)?
            .len())
    }

    /// Fetch GEG data in a date range
    pub async fn fetch_by_date_range(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<GEGTable>> {
        let timestamps = self.available_timestamps()?;

        let mut all_entries = Vec::new();

        for timestamp in timestamps {
            if timestamp >= start && timestamp <= end {
                let entries = self.fetch_by_timestamp(timestamp).await?;
                all_entries.extend(entries);
            }
        }

        Ok(all_entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_geg_fetcher_initialize() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher = GEGFetcher::new(temp_dir.path())?;

        // Initialize the store
        let count = fetcher.initialize().await?;
        assert!(count > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_geg_fetcher_fetch_latest() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher = GEGFetcher::new(temp_dir.path())?;

        // Initialize and fetch
        fetcher.initialize().await?;
        let entries = fetcher.fetch_latest().await?;

        assert!(!entries.is_empty());

        // Print some stats
        for (i, entry) in entries.iter().take(5).enumerate() {
            println!("Entry {}: URL={}, entities={}", i + 1, entry.url, entry.entities.len());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_geg_fetcher_available_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher = GEGFetcher::new(temp_dir.path())?;

        fetcher.initialize().await?;

        let count = fetcher.available_files_count()?;
        assert!(count > 0);

        let timestamps = fetcher.available_timestamps()?;
        assert!(!timestamps.is_empty());

        Ok(())
    }
}
