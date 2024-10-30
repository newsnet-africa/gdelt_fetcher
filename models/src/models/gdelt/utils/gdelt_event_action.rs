use crate::generated::gdelt_action::GdeltAction;
// region: use statements
use crate::models::gdelt::{GDELTObject, ToProto};
// endregion

/// Represents an action in the GDELT dataset.
///
/// The `GDELTAction` struct holds various attributes related to an event action
/// in the GDELT dataset. It includes information such as whether the event is a root event,
/// event codes, quad class, Goldstein scale, number of mentions, sources, articles, and average tone.
#[derive(Debug, Clone)]
pub struct GDELTAction {
    // region: struct fields
    /// Indicates if the event is a root event.
    is_root_event: bool,
    /// The event code associated with the action.
    event_code: String,
    /// The base event code.
    event_base_code: String,
    /// The root event code.
    event_root_code: String,
    /// The quad class of the event.
    quad_class: u8,
    /// The Goldstein scale value of the event.
    goldstein_scale: f32,
    /// The number of mentions of the event.
    number_of_mentions: u128,
    /// The number of sources reporting the event.
    number_of_sources: u128,
    /// The number of articles about the event.
    number_of_articles: u128,
    /// The average tone of the event.
    avg_tone: f32,
    // endregion
}

impl ToProto for GDELTAction {
    type ProtoType = GdeltAction;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.event_code.is_empty()
            && self.event_base_code.is_empty()
            && self.event_root_code.is_empty()
            && self.quad_class == 0
            && self.goldstein_scale == 0.0
            && self.number_of_mentions == 0
            && self.number_of_sources == 0
            && self.number_of_articles == 0
            && self.avg_tone == 0.0
        {
            return None;
        }

        Some(GdeltAction {
            is_root_event: self.is_root_event,
            event_code: self.event_code.clone(),
            event_base_code: self.event_base_code.clone(),
            event_root_code: self.event_root_code.clone(),
            quad_class: self.quad_class as u32,
            goldstein_scale: self.goldstein_scale,
            number_of_mentions: self.number_of_mentions as u64,
            number_of_sources: self.number_of_sources as u64,
            number_of_articles: self.number_of_articles as u64,
            avg_tone: self.avg_tone,
        })
    }
}

// region: GDELTObject implementation
impl GDELTObject for GDELTAction {
    /// Creates a `GDELTAction` instance from a tab-delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the tab-delimited event action record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTAction` instance. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        let action_attributes: Vec<&str> = <Self as GDELTObject>::delimited_vector("\t", record);
        Self::new(action_attributes)
    }

    /// Creates a new `GDELTAction` instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the event action.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTAction` instance. Returns `None` if parsing fails.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // region: parse fields
        let is_root_event = {
            let string_ire = fields.get(0)?.parse::<u8>();
            match string_ire {
                Ok(ire) => match ire {
                    1 => true,
                    0 | _ => false,
                },
                Err(_) => false,
            }
        };

        let event_code = fields.get(1)?.to_string();
        let event_base_code = fields.get(2)?.to_string();
        let event_root_code = fields.get(3)?.to_string();
        let quad_class = fields.get(4)?.parse().unwrap_or(0);
        let goldstein_scale = fields.get(5)?.parse().unwrap_or(0.0);
        let number_of_mentions = fields.get(6)?.parse().unwrap_or(0);
        let number_of_sources = fields.get(7)?.parse().unwrap_or(0);
        let number_of_articles = fields.get(8)?.parse().unwrap_or(0);
        let avg_tone = fields.get(9)?.parse().unwrap_or(0.0);
        // endregion

        // If all fields are empty, return None. Otherwise, return a new GDELTEventAction instance.
        if event_code.is_empty()
            && event_base_code.is_empty()
            && event_root_code.is_empty()
            && quad_class == 0
            && goldstein_scale == 0.0
            && number_of_mentions == 0
            && number_of_sources == 0
            && number_of_articles == 0
            && avg_tone == 0.0
        {
            None
        } else {
            Some(Self {
                is_root_event,
                event_code,
                event_base_code,
                event_root_code,
                quad_class,
                goldstein_scale,
                number_of_mentions,
                number_of_sources,
                number_of_articles,
                avg_tone,
            })
        }
    }
}
// endregion

// region: GDELTAction implementation
impl GDELTAction {
    /// Returns whether the event is a root event.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the event is a root event, `false` otherwise.
    pub fn is_root_event(&self) -> bool {
        self.is_root_event
    }

    /// Returns the event code associated with the action.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice representing the event code.
    pub fn event_code(&self) -> &str {
        &self.event_code
    }

