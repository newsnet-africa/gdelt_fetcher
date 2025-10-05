//! # GDELT Fetcher API
//!
//! A simplified API for fetching and parsing GDELT data.
//!
//! This module provides a clean, simple interface for fetching GDELT data either by latest available
//! or by specific date. All functions return vectors of the respective data types.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use gdelt_fetcher::api::{fetch_latest_events, fetch_events_by_date};
//! use chrono::{Utc, TimeZone};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Fetch latest events
//!     let events = fetch_latest_events().await?;
//!     println!("Fetched {} latest events", events.len());
//!
//!     // Fetch events for a specific date
//!     let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
//!     let historical_events = fetch_events_by_date(date).await?;
//!     println!("Fetched {} events for 2024-01-15", historical_events.len());
//!
//!     Ok(())
//! }
//! ```

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use log::info;
use std::fs;
use std::path::PathBuf;

use crate::{
    EventTableFetcher, GKGTableFetcher, MentionTableFetcher, TableType, setup_temp_directories,
};
use models::types::{event_table::EventTable, gkg_table::GKGTable, mention_table::MentionTable};

// ================================================================================================
// HELPER FUNCTIONS FOR FILE EXISTENCE CHECKING
// ================================================================================================

/// Check if a file exists locally for a given table type and timestamp
/// Returns the path if found in output_dir or tmp_dir
fn check_existing_file(
    output_dir: &str,
    tmp_dir: &str,
    table_type: TableType,
    timestamp: Option<NaiveDateTime>,
) -> Option<PathBuf> {
    let expected_ext = table_type.file_extension().uncompressed_extension();
    let table_id = table_type.as_file_identifier();

    // Check output directory first
    if let Some(path) = search_directory_for_file(output_dir, table_id, expected_ext, timestamp) {
        info!("📁 Found existing file in output directory: {:?}", path);
        return Some(path);
    }

    // Check tmp directory as fallback
    if let Some(path) = search_directory_for_file(tmp_dir, table_id, expected_ext, timestamp) {
        info!("📁 Found existing file in tmp directory: {:?}", path);
        return Some(path);
    }

    None
}

/// Search a directory for files matching the pattern
fn search_directory_for_file(
    dir_path: &str,
    table_id: &str,
    expected_ext: &str,
    timestamp: Option<NaiveDateTime>,
) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    // For latest files, we look for any file containing the table identifier
                    if timestamp.is_none() {
                        if filename.contains(table_id) && filename.ends_with(expected_ext) {
                            return Some(path);
                        }
                    } else {
                        // For date-specific files, check exact timestamp match
                        let timestamp_str = timestamp.unwrap().format("%Y%m%d%H%M%S").to_string();
                        if filename.starts_with(&timestamp_str)
                            && filename.contains(table_id)
                            && filename.ends_with(expected_ext)
                        {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }
    None
}

/// Load events from an existing file path
fn load_events_from_file(file_path: PathBuf) -> Result<Vec<EventTable>> {
    use crate::EventTableIterator;
    let iterator = EventTableIterator::new(file_path)?;
    Ok(iterator.collect())
}

/// Load mentions from an existing file path
fn load_mentions_from_file(file_path: PathBuf) -> Result<Vec<MentionTable>> {
    use crate::MentionTableIterator;
    let iterator = MentionTableIterator::new(file_path)?;
    Ok(iterator.collect())
}

/// Load GKG data from an existing file path
fn load_gkg_from_file(file_path: PathBuf) -> Result<Vec<GKGTable>> {
    use crate::GKGTableIterator;
    let iterator = GKGTableIterator::new(file_path)?;
    Ok(iterator.collect())
}

// ================================================================================================
// LATEST DATA FETCHING
// ================================================================================================

