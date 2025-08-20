//! GCAM Database Debugging Tool
//!
//! This utility helps diagnose issues with GCAM enrichment by checking
//! the in-memory database state and testing lookups.

use anyhow::Result;
use models::gcam::{GCAMCodebookDatabase, debug_gcam_enrichment};

fn main() -> Result<()> {
    // Initialize logger to see debug output
    env_logger::init();

    println!("=== GCAM In-Memory Database Debugging Tool ===\n");

    // Create the in-memory database (no file path needed)
    println!("Creating in-memory database from embedded CSV data...");

    let db = match GCAMCodebookDatabase::new_temp() {
        Ok(db) => {
            let count = db.count();
            println!(
                "✓ In-memory database created successfully with {} entries",
                count
            );

            if count == 0 {
                println!("✗ Database is empty - CSV data may not be embedded correctly");
                return Ok(());
            }

            db
        }
        Err(e) => {
            println!("✗ Failed to create in-memory database: {}", e);
            return Ok(());
        }
    };

    // Get database diagnostics
    println!("\n=== Database Diagnostics ===");
    let diagnostics = db.get_diagnostics()?;
    println!("{}", diagnostics);

    // Test specific keys that were showing as None
    println!("\n=== Testing Specific Keys ===");
    let test_keys = vec!["wc", "c1.1", "c1.4", "c12.1", "c12.10"];

    for key in test_keys {
        println!("\nTesting key: '{}'", key);

        // Check if key exists in variable index
        match db.has_variable(key) {
            Ok(exists) => {
                if exists {
                    println!("  ✓ Key exists in variable index");

                    // Try to get the metadata
                    match db.get_by_variable(key) {
                        Ok(Some(metadata)) => {
                            println!("  ✓ Successfully retrieved metadata:");
                            println!("    - Dictionary: {}", metadata.dictionary);
                            println!("    - Dimension: {}", metadata.dimension_name);
                            println!("    - Language: {}", metadata.language);
                            println!("    - Type: {}", metadata.measurement_type);
                        }
                        Ok(None) => {
                            println!("  ✗ Key exists in index but metadata lookup returned None");
                        }
                        Err(e) => {
                            println!("  ✗ Error retrieving metadata: {}", e);
                        }
                    }
                } else {
                    println!("  ✗ Key not found in variable index");
                }
            }
            Err(e) => {
                println!("  ✗ Error checking if key exists: {}", e);
            }
        }

        // Test the enrichment function
        match db.enrich_gcam_entry(key, 1.0) {
            Ok(enriched) => {
                if enriched.metadata.is_some() {
                    println!("  ✓ Enrichment successful - metadata found");
                } else {
                    println!("  ✗ Enrichment returned None metadata");
                }
            }
            Err(e) => {
                println!("  ✗ Enrichment failed: {}", e);
            }
        }
    }

    // Show sample variables in the database
    println!("\n=== Sample Variables in Database ===");
    match db.list_variables(20) {
        Ok(variables) => {
            if variables.is_empty() {
                println!("No variables found in database");
            } else {
                println!("First 20 variables:");
                for (i, var) in variables.iter().enumerate() {
                    println!("  {}: {}", i + 1, var);
                }
            }
        }
        Err(e) => {
            println!("Error listing variables: {}", e);
        }
    }

    // Use the debug helper function
    println!("\n=== Debug Helper Output ===");
    match debug_gcam_enrichment(&db, "c1.1") {
        Ok(debug_info) => {
            println!("{}", debug_info);
        }
        Err(e) => {
            println!("Debug helper failed: {}", e);
        }
    }

    println!("\n=== Debugging Complete ===");
    println!("If all entries show metadata: None, the most likely causes are:");
    println!("1. The embedded GCAM-MASTER-CODEBOOK.csv file is missing or incorrectly formatted");
    println!("2. The CSV parsing macro failed during compilation");
    println!("3. The CSV file uses different variable names than expected");
    println!("\nTo fix:");
    println!(
        "1. Ensure GCAM-MASTER-CODEBOOK.csv exists in the project root and is properly formatted"
    );
    println!("2. Rebuild the project to regenerate the embedded database");
    println!("3. Check that the CSV has columns: Variable, DictionaryID, DimensionID, etc.");

    Ok(())
}
