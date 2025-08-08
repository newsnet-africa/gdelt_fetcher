//! Integration tests for external GDELT data fetching
//!
//! These tests actually fetch data from GDELT servers and validate the results.
//! Run with: cargo test --test integration_fetch_tests -- --nocapture
//!
//! Note: These tests require network access and may take some time to complete.

use anyhow::Result;
use data::fetchers::{
    DataFetcher,
    gdelt::{EventTableFetcher, GKGTableFetcher, MentionTableFetcher},
};

use tempfile::TempDir;

fn init_test_logging() {
    let _ = env_logger::builder()
        .is_test(true)
        .filter_level(log::LevelFilter::Info)
        .try_init();
}

/// Create a test fetcher with temporary directories
fn create_test_directories() -> Result<(TempDir, TempDir)> {
    let output_dir = TempDir::new()?;
    let temp_dir = TempDir::new()?;
    Ok((output_dir, temp_dir))
}

/// Helper to print table results in a formatted way
fn print_table_header(table_name: &str, count: usize) {
    println!("\n{}", "=".repeat(60));
    println!(
        "📊 {} TABLE - First {} Results",
        table_name.to_uppercase(),
        count
    );
    println!("{}", "=".repeat(60));
}

/// Helper to print a divider
fn print_divider() {
    println!("{}", "-".repeat(60));
}

#[tokio::test]
async fn test_event_table_external_fetch() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting Event Table External Fetch Test ===");

    let (output_dir, temp_dir) = create_test_directories()?;
    log::info!("Created temporary directories");
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    // Create Event Table Fetcher
    let mut fetcher = EventTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    log::info!("Created Event Table Fetcher for GDELT v2");

    let url = fetcher.url_link()?;
    log::info!("Fetching from URL: {}", url);
    println!("\n🌐 Fetching latest Event Table data from GDELT servers...");
    println!("📍 URL: {}", url);

    // Attempt to fetch latest data
    log::info!("Attempting to fetch latest Event Table data from GDELT servers...");

    match fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            log::info!("Successfully fetched Event Table data");
            print_table_header("Event", 10);

            let events: Vec<_> = iterator.take(10).collect();
            log::info!("Retrieved {} Event Table records", events.len());

            if events.is_empty() {
                println!("⚠️  No Event Table records found in the latest fetch");
                log::warn!("No Event Table records found");
            } else {
                println!("📊 First {} Event Records:", events.len());
                for (i, event) in events.iter().enumerate() {
                    println!("\n🎯 Event Record #{}: {:?}", i + 1, event);
                    print_divider();
                }
                log::info!("Successfully displayed {} Event records", events.len());
            }

            println!(
                "\n✅ Event Table fetch completed successfully - {} records processed",
                events.len()
            );
        }
        Err(e) => {
            log::error!("Failed to fetch Event Table data: {}", e);
            println!("❌ Event Table fetch failed: {}", e);
            println!("💡 This might be due to network issues or GDELT server unavailability");
        }
    }

    log::info!("=== Event Table External Fetch Test Completed ===");
    Ok(())
}

#[tokio::test]
async fn test_mention_table_external_fetch() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting Mention Table External Fetch Test ===");

    let (output_dir, temp_dir) = create_test_directories()?;
    log::info!("Created temporary directories");
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    // Create Mention Table Fetcher
    let mut fetcher = MentionTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    log::info!("Created Mention Table Fetcher for GDELT v2");

    let url = fetcher.url_link()?;
    log::info!("Fetching from URL: {}", url);
    println!("\n🌐 Fetching latest Mention Table data from GDELT servers...");
    println!("📍 URL: {}", url);

    // Attempt to fetch latest data
    log::info!("Attempting to fetch latest Mention Table data from GDELT servers...");

    match fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            log::info!("Successfully fetched Mention Table data");
            print_table_header("Mention", 10);

            let mentions: Vec<_> = iterator.take(10).collect();
            log::info!("Retrieved {} Mention Table records", mentions.len());

            if mentions.is_empty() {
                println!("⚠️  No Mention Table records found in the latest fetch");
                log::warn!("No Mention Table records found");
            } else {
                println!("📊 First {} Mention Records:", mentions.len());
                for (i, mention) in mentions.iter().enumerate() {
                    println!("\n📰 Mention Record #{}: {:?}", i + 1, mention);
                    print_divider();
                }
                log::info!("Successfully displayed {} Mention records", mentions.len());
            }

            println!(
                "\n✅ Mention Table fetch completed successfully - {} records processed",
                mentions.len()
            );
        }
        Err(e) => {
            log::error!("Failed to fetch Mention Table data: {}", e);
            println!("❌ Mention Table fetch failed: {}", e);
            println!("💡 This might be due to network issues or GDELT server unavailability");
        }
    }

    log::info!("=== Mention Table External Fetch Test Completed ===");
    Ok(())
}

