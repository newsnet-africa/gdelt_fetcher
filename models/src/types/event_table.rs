use actor::Actor;
use chrono::{DateTime, Utc};
use event_action::EventAction;
use event_geography::EventGeography;
use log::debug;
use url::Url;

use super::DatabaseTable;

#[derive(Debug)]
pub struct GlobalEventID(pub u128);
impl std::convert::TryFrom<Option<&str>> for GlobalEventID {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u128>()
                .map(GlobalEventID)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing GlobalEventID")),
        }
    }
}
#[derive(Debug)]
pub struct YearMonthDay(pub u32);
impl std::convert::TryFrom<Option<&str>> for YearMonthDay {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u32>()
                .map(YearMonthDay)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing YearMonthDay")),
        }
    }
}
#[derive(Debug)]
pub struct YearMonth(pub u32);
impl std::convert::TryFrom<Option<&str>> for YearMonth {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u32>()
                .map(YearMonth)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing YearMonth")),
        }
    }
}
#[derive(Debug)]
pub struct Year(pub u16);
impl std::convert::TryFrom<Option<&str>> for Year {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s.parse::<u16>().map(Year).map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing Year")),
        }
    }
}
#[derive(Debug)]
pub struct FractionDate(pub f32);
impl std::convert::TryFrom<Option<&str>> for FractionDate {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<f32>()
                .map(FractionDate)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing FractionDate")),
        }
    }
}
#[derive(Debug)]
pub struct YearMonthDayHourMinuteSecond(pub u64);
impl std::convert::TryFrom<Option<&str>> for YearMonthDayHourMinuteSecond {
    type Error = anyhow::Error;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u64>()
                .map(YearMonthDayHourMinuteSecond)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing YearMonthDayHourMinuteSecond")),
        }
    }
}

#[derive(Debug)]
pub struct EventTable {
    pub global_event_id: GlobalEventID,
    pub actor_1: Option<Actor>,
    pub actor_2: Option<Actor>,
    pub event_action: EventAction,
    pub actor_1_geograpy: Option<EventGeography>,
    pub actor_2_geography: Option<EventGeography>,
    pub action_geography: Option<EventGeography>,
    pub date_added: DateTime<Utc>,
    pub src_url: Url,
}

impl DatabaseTable for EventTable {}

impl std::convert::TryFrom<csv::StringRecord> for EventTable {
    type Error = anyhow::Error;

    fn try_from(record: csv::StringRecord) -> anyhow::Result<Self> {
        pub(crate) mod conversion {
            use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

            pub(crate) fn convert_to_utc_datetime(date_time_str: &str) -> DateTime<Utc> {
                let naive_datetime =
                    NaiveDateTime::parse_from_str(date_time_str, "%Y%m%d%H%M%S").unwrap();
                Utc.from_utc_datetime(&naive_datetime)
            }
        }

        if record.len() != 61 {
            return Err(anyhow::anyhow!("Expected 61 fields, got {}", record.len()));
        }

        let fields: [&str; 61] = record
            .iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert record to fixed-size array"))?;

        Ok(EventTable {
            global_event_id: GlobalEventID::try_from(Some(fields[0]))?,
            actor_1: Actor::try_from(&fields[5..15]).ok(),
            actor_2: Actor::try_from(&fields[15..25]).ok(),
            event_action: EventAction::try_from(&fields[25..35])?,
            actor_1_geograpy: EventGeography::try_from(&fields[35..43]).ok(),
            actor_2_geography: EventGeography::try_from(&fields[43..51]).ok(),
            action_geography: EventGeography::try_from(&fields[51..59]).ok(),
            date_added: conversion::convert_to_utc_datetime(fields[59]),
            src_url: Url::parse(fields[60]).unwrap(),
        })
    }
}

pub mod actor {
    use std::iter::Take;

    use anyhow::{anyhow, ensure};
    use csv::StringRecordIter;

    use crate::types::lookup_types::{
        country::CountryZone, ethnicity::Ethnicity, known_group::KnownGroup, religion::Religion,
        role::ActorRole,
    };

