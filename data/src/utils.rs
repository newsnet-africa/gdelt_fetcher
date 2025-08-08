//! Utility functions for GDELT data processing
//!
//! This module contains common utility functions used across the data fetching and processing
//! pipeline, including file management, CSV parsing helpers, and directory operations.

use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration for temporary directories and file paths
#[derive(Debug, Clone)]
pub struct TempDirectoryConfig {
    pub temp_dir: PathBuf,
    pub output_dir: PathBuf,
    pub data_type: String,
}

impl TempDirectoryConfig {
    /// Create a new temporary directory configuration
    pub fn new<P: AsRef<Path>>(
        base_temp_dir: P,
        base_output_dir: P,
        data_type: &str,
    ) -> Result<Self> {
        let temp_dir = base_temp_dir.as_ref().join(data_type);
        let output_dir = base_output_dir.as_ref().join(data_type);

        // Create directories if they don't exist
        fs::create_dir_all(&temp_dir)
            .with_context(|| format!("Failed to create temp directory: {:?}", temp_dir))?;
        fs::create_dir_all(&output_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

        Ok(Self {
            temp_dir,
            output_dir,
            data_type: data_type.to_string(),
        })
    }

    /// Get the download path for ZIP files
    pub fn zip_download_path(&self) -> PathBuf {
        self.temp_dir.join("latest_download.zip")
    }

    /// Get a specific file path in the temp directory
    pub fn temp_file_path(&self, filename: &str) -> PathBuf {
        self.temp_dir.join(filename)
    }

    /// Get a specific file path in the output directory
    pub fn output_file_path(&self, filename: &str) -> PathBuf {
        self.output_dir.join(filename)
    }
}

/// File finder utility for locating specific files in directories
pub struct FileFinder;

impl FileFinder {
    /// Find existing CSV files in a directory with custom filtering
    pub fn find_csv_with_filter<F>(output_dir: &Path, filter_fn: F) -> Option<PathBuf>
    where
        F: Fn(&str) -> bool,
    {
        Self::find_files_with_extension_and_filter(output_dir, "csv", filter_fn)
    }

    /// Find files with a specific extension and custom filtering
    pub fn find_files_with_extension_and_filter<F>(
        directory: &Path,
        extension: &str,
        filter_fn: F,
    ) -> Option<PathBuf>
    where
        F: Fn(&str) -> bool,
    {
        fs::read_dir(directory)
            .ok()?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    if path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map_or(false, |s| s.eq_ignore_ascii_case(extension))
                    {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
            .find(|path| {
                path.file_name()
                    .and_then(|f| f.to_str())
                    .map_or(false, &filter_fn)
            })
    }

    /// Find all files with a specific pattern in the filename
    pub fn find_files_containing(directory: &Path, pattern: &str) -> Result<Vec<PathBuf>> {
        let mut matching_files = Vec::new();

        if !directory.exists() {
            return Err(anyhow::anyhow!("Directory does not exist: {:?}", directory));
        }

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.to_lowercase().contains(&pattern.to_lowercase()) {
                        matching_files.push(path);
                    }
                }
            }
        }

        Ok(matching_files)
    }

    /// Find the most recent file in a directory based on modification time
    pub fn find_most_recent_file(directory: &Path) -> Result<Option<PathBuf>> {
        let mut most_recent: Option<(PathBuf, std::time::SystemTime)> = None;

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let metadata = entry.metadata()?;
                let modified = metadata.modified()?;

                match &most_recent {
                    None => most_recent = Some((path, modified)),
                    Some((_, latest_time)) => {
                        if modified > *latest_time {
                            most_recent = Some((path, modified));
                        }
                    }
                }
            }
        }

        Ok(most_recent.map(|(path, _)| path))
    }
}

/// File cleanup utilities
pub struct FileCleanup;

impl FileCleanup {
    /// Clean up a list of files, ignoring errors for non-existent files
    pub fn cleanup_files(files: &[&Path]) -> Result<()> {
        let mut errors = Vec::new();

        for file in files {
            if file.exists() {
                if let Err(e) = fs::remove_file(file) {
                    errors.push(format!("Failed to remove {:?}: {}", file, e));
                } else {
                    debug!("Successfully removed file: {:?}", file);
                }
            }
        }

        if !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Failed to clean up some files: {}",
                errors.join(", ")
            ));
        }

        Ok(())
    }

    /// Clean up all files in a directory with a specific pattern
    pub fn cleanup_files_matching(directory: &Path, pattern: &str) -> Result<()> {
        let matching_files = FileFinder::find_files_containing(directory, pattern)?;
        let file_refs: Vec<&Path> = matching_files.iter().map(|p| p.as_path()).collect();
        Self::cleanup_files(&file_refs)
    }

    /// Clean up temporary directory and all its contents
    pub fn cleanup_temp_directory(temp_dir: &Path) -> Result<()> {
        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir)
                .with_context(|| format!("Failed to remove temp directory: {:?}", temp_dir))?;
            debug!("Successfully removed temp directory: {:?}", temp_dir);
        }
        Ok(())
    }
}

