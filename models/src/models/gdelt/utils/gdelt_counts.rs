use crate::models::gdelt::utils::gdelt_categorylist::GDELTCategoryList;
use crate::models::gdelt::utils::gdelt_location::GDELTLocation;
use crate::models::gdelt::{CellItem, GDELTObject, ToProto};
use std::str::FromStr;
use crate::generated::gdelt_count::GdeltCount;

/// Represents a GDELT count with various attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct GDELTCount {
    // region Fields
    /// The type of the count, represented as an optional `GDELTCategoryList`.
    count_type: Option<GDELTCategoryList>,
    /// The count value, represented as an optional `i128`.
    count: Option<i128>,
    /// The type of the object, represented as an optional `String`.
    object_type: Option<String>,
    /// The location associated with the count, represented as an optional `GDELTLocation`.
    location: Option<GDELTLocation>,
    /// The approximate character offset, represented as an optional `u128`.
    approximate_char_offset: Option<u128>,
    // endregion
}

impl ToProto for GDELTCount {
    type ProtoType = GdeltCount;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.count_type.is_none()
            && self.count.is_none()
            && self.object_type.is_none()
            && self.location.is_none()
            && self.approximate_char_offset.is_none()
        {
            return None;
        }

        let count_type = self.count_type.as_ref().and_then(|ct| {
            match ct.to_proto() {
                None => { None }
                Some(val) => { Some(val as i32) }
            }
        });
        let count = match self.count {
            None => { None }
            Some(val) => { Some(val as i64)}
        };
        let object_type = self.object_type.clone();
        let location = self.location.as_ref().and_then(|loc| loc.to_proto());
        let approximate_char_offset = match self.approximate_char_offset {
            None => { None }
            Some(val) => {Some(val as u64)}
        };

        Some(GdeltCount {
            count_type,
            count,
            object_type,
            location,
            approximate_char_offset,
        })
    }
}

impl Default for GDELTCount {
    // region Default Implementation
    /// Provides a default implementation for `GDELTCount`.
    ///
    /// # Returns
    ///
    /// * `Self` - A `GDELTCount` instance with all fields set to `None`.
    fn default() -> Self {
        GDELTCount {
            count_type: None,
            count: None,
            object_type: None,
            location: None,
            approximate_char_offset: None,
        }
    }
    // endregion
}

impl CellItem for GDELTCount {
    // region CellItem Implementation
    /// Converts a delimited string into a vector of `GDELTCount` instances.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice containing the delimited data.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Self>>` - An optional vector of `GDELTCount` instances.
    fn vec_from_cell(string: &str) -> Option<Vec<Self>> {
        // Split the input string into a vector of strings using the delimiter ";".
        let list_of_items = <Self as GDELTObject>::delimited_vector(";", string);

        // Map each string slice to a `GDELTCount` instance, using `from_strings` method.
        let out_vec = list_of_items
            .iter()
            .map(|cou| GDELTCount::from_strings(cou).unwrap_or_default())
            .collect::<Vec<GDELTCount>>();

        // If all elements in the vector are default `GDELTCount` instances, return `None`.
        // Otherwise, return the vector.
        match out_vec.iter().all(|cou| cou.eq(&GDELTCount::default())) {
            true => None,
            false => Some(out_vec),
        }
    }
    // endregion
}

impl GDELTObject for GDELTCount {
    // region GDELTObject Implementation
    /// Creates a `GDELTCount` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTCount` instance.
    fn from_strings(record: &str) -> Option<Self> {
        // Split the record string into a vector of string slices using the delimiter "#".
        let count_attributes: Vec<&str> = <Self as GDELTObject>::delimited_vector("#", record);
        // Create a new `GDELTCount` instance from the vector of string slices.
        Self::new(count_attributes)
    }

    /// Creates a new `GDELTCount` instance from a vector of string slices.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the count.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTCount` instance.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Ensure the vector has at least 10 elements.
        if fields.len() < 10 {
            return None;
        }

        // region Field Parsing
        // Parse the count type from the first field.
        let count_type = match fields.get(0) {
            Some(&"") | None => None, // If the field is empty or not present, set to None.
            Some(&count_type) => GDELTCategoryList::from_str(count_type).ok(), // Parse the field to `GDELTCategoryList`.
        };
        // Parse the count value from the second field.
        let count = match fields.get(1) {
            Some(&"") | None => None, // If the field is empty or not present, set to None.
            Some(&count) => count.parse::<i128>().ok(), // Parse the field to `i128`.
        };
        // Parse the object type from the third field.
        let object_type = match fields.get(2) {
            Some(&"") | None => None, // If the field is empty or not present, set to None.
            Some(&object_type) => Some(object_type.to_string()), // Convert the field to `String`.
        };
        // Create a `GDELTLocation` instance from the remaining fields.
        let location = {
            let loc_fields = fields[3..].to_vec(); // Get the fields from the fourth position onwards.
            GDELTLocation::new(loc_fields) // Create a new `GDELTLocation` instance.
        };
        // Parse the approximate character offset from the eleventh field.
        let offset = match fields.get(10) {
            Some(&"") | None => None, // If the field is empty or not present, set to None.
            Some(&offset) => offset.parse::<u128>().ok(), // Parse the field to `u128`.
        };
        // endregion

        // If all fields are None, return None. Otherwise, return a new `GDELTCount` instance.
        if count_type.is_none()
            && count.is_none()
            && object_type.is_none()
            && location.is_none()
            && offset.is_none()
        {
            None
        } else {
            Some(Self {
                count_type,
                count,
                object_type,
                location,
                approximate_char_offset: offset,
            })
        }
    }
    // endregion
}
