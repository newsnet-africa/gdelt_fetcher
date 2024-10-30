use crate::generated::gdelt_location::gdelt_location::GdeltLocationTypes;
use crate::models::gdelt::ToProto;

/// Represents the different types of locations in the GDELT dataset.
#[derive(Debug, Clone, PartialEq)]
pub enum GDELTLocationTypes {
    /// Represents a city location.
    City,
    /// Represents a country location.
    Country,
    /// Represents a province location.
    Province,
    /// Represents an unknown location type.
    Unknown,
}

// region: Implement From<u8> for GDELTLocationTypes
impl From<u8> for GDELTLocationTypes {
    /// Converts a `u8` value to a `GDELTLocationTypes` enum variant.
    ///
    /// # Arguments
    ///
    /// * `num` - A `u8` value representing the location type.
    ///
    /// # Returns
    ///
    /// * A `GDELTLocationTypes` enum variant corresponding to the `u8` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use models::models::gdelt::utils::gdelt_location_types::GDELTLocationTypes;
    ///
    /// let location_type = GDELTLocationTypes::from(1);
    /// assert_eq!(location_type, GDELTLocationTypes::Country);
    /// ```
    fn from(num: u8) -> Self {
        match num {
            1 => Self::Country,
            2 | 5 => Self::Province,
            3 | 4 => Self::City,
            _ => Self::Unknown,
        }
    }
}
// endregion

// region: Implement From<GDELTLocationTypes> for u8
impl From<GDELTLocationTypes> for u8 {
    /// Converts a `GDELTLocationTypes` enum variant to a `u8` value.
    ///
    /// # Arguments
    ///
    /// * `value` - A `GDELTLocationTypes` enum variant.
    ///
    /// # Returns
    ///
    /// * A `u8` value corresponding to the `GDELTLocationTypes` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use models::models::gdelt::utils::gdelt_location_types::GDELTLocationTypes;
    ///
    ///
    /// let num = u8::from(GDELTLocationTypes::City);
    /// assert_eq!(num, 3);
    /// ```
    fn from(value: GDELTLocationTypes) -> Self {
        match value {
            GDELTLocationTypes::City => 3,
            GDELTLocationTypes::Country => 1,
            GDELTLocationTypes::Province => 2,
            GDELTLocationTypes::Unknown => 0,
        }
    }
}
// endregion

impl ToProto for GDELTLocationTypes {
    type ProtoType = i32;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        let proto_type = u8::from(self.clone());
        Some(proto_type as i32)
    }
}