    #[derive(Debug)]
    pub struct CAMEOActorCode(pub [[u8; 3]; 5]);
    #[derive(Debug)]
    pub struct ActorName(pub String);
    #[derive(Debug)]
    pub struct CAMEOCountryCode(pub [u8; 3]);
    #[derive(Debug)]
    pub struct CAMEOKnownGroupCode(pub [u8; 3]);
    #[derive(Debug)]
    pub struct CAMEOEthnicityCode(pub [u8; 3]);
    #[derive(Debug)]
    pub struct CAMEOReligionCode(pub [u8; 3]);
    #[derive(Debug)]
    pub struct CAMEORoleCode(pub [u8; 3]);

    // Implement for CAMEOActorCode
    impl TryFrom<Option<&str>> for CAMEOActorCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 15 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOActorCode must be 15 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let mut codes = [[0u8; 3]; 5];
                    for i in 0..5 {
                        let start = i * 3;
                        let end = start + 3;
                        codes[i] = s[start..end].as_bytes().try_into().map_err(|_| {
                            anyhow::anyhow!(format!(
                                "Invalid CAMEOActorCode segment for index {}",
                                i
                            ))
                        })?;
                    }
                    Ok(CAMEOActorCode(codes))
                }
                None => Err(anyhow::anyhow!("CAMEOActorCode cannot be None")),
            }
        }
    }

    // Implement for ActorName
    impl TryFrom<Option<&str>> for ActorName {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => Ok(ActorName(s.to_string())),
                None => Err(anyhow::anyhow!("ActorName cannot be None")),
            }
        }
    }

    // Implement for CAMEOCountryCode
    impl TryFrom<Option<&str>> for CAMEOCountryCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let bytes = s.as_bytes();
                    if bytes.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOCountryCode must be 3 bytes long, got {}",
                            bytes.len()
                        )));
                    }
                    Ok(CAMEOCountryCode(bytes[..3].try_into().unwrap()))
                }
                None => Err(anyhow::anyhow!("CAMEOCountryCode cannot be None")),
            }
        }
    }

    // Implement for CAMEOKnownGroupCode
    impl TryFrom<Option<&str>> for CAMEOKnownGroupCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let bytes = s.as_bytes();
                    if bytes.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOKnownGroupCode must be 3 bytes long, got {}",
                            bytes.len()
                        )));
                    }
                    Ok(CAMEOKnownGroupCode(bytes[..3].try_into().unwrap()))
                }
                None => Err(anyhow::anyhow!("CAMEOKnownGroupCode cannot be None")),
            }
        }
    }

    // Implement for CAMEOEthnicityCode
    impl TryFrom<Option<&str>> for CAMEOEthnicityCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let bytes = s.as_bytes();
                    if bytes.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOEthnicityCode must be 3 bytes long, got {}",
                            bytes.len()
                        )));
                    }
                    Ok(CAMEOEthnicityCode(bytes[..3].try_into().unwrap()))
                }
                None => Err(anyhow::anyhow!("CAMEOEthnicityCode cannot be None")),
            }
        }
    }

    // Implement for CAMEOReligionCode
    impl TryFrom<Option<&str>> for CAMEOReligionCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let bytes = s.as_bytes();
                    if bytes.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOReligionCode must be 3 bytes long, got {}",
                            bytes.len()
                        )));
                    }
                    Ok(CAMEOReligionCode(bytes[..3].try_into().unwrap()))
                }
                None => Err(anyhow::anyhow!("CAMEOReligionCode cannot be None")),
            }
        }
    }

    // Implement for CAMEORoleCode
    impl TryFrom<Option<&str>> for CAMEORoleCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let bytes = s.as_bytes();
                    if bytes.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEORoleCode must be 3 bytes long, got {}",
                            bytes.len()
                        )));
                    }
                    Ok(CAMEORoleCode(bytes[..3].try_into().unwrap()))
                }
                None => Err(anyhow::anyhow!("CAMEORoleCode cannot be None")),
            }
        }
    }

    #[derive(Debug)]
    pub struct Actor {
        pub name: Option<ActorName>,
        pub country: Option<CountryZone>,
        pub known_group: Option<KnownGroup>,
        pub ethnicity: Option<Ethnicity>,
        pub religion: (Option<Religion>, Option<Religion>),
        pub actor_type: (Option<ActorRole>, Option<ActorRole>, Option<ActorRole>),
    }

    impl TryFrom<&[&str]> for Actor {
        type Error = anyhow::Error;

        fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
            if fields.len() != 10 {
                return Err(anyhow::anyhow!(format!(
                    "Expected 10 fields for Actor, got {}",
                    fields.len()
                )));
            }

            let code = fields[0];
            ensure!(code.ne(""), "No actor ascertained from the CAMEO Code");

            Ok(Self {
                name: ActorName::try_from(Some(fields[1])).ok(),
                country: CountryZone::try_from(CAMEOCountryCode::try_from(Some(fields[2])).ok())
                    .ok(),
                known_group: KnownGroup::try_from(
                    CAMEOKnownGroupCode::try_from(Some(fields[3])).ok(),
                )
                .ok(),
                ethnicity: Ethnicity::try_from(CAMEOEthnicityCode::try_from(Some(fields[4])).ok())
                    .ok(),
                religion: (
                    Religion::try_from(CAMEOReligionCode::try_from(Some(fields[5])).ok()).ok(),
                    Religion::try_from(CAMEOReligionCode::try_from(Some(fields[6])).ok()).ok(),
                ),
                actor_type: (
                    ActorRole::try_from(CAMEORoleCode::try_from(Some(fields[7])).ok()).ok(),
                    ActorRole::try_from(CAMEORoleCode::try_from(Some(fields[8])).ok()).ok(),
                    ActorRole::try_from(CAMEORoleCode::try_from(Some(fields[9])).ok()).ok(),
                ),
            })
        }
    }
}

