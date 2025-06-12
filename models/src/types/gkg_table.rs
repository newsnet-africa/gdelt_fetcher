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

pub struct CategoryTheme(String);

pub struct Count {
    pub count_type: CategoryTheme,
    pub count: u128,
    pub object_type: ActorName,
    pub location: EventGeography,
}

pub struct PositiveScore(f32);
pub struct NegativeScore(f32);
pub struct Polarity(f32);
pub struct ActivityReferenceDensity(f32);
pub struct SelfGroupReferenceDensity(f32);
pub struct WordCount(u128);

pub struct AdvancedTone {
    pub tone: Tone,
    pub positive_score: PositiveScore,
    pub negative_score: NegativeScore,
    pub polarity: Polarity,
    pub activity_reference_density: ActivityReferenceDensity,
    pub selfgroup_reference_density: SelfGroupReferenceDensity,
    pub word_count: WordCount,
}

pub struct ImageURL(Url);
pub struct SocialMediaEmbedUrl(Url);

pub struct Quotation {
    pub offset: CharOffset,
    pub length: u128,
    pub verb: String,
}

pub struct Amount {
    amount: u128,
    object: String,
    offset: CharOffset,
}

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

impl TryFrom<StringRecord> for GKGTable {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        // Adjust the number of fields as per the actual GKG file format
        if record.len() != 18 {
            return Err(anyhow!(
                "Expected 18 fields for GKGTable, got {}",
                record.len()
            ));
        }
        let fields: Vec<&str> = record.iter().collect();

        Ok(GKGTable {
            global_knowledge_graph_id: GKGRecordID {
                record_date: chrono::NaiveDateTime::parse_from_str(fields[0], "%Y%m%d%H%M%S")
                    .map_err(|e| anyhow!("Invalid record_date: {}", e))
                    .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,
                sequence: fields[1].parse::<u128>().map_err(|e| anyhow!(e))?,
                is_translated: match fields[2] {
                    "1" => true,
                    "0" => false,
                    _ => return Err(anyhow!("Invalid is_translated value")),
                },
            },
            date: chrono::NaiveDateTime::parse_from_str(fields[3], "%Y%m%d%H%M%S")
                .map_err(|e| anyhow!("Invalid date: {}", e))
                .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))?,
            source_identifier_type: MentionType::try_from(Some((
                MentionTypeCode::try_from(Some(fields[4]))?,
                MentionIdentifier::try_from(Some(fields[5]))?,
            )))?,
            source_common_name: MentionSourceName::try_from(Some(fields[6]))?,
            counts: vec![],       // TODO: parse counts from fields[7]
            themes: vec![],       // TODO: parse themes from fields[8]
            locations: vec![],    // TODO: parse locations from fields[9]
            persons: vec![],      // TODO: parse persons from fields[10]
            organisation: vec![], // TODO: parse organisation from fields[11]
            tone: AdvancedTone {
                tone: Tone::try_from(Some(fields[12]))?,
                positive_score: PositiveScore(fields[13].parse::<f32>().map_err(|e| anyhow!(e))?),
                negative_score: NegativeScore(fields[14].parse::<f32>().map_err(|e| anyhow!(e))?),
                polarity: Polarity(fields[15].parse::<f32>().map_err(|e| anyhow!(e))?),
                activity_reference_density: ActivityReferenceDensity(
                    fields[16].parse::<f32>().map_err(|e| anyhow!(e))?,
                ),
                selfgroup_reference_density: SelfGroupReferenceDensity(
                    fields[17].parse::<f32>().map_err(|e| anyhow!(e))?,
                ),
                word_count: WordCount(0), // TODO: parse word_count if available
            },
            enhanced_dates: vec![], // TODO: parse enhanced_dates
            sharing_image: ImageURL(url::Url::parse("https://example.com").unwrap()), // TODO: parse sharing_image
            related_images: vec![],      // TODO: parse related_images
            social_media_images: vec![], // TODO: parse social_media_images
            social_media_videos: vec![], // TODO: parse social_media_videos
            all_names: vec![],           // TODO: parse all_names
            mention_doc_translation_info: (None, None), // TODO: parse translation info
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

impl TryFrom<Option<&str>> for PositiveScore {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<f32>().map(PositiveScore).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing PositiveScore")),
        }
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

impl TryFrom<Option<&str>> for Polarity {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<f32>().map(Polarity).map_err(|e| anyhow!(e)),
            None => Err(anyhow!("missing Polarity")),
        }
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
