use crate::generated::gdelt_location::GdeltLocation;
use super::gdelt_location_types::GDELTLocationTypes;
use crate::models::gdelt::{CellItem, DatabaseTableEnum, GDELTObject, ToProto};
use crate::generated::gdelt_location::gdelt_location::AdminCodes;
use crate::generated::gdelt_location::gdelt_location::Coordinates;

/// Represents a location in the GDELT dataset.
///
/// The `GDELTLocation` struct holds various attributes related to a location
/// in the GDELT dataset. It includes information such as location type, full name,
/// country code, administrative codes, coordinates, feature ID, and character offset.
#[derive(Debug, Clone, PartialEq)]
pub struct GDELTLocation {
    /// The type of the location.
    location_type: Option<GDELTLocationTypes>,
    /// The full name of the location.
    location_full_name: Option<String>,
    /// The country code of the location.
    location_country_code: Option<String>,
    /// The administrative codes of the location (admin1, admin2).
    location_admin_code: (Option<String>, Option<String>),
    /// The coordinates of the location (latitude, longitude).
    co_ord: Option<(f64, f64)>,
    /// The feature ID of the location.
    feature_id: Option<String>,
    /// The character offset of the location.
    char_offset: Option<u128>,
}

impl Default for GDELTLocation {
    /// Provides a default instance of `GDELTLocation` with all fields set to `None`.
    fn default() -> Self {
        GDELTLocation {
            location_type: None,
            location_full_name: None,
            location_country_code: None,
            location_admin_code: (None, None),
            co_ord: None,
            feature_id: None,
            char_offset: None,
        }
    }
}

impl ToProto for GDELTLocation {
    type ProtoType = GdeltLocation;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        if self.location_type.is_none()
            && self.location_full_name.is_none()
            && self.location_country_code.is_none()
            && self.location_admin_code.0.is_none()
            && self.location_admin_code.1.is_none()
            && self.co_ord.is_none()
            && self.feature_id.is_none()
            && self.char_offset.is_none()
        {
            return None;
        }

        let location_type = match &self.location_type {
            None => { None }
            Some(val) => { val.to_proto() }
        };
        let location_full_name = self.location_full_name.clone();
        let location_country_code = self.location_country_code.clone();
        let location_admin_code = match self.location_admin_code {
            (Some(ref admin1), Some(ref admin2)) => Some(AdminCodes {
                admin1: Some(admin1.clone()),
                admin2: Some(admin2.clone()),
            }),
            _ => None,
        };
        let co_ord = self.co_ord.map(|(lat, lon)| Coordinates {
            latitude: lat,
            longitude: lon,
        });
        let feature_id = self.feature_id.clone();
        let char_offset = match self.char_offset {
            None => { None }
            Some(v) => { Some(v as u64) }
        };

        Some(GdeltLocation {
            location_type,
            location_full_name,
            location_country_code,
            location_admin_code,
            co_ord,
            feature_id,
            char_offset,
        })
    }
}

impl GDELTObject for GDELTLocation {
    // region GDELTObject implementation

