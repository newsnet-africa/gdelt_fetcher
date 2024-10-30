// region: Imports
use crate::models::gdelt::{GDELTObject, ToProto};
use crate::generated::gdelt_tone::GdeltTone;
// endregion

// region: Tone Struct Definition
/// Represents the tone of a GDELT event.
///
/// The `Tone` struct holds various attributes related to the tone of an event,
/// including the overall tone, positive and negative scores, polarity, activity
/// reference density, self-reference density, and word count.
#[derive(Debug, Clone)]
pub struct Tone {
    /// The overall tone of the event.
    tone: f64,
    /// The positive score of the event.
    positive_score: f64,
    /// The negative score of the event.
    negative_score: f64,
    /// The polarity of the event.
    polarity: f64,
    /// The activity reference density of the event.
    activity_reference_density: f64,
    /// The self-reference density of the event.
    self_reference_density: f64,
    /// The word count of the event.
    word_count: u128,
}
// endregion

impl ToProto for Tone {
    type ProtoType = GdeltTone;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        // Parse the tone field.
        let tone = if self.tone == 0.0 { None } else { Some(self.tone) };

        // Parse the positive score field.
        let positive_score = if self.positive_score == 0.0 { None } else { Some(self.positive_score) };

        // Parse the negative score field.
        let negative_score = if self.negative_score == 0.0 { None } else { Some(self.negative_score) };

        // Parse the polarity field.
        let polarity = if self.polarity == 0.0 { None } else { Some(self.polarity) };

        // Parse the activity reference density field.
        let activity_reference_density = if self.activity_reference_density == 0.0 { None } else { Some(self.activity_reference_density) };

        // Parse the self-reference density field.
        let self_reference_density = if self.self_reference_density == 0.0 { None } else { Some(self.self_reference_density) };

        // Parse the word count field.
        let word_count = if self.word_count == 0 { None } else { Some(self.word_count as u64) };

        // Return None if all fields are None, otherwise return a new GdeltTone instance.
        if tone.is_none()
            && positive_score.is_none()
            && negative_score.is_none()
            && polarity.is_none()
            && activity_reference_density.is_none()
            && self_reference_density.is_none()
            && word_count.is_none()
        {
            None
        } else {
            Some(GdeltTone {
                tone: tone.unwrap_or(0.0),
                positive_score: positive_score.unwrap_or(0.0),
                negative_score: negative_score.unwrap_or(0.0),
                polarity: polarity.unwrap_or(0.0),
                activity_reference_density: activity_reference_density.unwrap_or(0.0),
                self_reference_density: self_reference_density.unwrap_or(0.0),
                word_count: word_count.unwrap_or(0),
            })
        }
    }
}

// region: GDELTObject Implementation
impl GDELTObject for Tone {
    // region: from_strings Method
    /// Creates a `Tone` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited tone attributes.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `Tone` instance. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        // Split the record into fields using the ',' delimiter.
        let tone_attributes: Vec<&str> = <Self as GDELTObject>::delimited_vector(",", record);

        // Create a new Tone instance from the parsed fields.
        Self::new(tone_attributes)
    }
    // endregion

    // region: new Method
    /// Creates a new `Tone` instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the tone.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `Tone` instance. Returns `None` if all fields are default values.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Parse the tone field.
        let tone = match fields.get(0) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the positive score field.
        let positive_score = match fields.get(1) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the negative score field.
        let negative_score = match fields.get(2) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the polarity field.
        let polarity = match fields.get(3) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the activity reference density field.
        let activity_reference_density = match fields.get(4) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the self-reference density field.
        let self_reference_density = match fields.get(5) {
            Some(&"") | None => 0.0,
            Some(value) => value.parse::<f64>().unwrap_or(0.0),
        };

        // Parse the word count field.
        let word_count = match fields.get(6) {
            Some(&"") | None => 0,
            Some(value) => value.parse::<u128>().unwrap_or(0),
        };

        // Return None if all fields are default values, otherwise return a new Tone instance.
        if tone == 0.0
            && positive_score == 0.0
            && negative_score == 0.0
            && polarity == 0.0
            && activity_reference_density == 0.0
            && self_reference_density == 0.0
            && word_count == 0
        {
            None
        } else {
            Some(Self {
                tone,
                positive_score,
                negative_score,
                polarity,
                activity_reference_density,
                self_reference_density,
                word_count,
            })
        }
    }
    // endregion
}
// endregion}
