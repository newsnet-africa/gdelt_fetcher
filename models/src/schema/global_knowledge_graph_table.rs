use crate::components::{
    base_components::{
        global_knowledge_graph::gdelt_category::Category,
        raw_types::{count::RawV2Count, tone::V1Tone},
    },
    *,
};
use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

use crate::components::super_components::geography::Geography;

use super::{
    mention_table::{CharOffset, MentionType, TranslationInfo},
    primary_keys::GlobalKnowledgeGraphRecordID,
};

pub struct GlobalKnowledgeGraphTable {
    pub gkgrecordid: GlobalKnowledgeGraphRecordID,
    pub date: DateTime<Utc>,
    pub source_collection_identifier: MentionType,
    pub source_common_name: V2SourceCommonName,
    pub counts: (Vec<V1Count>, Vec<V2Count>),
    pub themes: Vec<Category>,
    pub locations: Vec<(Geography, Option<CharOffset>)>,
    pub persons: Vec<(String, CharOffset)>,
    pub organisations: Vec<(String, CharOffset)>,
    pub tone: V1Tone,
    pub dates: Vec<(NaiveDate, CharOffset)>,
    pub gcam: String, // GCAM,
    pub sharing_url: Url,
    pub related_images: Vec<Url>,
    pub social_image_embeds: Vec<Url>,
    pub social_video_embeds: Vec<Url>,
    pub quotations: Vec<V2Quotation>,
    pub all_names: Vec<(String, CharOffset)>,
    pub amounts: Vec<V2Amount>,
    pub translation_info: TranslationInfo,
    pub extra_xml: String, // TODO: XML Object type
}
