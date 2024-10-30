// region Modules and Imports

use super::utils::types::{DatabaseType, JsonRustTypes};
use crate::gdelt_fetcher::DOWNLOAD_PATH_FOLDER;
use csv::ReaderBuilder;
use json::{object::Object, JsonValue};
use once_cell::sync::Lazy;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use thiserror::Error;
// endregion

// region Static Variables
/// The code defines several static variables using the Lazy type from the once_cell crate.
/// These variables are initialized only once and are used throughout the program.
/// For example, JSON_PATH_FOLDER is a static variable that holds the path to the JSON folder:
///
/// Static variable for the JSON path folder.
static JSON_PATH_FOLDER: Lazy<&Path> = Lazy::new(|| Path::new("./data/json"));

/// Similarly, EXPORT_HEADINGS, MENTIONS_HEADINGS, and GKG_HEADINGS
/// are static variables that store the headings and their types for different database types:
/// Static variable for the export headings.
static EXPORT_HEADINGS: Lazy<Vec<(String, JsonRustTypes)>> = Lazy::new(|| {
    vec![
        ("GlobalEventID".to_string(), JsonRustTypes::Number),
        ("Day".to_string(), JsonRustTypes::Number),
        ("MonthYear".to_string(), JsonRustTypes::Number),
        ("Year".to_string(), JsonRustTypes::Number),
        ("FractionDate".to_string(), JsonRustTypes::Number),
        ("Actor1Code".to_string(), JsonRustTypes::String),
        ("Actor1Name".to_string(), JsonRustTypes::String),
        ("Actor1CountryCode".to_string(), JsonRustTypes::String),
        ("Actor1KnownGroupCode".to_string(), JsonRustTypes::String),
        ("Actor1EthnicCode".to_string(), JsonRustTypes::String),
        ("Actor1Religion1Code".to_string(), JsonRustTypes::String),
        ("Actor1Religion2Code".to_string(), JsonRustTypes::String),
        ("Actor1Type1Code".to_string(), JsonRustTypes::String),
        ("Actor1Type2Code".to_string(), JsonRustTypes::String),
        ("Actor1Type3Code".to_string(), JsonRustTypes::String),
        ("Actor2Code".to_string(), JsonRustTypes::String),
        ("Actor2Name".to_string(), JsonRustTypes::String),
        ("Actor2CountryCode".to_string(), JsonRustTypes::String),
        ("Actor2KnownGroupCode".to_string(), JsonRustTypes::String),
        ("Actor2EthnicCode".to_string(), JsonRustTypes::String),
        ("Actor2Religion1Code".to_string(), JsonRustTypes::String),
        ("Actor2Religion2Code".to_string(), JsonRustTypes::String),
        ("Actor2Type1Code".to_string(), JsonRustTypes::String),
        ("Actor2Type2Code".to_string(), JsonRustTypes::String),
        ("Actor2Type3Code".to_string(), JsonRustTypes::String),
        ("IsRootEvent".to_string(), JsonRustTypes::Boolean),
        ("EventCode".to_string(), JsonRustTypes::String),
        ("EventBaseCode".to_string(), JsonRustTypes::String),
        ("EventRootCode".to_string(), JsonRustTypes::String),
        ("QuadClass".to_string(), JsonRustTypes::Number),
        ("GoldsteinScale".to_string(), JsonRustTypes::Number),
        ("NumMentions".to_string(), JsonRustTypes::Number),
        ("NumSources".to_string(), JsonRustTypes::Number),
        ("NumArticles".to_string(), JsonRustTypes::Number),
        ("AvgTone".to_string(), JsonRustTypes::Number),
        ("Actor1Geo_Type".to_string(), JsonRustTypes::Number),
        ("Actor1Geo_Fullname".to_string(), JsonRustTypes::String),
        ("Actor1Geo_CountryCode".to_string(), JsonRustTypes::String),
        ("Actor1Geo_ADM1Code".to_string(), JsonRustTypes::String),
        ("Actor1Geo_ADM2Code".to_string(), JsonRustTypes::String),
        ("Actor1Geo_Lat".to_string(), JsonRustTypes::Number),
        ("Actor1Geo_Long".to_string(), JsonRustTypes::Number),
        ("Actor1Geo_FeatureID".to_string(), JsonRustTypes::String),
        ("Actor2Geo_Type".to_string(), JsonRustTypes::Number),
        ("Actor2Geo_Fullname".to_string(), JsonRustTypes::String),
        ("Actor2Geo_CountryCode".to_string(), JsonRustTypes::String),
        ("Actor2Geo_ADM1Code".to_string(), JsonRustTypes::String),
        ("Actor2Geo_ADM2Code".to_string(), JsonRustTypes::String),
        ("Actor2Geo_Lat".to_string(), JsonRustTypes::Number),
        ("Actor2Geo_Long".to_string(), JsonRustTypes::Number),
        ("Actor2Geo_FeatureID".to_string(), JsonRustTypes::String),
        ("ActionGeo_Type".to_string(), JsonRustTypes::Number),
        ("ActionGeo_Fullname".to_string(), JsonRustTypes::String),
        ("ActionGeo_CountryCode".to_string(), JsonRustTypes::String),
        ("ActionGeo_ADM1Code".to_string(), JsonRustTypes::String),
        ("ActionGeo_ADM2Code".to_string(), JsonRustTypes::String),
        ("ActionGeo_Lat".to_string(), JsonRustTypes::Number),
        ("ActionGeo_Long".to_string(), JsonRustTypes::Number),
        ("ActionGeo_FeatureID".to_string(), JsonRustTypes::String),
        ("DATEADDED".to_string(), JsonRustTypes::Number),
        ("SOURCEURL".to_string(), JsonRustTypes::String),
    ]
});

