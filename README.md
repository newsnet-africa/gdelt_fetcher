# GDELT Fetcher

GDELT data fetching and processing library for the NewsNet decentralized news aggregation platform.

## Project Status: Basic Implementation Complete

The gdelt_fetcher crate provides foundational GDELT data access with basic fetching and parsing capabilities, but requires significant development for production use.

## Completed Components

### Simplified API (New!)
- [x] Clean, high-level API in `gdelt_fetcher::api` module:
  - `fetch_latest_events()` - Fetch the most recent events
  - `fetch_latest_mentions()` - Fetch the most recent mentions
  - `fetch_latest_gkg()` - Fetch the most recent GKG data
  - `fetch_events_by_date(date)` - Fetch events for specific date
  - `fetch_mentions_by_date(date)` - Fetch mentions for specific date
  - `fetch_gkg_by_date(date)` - Fetch GKG data for specific date
  - `fetch_all_latest()` - Fetch all data types concurrently
  - `fetch_all_by_date(date)` - Fetch all data types for specific date
- [x] All functions return vectors of strongly-typed data structures
- [x] Timezone-aware date handling with automatic UTC conversion
- [x] Comprehensive documentation and examples

### Main Library Interface
- [x] High-level convenience functions for fetching latest data:
  - `fetch_and_parse_events()` - Async function for latest GDELT events
  - `fetch_and_parse_mentions()` - Async function for latest mentions
  - `fetch_and_parse_gkg()` - Async function for Global Knowledge Graph data
- [x] Temporary directory management and cleanup utilities
- [x] Basic error handling with anyhow integration
- [x] Re-exports of core types and fetchers for convenience

### Data Models (models/ crate)
- [x] Complete type definitions for three main GDELT tables:
  - `EventTable` - Political, social, and conflict events
  - `MentionTable` - News mentions and source tracking
  - `GKGTable` - Global Knowledge Graph semantic data
- [x] GCAM (Global Content Analysis Measures) integration
  - Lookup types and code mappings
  - CSV parsing utilities
- [x] Rich type system with proper Rust representations
- [x] Serialization support with serde and bincode
- [x] Date/time handling with chrono integration

### Data Fetching (data/ crate)
- [x] Core fetching abstractions with `DataFetcher` trait
- [x] GDELT-specific fetcher implementations:
  - `EventTableFetcher` - Events data fetcher
  - `MentionTableFetcher` - Mentions data fetcher
  - `GKGTableFetcher` - GKG data fetcher
  - `GdeltFetcher` - Base implementation
- [x] File extension abstractions (CSV, JSON support planned)
- [x] GDELT version handling (V1, V2)
- [x] Table type enumeration and classification
- [x] Basic BigQuery fetcher structure (incomplete implementation)

### Utility Scripts
- [x] GCAM database population script (`populate_gcam_db.rs`)
- [x] CSV encoding fix utility (`fix_csv_encoding.rs`)
- [x] GCAM enrichment verification script (`verify_gcam_enrichment.rs`)

## Quick Start

### Using the Simplified API (Recommended)

```rust
use gdelt_fetcher::api::{fetch_latest_events, fetch_events_by_date};
use chrono::{Utc, TimeZone};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fetch latest events
    let events = fetch_latest_events().await?;
    println!("Fetched {} latest events", events.len());

    // Fetch events for a specific date
    let date = Utc.with_ymd_and_hms(2024, 1, 15, 0, 0, 0).unwrap();
    let historical_events = fetch_events_by_date(date).await?;
    println!("Fetched {} events for 2024-01-15", historical_events.len());

    // Fetch all data types at once
    let (events, mentions, gkg_data) = fetch_all_latest().await?;
    println!("Fetched {} events, {} mentions, {} GKG records",
             events.len(), mentions.len(), gkg_data.len());

    Ok(())
}
```

See `examples/simple_api_example.rs` for comprehensive usage examples and `API_README.md` for detailed API documentation.

### Using the Lower-Level Interface

```rust
use gdelt_fetcher::{fetch_and_parse_events, fetch_and_parse_mentions, fetch_and_parse_gkg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let events = fetch_and_parse_events().await?;
    let mentions = fetch_and_parse_mentions().await?;
    let gkg_data = fetch_and_parse_gkg().await?;
    
    println!("Fetched {} events, {} mentions, {} GKG records",
             events.len(), mentions.len(), gkg_data.len());
    Ok(())
}
```

## TODO

