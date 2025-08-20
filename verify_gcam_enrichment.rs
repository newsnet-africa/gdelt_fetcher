//! GCAM Enrichment Verification Tool
//!
//! This tool helps debug why specific GCAM keys are showing metadata: None
//! in GKGTable entries by testing the exact enrichment process.

use anyhow::Result;
use models::gcam::{GCAMCodebookDatabase, verify_gcam_enrichment};
use std::env;

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    println!("=== GCAM Enrichment Verification Tool ===\n");

    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    let db_path = if args.len() > 1 {
        &args[1]
    } else {
        "./gcam_db"
    };

    // Test keys that are showing as None in GKGTable
    let problematic_keys = vec![
        "wc", "c1.1", "c1.4", "c12.1", "c12.10", "c12.12", "c12.13", "c12.14", "c12.3", "c12.4",
        "c12.5", "c12.7", "c12.8", "c12.9", "c13.1", "c13.12", "c13.2", "c13.3", "c13.4", "c14.1",
        "c14.10", "c14.11", "c14.2", "c14.3", "c14.4", "c14.5", "c14.6", "c14.7",
    ];

    println!(
        "Testing {} problematic keys from GKGTable",
        problematic_keys.len()
    );
    println!("Database path: {}\n", db_path);

    // Run the verification
    match verify_gcam_enrichment(db_path, &problematic_keys) {
        Ok(report) => {
            println!("{}", report);
        }
        Err(e) => {
            eprintln!("❌ Verification failed: {}", e);
            return Err(e);
        }
    }

    // Additional test: Try to open the database directly and test a few keys manually
    println!("\n{}", "=".repeat(60));
    println!("MANUAL VERIFICATION");
    println!("{}", "=".repeat(60));

    match GCAMCodebookDatabase::new(db_path) {
        Ok(db) => {
            println!("✅ Successfully opened database at: {}", db_path);
            println!("📊 Total entries: {}", db.count());

            // Test manual enrichment
            println!("\n🧪 Manual enrichment test:");
            let test_keys = ["c1.1", "c12.1", "wc"];

            for &key in &test_keys {
                match db.enrich_gcam_entry(key, 1.0) {
                    Ok(enriched) => {
                        println!(
                            "Key '{}': metadata = {:?}",
                            key,
                            enriched.metadata.is_some()
                        );
                        if let Some(meta) = &enriched.metadata {
                            println!("  - Dictionary: {}", meta.dictionary);
                            println!("  - Dimension: {}", meta.dimension_name);
                        }
                    }
                    Err(e) => {
                        println!("Key '{}': ERROR - {}", key, e);
                    }
                }
            }

            // Show some sample variables that DO exist
            println!("\n📋 Sample variables that exist in database:");
            match db.list_variables(20) {
                Ok(vars) => {
                    for (i, var) in vars.iter().take(10).enumerate() {
                        println!("  {}. {}", i + 1, var);
                    }
                    if vars.len() > 10 {
                        println!("  ... and {} more", vars.len() - 10);
                    }
                }
                Err(e) => {
                    println!("  Failed to list variables: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to open database: {}", e);
            eprintln!("\nPossible issues:");
            eprintln!("1. Database doesn't exist at '{}'", db_path);
            eprintln!("2. Database is corrupted");
            eprintln!("3. Permission issues");
            eprintln!("\nTry running: cargo run --bin populate_gcam_db");
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("NEXT STEPS");
    println!("{}", "=".repeat(60));
    println!("If keys exist in database but GKGTable shows metadata: None:");
    println!("1. Check that GKGTable is using the same database path");
    println!("2. Verify the GKGTable is calling enrich_gcam_entry correctly");
    println!("3. Check if there are multiple database instances");
    println!("\nIf keys don't exist in database:");
    println!("1. The CSV file may not contain these specific variables");
    println!("2. The variables may be in a different format");
    println!("3. Re-populate the database with the correct CSV file");

    Ok(())
}
