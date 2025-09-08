//! Basic GDELT Fetcher Usage Examples
//!
//! This example demonstrates the core functionality of the GDELT Fetcher library,
//! showing how to fetch, parse, and analyze GDELT data for events, mentions, and GKG records.

use anyhow::Result;
use chrono::Timelike;
use gdelt_fetcher::{
    DataFetcher, EventTableFetcher, MentionTableFetcher, fetch_and_parse_events,
    fetch_and_parse_gkg, fetch_and_parse_mentions,
};
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

/// Main example function demonstrating various GDELT operations
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to see detailed operation info
    env_logger::init();

    println!("🌍 GDELT Fetcher - Basic Usage Examples");
    println!("=====================================\n");

    // Example 1: Simple data fetching
    basic_fetching_example().await?;

    // Example 2: Advanced fetcher usage with custom directories
    advanced_fetcher_example().await?;

    // Example 3: Data analysis and statistics
    data_analysis_example().await?;

    // Example 4: Error handling and retry logic
    error_handling_example().await?;

    // Example 5: Performance monitoring
    performance_monitoring_example().await?;

    println!("\n🎉 All examples completed successfully!");
    Ok(())
}

/// Example 1: Basic data fetching using convenience functions
async fn basic_fetching_example() -> Result<()> {
    println!("📊 Example 1: Basic Data Fetching");
    println!("---------------------------------");

    println!("🔄 Fetching latest GDELT events...");
    let events = fetch_and_parse_events().await?;
    println!("✅ Fetched {} events", events.len());

    // Display a sample event
    if let Some(first_event) = events.first() {
        println!("📝 Sample Event:");
        println!("  Event ID: {:?}", first_event.global_event_id);
        println!(
            "  Date: {}",
            first_event.date_added.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "  Actor 1: {:?}",
            first_event.actor1.as_ref().and_then(|a| a.name.as_ref())
        );
        println!(
            "  Actor 2: {:?}",
            first_event.actor2.as_ref().and_then(|a| a.name.as_ref())
        );
        println!("  Event Code: {:?}", first_event.event_action.event_code);
        println!(
            "  Goldstein Scale: {:.2}",
            first_event.event_action.goldstein_scale.0
        );
        println!("  Average Tone: {:.2}", first_event.event_action.avg_tone.0);
    }

    println!("\n🔄 Fetching latest mentions...");
    let mentions = fetch_and_parse_mentions().await?;
    println!("✅ Fetched {} mentions", mentions.len());

    // Display a sample mention
    if let Some(first_mention) = mentions.first() {
        println!("📝 Sample Mention:");
        println!("  Event ID: {:?}", first_mention.global_event_id);
        println!("  Source: {:?}", first_mention.mention_source_name);
        println!("  Source: {:?}", first_mention.mention_source_name);
        println!(
            "  Published: {}",
            first_mention.mention_date.format("%Y-%m-%d %H:%M:%S")
        );
        println!("  Mention Type: {:?}", first_mention.mention_type);
        println!("  Confidence: {:.2}", first_mention.confidence.0);
    }

    println!("\n🔄 Fetching latest GKG data...");
    let gkg_data = fetch_and_parse_gkg().await?;
    println!("✅ Fetched {} GKG records", gkg_data.len());

    // Display a sample GKG record
    if let Some(first_gkg) = gkg_data.first() {
        println!("📝 Sample GKG Record:");
        println!("  Date: {}", first_gkg.date.format("%Y-%m-%d"));
        println!("  Source Name: {:?}", first_gkg.source_common_name);
        println!("  Document ID: {:?}", first_gkg.document_identifier);
        if !first_gkg.v1_themes.is_empty() {
            println!(
                "  Themes: {:?}",
                &first_gkg.v1_themes[..std::cmp::min(3, first_gkg.v1_themes.len())]
            );
        }
        if !first_gkg.v1_locations.is_empty() {
            println!(
                "  Locations: {:?}",
                &first_gkg.v1_locations[..std::cmp::min(3, first_gkg.v1_locations.len())]
            );
        }
    }

    println!("\n✅ Basic fetching example completed\n");
    Ok(())
}

