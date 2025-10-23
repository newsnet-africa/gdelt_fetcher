# GDELT Fetcher

A high-performance Rust library for fetching, parsing, and processing GDELT (Global Database of Events, Language, and Tone) data with support for both native and WASM environments.

## Status

⚠️ **Note**: This library is currently undergoing refactoring to integrate with the new netabase_store API. The core fetching and parsing functionality is stable, but some features (particularly persistent masterlist storage) are temporarily unavailable.

## Features

### Current Features

- **GDELT Data Fetching**:
  - Events table (export)
  - Mentions table
  - GKG (Global Knowledge Graph) table
  - Support for both GDELT v2 and v3 GEG data

- **Flexible Querying**:
  - Fetch latest data (automatically rounded to 15-minute intervals)
  - Fetch data for specific timestamps
  - Fetch all data for a specific day
  - Fetch data for date ranges

- **Cross-Platform Support**:
  - Native (using reqwest + tokio)
  - WASM (using wasm-bindgen + web-sys)

- **Configurable Storage**:
  - In-memory processing (default, WASM-compatible)
  - Save to tmp directory (native only)
  - Save to custom path (native only)

- **GCAM (Global Content Analysis Measures)**:
  - Enriched GKG data with GCAM dimensions
  - Efficient HashMap-based lookups
  - Support for all GCAM variables

- **High-Performance Parsing**:
  - Streaming CSV parser
  - Parallel processing support
  - Memory-efficient data structures

- **Polite Fetching**:
  - Built-in rate limiting
  - Respectful of GDELT servers
  - Automatic retry with backoff (coming in 1.0.0)

### TODO for 1.0.0

- [ ] **Complete Netabase Integration**:
  - Persistent masterlist storage using new netabase_store API
  - Refactor masterlist_manager to use new storage layer
  - Update fetcher to use new MasterfilelistStore

- [ ] **Fetch Profiles/Modes**:
  - Polling mode: Automatic updates every 15 minutes
  - Batch mode: Optimized for large date ranges
  - Real-time mode: WebSocket-based live updates

- [ ] **Enhanced GCAM Support**:
  - Pre-configured GCAM variable groups (tone, geography, etc.)
  - GCAM aggregation and analysis utilities
  - Time-series helpers for tracking GCAM trends

- [ ] **Caching & Performance**:
  - LRU cache for recently fetched data
  - Compressed storage options
  - Delta updates for incremental fetching

- [ ] **Error Recovery**:
  - Automatic retry with exponential backoff
  - Partial data recovery on parse errors
  - Connection failure handling

- [ ] **Data Validation**:
  - Schema validation for parsed data
  - Data quality checks
  - Anomaly detection

- [ ] **Export & Integration**:
  - Export to JSON, Parquet, Arrow
  - Integration with popular data analysis tools
  - Streaming API for real-time processing

## Installation

Add to your `Cargo.toml`:

```toml
# For native platforms
[dependencies]
gdelt_fetcher = "0.1"

# For WASM platforms
[dependencies]
gdelt_fetcher = { version = "0.1", features = ["wasm"] }

# With optional netabase integration (when refactoring complete)
# gdelt_fetcher = { version = "0.1", features = ["wasm-netabase"] }
```

## Quick Start

### Fetch Latest Data

```rust
use gdelt_fetcher::GdeltFetcher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a fetcher with in-memory processing
    let mut fetcher = GdeltFetcher::new();

    // Initialize (downloads master file lists)
    fetcher.initialize().await?;

    // Fetch the latest events
    let events = fetcher.fetch_latest_events().await?;
    println!("Fetched {} events", events.len());

    // Fetch the latest GKG data
    let gkg = fetcher.fetch_latest_gkg().await?;
    println!("Fetched {} GKG records", gkg.len());

    Ok(())
}
```

### Fetch Historical Data

```rust
use chrono::NaiveDate;

// Fetch all events for a specific day
let date = NaiveDate::from_ymd_opt(2024, 10, 15).unwrap();
let day_events = fetcher.fetch_events_day(date).await?;
println!("Fetched {} intervals of data", day_events.len());

// Fetch data for a date range
let start = NaiveDate::from_ymd_opt(2024, 10, 1).unwrap();
let end = NaiveDate::from_ymd_opt(2024, 10, 7).unwrap();
let range_events = fetcher.fetch_events_range(start, end).await?;
println!("Fetched {} days of data", range_events.len());
```

### Save Downloaded Data (Native Only)

