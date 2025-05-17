use super::raw_types::{language::RawTranslationInfo, tone::ToneValue, *};
use crate::components::base_components::raw_types;
use csv::StringRecordIter;
pub type RawEventTimeDate = V2Date;
pub type RawMentionTimeDate = V2Date;
pub type RawMentionType = V2SourceCollectionIdentifier;
pub type RawMentionSourceName<'a> = RawV2SourceCommonName<'a>;

pub struct RawMention<'a> {
    global_event_id: GlobalEventID,
    event_time_date: RawEventTimeDate,
    mention_time_date: RawMentionTimeDate,
    mention_type: RawMentionType,
    mention_source_name: RawMentionSourceName<'a>,
    mention_identifier: V2SourceCollectionIdentifier,
    sentence_id: SentenceID,
    actor_1_char_offset: CharOffset,
    actor_2_char_offset: CharOffset,
    action_char_offset: CharOffset,
    in_raw_text: InRawText,
    confidence: Confidence,
    mention_doc_len: DocLength,
    mention_doc_tone: ToneValue,
    mention_doc_translation_info: RawTranslationInfo<'a>,
}
impl<'a> From<StringRecordIter<'a>> for RawMention<'a> {
    fn from(mut iter: StringRecordIter<'a>) -> Self {
        RawMention {
            global_event_id: GlobalEventID(iter.next().unwrap().parse().unwrap()),
            event_time_date: RawEventTimeDate::from(iter.next().unwrap()),
            mention_time_date: RawMentionTimeDate::from(iter.next().unwrap()),
            mention_type: RawMentionType::from(iter.next().unwrap()),
            mention_source_name: RawMentionSourceName::from(iter.next().unwrap()),
            mention_identifier: V2SourceCollectionIdentifier::from(iter.next().unwrap()),
            sentence_id: SentenceID::from(iter.next().unwrap()),
            actor_1_char_offset: CharOffset::from(iter.next().unwrap()),
            actor_2_char_offset: CharOffset::from(iter.next().unwrap()),
            action_char_offset: CharOffset::from(iter.next().unwrap()),
            in_raw_text: InRawText::from(iter.next().unwrap()),
            confidence: Confidence::from(iter.next().unwrap()),
            mention_doc_len: DocLength::from(iter.next().unwrap()),
            mention_doc_tone: ToneValue::from(iter.next().unwrap()),
            mention_doc_translation_info: {
                // You may need to parse multiple fields for this, adjust as needed
                // For example, if it needs 3 fields:
                let lang_code = iter.next().unwrap();
                let engine = iter.next().unwrap();
                let model = iter.next().unwrap();
                RawTranslationInfo(
                    raw_types::language::SourceLanguageCode(
                        lang_code.as_bytes().try_into().unwrap(),
                    ),
                    (
                        raw_types::language::RawTranslationEngine(engine),
                        raw_types::language::RawTranslationModel(model),
                    ),
                )
            },
        }
    }
}
