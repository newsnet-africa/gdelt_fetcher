//! Comprehensive example demonstrating the new GDELT fetcher system
//!
//! This example shows how to:
//! - Fetch different types of GDELT data (Events, Mentions, GKG)
//! - Use both GDELT v2 and v3 versions
//! - Fetch latest data vs historical data by date
//! - Work with translation files
//! - Use custom file formats (CSV vs JSON)
//! - Build URLs programmatically
//! - Handle errors gracefully

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use data::fetchers::gdelt::{
    EventTableFetcher, FileExtension, GKGTableFetcher, GdeltFetcher, GdeltUrlBuilder, GdeltVersion,
    JsonExtension, MentionTableFetcher, TableType, TableTypeConfig,
};
use log::{error, info, warn};
use std::path::Path;
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger to see what's happening
    env_logger::init();

    println!("🚀 GDELT Data Fetcher Examples");
    println!("==============================\n");

    // Create temporary directories for this example
    let output_dir = TempDir::new()?;
    let temp_dir = TempDir::new()?;

    println!("📁 Using temporary directories:");
    println!("   Output: {:?}", output_dir.path());
    println!("   Temp:   {:?}\n", temp_dir.path());

    // Example 1: Basic usage - Fetch latest events from GDELT v2
    println!("📊 Example 1: Fetch Latest Events (GDELT v2)");
    match fetch_latest_events_v2(output_dir.path(), temp_dir.path()).await {
        Ok(count) => println!("   ✅ Successfully fetched {} event records\n", count),
        Err(e) => println!("   ❌ Failed to fetch events: {}\n", e),
    }

    // Example 2: Fetch latest mentions from GDELT v3
    println!("📝 Example 2: Fetch Latest Mentions (GDELT v3)");
    match fetch_latest_mentions_v3(output_dir.path(), temp_dir.path()).await {
        Ok(count) => println!("   ✅ Successfully fetched {} mention records\n", count),
        Err(e) => println!("   ❌ Failed to fetch mentions: {}\n", e),
    }

    // Example 3: Fetch GKG data with translation
    println!("🌍 Example 3: Fetch GKG Data with Translation");
    match fetch_gkg_with_translation(output_dir.path(), temp_dir.path()).await {
        Ok(count) => println!(
            "   ✅ Successfully fetched {} GKG translation records\n",
            count
        ),
        Err(e) => println!("   ❌ Failed to fetch GKG translations: {}\n", e),
    }

    // Example 4: Fetch historical data by date
    println!("📅 Example 4: Fetch Historical Data by Date");
    let historical_date = DateTime::from_timestamp(1640995200, 0) // 2022-01-01 00:00:00 UTC
        .unwrap()
        .naive_utc();
    match fetch_historical_events(output_dir.path(), temp_dir.path(), historical_date).await {
        Ok(count) => println!(
            "   ✅ Successfully fetched {} historical event records\n",
            count
        ),
        Err(e) => println!("   ❌ Failed to fetch historical events: {}\n", e),
    }

    // Example 5: URL building and validation
    println!("🔗 Example 5: URL Building and Validation");
    demonstrate_url_building().await?;

    // Example 6: Working with custom table configurations
    println!("⚙️  Example 6: Custom Table Configurations");
    demonstrate_custom_configs(output_dir.path(), temp_dir.path()).await?;

    // Example 7: Direct GDELT fetcher usage
    println!("🛠️  Example 7: Direct GDELT Fetcher Usage");
    demonstrate_direct_fetcher_usage(output_dir.path(), temp_dir.path()).await?;

    // Example 8: Error handling and recovery
    println!("🔧 Example 8: Error Handling and Recovery");
    demonstrate_error_handling(output_dir.path(), temp_dir.path()).await?;

    println!("🎉 All examples completed!");

    Ok(())
}

/// Example 1: Fetch latest events from GDELT v2
async fn fetch_latest_events_v2<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<usize> {
    let mut fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;

    info!("Fetching latest events from GDELT v2...");
    let events_iterator = fetcher.fetch_latest_async().await?;

    let mut count = 0;
    for event in events_iterator.take(10) {
        // Just take first 10 for demo
        count += 1;
        println!("   Event {}: {:?}", count, event.global_event_id);
    }

    Ok(count)
}

/// Example 2: Fetch latest mentions from GDELT v3
async fn fetch_latest_mentions_v3<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<usize> {
    let mut fetcher = MentionTableFetcher::new_v3(output_dir, temp_dir)?;

    info!("Fetching latest mentions from GDELT v3...");
    let mentions_iterator = fetcher.fetch_latest_async().await?;

    let mut count = 0;
    for mention in mentions_iterator.take(5) {
        // Just take first 5 for demo
        count += 1;
        println!(
            "   Mention {}: Event ID {:?}",
            count, mention.global_event_id
        );
    }

    Ok(count)
}

/// Example 3: Fetch GKG data with translation
async fn fetch_gkg_with_translation<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<usize> {
    let mut fetcher = GKGTableFetcher::new_v2(output_dir, temp_dir)?.with_translation(true);

    info!("Fetching GKG translation data from GDELT v2...");
    let gkg_iterator = fetcher.fetch_latest_async().await?;

    let mut count = 0;
    for gkg_record in gkg_iterator.take(3) {
        // Just take first 3 for demo
        count += 1;
        println!(
            "   GKG Translation Record {}: {:?}",
            count, gkg_record.global_knowledge_graph_id
        );
    }

    Ok(count)
}

