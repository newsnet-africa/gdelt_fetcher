use anyhow::Result;
use gdelt_fetcher::{fetch_and_parse_latest_events, EventTable};
use std::env;

/// Fetch and display GDELT Event table entries
///
/// Usage: cargo run --example fetch_events [count]
///
/// Arguments:
///   count - Number of items to display (default: 10)
///
/// Example:
///   cargo run --example fetch_events 20
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // Get count from command line args, default to 10
    let args: Vec<String> = env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };

    println!("{}", "=".repeat(80));
    println!("GDELT Event Table Fetcher");
    println!("{}", "=".repeat(80));
    println!("Fetching latest GDELT events...\n");

    let events = fetch_and_parse_latest_events().await?;

    println!("✅ Successfully fetched {} events", events.len());
    println!("📊 Displaying first {} events:\n", count);

    for (i, event) in events.iter().take(count).enumerate() {
        print_event(i + 1, event);
    }

    println!("\n{}", "=".repeat(80));
    println!("Summary:");
    println!("  Total events fetched: {}", events.len());
    println!("  Events displayed: {}", count.min(events.len()));
    println!("{}", "=".repeat(80));

    Ok(())
}

fn print_event(index: usize, event: &EventTable) {
    println!("{}", "━".repeat(80));
    println!("Event #{}", index);
    println!("{}", "━".repeat(80));

    println!("  Global Event ID: {}", event.global_event_id.0);
    println!("  Event Date: {}", event.date.date);
    println!("  Date Added: {}", event.date_added);
    println!("  Source URL: {}", event.source_url);

    // Actor information
    if let Some(actor1) = &event.actor1 {
        if let Some(name) = &actor1.name {
            println!("  Actor 1: {}", name);
        }
        if let Some(country) = &actor1.country_code {
            println!("    Country: {:?}", country);
        }
    }

    if let Some(actor2) = &event.actor2 {
        if let Some(name) = &actor2.name {
            println!("  Actor 2: {}", name);
        }
        if let Some(country) = &actor2.country_code {
            println!("    Country: {:?}", country);
        }
    }

    // Event details
    if let Some(event_code) = &event.event_action.event_code {
        println!("  Event Code: {:?}", event_code);
    }

    println!("  Event Root Code: {}", event.event_action.event_root_code.0);
    println!("  Quad Class: {:?}", event.event_action.quad_class);

    // Goldstein scale (conflict/cooperation measure)
    let goldstein_scale = event.event_action.goldstein_scale.0;
    let sentiment = if goldstein_scale > 0.0 {
        "Cooperative"
    } else if goldstein_scale < 0.0 {
        "Conflictual"
    } else {
        "Neutral"
    };
    println!("  Goldstein Scale: {:.2} ({})", goldstein_scale, sentiment);

    // Tone
    println!("  Average Tone: {:.2}", event.event_action.avg_tone.0);

    // Mentions and articles
    println!("  Mentions: {}", event.event_action.num_mentions.0);
    println!("  Sources: {}", event.event_action.num_sources.0);
    println!("  Articles: {}", event.event_action.num_articles.0);

    // Geographic information
    if let Some(geo) = &event.action_geography {
        if let Some(fullname) = &geo.fullname {
            println!("  Location: {}", fullname);
        }

        if let Some(coords) = &geo.coordinates {
            println!("    Coordinates: ({:.4}, {:.4})", coords.latitude, coords.longitude);
        }

        if let Some(country) = &geo.country_code {
            println!("    Country: {:?}", country);
        }
    }

    println!();
}
