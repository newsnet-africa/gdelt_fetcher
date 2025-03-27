use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Schema,
    components::shared::{
        document_type::DocumentType, id_components::GKGRecordID, language::Language,
        location::Location, theme_counts::ThemeCounts, tone::Tone,
    },
    data_sources::SourceRecord,
};
pub type GKGPersons = (String, u128);
pub type GKGOrganisation = (String, u128);

#[repr(transparent)]
pub struct Name(String);

pub struct GKGQuotation {
    pub offset: u128,
    pub length: u128,
    pub verb: String,
    pub quote: String,
}

pub struct GKGAmount {
    pub amount: u128,
    pub object: String,
    pub offset: u128,
}

pub struct TranslationInfo {
    pub source_lang: Language,
    pub citation: String,
}

#[repr(transparent)]
pub struct Author(String);

#[repr(transparent)]
pub struct Institution(String);
#[repr(transparent)]
pub struct Publisher(String);

pub struct Reference {
    pub authors: Vec<Author>,
    pub title: String,
    pub book_title: String,
    pub date: NaiveDate,
    pub journal: String,
    pub volume: u16,
    pub issue: u16,
    pub institution: Institution,
    pub publisher: Publisher,
    pub location: Location,
    pub marker: String,
}

pub struct GlobalKnowledgeGraph {
    pub gkg_record_id: GKGRecordID,
    pub date: NaiveDate,
    pub source_code_identifier: DocumentType,
    pub source_common_name: String,
    pub theme_counts: ThemeCounts,
    pub locations: Vec<Location>,
    pub persons: Vec<GKGPersons>,
    pub organisations: Vec<GKGOrganisation>,
    pub tone: Tone,
    pub dates: NaiveDate,
    pub gcam: String,
    pub sharing_image: Url,
    pub related_images: Vec<Url>,
    pub social_image_embeds: Vec<Url>,
    pub social_video_embeds: Vec<Url>,
    pub quotations: Vec<GKGQuotation>,
    pub names: Vec<Name>,
    pub reference: Reference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalKnowledgeGraphCSVRecord<'a> {}

impl<'a> From<GlobalKnowledgeGraphCSVRecord<'a>> for GlobalKnowledgeGraph {
    fn from(value: GlobalKnowledgeGraphCSVRecord) -> Self {
        todo!()
    }
}
impl<'a> From<&'a GlobalKnowledgeGraphCSVRecord<'a>> for GlobalKnowledgeGraph {
    fn from(value: &GlobalKnowledgeGraphCSVRecord) -> Self {
        todo!()
    }
}

impl<'a> Schema<'a> for GlobalKnowledgeGraph {
    type Source = GlobalKnowledgeGraphCSVRecord<'a>;

    type Key = GKGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalKnowledgeGraph> for GlobalKnowledgeGraphCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
