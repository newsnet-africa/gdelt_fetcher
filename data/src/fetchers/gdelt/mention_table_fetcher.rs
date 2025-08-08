use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use csv::ReaderBuilder;
use std::fs::File;
use std::path::PathBuf;
use url::Url;

use models::types::mention_table::MentionTable;

use super::gdelt_fetcher::{GdeltFetcher, GdeltUrlBuilder, GdeltVersion, TableType};
use crate::fetchers::{DataFetcher, DataSource, RawDataFetcher};

/// Iterator for MentionTable records from CSV files
pub struct MentionTableIterator {
    csv_reader: csv::Reader<File>,
}

impl MentionTableIterator {
    pub fn new(file_path: PathBuf) -> Result<Self> {
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open mention table file: {:?}", file_path))?;

        let csv_reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        Ok(Self { csv_reader })
    }

    /// Create iterator from file path with custom CSV settings
    pub fn with_csv_config(file_path: PathBuf, delimiter: u8, has_headers: bool) -> Result<Self> {
        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open mention table file: {:?}", file_path))?;

        let csv_reader = ReaderBuilder::new()
            .has_headers(has_headers)
            .delimiter(delimiter)
            .from_reader(file);

        Ok(Self { csv_reader })
    }

    /// Count total records without consuming the iterator
    pub fn count_records(file_path: &PathBuf) -> Result<usize> {
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_reader(file);

        let mut count = 0;
        let mut record = csv::StringRecord::new();
        while reader.read_record(&mut record)? {
            count += 1;
        }
        Ok(count)
    }
}

impl Iterator for MentionTableIterator {
    type Item = MentionTable;

    fn next(&mut self) -> Option<Self::Item> {
        let mut record = csv::StringRecord::new();
        match self.csv_reader.read_record(&mut record) {
            Ok(true) => match MentionTable::try_from(record) {
                Ok(mention) => Some(mention),
                Err(e) => {
                    log::warn!("Failed to parse mention record: {}", e);
                    // Continue to next record instead of stopping
                    self.next()
                }
            },
            Ok(false) => None, // End of file
            Err(e) => {
                log::error!("CSV read error: {}", e);
                None
            }
        }
    }
}

/// Fetcher for MentionTable data
pub struct MentionTableFetcher {
    gdelt_fetcher: GdeltFetcher,
    last_fetch_time: Option<DateTime<Utc>>,
    is_translation: bool,
}

impl MentionTableFetcher {
    pub fn new(gdelt_fetcher: GdeltFetcher) -> Self {
        Self {
            gdelt_fetcher,
            last_fetch_time: None,
            is_translation: false,
        }
    }

    pub fn new_v2<P: AsRef<std::path::Path>>(output_dir: P, temp_dir: P) -> Result<Self> {
        let fetcher = GdeltFetcher::new_v2(output_dir, temp_dir)?;
        Ok(Self::new(fetcher))
    }

    pub fn new_v3<P: AsRef<std::path::Path>>(output_dir: P, temp_dir: P) -> Result<Self> {
        let fetcher = GdeltFetcher::new_v3(output_dir, temp_dir)?;
        Ok(Self::new(fetcher))
    }

    /// Create fetcher with custom GDELT version
    pub fn with_version<P: AsRef<std::path::Path>>(
        output_dir: P,
        temp_dir: P,
        version: GdeltVersion,
    ) -> Result<Self> {
        let fetcher = GdeltFetcher::new_with_version(output_dir, temp_dir, version)?;
        Ok(Self::new(fetcher))
    }

    /// Enable or disable translation data fetching
    pub fn with_translation(mut self, is_translation: bool) -> Self {
        self.is_translation = is_translation;
        self
    }

    /// Get the underlying GDELT fetcher
    pub fn gdelt_fetcher(&self) -> &GdeltFetcher {
        &self.gdelt_fetcher
    }

    /// Get the GDELT version being used
    pub fn version(&self) -> GdeltVersion {
        self.gdelt_fetcher.version()
    }

    /// Fetch latest mentions asynchronously
    pub async fn fetch_latest_async(&mut self) -> Result<MentionTableIterator> {
        let file_path = if self.is_translation {
            self.gdelt_fetcher
                .fetch_latest_mentions_translation()
                .await?
        } else {
            self.gdelt_fetcher.fetch_latest_mentions().await?
        };

        self.last_fetch_time = Some(Utc::now());
        MentionTableIterator::new(file_path)
    }

