// region: Imports
use crate::models::gdelt::event::Event;
use crate::models::gdelt::gkg::GlobalKnowledgeGraph;
use crate::models::gdelt::mentions::Mentions;
// endregion

// region: Module Declarations
pub mod event;
pub mod gkg;
pub mod mentions;
pub mod utils;
// endregion

// region: Proto Trait
pub trait ToProto {
    type ProtoType;
    
    fn to_proto(&self) -> Option<Self::ProtoType>;
}
// endregion

// region: DatabaseTableEntry Trait
/// A trait that defines common behaviors for database table entries.
pub trait DatabaseTableEntry {
    /// Creates a blank instance of the implementing type.
    fn blank() -> Self;

    /// Creates an instance of the implementing type from a CSV row.
    ///
    /// # Arguments
    ///
    /// * `row` - A string slice that holds the CSV row.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional instance of the implementing type. Returns `None` if parsing fails.
    fn from_csv_row(row: &str) -> Option<Self>
    where
        Self: Sized;

    /// Splits a delimited string into a vector of string slices.
    ///
    /// # Arguments
    ///
    /// * `delimiter` - The character used as the delimiter.
    /// * `record` - The string slice to be split.
    ///
    /// # Returns
    ///
    /// * `Vec<&str>` - A vector of string slices.
    fn delimited_vector(delimiter: char, record: &str) -> Vec<&str> {
        record.split(delimiter).collect()
    }
}
// endregion

// region: DatabaseTableEnum Definition
/// An enum that represents different types of database table entries.
#[derive(Clone)]
pub enum DatabaseTableEnum {
    Mentions(Option<Mentions>),
    GlobalKnowledgeGraph(Option<GlobalKnowledgeGraph>),
    Event(Option<Event>),
}
// endregion

// region: DatabaseTableEntry Implementation for DatabaseTableEnum
/// Implements the `DatabaseTableEntry` trait for the `DatabaseTableEnum` enum.
impl DatabaseTableEntry for DatabaseTableEnum {
    /// This method is not used in the current context, so it panics.
    fn blank() -> Self {
        panic!("Not implemented for enum");
    }

    /// This method is not used in the current context, so it panics.
    fn from_csv_row(row: &str) -> Option<Self> {
        panic!("Not implemented for enum");
    }
}
// endregion

// region: GDELTObject Trait
/// A trait that defines common behaviors for GDELT objects.
pub trait GDELTObject {
    /// Creates an instance of the implementing type from a delimited string.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the delimited event attributes.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional instance of the implementing type. Returns `None` if parsing fails.
    fn from_strings(record: &str) -> Option<Self>
    where
        Self: Sized;

    /// Splits a delimited string into a vector of string slices.
    ///
    /// # Arguments
    ///
    /// * `delimiter` - The string used as the delimiter.
    /// * `record` - The string slice to be split.
    ///
    /// # Returns
    ///
    /// * `Vec<&str>` - A vector of string slices.
    fn delimited_vector<'a>(delimiter: &str, record: &'a str) -> Vec<&'a str> {
        record.split(delimiter).collect()
    }

    /// Creates a new instance of the implementing type from a vector of string fields.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields of the event.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - An optional instance of the implementing type. Returns `None` if all fields are default values.
    fn new(fields: Vec<&str>) -> Option<Self>
    where
        Self: Sized;
}
// endregion

// region: CellItem Trait
/// A trait that defines common behaviors for cell items.
pub trait CellItem {
    /// Creates a vector of instances of the implementing type from a string.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to be parsed.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Self>>` - An optional vector of instances of the implementing type. Returns `None` if parsing fails.
    fn vec_from_cell(string: &str) -> Option<Vec<Self>>
    where
        Self: Sized;
}
// endregion
