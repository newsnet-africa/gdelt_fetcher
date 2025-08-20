//! GDELT Global Knowledge Graph (GKG) Table structures and parsing
//!
//! This module implements the GDELT GKG V2.1 format according to the official codebook.
//! The GKG format contains enhanced semantic information extracted from news articles.

use anyhow::{Result, anyhow};
use chrono::{DateTime, TimeZone, Utc};
use csv::StringRecord;

use std::fmt;
use url::Url;

use super::event_table::{
    ADM1Code, ADM2Code, Coordinates, FIPSCountryCode, FeatureID, event_geography::EventGeography,
};
use super::lookup_types::country::CountryZone;
use super::lookup_types::geography_type::GeographyType;
use crate::gcam::lookup::EnrichedGCAMEntry;
use crate::gcam::memory_database::GCAMCodebookDatabase;

/// Character offset within a document
#[derive(Debug, Clone, PartialEq)]
pub struct CharOffset(pub u64);

/// GKG Record ID containing date, sequence, and translation flag
#[derive(Debug, Clone, PartialEq)]
pub struct GKGRecordID {
    pub record_date: DateTime<Utc>,
    pub sequence: u64,
    pub is_translated: bool,
}

/// Source collection identifier enum matching V2SOURCECOLLECTIONIDENTIFIER
#[derive(Debug, Clone, PartialEq)]
pub enum SourceCollectionIdentifier {
    Web = 1,
    CitationOnly = 2,
    Core = 3,
    DTIC = 4,
    JSTOR = 5,
    NonTextualSource = 6,
}

impl TryFrom<u8> for SourceCollectionIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(Self::Web),
            2 => Ok(Self::CitationOnly),
            3 => Ok(Self::Core),
            4 => Ok(Self::DTIC),
            5 => Ok(Self::JSTOR),
            6 => Ok(Self::NonTextualSource),
            _ => Err(anyhow!("Invalid source collection identifier: {}", value)),
        }
    }
}

/// Theme with optional character offset
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub name: String,
    pub offset: Option<CharOffset>,
}

/// Count entry from V1COUNTS or V2.1COUNTS
#[derive(Debug, Clone, PartialEq)]
pub struct Count {
    pub count_type: String,
    pub count: u64,
    pub object_type: Option<String>,
    pub location: Option<EventGeography>,
    pub offset: Option<CharOffset>,
}

/// Enhanced date with resolution and offset
#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedDate {
    pub resolution: u8, // 1=year, 2=month, 3=day, 4=month-day without year
    pub month: u8,
    pub day: u8,
    pub year: u16,
    pub offset: CharOffset,
}

/// V1.5TONE structure
#[derive(Debug, Clone, PartialEq)]
pub struct Tone {
    pub tone: f32,
    pub positive_score: f32,
    pub negative_score: f32,
    pub polarity: f32,
    pub activity_reference_density: f32,
    pub selfgroup_reference_density: f32,
    pub word_count: u64,
}

/// Quotation with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct Quotation {
    pub offset: CharOffset,
    pub length: u64,
    pub verb: Option<String>,
    pub quote: String,
}

/// Named entity with offset
#[derive(Debug, Clone, PartialEq)]
pub struct NamedEntity {
    pub name: String,
    pub offset: CharOffset,
}

/// Amount with object and offset
#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    pub amount: f64,
    pub object: Option<String>,
    pub offset: CharOffset,
}

/// Translation information
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationInfo {
    pub source_language_code: Option<String>,
    pub engine: Option<String>,
}

/// Main GKG Table structure according to V2.1 specification
#[derive(Debug, Clone, PartialEq)]
pub struct GKGTable {
    /// V2GLOBALKNOWLEDGEGRAPHID - Unique identifier for the record
    pub global_knowledge_graph_id: GKGRecordID,

    /// V2DATE - Publication date of the document
    pub date: DateTime<Utc>,

    /// V2SOURCECOLLECTIONIDENTIFIER - Type of source (1-6)
    pub source_collection_identifier: SourceCollectionIdentifier,

    /// V2SOURCECOMMONNAME - Human-friendly source name
    pub source_common_name: String,

    /// V2DOCUMENTIDENTIFIER - External identifier for the document
    pub document_identifier: String,

    /// V1COUNTS - Simple counts without offsets
    pub v1_counts: Vec<Count>,

    /// V2.1COUNTS - Counts with character offsets
    pub v2_counts: Vec<Count>,

    /// V1THEMES - Simple themes without offsets
    pub v1_themes: Vec<String>,

    /// V2ENHANCEDTHEMES - Themes with character offsets
    pub v2_enhanced_themes: Vec<Theme>,

    /// V1LOCATIONS - Simple locations without offsets
    pub v1_locations: Vec<EventGeography>,

    /// V2ENHANCEDLOCATIONS - Locations with character offsets and ADM2
    pub v2_enhanced_locations: Vec<(EventGeography, CharOffset)>,

    /// V1PERSONS - Simple person names without offsets
    pub v1_persons: Vec<String>,

    /// V2ENHANCEDPERSONS - Person names with character offsets
    pub v2_enhanced_persons: Vec<NamedEntity>,

    /// V1ORGANIZATIONS - Simple organization names without offsets
    pub v1_organizations: Vec<String>,

    /// V2ENHANCEDORGANIZATIONS - Organization names with character offsets
    pub v2_enhanced_organizations: Vec<NamedEntity>,

    /// V1.5TONE - Emotional analysis of the document
    pub tone: Tone,

    /// V2.1ENHANCEDDATES - Date references with offsets
    pub enhanced_dates: Vec<EnhancedDate>,

    /// V2GCAM - Global Content Analysis Measures (Enhanced)
    pub gcam: Vec<EnrichedGCAMEntry>,

    /// V2.1SHARINGIMAGE - Sharing image URL
    pub sharing_image: Option<Url>,

    /// V2.1RELATEDIMAGES - Related image URLs
    pub related_images: Vec<Url>,

    /// V2.1SOCIALIMAGEEMBEDS - Social media image embed URLs
    pub social_image_embeds: Vec<Url>,

    /// V2.1SOCIALVIDEOEMBEDS - Social media video embed URLs
    pub social_video_embeds: Vec<Url>,

    /// V2.1QUOTATIONS - Extracted quotations with metadata
    pub quotations: Vec<Quotation>,

    /// V2.1ALLNAMES - All proper names with offsets
    pub all_names: Vec<NamedEntity>,

