use chrono::{DateTime, TimeZone, Utc};
use url::Url;

use crate::types::{
    event_table::{actor::ActorName, event_action::Tone, event_geography::EventGeography},
    lookup_types::{actor_type::ActorType, mention_type::MentionType, social_embeds::SocialEmbed},
    mention_table::{
        CharOffset, Engine, MentionIdentifier, MentionSourceName, MentionTypeCode,
        SourceLanguageCode,
    },
};

#[derive(Debug)]
pub struct GKGRecordID {
    pub record_date: DateTime<Utc>,
    pub sequence: u128,
    pub is_translated: bool,
}

#[derive(Debug)]
pub struct CategoryTheme(String);

#[derive(Debug)]
pub struct Count {
    pub count_type: CategoryTheme,
    pub count: u128,
    pub object_type: ActorName,
    pub location: EventGeography,
}

#[derive(Debug)]
pub struct PositiveScore(f32);
#[derive(Debug)]
pub struct NegativeScore(f32);
#[derive(Debug)]
pub struct Polarity(f32);
#[derive(Debug)]
pub struct ActivityReferenceDensity(f32);
#[derive(Debug)]
pub struct SelfGroupReferenceDensity(f32);
#[derive(Debug)]
pub struct WordCount(u128);

#[derive(Debug)]
pub struct AdvancedTone {
    pub tone: Tone,
    pub positive_score: PositiveScore,
    pub negative_score: NegativeScore,
    pub polarity: Polarity,
    pub activity_reference_density: ActivityReferenceDensity,
    pub selfgroup_reference_density: SelfGroupReferenceDensity,
    pub word_count: WordCount,
}

#[derive(Debug)]
pub struct ImageURL(Url);
#[derive(Debug)]
pub struct SocialMediaEmbedUrl(Url);

#[derive(Debug)]
pub struct Quotation {
    pub offset: CharOffset,
    pub length: u128,
    pub verb: String,
}

#[derive(Debug)]
pub struct Amount {
    amount: u128,
    object: String,
    offset: CharOffset,
}

#[derive(Debug)]
pub struct GKGTable {
    pub global_knowledge_graph_id: GKGRecordID,
    date: DateTime<Utc>,
    source_identifier_type: MentionType,
    source_common_name: MentionSourceName,
    counts: Vec<(Count, CharOffset)>,
    themes: Vec<(CategoryTheme, CharOffset)>,
    locations: Vec<(EventGeography, CharOffset)>,
    persons: Vec<(ActorType, CharOffset)>,
    organisation: Vec<(ActorType, CharOffset)>,
    tone: AdvancedTone,
    enhanced_dates: Vec<(DateTime<Utc>, CharOffset)>,
    sharing_image: ImageURL,
    related_images: Vec<ImageURL>,
    social_media_images: Vec<SocialEmbed>,
    social_media_videos: Vec<SocialEmbed>,
    all_names: Vec<ActorType>,
    mention_doc_translation_info: (Option<SourceLanguageCode>, Option<Engine>),
}

use anyhow::anyhow;
use csv::StringRecord;
use std::convert::TryFrom;

impl GKGTable {
    pub fn tone(&self) -> &AdvancedTone {
        &self.tone
    }
}

fn parse_f32_or_default(s: &str) -> Result<f32, anyhow::Error> {
    if s.trim().is_empty() {
        Ok(0.0)
    } else {
        s.parse::<f32>().map_err(|e| anyhow!(e))
    }
}

impl TryFrom<StringRecord> for GKGTable {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        // Adjust the number of fields as per the actual GKG file format
        if record.len() != 27 {
            return Err(anyhow!(
                "Expected 27 fields for GKGTable, got {}",
                record.len()
            ));
        }
        let fields: Vec<&str> = record.iter().collect();

