use chrono::{DateTime, Utc};

pub struct GlobalEventID(u128);

pub struct MentionID(u128);

pub struct GlobalKnowledgeGraphRecordID(DateTime<Utc>, bool, u64);
