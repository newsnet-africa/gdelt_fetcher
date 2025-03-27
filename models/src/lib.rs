#![feature(error_iter)]
pub mod components;
pub mod gdelt_schema;

use data_sources::SourceRecord;
use gdelt_schema::SchemaKey;
pub mod data_sources {
    use crate::Schema;
    use rss::Item;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    pub trait SourceRecord<'a, RecordSchema: Schema<'a, Source = Self> + From<Self>>:
        Debug + Deserialize<'a> + Into<RecordSchema>
    {
        fn validate(&self) -> bool;
    }

    pub mod rss_source {

        use crate::Schema;
        use core::error::Source;
        use rss::{
            Category, Enclosure, Guid, Item,
            extension::{
                ExtensionMap, dublincore::DublinCoreExtension, itunes::ITunesItemExtension,
            },
        };

        use super::SourceRecord;

        pub trait TableRecordRSS<'a, RecordSchema: Schema<'a, Source = Self> + From<Self>>:
            SourceRecord<'a, RecordSchema> + From<&'a Item>
        where
            Self: 'a,
        {
            fn metadata(&self) -> &&&RSSMetadata;
        }

        pub struct RSSMetadata<'a> {
            pub title: Option<String>,
            pub link: Option<String>,
            pub description: Option<String>,
            pub author: Option<String>,
            pub categories: Vec<Category>,
            pub comments: Option<String>,
            pub enclosure: Option<Enclosure>,
            pub guid: Option<Guid>,
            pub pub_date: Option<String>,
            pub source: Option<Source<'a>>,
            pub extensions: ExtensionMap,
            pub itunes_ext: Option<ITunesItemExtension>,
            pub dublin_core_ext: Option<DublinCoreExtension>,
        }
    }
}

pub trait SchemaField<'a, TRecord: Schema<'a>>: From<&'a TRecord::Source> {
    fn nullable(&self) -> bool;
}

pub trait Schema<'source>: From<&'source Self::Source> + From<Self::Source> {
    type Source: 'source + SourceRecord<'source, Self>;
    type Key: SchemaKey<'source, Self>;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key>;
    fn id(&self) -> Self::Key;
}
