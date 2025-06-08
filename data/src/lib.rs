use chrono::Duration;
use futures::StreamExt;
use log::info;
use std::sync::OnceLock;
use std::{
    fs::{self, File},
    io,
    path::Path,
    str::FromStr,
};
use utils::{extract_date, extract_db_type};
use zip::ZipArchive;

pub mod utils;

pub static TMP_DIR: OnceLock<&'static str> = OnceLock::new();
pub static ZIP_PATH: OnceLock<&'static str> = OnceLock::new();
pub static OUTPUT_DIR: OnceLock<&'static str> = OnceLock::new();

pub fn init_paths() {
    TMP_DIR.get_or_init(|| "./tmp");
    ZIP_PATH.get_or_init(|| "./tmp/latest_download.zip");
    OUTPUT_DIR.get_or_init(|| "./tmp/output");
}

#[derive(Debug)]
pub struct GDELTDatabase {
    pub link: reqwest::Url,
    pub date: chrono::NaiveDateTime,
    pub file: Option<File>,
    pub db_type: DatabaseType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DatabaseType {
    Events,
    GlobalKnowledgeGraph,
    Mentions,
}

impl TryFrom<&str> for DatabaseType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "export" => Ok(DatabaseType::Events),
            "gkg" => Ok(DatabaseType::GlobalKnowledgeGraph),
            "mentions" => Ok(DatabaseType::Mentions),
            _ => Err(anyhow::Error::msg("Invalid database type")),
        }
    }
}

impl TryFrom<&DatabaseType> for String {
    type Error = anyhow::Error;

    fn try_from(value: &DatabaseType) -> Result<Self, Self::Error> {
        match value {
            DatabaseType::Events => Ok("export".to_string()),
            DatabaseType::GlobalKnowledgeGraph => Ok("gkg".to_string()),
            DatabaseType::Mentions => Ok("mentions".to_string()),
        }
    }
}

impl GDELTDatabase {
    pub fn new(db_type: DatabaseType) -> anyhow::Result<Self> {
        use chrono::{Datelike, Timelike, Utc};
        let now = Utc::now()
            .checked_sub_signed(chrono::Duration::hours(1))
            .unwrap();
        let rounded_minute = (now.minute() / 15) * 15;
        let rounded_time = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_minute(rounded_minute)
            .unwrap();
        Self::from_date_and_type(rounded_time.naive_utc(), db_type)
    }

    pub fn from_url_str(url: &str) -> anyhow::Result<Self> {
        info!("Creating Instance of GDELTDatabase");
        let db_type = extract_db_type(url)?;
        let date = extract_date(url)?;
        let db_type_enum = match db_type.as_str() {
            "export" => DatabaseType::Events,
            "gkg" => DatabaseType::GlobalKnowledgeGraph,
            "mentions" => DatabaseType::Mentions,
            _ => return Err(anyhow::Error::msg("Link not supported")),
        };
        Ok(Self {
            link: reqwest::Url::from_str(url)?,
            date,
            file: None,
            db_type: db_type_enum,
        })
    }

    pub fn from_date_and_type(
        date: chrono::NaiveDateTime,
        db_type: DatabaseType,
    ) -> anyhow::Result<Self> {
        let db_type_str = match db_type {
            DatabaseType::Events => "export",
            DatabaseType::GlobalKnowledgeGraph => "gkg",
            DatabaseType::Mentions => "mentions",
        };

        let url = format!(
            "http://data.gdeltproject.org/gdeltv2/{date}.{db_type}.CSV.zip",
            date = date.format("%Y%m%d%H%M%S"),
            db_type = db_type_str
        );

        Ok(Self {
            link: reqwest::Url::from_str(&url)?,
            date,
            file: None,
            db_type,
        })
    }

