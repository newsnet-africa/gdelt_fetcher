// region Modules and Imports

use crate::gdelt_fetcher::DOWNLOAD_PATH_FOLDER;
use crate::utils::types::DatabaseType;
use csv::ReaderBuilder;
use models::models::gdelt::gkg::GlobalKnowledgeGraph;
use models::models::gdelt::mentions::Mentions;
use models::models::gdelt::{DatabaseTableEnum, GDELTObject};
use std::fs::File;
use thiserror::Error;
// endregion

// region Custom error types
/// Custom error types for the `csv_to_struct` function.
#[derive(Error, Debug)]
pub enum CsvToStructError {
    /// Error for file opening issues.
    #[error("Failed to open file: {0}")]
    FileOpenError(#[from] std::io::Error),

    /// Error for CSV reading issues.
    #[error("Failed to read CSV record: {0}")]
    CsvReadError(#[from] csv::Error),

    /// Error for path folder issues.
    #[error("Path Folder Error")]
    PathFolderError,
}
// endregion

// region Function: csv_to_struct
/// The function starts by defining custom error types using the thiserror crate.
/// These errors include FileOpenError, CsvReadError, and PathFolderError,
/// which handle issues related to file operations, CSV reading, and path folder problems, respectively.
///
/// Converts a CSV file to a corresponding struct based on the database type.
///
/// This function reads a CSV file for the given date and database type, and converts it into a corresponding struct.
///
/// # Arguments
///
/// * `date_fetch` - A string slice that holds the date for which the CSV file is to be fetched.
/// * `database_type` - The type of database (Mentions, GKG, or Export) to determine the struct to be used.
///
/// # Returns
///
/// A `Result` containing a `DatabaseTableEnum` with the populated struct, or an error if the operation fails.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened or if there is an issue reading the CSV records.
pub async fn csv_to_structs(
    date_fetch: &str,
    database_type: DatabaseType,
) -> Result<Vec<DatabaseTableEnum>, CsvToStructError> {
    // Determine the initial object and file extension based on the database type
    let (mut r_object, extension) = match database_type {
        DatabaseType::Mentions => (DatabaseTableEnum::Mentions(None), ".mentions.csv"),
        DatabaseType::GKG => (DatabaseTableEnum::GlobalKnowledgeGraph(None), ".gkg.csv"),
        DatabaseType::Export => (DatabaseTableEnum::Event(None), ".export.csv"),
    };

    // Construct the full file name using the date and extension
    let full_file_name = format!("{}{}", date_fetch, extension);

    // Lock the download path folder and construct the full path to the CSV file
    let download_path = DOWNLOAD_PATH_FOLDER.lock().await;
    let csv_file_path = format!(
        "{}/{}",
        download_path
            .to_str()
            .ok_or(CsvToStructError::PathFolderError)?,
        full_file_name
    );
    
    // Open the CSV file
    let file = File::open(csv_file_path)?;
    
    // Create a CSV reader with flexible settings
    let mut reader = ReaderBuilder::new().flexible(false).from_reader(file);
    
    // Create a vector to store the records
    let mut records_vec = Vec::new();
    
    // Iterate over the CSV records and populate the object
    for record in reader.records() {
        match record {
            Ok(ok_stringrecord) => {
                let sliced = ok_stringrecord.as_slice();
                match r_object.clone() {
                    DatabaseTableEnum::Mentions(_) => records_vec
                        .push(DatabaseTableEnum::Mentions(Mentions::from_strings(sliced))),
                    DatabaseTableEnum::GlobalKnowledgeGraph(_) => {
                        records_vec.push(DatabaseTableEnum::GlobalKnowledgeGraph(
                            GlobalKnowledgeGraph::from_strings(sliced),
                        ))
                    }
                    DatabaseTableEnum::Event(_) => records_vec
                        .push(DatabaseTableEnum::Mentions(Mentions::from_strings(sliced))),
                }
            }
            Err(_) => continue,
        }
    }
    
    // Return the vector of populated objects
    Ok(records_vec)
}
// endregion
