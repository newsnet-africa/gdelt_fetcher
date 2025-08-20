# GDELT Fetcher - Data Module

The data module provides core functionality for fetching, downloading, and processing GDELT data from remote sources. It handles HTTP requests, file compression, temporary storage, and streaming data processing.

## Overview

This module contains the foundational data fetching infrastructure used by the GDELT Fetcher library. It provides abstract traits and concrete implementations for downloading and processing GDELT data files from the GDELT Project's servers.

## Features

- 🌐 **HTTP Client**: Robust HTTP client with retry logic and error handling
- 📦 **Compression Support**: Automatic handling of ZIP compressed GDELT files
- 🔄 **Streaming Processing**: Memory-efficient streaming for large datasets
- 📁 **File Management**: Intelligent temporary file handling and cleanup
- ⚡ **Async/Await**: Built with Tokio for high-performance async operations
- 🛡️ **Error Handling**: Comprehensive error handling with detailed context
- 📊 **Progress Tracking**: Built-in progress reporting for long-running operations

## Architecture

```
data/src/
├── lib.rs              # Module exports and common types
├── fetchers/           # Core fetching abstractions and implementations
│   ├── mod.rs
│   ├── traits.rs       # DataFetcher trait definition
│   └── gdelt/          # GDELT-specific implementations
│       ├── mod.rs
│       ├── base.rs     # Base GDELT fetcher functionality
│       ├── events.rs   # Event table fetcher
│       ├── mentions.rs # Mention table fetcher
│       └── gkg.rs      # GKG table fetcher
├── http/               # HTTP client and networking
│   ├── mod.rs
│   ├── client.rs       # HTTP client implementation
│   └── retry.rs        # Retry logic and backoff
├── compression/        # File compression/decompression
│   ├── mod.rs
│   └── zip.rs          # ZIP file handling
└── storage/            # File storage and management
    ├── mod.rs
    ├── temp.rs         # Temporary file management
    └── cache.rs        # File caching (future)
```

## Core Traits

### DataFetcher

The main trait for all data fetching operations:

```rust
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait DataFetcher<T> {
    /// Fetch the latest data asynchronously
    async fn fetch_latest_async(&mut self) -> Result<Box<dyn Iterator<Item = T>>>;
    
    /// Fetch data for a specific date
    async fn fetch_date_async(&mut self, date: chrono::NaiveDate) -> Result<Box<dyn Iterator<Item = T>>>;
    
    /// Get the output directory for processed files
    fn output_dir(&self) -> &str;
    
    /// Get the temporary directory for downloads
    fn temp_dir(&self) -> &str;
}
```

## GDELT-Specific Implementations

### GdeltFetcher

Base implementation for all GDELT data types:

```rust
pub struct GdeltFetcher {
    pub output_dir: String,
    pub temp_dir: String,
    pub table_type: TableType,
    pub version: GdeltVersion,
    pub file_extension: Box<dyn FileExtension>,
}

impl GdeltFetcher {
    pub fn new_v2<P: ToString>(
        output_dir: P,
        temp_dir: P,
        table_type: TableType,
        extension: Box<dyn FileExtension>,
    ) -> Result<Self> {
        // Implementation details...
    }
}
```

### Specialized Fetchers

#### EventTableFetcher
```rust
pub type EventTableFetcher = GdeltFetcher;

impl EventTableFetcher {
    pub fn new_v2<P: ToString>(output_dir: P, temp_dir: P) -> Result<Self> {
        GdeltFetcher::new_v2(
            output_dir,
            temp_dir,
            TableType::Events,
            Box::new(CsvExtension),
        )
    }
}
```

#### MentionTableFetcher
```rust
pub type MentionTableFetcher = GdeltFetcher;

impl MentionTableFetcher {
    pub fn new_v2<P: ToString>(output_dir: P, temp_dir: P) -> Result<Self> {
        GdeltFetcher::new_v2(
            output_dir,
            temp_dir,
            TableType::Mentions,
            Box::new(CsvExtension),
        )
    }
}
```

#### GKGTableFetcher
```rust
pub type GKGTableFetcher = GdeltFetcher;

impl GKGTableFetcher {
    pub fn new_v2<P: ToString>(output_dir: P, temp_dir: P) -> Result<Self> {
        GdeltFetcher::new_v2(
            output_dir,
            temp_dir,
            TableType::GKG,
            Box::new(CsvExtension),
        )
    }
}
```

## Configuration

### Table Types
```rust
pub enum TableType {
    Events,
    Mentions,
    GKG,
}
```

### GDELT Versions
```rust
pub enum GdeltVersion {
    V1,
    V2,
}
```

### File Extensions
```rust
pub trait FileExtension {
    fn extension(&self) -> &str;
    fn content_type(&self) -> &str;
}

pub struct CsvExtension;
pub struct JsonExtension;
```

## HTTP Client