    /// Fetch mentions by date asynchronously
    pub async fn fetch_date_async(
        &mut self,
        date: DateTime<impl TimeZone>,
    ) -> Result<MentionTableIterator> {
        let naive_date = date.naive_utc();
        let file_path = if self.is_translation {
            self.gdelt_fetcher
                .fetch_mentions_translation_by_date(naive_date)
                .await?
        } else {
            self.gdelt_fetcher
                .fetch_mentions_by_date(naive_date)
                .await?
        };

        self.last_fetch_time = Some(Utc::now());
        MentionTableIterator::new(file_path)
    }

    /// Get count of records in the latest file
    pub fn count_latest_records(&self) -> Result<usize> {
        let files = self
            .gdelt_fetcher
            .get_local_files_by_table(TableType::Mentions)?;
        if let Some(latest_file) = files.first() {
            MentionTableIterator::count_records(latest_file)
        } else {
            Ok(0)
        }
    }
}

impl DataFetcher<MentionTable, PathBuf> for MentionTableFetcher {
    type FetchBatch = MentionTableIterator;

    fn url_link(&self) -> Result<Url> {
        let builder = GdeltUrlBuilder::new()
            .with_version(self.gdelt_fetcher.version())
            .with_table_type(TableType::Mentions)
            .with_translation(self.is_translation);

        builder.build()
    }

    fn fetch_latest(&self) -> Result<Self::FetchBatch> {
        let rt = tokio::runtime::Runtime::new()?;
        let file_path = if self.is_translation {
            rt.block_on(async { self.gdelt_fetcher.fetch_latest_mentions_translation().await })?
        } else {
            rt.block_on(async { self.gdelt_fetcher.fetch_latest_mentions().await })?
        };

        MentionTableIterator::new(file_path)
    }

    fn last_fetch(&self) -> Option<DateTime<impl TimeZone>> {
        self.last_fetch_time
    }

    fn fetch_date(&self, date: DateTime<impl TimeZone>) -> Result<Self::FetchBatch> {
        let naive_date = date.naive_utc();
        let rt = tokio::runtime::Runtime::new()?;
        let file_path = if self.is_translation {
            rt.block_on(async {
                self.gdelt_fetcher
                    .fetch_mentions_translation_by_date(naive_date)
                    .await
            })?
        } else {
            rt.block_on(async { self.gdelt_fetcher.fetch_mentions_by_date(naive_date).await })?
        };

        MentionTableIterator::new(file_path)
    }
}

impl RawDataFetcher for MentionTableFetcher {
    type RawDataFileFormat = PathBuf;

    fn file_path(&self) -> Result<PathBuf> {
        self.gdelt_fetcher.file_path()
    }

    fn fetched_file_paths(&self) -> Result<Vec<PathBuf>> {
        self.gdelt_fetcher.fetched_file_paths()
    }

    fn fetch_latest_raw(&self, datasource: DataSource) -> Result<Self::RawDataFileFormat> {
        self.gdelt_fetcher.fetch_latest_raw(datasource)
    }

    fn fetch_date_raw(
        &self,
        date: DateTime<impl TimeZone>,
        datasource: DataSource,
    ) -> Result<Self::RawDataFileFormat> {
        self.gdelt_fetcher.fetch_date_raw(date, datasource)
    }

    fn source(&self) -> DataSource {
        self.gdelt_fetcher.source()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fetchers::gdelt::test_utils::init_test_logging;
    use tempfile::TempDir;

    fn create_test_fetcher() -> Result<(MentionTableFetcher, TempDir, TempDir)> {
        let output_dir = TempDir::new()?;
        let temp_dir = TempDir::new()?;
        let fetcher = MentionTableFetcher::new_v2(output_dir.path(), temp_dir.path())?;
        Ok((fetcher, output_dir, temp_dir))
    }

    #[test]
    fn test_mention_table_fetcher_creation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_mention_table_fetcher_creation");

        log::debug!("Creating test fetcher with temporary directories");
        let (_fetcher, _output_dir, _temp_dir) = create_test_fetcher()?;

        log::info!("Successfully created MentionTableFetcher");
        log::debug!("Output dir: {:?}", _output_dir.path());
        log::debug!("Temp dir: {:?}", _temp_dir.path());

        log::info!("test_mention_table_fetcher_creation completed successfully");
        Ok(())
    }

    #[test]
    fn test_mention_table_fetcher_v3_creation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_mention_table_fetcher_v3_creation");

