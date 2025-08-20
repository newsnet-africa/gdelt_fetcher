//! GCAM Database Verification Utility
//!
//! This utility verifies the in-memory GCAM database that is embedded at compile time.
//! The database is no longer populated from external files but is built into the binary.

use anyhow::Result;
use models::gcam::GCAMCodebookDatabase;

fn main() -> Result<()> {
    // Initialize logger for debug output
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("=== GCAM In-Memory Database Verification Utility ===\n");
    println!("Note: The GCAM database is now embedded at compile time.");
    println!(
        "This utility verifies the embedded data instead of populating from external files.\n"
    );

    // Create the in-memory database (loads embedded data)
    println!("1. Creating in-memory database from embedded CSV data...");
    let db = GCAMCodebookDatabase::new_temp()
        .map_err(|e| anyhow::anyhow!("Failed to create in-memory database: {}", e))?;

    let count = db.count();
    println!("✅ Database created successfully with {} entries", count);

    if count == 0 {
        println!("❌ Warning: Database is empty!");
        println!("   This indicates the embedded CSV data was not loaded correctly.");
        println!("   Check that GCAM-MASTER-CODEBOOK-fixed.csv exists and is properly embedded.");
        return Ok(());
    }

    // Test basic functionality
    println!("\n2. Testing database functionality...");

    // Get some sample variables
    let sample_vars = db.list_variables(10)?;
    println!(
        "   Sample variables: {:?}",
        sample_vars.iter().take(5).collect::<Vec<_>>()
    );

    // Test enrichment with known variables
    if !sample_vars.is_empty() {
        let test_var = &sample_vars[0];
        match db.enrich_gcam_entry(test_var, 1.0) {
            Ok(enriched) => {
                if enriched.metadata.is_some() {
                    println!("✅ Enrichment test passed for variable '{}'", test_var);
                } else {
                    println!(
                        "❌ Enrichment test failed: metadata is None for '{}'",
                        test_var
                    );
                }
            }
            Err(e) => {
                println!("❌ Enrichment test error for '{}': {}", test_var, e);
            }
        }
    }

    // Show dictionary statistics
    println!("\n3. Dictionary statistics:");
    match db.get_dictionary_stats() {
        Ok(stats) => {
            println!("   Total dictionaries: {}", stats.len());
            for (i, (name, count)) in stats.iter().take(5).enumerate() {
                println!("   {}. {} ({} entries)", i + 1, name, count);
            }
        }
        Err(e) => {
            println!("   Failed to get dictionary stats: {}", e);
        }
    }

    // Test some common GCAM keys that should exist
    println!("\n4. Testing common GCAM keys:");
    let common_keys = ["c1.1", "c1.2", "c1.3", "c1.4", "c12.1"];
    let mut found_count = 0;

    for key in &common_keys {
        if db.has_variable(key)? {
            found_count += 1;
            println!("   ✅ '{}' found", key);
        } else {
            println!("   ❌ '{}' not found", key);
        }
    }

    // Test some v-prefixed keys that appear in real GKG data
    println!("\n5. Testing v-prefixed GCAM keys:");
    let v_keys = ["v10.1", "v19.1", "v20.1", "v21.1", "v42.2"];
    let mut v_found_count = 0;

    for key in &v_keys {
        if db.has_variable(key)? {
            v_found_count += 1;
            println!("   ✅ '{}' found", key);
        } else {
            println!("   ❌ '{}' not found", key);
        }
    }

    println!("\n=== Summary ===");
    println!("📊 Database entries: {}", count);
    println!(
        "🔍 Common keys found: {}/{}",
        found_count,
        common_keys.len()
    );
    println!(
        "🔬 V-prefixed keys found: {}/{}",
        v_found_count,
        v_keys.len()
    );

    if count > 2000 && found_count > 2 && v_found_count > 2 {
        println!("✅ Database verification PASSED");
        println!("   The in-memory GCAM database is working correctly!");
        println!("   GCAM enrichment should now work in GKG table parsing.");
    } else {
        println!("❌ Database verification FAILED");
        println!("   The database may not be properly populated or embedded.");
        println!("   GCAM enrichment may not work correctly.");
    }

    println!("\n💡 Usage Notes:");
    println!("   - The database is now embedded at compile time");
    println!("   - No external CSV files are needed at runtime");
    println!("   - GKG table parsing automatically enriches GCAM entries");
    println!("   - Use GCAMCodebookDatabase::new_temp() to access the database");

    Ok(())
}
