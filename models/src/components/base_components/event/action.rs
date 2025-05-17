use super::super::raw_types::{tone::ToneValue, *};

pub struct RawEventAction {
    pub is_root_event: IsRootEvent,
    pub event_code: CAMEOEventCode,
    pub event_base_code: CAMEOEventBaseCode,
    pub event_root_code: CAMEOEventRootCode,
    pub quad_class: RawQuadClass,
    pub goldstein_scale: GoldsteinScale,
    pub number_of_mentions: NumberOfMentions,
    pub number_of_sources: NumberOfSources,
    pub number_of_articles: NumberOfArticles,
    pub average_tone: ToneValue,
}
impl<'a> From<csv::StringRecordIter<'a>> for RawEventAction {
    fn from(mut iter: csv::StringRecordIter<'a>) -> Self {
        RawEventAction {
            is_root_event: super::super::raw_types::IsRootEvent(
                iter.next().unwrap().parse().unwrap(),
            ),
            event_code: CAMEOEventCode::from(iter.next().unwrap()),
            event_base_code: CAMEOEventBaseCode::from(iter.next().unwrap()),
            event_root_code: CAMEOEventRootCode::from(iter.next().unwrap()),
            quad_class: RawQuadClass::from(iter.next().unwrap()),
            goldstein_scale: GoldsteinScale::from(iter.next().unwrap()),
            number_of_mentions: NumberOfMentions::from(iter.next().unwrap()),
            number_of_sources: NumberOfSources::from(iter.next().unwrap()),
            number_of_articles: NumberOfArticles::from(iter.next().unwrap()),
            average_tone: ToneValue::from(iter.next().unwrap()),
        }
    }
}
