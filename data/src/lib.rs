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
}
