use std::fmt::Display;
use crate::models::gdelt::{CellItem, GDELTObject, ToProto};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use chrono::Datelike;
use chrono::Timelike;
use crate::generated::gdelt_date::GdeltDate;
use prost_types::{Timestamp, TimestampError};

// region DateResolution Enum
/// Enum representing different date resolutions.
#[derive(Debug, PartialEq)]
pub enum DateResolution {
    YearMonthDate,                // Resolution up to year, month, and date.
    YearMonth,                    // Resolution up to year and month.
    MonthDay,                     // Resolution up to month and day.
    Year,                         // Resolution up to year.
    YearMonthDayHourMinuteSecond, // Resolution up to year, month, day, hour, minute, and second.
}

// endregion

// region GDELTDate Struct
/// Struct representing a GDELT date with a full date and an optional offset.
#[derive(Debug, Clone, PartialEq)]
pub struct GDELTDate {
    pub full_date: NaiveDateTime, // The full date and time.
    pub offset: Option<u128>,     // An optional offset.
}

// impl Display

#[macro_export]
macro_rules! naive_date_time_to_timestamp {
    ($naive_date_time:expr) => {{
        let year = $naive_date_time.year() as i64;
        let month = $naive_date_time.month() as u8;
        let day = $naive_date_time.day() as u8;
        let hour = $naive_date_time.hour() as u8;
        let minute = $naive_date_time.minute() as u8;
        let second = $naive_date_time.second() as u8;

        Timestamp::date_time(year, month, day, hour, minute, second)
    }};
}

impl Default for GDELTDate {
    fn default() -> Self {
        Self {
            full_date: NaiveDateTime::default(),
            offset: None,
        }
    }
}
impl GDELTDate {
    // region GDELTDate Methods
    /// Returns the full date and time.
    pub fn full_date(&self) -> NaiveDateTime {
        self.full_date
    }

    /// Returns the optional offset.
    pub fn offset(&self) -> Option<u128> {
        self.offset
    }
    // endregion
    
    /// Creates a `GDELTDate` instance from a string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the date record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTDate` instance. Returns `None` if parsing fails.
    pub fn from_string(record: &str) -> Option<Self> {
        let result = record.parse::<i64>();
        
        match result {
            Ok(int) => {
                let full_date = Self::naive_date_from_int(int)?;
                Some(Self {
                    full_date,
                    offset: None,
                })
            }
            Err(_) => {
                let result = record.parse::<f32>();
                match result {
                    Ok(float) => {
                        let full_date = Self::naive_date_from_float(float)?;
                        Some(Self {
                            full_date,
                            offset: None,
                        })
                    }
                    Err(_) => {
                        let gdelt_date = Self::from_string(record)?;
                        Some(gdelt_date)
                    }
                }
            }
        }
    }
}

// endregion

impl ToProto for GDELTDate {
    type ProtoType = GdeltDate;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        let temp = self.clone();
        let full_date = temp.full_date;
        let offset = match temp.offset {
            None => { None }
            Some(val) => { Some(val as u64) }
        };

        Some(GdeltDate {
            date: match naive_date_time_to_timestamp!(full_date) {
                Ok(ts) => { Some(ts) }
                Err(_) => { None }
            },
            offset,
        })
    }
}
// region GDELTObject Implementation
impl GDELTObject for GDELTDate {
    
    // region Date Conversion Methods

    
    // endregion
    
    
    fn from_strings(record: &str) -> Option<Self> {
        let result = record.parse::<i64>();

        match result {
            Ok(int) => {
                let full_date = Self::naive_date_from_int(int)?;
                Some(Self {
                    full_date,
                    offset: None,
                })
            }
            Err(_) => {
                let result = record.parse::<f32>();
                match result {
                    Ok(float) => {
                        let full_date = Self::naive_date_from_float(float)?;
                        Some(Self {
                            full_date,
                            offset: None,
                        })
                    }
                    Err(_) => {
                        Self::from_string(record)
                    }
                }
            }
        }
    }

