use super::raw_types::{
    RawCharOffset, RawConfidence, RawDocLength, RawGlobalEventID, RawInRawText, RawSentenceID,
    RawSourceUrl, RawV2Date, RawV2SourceCollectionIdentifier, RawV2SourceCommonName,
    language::RawTranslationInfo,
    tone::{RawToneValue, RawV1Tone},
};

pub type RawEventTimeDate = RawV2Date;
pub type RawMentionTimeDate = RawV2Date;
pub type RawMentionType = RawV2SourceCollectionIdentifier;
pub type RawMentionSourceName<'a> = RawV2SourceCommonName<'a>;

pub struct RawMention<'a> {
    global_event_id: RawGlobalEventID,
    event_time_date: RawEventTimeDate,
    mention_time_date: RawMentionTimeDate,
    mention_type: RawMentionType,
    mention_source_name: RawMentionSourceName<'a>,
    mention_identifier: RawV2SourceCollectionIdentifier,
    sentence_id: RawSentenceID,
    actor_1_char_offset: RawCharOffset,
    actor_2_char_offset: RawCharOffset,
    action_char_offset: RawCharOffset,
    in_raw_text: RawInRawText,
    confidence: RawConfidence,
    mention_doc_len: RawDocLength,
    mention_doc_tone: RawToneValue,
    mention_doc_translation_info: RawTranslationInfo<'a>,
}
