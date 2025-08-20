//! GCAM (Global Content Analysis Measures) Module
//!
//! This module provides comprehensive support for GCAM data enrichment, including:
//! - Dictionary and language type definitions
//! - Fast in-memory database for codebook lookups
//! - Enhanced GKG table structures with enriched GCAM entries
//! - Bincode serialization support for performance
//!
//! ## Module Structure
//!
//! - `lookup`: Core types, enums, and data structures for GCAM
//! - `memory_database`: In-memory BTreeMap database for fast codebook lookups
//! - `enhanced_gkg`: Enhanced GKG table with enriched GCAM entries
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use models::gcam::{GCAMCodebookDatabase, GCAMCodebookParser, Dictionary, debug_gcam_enrichment};
//! use models::types::gkg_table::GKGTable;
//! use csv::StringRecord;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Setup database
//! let db = GCAMCodebookDatabase::new("./gcam_db")?;
//! let count = GCAMCodebookParser::parse_and_populate("GCAM-MASTER-CODEBOOK.csv", &db)?;
//! println!("Loaded {} GCAM codebook entries", count);
//!
//! // Debug enrichment if entries appear to have no metadata
//! let debug_info = debug_gcam_enrichment(&db, "c1.1")?;
//! println!("Debug info: {}", debug_info);
//!
//! // Create GKG table with enriched GCAM entries (csv_record would be from actual CSV parsing)
//! let csv_record = StringRecord::new(); // Example placeholder
//! let gkg_table = GKGTable::try_from_with_gcam_db(csv_record, &db)?;
//!
//! // Access enriched data
//! let forest_entries = gkg_table.gcam_by_dictionary(&Dictionary::ForestValues);
//! let stats = gkg_table.gcam_coverage_stats();
//!
//! // Check if entries are properly enriched
//! for entry in &forest_entries {
//!     if entry.metadata.is_some() {
//!         println!("✓ Entry {} has metadata: {}", entry.key, entry.dimension_name().unwrap_or("unknown"));
//!     } else {
//!         println!("✗ Entry {} missing metadata", entry.key);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Troubleshooting Enrichment Issues
//!
//! If your `EnrichedGCAMEntry` instances show `metadata: None`, this means the database
//! doesn't contain the GCAM key you're looking for. Common causes:
//!
//! 1. **Database not populated**: Make sure you've called `GCAMCodebookParser::parse_and_populate()`
//! 2. **Wrong CSV file**: Ensure you're using the correct GCAM Master Codebook CSV
//! 3. **Key format mismatch**: GCAM keys like "c1.1" must exactly match the codebook
//!
//! Use the `debug_gcam_enrichment()` function to diagnose issues.

pub mod debug_test;
pub mod lookup;
pub mod memory_database;

// Re-export commonly used types for convenience
pub use lookup::{Dictionary, EnrichedGCAMEntry, GCAMCodebookEntry, Language, MeasurementType};
pub use memory_database::{GCAMCodebookDatabase, GCAMCodebookParser};

/// Debug helper function to diagnose GCAM enrichment issues
pub fn debug_gcam_enrichment(db: &GCAMCodebookDatabase, test_key: &str) -> anyhow::Result<String> {
    let diagnostics = db.get_diagnostics()?;
    let has_key = db.has_variable(test_key)?;
    let sample_vars = db.list_variables(10)?;
    let raw_contents = db.debug_raw_contents()?;
    let lookup_chain = db.test_lookup_chain(test_key)?;

    Ok(format!(
        "{}\n\nTest key '{}' exists: {}\nFirst 10 variables: {:?}\n\n{}\n\n{}",
        diagnostics, test_key, has_key, sample_vars, raw_contents, lookup_chain
    ))
}