/// Example 2: Advanced usage with custom directories and direct fetcher control
async fn advanced_fetcher_example() -> Result<()> {
    println!("🛠️  Example 2: Advanced Fetcher Usage");
    println!("------------------------------------");

    // Set up custom directories
    let output_dir = "./examples_data/output";
    let temp_dir = "./examples_data/temp";

    println!("📁 Using custom directories:");
    println!("  Output: {}", output_dir);
    println!("  Temp: {}", temp_dir);

    // Create event fetcher with custom configuration
    println!("\n🔧 Creating custom EventTableFetcher...");
    let mut event_fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;

    // Get URL information
    let url = event_fetcher.url_link()?;
    println!("📍 Fetching from URL: {}", url);

    // Fetch data using iterator (memory efficient)
    println!("🔄 Fetching events with iterator...");
    let event_iterator = event_fetcher.fetch_latest_async().await?;

    let mut count = 0;
    let mut high_impact_events = Vec::new();

    for event in event_iterator.take(100) {
        // Process first 100 events
        count += 1;

        // Identify high-impact events (|Goldstein Scale| > 5)
        let goldstein = event.event_action.goldstein_scale.0;
        if goldstein.abs() > 5.0 {
            high_impact_events.push((event.global_event_id.0.to_string(), goldstein));
        }
    }

    println!("✅ Processed {} events", count);
    println!("🚨 Found {} high-impact events:", high_impact_events.len());

    for (event_id, impact) in high_impact_events.iter().take(5) {
        println!("  Event {:?}: Impact {:.2}", event_id, impact);
    }

    // Create mention fetcher
    println!("\n🔧 Creating MentionTableFetcher...");
    let mut mention_fetcher = MentionTableFetcher::new_v2(output_dir, temp_dir)?;
    let mention_iterator = mention_fetcher.fetch_latest_async().await?;

    // Analyze mention sources
    let mut source_count = HashMap::new();
    let mut processed_mentions = 0;

    for mention in mention_iterator.take(200) {
        processed_mentions += 1;
        let source = mention.mention_source_name.0;
        *source_count.entry(source).or_insert(0) += 1;
    }

    println!("✅ Processed {} mentions", processed_mentions);

    // Show top sources
    let mut sorted_sources: Vec<_> = source_count.iter().collect();
    sorted_sources.sort_by(|a, b| b.1.cmp(a.1));

    println!("📰 Top mention sources:");
    for (source, count) in sorted_sources.iter().take(5) {
        println!("  {}: {} mentions", source, count);
    }

    println!("\n✅ Advanced fetcher example completed\n");
    Ok(())
}

/// Example 3: Data analysis and statistical insights
async fn data_analysis_example() -> Result<()> {
    println!("📈 Example 3: Data Analysis");
    println!("---------------------------");

    // Fetch data for analysis
    println!("🔄 Fetching data for analysis...");
    let events = fetch_and_parse_events().await?;
    let mentions = fetch_and_parse_mentions().await?;

    println!(
        "✅ Data fetched: {} events, {} mentions",
        events.len(),
        mentions.len()
    );

    // Analyze event sentiment distribution
    println!("\n😊 Sentiment Analysis:");
    let mut positive_events = 0;
    let mut negative_events = 0;
    let mut neutral_events = 0;
    let mut total_goldstein = 0.0;
    let mut goldstein_count = 0;

    for event in &events {
        let goldstein = event.event_action.goldstein_scale.0;
        total_goldstein += goldstein;
        goldstein_count += 1;

        if goldstein > 1.0 {
            positive_events += 1;
        } else if goldstein < -1.0 {
            negative_events += 1;
        } else {
            neutral_events += 1;
        }
    }

    if goldstein_count > 0 {
        let avg_goldstein = total_goldstein / goldstein_count as f64;
        println!("  Average Goldstein Scale: {:.3}", avg_goldstein);
        println!(
            "  Positive events (>1.0): {} ({:.1}%)",
            positive_events,
            100.0 * positive_events as f64 / goldstein_count as f64
        );
        println!(
            "  Negative events (<-1.0): {} ({:.1}%)",
            negative_events,
            100.0 * negative_events as f64 / goldstein_count as f64
        );
        println!(
            "  Neutral events: {} ({:.1}%)",
            neutral_events,
            100.0 * neutral_events as f64 / goldstein_count as f64
        );
    }

    // Analyze event types
    println!("\n🏷️  Event Type Analysis:");
    let mut event_codes = HashMap::new();

    for event in &events {
        if let Some(code) = &event.event_action.event_code {
            *event_codes.entry(format!("{:?}", code)).or_insert(0) += 1;
        }
    }

    let mut sorted_codes: Vec<_> = event_codes.iter().collect();
    sorted_codes.sort_by(|a, b| b.1.cmp(a.1));

    println!("  Top 10 Event Types:");
    for (i, (code, count)) in sorted_codes.iter().take(10).enumerate() {
        println!("  {}. {}: {} occurrences", i + 1, code, count);
    }

    // Analyze geographic distribution
    println!("\n🌍 Geographic Analysis:");
    let mut countries = HashMap::new();

    for event in &events {
        if let Some(geography) = &event.action_geography {
            if let Some(country) = &geography.country_code {
                *countries.entry(format!("{:?}", country)).or_insert(0) += 1;
            }
        }
    }

    let mut sorted_countries: Vec<_> = countries.iter().collect();
    sorted_countries.sort_by(|a, b| b.1.cmp(a.1));

    println!("  Top 10 Countries by Event Count:");
    for (i, (country, count)) in sorted_countries.iter().take(10).enumerate() {
        println!("  {}. {}: {} events", i + 1, country, count);
    }

    // Analyze mention timeline
    println!("\n⏰ Temporal Analysis:");
    let mut hourly_mentions = HashMap::new();

    for mention in &mentions {
        let hour = mention.mention_date.hour();
        *hourly_mentions.entry(hour).or_insert(0) += 1;
    }

    println!("  Mentions by Hour (UTC):");
    for hour in 0..24 {
        let count = hourly_mentions.get(&hour).unwrap_or(&0);
        let bar = "█".repeat(*count / 10);
        println!("  {:02}:00 {:4} {}", hour, count, bar);
    }

    println!("\n✅ Data analysis example completed\n");
    Ok(())
}