    pub async fn download_and_unzip(
        &self,
        download_path: &str,
        output_dir: &str,
    ) -> anyhow::Result<File> {
        let res = self.download_to_path(download_path).await;
        if res.is_err() {
            return Err(anyhow::anyhow!("Failed to download file: {:?}", res));
        }
        let unzipped_file = Self::unzip_single_file(download_path, output_dir)?;
        Ok(unzipped_file)
    }
    pub async fn download_to_path(&self, path: &str) -> anyhow::Result<()> {
        use std::path::Path as StdPath;
        log::info!("Making Request");
        let response = reqwest::get(self.link.clone()).await?;
        log::info!("Download Response: {response:?}");

        if let Some(parent) = StdPath::new(path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = tokio::fs::File::create(path).await?;
        let mut content = response.bytes_stream();

        while let Some(Ok(chunk)) = content.next().await {
            tokio::io::copy(&mut chunk.as_ref(), &mut file).await?;
        }

        Ok(())
    }

    pub fn unzip_single_file(zip_file_path: &str, output_dir: &str) -> anyhow::Result<File> {
        use std::fs::OpenOptions;
        use std::path::Path;

        // Check if the ZIP file exists
        if !Path::new(zip_file_path).exists() {
            return Err(anyhow::anyhow!(
                "ZIP file does not exist at {}",
                zip_file_path
            ));
        }

        // Ensure output directory exists
        fs::create_dir_all(output_dir)?;

        let file = File::open(zip_file_path)?;
        info!("File: {file:?}");
        let mut archive = ZipArchive::new(file)?;

        info!("Archive: {archive:?}");

        let mut zip_file = archive.by_index(0)?;
        info!("File: {:?}", zip_file.name());

        let output_path = Path::new(output_dir).join(zip_file.name().to_lowercase());
        info!("Output Path: {:?}", output_path);

        let mut outfile = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output_path)?;

        info!("Output File: {outfile:?}");

        io::copy(&mut zip_file, &mut outfile)?;

        fs::remove_file(zip_file_path)?;
        Ok(outfile)
    }
    pub async fn update_latest(&mut self) -> anyhow::Result<()> {
        use chrono::{Datelike, Timelike, Utc};
        use std::str::FromStr;

        init_paths();
        let zip_path = *ZIP_PATH.get().unwrap();

        // Get the current time in UTC and round down to the latest 15th minute
        let now = Utc::now().checked_sub_signed(Duration::hours(1)).unwrap();
        let rounded_minute = (now.minute() / 15) * 15;
        let rounded_time = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_minute(rounded_minute)
            .unwrap();

        // Generate the URL
        let url = format!(
            "http://data.gdeltproject.org/gdeltv2/{:04}{:02}{:02}{:02}{:02}{:02}.{}.CSV.zip",
            rounded_time.year(),
            rounded_time.month(),
            rounded_time.day(),
            rounded_time.hour(),
            rounded_time.minute(),
            rounded_time.second(),
            String::try_from(&self.db_type)?
        );

        // Update the instance fields
        self.link = reqwest::Url::from_str(&url)?;
        self.date = rounded_time.naive_utc();
        self.db_type = DatabaseType::Events;

        // Download the file
        self.download_to_path(zip_path).await?;

        Ok(())
    }

