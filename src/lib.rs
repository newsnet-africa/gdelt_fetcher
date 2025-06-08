#![feature(duration_constructors)]

use csv::ReaderBuilder;
use data::GDELTDatabase;
use log::debug;
use models::types::event_table::EventTable;
use models::types::mention_table::MentionTable;
use std::fs;
use std::fs::File;
use std::path::Path;

pub async fn fetch_and_parse_mentions() -> anyhow::Result<Vec<MentionTable>> {
    // Ensure tmp directory exists
    let tmp_dir = "./tmp";
    let zip_path = format!("{}/latest_download.zip", tmp_dir);
    let output_dir = format!("{}/output", tmp_dir);

    fs::create_dir_all(&output_dir)?;

    // Download and unzip only if file doesn't exist
    let csv_path = {
        // Check if a CSV already exists in output_dir
        let existing_csv = fs::read_dir(&output_dir)?
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|e| e == "csv")
                    .unwrap_or(false)
            })
            .map(|entry| entry.path());

        if let Some(csv) = existing_csv {
            csv
        } else {
            // Download and unzip
            let db = crate::GDELTDatabase::new(data::DatabaseType::Mentions)?;
            let _unzipped_file = db.download_and_unzip(&zip_path, &output_dir).await?;

            // Find the unzipped CSV file
            fs::read_dir(&output_dir)?
                .filter_map(|entry| entry.ok())
                .find(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|e| e == "csv")
                        .unwrap_or(false)
                })
                .map(|entry| entry.path())
                .ok_or_else(|| anyhow::anyhow!("No CSV file found after unzipping"))?
        }
    };

    // Parse CSV into MentionTable objects
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(&csv_path)?;
    let mut results = Vec::new();
    for result in rdr.records() {
        let record = result?;
        match MentionTable::try_from(record) {
            Ok(mention) => {
                debug!("MentionTable parsed: {mention:?}");
                results.push(mention)
            }
            Err(e) => log::warn!("Failed to parse event: {e}"),
        }
    }

    // Clean up: delete CSV and ZIP files
    fs::remove_file(&csv_path)?;
    fs::remove_file(&zip_path)?;

    Ok(results)
}

