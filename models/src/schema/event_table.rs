use chrono::{DateTime, NaiveDateTime, Utc};
use url::Url;

use crate::components::super_components::{
    actor::Actor, event_action::EventAction, geography::EventGeography,
};

use super::primary_keys::GlobalEventID;

pub enum EventSource {
    URL(Url),
    Citation(String),
}

pub struct EventTable {
    pub global_event_id: GlobalEventID,
    pub day: DateTime<Utc>,
    pub actor_1: Actor,
    pub actor_2: Actor,
    pub event: EventAction,
    pub geography: EventGeography,
    pub date_added: DateTime<Utc>,
    pub source: EventSource,
}
