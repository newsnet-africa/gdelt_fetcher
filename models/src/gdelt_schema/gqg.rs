use chrono::NaiveDate;
use url::Url;

use crate::{Schema, components::shared::language::Language};

pub struct Quote {
    pub pre: String,
    pub quote: String,
    pub post: String,
}

pub struct GlobalQuotationGraph {
    pub date: NaiveDate,
    pub url: Url,
    pub title: String,
    pub lang: Language,
    pub quotes: Vec<Quote>,
}

pub struct GlobalQuotatioGraphCSVRecord<'a> {
    date: &'a str,
    url: &'a str,
    title: &'a str,
    lang: &'a str,
    quotes: Vec<&'a str>,
}

impl<'a> Schema<'a> for GlobalQuotationGraph {
    type Source = GlobalQuotationGraphCSVRecord<'a>;

    type Key = GQGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalQuotationGraph> for GlobalQuotationGraphCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