    fn new(fields: Vec<&str>) -> Option<Self> {
        match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => {
                let gdelt_date = Self::from_string(value)?;
                Some(gdelt_date)
            }
        }
    }
}

// endregion
// region CellItem Implementation
impl CellItem for GDELTDate {
    fn vec_from_cell(string: &str) -> Option<Vec<Self>> {
        let chunk = <Self as GDELTObject>::delimited_vector(";", string);

        let out_vec = chunk
            .iter()
            .map(|ch| Self::from_string(ch).unwrap_or_default())
            .collect::<Vec<Self>>();

        match out_vec.iter().all(|dat| dat.eq(&GDELTDate::default())) {
            true => None,
            false => Some(out_vec),
        }
    }
}

// endregion

impl GDELTDate {
    // region Date Conversion Methods
    /// Converts a string to a `NaiveDateTime`.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the date record.
    ///
    /// # Returns
    ///
    /// * `NaiveDateTime` - A `NaiveDateTime` instance.
    pub fn date_from_string(string_date: &str) -> Option<DateTime<Utc>> {
        let result = string_date.parse::<i64>();
        
        match result {
            Ok(int) => Self::date_from_int(int),
            Err(_) => {
                let result = string_date.parse::<f32>();
                match result {
                    Ok(float) => Self::date_from_float(float),
                    Err(_) => Self::from_string(string_date)
                        .map(|gdelt_date| gdelt_date.full_date.and_utc()),
                }
            }
        }
    }
    
    /// Checks the resolution of the given integer date.
    ///
    /// # Arguments
    ///
    /// * `int` - An integer representing the date.
    ///
    /// # Returns
    ///
    /// * `Option<DateResolution>` - An optional `DateResolution`.
    pub fn check_resolution(int: i64) -> Option<DateResolution> {
        let length = if int.is_negative() {
            int.to_string().len() - 1
        } else {
            int.to_string().len()
        };
        
        match length {
            4 => Some(DateResolution::Year),
            8 => Some(DateResolution::YearMonthDate),
            6 => Some(DateResolution::YearMonth),
            14 => Some(DateResolution::YearMonthDayHourMinuteSecond),
            _ => None,
        }
    }
    
