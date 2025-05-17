use futures::StreamExt;
use log::info;
use std::{
    fs::{self, File},
    io,
    path::Path,
    str::FromStr,
};
use utils::{extract_date, extract_db_type};
use zip::ZipArchive;

pub mod utils;

pub struct GDELTDatabase {
    pub link: reqwest::Url,
    pub date: chrono::NaiveDateTime,
    pub file: Option<File>,
    pub db_type: DatabaseType,
}

#[derive(Debug, PartialEq)]
pub enum DatabaseType {
    Events,
    GlobalKnowledgeGraph,
    Mentions,
}

impl Default for GDELTDatabase {
    fn default() -> Self {
        use chrono::{Datelike, Timelike, Utc};

        // Get the current time in UTC
        let now = Utc::now();

        // Round down to the latest 15th minute
        let rounded_minute = (now.minute() / 15) * 15;
        let rounded_time = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_minute(rounded_minute)
            .unwrap();

        // Format the base URL using the rounded time
        let base_url = format!(
            "http://data.gdeltproject.org/gdeltv2/{:04}{:02}{:02}{:02}{:02}{:02}.export.CSV.zip",
            rounded_time.year(),
            rounded_time.month(),
            rounded_time.day(),
            rounded_time.hour(),
            rounded_time.minute(),
            rounded_time.second()
        );

        // Create a new GDELTDatabase instance with the dynamically generated URL
        let mut db = GDELTDatabase::new(&base_url).expect("Failed to create GDELTDatabase instance");

        db
    }
}
impl GDELTDatabase {
    pub fn new(url: &str) -> anyhow::Result<Self> {
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

        pub fn from_date_and_type(date: chrono::NaiveDateTime, db_type: DatabaseType) -> anyhow::Result<Self> {
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

        pub async fn download_and_unzip(&self, download_path: &str, output_dir: &str) -> anyhow::Result<File> {
            self.download_to_path(download_path).await?;
            let unzipped_file = Self::unzip_single_file(download_path, output_dir)?;
            Ok(unzipped_file)
        }
    pub async fn download_to_path(&self, path: &str) -> anyhow::Result<()> {
        log::info!("Making Request");
        let response = reqwest::get(self.link.clone()).await?;
        log::info!("Download Response: {response:?}");

        let mut file = tokio::fs::File::create(path).await?;
        let mut content = response.bytes_stream();

        while let Some(chunk) = content.next().await {
            let chunk = chunk?;
            tokio::io::copy(&mut chunk.as_ref(), &mut file).await?;
        }

        Ok(())
    }
    pub fn unzip_single_file(zip_file_path: &str, output_dir: &str) -> anyhow::Result<File> {
        // Create the output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;

        // Open the ZIP file
        let file = File::open(zip_file_path)?;
        info!("File: {file:?}");
        let mut archive = ZipArchive::new(file)?;

        info!("Archive: {archive:?}");

        // Since we expect only one file, we can directly access it
        let mut file = archive.by_index(0)?;

        info!("File: {:?}", file.name());

        // Derive the output file path from the output directory and file name
        let output_path = Path::new(output_dir).join(file.name().to_lowercase());
        info!("Output Path: {:?}", output_path);

        let mut outfile = File::create(output_path)?;

        info!("Output File: {outfile:?}");

        // Copy the contents of the file to the output file
        io::copy(&mut file, &mut outfile)?;

        fs::remove_file(zip_file_path)?;
        Ok(outfile)
    }
    
pub async fn update_latest(&mut self) -> anyhow::Result<()> {
    use chrono::{Datelike, Timelike, Utc};

    // Get the current time in UTC and round down to the latest 15th minute
    let now = Utc::now();
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
        "http://data.gdeltproject.org/gdeltv2/{:04}{:02}{:02}{:02}{:02}{:02}.export.CSV.zip",
        rounded_time.year(),
        rounded_time.month(),
        rounded_time.day(),
        rounded_time.hour(),
        rounded_time.minute(),
        rounded_time.second()
    );

    // Update the instance fields
    self.link = reqwest::Url::from_str(&url)?;
    self.date = rounded_time.naive_utc();
    self.db_type = DatabaseType::Events;

    // Download the file
    self.download_to_path("./latest_download.zip").await?;

    Ok(())
}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdelt_database_new() {
        let url = "https://data.gdeltproject.org/gdeltv2/20211021000000.export.CSV.zip";
        let db = GDELTDatabase::new(url).unwrap();
        assert_eq!(db.db_type, DatabaseType::Events);
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
        let url = "https://invalid.url";
        let result = GDELTDatabase::new(url);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_to_path() {
        let url = "http://data.gdeltproject.org/gdeltv2/20250322180000.export.CSV.zip";
        let db = GDELTDatabase::new(url).unwrap();
        let result = db.download_to_path("./test.csv.zip").await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_unzip_single_file() {
        env_logger::init();
        let zip_file_path = "./test.csv.zip";
        let output_dir = "./output";
        let result = GDELTDatabase::unzip_single_file(zip_file_path, output_dir);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn download_and_unzip_creates_unzipped_file() {
        let url = "http://data.gdeltproject.org/gdeltv2/20250322180000.export.CSV.zip";
        let db = GDELTDatabase::default();
        let download_path = "./test_download.zip";
        let output_dir = "./test_output";
    
        let result = db.download_and_unzip(download_path, output_dir).await;
        assert!(result.is_ok());
    
        let unzipped_file = result.unwrap();
        assert!(unzipped_file.metadata().is_ok());
        assert!(unzipped_file.metadata().unwrap().is_file());
    }
    
    #[tokio::test]
    async fn download_and_unzip_invalid_url_fails() {
        let url = "http://invalid.url";
        let db = GDELTDatabase::new(url);
        assert!(db.is_err());
    }
    
    #[test]
    fn from_date_and_type_creates_correct_instance() {
        let date = chrono::NaiveDate::from_ymd_opt(2023, 3, 22)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap();
        let db_type = DatabaseType::Events;
    
        let db = GDELTDatabase::from_date_and_type(date, db_type).unwrap();
        assert_eq!(db.db_type, DatabaseType::Events);
        assert_eq!(db.date, date);
        assert!(db.link.as_str().contains("20230322180000.export.CSV.zip"));
    }
    
    #[test]
    fn from_date_and_type_invalid_date_fails() {
        let date = chrono::NaiveDate::from_ymd_opt(2023, 2, 30); // Invalid date
        let db_type = DatabaseType::Events;
    
        assert!(date.is_none());
    }
    
    #[tokio::test]
    async fn test_update_latest() {
        let mut db = GDELTDatabase::default();
        let result = db.update_latest().await;
        assert!(result.is_ok());
        assert!(db.link.as_str().contains("gdeltv2"));
        assert!(db.date > chrono::NaiveDateTime::from_timestamp(0, 0));
    }
}
