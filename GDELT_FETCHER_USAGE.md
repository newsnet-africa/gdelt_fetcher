# GDELT Fetcher System Documentation

## Overview

The GDELT Fetcher System is a comprehensive Rust library for downloading, parsing, and working with GDELT (Global Database of Events, Language, and Tone) data. This system provides a modern, type-safe, and efficient way to interact with both GDELT v2 and v3 data sources.

## Key Features

- ✅ **Multi-version Support**: Works with both GDELT v2 and v3
- ✅ **Type-safe Enums**: Strong typing for table types, file extensions, and versions
- ✅ **Hash Validation**: Automatic MD5 hash verification for downloaded files
- ✅ **Translation Support**: Built-in support for translation files
- ✅ **Flexible File Formats**: Support for CSV (compressed/uncompressed) and JSON files
- ✅ **URL Building**: Programmatic URL construction with validation
- ✅ **Error Recovery**: Robust error handling with retry mechanisms
- ✅ **Async/Await**: Full async support for non-blocking operations
- ✅ **Iterator-based Processing**: Memory-efficient streaming of large datasets
- ✅ **Local File Management**: Automatic cleanup and file organization

## Architecture

### Core Components

1. **`GdeltFetcher`** - Low-level fetcher for downloading and managing files
2. **Table-specific Fetchers** - High-level fetchers for each data type:
   - `EventTableFetcher` - For event/export data
   - `MentionTableFetcher` - For mentions data
   - `GKGTableFetcher` - For Global Knowledge Graph data
3. **`GdeltUrlBuilder`** - URL construction and validation
4. **Type System** - Enums and configurations for type safety

### Type System

#### GDELT Versions
```rust
pub enum GdeltVersion {
    V2,  // GDELT v2
    V3,  // GDELT v3
}
```

#### Table Types
```rust
pub enum TableType {
    Export,   // Event data (uses .CSV.zip)
    Mentions, // Mentions data (uses .CSV.zip)
    Gkg,      // Global Knowledge Graph (uses .csv.zip)
}
```

#### File Extensions
```rust
pub enum FileExtension {
    Csv(CsvExtension),    // CSV files with compression variants
    Json(JsonExtension),  // JSON files with compression variants
}

pub enum CsvExtension {
    Upper, // .CSV.zip
    Lower, // .csv.zip
}

pub enum JsonExtension {
    Compressed,   // .json.zip
    Uncompressed, // .json
}
```

## Quick Start

### Basic Usage

```rust
use data::fetchers::gdelt::{EventTableFetcher, GdeltVersion};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an event table fetcher for GDELT v2
    let mut fetcher = EventTableFetcher::new_v2("./output", "./temp")?;
    
    // Fetch the latest events
    let events_iterator = fetcher.fetch_latest_async().await?;
    
    // Process events
    for event in events_iterator.take(100) {
        println!("Event ID: {:?}", event.global_event_id);
        // Process event data...
    }
    
    Ok(())
}
```

### Fetch Different Data Types

```rust
// Events (Export data)
let mut event_fetcher = EventTableFetcher::new_v2("./output", "./temp")?;
let events = event_fetcher.fetch_latest_async().await?;

// Mentions
let mut mention_fetcher = MentionTableFetcher::new_v3("./output", "./temp")?;
let mentions = mention_fetcher.fetch_latest_async().await?;

// GKG (Global Knowledge Graph)
let mut gkg_fetcher = GKGTableFetcher::new_v2("./output", "./temp")?;
let gkg_data = gkg_fetcher.fetch_latest_async().await?;
```

### Historical Data by Date

```rust
use chrono::NaiveDateTime;

let historical_date = NaiveDateTime::from_timestamp_opt(1640995200, 0).unwrap(); // 2022-01-01
let events = event_fetcher.fetch_date_async(historical_date).await?;
```

### Translation Data

```rust
// Fetch translation files
let translation_fetcher = EventTableFetcher::new_v2("./output", "./temp")?
    .with_translation(true);
    
let translated_events = translation_fetcher.fetch_latest_async().await?;
```

## Advanced Usage

### URL Building

