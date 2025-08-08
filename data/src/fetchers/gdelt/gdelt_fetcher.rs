use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, NaiveDateTime};
use md5;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use url::Url;

use crate::fetchers::{DataSource, HttpDatatypes, RawDataFetcher};

/// GDELT version enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GdeltVersion {
    V2,
    V3,
}

impl GdeltVersion {
    /// Get the URL path component for this version
    pub fn as_url_component(&self) -> &'static str {
        match self {
            GdeltVersion::V2 => "gdeltv2",
            GdeltVersion::V3 => "gdeltv3",
        }
    }

    /// Get the base URL for this version
    pub fn base_url(&self) -> String {
        format!("http://data.gdeltproject.org/{}", self.as_url_component())
    }

    /// Get the lastupdate.txt URL for this version
    pub fn lastupdate_url(&self) -> String {
        format!("{}/lastupdate.txt", self.base_url())
    }

    /// Get the masterfilelist.txt URL for this version
    pub fn masterfilelist_url(&self) -> String {
        format!("{}/masterfilelist.txt", self.base_url())
    }
}

impl std::str::FromStr for GdeltVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "v2" | "2" | "gdeltv2" => Ok(GdeltVersion::V2),
            "v3" | "3" | "gdeltv3" => Ok(GdeltVersion::V3),
            _ => Err(anyhow!("Invalid GDELT version: {}", s)),
        }
    }
}

/// GDELT table types with their corresponding file extensions and identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    Export,
    Mentions,
    Gkg,
}

impl TableType {
    /// Get the file identifier string used in GDELT URLs
    pub fn as_file_identifier(&self) -> &'static str {
        match self {
            TableType::Export => "export",
            TableType::Mentions => "mentions",
            TableType::Gkg => "gkg",
        }
    }

    /// Get the appropriate file extension for this table type
    pub fn file_extension(&self) -> FileExtension {
        match self {
            TableType::Export => FileExtension::Csv(CsvExtension::Upper),
            TableType::Mentions => FileExtension::Csv(CsvExtension::Upper),
            TableType::Gkg => FileExtension::Csv(CsvExtension::Lower),
        }
    }

    /// Create a table type with a custom file extension
    pub fn with_custom_extension(&self, extension: FileExtension) -> TableTypeConfig {
        TableTypeConfig {
            table_type: *self,
            file_extension: extension,
        }
    }

    /// Create an export table with JSON extension
    pub fn export_json() -> TableTypeConfig {
        TableType::Export.with_custom_extension(FileExtension::Json(JsonExtension::Compressed))
    }

    /// Create a mentions table with JSON extension
    pub fn mentions_json() -> TableTypeConfig {
        TableType::Mentions.with_custom_extension(FileExtension::Json(JsonExtension::Compressed))
    }

    /// Create a GKG table with JSON extension
    pub fn gkg_json() -> TableTypeConfig {
        TableType::Gkg.with_custom_extension(FileExtension::Json(JsonExtension::Compressed))
    }
}

/// Configuration for a table type with custom file extension
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableTypeConfig {
    pub table_type: TableType,
    pub file_extension: FileExtension,
}

impl TableTypeConfig {
    pub fn new(table_type: TableType, file_extension: FileExtension) -> Self {
        Self {
            table_type,
            file_extension,
        }
    }

    /// Get the file identifier string used in GDELT URLs
    pub fn as_file_identifier(&self) -> &'static str {
        self.table_type.as_file_identifier()
    }

    /// Get the configured file extension
    pub fn file_extension(&self) -> FileExtension {
        self.file_extension
    }
}

impl std::str::FromStr for TableType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "export" => Ok(TableType::Export),
            "mentions" => Ok(TableType::Mentions),
            "gkg" => Ok(TableType::Gkg),
            _ => Err(anyhow!("Invalid table type: {}", s)),
        }
    }
}

/// CSV file extension variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CsvExtension {
    Upper, // .CSV.zip
    Lower, // .csv.zip
}

impl CsvExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            CsvExtension::Upper => "CSV.zip",
            CsvExtension::Lower => "csv.zip",
        }
    }
}

/// JSON file extension variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonExtension {
    Compressed,   // .json.zip
    Uncompressed, // .json
}

impl JsonExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            JsonExtension::Compressed => "json.zip",
            JsonExtension::Uncompressed => "json",
        }
    }
}

/// File extensions used by GDELT files with nested CSV and JSON variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileExtension {
    Csv(CsvExtension),
    Json(JsonExtension),
}

impl FileExtension {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileExtension::Csv(csv_ext) => csv_ext.as_str(),
            FileExtension::Json(json_ext) => json_ext.as_str(),
        }
    }

    /// Check if this extension represents a compressed file
    pub fn is_compressed(&self) -> bool {
        match self {
            FileExtension::Csv(_) => true, // CSV files are always zipped
            FileExtension::Json(JsonExtension::Compressed) => true,
            FileExtension::Json(JsonExtension::Uncompressed) => false,
        }
    }

    /// Get the uncompressed file extension
    pub fn uncompressed_extension(&self) -> &'static str {
        match self {
            FileExtension::Csv(CsvExtension::Upper) => "CSV",
            FileExtension::Csv(CsvExtension::Lower) => "csv",
            FileExtension::Json(_) => "json",
        }
    }
}

