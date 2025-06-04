use std::collections::hash_map::Iter;

use actor::Actor;
use chrono::{DateTime, Utc};
use event_action::EventAction;
use event_geography::EventGeography;
use url::Url;

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
        if record.len() < 61 {
            return Err(anyhow::anyhow!("Not enough fields"));
        }
        let mut iter_record = record.iter();

        Ok(EventTable {
            global_event_id: {
                let iter_record = &mut iter_record;
                GlobalEventID::try_from(iter_record.next())?
            },
            actor_1: {
                let iter_record = &mut iter_record;
                iter_record.skip(4);
                Actor::try_from(&mut iter_record.take(10)).ok()
            },
            actor_2: {
                let iter_record = &mut iter_record;
                Actor::try_from(&mut iter_record.take(10)).ok()
            },
            event_action: {
                let iter_record = &mut iter_record;
                EventAction::try_from(&mut iter_record.take(10))?
            },
            actor_1_geograpy: {
                let iter_record = &mut iter_record;
                EventGeography::try_from(&mut iter_record.take(8)).ok()
            },
            actor_2_geography: {
                let iter_record = &mut iter_record;
                EventGeography::try_from(&mut iter_record.take(8)).ok()
            },
            action_geography: {
                let iter_record = &mut iter_record;
                EventGeography::try_from(&mut iter_record.take(8)).ok()
            },
            date_added: {
                let iter_record = &mut iter_record;
                let field = iter_record.next();
                match field {
                    Some(s) => conversion::convert_to_utc_datetime(s),
                    None => Utc::now(),
                }
            },
            src_url: record[9].parse::<Url>()?,
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

    impl<'a, 'iter> TryFrom<&'iter mut Take<&'iter mut StringRecordIter<'a>>> for Actor {
        type Error = anyhow::Error;

        fn try_from(
            value: &'iter mut Take<&'iter mut StringRecordIter<'a>>,
        ) -> Result<Self, Self::Error> {
            assert_eq!(value.size_hint(), (10, Some(10)));
            let code = value
                .next()
                .ok_or(anyhow::anyhow!("Actor does not exist"))?;
            ensure!(code.ne(""), "No actor ascertained from the CAMEO Code");
            Ok(Self {
                name: ActorName::try_from(value.next()).ok(),
                country: CountryZone::try_from(CAMEOCountryCode::try_from(value.next())?).ok(),
                known_group: KnownGroup::try_from(CAMEOKnownGroupCode::try_from(value.next())?)
                    .ok(),
                ethnicity: Ethnicity::try_from(CAMEOEthnicityCode::try_from(value.next())?).ok(),
                religion: (
                    Religion::try_from(CAMEOReligionCode::try_from(value.next())?).ok(),
                    Religion::try_from(CAMEOReligionCode::try_from(value.next())?).ok(),
                ),
                actor_type: (
                    ActorRole::try_from(CAMEORoleCode::try_from(value.next())?).ok(),
                    ActorRole::try_from(CAMEORoleCode::try_from(value.next())?).ok(),
                    ActorRole::try_from(CAMEORoleCode::try_from(value.next())?).ok(),
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

    impl<'a, 'iter> TryFrom<&'iter mut Take<&'iter mut StringRecordIter<'a>>> for EventAction {
        type Error = anyhow::Error;

        fn try_from(
            value: &'iter mut Take<&'iter mut StringRecordIter<'a>>,
        ) -> Result<Self, Self::Error> {
            assert_eq!(value.size_hint(), (10, Some(10)));
            let mut value = value;
            Ok(Self {
                is_root_event: IsRootEvent::try_from(value.next())?,
                event_action: EventActionDescription::try_from(CAMEOEventCode::try_from(
                    value.next(),
                )?)?,
                quad_class: QuadClass::try_from(QuadClassCode::try_from(value.next())?)?,
                goldstein_scale: GoldsteinScale::try_from(value.next())?,
                number_of_mentions: NumberOfMentions::try_from(value.next())?,
                number_of_sources: NumberOfSources::try_from(value.next())?,
                number_of_articles: NumberOfArticles::try_from(value.next())?,
                average_tone: Tone::try_from(value.next())?,
            })
        }
    }
    // Implement for IsRootEvent
    impl TryFrom<Option<&str>> for IsRootEvent {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => match s {
                    "true" => Ok(IsRootEvent(true)),
                    "false" => Ok(IsRootEvent(false)),
                    _ => Err(anyhow::anyhow!(format!("Invalid IsRootEvent value: {}", s))),
                },
                None => Err(anyhow::anyhow!("IsRootEvent cannot be None".to_string())),
            }
        }
    }

    // Implement for CAMEOEventCode
    impl TryFrom<Option<&str>> for CAMEOEventCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
            match value {
                Some(s) => {
                    if s.len() != 4 {
                        return Err(anyhow::anyhow!(format!(
                            "CAMEOEventCode must be 4 bytes long, got {}",
                            s.len()
                        )));
                    }
                    let bytes = s.as_bytes();
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
        pub geography_type: GeographyType,
        pub geograpgy_fullname: GeographyFullName,
        pub geography_country: CountryZone,
        pub coordinates: (Latitude, Longitude),
        pub feature_id: FeatureID,
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

    impl<'a, 'iter> TryFrom<&'iter mut Take<&'iter mut StringRecordIter<'a>>> for EventGeography {
        type Error = anyhow::Error;

        fn try_from(
            value: &'iter mut Take<&'iter mut StringRecordIter<'a>>,
        ) -> Result<Self, Self::Error> {
            assert_eq!(value.size_hint(), (10, Some(10)));
            Ok(Self {
                geography_type: GeographyType::try_from(GeographyTypeCode::try_from(
                    value.next(),
                )?)?,
                geograpgy_fullname: GeographyFullName::try_from(value.next())?,
                geography_country: CountryZone::try_from(CAMEOCountryCode::try_from(
                    value.next(),
                )?)?,
                coordinates: {
                    let _ = value.skip(1);
                    let lat = value.next();
                    let lon = value.next();
                    (
                        Latitude::try_from(lat).expect("Latitude Parse Error"),
                        Longitude::try_from(lon).expect("Longitude Parse Error"),
                    )
                },
                feature_id: FeatureID::try_from(value.next())?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::ReaderBuilder;

    fn get_sample_row() -> String {
        "\
1233702880\t20240322\t202403\t2024\t2024.2247\t\t\t\t\tUSA\tIDAHO\tUSA\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t1\t080\t080\t08\t2\t5.0\t2\t1\t2\t0\t0\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t3\tEnglish Point, Idaho, United States\tUS\tUSID\t\t47.7866\t-116.71\t394276\t3\tEnglish Point, Idaho, United States\tUS\tUSID\t\t47.7866\t-116.71\t394276\t20250322180000\thttps://bonnercountydailybee.com/news/2025/mar/22/ita-seeks-volunteers-for-panhandle-projects/"
            .to_string()
    }

    #[test]
    fn test_event_table_try_from() {
        let data = get_sample_row();
        let mut rdr = ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(data.as_bytes());

        for result in rdr.records() {
            let record = result.expect("CSV record parse error");
            let event = EventTable::try_from(record);
            assert!(event.is_ok(), "Failed to parse EventTable: {:?}", event);
        }
    }

#[test]
fn test_event_table_fields() {
    let data = get_sample_row();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(data.as_bytes());

    for result in rdr.records() {
        let record = result.expect("CSV record parse error");
        let event = EventTable::try_from(record).expect("Failed to parse EventTable");

        assert_eq!(event.global_event_id.0, 1233702880);
        // Example: check src_url
        assert_eq!(
            event.src_url.as_str(),
            "https://bonnercountydailybee.com/news/2025/mar/22/ita-seeks-volunteers-for-panhandle-projects/"
        );
        // You can add more assertions for other fields as needed
    }
}

#[test]
fn test_event_table_substructs() {
    let data = get_sample_row();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(data.as_bytes());

    for result in rdr.records() {
        let record = result.expect("CSV record parse error");
        let event = EventTable::try_from(record).expect("Failed to parse EventTable");

        // Example: check that actor_1 exists and has expected values
        assert!(event.actor_1.is_some());
        // Example: check that event_action fields are parsed
        assert!(event.event_action.goldstein_scale.0 == 5.0);
        // Example: check that action_geography exists
        assert!(event.action_geography.is_some());
        // Add more detailed checks as needed for your sub-structs
    }
}
}