```rust
use data::fetchers::gdelt::{GdeltUrlBuilder, GdeltVersion, TableType};
use chrono::NaiveDateTime;

let timestamp = NaiveDateTime::from_timestamp_opt(1704067200, 0).unwrap();

let url = GdeltUrlBuilder::new()
    .with_version(GdeltVersion::V2)
    .with_timestamp(timestamp)
    .with_table_type(TableType::Export)
    .with_translation(false)
    .build()?;

println!("Generated URL: {}", url);
// Output: http://data.gdeltproject.org/gdeltv2/20240101000000.export.CSV.zip
```

### Custom File Configurations

```rust
use data::fetchers::gdelt::{TableType, FileExtension, JsonExtension};

// Create custom configurations for JSON files (if supported)
let json_config = TableType::Export.with_custom_extension(
    FileExtension::Json(JsonExtension::Compressed)
);

// Use with low-level fetcher
let fetcher = GdeltFetcher::new_v2("./output", "./temp")?;
let data_path = fetcher.fetch_table_data_with_config(json_config, false).await?;
```

### Direct File Management

```rust
let fetcher = GdeltFetcher::new_v2("./output", "./temp")?;

// Get latest file list
let file_entries = fetcher.fetch_latest_file_list().await?;

// Filter for specific table type
let export_entries = fetcher.find_entries_by_criteria(
    &file_entries, 
    Some(TableType::Export), 
    Some(false), // not translation
    None         // any timestamp
);

// Download specific file with hash verification
if let Some(entry) = export_entries.first() {
    let file_path = fetcher.download_and_verify_file(entry).await?;
    println!("Downloaded: {:?}", file_path);
}
```

### Error Handling

```rust
use log::{warn, error};

match event_fetcher.fetch_latest_async().await {
    Ok(events) => {
        let count: usize = events.count();
        println!("Successfully processed {} events", count);
    }
    Err(e) => {
        error!("Failed to fetch events: {}", e);
        
        // Implement fallback strategy
        if let Some(local_file) = find_cached_file()? {
            warn!("Using cached data from: {:?}", local_file);
            // Process local file...
        }
    }
}
```

## File Management

### Local File Operations

```rust
// Get all local files for a table type
let local_events = fetcher.get_local_files_by_table(TableType::Export)?;

// Check if specific file exists
let timestamp = NaiveDateTime::from_timestamp_opt(1704067200, 0).unwrap();
if let Some(local_file) = fetcher.local_file_exists(TableType::Export, timestamp) {
    println!("File exists locally: {:?}", local_file);
}

// Clean up old files (keep only latest 5)
fetcher.cleanup_old_files(5)?;
```

### Version Compatibility

```rust
// Check URL compatibility
let url = "http://data.gdeltproject.org/gdeltv3/20240101000000.export.CSV.zip";
let is_compatible = fetcher.is_url_compatible(url);

// Convert URL between versions
let v2_url = "http://data.gdeltproject.org/gdeltv2/20240101000000.export.CSV.zip";
let v3_url = GdeltFetcher::convert_url_version(v2_url, GdeltVersion::V3);
```

## Configuration Options

### Fetcher Creation Options

```rust
// Standard constructors
EventTableFetcher::new_v2(output_dir, temp_dir)?     // GDELT v2
EventTableFetcher::new_v3(output_dir, temp_dir)?     // GDELT v3

// Custom version
EventTableFetcher::with_version(output_dir, temp_dir, GdeltVersion::V2)?

// With translation support
EventTableFetcher::new_v2(output_dir, temp_dir)?.with_translation(true)
```

### CSV Reader Configuration

```rust
// Custom CSV settings for special cases
let iterator = EventTableIterator::with_csv_config(
    file_path,
    b'\t',    // delimiter (tab)
    false     // has_headers
)?;
```

## Performance Considerations

### Memory Efficient Processing

```rust
// Process data in chunks to manage memory
let events = fetcher.fetch_latest_async().await?;

for chunk in events.chunks(1000) { // Process 1000 records at a time
    // Process chunk...
    
    // Allow other tasks to run
    tokio::task::yield_now().await;
}
```

### Concurrent Processing

