use actor::Actor;
use chrono::{DateTime, Utc};
use event_action::EventAction;
use event_geography::EventGeography;
use url::Url;

pub struct GlobalEventID(pub u128);
pub struct YearMonthDay(pub u32);
pub struct YearMonth(pub u32);
pub struct Year(pub u16);
pub struct FractionDate(pub f32);
pub struct YearMonthDayHourMinuteSecond(pub u64);

pub struct EventTable {
    pub global_event_id: GlobalEventID,
    pub date: DateTime<Utc>,
    pub actor_1: Actor,
    pub actor_2: Actor,
    pub event_action: EventAction,
    pub actor_1_geograpy: EventGeography,
    pub actor_2_geography: EventGeography,
    pub action_geography: EventGeography,
    pub date_added: DateTime<Utc>,
    pub src_url: Url,
}

pub mod actor {
    use crate::types::lookup_types::{
        country::CountryZone, ethnicity::Ethnicity, known_group::KnownGroup, religion::Religion,
        role::ActorRole,
    };

    pub struct CAMEOActorCode(pub [[u8; 3]; 5]);
    pub struct ActorName(pub String);
    pub struct CAMEOCountryCode(pub [u8; 3]);
    pub struct CAMEOKnownGroupCode(pub [u8; 3]);
    pub struct CAMEOEthnicityCode(pub [u8; 3]);
    pub struct CAMEOReligionCode(pub [u8; 3]);
    pub struct CAMEORoleCode(pub [u8; 3]);

    pub struct Actor {
        pub name: Option<ActorName>,
        pub country: Option<CountryZone>,
        pub known_group: Option<KnownGroup>,
        pub ethnicity: Option<Ethnicity>,
        pub religion: (Option<Religion>, Option<Religion>),
        pub actor_type: (Option<ActorRole>, Option<ActorRole>, Option<ActorRole>),
    }
}

pub mod event_action {
    use crate::types::lookup_types::{
        event_action_description::top_level_actions::EventActionDescription, quad_class::QuadClass,
    };

    pub struct IsRootEvent(pub bool);
    pub struct CAMEOEventCode(pub [u8; 4]);
    pub struct CAMEOEventBaseCode(pub [u8; 3]);
    pub struct CAMEOEventRootCode(pub [u8; 2]);
    pub struct QuadClassCode(pub u8);
    pub struct GoldsteinScale(pub f32);
    pub struct NumberOfMentions(pub u64);
    pub struct NumberOfSources(pub u64);
    pub struct NumberOfArticles(pub u64);
    pub struct Tone(pub f64);

    pub struct EventAction {
        pub is_root_event: IsRootEvent,
        pub event_action: EventActionDescription,
        pub quad_class: QuadClass,
        pub goldstein_scale: GoldsteinScale,
        pub number_of_mentions: NumberOfMentions,
        pub number_of_sources: NumberOfSources,
        pub number_of_articles: NumberOfArticles,
        pub average_tone: Tone,
    }
}

pub mod event_geography {
    use crate::types::lookup_types::{country::CountryZone, geography_type::GeographyType};

    pub struct GeographyTypeCode(pub u8);
    pub struct GeographyFullName(pub String);
    pub struct FIPSCountryCode(pub [u8; 2]);
    pub struct FIPSAdministrationCode(pub [u8; 2]);
    pub struct Administration2Code(pub String);
    pub struct Latitude(pub f64);
    pub struct Longitude(pub f64);
    pub struct FeatureID(pub String);

    pub struct EventGeography {
        pub geography_type: GeographyType,
        pub geograpgy_fullname: GeographyFullName,
        pub geography_country: CountryZone,
        pub coordinates: (Latitude, Longitude),
        pub feature_id: FeatureID,
    }
}
