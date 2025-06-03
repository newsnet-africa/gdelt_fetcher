use chrono::{DateTime, Utc};

use super::{event_table::GlobalEventID, lookup_types::mention_type::MentionType};

pub struct MentionTypeCode(pub u8);
pub struct MentionSourceName(pub String);
pub struct MentionIdentifier(pub String);
pub struct SentenceID(pub u128);
pub struct CharOffset(pub u128);
pub struct InRawText(pub bool);
pub struct Confidence(pub u8);
pub struct MentionDocLength(pub u128);
pub struct MentionDocTone(pub i32);
pub struct SourceLanguageCode(pub [u8; 3]);
pub struct Engine(pub String);

pub struct MentionTable {
    pub global_event_id: GlobalEventID,
    pub event_date: DateTime<Utc>,
    pub mention_date: DateTime<Utc>,
    pub mention_type: MentionType,
    pub mention_source_name: MentionSourceName,
    pub mention_identifier: MentionIdentifier,
    pub sentence_id: SentenceID,
    pub actor_1_char_offset: CharOffset,
    pub actor_2_char_offset: CharOffset,
    pub action_char_offset: CharOffset,
    pub in_raw_text: InRawText,
    pub confidence: Confidence,
    pub mention_doc_len: MentionDocLength,
    pub mention_doc_tone: MentionDocTone,
    pub mention_doc_translation_info: (SourceLanguageCode, Engine),
    pub extras: String,
}
