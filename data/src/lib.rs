// General TODO:
// 1. Write and test data fetching for the GKG tables. (I don't assume that this will be more complicated than the existing implementations, but still a chore)
// 2. Tests for this specific crate (the data crate) are failing. (I think it's just not parsing the URL properly idk)
// 3. Adjust the implementation to extract links from the http://data.gdeltproject.org/gdeltv2/lastupdate.txt link or the http://data.gdeltproject.org/gdeltv2/masterfilelist.txt link
//    instead of building the link blindly. This can let us use the checksums to verify the data.
// 4. Commenting everything.
// 5. Add implementations for Non CSV fetches. As far as I know, the only CSV pulls are for Events, Mentions and GKGs.
//    a. Some come as RSS feeds, which is great cause it's standardised
// 6. Organise these functions so that different data pulls are possible.
// 7. Manage the pulling of data from the network, (Although this is super low priority cause I think when the P2P database is self-managing, but just in case)
// 8. Set up the boilerplate for other types of data fetches that could be useful. The basics would include:
//    a. Adding a more general function (probably a macro honestly) to pull data from *any* source and save it anywhere.
//    b. Actually managing the temp data storage locations
// 9. CUSTOM ERROR TYPES. anyhow is great but it's probably a good idea to have an exhaustive list of errors so that we can standardise error handling and prevent unpleasant surprises
// 10. Remove as many ".expect("Blah")" as possible for similar reasons to 9. I hate Null pointer and random panics. Most should be recoverable anyways, I was either lazy or incapable of gracefully handling panics

use anyhow::anyhow;
use std::path::{Path, PathBuf};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use async_zip::base::read::seek::ZipFileReader;
use tokio::{
    fs::{File, OpenOptions, create_dir_all},
    io::BufReader,
};

use anyhow::Result;
use chrono::{Datelike, Duration, NaiveDateTime, Timelike};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use utils::{extract_date, extract_db_type};

pub mod utils;

//TODO: Move this out of here into a module of its own so taht non GDELT data can be pulled
#[derive(Debug)]
pub struct GDELTDatabase {
    pub link: reqwest::Url,
    pub date: NaiveDateTime, //TODO: Update to latest fetch so that we can track how old the data is
    pub file: Option<File>,
    pub db_type: DatabaseType,
}

