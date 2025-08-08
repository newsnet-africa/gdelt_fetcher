//! # GDELT Fetcher
//!
//! A Rust library for fetching and parsing GDELT (Global Database of Events, Language, and Tone) data.
//!
//! This library provides a simple interface for downloading and parsing the three main GDELT data tables:
//! - Events: The main event table containing coded events
//! - Mentions: References to events in news articles
//! - GKG (Global Knowledge Graph): Enhanced semantic information about events
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gdelt_fetcher::{fetch_and_parse_events, fetch_and_parse_mentions, fetch_and_parse_gkg};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Fetch latest events
//!     let events = fetch_and_parse_events().await?;
//!     println!("Fetched {} events", events.len());
//!
//!     // Fetch latest mentions
//!     let mentions = fetch_and_parse_mentions().await?;
//!     println!("Fetched {} mentions", mentions.len());
//!
//!     // Fetch latest GKG data
//!     let gkg_data = fetch_and_parse_gkg().await?;
//!     println!("Fetched {} GKG records", gkg_data.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! For more control over the fetching process, use the fetchers directly:
//!
//! ```rust,no_run
//! use gdelt_fetcher::{EventTableFetcher, MentionTableFetcher, GKGTableFetcher};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let output_dir = "./data";
//!     let temp_dir = "./tmp";
//!
//!     // Create specialized fetchers for more control
//!     let mut event_fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;
//!     let events_iter = event_fetcher.fetch_latest_async().await?;
//!
//!     // Process events one by one instead of collecting all at once
//!     for event in events_iter.take(10) {
//!         println!("Event: {:?}", event);
//!     }
//!
//!     Ok(())
//! }
//! ```

use anyhow::Result;
use log::info;
use std::fs;

// Re-export main types and fetchers for convenience
pub use data::fetchers::{
    DataFetcher,
    gdelt::{
        CsvExtension, EventTableFetcher, FileExtension, GKGTableFetcher, GdeltFetcher,
        GdeltVersion, JsonExtension, MentionTableFetcher, TableType,
    },
};

pub use models::types::{
    event_table::EventTable, gkg_table::GKGTable, mention_table::MentionTable,
};

/// Create temporary directories for a given data type
fn setup_temp_directories(data_type: &str) -> Result<(String, String)> {
    let tmp_dir = format!("./tmp/{}", data_type);
    let output_dir = format!("./data/{}", data_type);

    info!(
        "📁 Creating directories - Output: {}, Temp: {}",
        output_dir, tmp_dir
    );

    // Create directories if they don't exist
    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(&tmp_dir)?;

    info!("✅ Directories created successfully");
    Ok((tmp_dir, output_dir))
}

/// Fetch and parse the latest mention data from GDELT
///
/// This is a convenience function that sets up temporary directories,
/// creates a MentionTableFetcher, and fetches the latest data.
///
/// # Returns
/// A vector of parsed MentionTable records
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::fetch_and_parse_mentions;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mentions = fetch_and_parse_mentions().await?;
///     println!("Fetched {} mention records", mentions.len());
///     Ok(())
/// }
/// ```
pub async fn fetch_and_parse_mentions() -> Result<Vec<MentionTable>> {
    info!("🚀 Starting fetch_and_parse_mentions()");
    let start_time = std::time::Instant::now();

    info!("📁 Setting up temporary directories for mentions");
    let (tmp_dir, output_dir) = setup_temp_directories("mentions")?;
    info!(
        "✅ Directories created - Output: {}, Temp: {}",
        output_dir, tmp_dir
    );

    info!("🔧 Creating MentionTableFetcher");
    let mut fetcher = MentionTableFetcher::new_v2(&output_dir, &tmp_dir)?;
    let url = fetcher.url_link()?;
    info!("📍 Fetching from URL: {}", url);

    info!("🌐 Starting async fetch of latest mention data");
    let mention_iterator = fetcher.fetch_latest_async().await?;

    info!("📊 Processing mention records...");
    let results: Vec<MentionTable> = mention_iterator.collect();
    let duration = start_time.elapsed();

    info!(
        "✅ Successfully fetched {} mention records in {:?}",
        results.len(),
        duration
    );
    Ok(results)
}

