use super::raw_types::{
    RawAverageTone, RawCAMEOEventBaseCode, RawCAMEOEventCode, RawCAMEOEventRootCode,
    RawGoldsteinScale, RawIsRootEvent, RawNumberOfArticles, RawNumberOfMentions,
    RawNumberOfSources, RawQuadClass,
};

pub mod quad_class;
pub mod verb;

pub struct RawEventAction {
    pub is_root_event: RawIsRootEvent,
    pub event_code: RawCAMEOEventCode,
    pub event_base_code: RawCAMEOEventBaseCode,
    pub event_root_code: RawCAMEOEventRootCode,
    pub quad_class: RawQuadClass,
    pub goldstein_scale: RawGoldsteinScale,
    pub number_of_mentions: RawNumberOfMentions,
    pub number_of_sources: RawNumberOfSources,
    pub number_of_articles: RawNumberOfArticles,
    pub average_tone: RawAverageTone,
}
