use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Utc};
use csv::StringRecord;
use std::convert::TryFrom;

use url::Url;

use crate::types::lookup_types::country::CountryZone;
use crate::types::lookup_types::ethnicity::Ethnicity;
use crate::types::lookup_types::event_action_description::top_level_actions::EventActionDescription;
use crate::types::lookup_types::geography_type::GeographyType;
use crate::types::lookup_types::known_group::KnownGroup;
use crate::types::lookup_types::quad_class::{Manner, QuadClass};
use crate::types::lookup_types::religion::Religion;
use crate::types::lookup_types::role::ActorRole;

// Core event identification
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalEventID(pub u64);

impl TryFrom<Option<&str>> for GlobalEventID {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) if !s.trim().is_empty() => Ok(GlobalEventID(s.trim().parse()?)),
            _ => Err(anyhow::anyhow!("GlobalEventID cannot be empty")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EventDate {
    pub date: NaiveDate,
}

impl EventDate {
    /// Get date in YYYYMMDD format
    pub fn day_format(&self) -> u32 {
        self.date.format("%Y%m%d").to_string().parse().unwrap_or(0)
    }

    /// Get date in YYYYMM format
    pub fn month_year_format(&self) -> u32 {
        self.date.format("%Y%m").to_string().parse().unwrap_or(0)
    }

    /// Get date in YYYY format
    pub fn year_format(&self) -> u16 {
        self.date.year() as u16
    }

    /// Get fractional date as YYYY.FFFF
    pub fn fraction_date(&self) -> f64 {
        let year = self.date.year() as f64;
        let day_of_year = self.date.ordinal() as f64;
        year + (day_of_year / 365.0)
    }
}

// CAMEO codes as rich types
#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOCountryCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOKnownGroupCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOEthnicCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOReligionCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct CAMEOTypeCode(pub String);

// Actor representation with all CAMEO attributes
#[derive(Debug, Clone, PartialEq)]
pub struct Actor {
    pub code: Option<CAMEOCode>,
    pub name: Option<String>,
    pub country_code: Option<CountryZone>,
    pub known_group_code: Option<KnownGroup>,
    pub ethnic_code: Option<Ethnicity>,
    pub religion1_code: Option<Religion>,
    pub religion2_code: Option<Religion>,
    pub type1_code: Option<ActorRole>,
    pub type2_code: Option<ActorRole>,
    pub type3_code: Option<ActorRole>,
}

// Event Action codes and metrics
#[derive(Debug, Clone, PartialEq)]
pub struct EventCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct EventBaseCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct EventRootCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct GoldsteinScale(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct NumMentions(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct NumSources(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct NumArticles(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct AvgTone(pub f64);

#[derive(Debug, Clone, PartialEq)]
pub struct EventAction {
    pub is_root_event: bool,
    pub event_code: Option<EventActionDescription>,
    pub event_base_code: EventBaseCode,
    pub event_root_code: EventRootCode,
    pub quad_class: QuadClass,
    pub goldstein_scale: GoldsteinScale,
    pub num_mentions: NumMentions,
    pub num_sources: NumSources,
    pub num_articles: NumArticles,
    pub avg_tone: AvgTone,
}

// Geography types - now using GeographyType from lookup_types

#[derive(Debug, Clone, PartialEq)]
pub struct FIPSCountryCode(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ADM1Code(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ADM2Code(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FeatureID(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Geography {
    pub geo_type: Option<GeographyType>,
    pub fullname: Option<String>,
    pub country_code: Option<CountryZone>,
    pub adm1_code: Option<ADM1Code>,
    pub adm2_code: Option<ADM2Code>,
    pub coordinates: Option<Coordinates>,
    pub feature_id: Option<FeatureID>,
}

// Main EventTable structure - no duplicate data
#[derive(Debug, Clone, PartialEq)]
pub struct EventTable {
    // Core identification - single source of truth for date
    pub global_event_id: GlobalEventID,
    pub date: EventDate,

    // Actors with rich CAMEO information
    pub actor1: Option<Actor>,
    pub actor2: Option<Actor>,

    // Event action with all metrics
    pub event_action: EventAction,

    // Geography for all three dimensions
    pub actor1_geography: Option<Geography>,
    pub actor2_geography: Option<Geography>,
    pub action_geography: Option<Geography>,

    // Data management
    pub date_added: DateTime<Utc>,
    pub source_url: Url,
}

// Helper functions for parsing
fn parse_optional_string(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_optional_u32(s: &str) -> Option<u32> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn parse_optional_f64(s: &str) -> Option<f64> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn parse_coordinates(lat_str: &str, long_str: &str) -> Option<Coordinates> {
    let lat = parse_optional_f64(lat_str)?;
    let long = parse_optional_f64(long_str)?;
    Some(Coordinates {
        latitude: lat,
        longitude: long,
    })
}

fn parse_datetime_utc(date_str: &str) -> Result<DateTime<Utc>> {
    let trimmed = date_str.trim();
    if trimmed.len() != 14 {
        return Err(anyhow::anyhow!("Invalid datetime format: {}", trimmed));
    }

    let naive_datetime = NaiveDateTime::parse_from_str(trimmed, "%Y%m%d%H%M%S")?;
    Ok(Utc.from_utc_datetime(&naive_datetime))
}

// TryFrom implementations
impl TryFrom<&str> for EventDate {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        let date_str = s.trim();
        if date_str.len() != 8 {
            return Err(anyhow::anyhow!("Invalid date format: {}", date_str));
        }

        let year: i32 = date_str[0..4].parse()?;
        let month: u32 = date_str[4..6].parse()?;
        let day: u32 = date_str[6..8].parse()?;

        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{}-{}", year, month, day))?;

        Ok(EventDate { date })
    }
}

impl TryFrom<Option<&str>> for CAMEOCountryCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "CAMEOCountryCode must be exactly 3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOCountryCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOCountryCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOCountryCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for FIPSCountryCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.len() != 2 {
                    return Err(anyhow::anyhow!(
                        "FIPSCountryCode must be exactly 2 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "FIPSCountryCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(FIPSCountryCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("FIPSCountryCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for CAMEOEthnicCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("CAMEOEthnicCode cannot be empty"));
                }
                if trimmed.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "CAMEOEthnicCode must be exactly 3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOEthnicCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOEthnicCode(trimmed.to_lowercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOEthnicCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for CAMEOKnownGroupCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("CAMEOKnownGroupCode cannot be empty"));
                }
                if trimmed.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "CAMEOKnownGroupCode must be exactly 3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOKnownGroupCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOKnownGroupCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOKnownGroupCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for CAMEOReligionCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("CAMEOReligionCode cannot be empty"));
                }
                if trimmed.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "CAMEOReligionCode must be exactly 3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOReligionCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOReligionCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOReligionCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for CAMEOTypeCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("CAMEOTypeCode cannot be empty"));
                }
                if trimmed.len() != 3 {
                    return Err(anyhow::anyhow!(
                        "CAMEOTypeCode must be exactly 3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_alphabetic()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOTypeCode must contain only alphabetic characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOTypeCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOTypeCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for CAMEOCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("CAMEOCode cannot be empty"));
                }
                // CAMEO codes can be variable length concatenated codes
                if !trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
                    return Err(anyhow::anyhow!(
                        "CAMEOCode must contain only alphanumeric characters, got: '{}'",
                        trimmed
                    ));
                }
                Ok(CAMEOCode(trimmed.to_uppercase()))
            }
            None => Err(anyhow::anyhow!("CAMEOCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for EventCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("EventCode cannot be empty"));
                }
                // CAMEO event codes can be 2-4 characters and may be zero-padded
                if trimmed.len() < 2 || trimmed.len() > 4 {
                    return Err(anyhow::anyhow!(
                        "EventCode must be 2-4 characters, got: '{}'",
                        trimmed
                    ));
                }
                // Should contain only digits (can be zero-padded like "01", "025", "0251")
                if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    return Err(anyhow::anyhow!(
                        "EventCode must contain only digits, got: '{}'",
                        trimmed
                    ));
                }
                Ok(EventCode(trimmed.to_string()))
            }
            None => Err(anyhow::anyhow!("EventCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for EventBaseCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("EventBaseCode cannot be empty"));
                }
                // Event base codes are 2-3 characters
                if trimmed.len() < 2 || trimmed.len() > 3 {
                    return Err(anyhow::anyhow!(
                        "EventBaseCode must be 2-3 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    return Err(anyhow::anyhow!(
                        "EventBaseCode must contain only digits, got: '{}'",
                        trimmed
                    ));
                }
                Ok(EventBaseCode(trimmed.to_string()))
            }
            None => Err(anyhow::anyhow!("EventBaseCode cannot be None")),
        }
    }
}

impl TryFrom<Option<&str>> for EventRootCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self> {
        match value {
            Some(s) => {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    return Err(anyhow::anyhow!("EventRootCode cannot be empty"));
                }
                // Event root codes are 2 characters
                if trimmed.len() != 2 {
                    return Err(anyhow::anyhow!(
                        "EventRootCode must be exactly 2 characters, got: '{}'",
                        trimmed
                    ));
                }
                if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                    return Err(anyhow::anyhow!(
                        "EventRootCode must contain only digits, got: '{}'",
                        trimmed
                    ));
                }
                Ok(EventRootCode(trimmed.to_string()))
            }
            None => Err(anyhow::anyhow!("EventRootCode cannot be None")),
        }
    }
}

impl TryFrom<&[&str]> for Actor {
    type Error = anyhow::Error;

    fn try_from(fields: &[&str]) -> Result<Self> {
        if fields.len() < 10 {
            return Err(anyhow::anyhow!(
                "Expected at least 10 fields for Actor, got {}",
                fields.len()
            ));
        }

        // Parse country code and convert to CountryZone
        let country_code = match parse_optional_string(fields[2]).map(CAMEOCountryCode) {
            Some(code) => CountryZone::try_from(Some(code)).ok(),
            None => None,
        };

        // Parse known group code and convert to KnownGroup
        let known_group_code = match parse_optional_string(fields[3]).map(CAMEOKnownGroupCode) {
            Some(code) => KnownGroup::try_from(Some(code)).ok(),
            None => None,
        };

        // Parse ethnic code and convert to Ethnicity
        let ethnic_code = match parse_optional_string(fields[4]).map(CAMEOEthnicCode) {
            Some(code) => Ethnicity::try_from(Some(code)).ok(),
            None => None,
        };

        // Parse religion codes and convert to Religion
        let religion1_code = match parse_optional_string(fields[5]).map(CAMEOReligionCode) {
            Some(code) => Religion::try_from(Some(code)).ok(),
            None => None,
        };

        let religion2_code = match parse_optional_string(fields[6]).map(CAMEOReligionCode) {
            Some(code) => Religion::try_from(Some(code)).ok(),
            None => None,
        };

        // Parse type codes and convert to ActorRole
        let type1_code = match parse_optional_string(fields[7]).map(CAMEOTypeCode) {
            Some(code) => {
                use crate::types::event_table::actor::CAMEORoleCode;
                let role_code = CAMEORoleCode(code.0);
                ActorRole::try_from(Some(role_code)).ok()
            }
            None => None,
        };

        let type2_code = match parse_optional_string(fields[8]).map(CAMEOTypeCode) {
            Some(code) => {
                use crate::types::event_table::actor::CAMEORoleCode;
                let role_code = CAMEORoleCode(code.0);
                ActorRole::try_from(Some(role_code)).ok()
            }
            None => None,
        };

        let type3_code = match parse_optional_string(fields[9]).map(CAMEOTypeCode) {
            Some(code) => {
                use crate::types::event_table::actor::CAMEORoleCode;
                let role_code = CAMEORoleCode(code.0);
                ActorRole::try_from(Some(role_code)).ok()
            }
            None => None,
        };

        Ok(Actor {
            code: parse_optional_string(fields[0]).map(CAMEOCode),
            name: parse_optional_string(fields[1]),
            country_code,
            known_group_code,
            ethnic_code,
            religion1_code,
            religion2_code,
            type1_code,
            type2_code,
            type3_code,
        })
    }
}

impl TryFrom<&[&str]> for EventAction {
    type Error = anyhow::Error;

    fn try_from(fields: &[&str]) -> Result<Self> {
        if fields.len() < 10 {
            return Err(anyhow::anyhow!(
                "Expected at least 10 fields for EventAction, got {}",
                fields.len()
            ));
        }

        let is_root_event = fields[0].trim() == "1";

        let quad_class_num: u8 = fields[4]
            .trim()
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid QuadClass: {}", fields[4]))?;

        let quad_class = match quad_class_num {
            1 => QuadClass::Cooperation(Manner::Verbal),
            2 => QuadClass::Cooperation(Manner::Material),
            3 => QuadClass::Conflict(Manner::Verbal),
            4 => QuadClass::Conflict(Manner::Material),
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid QuadClass value: {}",
                    quad_class_num
                ));
            }
        };

        // Parse event code and convert to EventActionDescription
        let event_code = {
            use crate::types::event_table::event_action::CAMEOEventCode;
            let field_value = if fields[1].trim().is_empty() {
                None
            } else {
                Some(fields[1])
            };
            match CAMEOEventCode::try_from(field_value) {
                Ok(code) => EventActionDescription::try_from(Some(code)).ok(),
                Err(_) => None,
            }
        };

        Ok(EventAction {
            is_root_event,
            event_code,
            event_base_code: EventBaseCode(fields[2].trim().to_string()),
            event_root_code: EventRootCode(fields[3].trim().to_string()),
            quad_class,
            goldstein_scale: GoldsteinScale(fields[5].trim().parse()?),
            num_mentions: NumMentions(fields[6].trim().parse()?),
            num_sources: NumSources(fields[7].trim().parse()?),
            num_articles: NumArticles(fields[8].trim().parse()?),
            avg_tone: AvgTone(fields[9].trim().parse()?),
        })
    }
}

