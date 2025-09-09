# GDELT Fetcher - Models Module

Data models and type definitions for GDELT (Global Database of Events, Language, and Tone) data tables. Provides strongly-typed Rust representations with serialization and parsing capabilities.

## Project Status: Core Types Complete

The models module provides comprehensive type definitions for GDELT data structures but requires enhancements for parsing, validation, and utility functions.

## Completed Components

### Core Data Structures
- [x] Complete type definitions for three main GDELT tables:
  - `EventTable` - Political, social, and conflict events worldwide
  - `MentionTable` - News mentions and source tracking references
  - `GKGTable` - Global Knowledge Graph semantic information
- [x] Rich type system with proper Rust field representations
- [x] Serialization support with serde derive macros
- [x] Date/time handling with chrono integration
- [x] Optional field handling for nullable GDELT columns

### GCAM Integration
- [x] GCAM (Global Content Analysis Measures) infrastructure:
  - GCAM lookup types and code mappings
  - CSV parsing utilities for GCAM data
  - Basic GCAM code enumeration and classification
- [x] GCAM database integration utilities
- [x] Support for GCAM enrichment and validation

### Supporting Type System
- [x] Lookup types for GDELT code classifications
- [x] Geographic data structures with coordinate handling
- [x] Actor information with demographic and role coding
- [x] Event classification with CAMEO taxonomy support
- [x] Tone analysis and confidence scoring types

### Serialization and Parsing
- [x] Serde integration for JSON serialization
- [x] Bincode support for efficient binary serialization
- [x] Basic CSV field parsing utilities
- [x] Date format parsing for GDELT timestamp formats

## TODO

### Critical Parsing and Validation Issues
- [ ] Complete CSV parsing implementation
  - [ ] Fix field parsing for complex nested data structures
  - [ ] Add proper delimiter and escaping handling for CSV fields
  - [ ] Implement streaming CSV parser for large datasets
  - [ ] Add error recovery for malformed CSV records
  - [ ] Support for different CSV dialects and formats

- [ ] Data validation and quality assurance
  - [ ] Add field-level validation rules for all data types
  - [ ] Implement cross-field consistency checking
  - [ ] Create data completeness scoring and validation
  - [ ] Add range validation for numeric fields (coordinates, scores, etc.)
  - [ ] Implement enum validation for coded fields

### Field Coverage and Completeness
- [ ] Verify complete field coverage for all GDELT tables
  - [ ] Audit EventTable fields against latest GDELT schema
  - [ ] Complete MentionTable field implementations and parsing
  - [ ] Finish GKGTable complex field parsing (themes, locations, etc.)
  - [ ] Add support for nested and hierarchical data structures
  - [ ] Handle optional and conditional fields properly

- [ ] GDELT format evolution support
  - [ ] Add migration utilities between GDELT versions (1.0 vs 2.0)
  - [ ] Handle schema changes and field additions gracefully
  - [ ] Support for partial schema updates and backwards compatibility
  - [ ] Add format auto-detection for mixed datasets

### GCAM System Improvements
- [ ] Fix GCAM codebook integration issues
  - [ ] Replace current implementation with proper key-value lookup system
  - [ ] Add in-memory GCAM code caching for performance
  - [ ] Implement GCAM code validation and enrichment utilities
  - [ ] Create proper error handling for missing GCAM codes
  - [ ] Add GCAM metadata integration with main data structures

- [ ] Enhanced GCAM functionality
  - [ ] Add GCAM score calculation and normalization
  - [ ] Implement GCAM trend analysis utilities
  - [ ] Create GCAM code hierarchy and relationship mapping
  - [ ] Add GCAM-specific query and filtering capabilities

### Type System Enhancements
- [ ] Advanced type safety improvements
  - [ ] Add NewType wrappers for domain-specific identifiers
  - [ ] Implement type-safe coordinate and geographic data handling
  - [ ] Create strongly-typed event and actor code enumerations
  - [ ] Add validation traits for custom data types

- [ ] Performance optimizations
  - [ ] Implement zero-copy parsing where possible
  - [ ] Add string interning for repeated values (country codes, actor types)
  - [ ] Optimize memory layout for large data structures
  - [ ] Use compact representations for boolean and enum fields

