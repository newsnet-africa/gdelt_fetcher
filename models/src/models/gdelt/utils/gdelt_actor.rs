use crate::generated;
// region GDELTActor
use crate::models::gdelt::{GDELTObject, ToProto};
use crate::generated::gdelt_actor::GdeltActor;

/// Represents an actor in the GDELT dataset.
/// This struct holds various attributes related to an actor, such as codes, names, and other identifiers.
#[derive(Debug, Clone)]
pub struct GDELTActor {
    // region Fields
    /// The code of the actor, if available.
    code: Option<String>,
    /// The name of the actor, if available.
    name: Option<String>,
    /// The country code associated with the actor, if available.
    country_code: Option<String>,
    /// The known group code associated with the actor, if available.
    known_group_code: Option<String>,
    /// The ethnic code associated with the actor, if available.
    ethnic_code: Option<String>,
    /// A tuple containing two optional religion codes associated with the actor.
    religion_code: (Option<String>, Option<String>),
    /// A tuple containing three optional additional codes associated with the actor.
    codes: (Option<String>, Option<String>, Option<String>),
    // endregion
}

impl ToProto for GDELTActor {
    type ProtoType = GdeltActor;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.code.is_none()
            && self.name.is_none()
            && self.country_code.is_none()
            && self.known_group_code.is_none()
            && self.ethnic_code.is_none()
            && self.religion_code.0.is_none()
            && self.religion_code.1.is_none()
            && self.codes.0.is_none()
            && self.codes.1.is_none()
            && self.codes.2.is_none()
        {
            return None;
        }

        let code = self.code.clone();
        let name = self.name.clone();
        let country_code = self.country_code.clone();
        let known_group_code = self.known_group_code.clone();
        let ethnic_code = self.ethnic_code.clone();
        let religion_code = match self.religion_code {
            (Some(ref code1), Some(ref code2)) => Some(generated::gdelt_actor::gdelt_actor::ReligionCode {
                code1: Some(code1.clone()),
                code2: Some(code2.clone()),
            }),
            _ => None,
        };
        let codes = match self.codes {
            (Some(ref code1), Some(ref code2), Some(ref code3)) => Some(generated::gdelt_actor::gdelt_actor::AdditionalCodes {
                code1: Some(code1.clone()),
                code2: Some(code2.clone()),
                code3: Some(code3.clone()),
            }),
            _ => None,
        };

        Some(GdeltActor {
            code,
            name,
            country_code,
            known_group_code,
            ethnic_code,
            religion_code,
            codes,
        })
    }
}

// region GDELTObject Implementation
impl GDELTObject for GDELTActor {
    // region from_strings
    /// Creates a `GDELTActor` from a tab-delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A tab-delimited string representing the actor attributes.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An instance of `GDELTActor` if parsing is successful, otherwise `None`.
    fn from_strings(record: &str) -> Option<Self> {
        // Split the input string into a vector of &str using tab as the delimiter.
        let actor_attributes: Vec<&str> = <Self as GDELTObject>::delimited_vector("\t", record);
        // Create a new GDELTActor instance using the parsed attributes.
        Self::new(actor_attributes)
    }
    // endregion

