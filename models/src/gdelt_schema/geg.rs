use chrono::NaiveDate;
use url::Url;

use crate::{
    Schema,
    components::shared::{entity::Entity, id_components::GEGRecordID, sentiment::Sentiment},
    data_sources::SourceRecord,
};

pub struct GlobalEntityGraph {
    pub date: NaiveDate,
    pub url: Url,
    pub sentiment: Sentiment,
    pub entities: Vec<Entity>,
}
impl<'a> Schema<'a> for GlobalEntityGraph {
    type Source = GlobalEntityGraphJSONRecord<'a>;

    type Key = GEGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalEntityGraph> for GlobalEntityGraphJSONRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
