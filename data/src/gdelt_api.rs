use chrono::{DateTime, Duration, Utc};
use iso_country::Country;
use crate::utils::types::api_types::*;

pub struct GdeltApiRequest {
    query: Vec<QueryType>,
    mode: OutputMode,
    format: OutputFormat,
    timestamp: Some<Duration>,
    start_end_times: Some<(DateTime, DateTime)>,
    max_records: Some<u8>,
    timeline: Some<u8>,
    trans: Some<Translator>,
    sort: Some<SortType>,
    timezoom: bool
}

impl GdeltApiRequest {
    pub fn new(query: QueryType, mode: OutputMode, format: OutputFormat) {
        Self {
            query,
            mode,
            format,
            timestamp: None,
            start_end_times: None,
            max_records: None,
            timeline: None,
            trans: None,
            sort: None,
            timezoom: false,
        }
    }
}
