///! Netabase models for masterlist storage
///
/// Defines MasterlistEntry as a NetabaseModel that works with both
/// SledStore (native) and RedbStore/IndexedDBStore
///
/// This re-exports the MasterlistEntry from models::types::masterlist
/// as a netabase model for database storage.

use models::types::masterlist::{MasterlistEntry, TableType};

#[cfg(feature = "netabase")]
use netabase_store::netabase_definition_module;

// Define the netabase module with definitions and keys
#[cfg(feature = "netabase")]
#[netabase_definition_module(MasterlistDefinition, MasterlistKeys)]
pub mod masterlist_schema {
    use super::*;
    use netabase_macros::{netabase, NetabaseModel};

    /// Masterlist entry stored in the database
    ///
    /// This model is compatible with both native (sled/redb) and WASM (IndexedDB) storage
    /// It matches the structure from models::types::masterlist::MasterlistEntry
    #[derive(
        NetabaseModel,
        Clone,
        Debug,
        PartialEq,
        Eq,
        bincode::Encode,
        bincode::Decode,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[netabase(MasterlistDefinition)]
    pub struct MasterlistEntry {
        /// Composite primary key: timestamp + table_type + is_translation
        #[primary_key]
        pub id: String,

        /// File size in bytes (0 for GEG files which don't include size)
        pub size: u64,

        /// MD5 hash of the file (empty for GEG files)
        pub hash: String,

        /// URL to download the file
        pub url: String,

        /// Timestamp extracted from the filename (Unix timestamp)
        #[secondary_key]
        pub timestamp: i64,

        /// Table type (export, mentions, gkg, geg)
        #[secondary_key]
        pub table_type: TableType,

        /// Whether this is a translation file
        pub is_translation: bool,

        /// GDELT version (v2, v3_geg)
        pub version: String,
    }
}

#[cfg(feature = "netabase")]
pub use masterlist_schema::*;

// Conversion functions between the models types
#[cfg(feature = "netabase")]
impl From<MasterlistEntry> for masterlist_schema::MasterlistEntry {
    fn from(entry: MasterlistEntry) -> Self {
        Self {
            id: entry.id,
            size: entry.size,
            hash: entry.hash,
            url: entry.url,
            timestamp: entry.timestamp,
            table_type: entry.table_type,
            is_translation: entry.is_translation,
            version: entry.version,
        }
    }
}

#[cfg(feature = "netabase")]
impl From<masterlist_schema::MasterlistEntry> for MasterlistEntry {
    fn from(entry: masterlist_schema::MasterlistEntry) -> Self {
        Self {
            id: entry.id,
            size: entry.size,
            hash: entry.hash,
            url: entry.url,
            timestamp: entry.timestamp,
            table_type: entry.table_type,
            is_translation: entry.is_translation,
            version: entry.version,
        }
    }
}
