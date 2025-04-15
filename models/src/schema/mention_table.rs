use chrono::{DateTime, Utc};

use crate::components::base_components::event::AverageTone;

use super::primary_keys::GlobalEventID;

pub enum MentionType {
    Web(todo!()),
    CitationOnly(todo!()),
    Core(todo!()),
    DTIC(todo!()),
    JSTOR(todo!()),
    NonTextualSource(todo!()),
}

pub struct SentenceID(u128);

pub struct CharOffset(u128);

pub struct Confidence(u8);

pub struct TranslationInfo {
    pub source_lang: Language,
    pub engine: String,
}

pub struct MentionTable {
    pub global_event_id: GlobalEventID,
    pub event_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub mention_type: MentionType,
    pub mention_source_name: String,
    pub actor_1_offset: CharOffset,
    pub actor_2_offset: CharOffset,
    pub action_offset: CharOffset,
    pub in_raw_text: bool,
    pub confidence: Confidence,
    pub doc_len: u128,
    pub average_tone: AverageTone,
}
