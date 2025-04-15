use crate::components::base_components::event::{
    AverageTone, GoldsteinScale, NumberOfArticles, NumberOfMention, quad_class::QuadClass,
    verb::top_level_actions::Verb,
};

pub struct EventAction {
    pub is_root_event: bool,
    pub verb: Verb,
    pub quad_class: QuadClass,
    pub goldstein: GoldsteinScale,
    pub num_mentions: NumberOfMention,
    pub num_articles: NumberOfArticles,
    pub average_tone: AverageTone,
}
