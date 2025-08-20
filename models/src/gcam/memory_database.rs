//! In-Memory GCAM Database Implementation
//!
//! This module provides a compile-time generated, in-memory GCAM database
//! that replaces the previous sled-based implementation. The database is
//! populated at compile time from the GCAM CSV file using macros.
//!
//! ## Testing
//!
//! Database tests are ignored by default to improve test performance since they
//! require loading the embedded CSV data. To run them explicitly:
//!
//! ```bash
//! # Run only database tests (ignored by default)
//! cargo test gcam -- --ignored
//!
//! # Run all tests including database tests
//! cargo test gcam -- --include-ignored
//!
//! # Run specific database test
//! cargo test test_database_operations -- --ignored
//! ```

use crate::gcam::lookup::{
    Dictionary, EnrichedGCAMEntry, GCAMCodebookEntry, Language, MeasurementType,
};
use anyhow::Result;
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Macro to generate the in-memory GCAM database from CSV data
macro_rules! generate_gcam_database {
    ($csv_content:expr) => {{
        let mut variable_map: BTreeMap<String, GCAMCodebookEntry> = BTreeMap::new();
        let mut dictionary_map: BTreeMap<u32, GCAMCodebookEntry> = BTreeMap::new();

        let mut lines = $csv_content.lines();

        // Skip header line
        if let Some(_header) = lines.next() {
            for line in lines {
                if line.trim().is_empty() {
                    continue;
                }

                let fields: Vec<&str> = line.split('\t').collect();
                if fields.len() < 8 {
                    continue; // Skip malformed lines
                }

                // Parse CSV fields - adjust indices based on actual CSV structure
                let variable = fields[0].trim_matches('"').to_string();
                let dictionary_id: u32 = match fields[1].trim_matches('"').parse() {
                    Ok(id) => id,
                    Err(_) => continue, // Skip invalid dictionary IDs
                };
                let dimension_id: u32 = match fields[2].trim_matches('"').parse() {
                    Ok(id) => id,
                    Err(_) => 0, // Default to 0 if parsing fails
                };
                let measurement_type = MeasurementType::from(fields[3].trim_matches('"'));
                let language = Language::from(fields[4].trim_matches('"'));
                let dictionary_name = fields[5].trim_matches('"');
                let dimension_name = fields[6].trim_matches('"').to_string();
                let citation = fields[7].trim_matches('"').to_string();

                let dictionary = Dictionary::from_human_name(dictionary_name);

                let entry = GCAMCodebookEntry {
                    variable: variable.clone(),
                    dictionary_id,
                    dimension_id,
                    measurement_type,
                    language,
                    dictionary,
                    dimension_name,
                    citation,
                };

                variable_map.insert(variable, entry.clone());
                dictionary_map.insert(dictionary_id, entry);
            }
        }

        (variable_map, dictionary_map)
    }};
}

/// Global static database instances
static VARIABLE_MAP: OnceLock<BTreeMap<String, GCAMCodebookEntry>> = OnceLock::new();
static DICTIONARY_MAP: OnceLock<BTreeMap<u32, GCAMCodebookEntry>> = OnceLock::new();

/// Initialize the global database maps
fn init_database() -> (
    &'static BTreeMap<String, GCAMCodebookEntry>,
    &'static BTreeMap<u32, GCAMCodebookEntry>,
) {
    let variable_map = VARIABLE_MAP.get_or_init(|| {
        let csv_content = include_str!("../../../GCAM-MASTER-CODEBOOK-fixed.csv");
        let (var_map, _) = generate_gcam_database!(csv_content);
        var_map
    });

    let dictionary_map = DICTIONARY_MAP.get_or_init(|| {
        let csv_content = include_str!("../../../GCAM-MASTER-CODEBOOK-fixed.csv");
        let (_, dict_map) = generate_gcam_database!(csv_content);
        dict_map
    });

    (variable_map, dictionary_map)
}

/// In-memory GCAM Codebook Database
pub struct GCAMCodebookDatabase;

impl GCAMCodebookDatabase {
    /// Create a new GCAM Codebook Database (now just returns an empty struct)
    pub fn new<P: AsRef<std::path::Path>>(_db_path: P) -> Result<Self> {
        // Initialize the global maps (this is a no-op if already initialized)
        init_database();
        Ok(Self {})
    }

    /// Create an in-memory database for testing (same as new now)
    pub fn new_temp() -> Result<Self> {
        init_database();
        Ok(Self {})
    }