/// Example 4: Fetch historical data by specific date
async fn fetch_historical_events<P: AsRef<Path>>(
    output_dir: P,
    temp_dir: P,
    date: NaiveDateTime,
) -> Result<usize> {
    let mut fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;

    info!("Fetching historical events for date: {}", date);
    match fetcher.fetch_date_async(date).await {
        Ok(events_iterator) => {
            let mut count = 0;
            for event in events_iterator.take(5) {
                // Just take first 5 for demo
                count += 1;
                println!("   Historical Event {}: {:?}", count, event.global_event_id);
            }
            Ok(count)
        }
        Err(e) => {
            warn!("No historical data available for {}: {}", date, e);
            Ok(0)
        }
    }
}

/// Example 5: Demonstrate URL building capabilities
async fn demonstrate_url_building() -> Result<()> {
    let timestamp = DateTime::from_timestamp(1704067200, 0) // 2024-01-01 00:00:00 UTC
        .unwrap()
        .naive_utc();

    // Build different types of URLs
    let urls = vec![
        // GDELT v2 Events
        GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V2)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Export)
            .build()?,
        // GDELT v3 Mentions
        GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V3)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Mentions)
            .build()?,
        // GDELT v2 GKG with Translation
        GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V2)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Gkg)
            .with_translation(true)
            .build()?,
    ];

    for (i, url) in urls.iter().enumerate() {
        println!("   URL {}: {}", i + 1, url);
    }
    println!();

    Ok(())
}

/// Example 6: Demonstrate custom table configurations
async fn demonstrate_custom_configs<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<()> {
    let fetcher = GdeltFetcher::new_v2(output_dir, temp_dir)?;

    // Create custom table configurations for JSON format
    let export_json_config = TableType::export_json();
    let mentions_json_config = TableType::mentions_json();
    let gkg_json_config = TableType::gkg_json();

    println!("   Custom Configurations:");
    println!("   - Export JSON: {:?}", export_json_config);
    println!("   - Mentions JSON: {:?}", mentions_json_config);
    println!("   - GKG JSON: {:?}", gkg_json_config);

    // You could use these configs with fetcher.fetch_table_data_with_config()
    // if the GDELT project provided JSON format files

    println!();
    Ok(())
}

/// Example 7: Direct GDELT fetcher usage for advanced scenarios
async fn demonstrate_direct_fetcher_usage<P: AsRef<Path>>(
    output_dir: P,
    temp_dir: P,
) -> Result<()> {
    let fetcher = GdeltFetcher::new_v2(output_dir, temp_dir)?;

    // Get latest file list
    println!("   Fetching latest file list...");
    match fetcher.fetch_latest_file_list().await {
        Ok(entries) => {
            println!("   Found {} available files:", entries.len());
            for (i, entry) in entries.iter().take(3).enumerate() {
                println!(
                    "     {}. {} ({} bytes, type: {:?})",
                    i + 1,
                    entry.url.split('/').last().unwrap_or("unknown"),
                    entry.size,
                    entry.table_type
                );
            }
        }
        Err(e) => warn!("   Failed to fetch file list: {}", e),
    }

    // Show version capabilities
    println!("   Fetcher version: {:?}", fetcher.version());
    println!("   Output directory: {:?}", fetcher.file_path()?);

    println!();
    Ok(())
}

/// Example 8: Error handling and recovery strategies
async fn demonstrate_error_handling<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<()> {
    println!("   Demonstrating error handling strategies:");

    // Try to fetch with invalid configuration (this should gracefully handle errors)
    let mut fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;

    // Attempt to fetch very old data that might not exist
    let very_old_date = DateTime::from_timestamp(946684800, 0) // 2000-01-01
        .unwrap()
        .naive_utc();

    match fetcher.fetch_date_async(very_old_date).await {
        Ok(iterator) => {
            let count: usize = iterator.count();
            println!(
                "   ✅ Unexpectedly found {} records from {}",
                count, very_old_date
            );
        }
        Err(e) => {
            println!(
                "   ✅ Gracefully handled expected error for old date: {}",
                e
            );
        }
    }

    // Show URL validation
    let test_url = "http://data.gdeltproject.org/gdeltv2/20240101000000.export.CSV.zip";
    let compatible = fetcher.gdelt_fetcher().is_url_compatible(test_url);
    println!(
        "   URL compatibility check for '{}': {}",
        test_url, compatible
    );

    println!();
    Ok(())
}

/// Helper function to demonstrate record processing
fn process_events_sample<I>(events: I, sample_size: usize) -> usize
where
    I: Iterator<Item = models::types::event_table::EventTable>,
{
    let mut count = 0;
    for event in events.take(sample_size) {
        count += 1;
        // In a real application, you'd process each event here
        info!("Processing event: {:?}", event.global_event_id);
    }
    count
}

/// Helper function to demonstrate record filtering
fn filter_events_by_criteria<I>(events: I) -> Vec<models::types::event_table::EventTable>
where
    I: Iterator<Item = models::types::event_table::EventTable>,
{
    events
        .filter(|event| {
            // Example filter: only events with specific criteria
            // In a real app, you'd implement your business logic here
            true // For demo, accept all events
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = TempDir::new().unwrap();

        let result = EventTableFetcher::new_v2(output_dir.path(), temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_url_building() {
        let timestamp = DateTime::from_timestamp(1704067200, 0).unwrap().naive_utc();

        let url = GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V2)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Export)
            .build()
            .unwrap();

        assert!(url.as_str().contains("gdeltv2"));
        assert!(url.as_str().contains("export"));
        assert!(url.as_str().contains("20240101000000"));
    }
}
