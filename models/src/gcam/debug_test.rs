//! Debug test to verify in-memory GCAM database functionality

#[cfg(test)]
mod tests {
    use crate::gcam::GCAMCodebookDatabase;
    use std::env;

    #[test]
    #[ignore]
    fn debug_memory_database() {
        println!("\n=== DEBUG: In-Memory GCAM Database Analysis ===");

        // Show current working directory for reference
        let current_dir = env::current_dir().expect("Failed to get current directory");
        println!("Current working directory: {}", current_dir.display());

        // Create the in-memory database
        match GCAMCodebookDatabase::new_temp() {
            Ok(db) => {
                let count = db.count();
                println!("✅ In-memory database created with {} entries", count);

                if count > 0 {
                    // Test enrichment of known keys
                    let test_keys = ["c1.1", "c12.1", "c14.1"];

                    for key in &test_keys {
                        match db.enrich_gcam_entry(key, 1.0) {
                            Ok(enriched) => {
                                if enriched.metadata.is_some() {
                                    println!("✅ '{}' enrichment working: has metadata", key);
                                } else {
                                    println!("❌ '{}' enrichment returns None metadata", key);
                                }
                            }
                            Err(e) => {
                                println!("❌ '{}' enrichment error: {}", key, e);
                            }
                        }
                    }

                    // List some variables to verify database content
                    match db.list_variables(10) {
                        Ok(vars) => {
                            println!("Sample variables in database: {:?}", vars);
                        }
                        Err(e) => {
                            println!("Failed to list variables: {}", e);
                        }
                    }

                    // Get dictionary statistics
                    match db.get_dictionary_stats() {
                        Ok(stats) => {
                            println!("Dictionary statistics (top 5):");
                            for (i, (dict, count)) in stats.iter().take(5).enumerate() {
                                println!("  {}. {} ({} entries)", i + 1, dict, count);
                            }
                        }
                        Err(e) => {
                            println!("Failed to get dictionary stats: {}", e);
                        }
                    }

                    // Test lookup chain for debugging
                    match db.test_lookup_chain("c1.1") {
                        Ok(chain) => {
                            println!("Lookup chain for 'c1.1':");
                            for line in chain.lines() {
                                println!("  {}", line);
                            }
                        }
                        Err(e) => {
                            println!("Failed to get lookup chain: {}", e);
                        }
                    }
                } else {
                    println!("❌ Database is empty - CSV may not be embedded correctly");
                }
            }
            Err(e) => {
                println!("❌ Failed to create in-memory database: {}", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_memory_database_consistency() {
        println!("\n=== Testing In-Memory Database Consistency ===");

        // Create the database
        let db = GCAMCodebookDatabase::new_temp().expect("Failed to create database");

        let count = db.count();
        println!("Database has {} entries", count);

        if count > 0 {
            // Get some test variables
            let variables = db.list_variables(10).expect("Failed to list variables");

            for var in variables.iter().take(3) {
                // Test variable lookup
                let entry = db.get_by_variable(var).expect("Failed to get by variable");
                assert!(entry.is_some(), "Variable '{}' should exist", var);

                if let Some(entry) = entry {
                    // Test dictionary_id lookup
                    let dict_entry = db
                        .get_by_dictionary_id(entry.dictionary_id)
                        .expect("Failed to get by dictionary_id");
                    assert!(
                        dict_entry.is_some(),
                        "Dictionary ID {} should exist",
                        entry.dictionary_id
                    );

                    // Test enrichment
                    let enriched = db
                        .enrich_gcam_entry(var, 1.0)
                        .expect("Failed to enrich entry");
                    assert!(
                        enriched.metadata.is_some(),
                        "Enriched entry should have metadata"
                    );

                    println!("✅ Variable '{}' passed all consistency checks", var);
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn test_database_creation_paths() {
        println!("\n=== Testing Database Creation with Different Paths ===");

        // Test that path parameter is ignored for in-memory database
        let db1 = GCAMCodebookDatabase::new("./some/path").expect("Failed to create database");
        let db2 =
            GCAMCodebookDatabase::new("/tmp/another/path").expect("Failed to create database");
        let db3 = GCAMCodebookDatabase::new_temp().expect("Failed to create temp database");

        // All should have the same count since they use the same embedded data
        let count1 = db1.count();
        let count2 = db2.count();
        let count3 = db3.count();

        println!("Database counts: {} {} {}", count1, count2, count3);

        assert_eq!(
            count1, count2,
            "All database instances should have same count"
        );
        assert_eq!(
            count2, count3,
            "All database instances should have same count"
        );

        println!("✅ All database instances have consistent data");
    }
}