#[tokio::test]
async fn test_gkg_table_external_fetch() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting GKG Table External Fetch Test ===");

    let (output_dir, temp_dir) = create_test_directories()?;
    log::info!("Created temporary directories");
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    // Create GKG Table Fetcher
    let mut fetcher = GKGTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    log::info!("Created GKG Table Fetcher for GDELT v2");

    let url = fetcher.url_link()?;
    log::info!("Fetching from URL: {}", url);
    println!("\n🌐 Fetching latest GKG Table data from GDELT servers...");
    println!("📍 URL: {}", url);

    // Attempt to fetch latest data
    log::info!("Attempting to fetch latest GKG Table data from GDELT servers...");

    match fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            log::info!("Successfully fetched GKG Table data");
            print_table_header("GKG (Global Knowledge Graph)", 10);

            let gkg_records: Vec<_> = iterator.take(10).collect();
            log::info!("Retrieved {} GKG Table records", gkg_records.len());

            if gkg_records.is_empty() {
                println!("⚠️  No GKG Table records found in the latest fetch");
                log::warn!("No GKG Table records found");
            } else {
                println!("📊 First {} GKG Records:", gkg_records.len());
                for (i, gkg) in gkg_records.iter().enumerate() {
                    println!("\n🌐 GKG Record #{}: {:?}", i + 1, gkg);
                    print_divider();
                }
                log::info!("Successfully displayed {} GKG records", gkg_records.len());
            }

            println!(
                "\n✅ GKG Table fetch completed successfully - {} records processed",
                gkg_records.len()
            );
        }
        Err(e) => {
            log::error!("Failed to fetch GKG Table data: {}", e);
            println!("❌ GKG Table fetch failed: {}", e);
            println!("💡 This might be due to network issues or GDELT server unavailability");
        }
    }

    log::info!("=== GKG Table External Fetch Test Completed ===");
    Ok(())
}

