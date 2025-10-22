//! Example of using the new in-memory GDELT fetcher API
//!
//! This example demonstrates:
//! - Fetching latest data (automatically rounded to 15-minute intervals)
//! - Fetching data for a specific timestamp
//! - Fetching all data for a day
//! - Fetching data for a date range
//! - Optional persistent storage with netabase

use anyhow::Result;
use chrono::Utc;

const PRINT_LIMIT: usize = 5;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Example 1: Create an in-memory fetcher (no persistence)
    println!("=== Example 1: In-Memory Fetcher ===");
    let mut fetcher = data::GdeltFetcher::new();

    // Initialize (downloads master file lists)
    fetcher.initialize().await?;
    println!("Fetcher initialized with {} masterlist entries\n", fetcher.masterlist_count().await?);

    // Fetch latest events (automatically rounds to nearest 15-minute interval)
    println!("=== Fetching Latest Events ===");
    let events = fetcher.fetch_latest_events().await?;
    println!("Fetched {} events from latest interval", events.len());
    println!("\nFirst {} events:", PRINT_LIMIT.min(events.len()));
    for (i, event) in events.iter().take(PRINT_LIMIT).enumerate() {
        let actor1_code = event.actor1.as_ref()
            .and_then(|a| a.code.as_ref())
            .map(|s| s.0.as_str())
            .unwrap_or("N/A");
        let actor2_code = event.actor2.as_ref()
            .and_then(|a| a.code.as_ref())
            .map(|s| s.0.as_str())
            .unwrap_or("N/A");
        println!("  Event {}: GlobalEventID={}, Date={:?}, Actor1={}, Actor2={}",
            i + 1,
            event.global_event_id.0,
            event.date,
            actor1_code,
            actor2_code
        );
    }

    // Fetch latest mentions
    println!("\n=== Fetching Latest Mentions ===");
    let mentions = fetcher.fetch_latest_mentions().await?;
    println!("Fetched {} mentions from latest interval", mentions.len());
    println!("\nFirst {} mentions:", PRINT_LIMIT.min(mentions.len()));
    for (i, mention) in mentions.iter().take(PRINT_LIMIT).enumerate() {
        println!("  Mention {}: GlobalEventID={}, MentionDate={}, Confidence={}",
            i + 1,
            mention.global_event_id.0,
            mention.mention_date,
            mention.confidence.0
        );
    }

    // Fetch latest GKG data
    println!("\n=== Fetching Latest GKG ===");
    let gkg = fetcher.fetch_latest_gkg().await?;
    println!("Fetched {} GKG records from latest interval", gkg.len());
    println!("\nFirst {} GKG records:", PRINT_LIMIT.min(gkg.len()));
    for (i, gkg_record) in gkg.iter().take(PRINT_LIMIT).enumerate() {
        println!("  GKG {}: Date={}, DocumentIdentifier={:?}",
            i + 1,
            gkg_record.date,
            gkg_record.document_identifier
        );
    }

    // Example 2: Fetch data for a specific timestamp
    println!("\n=== Example 2: Specific Timestamp ===");
    let specific_time = Utc::now().naive_utc() - chrono::Duration::hours(2);
    let events_at_time = fetcher.fetch_events_at(specific_time).await?;
    println!("Fetched {} events from {}", events_at_time.len(), specific_time);
    println!("\nFirst {} events at specific time:", PRINT_LIMIT.min(events_at_time.len()));
    for (i, event) in events_at_time.iter().take(PRINT_LIMIT).enumerate() {
        println!("  Event {}: GlobalEventID={}, Date={:?}",
            i + 1,
            event.global_event_id.0,
            event.date
        );
    }

    // Example 3: Fetch all data for a specific day (returns Vec of Vec)
    println!("\n=== Example 3: Full Day of Data ===");
    let yesterday = Utc::now().naive_utc().date() - chrono::Duration::days(1);
    let day_events = fetcher.fetch_events_day(yesterday).await?;
    println!("Fetched {} intervals for {}", day_events.len(), yesterday);

    let total_events: usize = day_events.iter().map(|interval| interval.len()).sum();
    println!("Total events for the day: {}", total_events);

    // Print first few events from first interval
    if let Some(first_interval) = day_events.first() {
        println!("\nFirst {} events from first interval of the day:", PRINT_LIMIT.min(first_interval.len()));
        for (i, event) in first_interval.iter().take(PRINT_LIMIT).enumerate() {
            println!("  Event {}: GlobalEventID={}, Date={:?}",
                i + 1,
                event.global_event_id.0,
                event.date
            );
        }
    }

    // Example 4: Fetch data for a date range
    println!("\n=== Example 4: Date Range ===");
    let start_date = Utc::now().naive_utc().date() - chrono::Duration::days(3);
    let end_date = Utc::now().naive_utc().date() - chrono::Duration::days(1);

    let range_events = fetcher.fetch_events_range(start_date, end_date).await?;
    println!("Fetched data for {} days", range_events.len());

    for (i, day_data) in range_events.iter().enumerate() {
        let day_total: usize = day_data.iter().map(|interval| interval.len()).sum();
        println!("Day {}: {} intervals, {} total events", i + 1, day_data.len(), day_total);
    }

    // Print first few events from first day
    if let Some(first_day) = range_events.first() {
        if let Some(first_interval) = first_day.first() {
            println!("\nFirst {} events from first interval of date range:", PRINT_LIMIT.min(first_interval.len()));
            for (i, event) in first_interval.iter().take(PRINT_LIMIT).enumerate() {
                println!("  Event {}: GlobalEventID={}, Date={:?}",
                    i + 1,
                    event.global_event_id.0,
                    event.date
                );
            }
        }
    }

    // Example 5: Storage modes for CSV data (non-WASM only)
    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("\n=== Example 5: Storage Modes ===");

        // In-memory (default)
        println!("Storage mode: In-Memory (default)");
        let mut fetcher_memory = data::GdeltFetcher::new();
        println!("  Mode: {:?}", fetcher_memory.storage_mode());

        // Tmp directory
        println!("\nStorage mode: Tmp Directory");
        let mut fetcher_tmp = data::GdeltFetcher::with_tmp_storage();
        fetcher_tmp.initialize().await?;
        println!("  Mode: {:?}", fetcher_tmp.storage_mode());
        let tmp_events = fetcher_tmp.fetch_latest_events().await?;
        println!("  Fetched {} events and saved CSV to tmp directory", tmp_events.len());

        // Custom path
        println!("\nStorage mode: Custom Path");
        let mut fetcher_custom = data::GdeltFetcher::with_custom_path("./data/gdelt_example");
        fetcher_custom.initialize().await?;
        println!("  Mode: {:?}", fetcher_custom.storage_mode());
        let custom_events = fetcher_custom.fetch_latest_events().await?;
        println!("  Fetched {} events and saved CSV to custom path", custom_events.len());
    }

    #[cfg(target_arch = "wasm32")]
    {
        println!("\n=== Example 5: Storage Modes ===");
        println!("(Storage modes are only available on native targets, not WASM)");
        println!("WASM always uses in-memory processing");
    }

    // Example 6: Fetcher with persistent masterlist storage (requires netabase feature)
    #[cfg(feature = "netabase")]
    {
        println!("\n=== Example 6: Persistent Masterlist Storage ===");
        let mut persistent_fetcher = data::GdeltFetcher::with_masterlist_storage("./gdelt_storage")?;
        persistent_fetcher.initialize().await?;

        println!("Persistent fetcher initialized");
        println!("Masterlist entries: {}", persistent_fetcher.masterlist_count().await?);
    }

    #[cfg(not(feature = "netabase"))]
    {
        println!("\n=== Example 6: Persistent Masterlist Storage ===");
        println!("(Skipped - netabase feature not enabled)");
        println!("To enable: cargo run --example new_api_usage --features netabase");
    }

    println!("\n=== All Examples Completed ===");
    Ok(())
}