/// Example 4: Error handling and retry logic
async fn error_handling_example() -> Result<()> {
    println!("🛡️  Example 4: Error Handling");
    println!("-----------------------------");

    // Demonstrate robust error handling
    println!("🔄 Testing error handling with retry logic...");

    let max_retries = 3;
    let mut attempt = 0;

    loop {
        attempt += 1;
        println!("  Attempt {} of {}", attempt, max_retries);

        match fetch_and_parse_events().await {
            Ok(events) => {
                println!(
                    "✅ Successfully fetched {} events on attempt {}",
                    events.len(),
                    attempt
                );
                break;
            }
            Err(e) => {
                eprintln!("❌ Error on attempt {}: {}", attempt, e);

                if attempt >= max_retries {
                    eprintln!("💥 Failed after {} attempts", max_retries);
                    return Err(e);
                }

                // Exponential backoff
                let delay = Duration::from_secs(2_u64.pow(attempt - 1));
                println!("⏳ Waiting {:?} before retry...", delay);
                sleep(delay).await;
            }
        }
    }

    // Demonstrate partial failure handling
    println!("\n🔧 Testing graceful degradation...");

    let mut successful_fetches = 0;
    let mut failed_fetches = 0;

    // Try to fetch different data types
    // Try different fetching operations
    println!("Testing Events...");
    match fetch_and_parse_events().await {
        Ok(_) => {
            println!("✅ Events fetch successful");
            successful_fetches += 1;
        }
        Err(e) => {
            eprintln!("❌ Events fetch failed: {}", e);
            failed_fetches += 1;
        }
    }

    println!("Testing Mentions...");
    match fetch_and_parse_mentions().await {
        Ok(_) => {
            println!("✅ Mentions fetch successful");
            successful_fetches += 1;
        }
        Err(e) => {
            eprintln!("❌ Mentions fetch failed: {}", e);
            failed_fetches += 1;
        }
    }

    println!("Testing GKG...");
    match fetch_and_parse_gkg().await {
        Ok(_) => {
            println!("✅ GKG fetch successful");
            successful_fetches += 1;
        }
        Err(e) => {
            eprintln!("❌ GKG fetch failed: {}", e);
            failed_fetches += 1;
        }
    }

    // Results already processed above

    println!("📊 Operation Summary:");
    println!("  Successful: {}", successful_fetches);
    println!("  Failed: {}", failed_fetches);
    println!(
        "  Success Rate: {:.1}%",
        100.0 * successful_fetches as f64 / (successful_fetches + failed_fetches) as f64
    );

    println!("\n✅ Error handling example completed\n");
    Ok(())
}

