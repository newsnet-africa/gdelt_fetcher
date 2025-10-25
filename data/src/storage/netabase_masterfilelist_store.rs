///! Netabase-powered masterlist store
///
/// This implementation uses netabase_store's SledStore (native) or IndexedDBStore (WASM)
/// to provide cross-platform persistent storage with type-safety and secondary key indexing.

use anyhow::{Context, Result};
use chrono::{NaiveDate, NaiveDateTime};
use models::types::masterlist::{MasterlistEntry, TableType};

use super::masterfilelist_models::*;

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

// ============================================================================
// Native implementation using SledStore
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub struct NetabaseMasterfilelistStore {
    store: netabase_store::databases::sled_store::SledStore<MasterlistDefinition>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NetabaseMasterfilelistStore {
    /// Create a new store at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let store = netabase_store::databases::sled_store::SledStore::<MasterlistDefinition>::new(path)
            .context("Failed to open SledStore")?;
        Ok(Self { store })
    }

    /// Insert an entry
    pub fn insert(&self, entry: &MasterlistEntry) -> Result<()> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let netabase_entry: masterlist_schema::MasterlistEntry = entry.clone().into();
        tree.put(netabase_entry)
            .context("Failed to insert entry")?;
        Ok(())
    }

    /// Get an entry by its key
    pub fn get(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let id = MasterlistEntry::generate_id(&timestamp, table_type, is_translation);
        let key = MasterlistEntryPrimaryKey(id);

        match tree.get(key)? {
            Some(entry) => Ok(Some(entry.into())),
            None => Ok(None),
        }
    }

    /// Get all entries
    pub fn get_all(&self) -> Result<Vec<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let entries: Vec<_> = tree.iter()
            .map(|result| {
                let (_, entry) = result?;
                Ok(entry.into())
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(entries)
    }

    /// Get entries by table type (using secondary key index)
    pub fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let entries: Vec<_> = tree.get_by_secondary_key(
            MasterlistEntrySecondaryKeys::TableType(TableTypeSecondaryKey(table_type))
        )?
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
    ) -> Result<Option<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type)?;

        Ok(entries
            .into_iter()
            .filter(|e| e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp))
    }

    /// Get entries for a specific day
    pub fn get_by_day(&self, date: NaiveDate, table_type: TableType, is_translation: bool) -> Result<Vec<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type)?;

        Ok(entries
            .into_iter()
            .filter(|e| {
                let entry_date = e.get_timestamp().date();
                entry_date == date && e.is_translation == is_translation
            })
            .collect())
    }

    /// Get entries for a date range
    pub fn get_by_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Vec<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type)?;

        Ok(entries
            .into_iter()
            .filter(|e| {
                let entry_date = e.get_timestamp().date();
                entry_date >= start_date && entry_date <= end_date && e.is_translation == is_translation
            })
            .collect())
    }

    /// Count total entries
    pub fn count(&self) -> Result<usize> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
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
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        tree.clear()?;
        Ok(())
    }

    /// Update from parsed entries
    pub fn update_from_entries(&self, entries: &[MasterlistEntry]) -> Result<usize> {
        let mut added_count = 0;

        for entry in entries {
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
    store: netabase_store::databases::indexeddb_store::IndexedDBStore<MasterlistDefinition>,
}

#[cfg(target_arch = "wasm32")]
impl NetabaseMasterfilelistStore {
    /// Create a new store with IndexedDB backend
    pub async fn new(db_name: &str) -> Result<Self> {
        let store = netabase_store::databases::indexeddb_store::IndexedDBStore::<MasterlistDefinition>::new(db_name)
            .await
            .context("Failed to open IndexedDBStore")?;
        Ok(Self { store })
    }

    /// Insert an entry
    pub async fn insert(&self, entry: &MasterlistEntry) -> Result<()> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let netabase_entry: masterlist_schema::MasterlistEntry = entry.clone().into();
        tree.put(netabase_entry)
            .await
            .context("Failed to insert entry")?;
        Ok(())
    }

    /// Get an entry by its key
    pub async fn get(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let id = MasterlistEntry::generate_id(&timestamp, table_type, is_translation);
        let key = MasterlistEntryPrimaryKey(id);

        match tree.get(key).await? {
            Some(entry) => Ok(Some(entry.into())),
            None => Ok(None),
        }
    }

    /// Get all entries
    pub async fn get_all(&self) -> Result<Vec<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let entries: Vec<_> = tree.iter().await?
            .into_iter()
            .map(|(_, entry)| entry.into())
            .collect();
        Ok(entries)
    }

    /// Get entries by table type (using secondary key index)
    pub async fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<MasterlistEntry>> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        let entries: Vec<_> = tree.get_by_secondary_key(
            MasterlistEntrySecondaryKeys::TableType(TableTypeSecondaryKey(table_type))
        ).await?
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
    ) -> Result<Option<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type).await?;

        Ok(entries
            .into_iter()
            .filter(|e| e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp))
    }

    /// Get entries for a specific day
    pub async fn get_by_day(&self, date: NaiveDate, table_type: TableType, is_translation: bool) -> Result<Vec<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type).await?;

        Ok(entries
            .into_iter()
            .filter(|e| {
                let entry_date = e.get_timestamp().date();
                entry_date == date && e.is_translation == is_translation
            })
            .collect())
    }

    /// Get entries for a date range
    pub async fn get_by_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Vec<MasterlistEntry>> {
        let entries = self.get_by_table_type(table_type).await?;

        Ok(entries
            .into_iter()
            .filter(|e| {
                let entry_date = e.get_timestamp().date();
                entry_date >= start_date && entry_date <= end_date && e.is_translation == is_translation
            })
            .collect())
    }

    /// Count total entries
    pub async fn count(&self) -> Result<usize> {
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
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
        let tree = self.store.open_tree::<masterlist_schema::MasterlistEntry>();
        tree.clear().await?;
        Ok(())
    }

    /// Update from parsed entries
    pub async fn update_from_entries(&self, entries: &[MasterlistEntry]) -> Result<usize> {
        let mut added_count = 0;

        for entry in entries {
            let timestamp = entry.get_timestamp();
            if !self.exists(timestamp, entry.table_type, entry.is_translation).await? {
                self.insert(entry).await?;
                added_count += 1;
            }
        }

        Ok(added_count)
    }
}
