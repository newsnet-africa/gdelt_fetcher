//region Imports
use crate::models::gdelt::utils::gdelt_date::GDELTDate;
use crate::models::gdelt::utils::gdelt_source_collection_identifier::SourceCollectionIdentifier;
use crate::models::gdelt::{DatabaseTableEntry, GDELTObject, ToProto};
use crate::generated::mention::Mentions as GdeltMention;
//endregion

/// Represents a mention in the GDELT dataset.
///
/// This struct contains various fields that describe a mention, including
/// identifiers, dates, offsets, and other metadata.
#[derive(Debug, Clone)]
pub struct Mentions {
    /// The global event ID associated with the mention.
    global_event_id: Option<u128>,
    /// The date and time of the event.
    event_time_date: Option<GDELTDate>,
    /// The date and time of the mention.
    mention_time_date: Option<GDELTDate>,
    /// The type of mention.
    mention_type: Option<SourceCollectionIdentifier>,
    /// The source name of the mention.
    mention_source_name: Option<String>,
    /// The identifier of the mention.
    mention_identifier: Option<String>,
    /// The sentence ID where the mention occurs.
    sentence_id: Option<u128>,
    /// The character offset of the first actor in the mention.
    actor_1_char_offset: Option<u128>,
    /// The character offset of the second actor in the mention.
    actor_2_char_offset: Option<u128>,
    /// The character offset of the action in the mention.
    action_char_offset: Option<u128>,
    /// Indicates whether the mention is in raw text.
    in_raw_text: Option<bool>,
    /// The confidence level of the mention.
    confidence: Option<u8>,
    /// The length of the document containing the mention.
    mention_doc_length: Option<u128>,
    /// The tone of the document containing the mention.
    mention_doc_tone: Option<f64>,
    /// Translation information for the document containing the mention.
    mention_donc_translation_info: Option<String>,
    /// Any extra information about the mention.
    extras: Option<String>,
}

impl ToProto for Mentions {
    type ProtoType = GdeltMention;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.global_event_id.is_none()
            && self.event_time_date.is_none()
            && self.mention_time_date.is_none()
            && self.mention_type.is_none()
            && self.mention_source_name.is_none()
            && self.mention_identifier.is_none()
            && self.sentence_id.is_none()
            && self.actor_1_char_offset.is_none()
            && self.actor_2_char_offset.is_none()
            && self.action_char_offset.is_none()
            && self.in_raw_text.is_none()
            && self.confidence.is_none()
            && self.mention_doc_length.is_none()
            && self.mention_doc_tone.is_none()
            && self.mention_donc_translation_info.is_none()
            && self.extras.is_none()
        {
            None
        } else {
            Some(GdeltMention {
                global_event_id: self.global_event_id.map(|v| v as u64),
                event_time_date: match self.event_time_date.clone() {
                    Some(date) => date.to_proto(),
                    None => None,
                },
                mention_time_date: match self.mention_time_date.clone() {
                    Some(date) => date.to_proto(),
                    None => None,
                },
                mention_type: self.mention_type.clone().map(|sci| u8::from(sci) as i32),
                mention_source_name: self.mention_source_name.clone(),
                mention_identifier: self.mention_identifier.clone(),
                sentence_id: self.sentence_id.map(|id| id as u64),
                actor_1_char_offset: self.actor_1_char_offset.map(|id| id as u64),
                actor_2_char_offset: self.actor_2_char_offset.map(|id| id as u64),
                action_char_offset: self.action_char_offset.map(|id| id as u64),
                in_raw_text: self.in_raw_text,
                confidence: self.confidence.map(|id| id as u32),
                mention_doc_length: self.mention_doc_length.map(|id| id as u64),
                mention_doc_tone: self.mention_doc_tone,
                mention_doc_translation_info: self.mention_donc_translation_info.clone(),
                extras: self.extras.clone(),
            })
        }
    }
}

//region Implementation of Mentions
impl Mentions {
    /// Returns the global event ID associated with the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The global event ID.
    pub fn global_event_id(&self) -> Option<u128> {
        self.global_event_id
    }

    /// Returns the date and time of the event.
    ///
    /// # Returns
    ///
    /// * `&Option<GDELTDate>` - The date and time of the event.
    pub fn event_time_date(&self) -> &Option<GDELTDate> {
        &self.event_time_date
    }

    /// Returns the date and time of the mention.
    ///
    /// # Returns
    ///
    /// * `&Option<GDELTDate>` - The date and time of the mention.
    pub fn mention_time_date(&self) -> &Option<GDELTDate> {
        &self.mention_time_date
    }

    /// Returns the type of mention.
    ///
    /// # Returns
    ///
    /// * `&Option<SourceCollectionIdentifier>` - The type of mention.
    pub fn mention_type(&self) -> &Option<SourceCollectionIdentifier> {
        &self.mention_type
    }

    /// Returns the source name of the mention.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - The source name of the mention.
    pub fn mention_source_name(&self) -> &Option<String> {
        &self.mention_source_name
    }

    /// Returns the identifier of the mention.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - The identifier of the mention.
    pub fn mention_identifier(&self) -> &Option<String> {
        &self.mention_identifier
    }

    /// Returns the sentence ID where the mention occurs.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The sentence ID.
    pub fn sentence_id(&self) -> Option<u128> {
        self.sentence_id
    }

    /// Returns the character offset of the first actor in the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The character offset of the first actor.
    pub fn actor_1_char_offset(&self) -> Option<u128> {
        self.actor_1_char_offset
    }

    /// Returns the character offset of the second actor in the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The character offset of the second actor.
    pub fn actor_2_char_offset(&self) -> Option<u128> {
        self.actor_2_char_offset
    }

