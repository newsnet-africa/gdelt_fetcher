use super::operation::Operation;
use iso_country::Country;
use models::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;
use models::models::gdelt::utils::gdelt_languages::GDELTLanguage;

use super::ToRequestLink;

pub enum QueryType {
    QueryString(String),
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
    SourceLang(GDELTLanguage),
    Theme(GDELTCategoryList),
    Tone(Operation, f32),
    ToneAbs(Operation, f32),
}

impl ToRequestLink for QueryType {
    fn to_request_link(&self) -> String {
        match self {
            Self::QueryString(query_string) => query_string.clone(),
            Self::Exclude(vect) => {
                let mut query = String::new();
                for (i, q) in vect.iter().enumerate() {
                    query.push_str(&q.to_request_link());
                    if i < vect.len() - 1 {
                        query.push_str(" AND ");
                    }
                }
                query
            }
            Self::Domain(domain) => {
                format!("domain:{}", domain)
            }
            Self::DomainIs(domain) => {
                format!("domainis:{}", domain)
            }
            Self::ImageFaceTone(op, tone) => {
                format!("imagefacetone:{}{}", op.to_request_link(), tone)
            }
            Self::ImageNumFaces(op, num) => {
                format!("imagenumfaces:{}{}", op.to_request_link(), num)
            }
            Self::ImageORCMeta(meta) => {
                format!("imageorcmeta:{}", meta)
            }
            Self::ImageTags(vect) => {
                let mut query = String::new();
                for (i, q) in vect.iter().enumerate() {
                    query.push_str(&q);
                    if i < vect.len() - 1 {
                        query.push_str(" OR ");
                    }
                }
                query
            }
            Self::ImageWebCount(op, count) => {
                format!("imagewebcount:{}{}", op.to_request_link(), count)
            }
            Self::Near {
                distance,
                root_word,
                near_word,
            } => {
                format!("{} NEAR{}{}", root_word, distance, near_word)
            }
            Self::Repeat(quote) => {
                format!("repeat:{}", quote)
            }
            Self::SourceCountry(country) => {
                format!("sourcecountry:{}", country.name())
            }
            Self::SourceLang(lang) => {
                format!("sourcelang:{}", lang.to_request_link())
            }
            Self::Theme(theme) => {
                format!("theme:{}", theme.to_request_link())
            }
            Self::Tone(op, tone) => {
                format!("tone:{}{}", op.to_request_link(), tone)
            }
            Self::ToneAbs(op, tone) => {
                format!("toneabs:{}{}", op.to_request_link(), tone)
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
