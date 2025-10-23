///! Key-Value Store abstraction for cross-platform storage
///
/// Provides a unified interface for both native (sled) and WASM (IndexedDB) storage backends.

use anyhow::Result;

/// Trait for key-value storage operations
///
/// This trait provides async operations to support IndexedDB on WASM,
/// while also being compatible with sled on native platforms.
#[async_trait::async_trait]
pub trait KvStore: Send + Sync {
    /// Insert a key-value pair
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<()>;

    /// Get a value by key
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Iterate over all key-value pairs
    async fn iter(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>>;

    /// Clear all entries
    async fn clear(&self) -> Result<()>;

    /// Get the number of entries
    async fn len(&self) -> Result<usize>;

    /// Check if the store is empty
    async fn is_empty(&self) -> Result<bool> {
        Ok(self.len().await? == 0)
    }

    /// Flush any pending writes (no-op for IndexedDB)
    async fn flush(&self) -> Result<()>;
}

// ============================================================================
// Native (sled) implementation
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
pub struct SledKvStore {
    db: sled::Db,
}

#[cfg(not(target_arch = "wasm32"))]
impl SledKvStore {
    pub fn new(db: sled::Db) -> Self {
        Self { db }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait::async_trait]
impl KvStore for SledKvStore {
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }

    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?.map(|ivec| ivec.to_vec()))
    }

    async fn iter(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        let mut results = Vec::new();
        for item in self.db.iter() {
            let (key, value) = item?;
            results.push((key.to_vec(), value.to_vec()));
        }
        Ok(results)
    }

    async fn clear(&self) -> Result<()> {
        self.db.clear()?;
        Ok(())
    }

    async fn len(&self) -> Result<usize> {
        Ok(self.db.len())
    }

    async fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}

// ============================================================================
// WASM (IndexedDB) implementation
// ============================================================================

#[cfg(all(target_arch = "wasm32", feature = "wasm-netabase"))]
pub struct IndexedDBKvStore {
    tree: netabase_store::databases::indexeddb_store::IndexedDBTree,
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-netabase"))]
impl IndexedDBKvStore {
    pub fn new(tree: netabase_store::databases::indexeddb_store::IndexedDBTree) -> Self {
        Self { tree }
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-netabase"))]
#[async_trait::async_trait(?Send)]
impl KvStore for IndexedDBKvStore {
    async fn insert(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.tree
            .insert(key, value)
            .await
            .map_err(|e| anyhow::anyhow!("IndexedDB insert failed: {}", e))
    }

    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        self.tree
            .get(key)
            .await
            .map_err(|e| anyhow::anyhow!("IndexedDB get failed: {}", e))
    }

    async fn iter(&self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        self.tree
            .iter()
            .await
            .map_err(|e| anyhow::anyhow!("IndexedDB iter failed: {}", e))
    }

    async fn clear(&self) -> Result<()> {
        self.tree
            .clear()
            .await
            .map_err(|e| anyhow::anyhow!("IndexedDB clear failed: {}", e))
    }

    async fn len(&self) -> Result<usize> {
        self.tree
            .len()
            .await
            .map_err(|e| anyhow::anyhow!("IndexedDB len failed: {}", e))
    }

    async fn flush(&self) -> Result<()> {
        // IndexedDB auto-flushes, so this is a no-op
        Ok(())
    }
}