    /// V2.1AMOUNTS - Numeric amounts with objects and offsets
    pub amounts: Vec<Amount>,

    /// V2.1TRANSLATIONINFO - Translation provenance information
    pub translation_info: Option<TranslationInfo>,
}

impl GKGTable {
    /// Get the tone analysis
    pub fn tone(&self) -> &Tone {
        &self.tone
    }

    /// Get the publication date
    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    /// Get the source collection identifier
    pub fn source_collection_identifier(&self) -> &SourceCollectionIdentifier {
        &self.source_collection_identifier
    }

    /// Get the source common name
    pub fn source_common_name(&self) -> &str {
        &self.source_common_name
    }

    /// Get the document identifier
    pub fn document_identifier(&self) -> &str {
        &self.document_identifier
    }

    /// Get V1 counts (simple counts without offsets)
    pub fn v1_counts(&self) -> &[Count] {
        &self.v1_counts
    }

    /// Get V2 counts (counts with character offsets)
    pub fn v2_counts(&self) -> &[Count] {
        &self.v2_counts
    }

    /// Get V1 themes (simple themes without offsets)
    pub fn v1_themes(&self) -> &[String] {
        &self.v1_themes
    }

    /// Get V2 enhanced themes (themes with character offsets)
    pub fn v2_enhanced_themes(&self) -> &[Theme] {
        &self.v2_enhanced_themes
    }

    /// Get V1 locations (simple locations without offsets)
    pub fn v1_locations(&self) -> &[EventGeography] {
        &self.v1_locations
    }

    /// Get V2 enhanced locations (locations with character offsets)
    pub fn v2_enhanced_locations(&self) -> &[(EventGeography, CharOffset)] {
        &self.v2_enhanced_locations
    }

    /// Get V1 persons (simple person names without offsets)
    pub fn v1_persons(&self) -> &[String] {
        &self.v1_persons
    }

    /// Get V2 enhanced persons (person names with character offsets)
    pub fn v2_enhanced_persons(&self) -> &[NamedEntity] {
        &self.v2_enhanced_persons
    }

    /// Get V1 organizations (simple organization names without offsets)
    pub fn v1_organizations(&self) -> &[String] {
        &self.v1_organizations
    }

    /// Get V2 enhanced organizations (organization names with character offsets)
    pub fn v2_enhanced_organizations(&self) -> &[NamedEntity] {
        &self.v2_enhanced_organizations
    }

    /// Get enhanced dates
    pub fn enhanced_dates(&self) -> &[EnhancedDate] {
        &self.enhanced_dates
    }

    /// Get GCAM data
    pub fn gcam(&self) -> &[EnrichedGCAMEntry] {
        &self.gcam
    }