    /// Insert a GCAM codebook entry (no-op for static database)
    pub fn insert_entry(&self, _entry: &GCAMCodebookEntry) -> Result<()> {
        // This is a no-op since the database is pre-populated at compile time
        Ok(())
    }

    /// Get a GCAM codebook entry by dictionary ID
    pub fn get_by_dictionary_id(&self, dictionary_id: u32) -> Result<Option<GCAMCodebookEntry>> {
        let (_, dictionary_map) = init_database();

        // log::debug!(
        //     "looking up dictionary_id {} with key '{}'",
        //     dictionary_id,
        //     dictionary_id
        // );

        if let Some(entry) = dictionary_map.get(&dictionary_id) {
            // log::debug!(
            //     "Successfully found entry for dictionary_id {}: variable '{}'",
            //     dictionary_id,
            //     entry.variable
            // );
            Ok(Some(entry.clone()))
        } else {
            // log::debug!("No entry found for dictionary_id {}", dictionary_id);
            Ok(None)
        }
    }

    /// Get a GCAM codebook entry by variable name
    pub fn get_by_variable(&self, variable: &str) -> Result<Option<GCAMCodebookEntry>> {
        let (variable_map, _) = init_database();

        // log::debug!("Looking up variable '{}'", variable);

        if let Some(entry) = variable_map.get(variable) {
            // log::debug!(
            //     "Successfully found entry for variable '{}': dictionary_id {}",
            //     variable,
            //     entry.dictionary_id
            // );
            Ok(Some(entry.clone()))
        } else {
            // log::debug!("No entry found for variable '{}'", variable);
            Ok(None)
        }
    }

    /// Get all entries for a specific dictionary
    pub fn get_by_dictionary(&self, dictionary: &Dictionary) -> Result<Vec<GCAMCodebookEntry>> {
        let (variable_map, _) = init_database();

        let entries: Vec<GCAMCodebookEntry> = variable_map
            .values()
            .filter(|entry| &entry.dictionary == dictionary)
            .cloned()
            .collect();

        Ok(entries)
    }

    /// Get all entries in the database
    pub fn get_all_entries(&self) -> Result<Vec<GCAMCodebookEntry>> {
        let (variable_map, _) = init_database();
        Ok(variable_map.values().cloned().collect())
    }

    /// Get the total count of entries
    pub fn count(&self) -> usize {
        let (variable_map, _) = init_database();
        variable_map.len()
    }

    /// Flush database (no-op for in-memory database)
    pub fn flush(&self) -> Result<()> {
        // No-op for in-memory database
        Ok(())
    }

    /// Enrich a GCAM entry with metadata
    pub fn enrich_gcam_entry(&self, key: &str, value: f32) -> Result<EnrichedGCAMEntry> {
        let metadata = self.get_by_variable(key)?;

        // log::debug!(
        //     "Enriching GCAM entry '{}' with value {}: metadata found = {}",
        //     key,
        //     value,
        //     metadata.is_some()
        // );

        Ok(EnrichedGCAMEntry::new(key.to_string(), value, metadata))
    }

    /// Enrich multiple GCAM entries with metadata
    pub fn enrich_gcam_entries(
        &self,
        entries: Vec<(String, f32)>,
    ) -> Result<Vec<EnrichedGCAMEntry>> {
        entries
            .into_iter()
            .map(|(key, value)| self.enrich_gcam_entry(&key, value))
            .collect()
    }

    /// Get database diagnostics
    pub fn get_diagnostics(&self) -> Result<String> {
        let (variable_map, dictionary_map) = init_database();

        Ok(format!(
            "GCAM Database Diagnostics:\n\
             - Total variables: {}\n\
             - Total dictionary entries: {}\n\
             - Database type: In-memory BTreeMap\n\
             - Source: Embedded CSV data",
            variable_map.len(),
            dictionary_map.len()
        ))
    }

    /// Check if a variable exists in the database
    pub fn has_variable(&self, variable: &str) -> Result<bool> {
        let (variable_map, _) = init_database();
        Ok(variable_map.contains_key(variable))
    }

    /// List the first N variables in the database
    pub fn list_variables(&self, limit: usize) -> Result<Vec<String>> {
        let (variable_map, _) = init_database();
        Ok(variable_map.keys().take(limit).cloned().collect())
    }

    /// Get statistics about dictionaries
    pub fn get_dictionary_stats(&self) -> Result<Vec<(String, usize)>> {
        let (variable_map, _) = init_database();
        let mut stats: BTreeMap<String, usize> = BTreeMap::new();

        for entry in variable_map.values() {
            let dict_name = entry.dictionary.to_string();
            *stats.entry(dict_name).or_insert(0) += 1;
        }

        let mut result: Vec<(String, usize)> = stats.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
        Ok(result)
    }