/// Represents a GDELT file entry with metadata
#[derive(Debug, Clone)]
pub struct GdeltFileEntry {
    pub size: u64,
    pub hash: String,
    pub url: String,
    pub table_type: TableType,
    pub timestamp: NaiveDateTime,
    pub is_translation: bool,
}

impl GdeltFileEntry {
    /// Parse a line from GDELT file list format
    pub fn parse_from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid line format: expected 3 parts, got {}",
                parts.len()
            ));
        }

        let size: u64 = parts[0]
            .parse()
            .with_context(|| format!("Failed to parse size: {}", parts[0]))?;
        let hash = parts[1].to_string();
        let url = parts[2].to_string();

        // Extract information from URL
        let url_path = url
            .split('/')
            .last()
            .ok_or_else(|| anyhow!("Invalid URL format: {}", url))?;

        // Parse timestamp and determine if it's a translation file
        let (timestamp_str, remainder) = if url_path.len() >= 14 {
            (&url_path[0..14], &url_path[14..])
        } else {
            return Err(anyhow!("Invalid filename format: {}", url_path));
        };

        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d%H%M%S")
            .with_context(|| format!("Failed to parse timestamp: {}", timestamp_str))?;

        let is_translation = remainder.starts_with(".translation.");
        let table_part = if is_translation {
            remainder.strip_prefix(".translation.").unwrap()
        } else {
            remainder.strip_prefix(".").unwrap_or(remainder)
        };

        // Determine table type from the remaining part
        let table_type = if table_part.starts_with("export.") {
            TableType::Export
        } else if table_part.starts_with("mentions.") {
            TableType::Mentions
        } else if table_part.starts_with("gkg.") {
            TableType::Gkg
        } else {
            return Err(anyhow!("Unknown table type in filename: {}", table_part));
        };

        Ok(GdeltFileEntry {
            size,
            hash,
            url,
            table_type,
            timestamp,
            is_translation,
        })
    }
}

/// Builder for constructing GDELT URLs
#[derive(Debug, Clone)]
pub struct GdeltUrlBuilder {
    version: GdeltVersion,
    timestamp: NaiveDateTime,
    table_config: TableTypeConfig,
    is_translation: bool,
}

impl GdeltUrlBuilder {
    pub fn new() -> Self {
        Self {
            version: GdeltVersion::V2,
            timestamp: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
            table_config: TableTypeConfig::new(
                TableType::Export,
                TableType::Export.file_extension(),
            ),
            is_translation: false,
        }
    }

    pub fn with_version(mut self, version: GdeltVersion) -> Self {
        self.version = version;
        self
    }

    pub fn with_timestamp(mut self, timestamp: NaiveDateTime) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_table_type(mut self, table_type: TableType) -> Self {
        self.table_config = TableTypeConfig::new(table_type, table_type.file_extension());
        self
    }

    pub fn with_table_config(mut self, table_config: TableTypeConfig) -> Self {
        self.table_config = table_config;
        self
    }

    pub fn with_translation(mut self, is_translation: bool) -> Self {
        self.is_translation = is_translation;
        self
    }

    /// Build the complete URL for the GDELT file
    pub fn build(self) -> Result<Url> {
        let timestamp_str = self.timestamp.format("%Y%m%d%H%M%S");
        let extension = self.table_config.file_extension().as_str();
        let table_identifier = self.table_config.as_file_identifier();
        let base_url = self.version.base_url();

        let filename = if self.is_translation {
            format!(
                "{}.translation.{}.{}",
                timestamp_str, table_identifier, extension
            )
        } else {
            format!("{}.{}.{}", timestamp_str, table_identifier, extension)
        };

        let full_url = format!("{}/{}", base_url, filename);
        Url::parse(&full_url).with_context(|| format!("Failed to parse URL: {}", full_url))
    }

    /// Build URL from a GdeltFileEntry
    pub fn from_entry(entry: &GdeltFileEntry, version: GdeltVersion) -> Self {
        Self::new()
            .with_version(version)
            .with_timestamp(entry.timestamp)
            .with_table_type(entry.table_type)
            .with_translation(entry.is_translation)
    }

    /// Build URL from a GdeltFileEntry with custom table config
    pub fn from_entry_with_config(
        entry: &GdeltFileEntry,
        table_config: TableTypeConfig,
        version: GdeltVersion,
    ) -> Self {
        Self::new()
            .with_version(version)
            .with_timestamp(entry.timestamp)
            .with_table_config(table_config)
            .with_translation(entry.is_translation)
    }
}

/// Main GDELT fetcher implementation
pub struct GdeltFetcher {
    output_dir: PathBuf,
    temp_dir: PathBuf,
    version: GdeltVersion,
}

impl GdeltFetcher {
    pub fn new<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<Self> {
        Self::new_with_version(output_dir, temp_dir, GdeltVersion::V2)
    }

