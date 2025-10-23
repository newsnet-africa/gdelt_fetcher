use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a single entry in the GDELT masterfilelist
///
/// Format: <size> <hash> <url>
/// Example: 277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct MasterfilelistEntry {
    /// File size in bytes
    pub size: u64,
    /// MD5 hash of the file
    pub hash: String,
    /// URL to download the file
    pub url: String,
    /// Timestamp extracted from the filename (rounded to nearest 15 minutes)
    #[bincode(with_serde)]
    pub timestamp: NaiveDateTime,
    /// Table type (export, mentions, gkg)
    #[bincode(with_serde)]
    pub table_type: TableType,
    /// Whether this is a translation file
    pub is_translation: bool,
}

impl MasterfilelistEntry {
    /// Parse a masterfilelist entry from a line in the format:
    /// GDELT v2: <size> <hash> <url>
    /// GDELT v3 GEG: <url> (just the URL)
    pub fn from_line(line: &str) -> Result<Self, String> {
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
                // For GEG, we don't have size/hash in the masterfilelist
                (0, String::new(), url)
            }
            _ => {
                return Err(format!("Invalid line format: expected 1 or 3 parts, got {}", parts.len()));
            }
        };

        // Extract filename from URL
        let filename = url
            .rsplit('/')
            .next()
            .ok_or_else(|| format!("Invalid URL: {}", url))?;

        // Parse timestamp from filename (first 14 characters are YYYYMMDDHHMMSS)
        if filename.len() < 14 {
            return Err(format!("Filename too short to extract timestamp: {}", filename));
        }
        let timestamp_str = &filename[..14];
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d%H%M%S")
            .map_err(|e| format!("Failed to parse timestamp from '{}': {}", timestamp_str, e))?;

        // Determine if this is a translation file
        let is_translation = filename.contains(".translation.");

        // Determine table type based on URL pattern
        let table_type = if filename.contains(".export.") {
            TableType::Export
        } else if filename.contains(".mentions.") {
            TableType::Mentions
        } else if filename.contains(".gkg.") || filename.contains("gkg") {
            TableType::Gkg
        } else if url.contains("gdeltv3/geg") || filename.contains(".geg-") {
            // GEG files are considered GKG-like for table type purposes
            TableType::Gkg
        } else {
            return Err(format!("Unknown table type in filename: {}", filename));
        };

        Ok(Self {
            size,
            hash,
            url,
            timestamp,
            table_type,
            is_translation,
        })
    }

    /// Get a unique key for this entry (timestamp + table_type)
    pub fn key(&self) -> String {
        format!(
            "{}_{:?}{}",
            self.timestamp.format("%Y%m%d%H%M%S"),
            self.table_type,
            if self.is_translation { "_trans" } else { "" }
        )
    }
}

/// Table types in GDELT
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "netabase", derive(bincode::Encode, bincode::Decode))]
pub enum TableType {
    Export,
    Mentions,
    Gkg,
}

impl TableType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TableType::Export => "export",
            TableType::Mentions => "mentions",
            TableType::Gkg => "gkg",
        }
    }
}

impl std::fmt::Display for TableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Masterfilelist collection for a GDELT version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Masterfilelist {
    /// GDELT version (v2 or v3)
    pub version: String,
    /// All entries in the masterfilelist
    pub entries: Vec<MasterfilelistEntry>,
    /// Last update timestamp
    pub last_updated: Option<NaiveDateTime>,
}

impl Masterfilelist {
    pub fn new(version: String) -> Self {
        Self {
            version,
            entries: Vec::new(),
            last_updated: None,
        }
    }

    /// Parse a masterfilelist from text content
    pub fn from_content(content: &str, version: String) -> Result<Self, String> {
        let mut entries = Vec::new();

        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match MasterfilelistEntry::from_line(line) {
                Ok(entry) => entries.push(entry),
                Err(e) => {
                    eprintln!("Warning: Failed to parse line {}: {} - Error: {}", i + 1, line, e);
                }
            }
        }

