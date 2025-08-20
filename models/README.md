# GDELT Fetcher - Models Module

The models module defines the data structures and type definitions for GDELT (Global Database of Events, Language, and Tone) data tables. It provides strongly-typed representations of GDELT's three main data streams with serialization, validation, and parsing capabilities.

## Overview

This module contains the core data models used throughout the GDELT Fetcher ecosystem. It defines type-safe structures for Events, Mentions, and Global Knowledge Graph (GKG) data, along with utilities for parsing, validation, and serialization.

## Features

- 📊 **Type-Safe Models**: Strongly-typed representations of all GDELT data fields
- 🔄 **Serde Integration**: Built-in serialization/deserialization support
- ✅ **Data Validation**: Field validation and constraint checking
- 📝 **CSV Parsing**: Direct parsing from GDELT CSV format
- 🗜️ **Compression Support**: Bincode serialization for efficient storage
- 📅 **Date/Time Handling**: Proper handling of GDELT's date/time formats
- 🌍 **Geographic Data**: Structured geographic information with coordinates
- 🔢 **Numeric Types**: Appropriate numeric types for scores and measurements

## Architecture

```
models/src/
├── lib.rs              # Module exports and common utilities
├── types/              # Core data type definitions
│   ├── mod.rs
│   ├── event_table.rs  # Event table structure
│   ├── mention_table.rs # Mention table structure
│   ├── gkg_table.rs    # GKG table structure
│   └── common.rs       # Shared field types
├── parsing/            # Data parsing utilities
│   ├── mod.rs
│   ├── csv_parser.rs   # CSV parsing logic
│   ├── date_parser.rs  # Date/time parsing
│   └── numeric_parser.rs # Numeric field parsing
├── validation/         # Data validation
│   ├── mod.rs
│   ├── field_validation.rs # Individual field validation
│   └── record_validation.rs # Full record validation
└── utils/              # Utility functions
    ├── mod.rs
    ├── formatting.rs   # Output formatting
    └── constants.rs    # GDELT constants and enums
```

## Core Data Types

### EventTable

Represents a single GDELT event record with comprehensive metadata:

```rust
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventTable {
    /// Unique event identifier
    pub event_id: String,
    
    /// Date of the event (YYYYMMDD format)
    pub event_date: NaiveDate,
    
    /// Date the event was added to GDELT
    pub date_added: NaiveDateTime,
    
    /// Actor 1 information
    pub actor1: Actor,
    
    /// Actor 2 information
    pub actor2: Actor,
    
    /// Event classification
    pub event_code: EventCode,
    
    /// Base event code (root of CAMEO taxonomy)
    pub event_base_code: String,
    
    /// Event root code (top-level CAMEO category)
    pub event_root_code: String,
    
    /// Quad class (material vs. verbal cooperation/conflict)
    pub quad_class: QuadClass,
    
    /// Goldstein scale score (-10 to +10)
    pub goldstein_scale: Option<f64>,
    
    /// Number of mentions of this event
    pub num_mentions: u32,
    
    /// Number of sources reporting this event
    pub num_sources: u32,
    
    /// Number of articles mentioning this event
    pub num_articles: u32,
    
    /// Average tone of coverage (-100 to +100)
    pub avg_tone: Option<f64>,
    
    /// Geographic information
    pub geography: EventGeography,
    
    /// Additional metadata
    pub metadata: EventMetadata,
}
```

### MentionTable

Represents mentions of events in news sources:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MentionTable {
    /// Event ID this mention refers to
    pub event_id: String,
    
    /// Global Event ID (GDELT's internal identifier)
    pub global_event_id: String,
    
    /// Date and time the mention was published
    pub date_time_published: NaiveDateTime,
    
    /// Source information
    pub source: MentionSource,
    
    /// URL of the source document
    pub source_url: Option<String>,
    
    /// Tone of this specific mention
    pub mention_tone: Option<f64>,
    
    /// Type of mention (direct, indirect, etc.)
    pub mention_type: MentionType,
    
    /// Confidence score for event extraction
    pub confidence: Option<f64>,
    
    /// Character offset of mention in source text
    pub char_offset: Option<u32>,
    
    /// Additional source metadata
    pub source_metadata: SourceMetadata,
}
```

### GKGTable (Global Knowledge Graph)

Enhanced semantic information about events:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GKGTable {
    /// Date of the GKG record
    pub date: NaiveDate,
    
    /// Number of the 15-minute update this record appeared in
    pub num_update: u32,
    
    /// Source URL
    pub source_url: String,
    
    /// Source common name
    pub source_name: Option<String>,
    
    /// Document identifier
    pub document_id: Option<String>,
    
    /// V2 Counts (theme counts)
    pub v2_counts: Vec<ThemeCount>,
    
    /// V1 Themes (legacy theme list)
    pub v1_themes: Vec<String>,
    
    /// V2 Enhanced Themes with confidence scores
    pub v2_themes: Vec<EnhancedTheme>,
    
    /// V1 Locations (legacy location list)
    pub v1_locations: Vec<String>,
    
    /// V2 Enhanced Locations with coordinates
    pub v2_locations: Vec<EnhancedLocation>,
    
    /// Person names mentioned
    pub persons: Vec<String>,
    
    /// Organization names mentioned
    pub organizations: Vec<String>,
    
    /// V2 Tone information
    pub v2_tone: GKGTone,
    
    /// V2 Enhanced dates mentioned in text
    pub v2_dates: Vec<EnhancedDate>,
    
    /// GCAM information (if available)
    pub gcam: Option<GCAMData>,
    
    /// Social media metrics
    pub social_metrics: Option<SocialMetrics>,
    
    /// Additional quotations
    pub quotations: Vec<Quotation>,
    
    /// All names mentioned (persons + organizations)
    pub all_names: Vec<String>,
    
    /// Amounts mentioned in text
    pub amounts: Vec<Amount>,
    
    /// Translation information
    pub translation_info: Option<TranslationInfo>,
    
    /// Additional metadata
    pub extras: Option<serde_json::Value>,
}
```

