//! Simple API Example
//!
//! This example demonstrates how to use the simplified GDELT Fetcher API
//! to fetch data by latest and by date.

use anyhow::Result;
use chrono::{TimeZone, Utc};
use env_logger;
use gdelt_fetcher::api::{
    fetch_all_by_date, fetch_all_latest, fetch_events_by_date, fetch_gkg_by_date,
    fetch_latest_events, fetch_latest_gkg, fetch_latest_mentions, fetch_mentions_by_date,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    println!("🚀 GDELT Fetcher API Example");
    println!("{}", "=".repeat(50));

    // Example 1: Fetch latest events
    println!("\n📊 Fetching latest events...");
    match fetch_latest_events().await {
        Ok(events) => {
            println!("✅ Successfully fetched {} events", events.len());
            if !events.is_empty() {
                println!("   First event ID: {:?}", events[0].global_event_id);
                println!("   First event date: {:?}", events[0].date);
            }
        }
        Err(e) => println!("❌ Error fetching events: {}", e),
    }

    // Example 2: Fetch latest mentions
    println!("\n📰 Fetching latest mentions...");
    match fetch_latest_mentions().await {
        Ok(mentions) => {
            println!("✅ Successfully fetched {} mentions", mentions.len());
            if !mentions.is_empty() {
                println!(
                    "   First mention event ID: {:?}",
                    mentions[0].global_event_id
                );
                println!(
                    "   First mention source: {:?}",
                    mentions[0].mention_source_name
                );
            }
        }
        Err(e) => println!("❌ Error fetching mentions: {}", e),
    }

    // Example 3: Fetch latest GKG data
    println!("\n🌐 Fetching latest GKG data...");
    match fetch_latest_gkg().await {
        Ok(gkg_data) => {
            println!("✅ Successfully fetched {} GKG records", gkg_data.len());
            if !gkg_data.is_empty() {
                println!(
                    "   First GKG record ID: {:?}",
                    gkg_data[0].global_knowledge_graph_id
                );
                println!("   First GKG record date: {:?}", gkg_data[0].date);
            }
        }
        Err(e) => println!("❌ Error fetching GKG data: {}", e),
    }

    // Example 4: Fetch all latest data at once
    println!("\n🔄 Fetching all latest data types concurrently...");
    match fetch_all_latest().await {
        Ok((events, mentions, gkg_data)) => {
            println!(
                "✅ Successfully fetched all data: {} events, {} mentions, {} GKG records",
                events.len(),
                mentions.len(),
                gkg_data.len()
            );
        }
        Err(e) => println!("❌ Error fetching all latest data: {}", e),
    }

    // Example 5: Fetch data by specific date
    println!("\n📅 Fetching data for a specific date...");
    let target_date = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    println!("   Target date: {}", target_date.format("%Y-%m-%d"));

    match fetch_events_by_date(target_date).await {
        Ok(events) => {
            println!(
                "✅ Successfully fetched {} events for {}",
                events.len(),
                target_date.format("%Y-%m-%d")
            );
        }
        Err(e) => println!("❌ Error fetching events by date: {}", e),
    }

    match fetch_mentions_by_date(target_date).await {
        Ok(mentions) => {
            println!(
                "✅ Successfully fetched {} mentions for {}",
                mentions.len(),
                target_date.format("%Y-%m-%d")
            );
        }
        Err(e) => println!("❌ Error fetching mentions by date: {}", e),
    }

    match fetch_gkg_by_date(target_date).await {
        Ok(gkg_data) => {
            println!(
                "✅ Successfully fetched {} GKG records for {}",
                gkg_data.len(),
                target_date.format("%Y-%m-%d")
            );
        }
        Err(e) => println!("❌ Error fetching GKG data by date: {}", e),
    }

    // Example 6: Fetch all data types for a specific date
    println!("\n🗓️ Fetching all data types for a specific date...");
    match fetch_all_by_date(target_date).await {
        Ok((events, mentions, gkg_data)) => {
            println!(
                "✅ Successfully fetched all data for {}: {} events, {} mentions, {} GKG records",
                target_date.format("%Y-%m-%d"),
                events.len(),
                mentions.len(),
                gkg_data.len()
            );
        }
        Err(e) => println!("❌ Error fetching all data by date: {}", e),
    }

    // Example 7: Working with different date formats
    println!("\n🌍 Fetching data with another date...");
    let another_date = Utc.with_ymd_and_hms(2024, 2, 1, 12, 0, 0).unwrap();
    println!("   Date: {}", another_date.format("%Y-%m-%d %H:%M:%S UTC"));

    match fetch_events_by_date(another_date).await {
        Ok(events) => {
            println!(
                "✅ Successfully fetched {} events for {}",
                events.len(),
                another_date.format("%Y-%m-%d")
            );
        }
        Err(e) => println!(
            "❌ Error fetching events for {}: {}",
            another_date.format("%Y-%m-%d"),
            e
        ),
    }

    println!("\n🎉 Example completed!");
    println!("{}", "=".repeat(50));

    Ok(())
}