pub async fn fetch_and_parse_events() -> anyhow::Result<Vec<EventTable>> {
    // Ensure paths are initialized
    let tmp_dir = "./tmp";
    let zip_path = format!("{}/latest_download.zip", tmp_dir);
    let output_dir = format!("{}/output", tmp_dir);
    fs::create_dir_all(&output_dir)?;

    // Download and unzip only if file doesn't exist
    let csv_path = {
        // Check if a CSV already exists in output_dir
        let existing_csv = fs::read_dir(&output_dir)?
            .filter_map(|entry| entry.ok())
            .find(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|e| e == "csv")
                    .unwrap_or(false)
            })
            .map(|entry| entry.path());

        if let Some(csv) = existing_csv {
            csv
        } else {
            let db = crate::GDELTDatabase::new(data::DatabaseType::Events)?;

            // Download and unzip the latest events file
            let _unzipped_file = db.download_and_unzip(&zip_path, &output_dir).await?;

            // Find the first CSV file in the output directory
            fs::read_dir(&output_dir)?
                .filter_map(|entry| entry.ok())
                .find(|entry| {
                    entry
                        .path()
                        .extension()
                        .map(|e| e == "csv")
                        .unwrap_or(false)
                })
                .map(|entry| entry.path())
                .ok_or_else(|| anyhow::anyhow!("No CSV file found in output directory"))?
        }
    };

    // Open and parse the CSV
    let file = File::open(&csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(file);

    let mut events = Vec::new();
    for result in rdr.records() {
        let record = result?;
        match EventTable::try_from(record) {
            Ok(event) => {
                debug!("EventTable parsed: {event:?}");
                events.push(event)
            }
            Err(e) => log::warn!("Failed to parse event: {e}"),
        }
    }

    // Clean up: delete CSV and ZIP files
    fs::remove_file(&csv_path).ok();
    fs::remove_file(&zip_path).ok();

    Ok(events)
}
#[cfg(test)]
mod verbose_tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use data::DatabaseType;
    use std::fs;
    use std::io::Read;
    use std::time::Duration;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = env_logger::builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Trace)
                .try_init();
        });
    }

    #[test]
    fn test_gdelt_database_new_valid_url() {
        init_logger();
        let url = "http://data.gdeltproject.org/gdeltv2/20211021000000.mentions.CSV.zip";
        let db =
            GDELTDatabase::from_url_str(url).expect("Should create GDELTDatabase from valid URL");
        assert_eq!(
            db.db_type,
            DatabaseType::Mentions,
            "Database type should be Mentions"
        );
        assert_eq!(
            db.date,
            NaiveDate::from_ymd_opt(2021, 10, 21)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            "Date should be parsed correctly"
        );
        assert_eq!(db.link.as_str(), url, "URL should be set correctly");
        assert!(db.file.is_none(), "File should be None on creation");
    }

    #[test]
    fn test_gdelt_database_new_invalid_url() {
        init_logger();
        let url = "http://invalid.url";
        let result = GDELTDatabase::from_url_str(url);
        assert!(result.is_err(), "Should fail with invalid URL");
    }

    #[test]
    fn test_database_type_try_from() {
        assert_eq!(
            DatabaseType::try_from("export").unwrap(),
            DatabaseType::Events
        );
        assert_eq!(
            DatabaseType::try_from("gkg").unwrap(),
            DatabaseType::GlobalKnowledgeGraph
        );
        assert_eq!(
            DatabaseType::try_from("mentions").unwrap(),
            DatabaseType::Mentions
        );
        assert!(DatabaseType::try_from("unknown").is_err());
    }

    #[test]
    fn test_database_type_to_string() {
        assert_eq!(String::try_from(&DatabaseType::Events).unwrap(), "export");
        assert_eq!(
            String::try_from(&DatabaseType::GlobalKnowledgeGraph).unwrap(),
            "gkg"
        );
        assert_eq!(
            String::try_from(&DatabaseType::Mentions).unwrap(),
            "mentions"
        );
    }

    #[test]
    fn test_gdelt_database_default() {
        let db = GDELTDatabase::new(DatabaseType::Mentions).expect("Messuo");
        assert_eq!(
            db.db_type,
            DatabaseType::Mentions,
            "Default db_type should be Mentions"
        );
        assert!(
            db.link.as_str().contains("mentions.CSV.zip"),
            "Default URL should be for mentions"
        );
    }

    #[test]
    fn test_from_date_and_type_valid() {
        let date = NaiveDate::from_ymd_opt(2023, 3, 22)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap();
        let db = GDELTDatabase::from_date_and_type(date, DatabaseType::Mentions).unwrap();
        assert_eq!(db.db_type, DatabaseType::Mentions);
        assert_eq!(db.date, date);
        assert!(db.link.as_str().contains("20230322180000.mentions.CSV.zip"));
    }

    #[test]
    fn test_from_date_and_type_invalid_date() {
        let date = NaiveDate::from_ymd_opt(2023, 2, 30); // Invalid date
        assert!(date.is_none(), "Invalid date should return None");
    }

    #[tokio::test]
    async fn test_download_to_path_and_unzip() {
        init_logger();
        let url = "http://data.gdeltproject.org/gdeltv2/20250322180000.mentions.CSV.zip";
        let db = GDELTDatabase::from_url_str(url).unwrap();
        let zip_path = "./tmp/test_download.zip";
        let output_dir = "./tmp/test_output";

        // Download the file
        let download_result = db.download_to_path(zip_path).await;
        assert!(download_result.is_ok(), "Download should succeed");

        // Unzip the file
        let unzip_result = GDELTDatabase::unzip_single_file(zip_path, output_dir);
        assert!(unzip_result.is_ok(), "Unzip should succeed");

        // Check that the file exists in the output directory
        let output_files = fs::read_dir(output_dir).unwrap();
        let mut found = false;
        for entry in output_files {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                found = true;
                let mut file = fs::File::open(entry.path()).unwrap();
                let mut buf = [0u8; 10];
                let _ = file.read(&mut buf).unwrap();
                // Just check we can read some bytes
            }
        }
        assert!(
            found,
            "Should find at least one file in the output directory"
        );
    }

    #[tokio::test]
    async fn test_update_latest_sets_fields_and_downloads() {
        init_logger();
        let mut db = GDELTDatabase::new(DatabaseType::Events).expect("Fuck");
        let old_url = db.link.clone();
        tokio::time::sleep(Duration::from_mins(15));
        let result = db.update_latest().await;
        assert!(result.is_ok(), "update_latest should succeed");
        assert!(
            db.link.as_str().contains("gdeltv2"),
            "URL should be updated"
        );
        // assert_ne!(db.link, old_url, "URL should change after update");
    }

    #[tokio::test]
    async fn test_download_and_unzip_invalid_url() {
        init_logger();
        let url = "http://invalid.url";
        let db = GDELTDatabase::from_url_str(url);
        assert!(
            db.is_err(),
            "Should fail to create GDELTDatabase with invalid URL"
        );
    }
}