## Supporting Types

### Actor Information

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Actor {
    /// Three-character CAMEO actor code
    pub code: Option<String>,
    
    /// Human-readable actor name
    pub name: Option<String>,
    
    /// Country code for actor
    pub country_code: Option<String>,
    
    /// Known group code
    pub known_group_code: Option<String>,
    
    /// Ethnic code
    pub ethnic_code: Option<String>,
    
    /// Religion code 1
    pub religion1_code: Option<String>,
    
    /// Religion code 2
    pub religion2_code: Option<String>,
    
    /// Type code 1
    pub type1_code: Option<String>,
    
    /// Type code 2
    pub type2_code: Option<String>,
    
    /// Type code 3
    pub type3_code: Option<String>,
}
```

### Geographic Information

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventGeography {
    /// Action geo type
    pub action_geo_type: Option<GeoType>,
    
    /// Action geo fullname
    pub action_geo_fullname: Option<String>,
    
    /// Action geo country code
    pub action_geo_country_code: Option<String>,
    
    /// Action geo ADM1 code
    pub action_geo_adm1_code: Option<String>,
    
    /// Action geo ADM2 code
    pub action_geo_adm2_code: Option<String>,
    
    /// Action geo latitude
    pub action_geo_lat: Option<f64>,
    
    /// Action geo longitude
    pub action_geo_long: Option<f64>,
    
    /// Action geo feature ID
    pub action_geo_feature_id: Option<String>,
    
    /// Actor 1 geographic information
    pub actor1_geo: Option<ActorGeography>,
    
    /// Actor 2 geographic information
    pub actor2_geo: Option<ActorGeography>,
}
```

