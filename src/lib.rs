#![feature(duration_constructors)]
#![feature(duration_constructors_lite)]

//TODO: Defs clean this up:
// 1. Smaller functions
// 2. So much logic is rewritten and can be encapsulated (like the file finding and cleanup and other shit). do that
// 3. Most stuff about TODOing is in the submodules `data` (../data) and `models` (../models)

use csv::ReaderBuilder;
use data::GDELTDatabase;
use log::{debug, warn};
use models::types::event_table::EventTable;
use models::types::mention_table::MentionTable;
use std::fs;

use anyhow::Context;
use std::path::PathBuf;

use anyhow::Result;

pub async fn fetch_and_parse_mentions() -> Result<Vec<MentionTable>> {
    // Set up temporary directories and files
    let tmp_dir = "./tmp/mention";
    let zip_path = format!("{tmp_dir}/latest_download.zip");
    let output_dir = format!("{tmp_dir}/output");

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Attempt to find existing CSV files
    let mut csv_files = fs::read_dir(&output_dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                path.clone().extension().and_then(|ext| {
                    if ext.to_str().eq(&Some("csv")) || ext.to_str().eq(&Some("CSV")) {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<Vec<PathBuf>>();

    let csv_path = csv_files
        .iter()
        .find(|path| {
            // Check for mention files and exclude others
            path.file_name()
                .and_then(|f| f.to_str())
                .and_then(|s| (s.contains("mentions") && !s.contains("event")).then_some(()))
                .is_some()
        })
        .cloned();

    let csv_path = match csv_path {
        Some(path) => path,
        None => {
            // No valid mention CSV found, proceed to download
            let db = crate::GDELTDatabase::new(data::DatabaseType::Mentions)
                .await
                .expect("Failed to initialize database for mentions");
            db.download_and_unzip(&zip_path, &output_dir)
                .await
                .expect("Failed to download and unzip mentions");

            // Locate the newly downloaded CSV file
            fs::read_dir(&output_dir)
                .expect("Failed to read output directory after download")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        path.clone()
                            .extension()
                            .and_then(|ext| (ext == "csv" || ext == "CSV").then_some(path))
                    })
                })
                .next()
                .with_context(|| "No CSV file found after download")
                .expect(
                    "There was an issue over here right here by the mentions with the file reading",
                )
        }
    };

    // Validate and parse the CSV
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(&csv_path)?;

    let mut results = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() != 16 {
            warn!(
                "Skipping mention record with unexpected field count (found {}, expected 16)",
                record.len()
            );
            continue;
        }
        match MentionTable::try_from(record) {
            Ok(mention) => {
                // debug!("Parsed Mention: {:?}", mention);
                results.push(mention);
            }
            Err(e) => {
                warn!("Error parsing mention: {}", e);
            }
        }
    }

    // Optional: Remove temporary files
    fs::remove_file(&csv_path)?;
    fs::remove_file(&zip_path)?;

    Ok(results)
}