/// Static variable for the mentions headings.
static MENTIONS_HEADINGS: Lazy<Vec<(String, JsonRustTypes)>> = Lazy::new(|| {
    vec![
        ("GlobalEventID".to_string(), JsonRustTypes::Number),
        ("EventTimeDate".to_string(), JsonRustTypes::Number),
        ("MentionTimeDate".to_string(), JsonRustTypes::Number),
        ("MentionType".to_string(), JsonRustTypes::Short),
        ("MentionSourceName".to_string(), JsonRustTypes::String),
        ("MentionIdentifier".to_string(), JsonRustTypes::String),
        ("SentenceID".to_string(), JsonRustTypes::String),
        ("Actor1CharOffset".to_string(), JsonRustTypes::Short),
        ("Actor2CharOffset".to_string(), JsonRustTypes::Short),
        ("ActionCharOffset".to_string(), JsonRustTypes::Short),
        ("InRawText".to_string(), JsonRustTypes::Boolean),
        ("Confidence".to_string(), JsonRustTypes::Number),
        ("MentionDocLen".to_string(), JsonRustTypes::Number),
        ("MentionDocTone".to_string(), JsonRustTypes::Number),
        (
            "MentionDocTranslationInfo".to_string(),
            JsonRustTypes::String,
        ),
        ("Extras".to_string(), JsonRustTypes::String),
    ]
});

/// Static variable for the GKG headings.
static GKG_HEADINGS: Lazy<Vec<(String, JsonRustTypes)>> = Lazy::new(|| {
    vec![
        ("GKGRECORDID".to_string(), JsonRustTypes::String),
        ("V2.1DATE".to_string(), JsonRustTypes::Number),
        (
            "V2SOURCECOLLECTIONIDENTIFIER".to_string(),
            JsonRustTypes::Short,
        ),
        ("V2SOURCECOMMONNAME".to_string(), JsonRustTypes::String),
        ("V2DOCUMENTIDENTIFIER".to_string(), JsonRustTypes::String),
        ("V1COUNTS".to_string(), JsonRustTypes::Array),
        ("V2.1COUNTS".to_string(), JsonRustTypes::Array),
        ("V1THEMES".to_string(), JsonRustTypes::Array),
        ("V2ENHANCEDTHEMES".to_string(), JsonRustTypes::Array),
        ("V1LOCATIONS".to_string(), JsonRustTypes::Array),
        ("V2ENHANCEDLOCATIONS".to_string(), JsonRustTypes::Array),
        ("V1PERSONS".to_string(), JsonRustTypes::Array),
        ("V2ENHANCEDPERSONS".to_string(), JsonRustTypes::Array),
        ("V1ORGANIZATIONS".to_string(), JsonRustTypes::Array),
        ("V2ENHANCEDORGANIZATIONS".to_string(), JsonRustTypes::Array),
        ("V1.5TONE".to_string(), JsonRustTypes::Array),
        ("V2.1ENHANCEDDATES".to_string(), JsonRustTypes::Array),
        ("V2GCAM".to_string(), JsonRustTypes::Array),
        ("V2.1SHARINGIMAGE".to_string(), JsonRustTypes::String),
        ("V2.1RELATEDIMAGES".to_string(), JsonRustTypes::Array),
        ("V2.1SOCIALIMAGEEMBEDS".to_string(), JsonRustTypes::Array),
        ("V2.1SOCIALVIDEOEMBEDS".to_string(), JsonRustTypes::Array),
        ("V2.1QUOTATIONS".to_string(), JsonRustTypes::Array),
        ("V2.1ALLNAMES".to_string(), JsonRustTypes::Array),
        ("V2.1AMOUNTS".to_string(), JsonRustTypes::Array),
        ("V2.1TRANSLATIONINFO".to_string(), JsonRustTypes::Array),
        ("V2EXTRASXML".to_string(), JsonRustTypes::String),
    ]
});
// endregion