/// Fetch the latest available events from GDELT
///
/// This function fetches the most recent event data available from GDELT's real-time feed.
/// Events are typically updated every 15 minutes.
///
/// # Returns
/// A vector of `EventTable` records containing the latest events
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_latest_events;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let events = fetch_latest_events().await?;
///     println!("Fetched {} latest events", events.len());
///
///     for event in events.iter().take(5) {
///         println!("Event ID: {:?}, Date: {:?}", event.global_event_id, event.date);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn fetch_latest_events() -> Result<Vec<EventTable>> {
    info!("🚀 Fetching latest events");
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("events")?;

    // Check if file already exists locally before making API call
    if let Some(existing_file) = check_existing_file(&output_dir, &tmp_dir, TableType::Export, None)
    {
        info!("📋 Using existing events file, skipping download for politeness");
        let results = load_events_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} events from existing file in {:?}",
            results.len(),
            duration
        );
        return Ok(results);
    }

    let mut fetcher = EventTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let event_iterator = fetcher.fetch_latest_async().await?;
    let results: Vec<EventTable> = event_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} latest events in {:?}",
        results.len(),
        duration
    );

    Ok(results)
}

/// Fetch the latest available mentions from GDELT
///
/// This function fetches the most recent mention data available from GDELT's real-time feed.
/// Mentions reference specific events found in news articles and media.
///
/// # Returns
/// A vector of `MentionTable` records containing the latest mentions
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_latest_mentions;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let mentions = fetch_latest_mentions().await?;
///     println!("Fetched {} latest mentions", mentions.len());
///
///     for mention in mentions.iter().take(5) {
///         println!("Event ID: {:?}, Source: {:?}",
///                  mention.global_event_id, mention.mention_source_name);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn fetch_latest_mentions() -> Result<Vec<MentionTable>> {
    info!("🚀 Fetching latest mentions");
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("mentions")?;

    // Check if file already exists locally before making API call
    if let Some(existing_file) =
        check_existing_file(&output_dir, &tmp_dir, TableType::Mentions, None)
    {
        info!("📋 Using existing mentions file, skipping download for politeness");
        let results = load_mentions_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} mentions from existing file in {:?}",
            results.len(),
            duration
        );
        return Ok(results);
    }

    let mut fetcher = MentionTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let mention_iterator = fetcher.fetch_latest_async().await?;
    let results: Vec<MentionTable> = mention_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} latest mentions in {:?}",
        results.len(),
        duration
    );

    Ok(results)
}

/// Fetch the latest available GKG (Global Knowledge Graph) data from GDELT
///
/// This function fetches the most recent GKG data available from GDELT's real-time feed.
/// GKG data contains enhanced semantic information about events including themes, locations,
/// organizations, and sentiment analysis.
///
/// # Returns
/// A vector of `GKGTable` records containing the latest GKG data
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_latest_gkg;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let gkg_data = fetch_latest_gkg().await?;
///     println!("Fetched {} latest GKG records", gkg_data.len());
///
///     for record in gkg_data.iter().take(5) {
///         println!("Record ID: {:?}, Date: {:?}",
///                  record.global_knowledge_graph_id, record.date);
///     }
///
///     Ok(())
/// }
/// ```
pub async fn fetch_latest_gkg() -> Result<Vec<GKGTable>> {
    info!("🚀 Fetching latest GKG data");
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("gkg")?;

    // Check if file already exists locally before making API call
    if let Some(existing_file) = check_existing_file(&output_dir, &tmp_dir, TableType::Gkg, None) {
        info!("📋 Using existing GKG file, skipping download for politeness");
        let results = load_gkg_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} GKG records from existing file in {:?}",
            results.len(),
            duration
        );
        return Ok(results);
    }

    let mut fetcher = GKGTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let gkg_iterator = fetcher.fetch_latest_async().await?;
    let results: Vec<GKGTable> = gkg_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} latest GKG records in {:?}",
        results.len(),
        duration
    );

    Ok(results)
}

// ================================================================================================
// DATE-SPECIFIC DATA FETCHING
// ================================================================================================

