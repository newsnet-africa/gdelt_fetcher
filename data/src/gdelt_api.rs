use std::fmt::Display;

use crate::utils::types::api_types::output_mode::OutputMode;
use crate::utils::types::api_types::query_types::QueryType;
use crate::utils::types::api_types::sort_types::SortType;
use crate::utils::types::api_types::translator::{self, Translator};
use crate::utils::types::api_types::BASE_API_URL;
use crate::utils::types::api_types::{output_format::OutputFormat, ToRequestLink};
use chrono::{DateTime, Duration, Utc};

pub struct GdeltApiRequest {
    query: Vec<QueryType>,
    mode: Option<OutputMode>,
    format: Option<OutputFormat>,
    timespan: Option<Duration>,
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
            timespan: None,
            start_end_times: None,
            max_records: None,
            timeline: None,
            trans: None,
            sort: None,
            timezoom: false,
        }
    }

    pub fn set_mode(&mut self, mode: OutputMode) {
        self.mode = Some(mode);
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = Some(format);
    }

    pub fn set_timespan(&mut self, timespan: Duration) {
        self.timespan = Some(timespan);
    }

    pub fn set_start_end_times(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.start_end_times = Some((start, end));
    }

    pub fn set_max_records(&mut self, max_records: u8) {
        self.max_records = Some(max_records);
    }

    pub fn set_timeline(&mut self, timeline: u8) {
        self.timeline = Some(timeline);
    }

    pub fn set_trans(&mut self, trans: Translator) {
        self.trans = Some(trans)
    }

    pub fn set_sort(&mut self, sort: SortType) {
        self.sort = Some(sort);
    }

    pub fn set_timezoom(&mut self, timezoom: bool) {
        self.timezoom = timezoom;
    }
}

impl Display for GdeltApiRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_request_link())
    }
}

impl ToRequestLink for GdeltApiRequest {
    fn to_request_link(&self) -> String {
        let mut complete_request = String::new();
        let query = format!("query={}", self.query.to_request_link());
        complete_request.push_str(&query);

        if let Some(mode) = &self.mode {
            let mode_str = format!("&mode={}", mode.to_request_link());
            complete_request.push_str(&mode_str);
        }

        if let Some(format) = &self.format {
            let format_str = format!("&format={}", format.to_request_link());
            complete_request.push_str(&format_str);
        }

        if let Some(timespan) = &self.timespan {
            let timespan_str = format!("&timespan={}", timespan.num_days());
            complete_request.push_str(&timespan_str);
        }

        if let Some((start, end)) = &self.start_end_times {
            let start_end_str = format!(
                "&startdatetime={}&enddatetime={}",
                start.to_rfc3339(),
                end.to_rfc3339()
            );
            complete_request.push_str(&start_end_str);
        }

        if let Some(max_records) = &self.max_records {
            let max_records_str = format!("&maxrecords={}", max_records);
            complete_request.push_str(&max_records_str);
        }

        if let Some(timeline) = &self.timeline {
            let timeline_str = format!("&timeline={}", timeline);
            complete_request.push_str(&timeline_str);
        }

        if let Some(trans) = &self.trans {
            let trans_str = format!("&trans={}", trans.to_request_link());
            complete_request.push_str(&trans_str);
        }

        if let Some(sort_type) = &self.sort {
            let sort_str = format!("&sort={}", sort_type.to_request_link());
            complete_request.push_str(&sort_str);
        }

        let timezoom_str = if self.timezoom {
            "&timezoom=yes".to_string()
        } else {
            "&timezoom=no".to_string()
        };
        complete_request.push_str(&timezoom_str);

        let mut request = BASE_API_URL.clone().to_string();
        request.push_str(&complete_request);
        request
    }
}

// TODO: Change String to &str
//