/// Fetch and parse the latest event data from GDELT
///
/// This is a convenience function that sets up temporary directories,
/// creates an EventTableFetcher, and fetches the latest data.
///
/// # Returns
/// A vector of parsed EventTable records
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::fetch_and_parse_events;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let events = fetch_and_parse_events().await?;
///     println!("Fetched {} event records", events.len());
///     Ok(())
/// }
/// ```
pub async fn fetch_and_parse_events() -> Result<Vec<EventTable>> {
    info!("🚀 Starting fetch_and_parse_events()");
    let start_time = std::time::Instant::now();

    info!("📁 Setting up temporary directories for events");
    let (tmp_dir, output_dir) = setup_temp_directories("events")?;
    info!(
        "✅ Directories created - Output: {}, Temp: {}",
        output_dir, tmp_dir
    );

    info!("🔧 Creating EventTableFetcher");
    let mut fetcher = EventTableFetcher::new_v2(&output_dir, &tmp_dir)?;
    let url = fetcher.url_link()?;
    info!("📍 Fetching from URL: {}", url);

    info!("🌐 Starting async fetch of latest event data");
    let event_iterator = fetcher.fetch_latest_async().await?;

    info!("📊 Processing event records...");
    let results: Vec<EventTable> = event_iterator.collect();
    let duration = start_time.elapsed();

    info!(
        "✅ Successfully fetched {} event records in {:?}",
        results.len(),
        duration
    );
    Ok(results)
}