// #[cfg(test)]
// mod integration_tests {
//     use super::*;
//     use csv::ReaderBuilder;
//     use models::types::mention_table::MentionTable;
//     use std::fs;
//     use std::path::Path;

//     // Mock GDELTDatabase to avoid actual downloads
//     struct MockGDELTDatabase;

//     impl MockGDELTDatabase {
//         async fn download_and_unzip(
//             &self,
//             _zip_path: &str,
//             output_dir: &str,
//         ) -> anyhow::Result<()> {
//             // Create a dummy CSV file for testing
//             let csv_content = "GLOBALEVENTID\tMentionTimeDate\tMentionType\tMentionSourceName\tMentionIdentifier\tSentenceID\tActor1CharOffset\tActor2CharOffset\tActionCharOffset\tInRawText\tConfidence\tMentionDocLen\tMentionDocTone\tMentionDocOffsets\tMentionDocIDs\tMentionDocTitles\tMentionDocSources\tMentionDocLocations\tMentionDocCountries\tMentionDocLanguages\tMentionDocThemes\tMentionDocPersons\tMentionDocOrganizations\tMentionDocGeo_Type\tMentionDocGeo_FullName\tMentionDocGeo_CountryCode\tMentionDocGeo_ADM1Code\tMentionDocGeo_Lat\tMentionDocGeo_Long\tMentionDocGeo_FeatureID\n\
//                              1\t20240101\t1\tTestSource\tTestIdentifier\t1\t1\t2\t3\t1\t50\t100\t0.5\t1,2,3\tDoc1,Doc2\tTitle1,Title2\tSource1,Source2\tLocation1,Location2\tCountry1,Country2\tLanguage1,Language2\tTheme1,Theme2\tPerson1,Person2\tOrganization1,Organization2\t1\tTestGeo\tUS\tCA\t34.0522\t-118.2437\t12345";

//             let csv_path = Path::new(output_dir).join("test_mentions.csv");
//             fs::write(&csv_path, csv_content)?;

//             Ok(())
//         }
//     }

//     async fn setup_test_environment() -> anyhow::Result<(String, String)> {
//         let tmp_dir = "./tmp_test";
//         let output_dir = format!("{}/output", tmp_dir);

//         // Ensure the directories exist
//         fs::create_dir_all(&output_dir)?;

//         Ok((tmp_dir.to_string(), output_dir))
//     }

//     async fn cleanup_test_environment(tmp_dir: &str) -> anyhow::Result<()> {
//         // Clean up the temporary directory
//         fs::remove_dir_all(tmp_dir)?;
//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseMentions_parsesSuccessfullyWithValidData() -> anyhow::Result<()> {
//         let (tmp_dir, output_dir) = setup_test_environment().await?;

//         // Mock the download and unzip process
//         let db = MockGDELTDatabase;
//         db.download_and_unzip("test_zip.zip", &output_dir).await?;

//         // Call the function under test
//         let mentions = fetch_and_parse_mentions().await?;

//         // Assert that the mentions were parsed correctly
//         assert_eq!(mentions.len(), 1);
//         assert_eq!(mentions[0].global_event_id.0, 1);

//         // Clean up the test environment
//         cleanup_test_environment(&tmp_dir).await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseMentions_returnsErrorWhenNoCsvIsFound() -> anyhow::Result<()> {
//         let (tmp_dir, output_dir) = setup_test_environment().await?;

//         // Do not create a CSV file, simulating a missing CSV after unzipping

//         // Call the function under test
//         let result = fetch_and_parse_mentions().await;

