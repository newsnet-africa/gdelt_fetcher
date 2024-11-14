use crate::utils::types::api_types::output_format::OutputFormat;
use crate::utils::types::api_types::output_mode::OutputMode;
use crate::utils::types::api_types::query_types::QueryType;
use crate::utils::types::api_types::sort_types::SortType;
use crate::utils::types::api_types::translator::Translator;
use chrono::{DateTime, Duration, Utc};

pub struct GdeltApiRequest {
    query: Vec<QueryType>,
    mode: Option<OutputMode>,
    format: Option<OutputFormat>,
    timestamp: Option<Duration>,
    start_end_times: Option<(DateTime<Utc>, DateTime<Utc>)>,
    max_records: Option<u8>,
    timeline: Option<u8>,
    trans: Option<Translator>,
    sort: Option<SortType>,
    timezoom: bool,
}

impl GdeltApiRequest {
    pub fn new(query: Vec<QueryType>) -> Self {
        Self {
            format: None,
            query,
            mode: None,
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