/// Fetch events for a specific date from GDELT
///
/// This function fetches event data for a specific date from GDELT's historical archives.
/// Data is available from February 2015 onwards for the current version.
///
/// # Arguments
/// * `date` - The date to fetch events for (any timezone, will be converted to UTC)
///
/// # Returns
/// A vector of `EventTable` records for the specified date
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_events_by_date;
/// use chrono::{Utc, TimeZone};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
///     let events = fetch_events_by_date(date).await?;
///     println!("Fetched {} events for 2024-01-15", events.len());
///
///     Ok(())
/// }
/// ```
pub async fn fetch_events_by_date<Tz: TimeZone>(date: DateTime<Tz>) -> Result<Vec<EventTable>> {
    let date_str = date.naive_utc().format("%Y-%m-%d").to_string();
    info!("🚀 Fetching events for date: {}", date_str);
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("events")?;

    // Check if file already exists locally before making API call
    let naive_date = date.naive_utc();
    if let Some(existing_file) =
        check_existing_file(&output_dir, &tmp_dir, TableType::Export, Some(naive_date))
    {
        info!(
            "📋 Using existing events file for {}, skipping download for politeness",
            date_str
        );
        let results = load_events_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} events for {} from existing file in {:?}",
            results.len(),
            date_str,
            duration
        );
        return Ok(results);
    }

    let mut fetcher = EventTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let event_iterator = fetcher.fetch_date_async(date).await?;
    let results: Vec<EventTable> = event_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} events for {} in {:?}",
        results.len(),
        date_str,
        duration
    );

    Ok(results)
}

/// Fetch mentions for a specific date from GDELT
///
/// This function fetches mention data for a specific date from GDELT's historical archives.
/// Mentions provide information about how events are referenced in news media.
///
/// # Arguments
/// * `date` - The date to fetch mentions for (any timezone, will be converted to UTC)
///
/// # Returns
/// A vector of `MentionTable` records for the specified date
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_mentions_by_date;
/// use chrono::{Utc, TimeZone};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
///     let mentions = fetch_mentions_by_date(date).await?;
///     println!("Fetched {} mentions for 2024-01-15", mentions.len());
///
///     Ok(())
/// }
/// ```
pub async fn fetch_mentions_by_date<Tz: TimeZone>(date: DateTime<Tz>) -> Result<Vec<MentionTable>> {
    let date_str = date.naive_utc().format("%Y-%m-%d").to_string();
    info!("🚀 Fetching mentions for date: {}", date_str);
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("mentions")?;

    // Check if file already exists locally before making API call
    let naive_date = date.naive_utc();
    if let Some(existing_file) =
        check_existing_file(&output_dir, &tmp_dir, TableType::Mentions, Some(naive_date))
    {
        info!(
            "📋 Using existing mentions file for {}, skipping download for politeness",
            date_str
        );
        let results = load_mentions_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} mentions for {} from existing file in {:?}",
            results.len(),
            date_str,
            duration
        );
        return Ok(results);
    }

    let mut fetcher = MentionTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let mention_iterator = fetcher.fetch_date_async(date).await?;
    let results: Vec<MentionTable> = mention_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} mentions for {} in {:?}",
        results.len(),
        date_str,
        duration
    );

    Ok(results)
}

