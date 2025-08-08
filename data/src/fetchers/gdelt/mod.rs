use std::fs;
use std::path::PathBuf;

pub mod event_table_fetcher;
pub mod gdelt_fetcher;
pub mod gkg_table_fetcher;
pub mod mention_table_fetcher;

// Re-export main types for easier access
pub use gdelt_fetcher::{
    CsvExtension, FileExtension, GdeltFetcher, GdeltFileEntry, GdeltUrlBuilder, GdeltVersion,
    JsonExtension, TableType, TableTypeConfig,
};

// Re-export table fetchers
pub use event_table_fetcher::{EventTableFetcher, EventTableIterator};
pub use gkg_table_fetcher::{GKGTableFetcher, GKGTableIterator};
pub use mention_table_fetcher::{MentionTableFetcher, MentionTableIterator};

/// Find files in a directory that contain a specific string and have a specific file extension
pub fn find_files_with_string_and_type(
    directory: &std::path::Path,
    search_string: &str,
    file_extension: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut matching_files = Vec::new();

    if !directory.exists() {
        return Err(anyhow::anyhow!("Directory does not exist: {:?}", directory));
    }

    if !directory.is_dir() {
        return Err(anyhow::anyhow!("Path is not a directory: {:?}", directory));
    }

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            // Check if it has the correct extension and contains the search string
            if path.extension() == Some(std::ffi::OsStr::new(file_extension))
                && file_name
                    .to_lowercase()
                    .contains(&search_string.to_lowercase())
            {
                matching_files.push(path);
            }
        }
    }

    Ok(matching_files)
}

/// Find files with string and type, plus CSV validation for CSV files
pub fn find_files_with_string_and_type_validated(
    directory: &std::path::Path,
    search_string: &str,
    file_extension: &str,
) -> anyhow::Result<Vec<PathBuf>> {
    let files = find_files_with_string_and_type(directory, search_string, file_extension)?;

    // If the extension is CSV, validate the files
    if file_extension == "csv" {
        let mut validated_files = Vec::new();
        for file_path in files {
            if verify_csv_format(&file_path).unwrap_or(false) {
                validated_files.push(file_path);
            }
        }
        Ok(validated_files)
    } else {
        Ok(files)
    }
}

/// Verifies that a CSV file is valid by attempting to read its headers and first record
fn verify_csv_format(file_path: &std::path::Path) -> anyhow::Result<bool> {
    use csv::Reader;

    let file = fs::File::open(file_path)?;
    let mut reader = Reader::from_reader(file);

    // Try to read the headers to verify CSV structure
    let _headers = reader.headers()?;

    // Try to read at least one record to ensure the CSV is properly formatted
    let mut record = csv::StringRecord::new();
    match reader.read_record(&mut record) {
        Ok(true) => Ok(true),  // Successfully read a record
        Ok(false) => Ok(true), // Empty file but valid headers
        Err(e) => Err(anyhow::anyhow!("Invalid CSV format: {}", e)),
    }
}

#[cfg(test)]
pub mod test_utils {
    /// Initialize logging for tests with debug level output
    /// This function can be called from any test to enable comprehensive logging
    pub fn init_test_logging() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }

    /// Initialize logging for tests with a custom log level
    pub fn init_test_logging_with_level(level: log::LevelFilter) {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(level)
            .try_init();
    }
}