    /// Downloads GDELT databases in batches between `start` and `end` (inclusive), at 15-minute intervals.
    pub async fn download_batch(
        start: chrono::NaiveDateTime,
        end: chrono::NaiveDateTime,
        db_type: DatabaseType,
        output_dir: &str,
    ) -> anyhow::Result<()> {
        use chrono::Duration;

        let mut current = start;
        while current <= end {
            let db = GDELTDatabase::from_date_and_type(current, db_type.clone())?;
            let file_name = format!(
                "{}/{}.{}.CSV.zip",
                output_dir,
                current.format("%Y%m%d%H%M%S"),
                String::try_from(&db_type)?
            );
            db.download_to_path(&file_name).await?;
            current += Duration::minutes(15);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use std::sync::Once;

    use super::*;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            let _ = env_logger::builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Trace)
                .try_init();
        });
    }

    #[test]
    fn test_gdelt_database_new() {
        init_logger();
        log::info!("Running test_gdelt_database_new");
        let url = "https://data.gdeltproject.org/gdeltv2/20211021000000.mentions.CSV.zip";
        let db = GDELTDatabase::from_url_str(url).unwrap();
        log::debug!("Created db: {:?}", db);
        assert_eq!(db.db_type, DatabaseType::Mentions);
        assert_eq!(
            db.date,
            chrono::NaiveDate::from_ymd_opt(2021, 10, 21)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        );
    }

    #[test]
    fn test_gdelt_database_new_invalid_url() {
        init_logger();
        log::info!("Running test_gdelt_database_new_invalid_url");
        let url = "https://invalid.url";
        let result = GDELTDatabase::from_url_str(url);
        log::debug!("Result: {:?}", result);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_to_path() {
        init_logger();
        log::info!("Running test_download_to_path");
        let url = "http://data.gdeltproject.org/gdeltv2/20250322180000.export.CSV.zip";
        let db = GDELTDatabase::from_url_str(url).unwrap();
        log::debug!("Created db: {:?}", db);
        let result = db.download_to_path("./tmp/test.csv.zip").await;
        log::debug!("Download result: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unzip_single_file() {
        init_logger();
        log::info!("Running test_unzip_single_file");
        let zip_file_path = "./tmp/test.csv.zip";
        let output_dir = "./tmp/output";
        let result = GDELTDatabase::unzip_single_file(zip_file_path, output_dir);
        log::debug!("Unzip result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn download_and_unzip_creates_unzipped_file() {
        init_logger();
        log::info!("Running download_and_unzip_creates_unzipped_file");
        let url = "http://data.gdeltproject.org/gdeltv2/20250322180000.mentions.CSV.zip";
        let db = GDELTDatabase::from_url_str(url).unwrap();
        log::debug!("Created db: {:?}", db);
        let download_path = "./tmp/test_download.zip";
        let output_dir = "./tmp/test_output";

        let result = db.download_and_unzip(download_path, output_dir).await;
        log::debug!("Download and unzip result: {:?}", result);
        assert!(result.is_ok());

        let unzipped_file = result.unwrap();
        assert!(unzipped_file.metadata().is_ok());
        assert!(unzipped_file.metadata().unwrap().is_file());
    }

    #[tokio::test]
    async fn download_and_unzip_invalid_url_fails() {
        init_logger();
        log::info!("Running download_and_unzip_invalid_url_fails");
        let url = "http://invalid.url";
        let db = GDELTDatabase::from_url_str(url);
        log::debug!("DB creation result: {:?}", db);
        assert!(db.is_err());
    }

    #[test]
    fn from_date_and_type_creates_correct_instance() {
        init_logger();
        log::info!("Running from_date_and_type_creates_correct_instance");
        let date = chrono::NaiveDate::from_ymd_opt(2023, 3, 22)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap();
        let db_type = DatabaseType::Mentions;

        let db = GDELTDatabase::from_date_and_type(date, db_type).unwrap();
        log::debug!("Created db: {:?}", db);
        assert_eq!(db.db_type, DatabaseType::Mentions);
        assert_eq!(db.date, date);
        assert!(db.link.as_str().contains("20230322180000.mentions.CSV.zip"));
    }

    #[test]
    fn from_date_and_type_invalid_date_fails() {
        init_logger();
        log::info!("Running from_date_and_type_invalid_date_fails");
        let date = chrono::NaiveDate::from_ymd_opt(2023, 2, 30); // Invalid date
        let _db_type = DatabaseType::Events;

        log::debug!("Date result: {:?}", date);
        assert!(date.is_none());
    }

    #[tokio::test]
    async fn test_update_latest() {
        init_logger();
        log::info!("Running test_update_latest");
        let mut db = GDELTDatabase::new(DatabaseType::Events).expect("Messed up");
        log::debug!("Initial db: {:?}", db);
        let result = db.update_latest().await;
        log::debug!("Update result: {:?}", result);
        assert!(result.is_ok());
        assert!(db.link.as_str().contains("gdeltv2"));
        assert!(db.date > NaiveDateTime::new(NaiveDate::default(), NaiveTime::default()));
    }
}