// region Enum: CsvToJsonError

/// The code defines an enum CsvToJsonError to represent various errors that can occur during the
/// CSV to JSON conversion process. Each variant of the enum corresponds to a specific type of error,
/// such as file opening errors, CSV reading errors, and JSON conversion errors:
///
/// Enum representing possible errors that can occur during CSV to JSON conversion.
#[derive(Error, Debug)]
pub enum CsvToJsonError {
    // region Variant: FileOpenError
    /// Error indicating failure to open the file.
    #[error("Failed to open file: {0}")]
    FileOpenError(#[from] std::io::Error),
    // endregion

    // region Variant: CsvReadError
    /// Error indicating failure to read a CSV record.
    #[error("Failed to read CSV record: {0}")]
    CsvReadError(#[from] csv::Error),
    // endregion

    // region Variant: PathFolderError
    /// Error indicating an issue with the path folder.
    #[error("Path Folder Error")]
    PathFolderError,
    // endregion

    // region Variant: JsonConversionError
    /// Error indicating failure during JSON conversion.
    #[error("JSON Conversion Error: {0}")]
    JsonConversionError(String),
    // endregion
}
// endregion

// region Functions
// region Function: string_to_json_value
/// The string_to_json_value function converts a string to a JSON value based on the provided type
/// and inserts it into a JSON object. It takes a JSON object,
/// a tuple containing a heading and its type, and a value.
/// The function matches the type and converts the value to the appropriate JSON type,
/// then inserts it into the JSON object:
///
/// Converts a string to a JSON value based on the provided type and inserts it into a JSON object.
///
/// This function takes a JSON object, a tuple containing a heading and its type, and a value.
/// It converts the value to the appropriate JSON type and inserts it into the JSON object.
///
/// # Arguments
///
/// * `json_object` - The JSON object to insert the value into.
/// * `item` - A tuple containing the heading and its type, and the value to be converted.
///
/// # Returns
///
/// A `Result` containing the updated JSON object or an error if the conversion fails.
///
/// # Errors
///
/// This function will return an error if the value cannot be parsed to the specified type.
pub fn string_to_json_value(
    mut json_object: Object,
    item: ((String, JsonRustTypes), String),
) -> Result<Object, Box<dyn Error>> {
    // Destructure the item tuple into heading, type, and value
    let ((heading, mut jrtype), value) = item;

    // region Handle Empty Value
    // If the value is empty or only contains whitespace, set the type to None
    if value.trim().is_empty() {
        jrtype = JsonRustTypes::None;
    }
    // endregion

    // region Convert Value
    // Match the type and convert the value to the appropriate JSON type
    let json_value = match jrtype {
        // If the type is None, set the JSON value to Null
        JsonRustTypes::None => Ok(JsonValue::Null),
        // If the type is Short, convert the value to a JSON string
        JsonRustTypes::Short => Ok(JsonValue::from(value.clone())),
        // If the type is String, convert the value to a JSON string
        JsonRustTypes::String => Ok(JsonValue::String(value.clone())),
        // If the type is Number, parse the value to a float and convert it to a JSON number
        JsonRustTypes::Number => value
            .parse::<f64>()
            .map(|v| JsonValue::Number(json::number::Number::from(v)))
            .map_err(|e| Box::new(e) as Box<dyn Error>),
        // If the type is Boolean, parse the value to an integer and convert it to a JSON boolean
        JsonRustTypes::Boolean => value
            .parse::<i32>()
            .map(|v| JsonValue::Boolean(v != 0))
            .map_err(|e| Box::new(e) as Box<dyn Error>),
        // If the type is Object, set the JSON value to an empty JSON object
        JsonRustTypes::Object => Ok(JsonValue::Object(Object::new())),
        // If the type is Array, set the JSON value to an empty JSON array
        JsonRustTypes::Array => Ok(JsonValue::Array(Vec::new())),
    };
    // endregion

    // region Insert Value
    // Insert the converted JSON value into the JSON object
    match json_value {
        Ok(value) => json_object.insert(&heading, value),
        // If there is an error during conversion, print an error message
        Err(e) => eprintln!(
            "Couldn't parse to JSON, Error:\n{}\nItem: {}: {:?}",
            e, heading, value
        ),
    }
    // endregion

    // Return the updated JSON object
    Ok(json_object)
}
// endregion

// region Function: csv_to_json
/// The function first determines the appropriate headings and file extension based on the
/// database_type parameter. It uses a match statement to select the correct headings and extension
/// for Mentions, GKG, or Export database types.
///
/// Converts a CSV file to a JSON object based on the database type.
///
/// This function reads a CSV file for the given date and database type, and converts it into a JSON object.
///
/// # Arguments
///
/// * `date_fetch` - A string slice that holds the date for which the CSV file is to be fetched.
/// * `database_type` - The type of database (Mentions, GKG, or Export) to determine the headings to be used.
///
/// # Returns
///
/// A `Result` containing a vector of JSON objects or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened or if there is an issue reading the CSV records.
pub async fn csv_to_json(
    date_fetch: &str,
    database_type: DatabaseType,
) -> Result<Vec<Object>, CsvToJsonError> {
    // region Determine Headings and Extension
    let (headings, extension) = match database_type {
        DatabaseType::Mentions => ((*MENTIONS_HEADINGS).clone(), ".mentions.csv"),
        DatabaseType::GKG => ((*GKG_HEADINGS).clone(), ".gkg.csv"),
        DatabaseType::Export => ((*EXPORT_HEADINGS).clone(), ".export.csv"),
    };
    // endregion

    // region Construct File Path
    let full_file_name = format!("{}{}", date_fetch, extension);

    let download_path = DOWNLOAD_PATH_FOLDER.lock().await;
    let csv_file_path = format!(
        "{}{}",
        download_path
            .to_str()
            .ok_or(CsvToJsonError::PathFolderError)?,
        full_file_name
    );
    // endregion

    // region Open CSV File
    let file = File::open(csv_file_path)?;
    // endregion

    // region Create CSV Reader
    let mut reader = ReaderBuilder::new().flexible(true).from_reader(file);
    // endregion

    // region Read CSV Records
    let mut table: Vec<Vec<((String, JsonRustTypes), String)>> = Vec::new();
    let mut objects: Vec<Object> = Vec::new();

    for res in reader.records() {
        let unwrapped = res?;
        let string: Vec<String> = unwrapped
            .as_slice()
            .split("\t")
            .map(|s| s.to_string())
            .collect();
        let zipped: Vec<((String, JsonRustTypes), String)> = string
            .iter()
            .zip(headings.iter())
            .map(|(value, key)| (key.clone(), value.clone()))
            .collect();
        table.push(zipped);
    }
    // endregion

    // region Convert Records to JSON
    for idem in table {
        let mut json_object: Result<Object, Box<dyn Error>> = Ok(Object::new());
        for item in idem {
            json_object = Ok(string_to_json_value(json_object.unwrap(), item.clone())
                .map_err(|e| CsvToJsonError::JsonConversionError(e.to_string()))?);
        }
        objects.push(json_object.unwrap())
    }
    // endregion

    // Return the JSON objects
    Ok(objects)
}
// endregion
// endregion