    /// Create a new GDELT v2 fetcher
    pub fn new_v2<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<Self> {
        Self::new_with_version(output_dir, temp_dir, GdeltVersion::V2)
    }

    /// Create a new GDELT v3 fetcher
    pub fn new_v3<P: AsRef<Path>>(output_dir: P, temp_dir: P) -> Result<Self> {
        Self::new_with_version(output_dir, temp_dir, GdeltVersion::V3)
    }

    pub fn new_with_version<P: AsRef<Path>>(
        output_dir: P,
        temp_dir: P,
        version: GdeltVersion,
    ) -> Result<Self> {
        let output_dir = output_dir.as_ref().to_path_buf();
        let temp_dir = temp_dir.as_ref().to_path_buf();

        // Create directories if they don't exist
        fs::create_dir_all(&output_dir)?;
        fs::create_dir_all(&temp_dir)?;

        Ok(Self {
            output_dir,
            temp_dir,
            version,
        })
    }

    pub fn version(&self) -> GdeltVersion {
        self.version
    }

    pub fn set_version(&mut self, version: GdeltVersion) {
        self.version = version;
    }

    /// Fetch the latest file list from GDELT
    pub async fn fetch_latest_file_list(&self) -> Result<Vec<GdeltFileEntry>> {
        let url = self.version.lastupdate_url();
        let response = reqwest::get(&url).await?;
        let content = response.text().await?;

        let mut entries = Vec::new();
        for line in content.lines() {
            if !line.trim().is_empty() {
                match GdeltFileEntry::parse_from_line(line) {
                    Ok(entry) => entries.push(entry),
                    Err(e) => log::warn!("Failed to parse line '{}': {}", line, e),
                }
            }
        }

        Ok(entries)
    }

    /// Fetch the master file list from GDELT
    pub async fn fetch_master_file_list(&self) -> Result<Vec<GdeltFileEntry>> {
        let url = self.version.masterfilelist_url();
        let response = reqwest::get(&url).await?;
        let content = response.text().await?;

        let mut entries = Vec::new();
        for line in content.lines() {
            if !line.trim().is_empty() {
                match GdeltFileEntry::parse_from_line(line) {
                    Ok(entry) => entries.push(entry),
                    Err(e) => log::warn!("Failed to parse line '{}': {}", line, e),
                }
            }
        }

        Ok(entries)
    }

