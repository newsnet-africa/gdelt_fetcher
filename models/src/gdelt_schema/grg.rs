use chrono::NaiveDate;
use url::Url;

use crate::Schema;

pub struct GlobalRelationGraph {
    pub date: NaiveDate,
    pub pre: String,
    pub verb: String,
    pub post: String,
    pub url: Url,
    pub title: String,
}

pub struct GlobalRelationGraphJSONRecord<'a> {
    pub date: &'a str,
    pub pre: &'a str,
    pub verb: &'a str,
    pub post: &'a str,
    pub url: &'a str,
    pub title: &'a str,
}
impl<'a> Schema<'a> for GlobalRelationGraph {
    type Source = GlobalRelationGraphJSONRecord<'a>;

    type Key = GRGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalRelationGraph> for GlobalRelationGraphCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
