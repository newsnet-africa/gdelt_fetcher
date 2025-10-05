//! # Polite Fetching Example
//!
//! This example demonstrates the new local file checking feature that avoids
//! unnecessary API calls when data already exists locally.
//!
//! The fetcher will now:
//! 1. First check if the requested file exists in the output directory
//! 2. Fall back to checking the tmp directory
//! 3. Only make an API call if the file doesn't exist locally
//!
//! This is especially useful for:
//! - Development and testing (avoid hitting APIs repeatedly)
//! - Being polite to GDELT servers
//! - Faster iteration when working with the same datasets
//!
//! Run with: cargo run --example polite_fetching

use anyhow::Result;
use chrono::{TimeZone, Utc};
use gdelt_fetcher::api::{
    fetch_all_latest, fetch_events_by_date, fetch_latest_events, fetch_latest_mentions,
};

use std::fs;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to see the polite behavior in action
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("🤝 GDELT Polite Fetching Example");
    println!("================================");
    println!();

    // Example 1: Fetch latest events twice to demonstrate caching
    println!("📊 Example 1: Fetching latest events twice");
    println!("First call - will download from API:");
    let events1 = fetch_latest_events().await?;
    println!("✅ First call completed, got {} events", events1.len());

    println!("\nSecond call - should use local file:");
    let events2 = fetch_latest_events().await?;
    println!("✅ Second call completed, got {} events", events2.len());
    println!("Notice how the second call was much faster! 🚀");
    println!();

    // Example 2: Fetch different data types
    println!("📊 Example 2: Fetching different data types");
    let mentions = fetch_latest_mentions().await?;
    println!("✅ Fetched {} mentions", mentions.len());
    println!();

    // Example 3: Fetch historical data by date
    println!("📊 Example 3: Fetching historical data");
    let historical_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    println!(
        "Fetching events for {} (first time):",
        historical_date.format("%Y-%m-%d")
    );

    match fetch_events_by_date(historical_date).await {
        Ok(historical_events) => {
            println!("✅ Fetched {} historical events", historical_events.len());

            println!("\nFetching the same date again (should use cache):");
            match fetch_events_by_date(historical_date).await {
                Ok(cached_events) => {
                    println!("✅ Retrieved {} events from cache", cached_events.len());
                    println!(
                        "Files matched: {}",
                        historical_events.len() == cached_events.len()
                    );
                }
                Err(e) => println!("⚠️ Error on second fetch: {}", e),
            }
        }
        Err(e) => {
            println!("⚠️ Could not fetch historical data (this is normal): {}", e);
            println!("Historical data might not be available for this date.");
        }
    }
    println!();

    // Example 4: Batch fetching
    println!("📊 Example 4: Batch fetching all data types");
    println!("This will use cached files where available:");

    let start_time = std::time::Instant::now();
    let (batch_events, batch_mentions, batch_gkg) = fetch_all_latest().await?;
    let duration = start_time.elapsed();

    println!("✅ Batch fetch completed in {:?}:", duration);
    println!("  - Events: {}", batch_events.len());
    println!("  - Mentions: {}", batch_mentions.len());
    println!("  - GKG: {}", batch_gkg.len());
    println!();

    // Example 5: Show directory structure
    println!("📂 Example 5: Cached files location");
    show_cached_files()?;

    println!("🎉 Example completed!");
    println!();
    println!("Key benefits of polite fetching:");
    println!("  ✅ Faster development iteration");
    println!("  ✅ Reduced load on GDELT servers");
    println!("  ✅ Works offline after initial download");
    println!("  ✅ Automatically handles both tmp and output directories");

    Ok(())
}

/// Show the structure of cached files
fn show_cached_files() -> Result<()> {
    let dirs = ["./data", "./tmp"];

    for base_dir in &dirs {
        if let Ok(entries) = fs::read_dir(base_dir) {
            println!("📁 {}:", base_dir);
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(name) = path.file_name() {
                            println!("  📂 {}/", name.to_string_lossy());

                            // Show files in subdirectory
                            if let Ok(subentries) = fs::read_dir(&path) {
                                for subentry in subentries {
                                    if let Ok(subentry) = subentry {
                                        let subpath = subentry.path();
                                        if subpath.is_file() {
                                            if let Some(filename) = subpath.file_name() {
                                                let metadata = fs::metadata(&subpath)?;
                                                let size_mb = metadata.len() as f64 / 1_048_576.0;
                                                println!(
                                                    "    📄 {} ({:.1} MB)",
                                                    filename.to_string_lossy(),
                                                    size_mb
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            println!("📁 {} (not found)", base_dir);
        }
    }

    Ok(())
}