### Utility Functions and Helpers
- [ ] Date and time utilities
  - [ ] Add comprehensive GDELT date format parsing
  - [ ] Implement timezone handling and conversion utilities
  - [ ] Create date range validation and normalization
  - [ ] Add temporal query helpers and date math functions

- [ ] Geographic and spatial utilities
  - [ ] Add coordinate validation and normalization
  - [ ] Implement geographic distance calculations
  - [ ] Create geospatial indexing and lookup utilities
  - [ ] Add support for different coordinate systems and projections

- [ ] Text processing utilities
  - [ ] Add text normalization and cleaning functions
  - [ ] Implement language detection and encoding handling
  - [ ] Create text similarity and matching utilities
  - [ ] Add support for multilingual content processing

### Testing and Quality Assurance
- [ ] Comprehensive testing framework
  - [ ] Unit tests for all data structures with high coverage
  - [ ] Property-based testing for data validation rules
  - [ ] Round-trip serialization testing for all formats
  - [ ] Performance benchmarks for parsing and serialization
  - [ ] Fuzzing tests for parser robustness

- [ ] Test data and fixtures
  - [ ] Create comprehensive test datasets for all table types
  - [ ] Add edge case and malformed data test samples
  - [ ] Generate synthetic data for stress testing
  - [ ] Add regression test data for schema changes

### Documentation and Examples
- [ ] Comprehensive field documentation
  - [ ] Document all fields with descriptions and examples
  - [ ] Add GDELT field mapping and transformation notes
  - [ ] Create field validation rule documentation
  - [ ] Add examples of complex data structures and parsing

- [ ] Usage examples and guides
  - [ ] Basic parsing examples for each table type
  - [ ] Advanced data manipulation and transformation examples
  - [ ] Integration examples with analysis frameworks
  - [ ] Performance optimization examples and best practices

### Production Features
- [ ] Error handling improvements
  - [ ] Create specific error types for different validation failures
  - [ ] Add detailed error context and debugging information
  - [ ] Implement error recovery strategies for partial data corruption
  - [ ] Add logging and monitoring for data quality issues

- [ ] Schema evolution and migration
  - [ ] Add schema versioning and migration utilities
  - [ ] Create data transformation pipelines for format changes
  - [ ] Implement backwards compatibility layers
  - [ ] Add automated migration testing and validation

### Advanced Features
- [ ] Machine learning integration helpers
  - [ ] Feature extraction utilities for ML pipelines
  - [ ] Data normalization and standardization for ML models
  - [ ] Time series preparation and windowing functions
  - [ ] Graph construction utilities for network analysis

- [ ] Analytics and aggregation utilities
  - [ ] Pre-built aggregation functions for common analysis patterns
  - [ ] Time-based windowing and bucketing utilities
  - [ ] Geographic aggregation and spatial analysis helpers
  - [ ] Actor and entity relationship extraction utilities

## Development Commands

```bash
# Build the models module
cargo build -p models

# Run tests with coverage
cargo test -p models

# Run with logging to debug parsing issues
RUST_LOG=debug cargo test -p models

# Check for unused dependencies
cargo machete models/

# Run clippy for additional checks
cargo clippy -p models
```

## Current Structure

```
models/src/
├── lib.rs                          # Module exports and common utilities
├── types/                          # Core data type definitions
│   ├── mod.rs                     # Type module exports
│   ├── event_table.rs             # EventTable structure and fields
│   ├── mention_table.rs           # MentionTable structure and fields
│   ├── gkg_table.rs              # GKGTable structure and fields
│   └── lookup_types/             # Supporting lookup and enum types
│       └── [various lookup files]
└── gcam/                          # GCAM integration and utilities
    └── [GCAM-related files]
```

## Dependencies

### Core Dependencies
- `serde` - Serialization framework with derive support
- `chrono` - Date and time handling with serde integration
- `csv` - CSV parsing and writing capabilities
- `anyhow` - Error handling with rich context

### Development Dependencies
- `bincode` - Binary serialization for testing
- `serde_json` - JSON serialization for debugging

## Architecture Notes

The models are designed to closely mirror GDELT's data schema while providing Rust-native ergonomics. All major tables support both human-readable JSON serialization for debugging and efficient binary serialization for storage and network transmission.

The type system emphasizes correctness and validation while maintaining performance for large datasets. Optional fields are used extensively to handle GDELT's variable data completeness.

## License

Part of the NewsNet project, licensed under MIT License.