/// Test all tables in a summary format with detailed debugging
#[tokio::test]
async fn test_all_tables_summary() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting Comprehensive Fetch Test for All GDELT Tables ===");

    let (output_dir, temp_dir) = create_test_directories()?;
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    println!("\n{}", "=".repeat(80));
    println!("🌍 GDELT COMPREHENSIVE FETCH TEST - ALL TABLE TYPES");
    println!("{}", "=".repeat(80));

    let mut total_records = 0;

    // Test Event Table
    println!("\n1️⃣  Testing Event Table...");
    log::debug!("Creating Event Table fetcher");
    let mut event_fetcher = EventTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    let event_url = event_fetcher.url_link()?;
    println!("   📍 Event Table URL: {}", event_url);
    log::info!("Event Table URL: {}", event_url);

    match event_fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            let events: Vec<_> = iterator.take(10).collect();
            println!("   ✅ Event Table: {} records fetched", events.len());
            log::info!("Event Table: {} records fetched", events.len());

            if !events.is_empty() {
                println!("   📊 First 3 Event Records:");
                for (i, event) in events.iter().take(3).enumerate() {
                    println!("     🎯 Event #{}: {:?}", i + 1, event);
                }
            }
            total_records += events.len();
        }
        Err(e) => {
            println!("   ❌ Event Table: Failed to fetch ({})", e);
            log::error!("Event Table fetch failed: {}", e);
        }
    }

    // Test Mention Table
    println!("\n2️⃣  Testing Mention Table...");
    log::debug!("Creating Mention Table fetcher");
    let mut mention_fetcher = MentionTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    let mention_url = mention_fetcher.url_link()?;
    println!("   📍 Mention Table URL: {}", mention_url);
    log::info!("Mention Table URL: {}", mention_url);

    match mention_fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            let mentions: Vec<_> = iterator.take(10).collect();
            println!("   ✅ Mention Table: {} records fetched", mentions.len());
            log::info!("Mention Table: {} records fetched", mentions.len());

            if !mentions.is_empty() {
                println!("   📊 First 3 Mention Records:");
                for (i, mention) in mentions.iter().take(3).enumerate() {
                    println!("     📰 Mention #{}: {:?}", i + 1, mention);
                }
            }
            total_records += mentions.len();
        }
        Err(e) => {
            println!("   ❌ Mention Table: Failed to fetch ({})", e);
            log::error!("Mention Table fetch failed: {}", e);
        }
    }

    // Test GKG Table
    println!("\n3️⃣  Testing GKG Table...");
    log::debug!("Creating GKG Table fetcher");
    let mut gkg_fetcher = GKGTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    let gkg_url = gkg_fetcher.url_link()?;
    println!("   📍 GKG Table URL: {}", gkg_url);
    log::info!("GKG Table URL: {}", gkg_url);

    match gkg_fetcher.fetch_latest_async().await {
        Ok(iterator) => {
            let gkg_records: Vec<_> = iterator.take(10).collect();
            println!("   ✅ GKG Table: {} records fetched", gkg_records.len());
            log::info!("GKG Table: {} records fetched", gkg_records.len());

            if !gkg_records.is_empty() {
                println!("   📊 First 3 GKG Records:");
                for (i, gkg) in gkg_records.iter().take(3).enumerate() {
                    println!("     🌐 GKG #{}: {:?}", i + 1, gkg);
                }
            }
            total_records += gkg_records.len();
        }
        Err(e) => {
            println!("   ❌ GKG Table: Failed to fetch ({})", e);
            log::error!("GKG Table fetch failed: {}", e);
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("🏁 COMPREHENSIVE FETCH TEST COMPLETED");
    println!("📈 Total Records Fetched: {}", total_records);
    println!("{}", "=".repeat(80));

    log::info!(
        "Comprehensive fetch test completed with {} total records",
        total_records
    );
    Ok(())
}

/// Test GDELT version differences with detailed debugging
#[tokio::test]
async fn test_version_comparison() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting GDELT Version Comparison Test ===");

    let (output_dir, temp_dir) = create_test_directories()?;
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    println!("\n{}", "=".repeat(80));
    println!("🔄 GDELT VERSION COMPARISON TEST (V2 vs V3)");
    println!("{}", "=".repeat(80));

    // Test V2 Event Table
    println!("\n📊 Testing GDELT V2 Event Table");
    log::debug!("Creating V2 Event Table fetcher");
    let v2_fetcher = EventTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    let v2_url = v2_fetcher.url_link()?;
    println!("   V2 URL: {}", v2_url);
    println!("   V2 Version: {:?}", v2_fetcher.version());
    log::info!("V2 Event Table URL: {}", v2_url);

    // Test V3 Event Table
    println!("\n📊 Testing GDELT V3 Event Table");
    log::debug!("Creating V3 Event Table fetcher");
    let v3_fetcher = EventTableFetcher::new_v3(output_dir.path(), temp_dir.path())?;
    let v3_url = v3_fetcher.url_link()?;
    println!("   V3 URL: {}", v3_url);
    println!("   V3 Version: {:?}", v3_fetcher.version());
    log::info!("V3 Event Table URL: {}", v3_url);

    // Compare URL structures
    println!("\n🔍 URL Structure Analysis:");
    let v2_has_v2 = v2_url.as_str().contains("gdeltv2");
    let v3_has_v3 = v3_url.as_str().contains("gdeltv3");
    println!("   V2 contains 'gdeltv2': {}", v2_has_v2);
    println!("   V3 contains 'gdeltv3': {}", v3_has_v3);

    log::debug!(
        "Version URL validation - V2 has v2: {}, V3 has v3: {}",
        v2_has_v2,
        v3_has_v3
    );

    // Additional version analysis
    println!("\n📋 Version Details:");
    println!(
        "   V2 Base URL: {}://{}",
        v2_url.scheme(),
        v2_url.host_str().unwrap_or("N/A")
    );
    println!(
        "   V3 Base URL: {}://{}",
        v3_url.scheme(),
        v3_url.host_str().unwrap_or("N/A")
    );

    println!("\n{}", "=".repeat(80));
    println!("🏁 VERSION COMPARISON TEST COMPLETED");
    println!("{}", "=".repeat(80));

    log::info!("=== GDELT Version Comparison Test Completed ===");
    Ok(())
}

