use chrono::{DateTime, Duration, Utc};
use iso_country::Country;

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

enum QueryType {
    Quote(String),
    BooleanOR(Vec<String>),
    Exclude(Vec<QueryType>),
    Domain(String),
    DomainIs(String),
    ImageFaceTone(Operation, f16),
    ImageNumFaces(Operation, u8),
    ImageORCMeta(String),
    ImageTags(Vec<String>),
    ImageWebCount(Operation, u128),
    Near {
        distance: u8,
        root_word: String,
        near_word: String,
    },
    Repeat(String),
    SourceCountry(Country),
    SourceLang(Language),
    Theme(GKGTheme),
    Tone(Operation, f16),
    ToneAbs(Operation, f16),
}

enum Mode {
    ArtList,
    ArtGallery,
    ImageCollage,
    ImageCollageInfo,
    ImageGallery,
    ImageCollageShare,
    TimelineVol,
    TimelineVolRow,
    TimelineVolInfo,
    TimelineTone,
    TimelineLang,
    TimelineSourceCountry,
    // TODO: The rest of these
}