        // Parse compound field 0: "YYYYMMDDHHMMSS-sequence"
        let compound_field_parts: Vec<&str> = fields[0].split('-').collect();
        if compound_field_parts.len() != 2 {
            return Err(anyhow!(
                "Invalid compound field format in field 0: {}",
                fields[0]
            ));
        }
        let record_date_str = compound_field_parts[0];
        let sequence_str = compound_field_parts[1];

        Ok(GKGTable {
            global_knowledge_graph_id: GKGRecordID {
                record_date: chrono::NaiveDateTime::parse_from_str(record_date_str, "%Y%m%d%H%M%S")
                    .map_err(|e| anyhow!("Invalid record_date: {}", e))
                    .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,
                sequence: sequence_str.parse::<u128>().map_err(|e| anyhow!(e))?,
                is_translated: match fields[2] {
                    "1" => true,
                    "0" => false,
                    _ => return Err(anyhow!("Invalid is_translated value")),
                },
            },
            date: chrono::NaiveDateTime::parse_from_str(fields[1], "%Y%m%d%H%M%S")
                .map_err(|e| anyhow!("Invalid date: {}", e))
                .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,
            source_identifier_type: MentionType::try_from(Some((
                MentionTypeCode::try_from(Some("1"))?, // Fixed value as per actual data
                MentionIdentifier::try_from(Some(fields[4]))?, // Document URL
            )))?,
            source_common_name: MentionSourceName::try_from(Some(fields[3]))?,
            counts: vec![],       // TODO: parse counts from fields[7]
            themes: vec![],       // TODO: parse themes from fields[8]
            locations: vec![],    // TODO: parse locations from fields[9]
            persons: vec![],      // TODO: parse persons from fields[10]
            organisation: vec![], // TODO: parse organisation from fields[11]
            tone: {
                // Parse comma-separated tone values from field 15
                let tone_parts: Vec<&str> = fields[15].split(',').collect();
                AdvancedTone {
                    tone: Tone::try_from(tone_parts.get(0).copied())?,
                    positive_score: PositiveScore(
                        tone_parts
                            .get(1)
                            .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                    ),
                    negative_score: NegativeScore(
                        tone_parts
                            .get(2)
                            .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                    ),
                    polarity: Polarity(
                        tone_parts
                            .get(3)
                            .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                    ),
                    activity_reference_density: ActivityReferenceDensity(
                        tone_parts
                            .get(4)
                            .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                    ),
                    selfgroup_reference_density: SelfGroupReferenceDensity(
                        tone_parts
                            .get(5)
                            .map_or(Ok(0.0), |s| parse_f32_or_default(s))?,
                    ),
                    word_count: WordCount(
                        tone_parts
                            .get(6)
                            .map_or(0, |s| s.parse::<u128>().unwrap_or(0)),
                    ),
                }
            },
            enhanced_dates: vec![], // TODO: parse enhanced_dates from fields[19]
            sharing_image: ImageURL(url::Url::parse("https://example.com").unwrap()), // TODO: parse from fields[20]
            related_images: vec![], // TODO: parse related_images from fields[21]
            social_media_images: vec![], // TODO: parse social_media_images from fields[22]
            social_media_videos: vec![], // TODO: parse social_media_videos from fields[23]
            all_names: vec![],      // TODO: parse all_names from fields[24]
            mention_doc_translation_info: (None, None), // TODO: parse translation info from fields[25-26]
        })
    }
}

impl TryFrom<(&str, &str, &str)> for GKGRecordID {
    type Error = anyhow::Error;

    fn try_from(values: (&str, &str, &str)) -> Result<Self, Self::Error> {
        let record_date = chrono::NaiveDateTime::parse_from_str(values.0, "%Y%m%d%H%M%S")
            .map_err(|e| anyhow!("Invalid record_date: {}", e))
            .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?;
        let sequence = values.1.parse::<u128>().map_err(|e| anyhow!(e))?;
        let is_translated = match values.2 {
            "1" => true,
            "0" => false,
            _ => return Err(anyhow!("Invalid is_translated value")),
        };
        Ok(GKGRecordID {
            record_date,
            sequence,
            is_translated,
        })
    }
}

