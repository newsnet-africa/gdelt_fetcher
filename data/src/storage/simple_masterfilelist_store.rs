///! Simplified masterfilelist store using sled directly (without netabase macros)
///
/// This is a simpler implementation that stores masterfilelist entries
/// using sled directly with bincode 2.0 serialization.

use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use models::types::masterfilelist::{Masterfilelist, MasterfilelistEntry, TableType};
use std::path::Path;

/// Simple key-value store for masterfilelist entries
pub struct SimpleMasterfilelistStore {
    db: sled::Db,
}

impl SimpleMasterfilelistStore {
    /// Create a new store at the given path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path.as_ref()).context("Failed to open sled database")?;
        Ok(Self { db })
    }

    /// Insert an entry
    pub fn insert(&self, entry: &MasterfilelistEntry) -> Result<()> {
        let key = entry.key();
        let value = bincode::encode_to_vec(entry, bincode::config::standard())
            .context("Failed to serialize entry")?;
        self.db.insert(key.as_bytes(), value)?;
        Ok(())
    }

    /// Insert all entries from a Masterfilelist
    pub fn insert_all(&self, masterlist: &Masterfilelist) -> Result<usize> {
        let mut count = 0;
        for entry in &masterlist.entries {
            self.insert(entry)?;
            count += 1;
        }
        self.db.flush()?;
        Ok(count)
    }

    /// Get an entry by its key
    pub fn get(&self, key: &str) -> Result<Option<MasterfilelistEntry>> {
        match self.db.get(key.as_bytes())? {
            Some(ivec) => {
                let (entry, _) = bincode::decode_from_slice::<MasterfilelistEntry, _>(
                    &ivec,
                    bincode::config::standard()
                ).context("Failed to deserialize entry")?;
                Ok(Some(entry))
            }
            None => Ok(None),
        }
    }

    /// Get entry by timestamp, table type, and translation flag
    pub fn get_by_params(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterfilelistEntry>> {
        // Generate key in the same format as entry.key()
        let key = format!(
            "{}_{:?}{}",
            timestamp.format("%Y%m%d%H%M%S"),
            table_type,
            if is_translation { "_trans" } else { "" }
        );
        self.get(&key)
    }

    /// Get all entries
    pub fn get_all(&self) -> Result<Vec<MasterfilelistEntry>> {
        let mut entries = Vec::new();

        for item in self.db.iter() {
            let (_key, value) = item?;
            match bincode::decode_from_slice::<MasterfilelistEntry, _>(&value, bincode::config::standard()) {
                Ok((entry, _)) => entries.push(entry),
                Err(e) => {
                    eprintln!("Warning: Failed to deserialize entry: {}", e);
                }
            }
        }

        Ok(entries)
    }

    /// Get entries by table type
    pub fn get_by_table_type(&self, table_type: TableType) -> Result<Vec<MasterfilelistEntry>> {
        let all_entries = self.get_all()?;
        Ok(all_entries
            .into_iter()
            .filter(|e| e.table_type == table_type)
            .collect())
    }

    /// Get the latest entry for a specific table type
    pub fn get_latest(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<Option<MasterfilelistEntry>> {
        let entries = self.get_by_table_type(table_type)?;

        Ok(entries
            .into_iter()
            .filter(|e| e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp))
    }

    /// Count total entries
    pub fn count(&self) -> Result<usize> {
        Ok(self.db.len())
    }

    /// Get all unique timestamps
    pub fn all_timestamps(&self) -> Result<Vec<NaiveDateTime>> {
        let entries = self.get_all()?;
        let mut timestamps: Vec<_> = entries.iter().map(|e: &MasterfilelistEntry| e.timestamp).collect();
        timestamps.sort();
        timestamps.dedup();
        Ok(timestamps)
    }

    /// Check if an entry exists by key
    pub fn exists(&self, key: &str) -> Result<bool> {
        Ok(self.db.contains_key(key.as_bytes())?)
    }

    /// Check if an entry exists by params
    pub fn exists_by_params(
        &self,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<bool> {
        let key = format!(
            "{}_{:?}{}",
            timestamp.format("%Y%m%d%H%M%S"),
            table_type,
            if is_translation { "_trans" } else { "" }
        );
        self.exists(&key)
    }

    /// Clear all entries
    pub fn clear(&self) -> Result<()> {
        self.db.clear()?;
        Ok(())
    }

    /// Update from text content
    pub fn update_from_content(&self, content: &str, version: &str) -> Result<usize> {
        let masterfilelist = Masterfilelist::from_content(content, version.to_string())
            .map_err(|e| anyhow::anyhow!("Failed to parse masterfilelist: {}", e))?;

        let mut added_count = 0;

        for entry in &masterfilelist.entries {
            let key = entry.key();
            if !self.exists(&key)? {
                self.insert(entry)?;
                added_count += 1;
            }
        }

        self.db.flush()?;
        Ok(added_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_simple_store() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let store = SimpleMasterfilelistStore::new(temp_dir.path())?;

        let content = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
9555639 b02920524f0b48c07bdab6c6d354a789 http://data.gdeltproject.org/gdeltv2/20150219004500.gkg.csv.zip
225092 6b4e1d0421548dbba59754d0f164d2a1 http://data.gdeltproject.org/gdeltv2/20150219010000.export.CSV.zip
"#;

        let added = store.update_from_content(content, "v2")?;
        assert_eq!(added, 3);

        let all_entries = store.get_all()?;
        assert_eq!(all_entries.len(), 3);

        Ok(())
    }

    #[test]
    fn test_get_latest() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let store = SimpleMasterfilelistStore::new(temp_dir.path())?;

        let content = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
286852 275a862fe0b27cdd3c3eabe2d05a264d http://data.gdeltproject.org/gdeltv2/20150219010000.mentions.CSV.zip
268121 c9a62b0fdf05e4ae79a1ad1d9824af12 http://data.gdeltproject.org/gdeltv2/20150219011500.mentions.CSV.zip
"#;

        store.update_from_content(content, "v2")?;

        let latest = store.get_latest(TableType::Mentions, false)?;
        assert!(latest.is_some());

        let latest = latest.unwrap();
        assert_eq!(
            latest.timestamp.format("%Y%m%d%H%M%S").to_string(),
            "20150219011500"
        );

        Ok(())
    }
}
