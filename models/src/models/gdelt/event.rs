use std::str::FromStr;
// region: Imports
use super::utils::gdelt_event_action::GDELTAction;
use crate::models::gdelt::utils::gdelt_actor::GDELTActor;
use crate::models::gdelt::utils::gdelt_date::GDELTDate;
use crate::models::gdelt::{DatabaseTableEntry, GDELTObject, ToProto};
use chrono::DateTime;
use chrono::Utc;
use prost_types::{Timestamp};
use crate::generated::event::Event as GdeltEvent;
// endregion

// region: Event Struct Definition
/// Represents an event in the GDELT dataset.
///
/// The `Event` struct holds various attributes related to a GDELT event,
/// including global event ID, date, source URL, actors, and event action.
#[derive(Debug, Clone)]
pub struct Event {
    /// The global event ID.
    global_event_id: Option<u128>,
    /// The GDELT date of the event.
    gdelt_date: Option<GDELTDate>,
    /// The date the event was added.
    date_added: Option<DateTime<Utc>>,
    /// The source URL of the event.
    source_url: Option<String>,
    /// The first actor involved in the event.
    actor_1: Option<GDELTActor>,
    /// The second actor involved in the event.
    actor_2: Option<GDELTActor>,
    /// The action associated with the event.
    event_action: Option<GDELTAction>,
}
// endregion

impl ToProto for Event {
    type ProtoType = GdeltEvent;
    
    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.global_event_id.is_none()
            && self.gdelt_date.is_none()
            && self.date_added.is_none()
            && self.source_url.is_none()
            && self.actor_1.is_none()
            && self.actor_2.is_none()
            && self.event_action.is_none()
        {
            None
        } else {
            Some(GdeltEvent {
                global_event_id: self.global_event_id.map(|v| v as u64),
                gdelt_date: self.gdelt_date.as_ref().map_or_else(Default::default, |d| d.to_proto()),
                date_added: self.date_added.and_then(|d| Timestamp::from_str(d.to_string().as_str()).ok()),
                source_url: self.source_url.clone(),
                actor_1: self.actor_1.as_ref().and_then(|actor| actor.to_proto()),
                actor_2: self.actor_2.as_ref().and_then(|actor| actor.to_proto()),
                event_action: self.event_action.as_ref().and_then(|action| action.to_proto()),
            })
        }
    }
}

// region: Event Implementation
impl Event {
    // region: Getters
    /// Returns the global event ID.
    pub fn global_event_id(&self) -> Option<u128> {
        self.global_event_id
    }

    /// Returns a reference to the GDELT date.
    pub fn gdelt_date(&self) -> &Option<GDELTDate> {
        &self.gdelt_date
    }

    /// Returns the date the event was added.
    pub fn date_added(&self) -> Option<DateTime<Utc>> {
        self.date_added
    }

    /// Returns a reference to the source URL.
    pub fn source_url(&self) -> &Option<String> {
        &self.source_url
    }

    /// Returns a reference to the first actor.
    pub fn actor_1(&self) -> &Option<GDELTActor> {
        &self.actor_1
    }

    /// Returns a reference to the second actor.
    pub fn actor_2(&self) -> &Option<GDELTActor> {
        &self.actor_2
    }

    /// Returns a reference to the event action.
    pub fn event_action(&self) -> &Option<GDELTAction> {
        &self.event_action
    }
    // endregion

    // region: Setters
    /// Sets the global event ID.
    pub fn set_global_event_id(&mut self, global_event_id: Option<u128>) {
        self.global_event_id = global_event_id;
    }

    /// Sets the GDELT date.
    pub fn set_gdelt_date(&mut self, gdelt_date: Option<GDELTDate>) {
        self.gdelt_date = gdelt_date;
    }

    /// Sets the date the event was added.
    pub fn set_date_added(&mut self, date_added: Option<DateTime<Utc>>) {
        self.date_added = date_added;
    }

    /// Sets the source URL.
    pub fn set_source_url(&mut self, source_url: Option<String>) {
        self.source_url = source_url;
    }

    /// Sets the first actor.
    pub fn set_actor_1(&mut self, actor_1: Option<GDELTActor>) {
        self.actor_1 = actor_1;
    }

    /// Sets the second actor.
    pub fn set_actor_2(&mut self, actor_2: Option<GDELTActor>) {
        self.actor_2 = actor_2;
    }

    /// Sets the event action.
    pub fn set_event_action(&mut self, event_action: Option<GDELTAction>) {
        self.event_action = event_action;
    }
    // endregion
}
// endregion

// region: DatabaseTableEntry Implementation
impl DatabaseTableEntry for Event {
    /// Creates a blank `Event` instance with all fields set to `None`.
    fn blank() -> Self {
        Self {
            global_event_id: None,
            gdelt_date: None,
            date_added: None,
            source_url: None,
            actor_1: None,
            actor_2: None,
            event_action: None,
        }
    }

    /// Creates an `Event` instance from a CSV row.
    ///
    /// # Arguments
    ///
    /// * `row` - A string slice that holds the CSV row.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `Event` instance. Returns `None` if parsing fails.
    fn from_csv_row(row: &str) -> Option<Self> {
        Self::from_strings(row)
    }
}
// endregion

// region: GDELTObject Implementation
impl GDELTObject for Event {
    /// Creates an `Event` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited event attributes.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `Event` instance. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        // Split the record into fields using the '\t' delimiter.
        let fields = <Self as GDELTObject>::delimited_vector("\t", record);
        Self::new(fields)
    }

    /// Creates a new `Event` instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the event.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `Event` instance. Returns `None` if all fields are default values.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Parse the global event ID field.
        let global_event_id = match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => match value.parse::<u128>() {
                Ok(value_u128) => Some(value_u128),
                Err(_) => None,
            },
        };

        // Parse the GDELT date field.
        let gdelt_date = match fields.get(1) {
            Some(&"") | None => None,
            Some(value) => GDELTDate::from_strings(value),
        };

        // Parse the date added field.
        let date_added = match fields.get(59) {
            Some(&"") | None => None,
            Some(value) => match value.parse::<i64>() {
                Ok(value) => GDELTDate::date_from_int(value),
                Err(_) => None,
            },
        };

        // Parse the source URL field.
        let source_url = match fields.get(60) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the first actor fields.
        let actor_1 = match fields.get(5..=14) {
            Some(values) => GDELTActor::new(values.to_vec()),
            None => None,
        };

        // Parse the second actor fields.
        let actor_2 = match fields.get(15..=24) {
            Some(values) => GDELTActor::new(values.to_vec()),
            None => None,
        };

        // Parse the event action fields.
        let event_action = match fields.get(25..=34) {
            Some(values) => GDELTAction::new(values.to_vec()),
            None => None,
        };

        // If all fields are empty, return None. Otherwise, return a new Event instance.
        if global_event_id.is_none()
            && gdelt_date.is_none()
            && date_added.is_none()
            && source_url.is_none()
            && actor_1.is_none()
            && actor_2.is_none()
            && event_action.is_none()
        {
            None
        } else {
            Some(Self {
                global_event_id,
                gdelt_date,
                date_added,
                source_url,
                actor_1,
                actor_2,
                event_action,
            })
        }
    }
}
// endregion