impl PositiveScore {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<Option<&str>> for PositiveScore {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<f32>().map(PositiveScore).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing PositiveScore")),
        }
    }
}

impl NegativeScore {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<Option<&str>> for NegativeScore {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<f32>().map(NegativeScore).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing NegativeScore")),
        }
    }
}

impl Polarity {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<Option<&str>> for Polarity {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<f32>().map(Polarity).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing Polarity")),
        }
    }
}

impl ActivityReferenceDensity {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<Option<&str>> for ActivityReferenceDensity {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<f32>()
                .map(ActivityReferenceDensity)
                .map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing ActivityReferenceDensity")),
        }
    }
}

impl SelfGroupReferenceDensity {
    pub fn value(&self) -> f32 {
        self.0
    }
}

impl TryFrom<Option<&str>> for SelfGroupReferenceDensity {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<f32>()
                .map(SelfGroupReferenceDensity)
                .map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing SelfGroupReferenceDensity")),
        }
    }
}

impl TryFrom<Option<&str>> for WordCount {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<u128>().map(WordCount).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing WordCount")),
        }
    }
}

impl TryFrom<Option<&str>> for ImageURL {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => url::Url::parse(s).map(ImageURL).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing ImageURL")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;
    use log::debug;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = env_logger::try_init();
        });
    }

    #[test]
    fn test_debug_gkg_data_structure() {
        init_logger();

        println!("=== GKG Table Debug Test ===");
        let record = make_valid_gkg_record();
        println!("Total fields: {}", record.len());
        println!("First 10 fields:");
        for (i, field) in record.iter().take(10).enumerate() {
            println!("  Field {}: '{}'", i, field);
        }

        // Test parsing
        match GKGTable::try_from(record) {
            Ok(gkg) => {
                println!("✅ Parsing successful!");
                println!("Record ID: {}", gkg.global_knowledge_graph_id.sequence);
                println!(
                    "Is translated: {}",
                    gkg.global_knowledge_graph_id.is_translated
                );
            }
            Err(e) => {
                println!("❌ Parsing failed: {}", e);
            }
        }
    }

    fn make_valid_gkg_record() -> StringRecord {
        let fields = vec![
            "20250322164500",              // record_date
            "12345",                       // sequence
            "0",                           // is_translated
            "20250322164500",              // date
            "1",                           // source_identifier_type (mention_type_code)
            "http://example.com/article1", // source_identifier (mention_identifier)
            "Example News",                // source_common_name
            "",                            // counts (empty for now)
            "",                            // themes (empty for now)
            "",                            // locations (empty for now)
            "",                            // persons (empty for now)
            "",                            // organisation (empty for now)
            "-2.5",                        // tone
            "5.2",                         // positive_score
            "7.8",                         // negative_score
            "-1.3",                        // polarity
            "4.1",                         // activity_reference_density
            "2.9",                         // selfgroup_reference_density
        ];
        StringRecord::from(fields)
    }

    fn make_record_with_fields(fields: Vec<&str>) -> StringRecord {
        StringRecord::from(fields)
    }

    #[test]
    fn test_gkg_table_try_from_valid_record() {
        init_logger();

        let record = make_valid_gkg_record();
        let result = GKGTable::try_from(record);

        assert!(result.is_ok(), "Valid GKG record should parse successfully");

        let gkg = result.unwrap();
        assert_eq!(gkg.global_knowledge_graph_id.sequence, 12345);
        assert!(!gkg.global_knowledge_graph_id.is_translated);
        assert_eq!(gkg.tone().positive_score.value(), 5.2);
        assert_eq!(gkg.tone().negative_score.value(), 7.8);
        assert_eq!(gkg.tone().polarity.value(), -1.3);
        assert_eq!(gkg.tone().activity_reference_density.value(), 4.1);
        assert_eq!(gkg.tone().selfgroup_reference_density.value(), 2.9);
    }

    #[test]
    fn test_gkg_table_try_from_wrong_field_count() {
        init_logger();

        // Test with too few fields
        let record = make_record_with_fields(vec!["field1", "field2", "field3"]);
        let result = GKGTable::try_from(record);
        assert!(result.is_err(), "Should fail with wrong field count");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected 18 fields")
        );

        // Test with too many fields
        let fields: Vec<&str> = (0..25)
            .map(|i| match i {
                0 | 3 => "20250322164500",
                1 => "12345",
                2 => "0",
                4 => "1",
                5 => "http://example.com",
                6 => "Example News",
                12 => "-2.5",
                13 => "5.2",
                14 => "7.8",
                15 => "-1.3",
                16 => "4.1",
                17 => "2.9",
                _ => "",
            })
            .collect();
        let record = make_record_with_fields(fields);
        let result = GKGTable::try_from(record);
        assert!(result.is_err(), "Should fail with too many fields");
    }

    #[test]
    fn test_gkg_table_try_from_invalid_date() {
        init_logger();

        let fields = vec![
            "invalid_date", // invalid record_date
            "12345",
            "0",
            "20250322164500",
            "1",
            "http://example.com",
            "Example News",
            "",
            "",
            "",
            "",
            "",
            "-2.5",
            "5.2",
            "7.8",
            "-1.3",
            "4.1",
            "2.9",
        ];
        let record = make_record_with_fields(fields);
        let result = GKGTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid date");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid record_date")
        );
    }

    #[test]
    fn test_gkg_table_try_from_invalid_sequence() {
        init_logger();

        let fields = vec![
            "20250322164500",
            "not_a_number", // invalid sequence
            "0",
            "20250322164500",
            "1",
            "http://example.com",
            "Example News",
            "",
            "",
            "",
            "",
            "",
            "-2.5",
            "5.2",
            "7.8",
            "-1.3",
            "4.1",
            "2.9",
        ];
        let record = make_record_with_fields(fields);
        let result = GKGTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid sequence number");
    }

    #[test]
    fn test_gkg_table_try_from_invalid_translated_flag() {
        init_logger();

        let fields = vec![
            "20250322164500",
            "12345",
            "2", // invalid is_translated (should be 0 or 1)
            "20250322164500",
            "1",
            "http://example.com",
            "Example News",
            "",
            "",
            "",
            "",
            "",
            "-2.5",
            "5.2",
            "7.8",
            "-1.3",
            "4.1",
            "2.9",
        ];
        let record = make_record_with_fields(fields);
        let result = GKGTable::try_from(record);
        assert!(
            result.is_err(),
            "Should fail with invalid is_translated value"
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid is_translated value")
        );
    }

    #[test]
    fn test_gkg_table_try_from_invalid_numeric_fields() {
        init_logger();

        let fields = vec![
            "20250322164500",
            "12345",
            "0",
            "20250322164500",
            "1",
            "http://example.com",
            "Example News",
            "",
            "",
            "",
            "",
            "",
            "not_a_number", // invalid tone
            "5.2",
            "7.8",
            "-1.3",
            "4.1",
            "2.9",
        ];
        let record = make_record_with_fields(fields);
        let result = GKGTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid tone value");
    }

    #[test]
    fn test_gkg_record_id_try_from_valid() {
        init_logger();

        let result = GKGRecordID::try_from(("20250322164500", "12345", "1"));
        assert!(
            result.is_ok(),
            "Valid GKGRecordID should parse successfully"
        );

        let record_id = result.unwrap();
        assert_eq!(record_id.sequence, 12345);
        assert!(record_id.is_translated);
    }

    #[test]
    fn test_gkg_record_id_try_from_invalid_date() {
        init_logger();

        let result = GKGRecordID::try_from(("invalid_date", "12345", "0"));
        assert!(result.is_err(), "Should fail with invalid date");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid record_date")
        );
    }

    #[test]
    fn test_gkg_record_id_try_from_invalid_sequence() {
        init_logger();

        let result = GKGRecordID::try_from(("20250322164500", "not_a_number", "0"));
        assert!(result.is_err(), "Should fail with invalid sequence");
    }

    #[test]
    fn test_gkg_record_id_try_from_invalid_translated() {
        init_logger();

        let result = GKGRecordID::try_from(("20250322164500", "12345", "invalid"));
        assert!(result.is_err(), "Should fail with invalid is_translated");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid is_translated value")
        );
    }
}