//         // Assert that the function returns an error
//         assert!(result.is_err());
//         assert_eq!(
//             result.unwrap_err().to_string(),
//             "No CSV file found after unzipping"
//         );

//         // Clean up the test environment
//         cleanup_test_environment(&tmp_dir).await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseMentions_handlesEmptyCsvFileGracefully() -> anyhow::Result<()> {
//         let (tmp_dir, output_dir) = setup_test_environment().await?;

//         // Create an empty CSV file
//         let csv_path = Path::new(&output_dir).join("test_mentions.csv");
//         fs::write(&csv_path, "")?;

//         // Mock the download and unzip process
//         let db = MockGDELTDatabase;
//         db.download_and_unzip("test_zip.zip", &output_dir).await?;

//         // Call the function under test
//         let mentions = fetch_and_parse_mentions().await?;

//         // Assert that the function returns an empty vector
//         assert_eq!(mentions.len(), 0);

//         // Clean up the test environment
//         cleanup_test_environment(&tmp_dir).await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseEvents_parsesEventsSuccessfullyWithValidData() -> anyhow::Result<()> {
//         // Create a temporary directory and file for testing
//         let tmp_dir = "./tmp_events_test";
//         let output_dir = format!("{}/output", tmp_dir);
//         fs::create_dir_all(&output_dir)?;

//         let csv_content = "1233702893\t20240322\t202403\t2024\t2024.2247\tUSAGOV\tUNITED STATES\tUSA\t\t\t\tGOV\t\t\tUSA\tUNITED STATES\tUSA\t\t\t\t0\t050\t050\t05\t1\t3.5\t2\t1\t2\t-3.71155885471898\t2\tWashington, United States\tUS\tUSWA\t\t47.3917\t-121.571\tWA\t2\tWashington, United States\tUS\tUSWA\t\t47.3917\t-121.571\tWA\t2\tWashington, United States\tUS\tUSWA\t\t47.3917\t-121.571\tWA\t20250322180000\thttps://www.yakimaherald.com/news/northwest/wa-state-workers-slam-furloughs-other-pay-cut-plans-claiming-they-are-a-tax-on/article_e49c4f10-11a1-5b7a-b947-c49482ea1ae0.html\n".to_string();
//         let csv_path = Path::new(&output_dir).join("events_test.csv");
//         fs::write(&csv_path, csv_content)?;

//         // Mock the GDELTDatabase download and unzip
//         struct MockGDELTDatabase;
//         impl MockGDELTDatabase {
//             async fn download_and_unzip(
//                 &self,
//                 _zip_path: &str,
//                 csv_path: &str,
//             ) -> anyhow::Result<()> {
//                 // Simulate successful download and unzip
//                 Ok(())
//             }
//         }

//         // Call the function under test
//         let db = MockGDELTDatabase;
//         let events = fetch_and_parse_events().await?;

//         // Assert that the events were parsed correctly
//         assert_eq!(events.len(), 1);
//         assert_eq!(events[0].global_event_id.0, 1233702893);

//         // Clean up the test environment
//         fs::remove_dir_all(tmp_dir)?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseEvents_handlesInvalidEventDataGracefully() -> anyhow::Result<()> {
//         // Create a temporary directory and file for testing
//         let tmp_dir = "./tmp_events_test";
//         let output_dir = format!("{}/output", tmp_dir);
//         fs::create_dir_all(&output_dir)?;

//         // Invalid data: fewer fields than expected
//         let csv_content = "1233702893\t20240322\t202403\n".to_string();
//         let csv_path = Path::new(&output_dir).join("events_test.csv");
//         fs::write(&csv_path, csv_content)?;

//         // Mock the GDELTDatabase download and unzip
//         struct MockGDELTDatabase;
//         impl MockGDELTDatabase {
//             async fn download_and_unzip(
//                 &self,
//                 _zip_path: &str,
//                 csv_path: &str,
//             ) -> anyhow::Result<()> {
//                 // Simulate successful download and unzip
//                 Ok(())
//             }
//         }

//         // Call the function under test
//         let db = MockGDELTDatabase;
//         let events = fetch_and_parse_events().await?;

//         // Assert that no events were parsed due to the error, but no error was returned
//         assert_eq!(events.len(), 0);

