use crate::generated::gdelt_quotation::GdeltQuotation as ProtoGdeltQuotation;
use crate::models::gdelt::ToProto;
use crate::models::gdelt::{CellItem, GDELTObject};
use std::u128;

/// Represents a quotation in the GDELT dataset.
#[derive(Debug, Clone, PartialEq)]
pub struct GDELTQuotation {
    /// The offset of the quotation in the text.
    offset: Option<u128>,
    /// The length of the quotation.
    length: Option<u128>,
    /// The verb associated with the quotation.
    verb: Option<String>,
    /// The actual quoted text.
    quote: Option<String>,
}

impl Default for GDELTQuotation {
    /// Provides a default instance of `GDELTQuotation` with all fields set to `None`.
    fn default() -> Self {
        Self {
            offset: None,
            length: None,
            verb: None,
            quote: None,
        }
    }
}

impl ToProto for GDELTQuotation {
    type ProtoType = ProtoGdeltQuotation;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        // Parse the offset field.
        let offset = match self.offset {
            Some(value) => Some(value as u64),
            None => None,
        };

        // Parse the length field.
        let length = match self.length {
            Some(value) => Some(value as u64),
            None => None,
        };

        // Parse the verb field.
        let verb = match &self.verb {
            Some(value) => Some(value.clone()),
            None => None,
        };

        // Parse the quote field.
        let quote = match &self.quote {
            Some(value) => Some(value.clone()),
            None => None,
        };

        // Return None if all fields are None, otherwise return a new ProtoGdeltQuotation instance.
        if offset.is_none() && length.is_none() && verb.is_none() && quote.is_none() {
            None
        } else {
            Some(ProtoGdeltQuotation {
                offset,
                length,
                verb,
                quote,
            })
        }
    }
}

impl GDELTObject for GDELTQuotation {
    /// Creates a `GDELTQuotation` instance from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited quotation record.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTQuotation` instance. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        // Split the record into fields using the '|' delimiter.
        let fields: Vec<&str> = <Self as GDELTObject>::delimited_vector("|", record);

        // Create a new GDELTQuotation instance from the parsed fields.
        Self::new(fields)
    }

    /// Creates a new `GDELTQuotation` instance from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the quotation.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional `GDELTQuotation` instance. Returns `None` if all fields are `None`.
    fn new(fields: Vec<&str>) -> Option<Self> {
        // Parse the offset field.
        let offset = match fields.get(0) {
            Some(&"") | None => None,
            Some(value) => Some(value.parse::<u128>().unwrap()),
        };

        // Parse the length field.
        let length = match fields.get(1) {
            Some(&"") | None => None,
            Some(value) => Some(value.parse::<u128>().unwrap()),
        };

        // Parse the verb field.
        let verb = match fields.get(2) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Parse the quote field.
        let quote = match fields.get(3) {
            Some(&"") | None => None,
            Some(value) => Some(value.to_string()),
        };

        // Return None if all fields are None, otherwise return a new GDELTQuotation instance.
        if offset.is_none() && length.is_none() && verb.is_none() && quote.is_none() {
            None
        } else {
            Some(Self {
                offset,
                length,
                verb,
                quote,
            })
        }
    }
}

impl CellItem for GDELTQuotation {
    /// Creates a vector of `GDELTQuotation` instances from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice that holds the delimited quotations.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Self>>` - An optional vector of `GDELTQuotation` instances. Returns `None` if all quotations are default.
    fn vec_from_cell(string: &str) -> Option<Vec<Self>> {
        // Split the string into individual quotation records using the '#' delimiter.
        let list_of_items = <Self as GDELTObject>::delimited_vector("#", string);

        // Map each record to a GDELTQuotation instance, using the default if parsing fails.
        let out_vec = list_of_items
            .iter()
            .map(|quo| GDELTQuotation::from_strings(quo).unwrap_or_default())
            .collect::<Vec<GDELTQuotation>>();

        // Return None if all quotations are default, otherwise return the vector of quotations.
        match out_vec.iter().all(|quo| quo.eq(&GDELTQuotation::default())) {
            true => None,
            false => Some(out_vec),
        }
    }
}