pub mod event_action {
    use std::iter::Take;

    use csv::StringRecordIter;

    use crate::types::lookup_types::{
        event_action_description::top_level_actions::EventActionDescription, quad_class::QuadClass,
    };

    #[derive(Debug)]
    pub struct IsRootEvent(pub bool);
    #[derive(Debug)]
    pub struct CAMEOEventCode(pub [u8; 4]);
    #[derive(Debug)]
    pub struct CAMEOEventBaseCode(pub [u8; 3]);
    #[derive(Debug)]
    pub struct CAMEOEventRootCode(pub [u8; 2]);
    #[derive(Debug)]
    pub struct QuadClassCode(pub u8);
    #[derive(Debug)]
    pub struct GoldsteinScale(pub f32);
    #[derive(Debug)]
    pub struct NumberOfMentions(pub u64);
    #[derive(Debug)]
    pub struct NumberOfSources(pub u64);
    #[derive(Debug)]
    pub struct NumberOfArticles(pub u64);
    #[derive(Debug)]
    pub struct Tone(pub f64);

    #[derive(Debug)]
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

    impl TryFrom<&[&str]> for EventAction {
        type Error = anyhow::Error;

        fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
            if fields.len() != 10 {
                return Err(anyhow::anyhow!(format!(
                    "Expected 10 fields for EventAction, got {}",
                    fields.len()
                )));
            }