### Critical Infrastructure Issues
- [ ] Fix GCAM codebook indexing system
  - [ ] Replace current hack with proper key-value store implementation
  - [ ] Implement in-memory enum mapping for GCAM codes
  - [ ] Add on-demand lookup with caching for better performance
  - [ ] Create proper validation for GCAM code enrichment
  - [ ] Add error handling for missing or invalid GCAM codes

### Core Functionality Completion
- [ ] Complete HTTP client implementation in data crate
  - [ ] Add proper connection pooling and reuse
  - [ ] Implement retry logic with exponential backoff
  - [ ] Add timeout handling and adaptive timeouts
  - [ ] Support for conditional requests (If-Modified-Since)
  - [ ] Add progress reporting for large downloads
- [ ] File handling and compression improvements
  - [ ] Implement proper ZIP file extraction and validation
  - [ ] Add file hash verification and integrity checking
  - [ ] Support for resume capability on interrupted downloads
  - [ ] Add streaming decompression for memory efficiency
- [ ] Parser improvements and abstractions
  - [ ] Abstract parser by input type (CSV, JSON, XML)
  - [ ] Add configurable CSV parsing options
  - [ ] Implement streaming parsers for very large files
  - [ ] Add data validation during parsing process

### Data Model Enhancements
- [ ] Complete field coverage for all GDELT tables
  - [ ] Verify all Event table fields are properly mapped
  - [ ] Complete Mention table field implementations
  - [ ] Finish GKG table complex field parsing
  - [ ] Add support for nested and optional fields
- [ ] Data validation and quality assurance
  - [ ] Implement field-level validation rules
  - [ ] Add cross-field consistency checking
  - [ ] Create data quality metrics and reporting
  - [ ] Add anomaly detection for unusual data patterns
- [ ] Schema evolution support
  - [ ] Handle GDELT format changes gracefully
  - [ ] Add migration utilities between versions
  - [ ] Support for partial schema updates

### Performance and Scalability
- [ ] Memory optimization for large datasets
  - [ ] Implement zero-copy parsing where possible
  - [ ] Add streaming processing for large files
  - [ ] Use memory-mapped files for very large datasets
  - [ ] Optimize temporary file handling and cleanup
- [ ] Parallel processing capabilities
  - [ ] Add concurrent downloads for multiple files
  - [ ] Implement parallel parsing of large datasets
  - [ ] Support for batch processing multiple date ranges
  - [ ] Add work-stealing for load distribution
- [ ] Caching and storage optimization
  - [ ] Implement intelligent caching with TTL
  - [ ] Add support for different cache backends
  - [ ] Create cache warming strategies
  - [ ] Add data deduplication to reduce storage

### BigQuery Integration
- [ ] Complete BigQuery implementation for large-scale processing
  - [ ] Add Google Cloud authentication integration
  - [ ] Implement optimized query generation for GDELT BigQuery tables
  - [ ] Add result streaming and pagination handling
  - [ ] Create cost optimization through intelligent query planning
  - [ ] Support for bulk data exports and imports

### API and Usability Improvements
- [ ] Create consolidated high-level API
  - [ ] Builder pattern for fetcher configuration
  - [ ] Unified interface for all table types with generic support
  - [ ] Configurable retry and timeout policies
  - [ ] Connection pooling and resource management
- [ ] Configuration system enhancement
  - [ ] Environment-based configuration with validation
  - [ ] Support for configuration files (TOML, JSON, YAML)
  - [ ] Runtime configuration updates without restart
  - [ ] Configuration templates for common use cases

### Testing and Quality Assurance
- [ ] Comprehensive testing framework
  - [ ] Unit tests for all core functionality with high coverage
  - [ ] Integration tests with real GDELT data samples
  - [ ] Property-based testing for data validation
  - [ ] Performance regression testing and benchmarking
  - [ ] Chaos engineering tests for network failures
- [ ] Mock and testing infrastructure
  - [ ] Mock GDELT servers for consistent testing
  - [ ] Test data generators for various scenarios
  - [ ] Automated testing across multiple Rust versions
  - [ ] Memory leak detection and profiling

### Documentation and Examples
- [ ] Comprehensive documentation system
  - [ ] API documentation with extensive examples
  - [ ] Architecture decision records for design choices
  - [ ] Performance tuning guides and best practices
  - [ ] Troubleshooting guides for common issues
- [ ] Working examples and tutorials
  - [ ] Basic usage examples for each table type
  - [ ] Advanced configuration and customization examples
  - [ ] Integration examples with data analysis frameworks
  - [ ] Real-world use case demonstrations