```rust
use gdelt_fetcher::StorageMode;
use std::path::PathBuf;

// Save to system tmp directory
let mut fetcher = GdeltFetcher::with_tmp_storage();
fetcher.initialize().await?;

// Or save to custom path
let custom_path = PathBuf::from("/data/gdelt");
let mut fetcher = GdeltFetcher::with_custom_path(custom_path);
fetcher.initialize().await?;

// Fetched CSV files will be saved to the specified location
let events = fetcher.fetch_latest_events().await?;
```

## Data Models

### Event Table

```rust
pub struct EventTable {
    pub global_event_id: u32,
    pub day: u32,
    pub month_year: u32,
    pub year: u32,
    pub fraction_date: f64,
    pub actor1_code: Option<String>,
    pub actor2_code: Option<String>,
    pub quad_class: Option<u8>,
    pub goldstein_scale: Option<f64>,
    pub num_mentions: Option<u32>,
    pub num_sources: Option<u32>,
    pub num_articles: Option<u32>,
    pub avg_tone: Option<f64>,
    // ... and many more fields
}
```

### GKG Table

```rust
pub struct GKGTable {
    pub gkg_record_id: String,
    pub v21_date: Option<u64>,
    pub v2_source_common_name: Option<String>,
    pub v2_document_identifier: Option<String>,
    pub v1_themes: Option<Vec<String>>,
    pub v2_enhanced_themes: Option<Vec<String>>,
    pub v1_locations: Option<Vec<Location>>,
    pub v2_enhanced_locations: Option<Vec<EnhancedLocation>>,
    pub v1_persons: Option<Vec<String>>,
    pub v2_enhanced_persons: Option<Vec<String>>,
    pub v1_organizations: Option<Vec<String>>,
    pub v2_enhanced_organizations: Option<Vec<String>>,
    pub v15_tone: Option<Tone>,
    pub v21_enhanced_dates: Option<Vec<EnhancedDate>>,
    pub v2_gcam: Option<Vec<EnrichedGCAMEntry>>,  // GCAM dimensions
    // ... and more
}
```

### GCAM Dimensions

The library includes comprehensive support for GCAM (Global Content Analysis Measures):

```rust
// Access GCAM data from GKG records
for gkg in gkg_records {
    if let Some(gcam_entries) = &gkg.v2_gcam {
        for entry in gcam_entries {
            println!(
                "Variable: {}, Score: {}, Label: {}",
                entry.variable_name,
                entry.score_with_label,
                entry.dimension_human_name
            );
        }
    }
}
```

## Architecture

### Components

1. **Fetchers**: HTTP clients for downloading GDELT data
   - Native: reqwest + tokio
   - WASM: web-sys Fetch API

2. **Parsers**: CSV parsers for GDELT formats
   - Event table parser
   - Mention table parser
   - GKG table parser

3. **Models**: Type-safe data structures
   - Comprehensive field coverage
   - Optional fields for missing data
   - Enriched types (GCAM, locations, etc.)

4. **Storage** (refactoring in progress):
   - In-memory cache
   - File-based persistence
   - Netabase integration (coming in 1.0.0)

## Performance

- **Streaming parsing**: Processes large CSV files without loading entirely into memory
- **Parallel processing**: Can fetch multiple time intervals concurrently
- **Efficient data structures**: Minimal memory overhead
- **Zero-copy where possible**: Reduces allocations

## WASM Support

The library is fully compatible with WASM environments:

```rust
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use gdelt_fetcher::GdeltFetcher;

#[wasm_bindgen]
pub async fn fetch_gdelt_data() -> Result<JsValue, JsValue> {
    let mut fetcher = GdeltFetcher::new();
    fetcher.initialize().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let events = fetcher.fetch_latest_events().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Convert to JSON for JavaScript
    Ok(serde_wasm_bindgen::to_value(&events)?)
}
```

## Examples

See the `examples/` directory (coming in 1.0.0 after refactoring):
- Basic fetching
- Historical data analysis
- GCAM analysis
- Real-time monitoring

## Testing

```bash
# Run all tests
cargo test

# Run with specific features
cargo test --features wasm
cargo test --features netabase
```

## Limitations

- GDELT data is updated every 15 minutes; this library respects that cadence
- Historical data availability depends on GDELT's retention policies
- Large date ranges can be slow; consider using batch processing
- Netabase integration is currently being refactored

## GDELT Data Format

This library supports:
- **GDELT 2.0**: Event, Mention, and GKG tables
- **GDELT 3.0 GEG**: Enhanced GKG format (partial support)

For more information about GDELT data formats, visit: https://www.gdeltproject.org/data.html

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Links

- [Netabase Store (storage layer)](../netabase_store)
- [Netabase (networking layer)](../netabase)
- [GDELT Project](https://www.gdeltproject.org/)

## Contributing

Contributions are welcome! Especially help with completing the netabase integration refactoring.
