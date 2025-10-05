# GDELT Fetcher API

A simplified, high-level API for fetching and parsing GDELT (Global Database of Events, Language, and Tone) data.

## Overview

The GDELT Fetcher API provides clean, simple functions to fetch GDELT data either by latest available or by specific date. All functions return vectors of strongly-typed data structures, making it easy to work with GDELT data in Rust applications.

## Supported Data Types

The API supports fetching three main GDELT data tables:

- **Events** (`EventTable`) - The main event table containing coded events
- **Mentions** (`MentionTable`) - References to events in news articles and media
- **GKG** (`GKGTable`) - Global Knowledge Graph with enhanced semantic information

## Quick Start

Add `gdelt_fetcher` to your `Cargo.toml`:

```toml
[dependencies]
gdelt_fetcher = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use gdelt_fetcher::api::{fetch_latest_events, fetch_latest_mentions, fetch_latest_gkg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch latest events
    let events = fetch_latest_events().await?;
    println!("Fetched {} events", events.len());

    // Fetch latest mentions
    let mentions = fetch_latest_mentions().await?;
    println!("Fetched {} mentions", mentions.len());

    // Fetch latest GKG data
    let gkg_data = fetch_latest_gkg().await?;
    println!("Fetched {} GKG records", gkg_data.len());

    Ok(())
}
```

### Fetching by Date

```rust
use gdelt_fetcher::api::{fetch_events_by_date, fetch_mentions_by_date, fetch_gkg_by_date};
use chrono::{Utc, TimeZone};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();

    // Fetch events for specific date
    let events = fetch_events_by_date(date).await?;
    println!("Fetched {} events for 2024-01-15", events.len());

    // Fetch mentions for specific date
    let mentions = fetch_mentions_by_date(date).await?;
    println!("Fetched {} mentions for 2024-01-15", mentions.len());

    // Fetch GKG data for specific date
    let gkg_data = fetch_gkg_by_date(date).await?;
    println!("Fetched {} GKG records for 2024-01-15", gkg_data.len());

    Ok(())
}
```

### Bulk Operations

Fetch all data types at once for better performance:

```rust
use gdelt_fetcher::api::{fetch_all_latest, fetch_all_by_date};
use chrono::{Utc, TimeZone};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch all latest data concurrently
    let (events, mentions, gkg_data) = fetch_all_latest().await?;
    println!("Fetched {} events, {} mentions, {} GKG records",
             events.len(), mentions.len(), gkg_data.len());

    // Fetch all data for a specific date concurrently
    let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
    let (events, mentions, gkg_data) = fetch_all_by_date(date).await?;
    println!("Fetched {} events, {} mentions, {} GKG records for 2024-01-15",
             events.len(), mentions.len(), gkg_data.len());

    Ok(())
}
```

## API Reference

### Latest Data Functions

| Function | Returns | Description |
|----------|---------|-------------|
| `fetch_latest_events()` | `Vec<EventTable>` | Fetch the most recent events |
| `fetch_latest_mentions()` | `Vec<MentionTable>` | Fetch the most recent mentions |
| `fetch_latest_gkg()` | `Vec<GKGTable>` | Fetch the most recent GKG data |
| `fetch_all_latest()` | `(Vec<EventTable>, Vec<MentionTable>, Vec<GKGTable>)` | Fetch all data types concurrently |

### Date-Specific Functions

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `fetch_events_by_date(date)` | `DateTime<Tz>` | `Vec<EventTable>` | Fetch events for specific date |
| `fetch_mentions_by_date(date)` | `DateTime<Tz>` | `Vec<MentionTable>` | Fetch mentions for specific date |
| `fetch_gkg_by_date(date)` | `DateTime<Tz>` | `Vec<GKGTable>` | Fetch GKG data for specific date |
| `fetch_all_by_date(date)` | `DateTime<Tz>` | `(Vec<EventTable>, Vec<MentionTable>, Vec<GKGTable>)` | Fetch all data types for specific date |

### Data Structures

#### EventTable

Contains the main GDELT event data including:
- `global_event_id` - Unique event identifier
- `date` - Event date
- `actor1`, `actor2` - Event participants
- `event_action` - Action/event type information
- Geographic and other metadata

#### MentionTable

Contains references to events in media:
- `global_event_id` - Reference to the event
- `event_date` - Date of the event
- `mention_date` - Date the event was mentioned
- `mention_source_name` - Source of the mention
- Character offset information

#### GKGTable

Contains enhanced semantic information:
- `global_knowledge_graph_id` - Unique GKG record identifier
- `date` - Publication date
- `source_collection_identifier` - Source type
- Themes, locations, organizations, sentiment data

## Working with Timezones

The API accepts `DateTime` objects with any timezone. Dates are automatically converted to UTC for GDELT processing:

```rust
use chrono::{TimeZone, FixedOffset};

// EST timezone example
let est = FixedOffset::west_opt(5 * 3600).unwrap();
let est_date = est.with_ymd_and_hms(2024, 1, 15, 14, 30, 0).unwrap();

let events = fetch_events_by_date(est_date).await?;
```

## Error Handling

All API functions return `anyhow::Result<T>`. Common error scenarios:

- Network connectivity issues
- GDELT service unavailable
- Invalid date (too old or future)
- File parsing errors

```rust
match fetch_latest_events().await {
    Ok(events) => {
        println!("Successfully fetched {} events", events.len());
        // Process events...
    }
    Err(e) => {
        eprintln!("Failed to fetch events: {}", e);
        // Handle error...
    }
}
```

## Data Availability

- **Latest data**: Updated every 15 minutes
- **Historical data**: Available from February 2015 onwards
- **Coverage**: Global events in multiple languages

## Performance Considerations

- Use bulk functions (`fetch_all_*`) for better performance when fetching multiple data types
- Data is downloaded and cached locally in `./data/` and `./tmp/` directories
- Large datasets: Consider filtering or processing data in chunks for memory efficiency

## Examples

See the `examples/simple_api_example.rs` file for comprehensive usage examples.

## Advanced Usage

For more control over the fetching process, use the lower-level fetchers directly:

```rust
use gdelt_fetcher::{EventTableFetcher, setup_temp_directories};

let (tmp_dir, output_dir) = setup_temp_directories("events")?;
let mut fetcher = EventTableFetcher::new_v2(&output_dir, &tmp_dir)?;
let events_iter = fetcher.fetch_latest_async().await?;

// Process events one by one instead of collecting all at once
for event in events_iter.take(100) {
    println!("Processing event: {:?}", event.global_event_id);
}
```

## Logging

Enable logging to see detailed information about the fetching process:

```rust
use env_logger;

fn main() {
    env_logger::init();
    // Your code here...
}
```

## License

This crate is licensed under the same terms as the main `gdelt_fetcher` project.