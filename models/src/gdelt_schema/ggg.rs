use chrono::NaiveDateTime;
use url::Url;

use crate::{
    Schema,
    components::{
        cameo::actors::countries::Country,
        shared::{language::Language, location::Location},
    },
};

pub struct GlobalGeographyGraph {
    pub date_time: NaiveDateTime,
    pub url: Url,
    pub title: String,
    pub sharing_image: Url,
    pub language: Language,
    pub tone: f64,
    pub domain_country: Country,
    pub location: Location,
    pub contextual_text: String,
}

impl<'a> Schema<'a> for GlobalGeographyGraph {
    type Source = GlobalGeographyGraphJSONRecord<'a>;

    type Key = GDGRecordID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, GlobalGeographyGraph> for GlobalGeographyGraphJSONRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
