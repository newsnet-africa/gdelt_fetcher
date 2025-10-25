//! Master file list types with netabase integration
//!
//! This module defines the data structures for storing and querying the GDELT master file list
//! using netabase for persistent storage with IndexedDB support in WASM.

use chrono::{NaiveDateTime, Timelike};
use serde::{Deserialize, Serialize};

#[cfg(feature = "netabase")]
use netabase_store::NetabaseModel;

/// Table types in GDELT
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, bincode::Encode, bincode::Decode)]
pub enum TableType {
    Export,
    Mentions,
    Gkg,
    Geg, // GEG (Google Entity Graph) from v3
}

impl TableType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableType::Export => "export",
            TableType::Mentions => "mentions",
            TableType::Gkg => "gkg",
            TableType::Geg => "geg",
        }
    }

    pub fn from_url(url: &str) -> Option<Self> {
        if url.contains(".export.") {
            Some(TableType::Export)
        } else if url.contains(".mentions.") {
            Some(TableType::Mentions)
        } else if url.contains(".gkg.") {
            Some(TableType::Gkg)
        } else if url.contains("geg_gcnlapi") || url.contains(".geg-") {
            Some(TableType::Geg)
        } else {
            None
        }
    }
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Represents a single entry in the GDELT masterfilelist
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct MasterlistEntry {
    /// Composite primary key: timestamp + table_type + is_translation
    pub id: String,

    /// File size in bytes (0 for GEG files which don't include size)
    pub size: u64,

    /// MD5 hash of the file (empty for GEG files)
    pub hash: String,

    /// URL to download the file
    pub url: String,

    /// Timestamp extracted from the filename (rounded to nearest 15 minutes)
    pub timestamp: i64, // Unix timestamp for better indexing

    /// Table type (export, mentions, gkg, geg)
    pub table_type: TableType,

    /// Whether this is a translation file
    pub is_translation: bool,

    /// GDELT version (v2, v3_geg)
    pub version: String,
}

impl MasterlistEntry {
    /// Create a new masterlist entry
    pub fn new(
        size: u64,
        hash: String,
        url: String,
        timestamp: NaiveDateTime,
        table_type: TableType,
        is_translation: bool,
        version: String,
    ) -> Self {
        let id = Self::generate_id(&timestamp, table_type, is_translation);
        Self {
            id,
            size,
            hash,
            url,
            timestamp: timestamp.and_utc().timestamp(),
            table_type,
            is_translation,
            version,
        }
    }

    /// Generate a unique ID for a masterlist entry
    pub fn generate_id(timestamp: &NaiveDateTime, table_type: TableType, is_translation: bool) -> String {
        format!(
            "{}_{:?}{}",
            timestamp.format("%Y%m%d%H%M%S"),
            table_type,
            if is_translation { "_trans" } else { "" }
        )
    }

    /// Parse a masterlist entry from a line in the format:
    /// GDELT v2: <size> <hash> <url>
    /// GDELT v3 GEG: <url> (just the URL)
    pub fn from_line(line: &str, version: &str) -> Result<Self, String> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        let (size, hash, url) = match parts.len() {
            // GDELT v2 format: size hash url
            3 => {
                let size = parts[0]
                    .parse::<u64>()
                    .map_err(|e| format!("Failed to parse size: {}", e))?;
                let hash = parts[1].to_string();
                let url = parts[2].to_string();
                (size, hash, url)
            }
            // GDELT v3 GEG format: just url
            1 => {
                let url = parts[0].to_string();
                (0, String::new(), url)
            }
            _ => {
                return Err(format!(
                    "Invalid line format: expected 1 or 3 parts, got {}",
                    parts.len()
                ));
            }
        };

        // Extract filename from URL
        let filename = url
            .rsplit('/')
            .next()
            .ok_or_else(|| format!("Invalid URL: {}", url))?;

        // Parse timestamp from filename (first 14 characters are YYYYMMDDHHMMSS)
        if filename.len() < 14 {
            return Err(format!(
                "Filename too short to extract timestamp: {}",
                filename
            ));
        }
        let timestamp_str = &filename[..14];
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d%H%M%S")
            .map_err(|e| format!("Failed to parse timestamp from '{}': {}", timestamp_str, e))?;