    /// Find entries matching specific criteria
    pub fn find_entries_by_criteria<'a>(
        &self,
        entries: &'a [GdeltFileEntry],
        table_type: Option<TableType>,
        is_translation: Option<bool>,
        timestamp: Option<NaiveDateTime>,
    ) -> Vec<&'a GdeltFileEntry> {
        entries
            .iter()
            .filter(|entry| {
                if let Some(tt) = table_type {
                    if entry.table_type != tt {
                        return false;
                    }
                }
                if let Some(is_trans) = is_translation {
                    if entry.is_translation != is_trans {
                        return false;
                    }
                }
                if let Some(ts) = timestamp {
                    if entry.timestamp != ts {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Download and verify a file with hash validation
    pub async fn download_and_verify_file(&self, entry: &GdeltFileEntry) -> Result<PathBuf> {
        let filename = entry
            .url
            .split('/')
            .last()
            .ok_or_else(|| anyhow!("Invalid URL: {}", entry.url))?;
        let _zip_path = self.temp_dir.join(filename);

        // Download the file
        log::info!("Downloading {} from {}", filename, entry.url);
        let response = reqwest::get(&entry.url).await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to download file: HTTP {}",
                response.status()
            ));
        }

        let content = response.bytes().await?;

        // Verify file size
        if content.len() as u64 != entry.size {
            return Err(anyhow!(
                "File size mismatch: expected {}, got {}",
                entry.size,
                content.len()
            ));
        }

        // Verify hash
        let computed_hash = format!("{:x}", md5::compute(&content));

        if computed_hash != entry.hash {
            return Err(anyhow!(
                "Hash mismatch: expected {}, got {}",
                entry.hash,
                computed_hash
            ));
        }

        // Write to file (use appropriate extension)
        let file_extension = entry.table_type.file_extension();
        let final_filename = if file_extension.is_compressed() {
            filename.to_string()
        } else {
            // For uncompressed files, ensure correct extension
            let base_name = filename
                .strip_suffix(&format!(".{}", file_extension.as_str()))
                .unwrap_or(filename);
            format!("{}.{}", base_name, file_extension.uncompressed_extension())
        };

        let final_path = self.temp_dir.join(&final_filename);
        let mut file = tokio::fs::File::create(&final_path).await?;
        file.write_all(&content).await?;
        file.sync_all().await?;

        log::info!("Successfully downloaded and verified: {}", final_filename);
        Ok(final_path)
    }

    /// Extract or copy data file and return path to the data file (CSV or JSON)
    pub async fn extract_or_copy_file(
        &self,
        file_path: &Path,
        expected_extension: FileExtension,
    ) -> Result<PathBuf> {
        if expected_extension.is_compressed() {
            // Handle compressed files (ZIP archives)
            let file = fs::File::open(file_path)?;
            let mut archive = zip::ZipArchive::new(file)?;

            // Determine expected uncompressed extension
            let expected_ext = expected_extension.uncompressed_extension();

            // Find a supported data file in the archive
            let mut data_file_name = None;
            for i in 0..archive.len() {
                let file = archive.by_index(i)?;
                let name = file.name();
                if name.ends_with(&format!(".{}", expected_ext)) {
                    data_file_name = Some(name.to_string());
                    break;
                }
            }

            let data_file_name = data_file_name
                .ok_or_else(|| anyhow!("No {} file found in archive", expected_ext))?;

            // Extract the data file
            let mut data_file = archive.by_name(&data_file_name)?;
            let output_path = self.output_dir.join(&data_file_name);

            let mut output_file = fs::File::create(&output_path)?;
            std::io::copy(&mut data_file, &mut output_file)?;

            log::info!("Extracted {} file to: {:?}", expected_ext, output_path);
            Ok(output_path)
        } else {
            // Handle uncompressed files - just copy to output directory
            let filename = file_path
                .file_name()
                .ok_or_else(|| anyhow!("Invalid file path: {:?}", file_path))?;
            let output_path = self.output_dir.join(filename);

            fs::copy(file_path, &output_path)?;
            log::info!("Copied uncompressed file to: {:?}", output_path);
            Ok(output_path)
        }
    }

    /// Fetch, download, verify and extract a GDELT file
    pub async fn fetch_table_data(
        &self,
        table_type: TableType,
        is_translation: bool,
    ) -> Result<PathBuf> {
        // Get latest file list
        let entries = self.fetch_latest_file_list().await?;

        // Find matching entry
        let matching_entries =
            self.find_entries_by_criteria(&entries, Some(table_type), Some(is_translation), None);

        let entry = matching_entries.first().ok_or_else(|| {
            anyhow!(
                "No matching entry found for table_type: {:?}, is_translation: {}",
                table_type,
                is_translation
            )
        })?;

        // Download and verify
        let zip_path = self.download_and_verify_file(entry).await?;

        // Extract or copy with expected file extension
        let expected_extension = entry.table_type.file_extension();
        let data_path = self
            .extract_or_copy_file(&zip_path, expected_extension)
            .await?;

        // Clean up zip file
        if let Err(e) = fs::remove_file(&zip_path) {
            log::warn!("Failed to clean up zip file {:?}: {}", zip_path, e);
        }

        Ok(data_path)
    }

    /// Fetch data for a specific date and table type
    pub async fn fetch_table_data_by_date(
        &self,
        table_type: TableType,
        is_translation: bool,
        timestamp: NaiveDateTime,
    ) -> Result<PathBuf> {
        // Get master file list
        let entries = self.fetch_master_file_list().await?;

        // Find matching entry
        let matching_entries = self.find_entries_by_criteria(
            &entries,
            Some(table_type),
            Some(is_translation),
            Some(timestamp),
        );

        let entry = matching_entries.first().ok_or_else(|| {
            anyhow!(
                "No matching entry found for table_type: {:?}, is_translation: {}, timestamp: {}",
                table_type,
                is_translation,
                timestamp
            )
        })?;

        // Download and verify
        let zip_path = self.download_and_verify_file(entry).await?;

        // Extract or copy with expected file extension
        let expected_extension = entry.table_type.file_extension();
        let data_path = self
            .extract_or_copy_file(&zip_path, expected_extension)
            .await?;

        // Clean up zip file
        if let Err(e) = fs::remove_file(&zip_path) {
            log::warn!("Failed to clean up zip file {:?}: {}", zip_path, e);
        }

        Ok(data_path)
    }

    /// Fetch data with custom table configuration (latest)
    pub async fn fetch_table_data_with_config(
        &self,
        table_config: TableTypeConfig,
        is_translation: bool,
    ) -> Result<PathBuf> {
        // Get latest file list
        let entries = self.fetch_latest_file_list().await?;

        // Find matching entry
        let matching_entries = self.find_entries_by_criteria(
            &entries,
            Some(table_config.table_type),
            Some(is_translation),
            None,
        );

        let entry = matching_entries.first().ok_or_else(|| {
            anyhow!(
                "No matching entry found for table_type: {:?}, is_translation: {}",
                table_config.table_type,
                is_translation
            )
        })?;

        // Download and verify
        let file_path = self.download_and_verify_file(entry).await?;

        // Extract or copy with custom file extension
        let data_path = self
            .extract_or_copy_file(&file_path, table_config.file_extension())
            .await?;

        // Clean up downloaded file if it was compressed
        if table_config.file_extension().is_compressed() {
            if let Err(e) = fs::remove_file(&file_path) {
                log::warn!("Failed to clean up file {:?}: {}", file_path, e);
            }
        }

        Ok(data_path)
    }

    /// Fetch data with custom table configuration by date
    pub async fn fetch_table_data_by_date_with_config(
        &self,
        table_config: TableTypeConfig,
        is_translation: bool,
        timestamp: NaiveDateTime,
    ) -> Result<PathBuf> {
        // Get master file list
        let entries = self.fetch_master_file_list().await?;

        // Find matching entry
        let matching_entries = self.find_entries_by_criteria(
            &entries,
            Some(table_config.table_type),
            Some(is_translation),
            Some(timestamp),
        );

        let entry = matching_entries.first().ok_or_else(|| {
            anyhow!(
                "No matching entry found for table_type: {:?}, is_translation: {}, timestamp: {}",
                table_config.table_type,
                is_translation,
                timestamp
            )
        })?;

        // Download and verify
        let file_path = self.download_and_verify_file(entry).await?;

        // Extract or copy with custom file extension
        let data_path = self
            .extract_or_copy_file(&file_path, table_config.file_extension())
            .await?;

        // Clean up downloaded file if it was compressed
        if table_config.file_extension().is_compressed() {
            if let Err(e) = fs::remove_file(&file_path) {
                log::warn!("Failed to clean up file {:?}: {}", file_path, e);
            }
        }

        Ok(data_path)
    }

    /// Convenience function to fetch the latest export data
    pub async fn fetch_latest_export(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Export, false).await
    }

    /// Convenience function to fetch the latest mentions data
    pub async fn fetch_latest_mentions(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Mentions, false).await
    }

    /// Convenience function to fetch the latest GKG data
    pub async fn fetch_latest_gkg(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Gkg, false).await
    }

    /// Convenience function to fetch the latest translation export data
    pub async fn fetch_latest_export_translation(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Export, true).await
    }

    /// Convenience function to fetch the latest translation mentions data
    pub async fn fetch_latest_mentions_translation(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Mentions, true).await
    }

    /// Convenience function to fetch the latest translation GKG data
    pub async fn fetch_latest_gkg_translation(&self) -> Result<PathBuf> {
        self.fetch_table_data(TableType::Gkg, true).await
    }

    /// Convenience function to fetch export data by timestamp
    pub async fn fetch_export_by_date(&self, timestamp: NaiveDateTime) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Export, false, timestamp)
            .await
    }

    /// Convenience function to fetch mentions data by timestamp
    pub async fn fetch_mentions_by_date(&self, timestamp: NaiveDateTime) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Mentions, false, timestamp)
            .await
    }

    /// Convenience function to fetch GKG data by timestamp
    pub async fn fetch_gkg_by_date(&self, timestamp: NaiveDateTime) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Gkg, false, timestamp)
            .await
    }

    /// Convenience function to fetch translation export data by timestamp
    pub async fn fetch_export_translation_by_date(
        &self,
        timestamp: NaiveDateTime,
    ) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Export, true, timestamp)
            .await
    }

    /// Convenience function to fetch translation mentions data by timestamp
    pub async fn fetch_mentions_translation_by_date(
        &self,
        timestamp: NaiveDateTime,
    ) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Mentions, true, timestamp)
            .await
    }

    /// Convenience function to fetch translation GKG data by timestamp
    pub async fn fetch_gkg_translation_by_date(&self, timestamp: NaiveDateTime) -> Result<PathBuf> {
        self.fetch_table_data_by_date(TableType::Gkg, true, timestamp)
            .await
    }

    /// Get the latest entry for a specific table type and translation flag
    pub fn get_latest_entry_for_table<'a>(
        &self,
        entries: &'a [GdeltFileEntry],
        table_type: TableType,
        is_translation: bool,
    ) -> Option<&'a GdeltFileEntry> {
        entries
            .iter()
            .filter(|entry| {
                entry.table_type == table_type && entry.is_translation == is_translation
            })
            .max_by_key(|entry| entry.timestamp)
    }

    /// Check if a file already exists locally for the given criteria
    pub fn local_file_exists(
        &self,
        table_type: TableType,
        timestamp: NaiveDateTime,
    ) -> Option<PathBuf> {
        let expected_ext = table_type.file_extension().uncompressed_extension();

        if let Ok(entries) = fs::read_dir(&self.output_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        // Check if filename matches expected pattern
                        let timestamp_str = timestamp.format("%Y%m%d%H%M%S").to_string();
                        let table_id = table_type.as_file_identifier();

                        if filename.starts_with(&timestamp_str)
                            && filename.contains(table_id)
                            && filename.ends_with(expected_ext)
                        {
                            return Some(path);
                        }
                    }
                }
            }
        }
        None
    }

    /// Get all local files for a specific table type
    pub fn get_local_files_by_table(&self, table_type: TableType) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let expected_ext = table_type.file_extension().uncompressed_extension();
        let table_id = table_type.as_file_identifier();

        for entry in fs::read_dir(&self.output_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.contains(table_id) && filename.ends_with(expected_ext) {
                        files.push(path);
                    }
                }
            }
        }

        Ok(files)
    }

    /// Clean up old files, keeping only the most recent N files for each table type
    pub fn cleanup_old_files(&self, keep_count: usize) -> Result<()> {
        for table_type in [TableType::Export, TableType::Mentions, TableType::Gkg] {
            let mut files = self.get_local_files_by_table(table_type)?;

            // Sort by modification time (newest first)
            files.sort_by_key(|path| {
                fs::metadata(path)
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH)
            });
            files.reverse();

            // Remove excess files
            if files.len() > keep_count {
                for file_to_remove in files.iter().skip(keep_count) {
                    if let Err(e) = fs::remove_file(file_to_remove) {
                        log::warn!("Failed to remove file {:?}: {}", file_to_remove, e);
                    } else {
                        log::info!("Cleaned up old file: {:?}", file_to_remove);
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract GDELT version from a URL
    pub fn extract_version_from_url(url: &str) -> Option<GdeltVersion> {
        if url.contains("/gdeltv2/") {
            Some(GdeltVersion::V2)
        } else if url.contains("/gdeltv3/") {
            Some(GdeltVersion::V3)
        } else {
            None
        }
    }

    /// Check if a URL is compatible with this fetcher's version
    pub fn is_url_compatible(&self, url: &str) -> bool {
        Self::extract_version_from_url(url).map_or(false, |version| version == self.version)
    }

    /// Convert a URL from one version to another
    pub fn convert_url_version(url: &str, target_version: GdeltVersion) -> Option<String> {
        let current_version = Self::extract_version_from_url(url)?;
        if current_version == target_version {
            return Some(url.to_string());
        }

        let converted = url.replace(
            &format!("/{}/", current_version.as_url_component()),
            &format!("/{}/", target_version.as_url_component()),
        );

        Some(converted)
    }

    /// Validate that all entries in a list match this fetcher's version
    pub fn validate_entries_version(&self, entries: &[GdeltFileEntry]) -> Result<()> {
        for entry in entries {
            if !self.is_url_compatible(&entry.url) {
                return Err(anyhow!(
                    "Entry URL version mismatch: expected {}, found in URL: {}",
                    self.version.as_url_component(),
                    entry.url
                ));
            }
        }
        Ok(())
    }
}

impl RawDataFetcher for GdeltFetcher {
    type RawDataFileFormat = PathBuf;

    fn file_path(&self) -> Result<PathBuf> {
        Ok(self.output_dir.clone())
    }

    fn fetched_file_paths(&self) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();

        for entry in fs::read_dir(&self.output_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    // Check against all possible file extensions
                    if ext == "csv" || ext == "CSV" || ext == "json" {
                        paths.push(path);
                    }
                }
            }
        }

        Ok(paths)
    }

    fn fetch_latest_raw(&self, _datasource: DataSource) -> Result<Self::RawDataFileFormat> {
        // This is a sync version - for async use fetch_table_data
        Err(anyhow!("Use async fetch_table_data method instead"))
    }

    fn fetch_date_raw(
        &self,
        _date: DateTime<impl chrono::TimeZone>,
        _datasource: DataSource,
    ) -> Result<Self::RawDataFileFormat> {
        // This is a sync version - for async use fetch_table_data_by_date
        Err(anyhow!("Use async fetch_table_data_by_date method instead"))
    }

    fn source(&self) -> DataSource {
        DataSource::Http(HttpDatatypes::CSV)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetchers::gdelt::test_utils::init_test_logging;

    #[test]
    fn test_table_type_parsing() {
        init_test_logging();
        log::info!("Starting test_table_type_parsing");

        log::debug!("Testing 'export' parsing");
        assert_eq!("export".parse::<TableType>().unwrap(), TableType::Export);
        log::debug!("Testing 'mentions' parsing");
        assert_eq!(
            "mentions".parse::<TableType>().unwrap(),
            TableType::Mentions
        );
        log::debug!("Testing 'gkg' parsing");
        assert_eq!("gkg".parse::<TableType>().unwrap(), TableType::Gkg);
        log::debug!("Testing invalid table type parsing");
        assert!("invalid".parse::<TableType>().is_err());

        log::info!("All table type parsing tests passed");
        log::info!("test_table_type_parsing completed successfully");
    }

    #[test]
    fn test_table_type_file_extensions() {
        init_test_logging();
        log::info!("Starting test_table_type_file_extensions");

        log::debug!("Testing Export table file extension");
        assert_eq!(
            TableType::Export.file_extension(),
            FileExtension::Csv(CsvExtension::Upper)
        );
        log::debug!("Testing Mentions table file extension");
        assert_eq!(
            TableType::Mentions.file_extension(),
            FileExtension::Csv(CsvExtension::Upper)
        );
        log::debug!("Testing GKG table file extension");
        assert_eq!(
            TableType::Gkg.file_extension(),
            FileExtension::Csv(CsvExtension::Lower)
        );

        log::info!("All table type file extension tests passed");
        log::info!("test_table_type_file_extensions completed successfully");
    }

    #[test]
    fn test_json_file_extensions() {
        init_test_logging();
        log::info!("Starting test_json_file_extensions");

        log::debug!("Testing compressed JSON extension");
        assert_eq!(
            FileExtension::Json(JsonExtension::Compressed).as_str(),
            "json.zip"
        );
        log::debug!("Testing uncompressed JSON extension");
        assert_eq!(
            FileExtension::Json(JsonExtension::Uncompressed).as_str(),
            "json"
        );

        log::info!("All JSON file extension tests passed");
        log::info!("test_json_file_extensions completed successfully");
    }

    #[test]
    fn test_file_extension_compression() {
        init_test_logging();
        log::info!("Starting test_file_extension_compression");

        log::debug!("Testing CSV Upper extension compression");
        assert!(FileExtension::Csv(CsvExtension::Upper).is_compressed());
        log::debug!("Testing CSV Lower extension compression");
        assert!(FileExtension::Csv(CsvExtension::Lower).is_compressed());
        log::debug!("Testing JSON Compressed extension compression");
        assert!(FileExtension::Json(JsonExtension::Compressed).is_compressed());
        log::debug!("Testing JSON Uncompressed extension compression");
        assert!(!FileExtension::Json(JsonExtension::Uncompressed).is_compressed());

        log::info!("All file extension compression tests passed");
        log::info!("test_file_extension_compression completed successfully");
    }

    #[test]
    fn test_custom_table_configurations() {
        init_test_logging();
        log::info!("Starting test_custom_table_configurations");

        log::debug!("Testing export JSON configuration");
        let export_json = TableType::export_json();
        assert_eq!(export_json.table_type, TableType::Export);
        assert_eq!(
            export_json.file_extension(),
            FileExtension::Json(JsonExtension::Compressed)
        );
        log::debug!("Export JSON configuration validated");

        log::debug!("Testing mentions JSON configuration");
        let mentions_json = TableType::mentions_json();
        assert_eq!(mentions_json.table_type, TableType::Mentions);
        assert_eq!(
            mentions_json.file_extension(),
            FileExtension::Json(JsonExtension::Compressed)
        );
        log::debug!("Mentions JSON configuration validated");

        log::debug!("Testing GKG JSON configuration");
        let gkg_json = TableType::gkg_json();
        assert_eq!(gkg_json.table_type, TableType::Gkg);
        assert_eq!(
            gkg_json.file_extension(),
            FileExtension::Json(JsonExtension::Compressed)
        );
        log::debug!("GKG JSON configuration validated");

        log::info!("All custom table configuration tests passed");
        log::info!("test_custom_table_configurations completed successfully");
    }

    #[test]
    fn test_url_builder() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_url_builder");

        log::debug!("Creating test timestamp for 2024-08-06 20:00:00");
        let timestamp = DateTime::from_timestamp(1722974400, 0).unwrap().naive_utc(); // 2024-08-06 20:00:00
        log::debug!("Timestamp created: {:?}", timestamp);

        log::debug!("Building GDELT v2 export URL");
        let url = GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V2)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Export)
            .build()?;
        log::info!("Built v2 export URL: {}", url.as_str());

        assert_eq!(
            url.as_str(),
            "http://data.gdeltproject.org/gdeltv2/20240806200000.export.CSV.zip"
        );
        log::debug!("v2 export URL validation passed");

        log::debug!("Building GDELT v3 export URL");
        let url_v3 = GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V3)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Export)
            .build()?;
        log::info!("Built v3 export URL: {}", url_v3.as_str());

        assert_eq!(
            url_v3.as_str(),
            "http://data.gdeltproject.org/gdeltv3/20240806200000.export.CSV.zip"
        );
        log::debug!("v3 export URL validation passed");

        log::debug!("Building GDELT v2 GKG translation URL");
        let url_translation = GdeltUrlBuilder::new()
            .with_version(GdeltVersion::V2)
            .with_timestamp(timestamp)
            .with_table_type(TableType::Gkg)
            .with_translation(true)
            .build()?;
        log::info!("Built v2 GKG translation URL: {}", url_translation.as_str());

        assert_eq!(
            url_translation.as_str(),
            "http://data.gdeltproject.org/gdeltv2/20240806200000.translation.gkg.csv.zip"
        );
        log::debug!("v2 GKG translation URL validation passed");

        log::info!("All URL builder tests passed");
        log::info!("test_url_builder completed successfully");
        Ok(())
    }

    #[test]
    fn test_gdelt_file_entry_parsing() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_gdelt_file_entry_parsing");

        log::debug!("Testing basic GDELT file entry parsing");
        let line = "92723 6838a0a7509acaf5821c0e7e86460f3d http://data.gdeltproject.org/gdeltv2/20250806200000.export.CSV.zip";
        log::debug!("Parsing line: {}", line);
        let entry = GdeltFileEntry::parse_from_line(line)?;
        log::info!(
            "Parsed entry - Size: {}, Hash: {}, Type: {:?}, Translation: {}",
            entry.size,
            entry.hash,
            entry.table_type,
            entry.is_translation
        );

        assert_eq!(entry.size, 92723);
        assert_eq!(entry.hash, "6838a0a7509acaf5821c0e7e86460f3d");
        assert_eq!(entry.table_type, TableType::Export);
        assert!(!entry.is_translation);
        log::debug!("Basic entry validation passed");

        log::debug!("Testing translation file entry parsing");
        let translation_line = "48910 3a59414c24547e5c3ade383e9f78365b http://data.gdeltproject.org/gdeltv2/20250806200000.translation.export.CSV.zip";
        log::debug!("Parsing translation line: {}", translation_line);
        let translation_entry = GdeltFileEntry::parse_from_line(translation_line)?;
        log::info!(
            "Parsed translation entry - Size: {}, Hash: {}, Type: {:?}, Translation: {}",
            translation_entry.size,
            translation_entry.hash,
            translation_entry.table_type,
            translation_entry.is_translation
        );

        assert_eq!(translation_entry.table_type, TableType::Export);
        assert!(translation_entry.is_translation);
        log::debug!("Translation entry validation passed");

        log::debug!("Testing v3 URL parsing");
        let v3_line = "92723 6838a0a7509acaf5821c0e7e86460f3d http://data.gdeltproject.org/gdeltv3/20250806200000.export.CSV.zip";
        log::debug!("Parsing v3 line: {}", v3_line);
        let v3_entry = GdeltFileEntry::parse_from_line(v3_line)?;
        log::info!("Parsed v3 entry - Type: {:?}", v3_entry.table_type);
        assert_eq!(v3_entry.table_type, TableType::Export);
        log::debug!("v3 entry validation passed");

        log::info!("All GDELT file entry parsing tests passed");
        log::info!("test_gdelt_file_entry_parsing completed successfully");
        Ok(())
    }

    #[test]
    fn test_gdelt_version_parsing() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_gdelt_version_parsing");

        log::debug!("Testing various V2 format parsing");
        assert_eq!("v2".parse::<GdeltVersion>().unwrap(), GdeltVersion::V2);
        assert_eq!("V2".parse::<GdeltVersion>().unwrap(), GdeltVersion::V2);
        assert_eq!("2".parse::<GdeltVersion>().unwrap(), GdeltVersion::V2);
        assert_eq!("gdeltv2".parse::<GdeltVersion>().unwrap(), GdeltVersion::V2);
        log::debug!("V2 parsing tests passed");

        log::debug!("Testing various V3 format parsing");
        assert_eq!("v3".parse::<GdeltVersion>().unwrap(), GdeltVersion::V3);
        assert_eq!("V3".parse::<GdeltVersion>().unwrap(), GdeltVersion::V3);
        assert_eq!("3".parse::<GdeltVersion>().unwrap(), GdeltVersion::V3);
        assert_eq!("gdeltv3".parse::<GdeltVersion>().unwrap(), GdeltVersion::V3);
        log::debug!("V3 parsing tests passed");

        log::debug!("Testing invalid version parsing");
        assert!("invalid".parse::<GdeltVersion>().is_err());
        log::debug!("Invalid version parsing test passed");

        log::info!("All GDELT version parsing tests passed");
        log::info!("test_gdelt_version_parsing completed successfully");
        Ok(())
    }

    #[test]
    fn test_gdelt_version_urls() {
        init_test_logging();
        log::info!("Starting test_gdelt_version_urls");

        log::debug!("Testing V2 base URL");
        let v2_base = GdeltVersion::V2.base_url();
        log::info!("V2 base URL: {}", v2_base);
        assert_eq!(v2_base, "http://data.gdeltproject.org/gdeltv2");

        log::debug!("Testing V3 base URL");
        let v3_base = GdeltVersion::V3.base_url();
        log::info!("V3 base URL: {}", v3_base);
        assert_eq!(v3_base, "http://data.gdeltproject.org/gdeltv3");

        log::debug!("Testing V2 lastupdate URL");
        let v2_lastupdate = GdeltVersion::V2.lastupdate_url();
        log::info!("V2 lastupdate URL: {}", v2_lastupdate);
        assert_eq!(
            v2_lastupdate,
            "http://data.gdeltproject.org/gdeltv2/lastupdate.txt"
        );

        log::debug!("Testing V3 lastupdate URL");
        let v3_lastupdate = GdeltVersion::V3.lastupdate_url();
        log::info!("V3 lastupdate URL: {}", v3_lastupdate);
        assert_eq!(
            v3_lastupdate,
            "http://data.gdeltproject.org/gdeltv3/lastupdate.txt"
        );

        log::debug!("Testing V2 masterfilelist URL");
        let v2_masterfilelist = GdeltVersion::V2.masterfilelist_url();
        log::info!("V2 masterfilelist URL: {}", v2_masterfilelist);
        assert_eq!(
            v2_masterfilelist,
            "http://data.gdeltproject.org/gdeltv2/masterfilelist.txt"
        );

        log::debug!("Testing V3 masterfilelist URL");
        let v3_masterfilelist = GdeltVersion::V3.masterfilelist_url();
        log::info!("V3 masterfilelist URL: {}", v3_masterfilelist);
        assert_eq!(
            v3_masterfilelist,
            "http://data.gdeltproject.org/gdeltv3/masterfilelist.txt"
        );

        log::info!("All GDELT version URL tests passed");
        log::info!("test_gdelt_version_urls completed successfully");
    }
}
