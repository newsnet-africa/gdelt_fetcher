//TODO:
// 1. Create Models for the other GDELT data tables. Some of it comes as RSS stuff or whatever
// 2. Check which fields are Nullable (Can be None), and adjust the functions accordingly. (Like Primary Keys obviously cannot be None, but I didn't check that) The instance where it is a None type, an error should be returned
//    a. The Errors returned should be recoverable for each record in the table
//    b. Corruption should be handled as well.
// 3. Write the functionality for the parsing of NewType for Intermediary type (So instead of parsing from &str, parse from u128). Super low priority, but makes it easier for data creation later on. I dont want to take a number, convert it to a string, then convert it to a newtype. That is silly

pub mod gcam;
pub mod types;

// Re-export commonly used types from GCAM module
pub use gcam::{
    Dictionary,
    EnrichedGCAMEntry,
    GCAMCodebookDatabase,
    GCAMCodebookEntry,
    GCAMCodebookParser,
    GCAMCoverageStats,
    GCAMEntry, // Keep for backward compatibility
    Language,
    MeasurementType,
};
