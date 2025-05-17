pub mod base_components;
pub mod super_components;

use base_components::raw_types::{
    location::{Latitude, Longitude, RawFeatureID, RawLocationName},
    *,
};

pub struct ActorName(pub String);
pub struct GlobalKnowledgeGraphRecordID(pub String);
pub struct V2SourceCommonName(pub String);
pub struct V2DocumentIdentifier(pub String);
pub struct V1Theme(pub String);
pub struct V1Person(pub String);
pub struct V1Organisation(pub String);
pub struct V2SharingImage(pub String);
pub struct V2RelatedImage(pub String);
pub struct V2SocialMediaEmbed(pub String);
pub struct V2SocialVideoEmbed(pub String);
pub struct PageURL(pub String);
pub struct PageTitle(pub String);
pub struct PageDomainFull(pub String);
pub struct PageDomainRoot(pub String);
pub struct PageLanguage(pub String);
pub struct FetchDateOriginal(pub String);
pub struct FetchDateCheck(pub String);
pub struct HTTPCode(pub String);
pub struct RedirectURL(pub String);
pub struct TitleNew(pub String);
pub struct LocationName(String);
pub struct Coordinates(Latitude, Longitude);
pub struct GeoFeature(String);

pub struct V2Theme(pub String, pub CharOffset);
pub struct V2Person(pub String, pub CharOffset);
pub struct V2Organisation(pub String, pub CharOffset);
pub struct V2AllName(pub String, pub CharOffset);
pub struct V2Amount(pub u128, pub String, pub CharOffset);
pub struct V2Quotation {
    pub char_offset: CharOffset,
    pub quotation_length: u128,
    pub quotation_verb: String,
    pub quote_value: String,
}
pub struct TranslationInfo {
    pub source_language_code: [u8; 3],
    pub engine: String,
    pub model: String,
}
pub struct CitedReference {
    pub author: String,
    pub title: String,
    pub book_title: String,
    pub date: String,
    pub journal: String,
    pub volume: String,
    pub issue: String,
    pub pages: String,
    pub institution: String,
    pub publisher: String,
    pub location: String,
}

pub struct V1Count {
    pub count_type: String,
    pub count_value: u128,
    pub count_object: String,
    pub location_type: u8,
    pub location_name: String,
    pub fips_country_code: [u8; 2],
    pub fips_administration_code: [u8; 3],
    pub longitude: f64,
    pub latitude: f64,
    pub feature_id: String,
}

pub struct V2Count {
    pub count_type: String,
    pub count_value: u128,
    pub count_object: String,
    pub location_type: u8,
    pub location_name: String,
    pub fips_country_code: [u8; 2],
    pub fips_administration_code: [u8; 3],
    pub longitude: f64,
    pub latitude: f64,
    pub feature_id: String,
    pub char_offset: CharOffset,
}

impl<'a> From<RawActorName<'a>> for ActorName {
    fn from(raw: RawActorName) -> Self {
        ActorName(raw.0.to_string())
    }
}

impl<'a> From<RawGlobalKnowledgeGraphRecordID<'a>> for GlobalKnowledgeGraphRecordID {
    fn from(raw: RawGlobalKnowledgeGraphRecordID) -> Self {
        GlobalKnowledgeGraphRecordID(raw.0.to_string())
    }
}

impl<'a> From<RawV2SourceCommonName<'a>> for V2SourceCommonName {
    fn from(raw: RawV2SourceCommonName) -> Self {
        V2SourceCommonName(raw.0.to_string())
    }
}

impl<'a> From<RawV2DocumentIdentifier<'a>> for V2DocumentIdentifier {
    fn from(raw: RawV2DocumentIdentifier) -> Self {
        V2DocumentIdentifier(raw.0.to_string())
    }
}

impl<'a> From<RawV1Theme<'a>> for V1Theme {
    fn from(raw: RawV1Theme) -> Self {
        V1Theme(raw.0.to_string())
    }
}

impl<'a> From<RawV1Person<'a>> for V1Person {
    fn from(raw: RawV1Person) -> Self {
        V1Person(raw.0.to_string())
    }
}

impl<'a> From<RawV1Organisation<'a>> for V1Organisation {
    fn from(raw: RawV1Organisation) -> Self {
        V1Organisation(raw.0.to_string())
    }
}

impl<'a> From<RawV2SharingImage<'a>> for V2SharingImage {
    fn from(raw: RawV2SharingImage) -> Self {
        V2SharingImage(raw.0.to_string())
    }
}

impl<'a> From<RawV2RelatedImage<'a>> for V2RelatedImage {
    fn from(raw: RawV2RelatedImage) -> Self {
        V2RelatedImage(raw.0.to_string())
    }
}

impl<'a> From<RawV2SocialMediaEmbed<'a>> for V2SocialMediaEmbed {
    fn from(raw: RawV2SocialMediaEmbed) -> Self {
        V2SocialMediaEmbed(raw.0.to_string())
    }
}

impl<'a> From<RawV2SocialVideoEmbed<'a>> for V2SocialVideoEmbed {
    fn from(raw: RawV2SocialVideoEmbed) -> Self {
        V2SocialVideoEmbed(raw.0.to_string())
    }
}

impl<'a> From<RawPageURL<'a>> for PageURL {
    fn from(raw: RawPageURL) -> Self {
        PageURL(raw.0.to_string())
    }
}

impl<'a> From<RawPageTitle<'a>> for PageTitle {
    fn from(raw: RawPageTitle) -> Self {
        PageTitle(raw.0.to_string())
    }
}

impl<'a> From<RawPageDomainFull<'a>> for PageDomainFull {
    fn from(raw: RawPageDomainFull) -> Self {
        PageDomainFull(raw.0.to_string())
    }
}

impl<'a> From<RawPageDomainRoot<'a>> for PageDomainRoot {
    fn from(raw: RawPageDomainRoot) -> Self {
        PageDomainRoot(raw.0.to_string())
    }
}

impl<'a> From<RawPageLanguage<'a>> for PageLanguage {
    fn from(raw: RawPageLanguage) -> Self {
        PageLanguage(raw.0.to_string())
    }
}

impl<'a> From<RawFetchDateOriginal<'a>> for FetchDateOriginal {
    fn from(raw: RawFetchDateOriginal) -> Self {
        FetchDateOriginal(raw.0.to_string())
    }
}

impl<'a> From<RawFetchDateCheck<'a>> for FetchDateCheck {
    fn from(raw: RawFetchDateCheck) -> Self {
        FetchDateCheck(raw.0.to_string())
    }
}

impl<'a> From<RawHTTPCode<'a>> for HTTPCode {
    fn from(raw: RawHTTPCode) -> Self {
        HTTPCode(raw.0.to_string())
    }
}

impl<'a> From<RawRedirectURL<'a>> for RedirectURL {
    fn from(raw: RawRedirectURL) -> Self {
        RedirectURL(raw.0.to_string())
    }
}

impl<'a> From<RawTitleNew<'a>> for TitleNew {
    fn from(raw: RawTitleNew) -> Self {
        TitleNew(raw.0.to_string())
    }
}

impl<'a> From<RawLocationName<'a>> for LocationName {
    fn from(value: RawLocationName<'a>) -> Self {
        Self(value.0.to_string())
    }
}

impl<'a> From<RawFeatureID<'a>> for GeoFeature {
    fn from(value: RawFeatureID<'a>) -> Self {
        Self(value.0.to_string())
    }
}
