use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use log::{info, warn};
use models::types::masterfilelist::{Masterfilelist, MasterfilelistEntry, TableType};
use std::path::Path;

use crate::storage::MasterfilelistStore;

/// GDELT version for masterfilelist fetching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdeltVersionForMasterfilelist {
    V2,
    V3Geg,
}

impl GdeltVersionForMasterfilelist {
    /// Get the masterfilelist URL for this version
    pub fn masterfilelist_url(&self) -> &'static str {
        match self {
            GdeltVersionForMasterfilelist::V2 => "http://data.gdeltproject.org/gdeltv2/masterfilelist.txt",
            GdeltVersionForMasterfilelist::V3Geg => "http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/MASTERFILELIST.TXT",
        }
    }

    /// Get the lastupdate URL for this version
    pub fn lastupdate_url(&self) -> &'static str {
        match self {
            GdeltVersionForMasterfilelist::V2 => "http://data.gdeltproject.org/gdeltv2/lastupdate.txt",
            // GEG doesn't have a lastupdate.txt, use the same masterfilelist
            GdeltVersionForMasterfilelist::V3Geg => "http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/MASTERFILELIST.TXT",
        }
    }

    /// Get the version string
    pub fn version_str(&self) -> &'static str {
        match self {
            GdeltVersionForMasterfilelist::V2 => "v2",
            GdeltVersionForMasterfilelist::V3Geg => "v3_geg",
        }
    }
}

/// Fetcher for GDELT masterfilelist with persistent storage
pub struct MasterfilelistFetcher {
    version: GdeltVersionForMasterfilelist,
    store: MasterfilelistStore,
    client: reqwest::Client,
}