    /// Returns the base event code.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice representing the base event code.
    pub fn event_base_code(&self) -> &str {
        &self.event_base_code
    }

    /// Returns the root event code.
    ///
    /// # Returns
    ///
    /// * `&str` - A string slice representing the root event code.
    pub fn event_root_code(&self) -> &str {
        &self.event_root_code
    }

    /// Returns the quad class of the event.
    ///
    /// # Returns
    ///
    /// * `u8` - An unsigned 8-bit integer representing the quad class.
    pub fn quad_class(&self) -> u8 {
        self.quad_class
    }

    /// Returns the Goldstein scale value of the event.
    ///
    /// # Returns
    ///
    /// * `f32` - A 32-bit floating point number representing the Goldstein scale value.
    pub fn goldstein_scale(&self) -> f32 {
        self.goldstein_scale
    }

    /// Returns the number of mentions of the event.
    ///
    /// # Returns
    ///
    /// * `u128` - An unsigned 128-bit integer representing the number of mentions.
    pub fn number_of_mentions(&self) -> u128 {
        self.number_of_mentions
    }

    /// Returns the number of sources reporting the event.
    ///
    /// # Returns
    ///
    /// * `u128` - An unsigned 128-bit integer representing the number of sources.
    pub fn number_of_sources(&self) -> u128 {
        self.number_of_sources
    }

    /// Returns the number of articles about the event.
    ///
    /// # Returns
    ///
    /// * `u128` - An unsigned 128-bit integer representing the number of articles.
    pub fn number_of_articles(&self) -> u128 {
        self.number_of_articles
    }

    /// Returns the average tone of the event.
    ///
    /// # Returns
    ///
    /// * `f32` - A 32-bit floating point number representing the average tone.
    pub fn avg_tone(&self) -> f32 {
        self.avg_tone
    }

    /// Sets whether the event is a root event.
    ///
    /// # Arguments
    ///
    /// * `is_root_event` - A boolean indicating if the event is a root event.
    pub fn set_is_root_event(&mut self, is_root_event: bool) {
        self.is_root_event = is_root_event;
    }

    /// Sets the event code associated with the action.
    ///
    /// # Arguments
    ///
    /// * `event_code` - A string representing the event code.
    pub fn set_event_code(&mut self, event_code: String) {
        self.event_code = event_code;
    }

    /// Sets the base event code.
    ///
    /// # Arguments
    ///
    /// * `event_base_code` - A string representing the base event code.
    pub fn set_event_base_code(&mut self, event_base_code: String) {
        self.event_base_code = event_base_code;
    }

    /// Sets the root event code.
    ///
    /// # Arguments
    ///
    /// * `event_root_code` - A string representing the root event code.
    pub fn set_event_root_code(&mut self, event_root_code: String) {
        self.event_root_code = event_root_code;
    }

    /// Sets the quad class of the event.
    ///
    /// # Arguments
    ///
    /// * `quad_class` - An unsigned 8-bit integer representing the quad class.
    pub fn set_quad_class(&mut self, quad_class: u8) {
        self.quad_class = quad_class;
    }

    /// Sets the Goldstein scale value of the event.
    ///
    /// # Arguments
    ///
    /// * `goldstein_scale` - A 32-bit floating point number representing the Goldstein scale value.
    pub fn set_goldstein_scale(&mut self, goldstein_scale: f32) {
        self.goldstein_scale = goldstein_scale;
    }

    /// Sets the number of mentions of the event.
    ///
    /// # Arguments
    ///
    /// * `number_of_mentions` - An unsigned 128-bit integer representing the number of mentions.
    pub fn set_number_of_mentions(&mut self, number_of_mentions: u128) {
        self.number_of_mentions = number_of_mentions;
    }

    /// Sets the number of sources reporting the event.
    ///
    /// # Arguments
    ///
    /// * `number_of_sources` - An unsigned 128-bit integer representing the number of sources.
    pub fn set_number_of_sources(&mut self, number_of_sources: u128) {
        self.number_of_sources = number_of_sources;
    }

    /// Sets the number of articles about the event.
    ///
    /// # Arguments
    ///
    /// * `number_of_articles` - An unsigned 128-bit integer representing the number of articles.
    pub fn set_number_of_articles(&mut self, number_of_articles: u128) {
        self.number_of_articles = number_of_articles;
    }

    /// Sets the average tone of the event.
    ///
    /// # Arguments
    ///
    /// * `avg_tone` - A 32-bit floating point number representing the average tone.
    pub fn set_avg_tone(&mut self, avg_tone: f32) {
        self.avg_tone = avg_tone;
    }
}
// endregion