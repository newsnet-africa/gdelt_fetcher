use anyhow::Result;
use data::fetchers::gdelt::{GdeltVersionForMasterfilelist, MasterfilelistFetcher};
use models::types::masterfilelist::TableType;
use std::env;

/// Fetch and display GDELT masterfilelist entries
///
/// This example demonstrates the masterfilelist fetcher which maintains
/// a persistent store of all available GDELT files and their metadata.
///
/// Usage: cargo run --example fetch_masterfilelist [count] [v2|geg]
///
/// Arguments:
///   count   - Number of items to display (default: 10)
///   version - GDELT version: "v2" or "geg" (default: v2)
///
/// Example:
///   cargo run --example fetch_masterfilelist 20 v2
///   cargo run --example fetch_masterfilelist 15 geg
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Get count and version from command line args
    let args: Vec<String> = env::args().collect();

    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };

    let version = if args.len() > 2 {
        match args[2].to_lowercase().as_str() {
            "geg" | "v3" => GdeltVersionForMasterfilelist::V3Geg,
            _ => GdeltVersionForMasterfilelist::V2,
        }
    } else {
        GdeltVersionForMasterfilelist::V2
    };

    println!("{}", "=".repeat(80));
    println!("GDELT Masterfilelist Fetcher");
    println!("{}", "=".repeat(80));
    println!("Version: {:?}", version);
    println!("Store directory: ./tmp/masterlist_store\n");

    // Create fetcher
    let store_dir = "./tmp/masterlist_store";
    let fetcher = MasterfilelistFetcher::new(store_dir, version)?;

    // Check if store is already initialized
    let existing_count = fetcher.count()?;

    if existing_count == 0 {
        println!("Initializing masterfilelist (this will download the full list)...");
        println!("This may take several minutes on first run...");
        let init_count = fetcher.initialize_store().await?;
        println!("✅ Initialized with {} entries\n", init_count);
    } else {
        println!("📦 Found existing store with {} entries", existing_count);
        println!("Checking for updates...");
        let added = fetcher.update_store().await?;
        if added > 0 {
            println!("✅ Added {} new entries\n", added);
        } else {
            println!("✅ Already up to date\n");
        }
    }

    // Show latest entries for each table type
    println!("{}", "━".repeat(80));
    println!("Latest Available Files:");
    println!("{}", "━".repeat(80));

    for table_type in [TableType::Export, TableType::Mentions, TableType::Gkg] {
        if let Some(latest) = fetcher.get_latest(table_type, false)? {
            println!("\n  📊 {} Table:", table_type.as_str().to_uppercase());
            println!("    Timestamp: {}", latest.timestamp);
            println!("    Size: {} bytes ({:.2} MB)", latest.size, latest.size as f64 / 1024.0 / 1024.0);
            println!("    Hash: {}", latest.hash);
            println!("    URL: {}", latest.url);
        }
    }

    // Get all entries for the specified table type and display some
    println!("\n\n{}", "━".repeat(80));
    println!("Recent Export Table Files ({}):", count);
    println!("{}", "━".repeat(80));

    let export_entries = fetcher.get_by_table_type(TableType::Export)?;

    // Sort by timestamp descending (newest first)
    let mut sorted_entries = export_entries.clone();
    sorted_entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for (i, entry) in sorted_entries.iter().take(count).enumerate() {
        println!("\n  {}. {}", i + 1, entry.timestamp);
        println!("     Size: {:.2} MB", entry.size as f64 / 1024.0 / 1024.0);
        println!("     URL: {}", entry.url);
    }

    // Show statistics
    println!("\n\n{}", "=".repeat(80));
    println!("Statistics:");
    println!("{}", "=".repeat(80));

    let total = fetcher.count()?;
    let export_count = fetcher.get_by_table_type(TableType::Export)?.len();
    let mentions_count = fetcher.get_by_table_type(TableType::Mentions)?.len();
    let gkg_count = fetcher.get_by_table_type(TableType::Gkg)?.len();

    println!("  Total files in store: {}", total);
    println!("  Export files: {}", export_count);
    println!("  Mentions files: {}", mentions_count);
    println!("  GKG files: {}", gkg_count);

    let timestamps = fetcher.all_timestamps()?;
    if let (Some(first), Some(last)) = (timestamps.first(), timestamps.last()) {
        println!("  Date range: {} to {}", first, last);
        println!("  Total timestamps: {}", timestamps.len());
    }

    println!("{}", "=".repeat(80));

    // Example: Fetch a specific file
    println!("\nDemo: Fetching and verifying a file...");
    if let Some(latest_export) = fetcher.get_latest(TableType::Export, false)? {
        println!("Downloading: {}", latest_export.url);
        println!("Expected size: {} bytes", latest_export.size);

        match fetcher.fetch_file(&latest_export).await {
            Ok(data) => {
                println!("✅ Successfully downloaded and verified!");
                println!("   Downloaded: {} bytes", data.len());
                println!("   Hash verified: ✓");
                println!("   Size verified: ✓");
            }
            Err(e) => {
                println!("❌ Failed to download: {}", e);
            }
        }
    }

    println!("\n{}", "=".repeat(80));

    Ok(())
}