/// Example 5: Performance monitoring and optimization
async fn performance_monitoring_example() -> Result<()> {
    println!("⚡ Example 5: Performance Monitoring");
    println!("------------------------------------");

    use std::time::Instant;

    // Monitor memory-efficient streaming vs. collecting all data
    println!("🔍 Comparing streaming vs. collection performance...");

    // Method 1: Streaming processing (memory efficient)
    println!("\n📊 Method 1: Streaming Processing");
    let start_time = Instant::now();

    let output_dir = "./examples_data/performance";
    let temp_dir = "./examples_data/temp_perf";

    let mut event_fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;
    let event_iterator = event_fetcher.fetch_latest_async().await?;

    let mut processed_count = 0;
    let mut significant_events = 0;

    for event in event_iterator.take(1000) {
        processed_count += 1;

        // Process event (simulate analysis work)
        let goldstein = event.event_action.goldstein_scale.0;
        if goldstein.abs() > 3.0 {
            significant_events += 1;
        }

        // Show progress every 100 events
        if processed_count % 100 == 0 {
            println!("  Processed {} events...", processed_count);
        }
    }

    let streaming_duration = start_time.elapsed();
    println!(
        "✅ Streaming: Processed {} events in {:?}",
        processed_count, streaming_duration
    );
    println!("  Found {} significant events", significant_events);

    // Method 2: Collect all then process (memory intensive)
    println!("\n📊 Method 2: Collection Processing");
    let start_time = Instant::now();

    let events = fetch_and_parse_events().await?;
    let collection_fetch_time = start_time.elapsed();

    let process_start = Instant::now();
    let significant_events_collected = events
        .iter()
        .filter(|event| event.event_action.goldstein_scale.0.abs() > 3.0)
        .count();
    let process_time = process_start.elapsed();

    println!(
        "✅ Collection: Fetched {} events in {:?}",
        events.len(),
        collection_fetch_time
    );
    println!("  Processing took {:?}", process_time);
    println!(
        "  Found {} significant events",
        significant_events_collected
    );

    // Performance comparison
    println!("\n⚖️  Performance Comparison:");
    println!("  Streaming approach: {:?} total", streaming_duration);
    println!(
        "  Collection approach: {:?} fetch + {:?} process = {:?} total",
        collection_fetch_time,
        process_time,
        collection_fetch_time + process_time
    );

    // Memory usage estimation
    println!("\n💾 Memory Usage Estimation:");
    let avg_event_size = 500; // Rough estimate in bytes
    let streaming_memory = avg_event_size; // Only one event in memory at a time
    let collection_memory = events.len() * avg_event_size;

    println!(
        "  Streaming: ~{} bytes ({} KB)",
        streaming_memory,
        streaming_memory / 1024
    );
    println!(
        "  Collection: ~{} bytes ({} MB)",
        collection_memory,
        collection_memory / (1024 * 1024)
    );

    // Throughput calculation
    let streaming_throughput = processed_count as f64 / streaming_duration.as_secs_f64();
    let collection_throughput = events.len() as f64 / collection_fetch_time.as_secs_f64();

    println!("\n🚀 Throughput:");
    println!("  Streaming: {:.0} events/second", streaming_throughput);
    println!("  Collection: {:.0} events/second", collection_throughput);

    println!("\n💡 Recommendation:");
    if streaming_duration < collection_fetch_time + process_time {
        println!("  Use streaming for better performance and memory efficiency");
    } else {
        println!("  Collection might be acceptable for smaller datasets");
    }

    println!("\n✅ Performance monitoring example completed\n");
    Ok(())
}

/// Helper function to format large numbers
#[allow(dead_code)]
fn format_number(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

/// Helper function to create a simple progress bar
#[allow(dead_code)]
fn progress_bar(current: usize, total: usize, width: usize) -> String {
    let progress = (current as f64 / total as f64 * width as f64) as usize;
    let bar = "█".repeat(progress);
    let empty = "░".repeat(width - progress);
    format!(
        "[{}{}] {:.1}%",
        bar,
        empty,
        100.0 * current as f64 / total as f64
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(500), "500");
        assert_eq!(format_number(1_500), "1.5K");
        assert_eq!(format_number(1_500_000), "1.5M");
    }

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(50, 100, 20);
        assert!(bar.contains("50.0%"));
        assert!(bar.len() > 20); // Should include brackets and percentage
    }
}