/// Verify that a GCAM database is working and can enrich specific keys
///
/// This function helps debug why GKGTable entries might show metadata: None
/// by testing the exact enrichment process that GKGTable uses.
pub fn verify_gcam_enrichment(db_path: &str, test_keys: &[&str]) -> anyhow::Result<String> {
    let mut output = String::new();
    output.push_str(&format!("=== GCAM Enrichment Verification ===\n"));
    output.push_str(&format!("Database path: {}\n\n", db_path));

    // Try to open the database
    let db = match GCAMCodebookDatabase::new(db_path) {
        Ok(db) => {
            output.push_str("✅ Database opened successfully\n");
            db
        }
        Err(e) => {
            output.push_str(&format!("❌ Failed to open database: {}\n", e));
            return Ok(output);
        }
    };

    // Check database state
    let total_entries = db.count();
    output.push_str(&format!(
        "📊 Total entries in database: {}\n",
        total_entries
    ));

    if total_entries == 0 {
        output.push_str("❌ Database is empty! You need to populate it first.\n");
        output.push_str("   Run: cargo run --bin populate_gcam_db\n");
        return Ok(output);
    }

    // Test enrichment for each key
    output.push_str("\n🧪 Testing enrichment for specific keys:\n");
    for &key in test_keys {
        output.push_str(&format!("\n🔍 Testing key: '{}'\n", key));

        // Step 1: Check if variable exists
        match db.has_variable(key) {
            Ok(exists) => {
                if exists {
                    output.push_str("  ✅ Variable exists in database\n");
                } else {
                    output.push_str("  ❌ Variable NOT found in database\n");
                    continue;
                }
            }
            Err(e) => {
                output.push_str(&format!("  ⚠️ Error checking variable: {}\n", e));
                continue;
            }
        }

        // Step 2: Test enrichment
        match db.enrich_gcam_entry(key, 1.0) {
            Ok(enriched) => {
                if let Some(metadata) = &enriched.metadata {
                    output.push_str("  ✅ Enrichment SUCCESS\n");
                    output.push_str(&format!("     Dictionary: {}\n", metadata.dictionary));
                    output.push_str(&format!("     Dimension: {}\n", metadata.dimension_name));
                    output.push_str(&format!("     Language: {}\n", metadata.language));
                } else {
                    output.push_str("  ❌ Enrichment FAILED - metadata is None\n");

                    // Additional debugging
                    match db.test_lookup_chain(key) {
                        Ok(lookup_debug) => {
                            output.push_str("     Lookup chain debug:\n");
                            for line in lookup_debug.lines() {
                                output.push_str(&format!("       {}\n", line));
                            }
                        }
                        Err(e) => {
                            output.push_str(&format!("     Failed to get lookup debug: {}\n", e));
                        }
                    }
                }
            }
            Err(e) => {
                output.push_str(&format!("  ⚠️ Enrichment error: {}\n", e));
            }
        }
    }

    // Show database diagnostics
    output.push_str("\n📈 Database Statistics:\n");
    match db.get_dictionary_stats() {
        Ok(stats) => {
            output.push_str(&format!("   Total dictionaries: {}\n", stats.len()));
            if !stats.is_empty() {
                output.push_str("   Top 5 dictionaries:\n");
                for (i, (name, count)) in stats.iter().take(5).enumerate() {
                    output.push_str(&format!("     {}. {} ({} entries)\n", i + 1, name, count));
                }
            }
        }
        Err(e) => {
            output.push_str(&format!("   Failed to get stats: {}\n", e));
        }
    }

    output.push_str("\n💡 Troubleshooting:\n");
    output.push_str("   - If variables exist but enrichment fails, check lookup chain debug\n");
    output.push_str("   - If variables don't exist, the database may have different keys\n");
    output.push_str("   - If database is empty, run populate_gcam_db first\n");
    output.push_str("   - Make sure GKGTable is using the same database path\n");

    Ok(output)
}

/// Convenient function to populate a GCAM database from a CSV file
///
/// This function creates or overwrites the database and populates it with the GCAM Master Codebook.
/// Returns the number of entries loaded.
///
/// # Arguments
/// * `csv_path` - Path to the GCAM Master Codebook CSV file
/// * `db_path` - Path where the database should be created
///
/// # Example
/// ```rust,no_run
/// use models::gcam::populate_gcam_database;
///
/// let count = populate_gcam_database("GCAM-MASTER-CODEBOOK.csv", "./gcam_db")?;
/// println!("Loaded {} GCAM entries", count);
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn populate_gcam_database<P1, P2>(csv_path: P1, db_path: P2) -> anyhow::Result<usize>
where
    P1: AsRef<std::path::Path>,
    P2: AsRef<std::path::Path>,
{
    let csv_path = csv_path.as_ref();
    let db_path = db_path.as_ref();

    // Check if CSV file exists
    if !csv_path.exists() {
        return Err(anyhow::anyhow!(
            "CSV file not found: {}",
            csv_path.display()
        ));
    }

    // Remove existing database if it exists
    if db_path.exists() {
        std::fs::remove_dir_all(db_path)
            .map_err(|e| anyhow::anyhow!("Failed to remove existing database: {}", e))?;
    }

    // Create new database
    let db = GCAMCodebookDatabase::new(db_path)
        .map_err(|e| anyhow::anyhow!("Failed to create database: {}", e))?;

    // Parse and populate
    let count = GCAMCodebookParser::parse_and_populate(csv_path, &db)
        .map_err(|e| anyhow::anyhow!("Failed to populate database: {}", e))?;

    // Flush to ensure data is written
    db.flush()
        .map_err(|e| anyhow::anyhow!("Failed to flush database: {}", e))?;

    Ok(count)
}

/// GCAM coverage statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GCAMCoverageStats {
    pub total_entries: usize,
    pub entries_with_metadata: usize,
    pub entries_without_metadata: usize,
    pub coverage_percentage: f64,
}

impl GCAMCoverageStats {
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow::anyhow!("Serialization failed: {}", e))
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        bincode::deserialize(bytes).map_err(|e| anyhow::anyhow!("Deserialization failed: {}", e))
    }
}

impl std::fmt::Display for GCAMCoverageStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GCAM Coverage: {}/{} ({:.1}%) entries have metadata",
            self.entries_with_metadata, self.total_entries, self.coverage_percentage
        )
    }
}

// Legacy compatibility - keep the simple GCAMEntry for backward compatibility
#[derive(Debug, Clone, PartialEq)]
pub struct GCAMEntry {
    pub key: String,
    pub value: f32,
}

impl GCAMEntry {
    pub fn new(key: String, value: f32) -> Self {
        Self { key, value }
    }
}

impl From<&EnrichedGCAMEntry> for GCAMEntry {
    fn from(enriched: &EnrichedGCAMEntry) -> Self {
        Self {
            key: enriched.key.clone(),
            value: enriched.value,
        }
    }
}

impl From<GCAMEntry> for EnrichedGCAMEntry {
    fn from(simple: GCAMEntry) -> Self {
        Self::from_simple(simple.key, simple.value)
    }
}
