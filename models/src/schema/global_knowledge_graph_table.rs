use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

use crate::components::{
    base_components::global_knowledge_graph::{
        Amount, GKGSourceCommonName, Quotation, counts::Count, gdelt_category::Category, tone::Tone,
    },
    super_components::geography::Geography,
};

use super::{
    mention_table::{CharOffset, MentionType, TranslationInfo},
    primary_keys::GlobalKnowledgeGraphRecordID,
};

pub struct GlobalKnowledgeGraphTable {
    pub gkgrecordid: GlobalKnowledgeGraphRecordID,
    pub date: DateTime<Utc>,
    pub source_collection_identifier: MentionType,
    pub source_common_name: GKGSourceCommonName,
    pub counts: Vec<Count>,
    pub themes: Vec<Category>,
    pub locations: Vec<(Geography, Option<CharOffset>)>,
    pub persons: Vec<(String, CharOffset)>,
    pub organisations: Vec<(String, CharOffset)>,
    pub tone: Tone,
    pub dates: Vec<(NaiveDate, CharOffset)>,
    pub gcam: GCAM,
    pub sharing_url: Url,
    pub related_images: Vec<Url>,
    pub social_image_embeds: Vec<Url>,
    pub social_video_embeds: Vec<Url>,
    pub quotations: Vec<Quotation>,
    pub all_names: Vec<(String, CharOffset)>,
    pub amounts: Vec<Amount>,
    pub translation_info: TranslationInfo,
    pub extra_xml: String, // TODO: XML Object type
}