        Ok(Self {
            version,
            entries,
            last_updated: Some(chrono::Utc::now().naive_utc()),
        })
    }

    /// Find entries by timestamp
    pub fn find_by_timestamp(&self, timestamp: NaiveDateTime) -> Vec<&MasterfilelistEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp == timestamp)
            .collect()
    }

    /// Find entries by table type
    pub fn find_by_table_type(&self, table_type: TableType) -> Vec<&MasterfilelistEntry> {
        self.entries
            .iter()
            .filter(|e| e.table_type == table_type)
            .collect()
    }

    /// Get the latest entry for a specific table type
    pub fn latest_for_table(&self, table_type: TableType, is_translation: bool) -> Option<&MasterfilelistEntry> {
        self.entries
            .iter()
            .filter(|e| e.table_type == table_type && e.is_translation == is_translation)
            .max_by_key(|e| e.timestamp)
    }

    /// Get all timestamps in the masterfilelist
    pub fn all_timestamps(&self) -> Vec<NaiveDateTime> {
        let mut timestamps: Vec<_> = self.entries.iter().map(|e| e.timestamp).collect();
        timestamps.sort();
        timestamps.dedup();
        timestamps
    }

    /// Update the masterfilelist with new entries from lastupdate.txt
    pub fn update_with_latest(&mut self, latest_content: &str) -> Result<usize, String> {
        let latest = Masterfilelist::from_content(latest_content, self.version.clone())?;
        let mut added_count = 0;

        for new_entry in latest.entries {
            // Check if we already have this entry
            if !self.entries.iter().any(|e| e.key() == new_entry.key()) {
                self.entries.push(new_entry);
                added_count += 1;
            }
        }

        self.last_updated = Some(chrono::Utc::now().naive_utc());
        Ok(added_count)
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the masterfilelist is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_masterfilelist_entry() {
        let line = "277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip";
        let entry = MasterfilelistEntry::from_line(line).unwrap();

        assert_eq!(entry.size, 277207);
        assert_eq!(entry.hash, "0897fb7630ac913409c48345dca7565e");
        assert_eq!(entry.table_type, TableType::Mentions);
        assert!(!entry.is_translation);
        assert_eq!(entry.timestamp.format("%Y%m%d%H%M%S").to_string(), "20150219004500");
    }

    #[test]
    fn test_parse_translation_entry() {
        let line = "48910 3a59414c24547e5c3ade383e9f78365b http://data.gdeltproject.org/gdeltv2/20250806200000.translation.export.CSV.zip";
        let entry = MasterfilelistEntry::from_line(line).unwrap();

        assert_eq!(entry.table_type, TableType::Export);
        assert!(entry.is_translation);
    }

    #[test]
    fn test_masterfilelist_from_content() {
        let content = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
9555639 b02920524f0b48c07bdab6c6d354a789 http://data.gdeltproject.org/gdeltv2/20150219004500.gkg.csv.zip
225092 6b4e1d0421548dbba59754d0f164d2a1 http://data.gdeltproject.org/gdeltv2/20150219010000.export.CSV.zip
"#;

        let masterlist = Masterfilelist::from_content(content, "v2".to_string()).unwrap();
        assert_eq!(masterlist.entries.len(), 3);
        assert_eq!(masterlist.version, "v2");
    }

    #[test]
    fn test_find_by_table_type() {
        let content = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
9555639 b02920524f0b48c07bdab6c6d354a789 http://data.gdeltproject.org/gdeltv2/20150219004500.gkg.csv.zip
225092 6b4e1d0421548dbba59754d0f164d2a1 http://data.gdeltproject.org/gdeltv2/20150219010000.export.CSV.zip
"#;

        let masterlist = Masterfilelist::from_content(content, "v2".to_string()).unwrap();
        let mentions = masterlist.find_by_table_type(TableType::Mentions);
        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].table_type, TableType::Mentions);
    }

    #[test]
    fn test_update_with_latest() {
        let initial = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
"#;

        let latest = r#"
277207 0897fb7630ac913409c48345dca7565e http://data.gdeltproject.org/gdeltv2/20150219004500.mentions.CSV.zip
9555639 b02920524f0b48c07bdab6c6d354a789 http://data.gdeltproject.org/gdeltv2/20150219004500.gkg.csv.zip
"#;

        let mut masterlist = Masterfilelist::from_content(initial, "v2".to_string()).unwrap();
        assert_eq!(masterlist.len(), 1);

        let added = masterlist.update_with_latest(latest).unwrap();
        assert_eq!(added, 1);
        assert_eq!(masterlist.len(), 2);
    }

    #[test]
    fn test_parse_geg_entry() {
        // GEG format: just the URL (no size or hash)
        let line = "http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080400.geg-gcnlapi.json.gz";
        let entry = MasterfilelistEntry::from_line(line).unwrap();

        assert_eq!(entry.size, 0); // Size not available in GEG format
        assert_eq!(entry.hash, ""); // Hash not available in GEG format
        assert_eq!(entry.table_type, TableType::Gkg); // GEG treated as GKG type
        assert!(!entry.is_translation);
        assert_eq!(entry.timestamp.format("%Y%m%d%H%M%S").to_string(), "20230206080400");
    }

    #[test]
    fn test_masterfilelist_from_geg_content() {
        let content = r#"
http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080400.geg-gcnlapi.json.gz
http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080500.geg-gcnlapi.json.gz
http://data.gdeltproject.org/gdeltv3/geg_gcnlapi/20230206080600.geg-gcnlapi.json.gz
"#;

        let masterlist = Masterfilelist::from_content(content, "v3_geg".to_string()).unwrap();
        assert_eq!(masterlist.entries.len(), 3);
        assert_eq!(masterlist.version, "v3_geg");

        // All should be GKG type
        for entry in &masterlist.entries {
            assert_eq!(entry.table_type, TableType::Gkg);
            assert_eq!(entry.size, 0);
            assert_eq!(entry.hash, "");
        }
    }
}