#[cfg(test)]
mod newtype_tests {
    use super::*;

    #[test]
    fn test_positive_score_try_from() {
        assert_eq!(PositiveScore::try_from(Some("5.2")).unwrap().0, 5.2);
        assert_eq!(PositiveScore::try_from(Some("0.0")).unwrap().0, 0.0);
        assert!(PositiveScore::try_from(Some("not_a_number")).is_err());
        assert!(PositiveScore::try_from(None).is_err());
    }

    #[test]
    fn test_negative_score_try_from() {
        assert_eq!(NegativeScore::try_from(Some("7.8")).unwrap().0, 7.8);
        assert_eq!(NegativeScore::try_from(Some("0.0")).unwrap().0, 0.0);
        assert!(NegativeScore::try_from(Some("not_a_number")).is_err());
        assert!(NegativeScore::try_from(None).is_err());
    }

    #[test]
    fn test_polarity_try_from() {
        assert_eq!(Polarity::try_from(Some("-1.3")).unwrap().0, -1.3);
        assert_eq!(Polarity::try_from(Some("2.5")).unwrap().0, 2.5);
        assert!(Polarity::try_from(Some("not_a_number")).is_err());
        assert!(Polarity::try_from(None).is_err());
    }

    #[test]
    fn test_activity_reference_density_try_from() {
        assert_eq!(
            ActivityReferenceDensity::try_from(Some("4.1")).unwrap().0,
            4.1
        );
        assert_eq!(
            ActivityReferenceDensity::try_from(Some("0.0")).unwrap().0,
            0.0
        );
        assert!(ActivityReferenceDensity::try_from(Some("not_a_number")).is_err());
        assert!(ActivityReferenceDensity::try_from(None).is_err());
    }

