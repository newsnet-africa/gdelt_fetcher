use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    Schema,
    components::shared::{id_components::GDGRecordID, language::Language},
    data_sources::SourceRecord,
};

pub struct PageMetadata {
    pub url: Url,
    pub title: String,
    pub domain_full: String,
    pub domain_root: String,
    pub language: Language,
}

pub struct Change {
    pub pre: String,
    pub post: String,
    pub from: u128,
    pub to: u128,
}

pub enum GlobalDifferenceGraph {
    HTTPError {
        id: GDGRecordID,
        meta: PageMetadata,
        fetchdate_origional: NaiveDateTime,
        fetchdate_checked: NaiveDateTime,
        http_code: String,
        http_size: u128,
    },
    HTTPRedirect {
        id: GDGRecordID,
        mata: PageMetadata,
        fetchdate_origional: NaiveDateTime,
        fetchdate_checked: NaiveDateTime,
        http_code: String,
        http_size: u128,
    },
    UnchangedHTML {
        id: GDGRecordID,
        mata: PageMetadata,
        fetchdate_origional: NaiveDateTime,
        fetchdate_checked: NaiveDateTime,
    },
    UnchangedContent {
        id: GDGRecordID,
        mata: PageMetadata,
        fetchdate_origional: NaiveDateTime,
        fetchdate_checked: NaiveDateTime,
    },
    PageTitleChange {
        id: GDGRecordID,
        meta: PageMetadata,
        fetchdate_orig: NaiveDateTime,
        fetchdate_check: NaiveDateTime,
        title_new: String,
        num_changes: u32,
        change_unit: String,
        from_numchars: u32,
        to_numchars: u32,
        from_changedchars: u32,
        to_changedchars: u32,
        tot_changedchars: u32,
        perc_changedchars: f32,
        changes: Vec<Change>,
    },
    PageTextChange {
        id: GDGRecordID,
        meta: PageMetadata,
        fetchdate_orig: NaiveDateTime,
        fetchdate_check: NaiveDateTime,
        title_new: String,
        num_changes: u32,
        change_unit: String,
        from_numchars: u32,
        to_numchars: u32,
        from_changedchars: u32,
        to_changedchars: u32,
        tot_changedchars: u32,
        perc_changedchars: f32,
        changes: Vec<Change>,
    },
}
#[derive(Debug, Serialize, Deserialize)]
pub enum GlobalDifferenceGraphCSVRecord<'a> {
    HTTP_ERROR {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
        http_code: u16,
        http_size: u64,
    },
    HTTP_REDIRECT {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
        http_code: u16,
        http_size: u64,
        redirect_url: &'a str,
    },
    UNCHANGED_HTML {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
    },
    UNCHANGED_CONTENT {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
    },
    PAGE_TITLECHANGE {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
        title_new: &'a str,
        num_changes: u32,
        change_unit: &'a str,
        from_numchars: u32,
        to_numchars: u32,
        from_changedchars: u32,
        to_changedchars: u32,
        tot_changedchars: u32,
        perc_changedchars: f32,
        changes: &'a str, // JSON array as a string
    },
    PAGE_TEXTCHANGE {
        page_url: &'a str,
        page_title: &'a str,
        page_domain_full: &'a str,
        page_domain_root: &'a str,
        page_lang: &'a str,
        fetchdate_orig: &'a str,
        fetchdate_check: &'a str,
        title_new: &'a str,
        num_changes: u32,
        change_unit: &'a str,
        from_numchars: u32,
        to_numchars: u32,
        from_changedchars: u32,
        to_changedchars: u32,
        tot_changedchars: u32,
        perc_changedchars: f32,
        changes: &'a str, // JSON array as a string
    },
}

impl<'a> From<GlobalDifferenceGraphCSVRecord<'a>> for GlobalDifferenceGraph {
    fn from(value: GlobalDifferenceGraphCSVRecord) -> Self {
        todo!()
    }
}
impl<'a> From<&'a GlobalDifferenceGraphCSVRecord<'a>> for GlobalDifferenceGraph {
    fn from(value: &GlobalDifferenceGraphCSVRecord) -> Self {
        todo!()
    }
}

impl<'a> Schema<'a> for GlobalDifferenceGraph {
    type Source = GlobalDifferenceGraphCSVRecord<'a>;

    type Key = GDGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalDifferenceGraph> for GlobalDifferenceGraphCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