    /// Returns the character offset of the action in the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The character offset of the action.
    pub fn action_char_offset(&self) -> Option<u128> {
        self.action_char_offset
    }

    /// Indicates whether the mention is in raw text.
    ///
    /// # Returns
    ///
    /// * `&Option<bool>` - `true` if the mention is in raw text, `false` otherwise.
    pub fn in_raw_text(&self) -> &Option<bool> {
        &self.in_raw_text
    }

    /// Returns the confidence level of the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u8>` - The confidence level.
    pub fn confidence(&self) -> Option<u8> {
        self.confidence
    }

    /// Returns the length of the document containing the mention.
    ///
    /// # Returns
    ///
    /// * `Option<u128>` - The document length.
    pub fn mention_doc_length(&self) -> Option<u128> {
        self.mention_doc_length
    }

    /// Returns the tone of the document containing the mention.
    ///
    /// # Returns
    ///
    /// * `Option<f64>` - The document tone.
    pub fn mention_doc_tone(&self) -> Option<f64> {
        self.mention_doc_tone
    }

    /// Returns the translation information for the document containing the mention.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - The translation information.
    pub fn mention_donc_translation_info(&self) -> &Option<String> {
        &self.mention_donc_translation_info
    }

    /// Returns any extra information about the mention.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - The extra information.
    pub fn extras(&self) -> &Option<String> {
        &self.extras
    }
}
//endregion

//region Implementation of DatabaseTableEntry for Mentions
impl DatabaseTableEntry for Mentions {
    /// Creates a blank `Mentions` instance with all fields set to `None`.
    ///
    /// # Returns
    ///
    /// * `Self` - A blank `Mentions` instance.
    fn blank() -> Self {
        Self {
            global_event_id: None,
            event_time_date: None,
            mention_time_date: None,
            mention_type: None,
            mention_source_name: None,
            mention_identifier: None,
            sentence_id: None,
            actor_1_char_offset: None,
            actor_2_char_offset: None,
            action_char_offset: None,
            in_raw_text: None,
            confidence: None,
            mention_doc_length: None,
            mention_doc_tone: None,
            mention_donc_translation_info: None,
            extras: None,
        }
    }

    /// Creates a `Mentions` instance from a CSV row.
    ///
    /// # Arguments
    ///
    /// * `row` - A string slice representing a row in a CSV file.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - A `Mentions` instance if parsing is successful, `None` otherwise.
    fn from_csv_row(row: &str) -> Option<Self> {
        Self::from_strings(row)
    }
}
//endregion

//region Implementation of GDELTObject for Mentions
impl GDELTObject for Mentions {
    /// Creates a `Mentions` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice representing a delimited record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - A `Mentions` instance if parsing is successful, `None` otherwise.
    fn from_strings(record: &str) -> Option<Self> {
        let fields = <Mentions as GDELTObject>::delimited_vector("\t", record);

        Self::new(fields)
    }

    /// Creates a `Mentions` instance from a vector of string slices.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - A `Mentions` instance if parsing is successful, `None` otherwise.
    fn new(fields: Vec<&str>) -> Option<Self> {
        let global_event_id = match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let event_time_date = match fields.get(1) {
            Some(&"") | None => None,
            Some(value) => GDELTDate::from_strings(value),
        };

        let mention_time_date = match fields.get(2) {
            Some(&"") | None => None,
            Some(value) => GDELTDate::from_strings(value),
        };

        let mention_type = match fields.get(3) {
            Some(&"") | None => None,
            Some(value) => SourceCollectionIdentifier::from_string(value),
        };

        let mention_source_name = match fields.get(4) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        let mention_identifier = match fields.get(5) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        let sentence_id = match fields.get(6) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let actor_1_char_offset = match fields.get(7) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let actor_2_char_offset = match fields.get(8) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let action_char_offset = match fields.get(9) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let in_raw_text = match fields.get(10) {
            Some(&"") | None => None,
            Some(value) => match value.parse::<u8>().ok() {
                Some(1) => Some(true),
                Some(0) => Some(false),
                _ => None,
            },
        };

        let confidence = match fields.get(11) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u8>().ok(),
        };

        let mention_doc_length = match fields.get(12) {
            Some(&"") | None => None,
            Some(value) => value.parse::<u128>().ok(),
        };

        let mention_doc_tone = match fields.get(13) {
            Some(&"") | None => None,
            Some(value) => value.parse::<f64>().ok(),
        };

        let mention_donc_translation_info = match fields.get(14) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        let extras = match fields.get(15) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        if global_event_id.is_none()
            && event_time_date.is_none()
            && mention_time_date.is_none()
            && mention_type.is_none()
            && mention_source_name.is_none()
            && mention_identifier.is_none()
            && sentence_id.is_none()
            && actor_1_char_offset.is_none()
            && actor_2_char_offset.is_none()
            && action_char_offset.is_none()
            && in_raw_text.is_none()
            && confidence.is_none()
            && mention_doc_length.is_none()
            && mention_doc_tone.is_none()
            && mention_donc_translation_info.is_none()
            && extras.is_none()
        {
            None
        } else {
            Some(Self {
                global_event_id,
                event_time_date,
                mention_time_date,
                mention_type,
                mention_source_name,
                mention_identifier,
                sentence_id,
                actor_1_char_offset,
                actor_2_char_offset,
                action_char_offset,
                in_raw_text,
                confidence,
                mention_doc_length,
                mention_doc_tone,
                mention_donc_translation_info,
                extras,
            })
        }
    }
}
//endregion