/// CSV parsing utilities and helpers
pub struct CsvUtils;

impl CsvUtils {
    /// Validate that a CSV file can be read properly
    pub fn validate_csv_format(file_path: &Path) -> Result<bool> {
        use csv::Reader;

        let file = fs::File::open(file_path)
            .with_context(|| format!("Failed to open file for validation: {:?}", file_path))?;
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

    /// Count the number of records in a CSV file without loading them into memory
    pub fn count_csv_records(file_path: &Path, has_headers: bool) -> Result<usize> {
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .delimiter(b'\t')
            .from_reader(file);

        let mut count = 0;
        for result in reader.records() {
            match result {
                Ok(_) => count += 1,
                Err(e) => {
                    warn!("Error reading record {}: {}", count + 1, e);
                    // Continue counting even if some records fail
                }
            }
        }

        Ok(count)
    }

    /// Get basic statistics about a CSV file
    pub fn get_csv_stats(file_path: &Path, has_headers: bool) -> Result<CsvStats> {
        let file = fs::File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .delimiter(b'\t')
            .from_reader(file);

        let mut total_records = 0;
        let mut max_fields = 0;
        let mut min_fields = usize::MAX;
        let mut field_count_distribution: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();

        for result in reader.records() {
            match result {
                Ok(record) => {
                    total_records += 1;
                    let field_count = record.len();
                    max_fields = max_fields.max(field_count);
                    min_fields = min_fields.min(field_count);
                    *field_count_distribution.entry(field_count).or_insert(0) += 1;
                }
                Err(_) => {
                    // Count errors but don't stop processing
                }
            }
        }

        if min_fields == usize::MAX {
            min_fields = 0;
        }

        Ok(CsvStats {
            total_records,
            max_fields,
            min_fields,
            field_count_distribution,
        })
    }
}

/// Statistics about a CSV file
#[derive(Debug, Clone)]
pub struct CsvStats {
    pub total_records: usize,
    pub max_fields: usize,
    pub min_fields: usize,
    pub field_count_distribution: std::collections::HashMap<usize, usize>,
}

impl CsvStats {
    /// Check if the CSV has consistent field counts
    pub fn has_consistent_field_count(&self) -> bool {
        self.field_count_distribution.len() <= 1
    }

    /// Get the most common field count
    pub fn most_common_field_count(&self) -> Option<usize> {
        self.field_count_distribution
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(fields, _)| *fields)
    }

    /// Check if the field count matches expected count for a given percentage of records
    pub fn field_count_consistency_percentage(&self, expected_fields: usize) -> f64 {
        if self.total_records == 0 {
            return 0.0;
        }

        let matching_records = self
            .field_count_distribution
            .get(&expected_fields)
            .unwrap_or(&0);

        (*matching_records as f64 / self.total_records as f64) * 100.0
    }
}

/// Retry utilities for network operations
pub struct RetryUtils;

impl RetryUtils {
    /// Execute an async operation with exponential backoff retry
    pub async fn retry_with_backoff<F, Fut, T>(
        operation: F,
        max_retries: usize,
        base_delay_ms: u64,
    ) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;

        for attempt in 1..=max_retries {
            debug!("Attempt {} of {}", attempt, max_retries);

            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        info!("Operation succeeded on attempt {}", attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}", attempt, e);
                    last_error = Some(e);

                    if attempt < max_retries {
                        let delay_ms = base_delay_ms * 2_u64.pow(attempt as u32 - 1);
                        debug!("Waiting {} ms before retry...", delay_ms);
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("Operation failed after {} attempts", max_retries)))
    }

    /// Execute an operation with simple retry (no backoff)
    pub async fn retry_simple<F, Fut, T>(operation: F, max_retries: usize) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        Self::retry_with_backoff(operation, max_retries, 1000).await
    }
}

/// URL validation utilities
pub struct UrlUtils;

impl UrlUtils {
    /// Validate that a URL is well-formed and reachable
    pub async fn validate_url_reachable(url: &str) -> Result<bool> {
        let url = url::Url::parse(url).with_context(|| format!("Invalid URL format: {}", url))?;

        match reqwest::get(url).await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                debug!("URL not reachable: {}", e);
                Ok(false)
            }
        }
    }

    /// Extract the filename from a URL
    pub fn extract_filename_from_url(url: &str) -> Result<String> {
        let url = url::Url::parse(url)?;
        let path_segments: Vec<&str> = url.path_segments().into_iter().flatten().collect();

        path_segments
            .last()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("No filename found in URL: {}", url))
    }

    /// Check if a URL points to a compressed file
    pub fn is_compressed_url(url: &str) -> bool {
        let compressed_extensions = [".zip", ".gz", ".tar.gz", ".bz2", ".xz"];
        let url_lower = url.to_lowercase();
        compressed_extensions
            .iter()
            .any(|ext| url_lower.ends_with(ext))
    }
}

/// Common constants used across the GDELT fetching system
pub mod constants {
    /// Default timeout for HTTP requests in seconds
    pub const DEFAULT_HTTP_TIMEOUT_SECS: u64 = 30;