    #[test]
    fn test_selfgroup_reference_density_try_from() {
        assert_eq!(
            SelfGroupReferenceDensity::try_from(Some("2.9")).unwrap().0,
            2.9
        );
        assert_eq!(
            SelfGroupReferenceDensity::try_from(Some("0.0")).unwrap().0,
            0.0
        );
        assert!(SelfGroupReferenceDensity::try_from(Some("not_a_number")).is_err());
        assert!(SelfGroupReferenceDensity::try_from(None).is_err());
    }

    #[test]
    fn test_word_count_try_from() {
        assert_eq!(WordCount::try_from(Some("100")).unwrap().0, 100);
        assert_eq!(WordCount::try_from(Some("0")).unwrap().0, 0);
        assert!(WordCount::try_from(Some("not_a_number")).is_err());
        assert!(WordCount::try_from(Some("-1")).is_err()); // negative should fail for u128
        assert!(WordCount::try_from(None).is_err());
    }

    #[test]
    fn test_image_url_try_from() {
        let valid_url = "https://example.com/image.jpg";
        let result = ImageURL::try_from(Some(valid_url));
        assert!(result.is_ok(), "Valid URL should parse successfully");

        let invalid_url = "not_a_valid_url";
        let result = ImageURL::try_from(Some(invalid_url));
        assert!(result.is_err(), "Invalid URL should fail");

        let result = ImageURL::try_from(None);
        assert!(result.is_err(), "None should fail");
        assert!(result.unwrap_err().to_string().contains("missing ImageURL"));
    }
}