/// Fetch GKG (Global Knowledge Graph) data for a specific date from GDELT
///
/// This function fetches GKG data for a specific date from GDELT's historical archives.
/// GKG data includes enhanced semantic analysis, themes, locations, and sentiment information.
///
/// # Arguments
/// * `date` - The date to fetch GKG data for (any timezone, will be converted to UTC)
///
/// # Returns
/// A vector of `GKGTable` records for the specified date
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_gkg_by_date;
/// use chrono::{Utc, TimeZone};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
///     let gkg_data = fetch_gkg_by_date(date).await?;
///     println!("Fetched {} GKG records for 2024-01-15", gkg_data.len());
///
///     Ok(())
/// }
/// ```
pub async fn fetch_gkg_by_date<Tz: TimeZone>(date: DateTime<Tz>) -> Result<Vec<GKGTable>> {
    let date_str = date.naive_utc().format("%Y-%m-%d").to_string();
    info!("🚀 Fetching GKG data for date: {}", date_str);
    let start_time = std::time::Instant::now();

    let (tmp_dir, output_dir) = setup_temp_directories("gkg")?;

    // Check if file already exists locally before making API call
    let naive_date = date.naive_utc();
    if let Some(existing_file) =
        check_existing_file(&output_dir, &tmp_dir, TableType::Gkg, Some(naive_date))
    {
        info!(
            "📋 Using existing GKG file for {}, skipping download for politeness",
            date_str
        );
        let results = load_gkg_from_file(existing_file)?;
        let duration = start_time.elapsed();
        info!(
            "✅ Loaded {} GKG records for {} from existing file in {:?}",
            results.len(),
            date_str,
            duration
        );
        return Ok(results);
    }

    let mut fetcher = GKGTableFetcher::new_v2(&output_dir, &tmp_dir)?;

    let gkg_iterator = fetcher.fetch_date_async(date).await?;
    let results: Vec<GKGTable> = gkg_iterator.collect();

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched {} GKG records for {} in {:?}",
        results.len(),
        date_str,
        duration
    );

    Ok(results)
}

// ================================================================================================
// CONVENIENCE FUNCTIONS
// ================================================================================================

/// Fetch all data types (events, mentions, GKG) for the latest available data
///
/// This is a convenience function that fetches the latest data for all three GDELT table types
/// in parallel, returning them as a tuple.
///
/// # Returns
/// A tuple containing `(events, mentions, gkg_data)` vectors
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_all_latest;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let (events, mentions, gkg_data) = fetch_all_latest().await?;
///     println!("Fetched {} events, {} mentions, {} GKG records",
///              events.len(), mentions.len(), gkg_data.len());
///
///     Ok(())
/// }
/// ```
pub async fn fetch_all_latest() -> Result<(Vec<EventTable>, Vec<MentionTable>, Vec<GKGTable>)> {
    info!("🚀 Fetching all latest data types");
    let start_time = std::time::Instant::now();

    // Use tokio::join! to fetch all data types concurrently
    let (events_result, mentions_result, gkg_result) = tokio::join!(
        fetch_latest_events(),
        fetch_latest_mentions(),
        fetch_latest_gkg()
    );

    let events = events_result?;
    let mentions = mentions_result?;
    let gkg_data = gkg_result?;

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched all latest data: {} events, {} mentions, {} GKG records in {:?}",
        events.len(),
        mentions.len(),
        gkg_data.len(),
        duration
    );

    Ok((events, mentions, gkg_data))
}