/// Fetch and parse the latest GKG (Global Knowledge Graph) data from GDELT
///
/// This is a convenience function that sets up temporary directories,
/// creates a GKGTableFetcher, and fetches the latest data.
///
/// # Returns
/// A vector of parsed GKGTable records
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::fetch_and_parse_gkg;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let gkg_data = fetch_and_parse_gkg().await?;
///     println!("Fetched {} GKG records", gkg_data.len());
///     Ok(())
/// }
/// ```
pub async fn fetch_and_parse_gkg() -> Result<Vec<GKGTable>> {
    info!("🚀 Starting fetch_and_parse_gkg()");
    let start_time = std::time::Instant::now();

    info!("📁 Setting up temporary directories for gkg");
    let (tmp_dir, output_dir) = setup_temp_directories("gkg")?;
    info!(
        "✅ Directories created - Output: {}, Temp: {}",
        output_dir, tmp_dir
    );

    info!("🔧 Creating GKGTableFetcher");
    let mut fetcher = GKGTableFetcher::new_v2(&output_dir, &tmp_dir)?;
    let url = fetcher.url_link()?;
    info!("📍 Fetching from URL: {}", url);

    info!("🌐 Starting async fetch of latest GKG data");
    let gkg_iterator = fetcher.fetch_latest_async().await?;

    info!("📊 Processing GKG records...");
    let results: Vec<GKGTable> = gkg_iterator.collect();
    let duration = start_time.elapsed();

    info!(
        "✅ Successfully fetched {} GKG records in {:?}",
        results.len(),
        duration
    );
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_logger() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .format_timestamp_millis()
            .format_module_path(true)
            .format_target(true)
            .try_init();
    }

    /// Helper function to format duration in a human-readable way
    fn format_duration(duration: std::time::Duration) -> String {
        let total_ms = duration.as_millis();
        if total_ms < 1000 {
            format!("{}ms", total_ms)
        } else if total_ms < 60_000 {
            format!("{:.2}s", duration.as_secs_f64())
        } else {
            let minutes = total_ms / 60_000;
            let seconds = (total_ms % 60_000) as f64 / 1000.0;
            format!("{}m {:.1}s", minutes, seconds)
        }
    }

    /// Helper function to create formatted section headers
    fn print_section_header(title: &str, subtitle: Option<&str>) {
        println!("\n{}", "=".repeat(80));
        println!("🎯 {}", title.to_uppercase());
        if let Some(sub) = subtitle {
            println!("   {}", sub);
        }
        println!("{}", "=".repeat(80));
    }

    /// Helper function to print timing information
    fn print_timing_info(operation: &str, duration: std::time::Duration, count: usize) {
        let formatted_duration = format_duration(duration);
        println!(
            "⏱️  {} completed in {} ({} records)",
            operation, formatted_duration, count
        );
        log::info!(
            "{} completed in {:?} with {} records",
            operation,
            duration,
            count
        );
    }

    #[tokio::test]
    async fn test_fetch_and_parse_events() -> Result<()> {
        init_logger();
        print_section_header(
            "Event Table Fetch Test",
            Some("Testing main library function"),
        );

        let start_time = std::time::Instant::now();
        log::debug!("Calling fetch_and_parse_events() at {:?}", start_time);

        let events = fetch_and_parse_events().await?;
        let duration = start_time.elapsed();

        print_timing_info("Event Table fetch", duration, events.len());

        assert!(!events.is_empty(), "Should fetch at least some events");

        // Print first 10 events for verification
        println!("\n📊 First 10 Event Records:");
        println!("{}", "=".repeat(80));
        for (i, event) in events.iter().take(10).enumerate() {
            println!("🎯 Event Record #{}: {:?}", i + 1, event);
            println!("{}", "-".repeat(40));
        }
        println!("{}", "=".repeat(80));
        println!("📈 Total Events Fetched: {}", events.len());

        log::info!("=== Event Table Fetch Test Completed Successfully ===");
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_and_parse_mentions() -> Result<()> {
        init_logger();
        print_section_header(
            "Mention Table Fetch Test",
            Some("Testing main library function"),
        );

        let start_time = std::time::Instant::now();
        log::debug!("Calling fetch_and_parse_mentions() at {:?}", start_time);

        let mentions = fetch_and_parse_mentions().await?;
        let duration = start_time.elapsed();

        print_timing_info("Mention Table fetch", duration, mentions.len());

        assert!(!mentions.is_empty(), "Should fetch at least some mentions");

        // Print first 10 mentions for verification
        println!("\n📊 First 10 Mention Records:");
        println!("{}", "=".repeat(80));
        for (i, mention) in mentions.iter().take(10).enumerate() {
            println!("📰 Mention Record #{}: {:?}", i + 1, mention);
            println!("{}", "-".repeat(40));
        }
        println!("{}", "=".repeat(80));
        println!("📈 Total Mentions Fetched: {}", mentions.len());

        log::info!("=== Mention Table Fetch Test Completed Successfully ===");
        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_and_parse_gkg() -> Result<()> {
        init_logger();
        print_section_header(
            "GKG Table Fetch Test",
            Some("Testing main library function"),
        );

        let start_time = std::time::Instant::now();
        log::debug!("Calling fetch_and_parse_gkg() at {:?}", start_time);

        let gkg_data = fetch_and_parse_gkg().await?;
        let duration = start_time.elapsed();

        print_timing_info("GKG Table fetch", duration, gkg_data.len());

        assert!(!gkg_data.is_empty(), "Should fetch at least some GKG data");

        // Print first 10 GKG records for verification
        println!("\n📊 First 10 GKG Records:");
        println!("{}", "=".repeat(80));
        for (i, gkg) in gkg_data.iter().take(10).enumerate() {
            println!("🌐 GKG Record #{}: {:?}", i + 1, gkg);
            println!("{}", "-".repeat(40));
        }
        println!("{}", "=".repeat(80));
        println!("📈 Total GKG Records Fetched: {}", gkg_data.len());

        log::info!("=== GKG Table Fetch Test Completed Successfully ===");
        Ok(())
    }

    #[tokio::test]
    async fn test_all_table_types() -> Result<()> {
        init_logger();
        print_section_header(
            "Comprehensive All Table Types Test",
            Some("Testing all GDELT table types in sequence"),
        );

        let overall_start = std::time::Instant::now();
        log::debug!("Starting comprehensive test at {:?}", overall_start);

        // Test that all three main functions work
        log::debug!("Fetching Events...");
        let events_start = std::time::Instant::now();
        let events = fetch_and_parse_events().await?;
        let events_duration = events_start.elapsed();
        log::info!(
            "✅ Events fetched: {} records in {:?}",
            events.len(),
            events_duration
        );

        log::debug!("Fetching Mentions...");
        let mentions_start = std::time::Instant::now();
        let mentions = fetch_and_parse_mentions().await?;
        let mentions_duration = mentions_start.elapsed();
        log::info!(
            "✅ Mentions fetched: {} records in {:?}",
            mentions.len(),
            mentions_duration
        );

        log::debug!("Fetching GKG data...");
        let gkg_start = std::time::Instant::now();
        let gkg_data = fetch_and_parse_gkg().await?;
        let gkg_duration = gkg_start.elapsed();
        log::info!(
            "✅ GKG data fetched: {} records in {:?}",
            gkg_data.len(),
            gkg_duration
        );

        let total_duration = overall_start.elapsed();

        println!("\n🎉 COMPREHENSIVE FETCH SUMMARY:");
        println!("{}", "=".repeat(70));
        println!(
            "📊 Events:   {:>8} records (took {:>8?})",
            events.len(),
            events_duration
        );
        println!(
            "📰 Mentions: {:>8} records (took {:>8?})",
            mentions.len(),
            mentions_duration
        );
        println!(
            "🌐 GKG:      {:>8} records (took {:>8?})",
            gkg_data.len(),
            gkg_duration
        );
        println!("{}", "-".repeat(70));
        println!(
            "📈 Total:    {:>8} records (took {:>8?})",
            events.len() + mentions.len() + gkg_data.len(),
            total_duration
        );
        println!("{}", "=".repeat(70));

        // Print first 10 records from each table type
        println!("\n📋 FIRST 10 RECORDS FROM EACH TABLE:");

        if !events.is_empty() {
            println!("\n🎯 First 10 Events:");
            println!("{}", "-".repeat(50));
            for (i, event) in events.iter().take(10).enumerate() {
                println!("Event #{}: {:?}", i + 1, event);
            }
        }

        if !mentions.is_empty() {
            println!("\n📰 First 10 Mentions:");
            println!("{}", "-".repeat(50));
            for (i, mention) in mentions.iter().take(10).enumerate() {
                println!("Mention #{}: {:?}", i + 1, mention);
            }
        }

        if !gkg_data.is_empty() {
            println!("\n🌐 First 10 GKG Records:");
            println!("{}", "-".repeat(50));
            for (i, gkg) in gkg_data.iter().take(10).enumerate() {
                println!("GKG #{}: {:?}", i + 1, gkg);
            }
        }
        println!("{}", "=".repeat(70));

        println!("\n🎯 PERFORMANCE SUMMARY:");
        println!(
            "⚡ Fastest fetch: {}",
            format_duration(
                [events_duration, mentions_duration, gkg_duration]
                    .iter()
                    .min()
                    .unwrap()
                    .clone()
            )
        );
        println!(
            "🐌 Slowest fetch: {}",
            format_duration(
                [events_duration, mentions_duration, gkg_duration]
                    .iter()
                    .max()
                    .unwrap()
                    .clone()
            )
        );
        println!(
            "📊 Average time per table: {}",
            format_duration(total_duration / 3)
        );

        log::info!(
            "=== All Table Types Test Completed Successfully in {} ===",
            format_duration(total_duration)
        );
        Ok(())
    }
}