    /// Default number of retry attempts for failed operations
    pub const DEFAULT_MAX_RETRIES: usize = 3;

    /// Default base delay in milliseconds for retry backoff
    pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

    /// Expected field counts for GDELT table types
    pub mod field_counts {
        /// Expected number of fields in GDELT Events table
        pub const EVENTS: usize = 61;

        /// Expected number of fields in GDELT Mentions table
        pub const MENTIONS: usize = 16;

        /// Expected number of fields in GDELT GKG table
        pub const GKG: usize = 27;
    }

    /// File extensions used by GDELT
    pub mod extensions {
        pub const CSV_UPPERCASE: &str = "CSV.zip";
        pub const CSV_LOWERCASE: &str = "csv.zip";
        pub const JSON_COMPRESSED: &str = "json.zip";
        pub const JSON_UNCOMPRESSED: &str = "json";
    }
}

/// Logging utilities for the data processing pipeline
pub struct LoggingUtils;

impl LoggingUtils {
    /// Log a summary of processing results
    pub fn log_processing_summary(
        data_type: &str,
        total_processed: usize,
        successful: usize,
        failed: usize,
    ) {
        let success_rate = if total_processed > 0 {
            (successful as f64 / total_processed as f64) * 100.0
        } else {
            0.0
        };

        info!(
            "{} processing completed: {}/{} successful ({:.1}%), {} failed",
            data_type, successful, total_processed, success_rate, failed
        );

        if failed > 0 && success_rate < 90.0 {
            warn!(
                "High failure rate for {}: {:.1}% success rate may indicate data quality issues",
                data_type, success_rate
            );
        }
    }

    /// Log the first N fields of a CSV record for debugging
    pub fn log_record_sample(record: &csv::StringRecord, n: usize, record_type: &str) {
        let fields: Vec<&str> = record.iter().take(n).collect();
        debug!(
            "Sample {} record (first {} fields): {:?}",
            record_type, n, fields
        );
    }

    /// Initialize logging for tests with a specific level
    #[cfg(test)]
    pub fn init_test_logging(level: log::LevelFilter) {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(level)
            .try_init();
    }

    /// Initialize default debug-level logging for tests
    #[cfg(test)]
    pub fn init_test_logging_debug() {
        Self::init_test_logging(log::LevelFilter::Debug);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_temp_directory_config() {
        let config = TempDirectoryConfig::new("./tmp", "./data", "test").unwrap();

        assert!(config.temp_dir.ends_with("test"));
        assert!(config.output_dir.ends_with("test"));
        assert_eq!(config.data_type, "test");

        // Cleanup
        let _ = fs::remove_dir_all("./tmp/test");
        let _ = fs::remove_dir_all("./data/test");
    }

    #[test]
    fn test_file_finder() {
        // Create test directory and files
        let test_dir = "./tmp/test_finder";
        fs::create_dir_all(test_dir).unwrap();

        fs::write(format!("{}/test.csv", test_dir), "test").unwrap();
        fs::write(format!("{}/mentions.csv", test_dir), "test").unwrap();
        fs::write(format!("{}/other.txt", test_dir), "test").unwrap();

        // Test finding CSV files
        let csv_file = FileFinder::find_csv_with_filter(Path::new(test_dir), |filename| {
            filename.contains("mentions")
        });
        assert!(csv_file.is_some());

        // Test finding files containing pattern
        let files = FileFinder::find_files_containing(Path::new(test_dir), "test").unwrap();
        assert_eq!(files.len(), 1); // test.csv

        // Cleanup
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_file_cleanup() {
        // Create test files
        let test_dir = "./tmp/test_cleanup";
        fs::create_dir_all(test_dir).unwrap();

        let file1 = format!("{}/file1.txt", test_dir);
        let file2 = format!("{}/file2.txt", test_dir);
        fs::write(&file1, "test").unwrap();
        fs::write(&file2, "test").unwrap();

        // Test cleanup
        let files = [Path::new(&file1), Path::new(&file2)];
        FileCleanup::cleanup_files(&files).unwrap();

        assert!(!Path::new(&file1).exists());
        assert!(!Path::new(&file2).exists());

        // Cleanup directory
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_url_utils() {
        // Test filename extraction
        let filename =
            UrlUtils::extract_filename_from_url("https://example.com/path/file.csv.zip").unwrap();
        assert_eq!(filename, "file.csv.zip");

        // Test compression detection
        assert!(UrlUtils::is_compressed_url("https://example.com/file.zip"));
        assert!(!UrlUtils::is_compressed_url("https://example.com/file.csv"));
    }

    #[test]
    fn test_csv_stats() {
        let stats = CsvStats {
            total_records: 100,
            max_fields: 27,
            min_fields: 27,
            field_count_distribution: [(27, 100)].into_iter().collect(),
        };

        assert!(stats.has_consistent_field_count());
        assert_eq!(stats.most_common_field_count(), Some(27));
        assert_eq!(stats.field_count_consistency_percentage(27), 100.0);
    }
}