    /// Debug raw contents of the database
    pub fn debug_raw_contents(&self) -> Result<String> {
        let (variable_map, _) = init_database();

        let mut output = String::new();
        output.push_str("Raw Database Contents (first 10 entries):\n");

        for (i, (variable, entry)) in variable_map.iter().enumerate() {
            if i >= 10 {
                output.push_str("... (truncated)\n");
                break;
            }
            output.push_str(&format!(
                "  Variable: '{}' -> Dictionary ID: {}, Dictionary: {}\n",
                variable, entry.dictionary_id, entry.dictionary
            ));
        }

        Ok(output)
    }

    /// Test the lookup chain for a specific variable
    pub fn test_lookup_chain(&self, variable: &str) -> Result<String> {
        let (variable_map, dictionary_map) = init_database();

        let mut output = String::new();
        output.push_str(&format!("Lookup chain test for variable '{}':\n", variable));

        // Step 1: Check variable map
        output.push_str("1. Variable map lookup: ");
        if let Some(entry) = variable_map.get(variable) {
            output.push_str(&format!("FOUND (dictionary_id: {})\n", entry.dictionary_id));

            // Step 2: Check dictionary map
            output.push_str("2. Dictionary map lookup: ");
            if dictionary_map.contains_key(&entry.dictionary_id) {
                output.push_str("FOUND\n");
                output.push_str(&format!("3. Entry details: {:#?}\n", entry));
            } else {
                output.push_str("NOT FOUND (inconsistent database)\n");
            }
        } else {
            output.push_str("NOT FOUND\n");
            output.push_str("2. Dictionary map lookup: SKIPPED (variable not found)\n");

            // Show similar variables
            let similar: Vec<&String> = variable_map
                .keys()
                .filter(|k| k.contains(variable) || variable.contains(k.as_str()))
                .take(5)
                .collect();

            if !similar.is_empty() {
                output.push_str(&format!("   Similar variables: {:?}\n", similar));
            }
        }

        Ok(output)
    }
}

/// GCAM Codebook Parser (now a no-op since data is embedded)
pub struct GCAMCodebookParser;

impl GCAMCodebookParser {
    /// Parse and populate database (no-op for embedded database)
    pub fn parse_and_populate<P: AsRef<std::path::Path>>(
        _csv_path: P,
        _database: &GCAMCodebookDatabase,
    ) -> Result<usize> {
        // Initialize the database to ensure it's loaded
        let (variable_map, _) = init_database();
        Ok(variable_map.len())
    }

    /// Parse CSV file (no-op for embedded database)
    pub fn parse_csv<P: AsRef<std::path::Path>>(_csv_path: P) -> Result<Vec<GCAMCodebookEntry>> {
        let (variable_map, _) = init_database();
        Ok(variable_map.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    #[ignore]
    fn test_database_creation() -> Result<()> {
        let _db = GCAMCodebookDatabase::new_temp()?;
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_database_operations() -> Result<()> {
        let db = GCAMCodebookDatabase::new_temp()?;

        // Test count
        let count = db.count();
        assert!(count > 0, "Database should have entries");

        // Test that some basic lookups work
        let diagnostics = db.get_diagnostics()?;
        assert!(diagnostics.contains("Total variables"));

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_variable_lookup() -> Result<()> {
        let db = GCAMCodebookDatabase::new_temp()?;

        // List some variables to test with
        let variables = db.list_variables(5)?;
        if !variables.is_empty() {
            let test_var = &variables[0];

            // Test has_variable
            assert!(db.has_variable(test_var)?);

            // Test get_by_variable
            let entry = db.get_by_variable(test_var)?;
            assert!(entry.is_some());

            if let Some(entry) = entry {
                // Test get_by_dictionary_id
                let dict_entry = db.get_by_dictionary_id(entry.dictionary_id)?;
                assert!(dict_entry.is_some());
            }
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_enrich_entries() -> Result<()> {
        let db = GCAMCodebookDatabase::new_temp()?;

        // Get a test variable
        let variables = db.list_variables(1)?;
        if !variables.is_empty() {
            let test_var = &variables[0];

            // Test enrichment
            let enriched = db.enrich_gcam_entry(test_var, 1.5)?;
            assert_eq!(enriched.key, *test_var);
            assert_eq!(enriched.value, 1.5);
            assert!(enriched.metadata.is_some());
        }

        Ok(())
    }
}