```rust
use futures::future::join_all;

// Fetch multiple data types concurrently
let futures = vec![
    fetcher1.fetch_latest_async(),
    fetcher2.fetch_latest_async(),
    fetcher3.fetch_latest_async(),
];

let results = join_all(futures).await;
```

## Migration from Legacy System

### Old System
```rust
// Old approach
let db = GDELTDatabase::new(DatabaseType::Events).await?;
let csv_path = db.download_and_unzip(zip_path, output_dir).await?;
// Manual CSV parsing...
```

### New System
```rust
// New approach
let mut fetcher = EventTableFetcher::new_v2(output_dir, temp_dir)?;
let events = fetcher.fetch_latest_async().await?;
// Automatic parsing with iterator
```

## Best Practices

### 1. Resource Management
```rust
// Use temporary directories that clean up automatically
let _temp_dir = tempfile::TempDir::new()?;
let _output_dir = tempfile::TempDir::new()?;
```

### 2. Error Handling
```rust
// Always handle network errors gracefully
match fetcher.fetch_latest_async().await {
    Ok(data) => process_data(data),
    Err(e) => {
        log::warn!("Network fetch failed: {}", e);
        use_cached_data()?
    }
}
```

### 3. Logging
```rust
// Initialize logging for debugging
env_logger::init();

// Use appropriate log levels
log::info!("Starting data fetch...");
log::debug!("Processing record: {:?}", record);
log::warn!("Using fallback data source");
log::error!("Critical error: {}", error);
```

### 4. Data Validation
```rust
// The system automatically validates:
// - File sizes match expected values
// - MD5 hashes are correct
// - CSV structure is valid
// - Timestamps are parseable

// Additional validation in your code:
for event in events {
    if event.global_event_id.0 == 0 {
        log::warn!("Invalid event ID detected");
        continue;
    }
    // Process valid event...
}
```

## API Reference

### Core Types

#### `GdeltFetcher`
- `new(output_dir, temp_dir)` - Create with GDELT v2
- `new_v2(output_dir, temp_dir)` - Create with GDELT v2
- `new_v3(output_dir, temp_dir)` - Create with GDELT v3
- `new_with_version(output_dir, temp_dir, version)` - Create with specific version
- `fetch_latest_file_list()` - Get latest available files
- `fetch_master_file_list()` - Get complete file list
- `download_and_verify_file(entry)` - Download with hash validation
- `cleanup_old_files(keep_count)` - Clean up old files

#### `EventTableFetcher` / `MentionTableFetcher` / `GKGTableFetcher`
- `new_v2(output_dir, temp_dir)` - Create for GDELT v2
- `new_v3(output_dir, temp_dir)` - Create for GDELT v3
- `with_translation(bool)` - Enable/disable translation files
- `fetch_latest_async()` - Fetch latest data
- `fetch_date_async(date)` - Fetch historical data
- `count_latest_records()` - Count records in latest file

#### `GdeltUrlBuilder`
- `new()` - Create new builder
- `with_version(version)` - Set GDELT version
- `with_timestamp(timestamp)` - Set timestamp
- `with_table_type(table_type)` - Set table type
- `with_translation(bool)` - Enable translation
- `build()` - Build final URL

### Error Types

All functions return `anyhow::Result<T>` for comprehensive error handling.

Common error scenarios:
- Network connectivity issues
- File not found (historical dates)
- Hash validation failures
- Invalid timestamps or URLs
- Insufficient disk space

## Examples

See `/examples/fetch_gdelt_data.rs` for comprehensive usage examples covering:
- Basic data fetching
- Historical data retrieval
- Translation file handling
- URL building
- Error handling strategies
- Performance optimization

## Testing

Run the test suite with:
```bash
cargo test
```

For integration tests with actual network calls:
```bash
cargo test --release -- --ignored
```

## Dependencies

- `anyhow` - Error handling
- `chrono` - Date/time handling
- `csv` - CSV parsing
- `md5` - Hash verification
- `reqwest` - HTTP client
- `tokio` - Async runtime
- `url` - URL parsing
- `zip` - Archive extraction

## License

This project is licensed under the same terms as the parent project.

## Contributing

1. Follow Rust best practices
2. Add tests for new functionality
3. Update documentation
4. Ensure backwards compatibility where possible