    // region new
    /// Creates a new `GDELTActor` from a vector of strings.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of strings representing the actor attributes.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An instance of `GDELTActor` if parsing is successful, otherwise `None`.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Parse the code from the first field, if available.
        let code = match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };
        // Parse the name from the second field, if available.
        let name = match fields.get(1) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };
        // Parse the country code from the third field, if available.
        let country_code = match fields.get(2) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };
        // Parse the known group code from the fourth field, if available.
        let known_group_code = match fields.get(3) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };
        // Parse the ethnic code from the fifth field, if available.
        let ethnic_code = match fields.get(4) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };
        // Parse the religion codes from the sixth and seventh fields, if available.
        let religion_code = (
            match fields.get(5) {
                Some(&"") | None => None,
                Some(value) => Some(value.to_string()),
            },
            match fields.get(6) {
                Some(&"") | None => None,
                Some(value) => Some(value.to_string()),
            },
        );
        // Parse the additional codes from the eighth, ninth, and tenth fields, if available.
        let codes = (
            match fields.get(7) {
                Some(&"") | None => None,
                Some(value) => Some(value.to_string()),
            },
            match fields.get(8) {
                Some(&"") | None => None,
                Some(value) => Some(value.to_string()),
            },
            match fields.get(9) {
                Some(&"") | None => None,
                Some(value) => Some(value.to_string()),
            },
        );

        // If all fields are empty, return None. Otherwise, return a new GDELTActor instance.
        if code.is_none()
            && name.is_none()
            && country_code.is_none()
            && known_group_code.is_none()
            && ethnic_code.is_none()
            && religion_code.0.is_none()
            && religion_code.1.is_none()
            && codes.0.is_none()
            && codes.1.is_none()
            && codes.2.is_none()
        {
            None
        } else {
            Some(Self {
                code,
                name,
                country_code,
                known_group_code,
                ethnic_code,
                religion_code,
                codes,
            })
        }
    }
    // endregion
}
// endregion

// region GDELTActor Implementation
impl GDELTActor {
    // region Getters
    /// Returns the code of the actor.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - A reference to the actor's code.
    pub fn code(&self) -> &Option<String> {
        &self.code
    }

    /// Returns the name of the actor.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - A reference to the actor's name.
    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    /// Returns the country code of the actor.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - A reference to the actor's country code.
    pub fn country_code(&self) -> &Option<String> {
        &self.country_code
    }

    /// Returns the known group code of the actor.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - A reference to the actor's known group code.
    pub fn known_group_code(&self) -> &Option<String> {
        &self.known_group_code
    }

    /// Returns the ethnic code of the actor.
    ///
    /// # Returns
    ///
    /// * `&Option<String>` - A reference to the actor's ethnic code.
    pub fn ethnic_code(&self) -> &Option<String> {
        &self.ethnic_code
    }

    /// Returns the religion codes of the actor.
    ///
    /// # Returns
    ///
    /// * `&(Option<String>, Option<String>)` - A reference to the tuple containing the actor's religion codes.
    pub fn religion_code(&self) -> &(Option<String>, Option<String>) {
        &self.religion_code
    }

    /// Returns the additional codes of the actor.
    ///
    /// # Returns
    ///
    /// * `&(Option<String>, Option<String>, Option<String>)` - A reference to the tuple containing the actor's additional codes.
    pub fn codes(&self) -> &(Option<String>, Option<String>, Option<String>) {
        &self.codes
    }
    // endregion

    // region Setters
    /// Sets the code of the actor.
    ///
    /// # Arguments
    ///
    /// * `code` - The new code to set.
    pub fn set_code(&mut self, code: Option<String>) {
        self.code = code;
    }

    /// Sets the name of the actor.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name to set.
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    /// Sets the country code of the actor.
    ///
    /// # Arguments
    ///
    /// * `country_code` - The new country code to set.
    pub fn set_country_code(&mut self, country_code: Option<String>) {
        self.country_code = country_code;
    }

    /// Sets the known group code of the actor.
    ///
    /// # Arguments
    ///
    /// * `known_group_code` - The new known group code to set.
    pub fn set_known_group_code(&mut self, known_group_code: Option<String>) {
        self.known_group_code = known_group_code;
    }

    /// Sets the ethnic code of the actor.
    ///
    /// # Arguments
    ///
    /// * `ethnic_code` - The new ethnic code to set.
    pub fn set_ethnic_code(&mut self, ethnic_code: Option<String>) {
        self.ethnic_code = ethnic_code;
    }

    /// Sets the religion codes of the actor.
    ///
    /// # Arguments
    ///
    /// * `religion_code` - The new religion codes to set.
    pub fn set_religion_code(&mut self, religion_code: (Option<String>, Option<String>)) {
        self.religion_code = religion_code;
    }

    /// Sets the additional codes of the actor.
    ///
    /// # Arguments
    ///
    /// * `codes` - The new codes to set.
    pub fn set_codes(&mut self, codes: (Option<String>, Option<String>, Option<String>)) {
        self.codes = codes;
    }
    // endregion
}
// endregion
// endregion