            Ok(Self {
                is_root_event: IsRootEvent::try_from(Some(fields[0]))?,
                event_action: EventActionDescription::try_from(
                    CAMEOEventCode::try_from(Some(fields[1])).ok(),
                )?,
                quad_class: QuadClass::try_from(QuadClassCode::try_from(Some(fields[4])).ok())?,
                goldstein_scale: GoldsteinScale::try_from(Some(fields[5]))?,
                number_of_mentions: NumberOfMentions::try_from(Some(fields[6]))?,
                number_of_sources: NumberOfSources::try_from(Some(fields[7]))?,
                number_of_articles: NumberOfArticles::try_from(Some(fields[8]))?,
                average_tone: Tone::try_from(Some(fields[9]))?,
            })
        }
    }
    // Implement for IsRootEvent
    impl TryFrom<Option<&str>> for IsRootEvent {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => match s {
                    "1" => Ok(IsRootEvent(true)),
                    "0" => Ok(IsRootEvent(false)),
                    _ => Err(anyhow::anyhow!(format!("Invalid IsRootEvent value: {}", s))),
                },
                None => Err(anyhow::anyhow!("IsRootEvent cannot be None".to_string())),
            }
        }
    }

    impl TryFrom<Option<&str>> for CAMEOEventCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let mut bytes = s.as_bytes().to_vec();
                    if bytes.len() > 4 || bytes.len() < 2 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOEventCode must be no more than 4 bytes long, or more than 2, got {}",
                            s.len()
                        )));
                    }
                    while bytes.len() < 4 {
                        bytes.push(0);
                    }
                    Ok(CAMEOEventCode([bytes[0], bytes[1], bytes[2], bytes[3]]))
                }
                None => Err(anyhow::anyhow!("CAMEOEventCode cannot be None".to_string())),
            }
        }
    }

    // Implement for CAMEOEventBaseCode
    impl TryFrom<Option<&str>> for CAMEOEventBaseCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 3 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOEventBaseCode must be 3 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let bytes = s.as_bytes();
                    Ok(CAMEOEventBaseCode([bytes[0], bytes[1], bytes[2]]))
                }
                None => Err(anyhow::anyhow!(
                    "CAMEOEventBaseCode cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for CAMEOEventRootCode
    impl TryFrom<Option<&str>> for CAMEOEventRootCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 2 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOEventRootCode must be 2 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let bytes = s.as_bytes();
                    Ok(CAMEOEventRootCode([bytes[0], bytes[1]]))
                }
                None => Err(anyhow::anyhow!(
                    "CAMEOEventRootCode cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for QuadClassCode
    impl TryFrom<Option<&str>> for QuadClassCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let code = s.parse::<u8>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid QuadClassCode value: {}", e))
                    })?;
                    Ok(QuadClassCode(code))
                }
                None => Err(anyhow::anyhow!("QuadClassCode cannot be None".to_string())),
            }
        }
    }

    // Implement for GoldsteinScale
    impl TryFrom<Option<&str>> for GoldsteinScale {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let scale = s.parse::<f32>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid GoldsteinScale value: {}", e))
                    })?;
                    Ok(GoldsteinScale(scale))
                }
                None => Err(anyhow::anyhow!("GoldsteinScale cannot be None".to_string())),
            }
        }
    }

    // Implement for NumberOfMentions
    impl TryFrom<Option<&str>> for NumberOfMentions {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let count = s.parse::<u64>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid NumberOfMentions value: {}", e))
                    })?;
                    Ok(NumberOfMentions(count))
                }
                None => Err(anyhow::anyhow!(
                    "NumberOfMentions cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for NumberOfSources
    impl TryFrom<Option<&str>> for NumberOfSources {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let count = s.parse::<u64>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid NumberOfSources value: {}", e))
                    })?;
                    Ok(NumberOfSources(count))
                }
                None => Err(anyhow::anyhow!(
                    "NumberOfSources cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for NumberOfArticles
    impl TryFrom<Option<&str>> for NumberOfArticles {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let count = s.parse::<u64>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid NumberOfArticles value: {}", e))
                    })?;
                    Ok(NumberOfArticles(count))
                }
                None => Err(anyhow::anyhow!(
                    "NumberOfArticles cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for Tone
    impl TryFrom<Option<&str>> for Tone {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let tone = s
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!(format!("Invalid Tone value: {}", e)))?;
                    Ok(Tone(tone))
                }
                None => Err(anyhow::anyhow!("Tone cannot be None".to_string())),
            }
        }
    }
}

pub mod event_geography {
    use csv::StringRecordIter;
    use log::debug;

    use crate::types::{
        event_table::actor::CAMEOCountryCode,
        lookup_types::{country::CountryZone, geography_type::GeographyType},
    };

    #[derive(Debug)]
    pub struct GeographyTypeCode(pub u8);
    #[derive(Debug)]
    pub struct GeographyFullName(pub String);
    #[derive(Debug)]
    pub struct FIPSCountryCode(pub [u8; 2]);
    #[derive(Debug)]
    pub struct FIPSAdministrationCode(pub [u8; 2]);
    #[derive(Debug)]
    pub struct Administration2Code(pub String);
    #[derive(Debug)]
    pub struct Latitude(pub f64);
    #[derive(Debug)]
    pub struct Longitude(pub f64);
    #[derive(Debug)]
    pub struct FeatureID(pub String);

