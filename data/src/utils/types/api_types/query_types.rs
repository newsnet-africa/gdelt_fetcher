use super::operation::Operation;
use iso_country::Country;
use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;
use models::models::gdelt::utils::gdelt_languages::GDELTLanguage;

use super::ToRequestLink;

pub enum QueryType {
    QueryString(String),
    Exclude(Box<QueryType>),
    Domain(String),
    DomainIs(String),
    ImageFaceTone(Operation<f32>),
    ImageNumFaces(Operation<u8>),
    ImageORCMeta(String),
    ImageTags(String),
    ImageWebTag(String),
    ImageWebCount(Operation<u128>),
    Near { distance: u8, root_word: String },
    Repeat { distance: u8, root_word: String },
    SourceCountry(Country),
    SourceLang(GDELTLanguage),
    Theme(GDELTCategoryList),
    Tone(Operation<f32>),
    ToneAbs(Operation<f32>),
}

impl ToRequestLink for QueryType {
    fn to_request_link(&self) -> String {
        match self {
            Self::QueryString(query_string) => query_string.clone(),
            Self::Exclude(query_type) => {
                format!("-\"{}\"", query_type.to_request_link())
            }
            Self::Domain(domain) => {
                format!("domain:{}", domain)
            }
            Self::DomainIs(domain) => {
                format!("domainis:{}", domain)
            }
            Self::ImageFaceTone(op) => {
                format!("imagefacetone:{}", op.to_request_link())
            }
            Self::ImageNumFaces(op) => {
                format!("imagenumfaces:{}", op.to_request_link())
            }
            Self::ImageORCMeta(meta) => {
                format!("imageorcmeta:\"{}\"", meta)
            }
            Self::ImageTags(vect) => {
                format!("imagetags:\"{}\"", vect)
            }
            Self::ImageWebCount(op) => {
                format!("imagewebcount:{}", op.to_request_link())
            }
            Self::ImageWebTag(tag) => {
                format!("imagewebtag:\"{}\"", tag)
            }
            Self::Near {
                distance,
                root_word,
            } => {
                format!("near{}:\"{}\"", root_word, distance)
            }
            Self::Repeat {
                distance,
                root_word,
            } => {
                format!("repeat{}:\"{}\"", root_word, distance)
            }
            Self::SourceCountry(country) => {
                format!(
                    "sourcecountry:{}",
                    country.name().to_lowercase().replace(" ", "")
                )
            }
            Self::SourceLang(lang) => {
                format!("sourcelang:{}", lang.to_request_link())
            }
            Self::Theme(theme) => {
                format!("theme:{}", theme.to_request_link())
            }
            Self::Tone(op) => {
                format!("tone:{}", op.to_request_link())
            }
            Self::ToneAbs(op) => {
                format!("toneabs:{}", op.to_request_link())
            }
        }
    }
}

impl ToRequestLink for Vec<QueryType> {
    fn to_request_link(&self) -> String {
        match self.len() {
            0 => String::new(),
            1 => self[0].to_request_link(),
            _ => {
                let mut query = "(".to_string();
                for (i, q) in self.iter().enumerate() {
                    query.push_str(&q.to_request_link());
                    if i < self.len() - 1 {
                        query.push_str(" OR ");
                    }
                }
                query
            }
        }
    }
}
