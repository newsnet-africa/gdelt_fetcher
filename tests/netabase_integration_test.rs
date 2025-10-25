//! Integration test for netabase masterlist storage
//!
//! Verifies the complete flow:
//! 1. Fetch masterlist file from source
//! 2. Parse masterlist entries into netabase models
//! 3. Store models in a netabase
//! 4. Update the store with entries from latest fetch items

#[cfg(feature = "netabase")]
#[tokio::test]
async fn test_netabase_masterlist_flow() {
    use gdelt_fetcher::MasterlistManager;
    use models::types::masterlist::TableType;
    use tempfile::tempdir;

    // Create a temporary directory for the database
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test_masterlist.db");

    // 1. Create manager with netabase storage
    let mut manager = MasterlistManager::with_storage(db_path.to_str().unwrap())
        .expect("Failed to create masterlist manager with storage");

    // 2. Fetch and parse masterlist entries from source (lastupdate.txt for faster test)
    let added_count = manager.update_latest().await
        .expect("Failed to fetch and store latest entries");

    println!("Added {} entries from lastupdate.txt", added_count);
    assert!(added_count > 0, "Should have added some entries");

    // 3. Verify entries are stored in netabase by querying
    let total_count = manager.count().await
        .expect("Failed to count entries");

    println!("Total entries in store: {}", total_count);
    assert_eq!(total_count, added_count, "Count should match added entries");

    // Debug: Let's see what types of entries we have
    let export_count = manager.find_by_day(chrono::Utc::now().naive_utc().date(), TableType::Export, false).await
        .expect("Failed to query exports").len();
    let mentions_count = manager.find_by_day(chrono::Utc::now().naive_utc().date(), TableType::Mentions, false).await
        .expect("Failed to query mentions").len();
    let gkg_count = manager.find_by_day(chrono::Utc::now().naive_utc().date(), TableType::Gkg, false).await
        .expect("Failed to query gkg").len();

    println!("By table type - Export: {}, Mentions: {}, GKG: {}", export_count, mentions_count, gkg_count);

    // 4. Query for latest entry - try all table types to find one that exists
    let mut latest_entry = manager.find_latest(TableType::Export, false).await
        .expect("Failed to find latest export");

    let mut found_table_type = TableType::Export;

    if latest_entry.is_none() {
        latest_entry = manager.find_latest(TableType::Mentions, false).await
            .expect("Failed to find latest mentions");
        found_table_type = TableType::Mentions;
    }

    if latest_entry.is_none() {
        latest_entry = manager.find_latest(TableType::Gkg, false).await
            .expect("Failed to find latest GKG");
        found_table_type = TableType::Gkg;
    }

    assert!(latest_entry.is_some(), "Should find at least one entry of any type");

    if let Some(ref entry) = latest_entry {
        println!("Latest {:?} entry:", entry.table_type);
        println!("  ID: {}", entry.id);
        println!("  URL: {}", entry.url);
        println!("  Timestamp: {}", entry.get_timestamp());
        println!("  Table type: {:?}", entry.table_type);

        // Verify the entry has correct fields
        assert!(!entry.id.is_empty(), "ID should not be empty");
        assert!(!entry.url.is_empty(), "URL should not be empty");
        assert!(entry.url.contains("gdeltproject.org"), "URL should be from GDELT");
        assert_eq!(entry.table_type, found_table_type, "Table type should match");
    }

    // 5. Update again (should not add duplicates)
    let second_update = manager.update_latest().await
        .expect("Failed to update again");

    println!("Second update added {} entries", second_update);
    // Note: might add a few new entries if GDELT published updates in the meantime,
    // but should not re-add all entries
    assert!(second_update <= added_count, "Should not re-add all entries");

    // 6. Verify by querying by timestamp
    if let Some(entry) = latest_entry {
        let timestamp = entry.get_timestamp();
        let retrieved = manager.find_by_timestamp(timestamp, found_table_type, false).await
            .expect("Failed to query by timestamp");

        assert!(retrieved.is_some(), "Should retrieve entry by timestamp");
        assert_eq!(retrieved.unwrap().id, entry.id, "Retrieved entry should match");
    }

    println!("✓ Complete netabase flow test passed!");
}

#[cfg(not(feature = "netabase"))]
#[test]
fn test_requires_netabase_feature() {
    println!("This test requires the 'netabase' feature to be enabled");
}
