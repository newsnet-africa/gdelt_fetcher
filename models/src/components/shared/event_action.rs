use crate::components::cameo::verb::EventCode;

use super::location::Location;

pub enum QuadClass {
    Verbal(EngagementType),
    Material(EngagementType),
}

pub enum EngagementType {
    Cooperation,
    Conflict,
}

pub struct EventAction {
    pub is_root_event: bool,
    pub event_code: EventCode,
    pub quad_class: QuadClass,
    pub goldstien_scale: f32,
    pub num_mentions: u128,
    pub num_sources: u128,
    pub num_articles: u128,
    pub average_tone: u8,
    pub location: Location,
}