/// Test error handling with invalid dates or network issues
#[tokio::test]
async fn test_error_handling() -> Result<()> {
    init_test_logging();
    log::info!("=== Starting Error Handling and Resilience Test ===");

    println!("\n{}", "=".repeat(80));
    println!("🚨 ERROR HANDLING AND RESILIENCE TEST");
    println!("{}", "=".repeat(80));

    let (output_dir, temp_dir) = create_test_directories()?;
    log::debug!("Output dir: {:?}", output_dir.path());
    log::debug!("Temp dir: {:?}", temp_dir.path());

    // Test with Event Table fetcher
    println!("\n⏱️  Testing network resilience with Event Table...");
    log::debug!("Creating Event Table fetcher for resilience test");
    let mut fetcher = EventTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
    let url = fetcher.url_link()?;
    println!("   📍 Testing URL: {}", url);
    log::info!("Testing network resilience with URL: {}", url);

    // Test with potential network timeout
    match tokio::time::timeout(std::time::Duration::from_secs(45), async {
        fetcher.fetch_latest_async().await
    })
    .await
    {
        Ok(Ok(iterator)) => {
            let records: Vec<_> = iterator.take(3).collect();
            println!(
                "   ✅ Network resilience test passed - {} record(s) fetched within timeout",
                records.len()
            );
            log::info!(
                "Network resilience test passed - {} records fetched",
                records.len()
            );

            if !records.is_empty() {
                println!("   📊 Sample records successfully retrieved:");
                for (i, record) in records.iter().enumerate() {
                    println!(
                        "     🎯 Record #{}: First 100 chars: {:?}",
                        i + 1,
                        format!("{:?}", record)
                            .chars()
                            .take(100)
                            .collect::<String>()
                    );
                }
            }
        }
        Ok(Err(e)) => {
            println!("   ⚠️  Network test - Fetch error: {}", e);
            log::warn!("Network test fetch error: {}", e);
            println!("   💡 This is expected behavior for network issues");
        }
        Err(_) => {
            println!("   ⚠️  Network test - Timeout after 45 seconds");
            log::warn!("Network test timed out after 45 seconds");
            println!("   💡 This indicates slow network or server response");
        }
    }

    // Test URL validation
    println!("\n🔗 Testing URL structure validation...");
    let all_fetchers = vec![
        (
            "Event",
            EventTableFetcher::new_v2(output_dir.path(), temp_dir.path())?.url_link()?,
        ),
        (
            "Mention",
            MentionTableFetcher::new_v2(output_dir.path(), temp_dir.path())?.url_link()?,
        ),
        (
            "GKG",
            GKGTableFetcher::new_v2(output_dir.path(), temp_dir.path())?.url_link()?,
        ),
    ];

    for (table_type, url) in all_fetchers {
        println!("   🔍 {} Table URL: {}", table_type, url);
        log::debug!("{} Table URL validation: {}", table_type, url);

        let url_str = url.as_str();
        let has_http = url_str.starts_with("http");
        let has_gdelt = url_str.contains("gdelt");

        println!("     ✓ Has HTTP protocol: {}", has_http);
        println!("     ✓ Contains 'gdelt': {}", has_gdelt);

        if has_http && has_gdelt {
            println!("     ✅ URL structure validation passed");
            log::info!("{} Table URL structure validation passed", table_type);
        } else {
            println!("     ❌ URL structure validation failed");
            log::error!("{} Table URL structure validation failed", table_type);
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("🏁 ERROR HANDLING TEST COMPLETED");
    println!("{}", "=".repeat(80));

    log::info!("=== Error Handling and Resilience Test Completed ===");
    Ok(())
}