impl TryFrom<&[&str]> for Geography {
    type Error = anyhow::Error;

    fn try_from(fields: &[&str]) -> Result<Self> {
        if fields.len() < 8 {
            return Err(anyhow::anyhow!(
                "Expected at least 8 fields for Geography, got {}",
                fields.len()
            ));
        }

        // Parse geography type and convert to GeographyType
        let geo_type = match fields[0].trim().parse::<u8>() {
            Ok(type_code) => {
                use crate::types::event_table::event_geography::GeographyTypeCode;
                match GeographyTypeCode::try_from(Some(type_code.to_string().as_str())) {
                    Ok(code) => GeographyType::try_from(Some(code)).ok(),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        };

        // Parse FIPS country code and convert to CountryZone
        let country_code = match parse_optional_string(fields[2]).map(FIPSCountryCode) {
            Some(code) => CountryZone::try_from(Some(code)).ok(),
            None => None,
        };

        Ok(Geography {
            geo_type,
            fullname: parse_optional_string(fields[1]),
            country_code,
            adm1_code: parse_optional_string(fields[3]).map(ADM1Code),
            adm2_code: parse_optional_string(fields[4]).map(ADM2Code),
            coordinates: parse_coordinates(fields[5], fields[6]),
            feature_id: parse_optional_string(fields[7]).map(FeatureID),
        })
    }
}

impl TryFrom<StringRecord> for EventTable {
    type Error = anyhow::Error;

    fn try_from(record: StringRecord) -> Result<Self> {
        let fields: Vec<&str> = record.iter().collect();

        // GDELT Event table should have exactly 61 fields according to codebook
        if fields.len() != 61 {
            return Err(anyhow::anyhow!(
                "Expected 61 fields for EventTable, got {}",
                fields.len()
            ));
        }

        Ok(EventTable {
            // Core identification (field 0)
            global_event_id: GlobalEventID(fields[0].trim().parse()?),

            // Single date representation (field 1) - other formats computed on demand
            date: EventDate::try_from(fields[1])?,

            // Actor1 (fields 5-14) - may be None if no actor identified
            actor1: if fields[5].trim().is_empty() {
                None
            } else {
                Some(Actor::try_from(&fields[5..15])?)
            },

            // Actor2 (fields 15-24) - may be None if no actor identified
            actor2: if fields[15].trim().is_empty() {
                None
            } else {
                Some(Actor::try_from(&fields[15..25])?)
            },

            // Event action (fields 25-34)
            event_action: EventAction::try_from(&fields[25..35])?,

            // Actor1 geography (fields 35-42) - may be None if no location identified
            actor1_geography: if fields[35].trim().is_empty() {
                None
            } else {
                Some(Geography::try_from(&fields[35..43])?)
            },

            // Actor2 geography (fields 43-50) - may be None if no location identified
            actor2_geography: if fields[43].trim().is_empty() {
                None
            } else {
                Some(Geography::try_from(&fields[43..51])?)
            },

            // Action geography (fields 51-58) - may be None if no location identified
            action_geography: if fields[51].trim().is_empty() {
                None
            } else {
                Some(Geography::try_from(&fields[51..59])?)
            },

            // Data management fields (fields 59-60)
            date_added: parse_datetime_utc(fields[59])?,
            source_url: Url::parse(fields[60].trim())
                .unwrap_or_else(|_| Url::parse("https://example.com").unwrap()),
        })
    }
}

impl crate::types::DatabaseTable for EventTable {}

#[cfg(test)]
mod tests {
    use super::{
        Actor, ActorRole, CountryZone, EventAction, EventDate, Geography, GeographyType, Manner,
        QuadClass,
    };
    use crate::types::lookup_types::religion::Religion;

    #[test]
    fn test_event_date_formats() {
        let date = EventDate::try_from("20250322").unwrap();

        // Test all format conversions
        assert_eq!(date.day_format(), 20250322);
        assert_eq!(date.month_year_format(), 202503);
        assert_eq!(date.year_format(), 2025);

        // Test fractional date (approximately)
        let fraction = date.fraction_date();
        assert!(fraction >= 2025.0 && fraction < 2026.0);
    }

    #[test]
    fn test_actor_parsing() {
        let fields = vec![
            "USAGOV",
            "UNITED STATES",
            "USA",
            "",
            "",
            "",
            "",
            "GOV",
            "",
            "",
        ];
        let actor = Actor::try_from(fields.as_slice()).unwrap();

        assert_eq!(actor.code.as_ref().unwrap().0, "USAGOV");
        assert_eq!(actor.name.as_ref().unwrap(), "UNITED STATES");
        assert_eq!(
            actor.country_code.as_ref().unwrap(),
            &CountryZone::UnitedStates
        );
        assert!(actor.known_group_code.is_none());
        assert_eq!(actor.type1_code.as_ref().unwrap(), &ActorRole::Government);
    }

    #[test]
    fn test_geography_parsing() {
        let fields = vec![
            "3",
            "Washington, DC, United States",
            "US",
            "USDC",
            "",
            "38.9072",
            "-77.0369",
            "531871",
        ];
        let geo = Geography::try_from(fields.as_slice()).unwrap();

        assert!(matches!(geo.geo_type, Some(GeographyType::City)));
        assert_eq!(
            geo.fullname.as_ref().unwrap(),
            "Washington, DC, United States"
        );
        assert_eq!(
            geo.country_code.as_ref().unwrap(),
            &CountryZone::UnitedStates
        );
        assert!(geo.coordinates.is_some());
        let coords = geo.coordinates.as_ref().unwrap();
        assert_eq!(coords.latitude, 38.9072);
        assert_eq!(coords.longitude, -77.0369);
    }

    #[test]
    fn test_event_action_quad_class() {
        let fields = vec!["1", "010", "01", "01", "1", "1.0", "5", "3", "8", "2.5"];
        let action = EventAction::try_from(fields.as_slice()).unwrap();

        assert!(action.is_root_event);
        assert!(action.event_code.is_some());
        assert!(matches!(
            action.quad_class,
            QuadClass::Cooperation(Manner::Verbal)
        ));
        assert_eq!(action.goldstein_scale.0, 1.0);
        assert_eq!(action.avg_tone.0, 2.5);
    }

    #[test]
    fn test_actor_enum_parsing() {
        // Test that codes are converted to proper enums instead of unit wrappers
        let fields = vec![
            "USAGOV",
            "UNITED STATES",
            "USA", // Should convert to CountryZone::UnitedStates
            "",
            "",
            "CHR", // Should convert to Religion::Christianity
            "",
            "GOV", // Should convert to ActorRole::Government
            "",
            "",
        ];
        let actor = Actor::try_from(fields.as_slice()).unwrap();

        // Verify enums are properly parsed
        assert!(matches!(
            actor.country_code,
            Some(CountryZone::UnitedStates)
        ));
        assert!(matches!(actor.religion1_code, Some(Religion::Christianity)));
        assert!(matches!(actor.type1_code, Some(ActorRole::Government)));

        // Verify None values for empty fields
        assert!(actor.known_group_code.is_none());
        assert!(actor.ethnic_code.is_none());
        assert!(actor.religion2_code.is_none());
        assert!(actor.type2_code.is_none());
        assert!(actor.type3_code.is_none());
    }
}

// Backward compatibility modules for other files that import from these paths
pub mod actor {
    use super::*;

    // Re-export types for backward compatibility
    pub type ActorName = String;
    pub use super::CAMEOCountryCode;
    pub use super::CAMEOEthnicCode as CAMEOEthnicityCode;
    pub use super::CAMEOKnownGroupCode;
    pub use super::CAMEOReligionCode;
    pub use super::CAMEOTypeCode as CAMEORoleCode;

    // Re-export the main Actor type
    pub use super::Actor;
}

pub mod event_action {
    use super::*;

    // CAMEO event code type
    #[derive(Debug, Clone, PartialEq)]
    pub struct CAMEOEventCode(pub String);

    impl TryFrom<Option<&str>> for CAMEOEventCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self> {
            match value {
                Some(s) => {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return Err(anyhow::anyhow!("CAMEOEventCode cannot be empty"));
                    }
                    // CAMEO event codes can be 2-4 characters and may be zero-padded
                    if trimmed.len() < 2 || trimmed.len() > 4 {
                        return Err(anyhow::anyhow!(
                            "CAMEOEventCode must be 2-4 characters, got: '{}'",
                            trimmed
                        ));
                    }
                    // Should contain only digits (can be zero-padded like "01", "025", "0251")
                    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
                        return Err(anyhow::anyhow!(
                            "CAMEOEventCode must contain only digits, got: '{}'",
                            trimmed
                        ));
                    }
                    Ok(CAMEOEventCode(trimmed.to_string()))
                }
                None => Err(anyhow::anyhow!("CAMEOEventCode cannot be None")),
            }
        }
    }

    // QuadClass code type
    #[derive(Debug, Clone, PartialEq)]
    pub struct QuadClassCode(pub u8);

    // Tone type for GKG compatibility
    #[derive(Debug, Clone, PartialEq)]
    pub struct Tone(pub f64);

    impl TryFrom<Option<&str>> for Tone {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self> {
            match value {
                Some(s) => {
                    let tone = s
                        .parse::<f64>()
                        .map_err(|e| anyhow::anyhow!("Invalid Tone value: {}", e))?;
                    Ok(Tone(tone))
                }
                None => Ok(Tone(0.0)), // Default to neutral tone
            }
        }
    }

    // Re-export main types
    pub use super::EventAction;
    pub use super::GoldsteinScale;

    impl TryFrom<Option<&str>> for QuadClassCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self> {
            match value {
                Some(s) => {
                    let code = s
                        .parse::<u8>()
                        .map_err(|e| anyhow::anyhow!("Invalid QuadClassCode value: {}", e))?;
                    Ok(QuadClassCode(code))
                }
                None => Err(anyhow::anyhow!("QuadClassCode cannot be None")),
            }
        }
    }
}

pub mod event_geography {
    use super::*;

    // Geography type code
    #[derive(Debug, Clone, PartialEq)]
    pub struct GeographyTypeCode(pub u8);

    // FIPS country code
    pub use super::FIPSCountryCode;

    // Re-export main Geography type as EventGeography for compatibility
    pub type EventGeography = super::Geography;

    impl TryFrom<Option<&str>> for GeographyTypeCode {
        type Error = anyhow::Error;

        fn try_from(value: Option<&str>) -> Result<Self> {
            match value {
                Some(s) => {
                    let code = s
                        .parse::<u8>()
                        .map_err(|e| anyhow::anyhow!("Invalid GeographyTypeCode value: {}", e))?;
                    Ok(GeographyTypeCode(code))
                }
                None => Err(anyhow::anyhow!("GeographyTypeCode cannot be None")),
            }
        }
    }
}