        log::debug!("Creating temporary directories");
        let output_dir = TempDir::new()?;
        let temp_dir = TempDir::new()?;
        log::debug!("Output dir: {:?}", output_dir.path());
        log::debug!("Temp dir: {:?}", temp_dir.path());

        log::debug!("Creating MentionTableFetcher with GDELT v3");
        let _fetcher = MentionTableFetcher::new_v3(output_dir.path(), temp_dir.path())?;
        log::info!("Successfully created MentionTableFetcher v3");

        log::info!("test_mention_table_fetcher_v3_creation completed successfully");
        Ok(())
    }

    #[test]
    fn test_mention_table_fetcher_with_translation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_mention_table_fetcher_with_translation");

        log::debug!("Creating base fetcher");
        let (fetcher, _output_dir, _temp_dir) = create_test_fetcher()?;
        log::debug!("Base fetcher created successfully");

        log::debug!("Creating translation fetcher with translation enabled");
        let _translation_fetcher = fetcher.with_translation(true);
        log::info!("Translation fetcher created successfully");

        log::info!("test_mention_table_fetcher_with_translation completed successfully");
        Ok(())
    }

    #[test]
    fn test_url_link_generation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_url_link_generation");

        log::debug!("Creating test fetcher");
        let (fetcher, _output_dir, _temp_dir) = create_test_fetcher()?;
        log::debug!("Test fetcher created successfully");

        log::debug!("Generating URL link");
        let url = fetcher.url_link()?;
        log::info!("Generated URL: {}", url.as_str());

        log::debug!("Validating URL contains 'gdeltv2'");
        assert!(url.as_str().contains("gdeltv2"));
        log::debug!("Validating URL contains 'mentions'");
        assert!(url.as_str().contains("mentions"));
        log::debug!("Validating URL contains 'CSV.zip'");
        assert!(url.as_str().contains("CSV.zip"));

        log::info!("All URL validations passed");
        log::info!("test_url_link_generation completed successfully");
        Ok(())
    }

    #[test]
    fn test_url_link_with_translation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_url_link_with_translation");

        log::debug!("Creating base test fetcher");
        let (fetcher, _output_dir, _temp_dir) = create_test_fetcher()?;
        log::debug!("Creating translation fetcher");
        let translation_fetcher = fetcher.with_translation(true);
        log::debug!("Translation fetcher created successfully");

        log::debug!("Generating URL link for translation fetcher");
        let url = translation_fetcher.url_link()?;
        log::info!("Generated translation URL: {}", url.as_str());

        log::debug!("Validating translation URL contains 'gdeltv2'");
        assert!(url.as_str().contains("gdeltv2"));
        log::debug!("Validating translation URL contains 'translation'");
        assert!(url.as_str().contains("translation"));
        log::debug!("Validating translation URL contains 'mentions'");
        assert!(url.as_str().contains("mentions"));

        log::info!("All translation URL validations passed");
        log::info!("test_url_link_with_translation completed successfully");
        Ok(())
    }

    #[test]
    fn test_version_access() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_version_access");

        log::debug!("Creating test fetcher");
        let (fetcher, _output_dir, _temp_dir) = create_test_fetcher()?;
        log::debug!("Test fetcher created successfully");

        log::debug!("Checking fetcher version");
        let version = fetcher.version();
        log::info!("Fetcher version: {:?}", version);

        log::debug!("Validating version is V2");
        assert_eq!(version, GdeltVersion::V2);
        log::info!("Version validation passed");

        log::info!("test_version_access completed successfully");
        Ok(())
    }

    #[test]
    fn test_custom_version_creation() -> Result<()> {
        init_test_logging();
        log::info!("Starting test_custom_version_creation");

        log::debug!("Creating temporary directories");
        let output_dir = TempDir::new()?;
        let temp_dir = TempDir::new()?;
        log::debug!("Output dir: {:?}", output_dir.path());
        log::debug!("Temp dir: {:?}", temp_dir.path());

        log::debug!("Creating MentionTableFetcher with custom version V3");
        let fetcher = MentionTableFetcher::with_version(
            output_dir.path(),
            temp_dir.path(),
            GdeltVersion::V3,
        )?;
        log::info!("Custom version fetcher created successfully");

        log::debug!("Validating custom version is V3");
        let version = fetcher.version();
        log::info!("Custom fetcher version: {:?}", version);
        assert_eq!(version, GdeltVersion::V3);
        log::info!("Custom version validation passed");

        log::info!("test_custom_version_creation completed successfully");
        Ok(())
    }
}