### Enums and Constants

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QuadClass {
    VerbalCooperation = 1,
    MaterialCooperation = 2,
    VerbalConflict = 3,
    MaterialConflict = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeoType {
    Country = 1,
    State = 2,
    City = 3,
    WorldCity = 4,
    WorldState = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MentionType {
    Direct,
    Indirect,
    Quote,
    Reference,
}
```

## Parsing and Validation

### CSV Parsing

The module provides robust CSV parsing with error handling:

```rust
use crate::parsing::CsvParser;
use crate::types::EventTable;

let parser = CsvParser::new();
let events: Vec<EventTable> = parser.parse_events_from_reader(csv_reader)?;
```

### Field Validation

```rust
use crate::validation::FieldValidator;

let validator = FieldValidator::new();

// Validate individual fields
assert!(validator.validate_event_date("20240315")?);
assert!(validator.validate_goldstein_scale(Some(5.2))?);
assert!(validator.validate_coordinates(Some(40.7128), Some(-74.0060))?);
```

### Record Validation

```rust
use crate::validation::RecordValidator;

let validator = RecordValidator::new();
let event = EventTable { /* ... */ };

match validator.validate_event(&event) {
    Ok(()) => println!("Event is valid"),
    Err(errors) => {
        for error in errors {
            eprintln!("Validation error: {}", error);
        }
    }
}
```

## Serialization

### Bincode (Binary)

```rust
use bincode;

let event = EventTable { /* ... */ };

// Serialize
let encoded: Vec<u8> = bincode::serialize(&event)?;

// Deserialize
let decoded: EventTable = bincode::deserialize(&encoded)?;
```

### JSON

```rust
use serde_json;

let event = EventTable { /* ... */ };

// Serialize
let json = serde_json::to_string(&event)?;

// Deserialize
let event: EventTable = serde_json::from_str(&json)?;
```

### CSV Output

```rust
use crate::utils::CsvFormatter;

let events = vec![/* EventTable instances */];
let formatter = CsvFormatter::new();
let csv_output = formatter.format_events(&events)?;
```

## Utilities

### Date/Time Formatting

```rust
use crate::utils::DateFormatter;

let formatter = DateFormatter::new();

// GDELT date format (YYYYMMDD)
let gdelt_date = formatter.to_gdelt_date(chrono::Utc::now().date_naive());

// Parse GDELT datetime (YYYYMMDDHHMMSS)
let datetime = formatter.parse_gdelt_datetime("20240315143022")?;
```

### Geographic Utilities

```rust
use crate::utils::GeoUtils;

let geo_utils = GeoUtils::new();

// Calculate distance between two points
let distance = geo_utils.haversine_distance(
    (40.7128, -74.0060), // New York
    (51.5074, -0.1278)   // London
);

// Validate coordinates
assert!(geo_utils.validate_coordinates(40.7128, -74.0060));
```

## Error Handling

The module provides comprehensive error handling:

```rust
use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    
    #[error("Invalid numeric value: {0}")]
    InvalidNumeric(String),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid enum value: {0}")]
    InvalidEnum(String),
}

pub fn parse_event_date(date_str: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y%m%d")
        .with_context(|| format!("Failed to parse event date: {}", date_str))
        .map_err(|e| ParseError::InvalidDate(e.to_string()).into())
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_event_table_creation() {
        let event = EventTable {
            event_id: "123456789".to_string(),
            event_date: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            // ... other fields
        };
        
        assert_eq!(event.event_id, "123456789");
        assert_eq!(event.event_date.year(), 2024);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = EventTable { /* ... */ };
        
        let serialized = bincode::serialize(&original).unwrap();
        let deserialized: EventTable = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(original, deserialized);
    }
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_goldstein_scale_range(scale in -10.0f64..=10.0f64) {
            let validator = FieldValidator::new();
            assert!(validator.validate_goldstein_scale(Some(scale)).is_ok());
        }

        #[test]
        fn test_coordinates_validation(
            lat in -90.0f64..=90.0f64,
            lon in -180.0f64..=180.0f64
        ) {
            let validator = FieldValidator::new();
            assert!(validator.validate_coordinates(Some(lat), Some(lon)).is_ok());
        }
    }
}
```

## Performance Considerations

### Memory Efficiency

- Use `Option<T>` for nullable fields to save memory
- Implement `Clone` efficiently for large structures
- Use string interning for repeated values like country codes

### Parsing Performance

- Stream parsing for large CSV files
- Lazy field parsing where possible
- Efficient string-to-enum conversions

### Serialization Performance

- Bincode for fast binary serialization
- Avoid unnecessary allocations during parsing
- Use zero-copy parsing where possible

## TODO

### High Priority
- [ ] Add support for GDELT 1.0 data format compatibility
- [ ] Implement incremental parsing for streaming data
- [ ] Add comprehensive field validation with custom rules
- [ ] Create builder patterns for complex structures
- [ ] Add support for custom field mappings

### Medium Priority
- [ ] Implement data compression for storage efficiency
- [ ] Add support for partial record updates
- [ ] Create conversion utilities between GDELT versions
- [ ] Add geographical region grouping utilities
- [ ] Implement time series aggregation functions

### Low Priority
- [ ] Add support for custom serialization formats
- [ ] Create data anonymization utilities
- [ ] Add statistical analysis helper functions
- [ ] Implement data quality metrics
- [ ] Add support for custom field extensions

### Documentation
- [ ] Add comprehensive field documentation with examples
- [ ] Create data model diagrams and visualizations
- [ ] Add migration guides between GDELT versions
- [ ] Create best practices guide for data handling
- [ ] Add performance tuning documentation

### Testing
- [ ] Add integration tests with real GDELT data samples
- [ ] Implement fuzzing tests for parser robustness
- [ ] Add performance benchmarks for parsing operations
- [ ] Create comprehensive validation test suite
- [ ] Add regression tests for data format changes

## Dependencies

### Core Dependencies
- `serde` - Serialization framework
- `chrono` - Date and time handling
- `csv` - CSV parsing and writing
- `anyhow` - Error handling
- `thiserror` - Custom error types

### Optional Dependencies
- `bincode` - Binary serialization
- `serde_json` - JSON serialization
- `geo` - Geographic calculations (future)

### Development Dependencies
- `proptest` - Property-based testing
- `tempfile` - Temporary files for tests

## Contributing

1. Follow Rust naming conventions and best practices
2. Add comprehensive tests for new data types
3. Update documentation for new fields or structures
4. Ensure backward compatibility when possible
5. Add validation rules for new fields
6. Include examples in documentation

## License

This module is part of the GDELT Fetcher project and is licensed under the MIT License.

## References

- [GDELT Project Documentation](https://www.gdeltproject.org/data.html)
- [CAMEO Event Codes](https://www.gdeltproject.org/data/lookups/CAMEO.eventcodes.txt)
- [GDELT 2.0 Schema](https://blog.gdeltproject.org/gdelt-2-0-our-global-world-in-realtime/)
- [Goldstein Scale](https://www.gdeltproject.org/data/lookups/CAMEO.goldsteinscale.txt)