//TODO: Extend this enum for all the possible GDELT Data tables that we could pull.
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
            _ => Err(anyhow::anyhow!("Invalid database type: {}", value)),
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
    pub async fn new(db_type: DatabaseType) -> Result<Self> {
        let tmp_dir = "./tmp";
        let output_dir = "./tmp/output";

        fs::create_dir_all(tmp_dir).await?;
        fs::create_dir_all(output_dir).await?;

        let now = chrono::Utc::now()
            .checked_sub_signed(Duration::hours(1))
            .unwrap();
        let rounded_minute = (now.minute() / 15) * 15;
        let rounded_time = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_minute(rounded_minute)
            .unwrap();
        Self::from_date_and_type(rounded_time.naive_utc(), db_type).await
    }

    pub async fn from_url_str(url: &str) -> Result<Self> {
        let tmp_dir = "./tmp";
        let output_dir = "./tmp/output";

        fs::create_dir_all(tmp_dir).await?;
        fs::create_dir_all(output_dir).await?;

        let db_type = extract_db_type(url)?;
        let date = extract_date(url)?;
        let db_type_enum = match db_type.as_str() {
            "export" => DatabaseType::Events,
            "gkg" => DatabaseType::GlobalKnowledgeGraph,
            "mentions" => DatabaseType::Mentions,
            _ => return Err(anyhow::anyhow!("Link not supported")),
        };

        Ok(GDELTDatabase {
            link: reqwest::Url::parse(url)?,
            date,
            file: None,
            db_type: db_type_enum,
        })
    }

    pub async fn from_date_and_type(date: NaiveDateTime, db_type: DatabaseType) -> Result<Self> {
        let tmp_dir = "./tmp";
        let output_dir = "./tmp/output";

        fs::create_dir_all(tmp_dir).await?;
        fs::create_dir_all(output_dir).await?;

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

        Ok(GDELTDatabase {
            link: reqwest::Url::parse(&url)?,
            date,
            file: None,
            db_type,
        })
    }

    pub async fn download_and_unzip(&self, download_path: &str, output_dir: &str) -> Result<File> {
        self.download_to_path(download_path).await?;
        Self::unzip_file(download_path, output_dir).await
    }

    pub async fn download_to_path(&self, path: &str) -> Result<()> {
        use futures::StreamExt;

        let parent = Path::new(path).parent().ok_or(anyhow::anyhow!(
            "Path {} does not have a parent directory",
            path
        ))?;
        tokio::fs::create_dir_all(&parent).await?;

        let response = reqwest::get(self.link.clone()).await?;
        let mut file = tokio::fs::File::create(path).await?;
        let mut content = response.bytes_stream();
        while let Some(chunk) = content.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Returns a relative path without reserved names, redundant separators, ".", or "..".
    fn sanitize_file_path(path: &str) -> PathBuf {
        // Replaces backwards slashes
        path.replace('\\', "/")
            // Sanitizes each component
            .split('/')
            .map(sanitize_filename::sanitize)
            .collect()
    }

    /// Extracts everything from the ZIP archive to the output directory
    pub async fn unzip_file(archive: &str, out_dir: &str) -> anyhow::Result<File> {
        let archive = File::open(archive).await?;
        let out_dir = Path::new(out_dir);
        let archive = BufReader::new(archive).compat();
        let mut reader = ZipFileReader::new(archive)
            .await
            .expect("Failed to read zip file");
        let entry = reader.file().entries().first().unwrap();
        let path = out_dir.join(Self::sanitize_file_path(entry.filename().as_str().unwrap()));
        let entry_is_dir = entry.dir().unwrap();

        let mut entry_reader = reader
            .reader_without_entry(0)
            .await
            .expect("Failed to read ZipEntry");

        if entry_is_dir && !path.exists() {
            create_dir_all(&path)
                .await
                .expect("Failed to create extracted directory");
            Err(anyhow!("Failed to create directory"))
        } else {
            // Creates parent directories. They may not exist if iteration is out of order
            // or the archive does not contain directory entries.
            let parent = path
                .parent()
                .expect("A file entry should have parent directories");
            if !parent.is_dir() {
                create_dir_all(parent)
                    .await
                    .expect("Failed to create parent directories");
            }
            let writer = OpenOptions::new()
                .write(true)
                .create(true) // TODO: Use create_new cause it is atomic and safer, then handle the AlreadyExists error thrown after the await here
                .open(&path)
                .await
                .expect("Failed to create extracted file");
            {
                futures_lite::io::copy(&mut entry_reader, &mut writer.compat_write())
                    .await
                    .expect("Failed to copy to extracted file");
            }
            Ok(File::open(&path).await?)
        }
    }

    pub async fn update_latest(&mut self) -> Result<()> {
        let tmp_dir = "./tmp";
        let output_dir = "./tmp/output";

        fs::create_dir_all(tmp_dir).await?;
        fs::create_dir_all(output_dir).await?;

        let now = chrono::Utc::now()
            .checked_sub_signed(Duration::hours(1))
            .unwrap();
        let rounded_minute = (now.minute() / 15) * 15;
        let rounded_time = now
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
            .with_minute(rounded_minute)
            .unwrap();

        let db_type_str = String::try_from(&self.db_type)?;
        let url = format!(
            "http://data.gdeltproject.org/gdeltv2/{:04}{:02}{:02}{:02}{:02}{:02}.{}.CSV.zip",
            rounded_time.year(),
            rounded_time.month(),
            rounded_time.day(),
            rounded_time.hour(),
            rounded_time.minute(),
            rounded_time.second(),
            db_type_str
        );

        self.link = reqwest::Url::parse(&url)?;
        self.date = rounded_time.naive_utc();
        self.db_type = DatabaseType::Events;

        Ok(())
    }

    pub async fn download_batch(
        start: NaiveDateTime,
        end: NaiveDateTime,
        db_type: DatabaseType,
        output_dir: &str,
    ) -> Result<()> {
        use chrono::Duration;

        let mut current = start;
        while current <= end {
            let db = GDELTDatabase::from_date_and_type(current, db_type.clone()).await?;
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
    use log::debug;

    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            let _ = env_logger::builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Trace)
                .write_style(env_logger::WriteStyle::Always)
                .try_init();
        });
    }

    #[tokio::test]
    async fn test_gdelt_database_new() {
        init_logger();
        let db_event = GDELTDatabase::new(DatabaseType::Events).await.unwrap();
        let db_mention = GDELTDatabase::new(DatabaseType::Mentions).await.unwrap();
        let db_gkg = GDELTDatabase::new(DatabaseType::GlobalKnowledgeGraph)
            .await
            .unwrap();
        assert_eq!(db_event.db_type, DatabaseType::Events);
        assert_eq!(db_mention.db_type, DatabaseType::Mentions);
        assert_eq!(db_gkg.db_type, DatabaseType::GlobalKnowledgeGraph);
        assert!(
            db_event.link.as_str().contains("gdeltv2"),
            "URL should contain 'gdeltv2'"
        );
    }

    #[tokio::test]
    async fn test_gdelt_database_new_invalid_url() {
        init_logger();
        let result = GDELTDatabase::from_url_str("http://invalid.url").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_download_to_path() -> Result<()> {
        init_logger();
        let url_event = "http://data.gdeltproject.org/gdeltv2/20250322180000.export.CSV.zip";
        let url_gkg = "http://data.gdeltproject.org/gdeltv2/20250322180000.gkg.csv.zip";
        let url_mention = "http://data.gdeltproject.org/gdeltv2/20250322180000.mention.CSV.zip";
        let db_event = GDELTDatabase::from_url_str(url_event).await?;
        let db_mention = GDELTDatabase::from_url_str(url_mention).await?;
        let db_gkg = GDELTDatabase::from_url_str(url_gkg).await?;
        debug!("Event: {db_event:?}");
        debug!("Mention: {db_mention:?}");
        debug!("GKG: {db_gkg:?}");
        let gkg_download_path = "./tmp/test/test_download_to_path/test_gkg.csv.zip";
        let mention_download_path = "./tmp/test/test_download_to_path/test_mention.csv.zip";
        let event_download_path = "./tmp/test/test_download_to_path/test_event.csv.zip";
        db_event.download_to_path(url_event).await?;
        db_mention.download_to_path(url_mention).await?;
        db_gkg.download_to_path(url_gkg).await?;
        assert!(Path::new(event_download_path).exists());
        assert!(Path::new(mention_download_path).exists());
        assert!(Path::new(gkg_download_path).exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_download_and_unzip() -> Result<()> {
        init_logger();
        let url_event = "http://data.gdeltproject.org/gdeltv2/20250322180000.export.CSV.zip";
        let url_gkg = "http://data.gdeltproject.org/gdeltv2/20250322180000.gkg.csv.zip";
        let url_mention = "http://data.gdeltproject.org/gdeltv2/20250322180000.mention.CSV.zip";
        let db_event = GDELTDatabase::from_url_str(url_event).await.unwrap();
        let db_mention = GDELTDatabase::from_url_str(url_mention).await.unwrap();
        let db_gkg = GDELTDatabase::from_url_str(url_gkg).await.unwrap();
        debug!("Event: {db_event:?}");
        debug!("Mention: {db_mention:?}");
        debug!("GKG: {db_gkg:?}");
        let gkg_download_path = "./tmp/test/test_download_and_unzip/test_gkg.csv.zip";
        let mention_download_path = "./tmp/test/test_download_and_unzip/test_mention.csv.zip";
        let event_download_path = "./tmp/test/test_download_and_unzip/test_event.csv.zip";
        let output_dir = "./tmp/test_download_and_unzip/";
        db_event
            .download_and_unzip(gkg_download_path, output_dir)
            .await?;
        db_mention
            .download_and_unzip(mention_download_path, output_dir)
            .await?;
        db_gkg
            .download_and_unzip(event_download_path, output_dir)
            .await?;
        let event_output_path = Path::new(output_dir).join("export.CSV");
        let gkg_output_path = Path::new(output_dir).join("mention.CSV");
        let mention_output_path = Path::new(output_dir).join("gkg.csv");
        assert!(event_output_path.exists());
        assert!(gkg_output_path.exists());
        assert!(mention_output_path.exists());
        Ok(())
    }

    #[tokio::test]
    async fn test_update_latest() -> Result<()> {
        init_logger();
        let mut db = GDELTDatabase::new(DatabaseType::Events).await.unwrap();
        db.update_latest().await?;
        assert!(
            db.link.as_str().contains("gdeltv2"),
            "Updated URL should contain 'gdeltv2'"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_from_date_and_type() -> Result<()> {
        init_logger();
        let date = NaiveDateTime::parse_from_str("2023-03-22 18:00:00", "%Y-%m-%d %H:%M:%S")?;
        let db = GDELTDatabase::from_date_and_type(date, DatabaseType::Mentions).await?;
        assert_eq!(db.db_type, DatabaseType::Mentions);
        assert!(db.link.as_str().contains("20230322180000.mentions.CSV.zip"));
        Ok(())
    }
}