    #[derive(Debug)]
    pub struct EventGeography {
        pub geography_type: Option<GeographyType>,
        pub geograpgy_fullname: Option<GeographyFullName>,
        pub geography_country: Option<CountryZone>,
        pub coordinates: Option<(Latitude, Longitude)>,
        pub feature_id: Option<FeatureID>,
    }

    use std::{convert::TryFrom, iter::Take};

    // Implement for GeographyTypeCode
    impl TryFrom<Option<&str>> for GeographyTypeCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let code = s.parse::<u8>().map_err(|e| {
                        anyhow::anyhow!(format!("Invalid GeographyTypeCode value: {}", e))
                    })?;
                    Ok(GeographyTypeCode(code))
                }
                None => Err(anyhow::anyhow!(
                    "GeographyTypeCode cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for GeographyFullName
    impl TryFrom<Option<&str>> for GeographyFullName {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => Ok(GeographyFullName(s.to_string())),
                None => Err(anyhow::anyhow!(
                    "GeographyFullName cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for FIPSCountryCode
    impl TryFrom<Option<&str>> for FIPSCountryCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 2 {
                        return Err(anyhow::anyhow!(format!(
                            "FIPSCountryCode must be 2 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let bytes = s.as_bytes();
                    Ok(FIPSCountryCode([bytes[0], bytes[1]]))
                }
                None => Err(anyhow::anyhow!(
                    "FIPSCountryCode cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for FIPSAdministrationCode
    impl TryFrom<Option<&str>> for FIPSAdministrationCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 2 {
                        return Err(anyhow::anyhow!(format!(
                            "FIPSAdministrationCode must be 2 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let bytes = s.as_bytes();
                    Ok(FIPSAdministrationCode([bytes[0], bytes[1]]))
                }
                None => Err(anyhow::anyhow!(
                    "FIPSAdministrationCode cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for Administration2Code
    impl TryFrom<Option<&str>> for Administration2Code {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => Ok(Administration2Code(s.to_string())),
                None => Err(anyhow::anyhow!(
                    "Administration2Code cannot be None".to_string()
                )),
            }
        }
    }

    // Implement for Latitude
    impl TryFrom<Option<&str>> for Latitude {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let val = s
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!(format!("Invalid Latitude value: {}", e)))?;
                    Ok(Latitude(val))
                }
                None => Err(anyhow::anyhow!("Latitude cannot be None".to_string())),
            }
        }
    }

    // Implement for Longitude
    impl TryFrom<Option<&str>> for Longitude {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    let val = s
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!(format!("Invalid Longitude value: {}", e)))?;
                    Ok(Longitude(val))
                }
                None => Err(anyhow::anyhow!("Longitude cannot be None".to_string())),
            }
        }
    }

    // Implement for FeatureID
    impl TryFrom<Option<&str>> for FeatureID {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => Ok(FeatureID(s.to_string())),
                None => Err(anyhow::anyhow!("FeatureID cannot be None".to_string())),
            }
        }
    }

    impl TryFrom<&[&str]> for EventGeography {
        type Error = anyhow::Error;

        fn try_from(fields: &[&str]) -> Result<Self, Self::Error> {
            if fields.len() != 8 {
                return Err(anyhow::anyhow!(format!(
                    "Expected 10 fields for EventGeography, got {}",
                    fields.len()
                )));
            }

            Ok(Self {
                geography_type: GeographyType::try_from(
                    GeographyTypeCode::try_from(Some(fields[0])).ok(),
                )
                .ok(),
                geograpgy_fullname: GeographyFullName::try_from(Some(fields[1])).ok(),
                geography_country: CountryZone::try_from(
                    FIPSCountryCode::try_from(Some(fields[2])).ok(),
                )
                .ok(),
                coordinates: {
                    let lat = Latitude::try_from(Some(fields[5])); //TODO: Better checking of the bounds of the array (yes even though it is checked in the beginning of this function). Do this for all the array slice fetches so that they are not wrapped in Some, which is fucking stupid imo
                    let lon = Longitude::try_from(Some(fields[6]));
                    match (lat, lon) {
                        (Ok(lati), Ok(long)) => Some((lati, long)),
                        _ => None,
                    }
                },
                feature_id: FeatureID::try_from(Some(fields[7])).ok(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::ReaderBuilder;
    use log::{debug, info};

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            env_logger::init();
        });
    }
    fn get_sample_row() -> String {
        "1233702893	20240322	202403	2024	2024.2247	USAGOV	UNITED STATES	USA					GOV			USA	UNITED STATES	USA								0	050	050	05	1	3.5	2	1	2	-3.71155885471898	2	Washington, United States	US	USWA		47.3917	-121.571	WA	2	Washington, United States	US	USWA		47.3917	-121.571	WA	2	Washington, United States	US	USWA		47.3917	-121.571	WA	20250322180000	https://www.yakimaherald.com/news/northwest/wa-state-workers-slam-furloughs-other-pay-cut-plans-claiming-they-are-a-tax-on/article_e49c4f10-11a1-5b7a-b947-c49482ea1ae0.html
".to_string()
    }

    fn default_test_string() -> String {
        "\
    1\t2\t3\t4\t5\t6\t7\t8\t9\t10\t11\t12\t13\t14\t15\t16\t17\t18\t19\t20\t21\t22\t23\t24\t25\t26\t27\t28\t29\t30\t31\t32\t33\t34\t35\t36\t37\t38\t39\t40\t41\t42\t43\t44\t45\t46\t47\t48\t49\t50\t51\t52\t53\t54\t55\t56\t57\t58\t59\t60\t20250322180000\thttps://example.com/test-url"
            .to_string()
    }

    #[test]
    fn test_event_table_try_from() {
        init_logger();
        let data = get_sample_row();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(data.as_bytes());

        for result in rdr.records() {
            let record = result.expect("CSV record parse error");
            let event = EventTable::try_from(record);
            debug!("Tested Event Table TryFrom: {:?}", event);
            assert!(event.is_ok(), "Failed to parse EventTable: {:?}", event);
        }
    }

    #[test]
    fn test_event_table_fields() {
        init_logger();
        let data = get_sample_row();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(data.as_bytes());

        for result in rdr.records() {
            let record = result.expect("CSV record parse error");
            let event = EventTable::try_from(record).expect("Failed to parse EventTable");

            assert_eq!(event.global_event_id.0, 1233702893);
            // Example: check src_url
            assert_eq!(
                event.src_url.as_str(),
                "https://www.yakimaherald.com/news/northwest/wa-state-workers-slam-furloughs-other-pay-cut-plans-claiming-they-are-a-tax-on/article_e49c4f10-11a1-5b7a-b947-c49482ea1ae0.html"
            );
            // You can add more assertions for other fields as needed
        }
    }

    #[test]
    fn test_event_table_substructs() {
        init_logger();
        let data = get_sample_row();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(data.as_bytes());

        for result in rdr.records() {
            let record = result.expect("CSV record parse error");
            let event = EventTable::try_from(record).expect("Failed to parse EventTable");
            debug!("Tested Event Table Substructs: {:?}", event);

            // Example: check that actor_1 exists and has expected values
            assert!(event.actor_1.is_some());
            // Example: check that event_action fields are parsed
            assert!(event.event_action.goldstein_scale.0 == 3.5);
            // Example: check that action_geography exists
            assert!(event.action_geography.is_some());
            // Add more detailed checks as needed for your sub-structs
        }
    }
}
#[cfg(test)]
mod newtype_tests {
    use super::*;
    use crate::types::event_table::{actor::*, event_action::*, event_geography::*};

    // Top-level newtypes
    #[test]
    fn test_global_event_id() {
        assert_eq!(GlobalEventID::try_from(Some("123")).unwrap().0, 123);
        assert!(GlobalEventID::try_from(Some("abc")).is_err());
        assert!(GlobalEventID::try_from(None).is_err());
    }

    #[test]
    fn test_year_month_day() {
        assert_eq!(
            YearMonthDay::try_from(Some("20240101")).unwrap().0,
            20240101
        );
        assert!(YearMonthDay::try_from(Some("bad")).is_err());
        assert!(YearMonthDay::try_from(None).is_err());
    }

    #[test]
    fn test_year_month() {
        assert_eq!(YearMonth::try_from(Some("202401")).unwrap().0, 202401);
        assert!(YearMonth::try_from(Some("bad")).is_err());
        assert!(YearMonth::try_from(None).is_err());
    }

    #[test]
    fn test_year() {
        assert_eq!(Year::try_from(Some("2024")).unwrap().0, 2024);
        assert!(Year::try_from(Some("bad")).is_err());
        assert!(Year::try_from(None).is_err());
    }

    #[test]
    fn test_fraction_date() {
        assert_eq!(FractionDate::try_from(Some("2024.5")).unwrap().0, 2024.5);
        assert!(FractionDate::try_from(Some("bad")).is_err());
        assert!(FractionDate::try_from(None).is_err());
    }

    #[test]
    fn test_year_month_day_hour_minute_second() {
        assert_eq!(
            YearMonthDayHourMinuteSecond::try_from(Some("20240101120000"))
                .unwrap()
                .0,
            20240101120000
        );
        assert!(YearMonthDayHourMinuteSecond::try_from(Some("bad")).is_err());
        assert!(YearMonthDayHourMinuteSecond::try_from(None).is_err());
    }

    // actor module newtypes
    #[test]
    fn test_cameo_actor_code() {
        let valid = "ABCDEFGHIJKLMNO";
        assert!(CAMEOActorCode::try_from(Some(valid)).is_ok());
        assert!(CAMEOActorCode::try_from(Some("SHORT")).is_err());
        assert!(CAMEOActorCode::try_from(None).is_err());
    }

    #[test]
    fn test_actor_name() {
        assert_eq!(ActorName::try_from(Some("Alice")).unwrap().0, "Alice");
        assert!(ActorName::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_country_code() {
        assert!(CAMEOCountryCode::try_from(Some("USA")).is_ok());
        assert!(CAMEOCountryCode::try_from(Some("US")).is_err());
        assert!(CAMEOCountryCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_known_group_code() {
        assert!(CAMEOKnownGroupCode::try_from(Some("ABC")).is_ok());
        assert!(CAMEOKnownGroupCode::try_from(Some("AB")).is_err());
        assert!(CAMEOKnownGroupCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_ethnicity_code() {
        assert!(CAMEOEthnicityCode::try_from(Some("DEF")).is_ok());
        assert!(CAMEOEthnicityCode::try_from(Some("D")).is_err());
        assert!(CAMEOEthnicityCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_religion_code() {
        assert!(CAMEOReligionCode::try_from(Some("GHJ")).is_ok());
        assert!(CAMEOReligionCode::try_from(Some("G")).is_err());
        assert!(CAMEOReligionCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_role_code() {
        assert!(CAMEORoleCode::try_from(Some("XYZ")).is_ok());
        assert!(CAMEORoleCode::try_from(Some("X")).is_err());
        assert!(CAMEORoleCode::try_from(None).is_err());
    }

    // event_action module newtypes
    #[test]
    fn test_is_root_event() {
        assert!(IsRootEvent::try_from(Some("1")).unwrap().0);
        assert!(!IsRootEvent::try_from(Some("0")).unwrap().0);
        assert!(IsRootEvent::try_from(Some("2")).is_err());
        assert!(IsRootEvent::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_event_code() {
        assert!(CAMEOEventCode::try_from(Some("1234")).is_ok());
        assert!(CAMEOEventCode::try_from(Some("12345")).is_err());
        assert!(CAMEOEventCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_event_base_code() {
        assert!(CAMEOEventBaseCode::try_from(Some("123")).is_ok());
        assert!(CAMEOEventBaseCode::try_from(Some("12")).is_err());
        assert!(CAMEOEventBaseCode::try_from(None).is_err());
    }

    #[test]
    fn test_cameo_event_root_code() {
        assert!(CAMEOEventRootCode::try_from(Some("12")).is_ok());
        assert!(CAMEOEventRootCode::try_from(Some("1")).is_err());
        assert!(CAMEOEventRootCode::try_from(None).is_err());
    }

    #[test]
    fn test_quad_class_code() {
        assert_eq!(QuadClassCode::try_from(Some("5")).unwrap().0, 5);
        assert!(QuadClassCode::try_from(Some("bad")).is_err());
        assert!(QuadClassCode::try_from(None).is_err());
    }

    #[test]
    fn test_goldstein_scale() {
        assert_eq!(GoldsteinScale::try_from(Some("1.5")).unwrap().0, 1.5);
        assert!(GoldsteinScale::try_from(Some("bad")).is_err());
        assert!(GoldsteinScale::try_from(None).is_err());
    }

    #[test]
    fn test_number_of_mentions() {
        assert_eq!(NumberOfMentions::try_from(Some("10")).unwrap().0, 10);
        assert!(NumberOfMentions::try_from(Some("bad")).is_err());
        assert!(NumberOfMentions::try_from(None).is_err());
    }

    #[test]
    fn test_number_of_sources() {
        assert_eq!(NumberOfSources::try_from(Some("2")).unwrap().0, 2);
        assert!(NumberOfSources::try_from(Some("bad")).is_err());
        assert!(NumberOfSources::try_from(None).is_err());
    }

    #[test]
    fn test_number_of_articles() {
        assert_eq!(NumberOfArticles::try_from(Some("3")).unwrap().0, 3);
        assert!(NumberOfArticles::try_from(Some("bad")).is_err());
        assert!(NumberOfArticles::try_from(None).is_err());
    }

    #[test]
    fn test_tone() {
        assert_eq!(Tone::try_from(Some("0.5")).unwrap().0, 0.5);
        assert!(Tone::try_from(Some("bad")).is_err());
        assert!(Tone::try_from(None).is_err());
    }

    // event_geography module newtypes
    #[test]
    fn test_geography_type_code() {
        assert_eq!(GeographyTypeCode::try_from(Some("1")).unwrap().0, 1);
        assert!(GeographyTypeCode::try_from(Some("bad")).is_err());
        assert!(GeographyTypeCode::try_from(None).is_err());
    }

    #[test]
    fn test_geography_full_name() {
        assert_eq!(
            GeographyFullName::try_from(Some("Test Place")).unwrap().0,
            "Test Place"
        );
        assert!(GeographyFullName::try_from(None).is_err());
    }

    #[test]
    fn test_fips_country_code() {
        assert!(FIPSCountryCode::try_from(Some("US")).is_ok());
        assert!(FIPSCountryCode::try_from(Some("U")).is_err());
        assert!(FIPSCountryCode::try_from(None).is_err());
    }

    #[test]
    fn test_fips_administration_code() {
        assert!(FIPSAdministrationCode::try_from(Some("WA")).is_ok());
        assert!(FIPSAdministrationCode::try_from(Some("W")).is_err());
        assert!(FIPSAdministrationCode::try_from(None).is_err());
    }

    #[test]
    fn test_administration2_code() {
        assert_eq!(
            Administration2Code::try_from(Some("ADM2")).unwrap().0,
            "ADM2"
        );
        assert!(Administration2Code::try_from(None).is_err());
    }

    #[test]
    fn test_latitude() {
        assert_eq!(Latitude::try_from(Some("47.0")).unwrap().0, 47.0);
        assert!(Latitude::try_from(Some("bad")).is_err());
        assert!(Latitude::try_from(None).is_err());
    }

    #[test]
    fn test_longitude() {
        assert_eq!(Longitude::try_from(Some("-122.0")).unwrap().0, -122.0);
        assert!(Longitude::try_from(Some("bad")).is_err());
        assert!(Longitude::try_from(None).is_err());
    }

    #[test]
    fn test_feature_id() {
        assert_eq!(FeatureID::try_from(Some("FID123")).unwrap().0, "FID123");
        assert!(FeatureID::try_from(None).is_err());
    }
}