    /// Converts an integer to a `NaiveDateTime`.
    ///
    /// # Arguments
    ///
    /// * `input` - An integer representing the date.
    ///
    /// # Returns
    ///
    /// * `Option<NaiveDateTime>` - An optional `NaiveDateTime`.
    pub fn naive_date_from_int(input: i64) -> Option<NaiveDateTime> {
        let str_input = input.abs().to_string();
        let resolution = Self::check_resolution(input)?;
        
        match resolution {
            DateResolution::Year => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                Some(NaiveDateTime::from(NaiveDate::from_ymd_opt(year, 1, 1)?))
            }
            DateResolution::YearMonth => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                Some(NaiveDateTime::from(NaiveDate::from_ymd_opt(
                    year, month, 1,
                )?))
            }
            DateResolution::YearMonthDate => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                let day: u32 = str_input.get(6..8)?.parse().ok()?;
                Some(NaiveDateTime::from(NaiveDate::from_ymd_opt(
                    year, month, day,
                )?))
            }
            DateResolution::YearMonthDayHourMinuteSecond => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                let day: u32 = str_input.get(6..8)?.parse().ok()?;
                let hour: u32 = str_input.get(8..10)?.parse().ok()?;
                let minute: u32 = str_input.get(10..12)?.parse().ok()?;
                let second: u32 = str_input.get(12..14)?.parse().ok()?;
                Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(year, month, day)?,
                    NaiveTime::from_hms_opt(hour, minute, second)?,
                ))
            }
            DateResolution::MonthDay => {
                let month: u32 = str_input.get(0..2)?.parse().ok()?;
                let day: u32 = str_input.get(2..4)?.parse().ok()?;
                Some(NaiveDateTime::from(NaiveDate::from_ymd_opt(0, month, day)?))
            }
        }
    }
    
    // region Date Conversion Methods
    /// Converts a string to a `NaiveDateTime`.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the date record.
    ///
    /// # Returns
    ///
    /// * `Option<NaiveDateTime>` - An optional `NaiveDateTime` instance.
    pub fn naive_date_from_string(string_date: &str) -> Option<NaiveDateTime> {
        let result = string_date.parse::<i64>();
        
        match result {
            Ok(int) => Self::naive_date_from_int(int),
            Err(_) => {
                let result = string_date.parse::<f32>();
                match result {
                    Ok(float) => Self::naive_date_from_float(float),
                    Err(_) => Self::from_string(string_date).map(|gdelt_date| gdelt_date.full_date),
                }
            }
        }
    }
    
    /// Converts a float to a `NaiveDateTime`.
    ///
    /// # Arguments
    ///
    /// * `input` - A float representing the date.
    ///
    /// # Returns
    ///
    /// * `Option<NaiveDateTime>` - An optional `NaiveDateTime` instance.
    pub fn naive_date_from_float(input: f32) -> Option<NaiveDateTime> {
        let int_part = input.floor();
        let decimal_part = input.fract();
        
        let date_fract = 365.0 * decimal_part;
        
        let year = int_part as i32;
        let date = NaiveDate::from_yo_opt(year, date_fract as u32);
        
        date.map(|d| d.and_hms_opt(0, 0, 0))? // Ensure NaiveDateTime is returned
    }
    
    /// Converts a float to a `DateTime<Utc>`.
    ///
    /// # Arguments
    ///
    /// * `input` - A float representing the date.
    ///
    /// # Returns
    ///
    /// * `Option<DateTime<Utc>>` - An optional `DateTime<Utc>` instance.
    pub fn date_from_float(input: f32) -> Option<DateTime<Utc>> {
        let int_part = input.floor();
        let decimal_part = input.fract();
        
        let date_fract = 365.0 * decimal_part;
        
        let year = int_part as i32;
        let date = NaiveDate::from_yo_opt(year, date_fract as u32);
        
        date.map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d.and_hms_opt(0, 0, 0).unwrap(), Utc)) // Convert NaiveDate to DateTime<Utc>
        
    }
    
    /// Converts an integer to a `DateTime<Utc>`.
    ///
    /// # Arguments
    ///
    /// * `input` - An integer representing the date.
    ///
    /// # Returns
    ///
    /// * `Option<DateTime<Utc>>` - An optional `DateTime<Utc>` instance.
    
    
    pub fn date_from_int(input: i64) -> Option<DateTime<Utc>> {
        let str_input = input.abs().to_string();
        let resolution = Self::check_resolution(input)?;
        
        match resolution {
            DateResolution::Year => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                NaiveDate::from_ymd_opt(year, 1, 1)
                    .map(|date| DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc))
            }
            DateResolution::YearMonth => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                NaiveDate::from_ymd_opt(year, month, 1)
                    .map(|date| DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc))
            }
            DateResolution::YearMonthDate => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                let day: u32 = str_input.get(6..8)?.parse().ok()?;
                NaiveDate::from_ymd_opt(year, month, day)
                    .map(|date| DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc))
            }
            DateResolution::YearMonthDayHourMinuteSecond => {
                let year: i32 = str_input.get(0..4)?.parse().ok()?;
                let month: u32 = str_input.get(4..6)?.parse().ok()?;
                let day: u32 = str_input.get(6..8)?.parse().ok()?;
                let hour: u32 = str_input.get(8..10)?.parse().ok()?;
                let minute: u32 = str_input.get(10..12)?.parse().ok()?;
                let second: u32 = str_input.get(12..14)?.parse().ok()?;
                NaiveDate::from_ymd_opt(year, month, day)
                    .map(|date| DateTime::<Utc>::from_utc(date.and_hms(hour, minute, second), Utc))
            }
            DateResolution::MonthDay => {
                let month: u32 = str_input.get(0..2)?.parse().ok()?;
                let day: u32 = str_input.get(2..4)?.parse().ok()?;
                // Assuming the year is the current year for MonthDay resolution
                let year = Utc::now().year();
                NaiveDate::from_ymd_opt(year, month, day)
                    .map(|date| DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc))
            }
        }
    }
}