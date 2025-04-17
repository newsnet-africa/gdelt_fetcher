use chrono::{DateTime, Utc};
use rust_iso639::{IndividualLanguages, LanguageCode};
use url::Url;

pub struct PageTitle(String);

pub struct PageDomain {
    pub root: Url,
    pub subdomain: Vec<String>,
}

pub struct GDGMetaData {
    pub page_url: Url,
    pub page_title: PageTitle,
    pub page_domain: PageDomain,
    pub page_lang: IndividualLanguages,
    pub fetchdate: (DateTime<Utc>, DateTime<Utc>),
}

pub enum ChangeUnit {
    Word,
    Char,
}

pub struct CharChange {
    from: u128,
    to: u128,
    total: u128,
}

pub enum GlobalDifferenceGraph {
    HTTPError {
        metadata: GDGMetaData,
        http_code: String, // TODO: Enum of HTTP Errors
        http_size: u128,
    },
    HTTPRedirect {
        metadata: GDGMetaData,
        http_code: String, // TODO: Enum of HTTP Errors
        http_size: u128,
        redirect_url: Url,
    },
    UnchangedHTML {
        metadata: GDGMetaData,
    },
    UnchangedContent {
        metadata: GDGMetaData,
    },
    PageTitleChange {
        metadata: GDGMetaData,
        title_new: PageTitle,
        number_of_changes: u128,
        change_unit: ChangeUnit,
    },
    PageTextChange {
        metadata: GDGMetaData,
        text_new: String,
        number_of_changes: u128,
        change_unit: ChangeUnit,
    },
}