pub async fn fetch_and_parse_events() -> Result<Vec<EventTable>> {
    // Set up temporary directories and files
    let tmp_dir = "./tmp/event";
    let zip_path = format!("{tmp_dir}/latest_download.zip");
    let output_dir = format!("{tmp_dir}/output");

    // Create directories if they don't exist
    fs::create_dir_all(&output_dir)?;
    fs::create_dir_all(format!("./{tmp_dir}/event"))?;

    // Look for existing CSV files
    let mut csv_files = fs::read_dir(&output_dir)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                path.clone().extension().and_then(|ext| {
                    if ext.to_str().eq(&Some("csv")) || ext.to_str().eq(&Some("CSV")) {
                        Some(path)
                    } else {
                        None
                    }
                })
            })
        })
        .collect::<Vec<PathBuf>>();

    let csv_path = csv_files
        .iter()
        .find(|path| {
            // Ensure it's an event file and not a mention/file
            let filename = path.file_name();
            filename
                .and_then(|f| f.to_str())
                .and_then(|s| {
                    (s.contains("export") && !s.contains("gkg") && !s.contains("mentions"))
                        .then_some(())
                })
                .is_some()
        })
        .cloned();

    let csv_path = match csv_path {
        Some(path) => path,
        None => {
            // No valid event CSV found; download the latest
            let db = crate::GDELTDatabase::new(data::DatabaseType::Events)
                .await
                .expect("Failed to initialize database for events");
            db.download_and_unzip(&zip_path, &output_dir)
                .await
                .expect("Failed to download and unzip events");

            fs::read_dir(&output_dir)
                .expect("Failed to read output directory after download")
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        path.clone()
                            .extension()
                            .and_then(|ext| (ext == "csv" || ext == "CSV").then_some(path))
                    })
                })
                .next()
                .with_context(|| "No CSV file found after download")
                .expect("There was an error reading the events over here")
        }
    };

    // Validate and parse the CSV
    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(&csv_path)?;

    let mut results = Vec::new();
    for result in rdr.records() {
        let record = result?;
        if record.len() != 61 {
            warn!(
                "Skipping event record with unexpected field count (found {}, expected 61)",
                record.len()
            );
            continue;
        }
        match EventTable::try_from(record) {
            Ok(event) => {
                // debug!("Parsed Event: {:?}", event);
                results.push(event);
            }
            Err(e) => {
                warn!("Error parsing event: {}", e);
            }
        }
    }

    // Optional: Clean up temporary files
    fs::remove_file(&csv_path)?;
    fs::remove_file(&zip_path)?;

    Ok(results)
}

#[cfg(test)]
mod verbose_tests {
    use super::*;
    use chrono::NaiveDate;
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

    #[tokio::test]
    async fn test_gdelt_database_new_valid_url() {
        init_logger();
        let url = "http://data.gdeltproject.org/gdeltv2/20211021000000.mentions.CSV.zip";
        let db = GDELTDatabase::from_url_str(url)
            .await
            .expect("Should create GDELTDatabase from valid URL");
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

    #[tokio::test]
    async fn test_gdelt_database_new_invalid_url() {
        init_logger();
        let url = "http://invalid.url";
        let result = GDELTDatabase::from_url_str(url).await;
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

    #[tokio::test]
    async fn test_gdelt_database_default() {
        let db = GDELTDatabase::new(DatabaseType::Mentions)
            .await
            .expect("Messuo");
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

    #[tokio::test]
    async fn test_from_date_and_type_valid() {
        let date = NaiveDate::from_ymd_opt(2023, 3, 22)
            .unwrap()
            .and_hms_opt(18, 0, 0)
            .unwrap();
        let db = GDELTDatabase::from_date_and_type(date, DatabaseType::Mentions)
            .await
            .unwrap();
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
        let db = GDELTDatabase::from_url_str(url).await.unwrap();
        let zip_path = "./tmp/test_download.zip";
        let output_dir = "./tmp/test_output";

        // Download the file
        let download_result = db.download_to_path(zip_path).await;
        assert!(download_result.is_ok(), "Download should succeed");

        // Unzip the file
        let unzip_result = GDELTDatabase::unzip_file(zip_path, output_dir).await;
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
        let mut db = GDELTDatabase::new(DatabaseType::Events)
            .await
            .expect("Fuuuuuuuuck");
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
        let db = GDELTDatabase::from_url_str(url).await;
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

    use std::sync::Once;

    use super::*;

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

    fn log_first_ten_fields<T: std::fmt::Debug>(label: &str, items: &[T]) {
        for (i, item) in items.iter().take(10).enumerate() {
            log::info!("{} {}: {:?}\n\n", label, i, item);
        }
    }

    #[tokio::test]
    async fn test_fetch_and_parse_events() -> anyhow::Result<()> {
        init_logger();
        let events = fetch_and_parse_events().await?;

        // Log the first ten fields of every created EventTable
        log_first_ten_fields("Event", &events);

        assert_ne!(events.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_and_parse_mentions() -> anyhow::Result<()> {
        init_logger();
        // Call fetch_and_parse_mentions
        let mentions = fetch_and_parse_mentions().await?;

        // Log the first ten fields of every created MentionTable
        log_first_ten_fields("Mention", &mentions);

        assert_ne!(mentions.len(), 0);

        Ok(())
    }
}
