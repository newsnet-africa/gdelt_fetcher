use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;
use url::Url;

use crate::{
    Schema,
    components::shared::{
        actor::Actor, event_action::EventAction, id_components::GlobalEventID, location::Location,
    },
    data_sources::SourceRecord,
};

pub type EventActor = (Actor, Location);

pub struct Event {
    pub global_event_id: GlobalEventID,
    pub date: NaiveDateTime,
    pub actors: [EventActor; 2],
    pub event_action: EventAction,
    pub location: Location,
    pub date_added: NaiveDateTime,
    pub source_url: Url,
}

#[derive(Debug, Deserialize)]
pub struct EventCSVRecord<'a> {
    pub global_event_id: u128,
    pub day: u32,
    pub month_year: u32,
    pub year: u16,
    pub fraction_date: f64,

    pub actor_1_code: &'a str,
    pub actor_1_name: &'a str,
    pub actor_1_country_code: [u8; 3],
    pub actor_1_known_group_code: &'a str,
    pub actor_1_ethnic_code: &'a str,
    pub actor_1_religion_1_code: &'a str,
    pub actor_1_religion_2_code: &'a str,
    pub actor_1_type_1_code: [u8; 3],
    pub actor_1_type_2_code: [u8; 3],
    pub actor_1_type_3_code: [u8; 3],

    pub actor_2_code: &'a str,
    pub actor_2_name: &'a str,
    pub actor_2_country_code: [u8; 3],
    pub actor_2_known_group_code: &'a str,
    pub actor_2_ethnic_code: &'a str,
    pub actor_2_religion_1_code: &'a str,
    pub actor_2_religion_2_code: &'a str,
    pub actor_2_type_1_code: [u8; 3],
    pub actor_2_type_2_code: [u8; 3],
    pub actor_2_type_3_code: [u8; 3],

    pub is_root_event: bool,
    pub event_code: &'a str,
    pub quad_class: u8,
    pub goldstein_scale: f32,
    pub num_mentions: u128,
    pub num_sources: u128,
    pub num_articles: u128,
    pub average_tone: f32,

    pub actor_1_geo_type: u8,
    pub actor_1_geo_fullname: &'a str,
    pub actor_1_geo_country_code: [u8; 2],
    pub actor_1_geo_adm_1_code: [u8; 2],
    pub actor_1_geo_adm_2_code: &'a str,
    pub actor_1_geo_longitude: f64,
    pub actor_1_geo_latitude: f64,
    pub actor_1_feature_id: &'a str,

    pub actor_2_geo_type: u8,
    pub actor_2_geo_fullname: &'a str,
    pub actor_2_geo_country_code: [u8; 2],
    pub actor_2_geo_adm_1_code: [u8; 2],
    pub actor_2_geo_adm_2_code: &'a str,
    pub actor_2_geo_longitude: f64,
    pub actor_2_geo_latitude: f64,
    pub actor_2_feature_id: &'a str,

    pub action_geo_type: u8,
    pub action_geo_fullname: &'a str,
    pub action_geo_country_code: [u8; 2],
    pub action_geo_adm_1_code: [u8; 2],
    pub action_geo_adm_2_code: &'a str,
    pub action_geo_longitude: f64,
    pub action_geo_latitude: f64,
    pub action_feature_id: &'a str,

    pub date_added: u128,
    pub source_url: &'a str,
}

impl<'a> From<EventCSVRecord<'a>> for Event {
    fn from(value: EventCSVRecord) -> Self {
        todo!()
    }
}
impl<'a> From<&'a EventCSVRecord<'a>> for Event {
    fn from(value: &EventCSVRecord) -> Self {
        todo!()
    }
}

impl<'a> Schema<'a> for Event {
    type Source = EventCSVRecord<'a>;

    type Key = GlobalEventID;

    fn depends_on<'other_schema, T: Schema<'other_schema>>(&self) -> Option<T::Key> {
        todo!()
    }

    fn id(&self) -> Self::Key {
        todo!()
    }
}

impl<'de> SourceRecord<'de, Event> for EventCSVRecord<'de> {
    fn validate(&self) -> bool {
        todo!()
    }
}