//         // Clean up the test environment
//         fs::remove_dir_all(tmp_dir)?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn fetchAndParseEvents_handlesEmptyCsvFileGracefully() -> anyhow::Result<()> {
//         // Create a temporary directory and file for testing
//         let tmp_dir = "./tmp_events_test";
//         let output_dir = format!("{}/output", tmp_dir);
//         fs::create_dir_all(&output_dir)?;

//         // Empty CSV content
//         let csv_content = "".to_string();
//         let csv_path = Path::new(&output_dir).join("events_test.csv");
//         fs::write(&csv_path, csv_content)?;

//         // Mock the GDELTDatabase download and unzip
//         struct MockGDELTDatabase;
//         impl MockGDELTDatabase {
//             async fn download_and_unzip(
//                 &self,
//                 _zip_path: &str,
//                 csv_path: &str,
//             ) -> anyhow::Result<()> {
//                 // Simulate successful download and unzip
//                 Ok(())
//             }
//         }

//         // Call the function under test
//         let db = MockGDELTDatabase;
//         let events = fetch_and_parse_events().await?;

//         // Assert that no events were parsed, but no error was returned
//         assert_eq!(events.len(), 0);

//         // Clean up the test environment
//         fs::remove_dir_all(tmp_dir)?;

//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::path::PathBuf;
    use std::{
        fs,
        sync::{Arc, Mutex},
    };

    // Mock GDELTDatabase
    #[derive(Debug, Default, Clone)]
    struct MockGDELTDatabase {
        pub downloaded_data: Arc<Mutex<Option<String>>>,
    }

    impl MockGDELTDatabase {
        pub fn set_downloaded_data(&self, data: String) {
            let mut downloaded_data = self.downloaded_data.lock().unwrap();
            *downloaded_data = Some(data);
        }
    }

    #[async_trait]
    trait GDELTDownloader {
        async fn download_and_unzip(
            &self,
            zip_path: &str,
            output_path: &str,
        ) -> anyhow::Result<PathBuf>;
    }

    #[async_trait]
    impl GDELTDownloader for MockGDELTDatabase {
        async fn download_and_unzip(
            &self,
            _zip_path: &str,
            output_path: &str,
        ) -> anyhow::Result<PathBuf> {
            let downloaded_data = self.downloaded_data.lock().unwrap();
            let data = downloaded_data
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("No data was downloaded"))?;

            fs::write(output_path, data)?;
            Ok(PathBuf::from(output_path))
        }
    }

    fn log_first_ten_fields<T: std::fmt::Debug>(label: &str, items: &[T]) {
        for (i, item) in items.iter().enumerate() {
            log::info!("{} {}: {:?}", label, i, item);
        }
    }

    #[tokio::test]
    async fn test_fetch_and_parse_events() -> anyhow::Result<()> {
        let mock_data = "123\t20240101\t2024\t2024\t2024.0\tUSA\tUSA\tUSA\t\t\t\t\t\t\tUSA\tUSA\tUSA\t\t\t\t\t\t\t0\t0\t0\t0\t0\t0.0\t0\t0\t0\t0.0\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t20240101000000\thttps://example.com".to_string();

        let mut mock_db = MockGDELTDatabase::default();

        mock_db.set_downloaded_data(mock_data);

        // Call fetch_and_parse_events
        let events = fetch_and_parse_events().await?;

        // Log the first ten fields of every created EventTable
        log_first_ten_fields("Event", &events);

        assert_ne!(events.len(), 0);
        // assert_eq!(events[0].global_event_id.0, 123);
        // assert_eq!(events[0].src_url.as_str(), "https://example.com/");

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_and_parse_mentions() -> anyhow::Result<()> {
        let mock_mention_data =
            "123\t20240101\t20240101\t1\tsource\t1\t1\t2\t3\ttext\t100\t20\t0.5\ten\tengine"
                .to_string();

        let mock_db = MockGDELTDatabase::default();
        mock_db.set_downloaded_data(mock_mention_data);

        // Call fetch_and_parse_mentions
        let mentions = fetch_and_parse_mentions().await?;

        // Log the first ten fields of every created MentionTable
        log_first_ten_fields("Mention", &mentions);

        assert_eq!(mentions.len(), 1);
        assert_eq!(mentions[0].global_event_id.0, 123);
        assert_eq!(mentions[0].confidence.0, 100);

        Ok(())
    }
}
