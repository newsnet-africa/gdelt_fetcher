use crate::components::base_components::{
    event::{RawEventAction, quad_class::QuadClass, verb::top_level_actions::Verb},
    raw_types::{
        RawAverageTone, RawGoldsteinScale, RawIsRootEvent, RawNumberOfArticles, RawNumberOfMentions,
    },
};

pub struct EventAction {
    pub is_root_event: RawIsRootEvent,
    pub verb: Verb,
    pub quad_class: QuadClass,
    pub goldstein: RawGoldsteinScale,
    pub num_mentions: RawNumberOfMentions,
    pub num_articles: RawNumberOfArticles,
    pub average_tone: RawAverageTone,
}

impl From<RawEventAction> for EventAction {
    fn from(value: RawEventAction) -> Self {
        Self {
            is_root_event: value.is_root_event,
            verb: Verb::from(value.event_code),
            quad_class: QuadClass::from(value.quad_class),
            goldstein: value.goldstein_scale,
            num_mentions: value.number_of_mentions,
            num_articles: value.number_of_articles,
            average_tone: value.average_tone,
        }
    }
}
