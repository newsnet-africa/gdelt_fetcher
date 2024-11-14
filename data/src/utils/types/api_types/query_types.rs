use iso_country::Country;
use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;
use models::models::gdelt::utils::language::Language;
use models::models::gdelt::utils::operation::Operation;

pub enum QueryType {
    Quote(String),
    BooleanOR(Vec<String>),
    Exclude(Vec<QueryType>),
    Domain(String),
    DomainIs(String),
    ImageFaceTone(Operation, f32),
    ImageNumFaces(Operation, u8),
    ImageORCMeta(String),
    ImageTags(Vec<String>),
    ImageWebCount(Operation, u128),
    Near {
        distance: u8,
        root_word: String,
        near_word: String,
    },
    Repeat(String),
    SourceCountry(Country),
    SourceLang(Language),
    Theme(GDELTCategoryList),
    Tone(Operation, f32),
    ToneAbs(Operation, f32),
}