        // Determine if this is a translation file
        let is_translation = filename.contains(".translation.");

        // Determine table type based on URL pattern
        let table_type = TableType::from_url(&url)
            .ok_or_else(|| format!("Unknown table type in filename: {}", filename))?;

        Ok(Self::new(
            size,
            hash,
            url,
            timestamp,
            table_type,
            is_translation,
            version.to_string(),
        ))
    }

    /// Get the timestamp as NaiveDateTime
    pub fn get_timestamp(&self) -> NaiveDateTime {
        chrono::DateTime::from_timestamp(self.timestamp, 0)
            .expect("Invalid timestamp")
            .naive_utc()
    }

    /// Round a timestamp to the nearest 15-minute interval
    pub fn round_to_15min(dt: NaiveDateTime) -> NaiveDateTime {
        let minute = dt.minute();
        let rounded_minute = (minute / 15) * 15;
        dt.with_minute(rounded_minute)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }

    /// Round a timestamp to the nearest 30-minute interval
    pub fn round_to_30min(dt: NaiveDateTime) -> NaiveDateTime {
        let minute = dt.minute();
        let rounded_minute = (minute / 30) * 30;
        dt.with_minute(rounded_minute)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }
}

/// Type alias for a collection of tables for a specific day
pub type EventTableDay = Vec<Vec<super::event_table::EventTable>>;
pub type MentionTableDay = Vec<Vec<super::mention_table::MentionTable>>;
pub type GkgTableDay = Vec<Vec<super::gkg_table::GKGTable>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_v2_entry() {
        let line = "277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip";
        let entry = MasterlistEntry::from_line(line, "v2").unwrap();

        assert_eq!(entry.size, 277207);
        assert_eq!(entry.hash, "0897fb7630ac913409c48345dca7565e");
        assert_eq!(entry.table_type, TableType::Mentions);
        assert!(!entry.is_translation);
        assert_eq!(
            entry.get_timestamp().format("%Y%m%d%H%M%S").to_string(),
            "20150219004500"
        );
    }

    #[test]
    fn test_parse_geg_entry() {
        let line =
            "http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080400.geg-gcnlapi.json.gz";
        let entry = MasterlistEntry::from_line(line, "v3_geg").unwrap();

        assert_eq!(entry.size, 0);
        assert_eq!(entry.hash, "");
        assert_eq!(entry.table_type, TableType::Geg);
        assert_eq!(
            entry.get_timestamp().format("%Y%m%d%H%M%S").to_string(),
            "20230206080400"
        );
    }

    #[test]
    fn test_round_to_15min() {
        let dt = NaiveDateTime::parse_from_str("2024-01-15 14:23:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let rounded = MasterlistEntry::round_to_15min(dt);
        assert_eq!(rounded.format("%Y-%m-%d %H:%M:%S").to_string(), "2024-01-15 14:15:00");
    }

    #[test]
    fn test_round_to_30min() {
        let dt = NaiveDateTime::parse_from_str("2024-01-15 14:23:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let rounded = MasterlistEntry::round_to_30min(dt);
        assert_eq!(rounded.format("%Y-%m-%d %H:%M:%S").to_string(), "2024-01-15 14:00:00");
    }

    #[test]
    fn test_table_type_from_url() {
        assert_eq!(
            TableType::from_url("http://data.gdeltproject.org/gdeltv2/20150219004500.export.CSV.zip"),
            Some(TableType::Export)
        );
        assert_eq!(
            TableType::from_url("http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip"),
            Some(TableType::Mentions)
        );
        assert_eq!(
            TableType::from_url("http://data.gdeltproject.org/gdeltv2/20150219004500.gkg.csv.zip"),
            Some(TableType::Gkg)
        );
        assert_eq!(
            TableType::from_url("http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080400.geg-gcnlapi.json.gz"),
            Some(TableType::Geg)
        );
    }
}
