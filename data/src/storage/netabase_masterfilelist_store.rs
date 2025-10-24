///! Netabase-powered masterfilelist store
///
/// This implementation uses netabase_store's SledStore (native) or IndexedDBStore (WASM)
/// to provide cross-platform persistent storage with type-safety and secondary key indexing.

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use models::types::masterfilelist::{Masterfilelist, TableType};

use super::masterfilelist_models::*;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

// ============================================================================
// Native implementation using SledStore
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub struct NetabaseMasterfilelistStore {
    store: netabase_store::databases::sled_store::SledStore<MasterfilelistDefinition>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NetabaseMasterfilelistStore {
    /// Create a new store at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let store = netabase_store::databases::sled_store::SledStore::<MasterfilelistDefinition>::new(path)
            .context("Failed to open SledStore")?;
        Ok(Self { store })
    }

    /// Insert an entry
    pub fn insert(&self, entry: &models::types::masterfilelist::MasterfilelistEntry) -> Result<()> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let netabase_entry: MasterfilelistEntry = entry.clone().into();
        tree.put(netabase_entry)
            .context("Failed to insert entry")?;
        Ok(())
    }

    /// Insert all entries from a Masterfilelist
    pub fn insert_all(&self, masterfilelist: &Masterfilelist) -> Result<usize> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let mut count = 0;
        for entry in &masterfilelist.entries {
            let netabase_entry: MasterfilelistEntry = entry.clone().into();
            tree.put(netabase_entry)?;
            count += 1;
        }
        Ok(count)
    }

    /// Get an entry by its key
    pub fn get(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let id = models::types::masterlist::MasterlistEntry::generate_id(&timestamp, table_type, is_translation);
        let key = MasterfilelistEntryPrimaryKey(id);

        match tree.get(key)? {
            Some(entry) => Ok(Some(entry.into())),
            None => Ok(None),
        }
    }

    /// Get all entries
    pub fn get_all(&self) -> Result<Vec<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let entries: Vec<_> = tree.iter()?
            .into_iter()
            .map(|(_, entry)| entry.into())
            .collect();
        Ok(entries)
    }

    /// Get entries by table type (using secondary key index)
    pub fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let entries: Vec<_> = tree.get_by_secondary_key(MasterfilelistEntrySecondaryKeys::TableTypeKey(table_type))?
            .into_iter()
            .map(|entry| entry.into())
            .collect();
        Ok(entries)
    }

    /// Get the latest entry for a specific table type
    pub fn get_latest(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<models::types::masterfilelist::MasterfilelistEntry>> {
        let entries = self.get_by_table_type(table_type)?;

        Ok(entries
            .into_iter()
            .filter(|e| e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp))
    }

    /// Count total entries
    pub fn count(&self) -> Result<usize> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        Ok(tree.len())
    }

    /// Get all unique timestamps
    pub fn all_timestamps(&self) -> Result<Vec<NaiveDateTime>> {
        let entries = self.get_all()?;
        let mut timestamps: Vec<_> = entries.iter().map(|e| e.get_timestamp()).collect();
        timestamps.sort();
        timestamps.dedup();
        Ok(timestamps)
    }

    /// Check if an entry exists
    pub fn exists(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<bool> {
        Ok(self.get(timestamp, table_type, is_translation)?.is_some())
    }

    /// Clear all entries
    pub fn clear(&self) -> Result<()> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        tree.clear()?;
        Ok(())
    }

    /// Update from text content
    pub fn update_from_content(&self, content: &str, version: &str) -> Result<usize> {
        let masterfilelist = Masterfilelist::from_content(content, version.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to parse masterfilelist: {}", e))?;

        let mut added_count = 0;

        for entry in &masterfilelist.entries {
            let timestamp = entry.get_timestamp();
            if !self.exists(timestamp, entry.table_type, entry.is_translation)? {
                self.insert(entry)?;
                added_count += 1;
            }
        }

        self.store.flush()?;
        Ok(added_count)
    }
}