impl MasterfilelistFetcher {
    /// Create a new masterfilelist fetcher
    pub fn new<P: AsRef<Path>>(
        store_path: P,
        version: GdeltVersionForMasterfilelist,
    ) -> Result<Self> {
        let store = MasterfilelistStore::new(store_path)
            .context("Failed to create masterfilelist store")?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            version,
            store,
            client,
        })
    }

    /// Fetch the full masterfilelist from GDELT
    pub async fn fetch_masterfilelist(&self) -> Result<Masterfilelist> {
        let url = self.version.masterfilelist_url();
        info!("Fetching masterfilelist from: {}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch masterfilelist")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch masterfilelist: HTTP {}", response.status());
        }

        let content = response
            .text()
            .await
            .context("Failed to read masterfilelist response")?;

        Masterfilelist::from_content(&content, self.version.version_str().to_string())
            .map_err(|e| anyhow::anyhow!("Failed to parse masterfilelist: {}", e))
    }

    /// Fetch the latest updates from lastupdate.txt
    pub async fn fetch_latest_updates(&self) -> Result<Masterfilelist> {
        let url = self.version.lastupdate_url();
        info!("Fetching latest updates from: {}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to fetch latest updates")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch latest updates: HTTP {}", response.status());
        }

        let content = response
            .text()
            .await
            .context("Failed to read latest updates response")?;

        Masterfilelist::from_content(&content, self.version.version_str().to_string())
            .map_err(|e| anyhow::anyhow!("Failed to parse latest updates: {}", e))
    }

    /// Initialize the store with the full masterfilelist (if empty)
    pub async fn initialize_store(&self) -> Result<usize> {
        let count = self.store.count()?;
        if count > 0 {
            info!("Store already initialized with {} entries", count);
            return Ok(count);
        }

        info!("Initializing store with full masterfilelist");
        let masterfilelist = self.fetch_masterfilelist().await?;

        let added = self.store.insert_all(&masterfilelist)?;
        info!("Initialized store with {} entries", added);

        Ok(added)
    }

    /// Update the store with the latest entries
    pub async fn update_store(&self) -> Result<usize> {
        info!("Updating store with latest entries");

        let latest = self.fetch_latest_updates().await?;
        let mut added_count = 0;

        for entry in &latest.entries {
            if !self
                .store
                .exists_by_params(entry.timestamp, entry.table_type, entry.is_translation)?
            {
                self.store.insert(entry)?;
                added_count += 1;
            }
        }

        info!("Added {} new entries to store", added_count);
        Ok(added_count)
    }

    /// Get an entry from the store
    pub fn get_entry(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterfilelistEntry>> {
        self.store.get_by_params(timestamp, table_type, is_translation)
    }

    /// Get the latest entry for a specific table type
    pub fn get_latest(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterfilelistEntry>> {
        self.store.get_latest(table_type, is_translation)
    }

    /// Get all entries for a specific table type
    pub fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<MasterfilelistEntry>> {
        self.store.get_by_table_type(table_type)
    }

    /// Get all entries
    pub fn get_all(&self) -> Result<Vec<MasterfilelistEntry>> {
        self.store.get_all()
    }

    /// Get all unique timestamps
    pub fn all_timestamps(&self) -> Result<Vec<NaiveDateTime>> {
        self.store.all_timestamps()
    }

    /// Count the total number of entries in the store
    pub fn count(&self) -> Result<usize> {
        self.store.count()
    }

    /// Clear all entries from the store
    pub fn clear_store(&self) -> Result<()> {
        self.store.clear()
    }

    /// Fetch a specific file from a masterfilelist entry
    ///
    /// This downloads the file content into memory (WASM-compatible approach)
    pub async fn fetch_file(&self, entry: &MasterfilelistEntry) -> Result<Vec<u8>> {
        info!("Fetching file from: {}", entry.url);

        let response = self
            .client
            .get(&entry.url)
            .send()
            .await
            .context("Failed to fetch file")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to fetch file: HTTP {}", response.status());
        }

        let content = response
            .bytes()
            .await
            .context("Failed to read file response")?;

        // Only verify size and hash for v2 entries (GEG doesn't provide these)
        if entry.size > 0 && !entry.hash.is_empty() {
            // Verify file size
            if content.len() as u64 != entry.size {
                warn!(
                    "File size mismatch: expected {}, got {}",
                    entry.size,
                    content.len()
                );
            }

            // Verify hash
            let computed_hash = format!("{:x}", md5::compute(&content));
            if computed_hash != entry.hash {
                anyhow::bail!(
                    "Hash mismatch: expected {}, got {}",
                    entry.hash,
                    computed_hash
                );
            }
        } else {
            info!("Skipping size/hash verification (not available for this entry)");
        }

        Ok(content.to_vec())
    }

    /// Decompress a gzipped file in memory
    pub fn decompress_gzip(&self, compressed: &[u8]) -> Result<Vec<u8>> {
        use std::io::Read;

        let mut decoder = flate2::read::GzDecoder::new(compressed);
        let mut decompressed = Vec::new();
        decoder
            .read_to_end(&mut decompressed)
            .context("Failed to decompress gzip")?;

        Ok(decompressed)
    }

    /// Extract a ZIP file in memory and return the first file's contents
    pub fn extract_zip(&self, zip_data: &[u8]) -> Result<Vec<u8>> {
        use std::io::Read;

        let cursor = std::io::Cursor::new(zip_data);
        let mut archive = zip::ZipArchive::new(cursor).context("Failed to open ZIP archive")?;

        if archive.len() == 0 {
            anyhow::bail!("ZIP archive is empty");
        }

        // Get the first file
        let mut file = archive.by_index(0).context("Failed to read first file")?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .context("Failed to read file contents")?;

        Ok(contents)
    }

    /// Fetch and decompress a file (WASM-compatible)
    pub async fn fetch_and_decompress(&self, entry: &MasterfilelistEntry) -> Result<Vec<u8>> {
        let compressed = self.fetch_file(entry).await?;

        // Check file extension to determine decompression method
        if entry.url.ends_with(".zip") {
            self.extract_zip(&compressed)
        } else if entry.url.ends_with(".gz") {
            self.decompress_gzip(&compressed)
        } else {
            // No compression
            Ok(compressed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_fetch_masterfilelist() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher =
            MasterfilelistFetcher::new(temp_dir.path(), GdeltVersionForMasterfilelist::V2)?;

        // This test requires network access
        let masterfilelist = fetcher.fetch_masterfilelist().await?;
        assert!(!masterfilelist.entries.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_initialize_and_update() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher =
            MasterfilelistFetcher::new(temp_dir.path(), GdeltVersionForMasterfilelist::V2)?;

        // Initialize store
        let count = fetcher.initialize_store().await?;
        assert!(count > 0);

        // Update store
        let added = fetcher.update_store().await?;
        // May be 0 if no new entries
        assert!(added >= 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let fetcher =
            MasterfilelistFetcher::new(temp_dir.path(), GdeltVersionForMasterfilelist::V2)?;

        fetcher.initialize_store().await?;

        let latest = fetcher.get_latest(TableType::Export, false)?;
        assert!(latest.is_some());

        Ok(())
    }
}