    /// Creates a `GDELTLocation` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited location record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTLocation` instance. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        let action_attributes: Vec<&str> = <Self as GDELTObject>::delimited_vector("\t", record);
        Self::new(action_attributes)
    }

    /// Creates a new `GDELTLocation` instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the location.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTLocation` instance. Returns `None` if parsing fails.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Determine if the location has an admin2 code based on the number of fields.
        let admin2 = if fields.len() == 8 { true } else { false };

        // Parse the location type from the first field.
        let location_type = match fields.get(0) {
            Some(&"") | None => None,
            Some(&location_type) => match location_type.parse::<u8>() {
                Ok(parsed) => Some(GDELTLocationTypes::from(parsed)),
                Err(_) => None,
            },
        };

        // Parse the full name of the location from the second field.
        let location_full_name = match fields.get(1) {
            Some(&"") | None => None,
            Some(&location_full_name) => Some(location_full_name.to_string()),
        };

        // Parse the country code of the location from the third field.
        let location_country_code = match fields.get(2) {
            Some(&"") | None => None,
            Some(&location_country_code) => Some(location_country_code.to_string()),
        };

        // Parse the admin1 code of the location from the fourth field.
        let location_admin_1_code = match fields.get(3) {
            Some(&"") | None => None,
            Some(&location_admin_1_code) => Some(location_admin_1_code.to_string()),
        };

        // Parse the admin2 code of the location if `admin2` is true.
        let location_admin_2_code = if admin2 {
            match fields.get(4) {
                Some(&"") | None => None,
                Some(&location_admin_2_code) => Some(location_admin_2_code.to_string()),
            }
        } else {
            None
        };

        // Parse the coordinates of the location from the last three fields.
        let co_ord = match fields.get(fields.len() - 3) {
            Some(&"") | None => None,
            Some(&latitude) => {
                let lat = latitude;
                let lon = match fields.get(fields.len() - 2) {
                    None => "0.0",
                    Some(&long) => long,
                };

                match (lat.parse::<f64>(), lon.parse::<f64>()) {
                    (Ok(flat), Ok(flon)) => Some((flat, flon)),
                    _ => None,
                }
            }
        };

        // Parse the feature ID of the location from the last field.
        let feature_id = match fields.get(fields.len() - 1) {
            Some(&"") | None => None,
            Some(&feature_id) => Some(feature_id.to_string()),
        };

        // Parse the character offset of the location from the last field.
        let char_offset = if admin2 {
            match fields.get(fields.len()) {
                Some(&"") | None => None,
                Some(&char_offset) => Some(char_offset.parse::<u128>().unwrap()),
            }
        } else {
            None
        };

        // If all fields are empty, return None. Otherwise, return a new GDELTLocation instance.
        if location_type.is_none()
            && location_full_name.is_none()
            && location_country_code.is_none()
            && location_admin_1_code.is_none()
            && location_admin_2_code.is_none()
            && co_ord.is_none()
            && feature_id.is_none()
            && char_offset.is_none()
        {
            None
        } else {
            Some(Self {
                location_type,
                location_full_name,
                location_country_code,
                location_admin_code: (location_admin_1_code, location_admin_2_code),
                co_ord,
                feature_id,
                char_offset,
            })
        }
    }
    // endregion
}

impl CellItem for GDELTLocation {
    // region CellItem implementation

    /// Converts a delimited string into a vector of `GDELTLocation` instances.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice that holds the delimited location records.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Self>>` - An optional vector of `GDELTLocation` instances. Returns `None` if parsing fails.
    fn vec_from_cell(string: &str) -> Option<Vec<Self>> {
        let list_of_items = <Self as GDELTObject>::delimited_vector(";", string);

        let out_vec = list_of_items
            .iter()
            .map(|loc| GDELTLocation::from_strings(loc).unwrap_or_default())
            .collect::<Vec<GDELTLocation>>();

        match out_vec.iter().all(|loc| {
            loc.eq(&GDELTLocation {
                location_type: None,
                location_full_name: None,
                location_country_code: None,
                location_admin_code: (None, None),
                co_ord: None,
                feature_id: None,
                char_offset: None,
            })
        }) {
            true => None,
            false => Some(out_vec),
        }
    }
    // endregion
}

impl GDELTLocation {
    // region Additional methods

    /// Creates a `GDELTLocation` instance from a delimited string and a database table enum.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited location record.
    /// * `database` - A `DatabaseTableEnum` indicating the type of database table.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTLocation` instance. Returns `None` if parsing fails.
    pub fn from_table(record: &str, database: DatabaseTableEnum) -> Option<Self> {
        // Determine the delimiter based on the database table type.
        let delimiter = match database {
            DatabaseTableEnum::Event(_) => "\t",
            DatabaseTableEnum::GlobalKnowledgeGraph(_) => "#",
            _ => return None,
        };

        // Split the record into fields using the determined delimiter.
        let values = <Self as GDELTObject>::delimited_vector(delimiter, record);

        // Determine if the location has an admin2 code based on the database table type.
        // let admin2 = matches!(database, DatabaseTableEnum::Event(_));

        // Create a new GDELTLocation instance from the parsed fields.
        Self::new(values)
    }
    // endregion
}