// ============================================================================
// WASM implementation using IndexedDBStore
// ============================================================================

#[cfg(target_arch = "wasm32")]
pub struct NetabaseMasterfilelistStore {
    store: netabase_store::databases::indexeddb_store::IndexedDBStore<MasterfilelistDefinition>,
}

#[cfg(target_arch = "wasm32")]
impl NetabaseMasterfilelistStore {
    /// Create a new store with IndexedDB backend
    pub async fn new(db_name: &str) -> Result<Self> {
        let store = netabase_store::databases::indexeddb_store::IndexedDBStore::<MasterfilelistDefinition>::new(db_name)
            .await
            .context("Failed to open IndexedDBStore")?;
        Ok(Self { store })
    }

    /// Insert an entry
    pub async fn insert(&self, entry: &models::types::masterfilelist::MasterfilelistEntry) -> Result<()> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let netabase_entry: MasterfilelistEntry = entry.clone().into();
        tree.put(netabase_entry)
            .await
            .context("Failed to insert entry")?;
        Ok(())
    }

    /// Insert all entries from a Masterfilelist
    pub async fn insert_all(&self, masterfilelist: &Masterfilelist) -> Result<usize> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let mut count = 0;
        for entry in &masterfilelist.entries {
            let netabase_entry: MasterfilelistEntry = entry.clone().into();
            tree.put(netabase_entry).await?;
            count += 1;
        }
        Ok(count)
    }

    /// Get an entry by its key
    pub async fn get(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let id = models::types::masterlist::MasterlistEntry::generate_id(&timestamp, table_type, is_translation);
        let key = MasterfilelistEntryPrimaryKey(id);

        match tree.get(key).await? {
            Some(entry) => Ok(Some(entry.into())),
            None => Ok(None),
        }
    }

    /// Get all entries
    pub async fn get_all(&self) -> Result<Vec<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let entries: Vec<_> = tree.iter().await?
            .into_iter()
            .map(|(_, entry)| entry.into())
            .collect();
        Ok(entries)
    }

    /// Get entries by table type (using secondary key index)
    pub async fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<models::types::masterfilelist::MasterfilelistEntry>> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        let entries: Vec<_> = tree.get_by_secondary_key(MasterfilelistEntrySecondaryKeys::TableTypeKey(table_type)).await?
            .into_iter()
            .map(|entry| entry.into())
            .collect();
        Ok(entries)
    }

    /// Get the latest entry for a specific table type
    pub async fn get_latest(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<models::types::masterfilelist::MasterfilelistEntry>> {
        let entries = self.get_by_table_type(table_type).await?;

        Ok(entries
            .into_iter()
            .filter(|e| e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp))
    }

    /// Count total entries
    pub async fn count(&self) -> Result<usize> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        Ok(tree.len().await?)
    }

    /// Get all unique timestamps
    pub async fn all_timestamps(&self) -> Result<Vec<NaiveDateTime>> {
        let entries = self.get_all().await?;
        let mut timestamps: Vec<_> = entries.iter().map(|e| e.get_timestamp()).collect();
        timestamps.sort();
        timestamps.dedup();
        Ok(timestamps)
    }

    /// Check if an entry exists
    pub async fn exists(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<bool> {
        Ok(self.get(timestamp, table_type, is_translation).await?.is_some())
    }

    /// Clear all entries
    pub async fn clear(&self) -> Result<()> {
        let tree = self.store.open_tree::<MasterfilelistEntry>();
        tree.clear().await?;
        Ok(())
    }

    /// Update from text content
    pub async fn update_from_content(&self, content: &str, version: &str) -> Result<usize> {
        let masterfilelist = Masterfilelist::from_content(content, version.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to parse masterfilelist: {}", e))?;

        let mut added_count = 0;

        for entry in &masterfilelist.entries {
            let timestamp = entry.get_timestamp();
            if !self.exists(timestamp, entry.table_type, entry.is_translation).await? {
                self.insert(entry).await?;
                added_count += 1;
            }
        }

        Ok(added_count)
    }
}
