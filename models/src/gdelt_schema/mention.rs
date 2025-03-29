use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::{
    Schema,
    components::shared::{
        document_type::DocumentType,
        id_components::{GlobalEventID, MentionIdentifier},
    },
    data_sources::SourceRecord,
};

use super::gkg::TranslationInfo;

pub struct Mention {
    pub global_event_id: GlobalEventID,
    pub event_time_date: NaiveDateTime,
    pub mention_time_date: NaiveDateTime,
    pub mention_type: DocumentType,
    pub mention_source_name: String,
    pub mention_identifier: String,
    pub sentence_id: u128,
    pub actor_1_offset: u128,
    pub actor_2_offset: u128,
    pub action_offset: u128,
    pub in_raw_text: bool,
    pub confidence: u8,
    pub mention_doc_len: u128,
    pub mention_doc_tone: u8,
    pub translation_info: TranslationInfo,
}

pub struct MentionCSVRecord<'a> {
    global_event_id: u128,
    event_time_date: u64,
    mention_time_date: u64,
    mention_type: u8,
    mention_source_name: &'a str,
    mention_identifier: &'a str,
    sentence_id: u128,
    actor_1_offset: u128,
    actor_2_offset: u128,
    action_offset: u128,
    in_raw_text: bool,
    confidence: u8,
    mention_doc_len: u128,
    mention_doc_tone: u8,
    translation_info: (&'a str, &'a str),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MentionCSVRecord<'a> {
    //TODO: This shit
}

impl<'a> From<MentionCSVRecord<'a>> for Mention {
    fn from(value: MentionCSVRecord) -> Self {
        todo!()
    }
}
impl<'a> From<&'a MentionCSVRecord<'a>> for Mention {
    fn from(value: &MentionCSVRecord) -> Self {
        todo!()
    }
}

impl<'a> Schema<'a> for Mention {
    type Source = MentionCSVRecord<'a>;

    type Key = MentionIdentifier;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, Mention> for MentionCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