    /// Get GCAM entries by dictionary type
    pub fn gcam_by_dictionary(
        &self,
        dictionary: &crate::gcam::lookup::Dictionary,
    ) -> Vec<&EnrichedGCAMEntry> {
        self.gcam
            .iter()
            .filter(|entry| {
                entry
                    .metadata
                    .as_ref()
                    .map(|meta| &meta.dictionary == dictionary)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get GCAM entries with available metadata
    pub fn gcam_with_metadata(&self) -> Vec<&EnrichedGCAMEntry> {
        self.gcam
            .iter()
            .filter(|entry| entry.metadata.is_some())
            .collect()
    }

    /// Get GCAM entries without metadata (unknown variables)
    pub fn gcam_without_metadata(&self) -> Vec<&EnrichedGCAMEntry> {
        self.gcam
            .iter()
            .filter(|entry| entry.metadata.is_none())
            .collect()
    }

    /// Get GCAM coverage statistics
    pub fn gcam_coverage_stats(&self) -> crate::gcam::GCAMCoverageStats {
        let total_entries = self.gcam.len();
        let entries_with_metadata = self.gcam_with_metadata().len();
        let entries_without_metadata = self.gcam_without_metadata().len();

        let coverage_percentage = if total_entries > 0 {
            (entries_with_metadata as f64 / total_entries as f64) * 100.0
        } else {
            0.0
        };

        crate::gcam::GCAMCoverageStats {
            total_entries,
            entries_with_metadata,
            entries_without_metadata,
            coverage_percentage,
        }
    }

    /// Get sharing image URL
    pub fn sharing_image(&self) -> Option<&Url> {
        self.sharing_image.as_ref()
    }

    /// Get related images
    pub fn related_images(&self) -> &[Url] {
        &self.related_images
    }

    /// Get social media image embeds
    pub fn social_image_embeds(&self) -> &[Url] {
        &self.social_image_embeds
    }

    /// Get social media video embeds
    pub fn social_video_embeds(&self) -> &[Url] {
        &self.social_video_embeds
    }

    /// Get all quotations
    pub fn quotations(&self) -> &[Quotation] {
        &self.quotations
    }

    /// Get all names
    pub fn all_names(&self) -> &[NamedEntity] {
        &self.all_names
    }

    /// Get amounts
    pub fn amounts(&self) -> &[Amount] {
        &self.amounts
    }

    /// Get translation info
    pub fn translation_info(&self) -> Option<&TranslationInfo> {
        self.translation_info.as_ref()
    }
}

impl fmt::Display for GKGTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GKG Record #{:?}", self.global_knowledge_graph_id)?;
        writeln!(f, "  Publication Date: {}", self.date)?;
        writeln!(
            f,
            "  Source Collection: {:?}",
            self.source_collection_identifier
        )?;
        writeln!(f, "  Source Name: {}", self.source_common_name)?;
        writeln!(f, "  Document ID: {}", self.document_identifier)?;

        // Show counts if available
        if !self.v1_counts.is_empty() {
            writeln!(f, "  V1 Counts ({} items):", self.v1_counts.len())?;
            for count in self.v1_counts.iter().take(3) {
                writeln!(f, "    - {:?}", count)?;
            }
            if self.v1_counts.len() > 3 {
                writeln!(f, "    ... {} more", self.v1_counts.len() - 3)?;
            }
        }

        if !self.v2_counts.is_empty() {
            writeln!(f, "  V2 Counts ({} items):", self.v2_counts.len())?;
            for count in self.v2_counts.iter().take(3) {
                writeln!(f, "    - {:?}", count)?;
            }
            if self.v2_counts.len() > 3 {
                writeln!(f, "    ... {} more", self.v2_counts.len() - 3)?;
            }
        }

        // Show themes if available
        if !self.v1_themes.is_empty() {
            writeln!(f, "  V1 Themes ({} items):", self.v1_themes.len())?;
            for theme in self.v1_themes.iter().take(5) {
                writeln!(f, "    - {}", theme)?;
            }
            if self.v1_themes.len() > 5 {
                writeln!(f, "    ... {} more", self.v1_themes.len() - 5)?;
            }
        }

        if !self.v2_enhanced_themes.is_empty() {
            writeln!(
                f,
                "  V2 Enhanced Themes ({} items):",
                self.v2_enhanced_themes.len()
            )?;
            for theme in self.v2_enhanced_themes.iter().take(5) {
                writeln!(
                    f,
                    "    - {} (offset: {})",
                    theme.name,
                    theme.offset.as_ref().map_or(0, |o| o.0)
                )?;
            }
            if self.v2_enhanced_themes.len() > 5 {
                writeln!(f, "    ... {} more", self.v2_enhanced_themes.len() - 5)?;
            }
        }

        // Show locations if available
        if !self.v1_locations.is_empty() {
            writeln!(f, "  V1 Locations ({} items):", self.v1_locations.len())?;
            for location in self.v1_locations.iter().take(3) {
                writeln!(f, "    - {:?}", location)?;
            }
            if self.v1_locations.len() > 3 {
                writeln!(f, "    ... {} more", self.v1_locations.len() - 3)?;
            }
        }

        if !self.v2_enhanced_locations.is_empty() {
            writeln!(
                f,
                "  V2 Enhanced Locations ({} items):",
                self.v2_enhanced_locations.len()
            )?;
            for (location, offset) in self.v2_enhanced_locations.iter().take(3) {
                writeln!(f, "    - {:?} (offset: {})", location, offset.0)?;
            }
            if self.v2_enhanced_locations.len() > 3 {
                writeln!(f, "    ... {} more", self.v2_enhanced_locations.len() - 3)?;
            }
        }

        // Show tone
        writeln!(
            f,
            "  Tone: {:.2} (pos: {:.2}, neg: {:.2}, word_count: {})",
            self.tone.tone,
            self.tone.positive_score,
            self.tone.negative_score,
            self.tone.word_count
        )?;

        // Show persons if available
        if !self.v1_persons.is_empty() {
            writeln!(
                f,
                "  V1 Persons ({} items): {}",
                self.v1_persons.len(),
                self.v1_persons
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if !self.v2_enhanced_persons.is_empty() {
            writeln!(
                f,
                "  V2 Enhanced Persons ({} items):",
                self.v2_enhanced_persons.len()
            )?;
            for person in self.v2_enhanced_persons.iter().take(3) {
                writeln!(f, "    - {} (offset: {})", person.name, person.offset.0)?;
            }
        }

        // Show organizations if available
        if !self.v1_organizations.is_empty() {
            writeln!(
                f,
                "  V1 Organizations ({} items): {}",
                self.v1_organizations.len(),
                self.v1_organizations
                    .iter()
                    .take(3)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if !self.v2_enhanced_organizations.is_empty() {
            writeln!(
                f,
                "  V2 Enhanced Organizations ({} items):",
                self.v2_enhanced_organizations.len()
            )?;
            for org in self.v2_enhanced_organizations.iter().take(3) {
                writeln!(f, "    - {} (offset: {})", org.name, org.offset.0)?;
            }
        }

        // Show GCAM info if available
        if !self.gcam.is_empty() {
            writeln!(f, "  GCAM Measures ({} entries)", self.gcam.len())?;
        }

        // Show sharing image
        if let Some(ref image) = self.sharing_image {
            writeln!(f, "  Sharing Image: {}", image)?;
        }

        // Show other media
        if !self.related_images.is_empty() {
            writeln!(f, "  Related Images: {} items", self.related_images.len())?;
        }
        if !self.social_image_embeds.is_empty() {
            writeln!(
                f,
                "  Social Image Embeds: {} items",
                self.social_image_embeds.len()
            )?;
        }
        if !self.social_video_embeds.is_empty() {
            writeln!(
                f,
                "  Social Video Embeds: {} items",
                self.social_video_embeds.len()
            )?;
        }

        // Show quotations
        if !self.quotations.is_empty() {
            writeln!(f, "  Quotations: {} items", self.quotations.len())?;
        }

        // Show amounts
        if !self.amounts.is_empty() {
            writeln!(f, "  Amounts: {} items", self.amounts.len())?;
        }

        // Show translation info
        if let Some(ref translation) = self.translation_info {
            writeln!(f, "  Translation: {:?}", translation)?;
        }

        Ok(())
    }
}

impl super::DatabaseTable for GKGTable {}

/// Parse floating point with default fallback
fn parse_f32_or_default(s: &str) -> Result<f32> {
    if s.is_empty() {
        Ok(0.0)
    } else {
        s.parse::<f32>()
            .map_err(|e| anyhow!("Failed to parse f32: {}", e))
    }
}

/// Parse counts from semicolon-delimited format
fn parse_counts(s: &str, with_offsets: bool) -> Result<Vec<Count>> {
    if s.is_empty() {
        return Ok(vec![]);
    }

    let mut counts = Vec::new();
    for count_str in s.split(';') {
        if count_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = count_str.split('#').collect();
        if parts.len() < 3 {
            continue; // Skip malformed count entries
        }

        let count_type = parts[0].to_string();
        let count = parts[1].parse::<u64>().unwrap_or(0);
        let object_type = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };

        // Parse location if present (parts 3-9)
        let location = if parts.len() > 9 {
            EventGeography::try_from_gkg_parts(&parts[3..]).ok()
        } else {
            None
        };

        // Parse offset if this is V2.1 format and offset is present
        let offset = if with_offsets && parts.len() > 10 {
            parts[10].parse::<u64>().ok().map(CharOffset)
        } else {
            None
        };

        counts.push(Count {
            count_type,
            count,
            object_type,
            location,
            offset,
        });
    }

    Ok(counts)
}

/// Parse themes from semicolon-delimited format
fn parse_themes_v1(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }

    s.split(';')
        .filter(|theme| !theme.is_empty())
        .map(|theme| theme.to_string())
        .collect()
}

/// Parse enhanced themes with offsets
fn parse_themes_v2(s: &str) -> Vec<Theme> {
    if s.is_empty() {
        return vec![];
    }

    let mut themes = Vec::new();
    for theme_str in s.split(';') {
        if theme_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = theme_str.split(',').collect();
        if parts.is_empty() {
            continue;
        }

        let name = parts[0].to_string();
        let offset = if parts.len() > 1 {
            parts[1].parse::<u64>().ok().map(CharOffset)
        } else {
            None
        };

        themes.push(Theme { name, offset });
    }

    themes
}

/// Parse persons/organizations with offsets
fn parse_named_entities(s: &str) -> Vec<NamedEntity> {
    if s.is_empty() {
        return vec![];
    }

    let mut entities = Vec::new();
    for entity_str in s.split(';') {
        if entity_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = entity_str.split(',').collect();
        if parts.is_empty() {
            continue;
        }

        let name = parts[0].to_string();
        let offset = if parts.len() > 1 {
            parts[1].parse::<u64>().unwrap_or(0)
        } else {
            0
        };

        entities.push(NamedEntity {
            name,
            offset: CharOffset(offset),
        });
    }

    entities
}

/// Parse simple semicolon-delimited strings
fn parse_simple_list(s: &str) -> Vec<String> {
    if s.is_empty() {
        return vec![];
    }

    s.split(';')
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

/// Parse GCAM data
fn parse_gcam(s: &str) -> Vec<EnrichedGCAMEntry> {
    parse_gcam_with_database(s, None)
}

fn parse_gcam_with_database(
    s: &str,
    gcam_db: Option<&GCAMCodebookDatabase>,
) -> Vec<EnrichedGCAMEntry> {
    if s.is_empty() {
        return vec![];
    }

    let mut entries = Vec::new();
    for entry_str in s.split(',') {
        if entry_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = entry_str.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].to_string();
        if let Ok(value) = parts[1].parse::<f32>() {
            let enriched_entry = if let Some(db) = gcam_db {
                db.enrich_gcam_entry(&key, value)
                    .unwrap_or_else(|_| EnrichedGCAMEntry::from_simple(key, value))
            } else {
                EnrichedGCAMEntry::from_simple(key, value)
            };
            entries.push(enriched_entry);
        }
    }

    entries
}

/// Parse URLs from semicolon-delimited format
fn parse_urls(s: &str) -> Vec<Url> {
    if s.is_empty() {
        return vec![];
    }

    s.split(';')
        .filter(|url_str| !url_str.is_empty())
        .filter_map(|url_str| Url::parse(url_str).ok())
        .collect()
}

/// Parse quotations from pound-delimited format with pipe-separated fields
fn parse_quotations(s: &str) -> Vec<Quotation> {
    if s.is_empty() {
        return vec![];
    }

    let mut quotations = Vec::new();
    for quote_str in s.split('#') {
        if quote_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = quote_str.split('|').collect();
        if parts.len() < 4 {
            continue; // Need at least offset, length, verb, quote
        }

        let offset = parts[0].parse::<u64>().unwrap_or(0);
        let length = parts[1].parse::<u64>().unwrap_or(0);
        let verb = if parts[2].is_empty() {
            None
        } else {
            Some(parts[2].to_string())
        };
        let quote = parts[3].to_string();

        quotations.push(Quotation {
            offset: CharOffset(offset),
            length,
            verb,
            quote,
        });
    }

    quotations
}

/// Parse amounts from semicolon-delimited format
fn parse_amounts(s: &str) -> Vec<Amount> {
    if s.is_empty() {
        return vec![];
    }

    let mut amounts = Vec::new();
    for amount_str in s.split(';') {
        if amount_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = amount_str.split(',').collect();
        if parts.len() < 3 {
            continue; // Need at least amount, object, offset
        }

        let amount = parts[0].parse::<f64>().unwrap_or(0.0);
        let object = if parts[1].is_empty() {
            None
        } else {
            Some(parts[1].to_string())
        };
        let offset = parts[2].parse::<u64>().unwrap_or(0);

        amounts.push(Amount {
            amount,
            object,
            offset: CharOffset(offset),
        });
    }

    amounts
}

/// Parse V1 locations from semicolon-delimited format
fn parse_locations_v1(s: &str) -> Vec<EventGeography> {
    if s.is_empty() {
        return vec![];
    }

    let mut locations = Vec::new();
    for location_str in s.split(';') {
        if location_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = location_str.split('#').collect();
        if parts.len() < 7 {
            continue; // Need at least 7 parts for V1 format
        }

        if let Ok(geography) = EventGeography::try_from_gkg_parts(&parts) {
            locations.push(geography);
        }
    }

    locations
}

/// Parse V2 enhanced locations with character offsets
fn parse_locations_v2(s: &str) -> Vec<(EventGeography, CharOffset)> {
    if s.is_empty() {
        return vec![];
    }

    let mut locations = Vec::new();
    for location_str in s.split(';') {
        if location_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = location_str.split('#').collect();
        if parts.len() < 8 {
            continue; // Need at least 8 parts for V2 format (includes offset)
        }

        // Last part is the character offset
        let offset = parts[parts.len() - 1].parse::<u64>().unwrap_or(0);

        // Parse geography from all parts except the last (offset)
        let geo_parts = &parts[..parts.len() - 1];
        if let Ok(geography) = EventGeography::try_from_gkg_parts(geo_parts) {
            locations.push((geography, CharOffset(offset)));
        }
    }

    locations
}

/// Parse enhanced dates with resolution and offsets
fn parse_enhanced_dates(s: &str) -> Vec<EnhancedDate> {
    if s.is_empty() {
        return vec![];
    }

    let mut dates = Vec::new();
    for date_str in s.split(';') {
        if date_str.is_empty() {
            continue;
        }

        let parts: Vec<&str> = date_str.split(',').collect();
        if parts.len() < 5 {
            continue; // Need at least resolution, month, day, year, offset
        }

        let resolution = parts[0].parse::<u8>().unwrap_or(0);
        let month = parts[1].parse::<u8>().unwrap_or(0);
        let day = parts[2].parse::<u8>().unwrap_or(0);
        let year = parts[3].parse::<u16>().unwrap_or(0);
        let offset = parts[4].parse::<u64>().unwrap_or(0);

        dates.push(EnhancedDate {
            resolution,
            month,
            day,
            year,
            offset: CharOffset(offset),
        });
    }

    dates
}

/// Parse translation info from semicolon-delimited format
fn parse_translation_info(s: &str) -> Option<TranslationInfo> {
    if s.is_empty() {
        return None;
    }

    let mut source_language_code = None;
    let mut engine = None;

    for part in s.split(';') {
        if let Some(lang_part) = part.strip_prefix("srclc:") {
            source_language_code = Some(lang_part.to_string());
        } else if let Some(eng_part) = part.strip_prefix("eng:") {
            engine = Some(eng_part.to_string());
        }
    }

    if source_language_code.is_some() || engine.is_some() {
        Some(TranslationInfo {
            source_language_code,
            engine,
        })
    } else {
        None
    }
}

impl GKGTable {
    /// Create GKGTable from StringRecord with GCAM database for enrichment
    pub fn try_from_with_gcam_db(
        record: StringRecord,
        gcam_db: &GCAMCodebookDatabase,
    ) -> Result<Self, anyhow::Error> {
        Self::try_from_with_optional_gcam_db(record, Some(gcam_db))
    }

    /// Create GKGTable from StringRecord with optional GCAM database for enrichment
    pub fn try_from_with_optional_gcam_db(
        record: StringRecord,
        gcam_db: Option<&GCAMCodebookDatabase>,
    ) -> Result<Self, anyhow::Error> {
        // GKG V2.1 format has variable number of fields, but minimum 16 for core data
        if record.len() < 16 {
            return Err(anyhow!(
                "Expected at least 16 fields for GKGTable V2.1 core data, got {}",
                record.len()
            ));
        }
        let fields: Vec<&str> = record.iter().collect();

        // Field 0: V2GLOBALKNOWLEDGEGRAPHID (compound: YYYYMMDDHHMMSS-sequence)
        let compound_field_parts: Vec<&str> = fields[0].split('-').collect();
        if compound_field_parts.len() != 2 {
            return Err(anyhow!(
                "Invalid compound field format in field 0: {}",
                fields[0]
            ));
        }
        let record_date_str = compound_field_parts[0];
        let sequence_str = compound_field_parts[1];

        // Parse V2SOURCECOLLECTIONIDENTIFIER from field 2
        let source_collection_identifier = SourceCollectionIdentifier::try_from(
            fields[2]
                .parse::<u8>()
                .map_err(|e| anyhow!("Invalid source collection identifier: {}", e))?,
        )?;

        // Parse V1.5TONE from field 15 (comma-separated values)
        let tone = if !fields[15].is_empty() {
            let tone_parts: Vec<&str> = fields[15].split(',').collect();
            Tone {
                tone: tone_parts
                    .get(0)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                positive_score: tone_parts
                    .get(1)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                negative_score: tone_parts
                    .get(2)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                polarity: tone_parts
                    .get(3)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                activity_reference_density: tone_parts
                    .get(4)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                selfgroup_reference_density: tone_parts
                    .get(5)
                    .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                word_count: tone_parts
                    .get(6)
                    .map_or(0, |s| s.parse::<u64>().unwrap_or(0)),
            }
        } else {
            Tone {
                tone: 0.0,
                positive_score: 0.0,
                negative_score: 0.0,
                polarity: 0.0,
                activity_reference_density: 0.0,
                selfgroup_reference_density: 0.0,
                word_count: 0,
            }
        };

        Ok(GKGTable {
            global_knowledge_graph_id: GKGRecordID {
                record_date: chrono::NaiveDateTime::parse_from_str(record_date_str, "%Y%m%d%H%M%S")
                    .map_err(|e| anyhow!("Invalid record_date: {}", e))
                    .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,
                sequence: sequence_str
                    .parse::<u64>()
                    .map_err(|e| anyhow!("Invalid sequence: {}", e))?,
                is_translated: fields[0].contains("-T"),
            },
            // Field 1: V2DATE - Publication date
            date: chrono::NaiveDateTime::parse_from_str(fields[1], "%Y%m%d%H%M%S")
                .map_err(|e| anyhow!("Invalid date: {}", e))
                .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,

            // Field 2: V2SOURCECOLLECTIONIDENTIFIER
            source_collection_identifier,

            // Field 3: V2SOURCECOMMONNAME
            source_common_name: fields.get(3).map_or("", |s| s).to_string(),

            // Field 4: V2DOCUMENTIDENTIFIER
            document_identifier: fields.get(4).map_or("", |s| s).to_string(),

            // Field 5: V1COUNTS (semicolon-delimited, pound-separated fields)
            v1_counts: parse_counts(fields.get(5).map_or("", |s| s), false)?,

            // Field 6: V2.1COUNTS (with character offsets)
            v2_counts: parse_counts(fields.get(6).map_or("", |s| s), true)?,

            // Field 7: V1THEMES (semicolon-delimited)
            v1_themes: parse_simple_list(fields.get(7).map_or("", |s| s)),

            // Field 8: V2ENHANCEDTHEMES (with character offsets)
            v2_enhanced_themes: parse_themes_v2(fields.get(8).map_or("", |s| s)),

            // Field 9: V1LOCATIONS (semicolon-delimited, pound-separated fields)
            v1_locations: parse_locations_v1(fields.get(9).map_or("", |s| s)),

            // Field 10: V2ENHANCEDLOCATIONS (with character offsets and ADM2)
            v2_enhanced_locations: parse_locations_v2(fields.get(10).map_or("", |s| s)),

            // Field 11: V1PERSONS (semicolon-delimited)
            v1_persons: parse_simple_list(fields.get(11).map_or("", |s| s)),

            // Field 12: V2ENHANCEDPERSONS (with character offsets)
            v2_enhanced_persons: parse_named_entities(fields.get(12).map_or("", |s| s)),

            // Field 13: V1ORGANIZATIONS (semicolon-delimited)
            v1_organizations: parse_simple_list(fields.get(13).map_or("", |s| s)),

            // Field 14: V2ENHANCEDORGANIZATIONS (with character offsets)
            v2_enhanced_organizations: parse_named_entities(fields.get(14).map_or("", |s| s)),

            // Field 15: V1.5TONE (already parsed above)
            tone,

            // Field 16: V2.1ENHANCEDDATES (semicolon-delimited blocks)
            enhanced_dates: parse_enhanced_dates(fields.get(16).map_or("", |s| s)),

            // Field 17: V2GCAM (comma-delimited blocks with colon key/value pairs)
            gcam: parse_gcam_with_database(fields.get(17).map_or("", |s| s), gcam_db),

            // Field 18: V2.1SHARINGIMAGE (textual URL)
            sharing_image: fields
                .get(18)
                .and_then(|s| if s.is_empty() { None } else { Some(s) })
                .and_then(|s| Url::parse(s).ok()),

            // Field 19: V2.1RELATEDIMAGES (semicolon-delimited URLs)
            related_images: parse_urls(fields.get(19).map_or("", |s| s)),

            // Field 20: V2.1SOCIALIMAGEEMBEDS (semicolon-delimited URLs)
            social_image_embeds: parse_urls(fields.get(20).map_or("", |s| s)),

            // Field 21: V2.1SOCIALVIDEOEMBEDS (semicolon-delimited URLs)
            social_video_embeds: parse_urls(fields.get(21).map_or("", |s| s)),

            // Field 22: V2.1QUOTATIONS (pound-delimited blocks with pipe-separated fields)
            quotations: parse_quotations(fields.get(22).map_or("", |s| s)),

            // Field 23: V2.1ALLNAMES (semicolon-delimited blocks with comma-separated fields)
            all_names: parse_named_entities(fields.get(23).map_or("", |s| s)),

            // Field 24: V2.1AMOUNTS (semicolon-delimited blocks with comma-separated fields)
            amounts: parse_amounts(fields.get(24).map_or("", |s| s)),

            // Field 26: V2.1TRANSLATIONINFO (semicolon-delimited fields)
            translation_info: parse_translation_info(fields.get(26).map_or("", |s| s)),
        })
    }
}

impl TryFrom<StringRecord> for GKGTable {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        // Use the in-memory GCAM database for enrichment by default
        match GCAMCodebookDatabase::new_temp() {
            Ok(db) => Self::try_from_with_optional_gcam_db(record, Some(&db)),
            Err(_) => Self::try_from_with_optional_gcam_db(record, None),
        }
    }
}

impl EventGeography {
    /// Parse EventGeography from GKG location parts
    fn try_from_gkg_parts(parts: &[&str]) -> Result<Self> {
        if parts.len() < 7 {
            return Err(anyhow!("Insufficient location parts"));
        }

        // GKG location format: Type#FullName#CountryCode#ADM1Code#Latitude#Longitude#FeatureID
        // Enhanced format adds ADM2Code between ADM1Code and Latitude

        let location_type = parts[0].parse::<u8>().unwrap_or(0);
        let full_name = parts[1].to_string();
        let country_code = parts[2].to_string();
        let adm1_code = parts[3].to_string();

        // Check if this is enhanced format with ADM2
        let (adm2_code, lat_idx, lon_idx, feature_idx) = if parts.len() > 8 {
            // Enhanced format with ADM2
            (Some(parts[4].to_string()), 5, 6, 7)
        } else {
            // Standard format without ADM2
            (None, 4, 5, 6)
        };

        let latitude = parts[lat_idx].parse::<f64>().unwrap_or(0.0);
        let longitude = parts[lon_idx].parse::<f64>().unwrap_or(0.0);
        let feature_id = parts.get(feature_idx).unwrap_or(&"").to_string();

        // Parse geo_type from location_type
        let geo_type = match location_type {
            1 => Some(GeographyType::Country),
            2 => Some(GeographyType::State),
            3 => Some(GeographyType::City),
            4 => Some(GeographyType::City),
            5 => Some(GeographyType::State),
            _ => None,
        };

        Ok(EventGeography {
            geo_type,
            fullname: if full_name.is_empty() {
                None
            } else {
                Some(full_name)
            },
            country_code: if country_code.is_empty() {
                None
            } else {
                match CountryZone::try_from(Some(FIPSCountryCode(country_code))) {
                    Ok(zone) => Some(zone),
                    Err(_) => None,
                }
            },
            adm1_code: if adm1_code.is_empty() {
                None
            } else {
                Some(ADM1Code(adm1_code))
            },
            adm2_code: adm2_code.map(ADM2Code),
            coordinates: Some(Coordinates {
                latitude,
                longitude,
            }),
            feature_id: if feature_id.is_empty() {
                None
            } else {
                Some(FeatureID(feature_id))
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = env_logger::try_init();
        });
    }

    #[test]
    fn test_source_collection_identifier_try_from() {
        assert!(matches!(
            SourceCollectionIdentifier::try_from(1).unwrap(),
            SourceCollectionIdentifier::Web
        ));
        assert!(matches!(
            SourceCollectionIdentifier::try_from(2).unwrap(),
            SourceCollectionIdentifier::CitationOnly
        ));
        assert!(matches!(
            SourceCollectionIdentifier::try_from(3).unwrap(),
            SourceCollectionIdentifier::Core
        ));
        assert!(matches!(
            SourceCollectionIdentifier::try_from(4).unwrap(),
            SourceCollectionIdentifier::DTIC
        ));
        assert!(matches!(
            SourceCollectionIdentifier::try_from(5).unwrap(),
            SourceCollectionIdentifier::JSTOR
        ));
        assert!(matches!(
            SourceCollectionIdentifier::try_from(6).unwrap(),
            SourceCollectionIdentifier::NonTextualSource
        ));
        assert!(SourceCollectionIdentifier::try_from(7).is_err());
    }

    #[test]
    fn test_parse_counts_v1() {
        let counts_str = "KILL#47#jihadists#1#Country#US#US#40#-100#12345";
        let counts = parse_counts(counts_str, false).unwrap();
        assert_eq!(counts.len(), 1);
        assert_eq!(counts[0].count_type, "KILL");
        assert_eq!(counts[0].count, 47);
        assert_eq!(counts[0].object_type, Some("jihadists".to_string()));
        assert!(counts[0].offset.is_none());
    }

    #[test]
    fn test_parse_counts_v2() {
        let counts_str = "PROTEST#126#protesters#2#State#US#CA#37#-122#67890#150";
        let counts = parse_counts(counts_str, true).unwrap();
        assert_eq!(counts.len(), 1);
        assert_eq!(counts[0].count_type, "PROTEST");
        assert_eq!(counts[0].count, 126);
        assert_eq!(counts[0].object_type, Some("protesters".to_string()));
        assert_eq!(counts[0].offset.as_ref().unwrap().0, 150);
    }

    #[test]
    fn test_parse_themes_v1() {
        let themes_str = "THEME1;THEME2;THEME3";
        let themes = parse_themes_v1(themes_str);
        assert_eq!(themes.len(), 3);
        assert_eq!(themes[0], "THEME1");
        assert_eq!(themes[1], "THEME2");
        assert_eq!(themes[2], "THEME3");
    }

    #[test]
    fn test_parse_themes_v2() {
        let themes_str = "THEME1,150;THEME2,300;THEME3,450";
        let themes = parse_themes_v2(themes_str);
        assert_eq!(themes.len(), 3);
        assert_eq!(themes[0].name, "THEME1");
        assert_eq!(themes[0].offset.as_ref().unwrap().0, 150);
        assert_eq!(themes[1].name, "THEME2");
        assert_eq!(themes[1].offset.as_ref().unwrap().0, 300);
    }

    #[test]
    fn test_parse_named_entities() {
        let entities_str = "John Smith,100;Jane Doe,200;Bob Johnson,300";
        let entities = parse_named_entities(entities_str);
        assert_eq!(entities.len(), 3);
        assert_eq!(entities[0].name, "John Smith");
        assert_eq!(entities[0].offset.0, 100);
        assert_eq!(entities[1].name, "Jane Doe");
        assert_eq!(entities[1].offset.0, 200);
    }

    #[test]
    fn test_parse_gcam() {
        let gcam_str = "wc:125,c2.21:4,c10.1:40,v10.1:3.21111111";
        let gcam = parse_gcam(gcam_str);
        assert_eq!(gcam.len(), 4);
        assert_eq!(gcam[0].key, "wc");
        assert_eq!(gcam[0].value, 125.0);
        assert_eq!(gcam[1].key, "c2.21");
        assert_eq!(gcam[1].value, 4.0);
        assert_eq!(gcam[2].key, "c10.1");
        assert_eq!(gcam[2].value, 40.0);
        assert_eq!(gcam[3].key, "v10.1");
        assert_eq!(gcam[3].value, 3.21111111);
    }

    #[test]
    #[ignore]
    fn test_gcam_enrichment_in_gkg_table() {
        // Create a mock GKG table record with GCAM data that should be enriched
        let fields = vec![
            "20230820120000-123",                                // Field 0: compound ID
            "20230820120000",                                    // Field 1: date
            "1",                       // Field 2: source collection identifier
            "Test Source",             // Field 3: source common name
            "http://example.com/test", // Field 4: document identifier
            "",                        // Field 5: V1 counts
            "",                        // Field 6: V2 counts
            "",                        // Field 7: V1 themes
            "",                        // Field 8: V2 enhanced themes
            "",                        // Field 9: V1 locations
            "",                        // Field 10: V2 enhanced locations
            "",                        // Field 11: V1 persons
            "",                        // Field 12: V2 enhanced persons
            "",                        // Field 13: V1 organizations
            "",                        // Field 14: V2 enhanced organizations
            "0,0,0,0,0,0,100",         // Field 15: tone
            "",                        // Field 16: enhanced dates
            "c1.1:0.75,v19.1:4.785,v20.1:0.471,unknown_var:1.0", // Field 17: GCAM
            "",                        // Field 18: sharing image
            "",                        // Field 19: related images
            "",                        // Field 20: social image embeds
            "",                        // Field 21: social video embeds
            "",                        // Field 22: quotations
            "",                        // Field 23: all names
            "",                        // Field 24: amounts
            "",                        // Field 25: reserved
            "",                        // Field 26: translation info
        ];

        let record = csv::StringRecord::from(fields);

        // Parse the GKG table - this should now use the in-memory database by default
        let gkg_table = GKGTable::try_from(record).expect("Failed to parse GKG table");

        // Verify GCAM entries were enriched
        assert_eq!(gkg_table.gcam.len(), 4);

        // Check that known variables were enriched
        let c1_1_entry = gkg_table.gcam.iter().find(|e| e.key == "c1.1").unwrap();
        assert!(c1_1_entry.metadata.is_some(), "c1.1 should have metadata");

        let v19_1_entry = gkg_table.gcam.iter().find(|e| e.key == "v19.1").unwrap();
        assert!(v19_1_entry.metadata.is_some(), "v19.1 should have metadata");

        let v20_1_entry = gkg_table.gcam.iter().find(|e| e.key == "v20.1").unwrap();
        assert!(v20_1_entry.metadata.is_some(), "v20.1 should have metadata");

        // Check that unknown variables have no metadata
        let unknown_entry = gkg_table
            .gcam
            .iter()
            .find(|e| e.key == "unknown_var")
            .unwrap();
        assert!(
            unknown_entry.metadata.is_none(),
            "unknown_var should not have metadata"
        );

        // Verify specific metadata content
        if let Some(metadata) = &c1_1_entry.metadata {
            assert_eq!(metadata.variable, "c1.1");
            assert_eq!(metadata.dimension_name, "AESTHETIC");
        }
    }

    #[test]
    fn test_parse_urls() {
        let urls_str = "https://example.com/image1.jpg;https://example.com/image2.png";
        let urls = parse_urls(urls_str);
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0].as_str(), "https://example.com/image1.jpg");
        assert_eq!(urls[1].as_str(), "https://example.com/image2.png");
    }

    #[test]
    fn test_parse_quotations() {
        let quotes_str = "100|50|said|This is a quote#200|30|replied|Another quote";
        let quotations = parse_quotations(quotes_str);
        assert_eq!(quotations.len(), 2);
        assert_eq!(quotations[0].offset.0, 100);
        assert_eq!(quotations[0].length, 50);
        assert_eq!(quotations[0].verb, Some("said".to_string()));
        assert_eq!(quotations[0].quote, "This is a quote");
    }

    #[test]
    fn test_parse_amounts() {
        let amounts_str = "47.5,dollars,100;1000,people,200";
        let amounts = parse_amounts(amounts_str);
        assert_eq!(amounts.len(), 2);
        assert_eq!(amounts[0].amount, 47.5);
        assert_eq!(amounts[0].object, Some("dollars".to_string()));
        assert_eq!(amounts[0].offset.0, 100);
    }

    #[test]
    fn test_parse_translation_info() {
        let trans_str = "srclc:fra;eng:Moses 2.1.1 / MosesCore Europarl fr-en / GT-FRA 1.0";
        let trans_info = parse_translation_info(trans_str).unwrap();
        assert_eq!(trans_info.source_language_code, Some("fra".to_string()));
        assert_eq!(
            trans_info.engine,
            Some("Moses 2.1.1 / MosesCore Europarl fr-en / GT-FRA 1.0".to_string())
        );
    }

    #[test]
    fn test_parse_locations_v1() {
        let locations_str = "1#Australia#AS#AS#-25#135#AS;4#Brisbane, Queensland, Australia#AS#AS04#-27.5#153.017#-1561728";
        let locations = parse_locations_v1(locations_str);
        assert_eq!(locations.len(), 2);

        // Check first location (Australia - country level)
        assert!(matches!(
            locations[0].geo_type,
            Some(GeographyType::Country)
        ));
        assert_eq!(locations[0].fullname, Some("Australia".to_string()));
        // Note: country_code is now CountryZone enum, not a string wrapper
        assert!(locations[0].country_code.is_some());

        // Check second location (Brisbane - world city)
        assert!(matches!(locations[1].geo_type, Some(GeographyType::City)));
        assert_eq!(
            locations[1].fullname,
            Some("Brisbane, Queensland, Australia".to_string())
        );
    }

    #[test]
    fn test_parse_enhanced_dates() {
        let dates_str = "3,3,15,2024,150;1,0,0,2023,300";
        let dates = parse_enhanced_dates(dates_str);
        assert_eq!(dates.len(), 2);

        // Check first date (day-level resolution)
        assert_eq!(dates[0].resolution, 3);
        assert_eq!(dates[0].month, 3);
        assert_eq!(dates[0].day, 15);
        assert_eq!(dates[0].year, 2024);
        assert_eq!(dates[0].offset.0, 150);

        // Check second date (year-level resolution)
        assert_eq!(dates[1].resolution, 1);
        assert_eq!(dates[1].month, 0);
        assert_eq!(dates[1].day, 0);
        assert_eq!(dates[1].year, 2023);
        assert_eq!(dates[1].offset.0, 300);
    }

    #[test]
    fn test_parse_locations_v2() {
        let locations_str = "1#Australian#AS#AS##-25#135#AS#57;4#Brisbane, Queensland, Australia#AS#AS04#154654#-27.5#153.017#-1561728#98";
        let locations = parse_locations_v2(locations_str);
        assert_eq!(locations.len(), 2);

        // Check first location with offset
        assert!(matches!(
            locations[0].0.geo_type,
            Some(GeographyType::Country)
        ));
        assert_eq!(locations[0].1.0, 57);

        // Check second location with offset
        assert!(matches!(locations[1].0.geo_type, Some(GeographyType::City)));
        assert_eq!(locations[1].1.0, 98);
    }

    #[test]
    fn test_gkg_table_parsing_with_minimal_fields() {
        init_logger();

        let fields = vec![
            "20250807220000-0",            // Field 0: compound ID
            "20250807220000",              // Field 1: date
            "1",                           // Field 2: source collection identifier
            "example.com",                 // Field 3: source common name
            "https://example.com/article", // Field 4: document identifier
            "",                            // Field 5: V1 counts
            "",                            // Field 6: V2 counts
            "THEME1;THEME2",               // Field 7: V1 themes
            "THEME1,100;THEME2,200",       // Field 8: V2 enhanced themes
            "",                            // Field 9: V1 locations
            "",                            // Field 10: V2 enhanced locations
            "john smith;jane doe",         // Field 11: V1 persons
            "John Smith,150;Jane Doe,250", // Field 12: V2 enhanced persons
            "company a;company b",         // Field 13: V1 organizations
            "Company A,300;Company B,400", // Field 14: V2 enhanced organizations
            "1.5,2.5,3.5,4.5,5.5,6.5,100", // Field 15: tone
            "1#0#0#2004#169",              // Field 16: enhanced dates
            "wc:100,c1.1:5",               // Field 17: GCAM
            "",                            // Field 18: sharing image
            "",                            // Field 19: related images
            "",                            // Field 20: social image embeds
            "",                            // Field 21: social video embeds
            "",                            // Field 22: quotations
            "",                            // Field 23: all names
            "",                            // Field 24: amounts
            "",                            // Field 25: reserved
            "",                            // Field 26: translation info
        ];

        let record = StringRecord::from(fields);
        let result = GKGTable::try_from(record);

        assert!(
            result.is_ok(),
            "Minimal valid record should parse successfully: {:?}",
            result.err()
        );

        let gkg = result.unwrap();
        assert_eq!(gkg.global_knowledge_graph_id.sequence, 0);
        assert!(!gkg.global_knowledge_graph_id.is_translated);
        assert_eq!(gkg.source_common_name, "example.com");
        assert_eq!(gkg.document_identifier, "https://example.com/article");
        assert_eq!(gkg.v1_themes.len(), 2);
        assert_eq!(gkg.v2_enhanced_themes.len(), 2);
        assert_eq!(gkg.tone.tone, 1.5);
        assert_eq!(gkg.tone.word_count, 100);
    }

    #[test]
    fn test_parse_actual_gkg_data() {
        init_logger();

        // Test parsing against actual local GKG data file
        let file_path = std::path::PathBuf::from("../gkg.csv");
        if !file_path.exists() {
            log::warn!("Local GKG data file not found, skipping test");
            return;
        }

        let file = std::fs::File::open(&file_path).expect("Failed to open GKG file");
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        let mut record_count = 0;
        let mut successful_parses = 0;
        let mut record = csv::StringRecord::new();

        // Parse first 10 records to validate our implementation
        while record_count < 10 && reader.read_record(&mut record).unwrap_or(false) {
            record_count += 1;

            match GKGTable::try_from(record.clone()) {
                Ok(gkg) => {
                    successful_parses += 1;
                    log::info!(
                        "Successfully parsed GKG record {}: ID={}",
                        record_count,
                        gkg.global_knowledge_graph_id.record_date
                    );

                    // Validate some key fields are populated
                    assert!(
                        !gkg.source_common_name.is_empty(),
                        "Source common name should not be empty"
                    );
                    assert!(
                        !gkg.document_identifier.is_empty(),
                        "Document identifier should not be empty"
                    );

                    // Log field values for verification
                    log::debug!(
                        "Record {}: source={}, themes={}, tone={:.2}",
                        record_count,
                        gkg.source_common_name,
                        gkg.v1_themes.len(),
                        gkg.tone.tone
                    );
                }
                Err(e) => {
                    log::error!("Failed to parse GKG record {}: {}", record_count, e);
                    log::debug!("Record had {} fields", record.len());
                }
            }
        }

        assert_eq!(
            record_count, successful_parses,
            "All {} records should parse successfully, but only {} did",
            record_count, successful_parses
        );

        log::info!(
            "Successfully parsed {}/{} actual GKG records",
            successful_parses,
            record_count
        );
    }
}