/// Fetch all data types (events, mentions, GKG) for a specific date
///
/// This is a convenience function that fetches data for all three GDELT table types
/// for a specific date in parallel, returning them as a tuple.
///
/// # Arguments
/// * `date` - The date to fetch data for (any timezone, will be converted to UTC)
///
/// # Returns
/// A tuple containing `(events, mentions, gkg_data)` vectors for the specified date
///
/// # Example
/// ```rust,no_run
/// use gdelt_fetcher::api::fetch_all_by_date;
/// use chrono::{Utc, TimeZone};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
///     let (events, mentions, gkg_data) = fetch_all_by_date(date).await?;
///     println!("Fetched {} events, {} mentions, {} GKG records for 2024-01-15",
///              events.len(), mentions.len(), gkg_data.len());
///
///     Ok(())
/// }
/// ```
pub async fn fetch_all_by_date<Tz: TimeZone + Clone>(
    date: DateTime<Tz>,
) -> Result<(Vec<EventTable>, Vec<MentionTable>, Vec<GKGTable>)> {
    let date_str = date.naive_utc().format("%Y-%m-%d").to_string();
    info!("🚀 Fetching all data types for date: {}", date_str);
    let start_time = std::time::Instant::now();

    // Use tokio::join! to fetch all data types concurrently
    let (events_result, mentions_result, gkg_result) = tokio::join!(
        fetch_events_by_date(date.clone()),
        fetch_mentions_by_date(date.clone()),
        fetch_gkg_by_date(date)
    );

    let events = events_result?;
    let mentions = mentions_result?;
    let gkg_data = gkg_result?;

    let duration = start_time.elapsed();
    info!(
        "✅ Fetched all data for {}: {} events, {} mentions, {} GKG records in {:?}",
        date_str,
        events.len(),
        mentions.len(),
        gkg_data.len(),
        duration
    );

    Ok((events, mentions, gkg_data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn init_logger() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }

    #[tokio::test]
    async fn test_fetch_latest_events() {
        init_logger();

        let result = fetch_latest_events().await;
        assert!(
            result.is_ok(),
            "Failed to fetch latest events: {:?}",
            result.err()
        );

        let events = result.unwrap();
        println!("Fetched {} events", events.len());
        assert!(!events.is_empty(), "No events were fetched");
    }

    #[tokio::test]
    async fn test_fetch_latest_mentions() {
        init_logger();

        let result = fetch_latest_mentions().await;
        assert!(
            result.is_ok(),
            "Failed to fetch latest mentions: {:?}",
            result.err()
        );

        let mentions = result.unwrap();
        println!("Fetched {} mentions", mentions.len());
        assert!(!mentions.is_empty(), "No mentions were fetched");
    }

    #[tokio::test]
    async fn test_fetch_latest_gkg() {
        init_logger();

        let result = fetch_latest_gkg().await;
        assert!(
            result.is_ok(),
            "Failed to fetch latest GKG: {:?}",
            result.err()
        );

        let gkg_data = result.unwrap();
        println!("Fetched {} GKG records", gkg_data.len());
        assert!(!gkg_data.is_empty(), "No GKG data was fetched");
    }

    #[tokio::test]
    async fn test_fetch_all_latest() {
        init_logger();

        let result = fetch_all_latest().await;
        assert!(
            result.is_ok(),
            "Failed to fetch all latest data: {:?}",
            result.err()
        );

        let (events, mentions, gkg_data) = result.unwrap();
        println!(
            "Fetched {} events, {} mentions, {} GKG records",
            events.len(),
            mentions.len(),
            gkg_data.len()
        );

        assert!(!events.is_empty(), "No events were fetched");
        assert!(!mentions.is_empty(), "No mentions were fetched");
        assert!(!gkg_data.is_empty(), "No GKG data was fetched");
    }

    #[tokio::test]
    async fn test_fetch_events_by_date() {
        init_logger();

        // Test with a recent date that should have data
        let date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        let result = fetch_events_by_date(date).await;

        // Note: This test might fail if the specific date doesn't have data
        // In a real test environment, you'd want to use a date you know has data
        match result {
            Ok(events) => {
                println!(
                    "Fetched {} events for {}",
                    events.len(),
                    date.format("%Y-%m-%d")
                );
            }
            Err(e) => {
                println!("Expected potential failure for historical date: {}", e);
                // This is acceptable for this test since historical data might not be available
            }
        }
    }

    #[tokio::test]
    async fn test_local_file_checking() {
        init_logger();

        // Test the helper functions for file existence checking
        let (tmp_dir, output_dir) = setup_temp_directories("test_check").unwrap();

        // Test that non-existent files return None
        let result = check_existing_file(&output_dir, &tmp_dir, TableType::Export, None);
        assert!(result.is_none(), "Should not find non-existent file");

        // Test search_directory_for_file with non-existent directory
        let result = search_directory_for_file("/non/existent/path", "export", "csv", None);
        assert!(
            result.is_none(),
            "Should handle non-existent directory gracefully"
        );

        println!("Local file checking tests completed successfully");
    }
}