### Features
- Automatic retry with exponential backoff
- Request timeout handling
- Response streaming for large files
- Progress reporting
- Error classification and handling

### Usage
```rust
use crate::http::HttpClient;

let client = HttpClient::new()?;
let response = client.get("https://api.gdeltproject.org/api/v2/lastupdate")
    .timeout(Duration::from_secs(30))
    .send()
    .await?;
```

## File Processing

### Compression Handling
```rust
use crate::compression::ZipProcessor;

let processor = ZipProcessor::new();
let extracted_files = processor.extract_all(&zip_path, &output_dir).await?;
```

### Temporary File Management
```rust
use crate::storage::TempFileManager;

let temp_manager = TempFileManager::new(&temp_dir);
let temp_file = temp_manager.create_temp_file("gdelt_events")?;

// File is automatically cleaned up when temp_file is dropped
```

## Error Handling

The module uses `anyhow::Result` for comprehensive error handling:

```rust
use anyhow::{Context, Result};

pub async fn download_file(url: &str, path: &Path) -> Result<()> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to fetch URL: {}", url))?;
    
    let bytes = response.bytes()
        .await
        .context("Failed to read response body")?;
    
    tokio::fs::write(path, &bytes)
        .await
        .with_context(|| format!("Failed to write file: {}", path.display()))?;
    
    Ok(())
}
```

## Performance Considerations

### Memory Usage
- Streaming processors to handle large files without loading into memory
- Lazy iterators for data processing
- Automatic cleanup of temporary files

### Network Efficiency
- Connection pooling and reuse
- Compression-aware downloads
- Parallel downloads for multiple files

### Disk I/O
- Asynchronous file operations
- Temporary file management with automatic cleanup
- Configurable buffer sizes for streaming

## Testing

### Unit Tests
```bash
cd data
cargo test
```

### Integration Tests
```bash
# Test with real GDELT servers (requires network)
cargo test --features network_tests
```

### Mock Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};

    #[tokio::test]
    async fn test_fetch_with_mock_server() {
        let mock_server = MockServer::start().await;
        
        Mock::given(method("GET"))
            .and(path("/api/v2/lastupdate"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_string("mock response"))
            .mount(&mock_server)
            .await;

        // Test implementation...
    }
}
```

## Development

### Adding New Data Sources

1. Implement the `DataFetcher` trait:
```rust
pub struct CustomDataFetcher {
    // Fields...
}

#[async_trait]
impl DataFetcher<CustomDataType> for CustomDataFetcher {
    async fn fetch_latest_async(&mut self) -> Result<Box<dyn Iterator<Item = CustomDataType>>> {
        // Implementation...
    }
}
```

2. Add URL generation logic
3. Implement parsing for the data format
4. Add tests for the new fetcher

### Extending File Format Support

1. Implement the `FileExtension` trait:
```rust
pub struct XmlExtension;

impl FileExtension for XmlExtension {
    fn extension(&self) -> &str { "xml" }
    fn content_type(&self) -> &str { "application/xml" }
}
```

2. Add parsing logic in the models crate
3. Update fetcher constructors to support the new format

## TODO

### High Priority
- [ ] Add retry logic with exponential backoff for failed downloads
- [ ] Implement resume capability for interrupted downloads
- [ ] Add progress reporting for large file downloads
- [ ] Implement connection pooling for better performance
- [ ] Add support for conditional requests (If-Modified-Since)

### Medium Priority
- [ ] Add caching layer for frequently accessed data
- [ ] Implement rate limiting to respect server limits
- [ ] Add support for parallel downloads
- [ ] Create metrics collection for monitoring
- [ ] Add support for custom HTTP headers

### Low Priority
- [ ] Add support for GDELT 1.0 format
- [ ] Implement data validation during download
- [ ] Add support for webhook-based updates
- [ ] Create admin tools for cache management
- [ ] Add support for custom data transformations

### Performance
- [ ] Optimize memory usage for large files
- [ ] Add benchmarks for download performance
- [ ] Implement adaptive timeout based on file size
- [ ] Add support for streaming decompression
- [ ] Optimize temporary file handling

### Testing
- [ ] Add property-based tests for HTTP client
- [ ] Create integration tests with real GDELT servers
- [ ] Add chaos engineering tests for network failures
- [ ] Implement performance regression tests
- [ ] Add mock servers for consistent testing

## Dependencies

### Core Dependencies
- `reqwest` - HTTP client with async support
- `tokio` - Async runtime
- `anyhow` - Error handling
- `url` - URL parsing and manipulation
- `zip` - ZIP file compression/decompression

### Development Dependencies
- `tempfile` - Temporary file creation for tests
- `wiremock` - HTTP mocking for tests

## Contributing

1. Follow the existing code style and patterns
2. Add tests for new functionality
3. Update documentation for API changes
4. Ensure all tests pass before submitting PR
5. Add appropriate error handling and logging

## License

This module is part of the GDELT Fetcher project and is licensed under the MIT License.