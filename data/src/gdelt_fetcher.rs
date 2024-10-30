// region Modules and Imports

use chrono::prelude::*;
use downloader::verify::Verification;
use once_cell::sync::Lazy;
use reqwest;
/// Importing necessary modules and crates for the GDELT file fetcher functionality.
use std::io::Cursor;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs::{self, File, OpenOptions};
use tokio::sync::Mutex;
use tokio::task;
use zip::read::ZipArchive;

use std::io::{self, Write, Read};
// endregion

// region Static Variables

/// The folder path where downloaded CSV files will be stored.
pub(crate) static DOWNLOAD_PATH_FOLDER: Lazy<Mutex<PathBuf>> =
    Lazy::new(|| Mutex::new(PathBuf::from("./data/csv/")));

/// The base URL for the GDELT project data.
static BASE_URL: Lazy<&str> = Lazy::new(|| "http://data.gdeltproject.org/gdeltv2/");
// endregion

// region Enums

/// Enum representing the type of data to fetch.
#[derive(Debug, Clone)]
pub enum FetchType {
    EVENTS,
    MENTIONS,
    GKG,
}

/// Enum representing possible errors during verification.
#[derive(Debug, Error)]
pub enum VerificationError {
    /// Error during setup.
    #[error("Setup error: {0}")]
    Setup(String),
    /// Error during verification.
    #[error("Verification error: {0:?}")]
    Verification(Verification),
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// Reqwest error.
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}
// endregion

// region Structs

/// Struct representing a GDELT file fetcher.
pub struct GdeltFileFetcher {
    /// The date of the last fetch.
    last_fetch_date: Option<DateTime<Local>>,
    /// The file name of the last fetch.
    last_fetch_file: Option<String>,
    /// The HTTP client for making requests.
    request: reqwest::Client,
}
// endregion

// region Implementations
impl GdeltFileFetcher {
    // region Constructor
    
    // region Getters and Setters
    
    // region Methods
    

    /// Creates a new `GdeltFileFetcher`.
    pub fn new() -> Self {
        Self {
            last_fetch_date: None,
            last_fetch_file: None,
            request: reqwest::Client::new(),
        }
    }
    
    // endregion
    
    /// Returns the date of the last fetch.
    pub fn last_fetch_date(&self) -> &Option<DateTime<Local>> {
        &self.last_fetch_date
    }
    
    /// Returns the file name of the last fetch.
    pub fn last_fetch_file(&self) -> &Option<String> {
        &self.last_fetch_file
    }
    
    /// Returns the HTTP client.
    pub fn request(&self) -> &reqwest::Client {
        &self.request
    }
    
    /// Sets the date of the last fetch.
    pub fn set_last_fetch_date(&mut self, last_fetch_date: Option<DateTime<Local>>) {
        self.last_fetch_date = last_fetch_date;
    }
    
    /// Sets the file name of the last fetch.
    pub fn set_last_fetch_file(&mut self, last_fetch_file: Option<String>) {
        self.last_fetch_file = last_fetch_file;
    }
    /// Sets the HTTP client.
    pub fn set_request(&mut self, request: reqwest::Client) {
        self.request = request;
    }

    // endregion
    
    /// Verifies the downloaded file by checking its metadata.
    ///
    /// This function checks if the file exists and has a non-zero size.
    /// If the file does not exist or has a zero size, it returns `Verification::Failed`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the downloaded file.
    ///
    /// # Returns
    ///
    /// A `Verification` result indicating the status of the verification.
    async fn verify_download(path: PathBuf) -> Verification {
        // Try to get the metadata of the file
        match fs::metadata(&path).await {
            // If the metadata is available, check the file size
            Ok(metadata) => {
                // If the file size is greater than zero, return `Verification::Ok`
                if metadata.len() > 0 {
                    Verification::Ok
                } else {
                    // If the file size is zero, print an error message and return `Verification::Failed`
                    println!("Metadata was found but is like non-existent?");
                    Verification::Failed
                }
            }
            // If the metadata is not available, print an error message and return `Verification::Failed`
            Err(e) => {
                println!("Metadata does not exist. Error: {}", e);
                Verification::Failed
            }
        }
    }

fn prepend_line_to_file(file_path: &str) -> &str {
    let ftype = file_path.split('.').collect::<Vec<&str>>()[1];
    
    let line = match ftype {
        "export" => "GlobalEventsID\tDay\tMonthYear\tYear\tFractionDate\tActor1Code\tActor1Name\tActor1CountryCode\tActor1KnownGroupCode\tActor1EthnicCode\tActor1Religion1Code\tActor1Religion2Code\tActor1Type1Code\tActor1Type2Code\tActor1Type3Code\tActor2Code\tActor2Name\tActor2CountryCode\tActor2KnownGroupCode\tActor2EthnicCode\tActor2Religion1Code\tActor2Religion2Code\tActor2Type1Code\tActor2Type2Code\tActor2Type3Code\tisRootEvent\tEventCode\tEventBaseCode\tEventRootCode\tQuadClass\tGoldstien\tNumMentions\tNumSources\tNumArticles\tAvgTone\tActor1GeoType\tActor1Geo_Fullname\tActor1Geo_CountryCode\tActor1GeoADM1Code\tActor1Geo_ADM2Code\tActor1Geo_Lat\tActor1Geo_Long\tActor1GeoFeatureID\tActor2GeoType\tActor2Geo_Fullname\tActor2Geo_CountryCode\tActor2GeoADM1Code\tActor2Geo_ADM2Code\tActor2Geo_Lat\tActor2Geo_Long\tActor2GeoFeatureID\tActionGeoType\tActionGeo_Fullname\tActionGeo_CountryCode\tActionGeoADM1Code\tActionGeo_ADM2Code\tActionGeo_Lat\tActionGeo_Long\tActionGeoFeatureID\tDateAdded\tSourceURL",
        "gkg" => "GKGRECORD\tV2.1DATE\tV2SOURCECOLLECTIONIDENTIFIER\tV2SOURCECOMMONNAME\tV2DOCUMENTIDENTIFIER\tV1COUNTS\tV2.1COUNTS\tV1THEMES\tV2ENHANCEDTHEMES\tV1LOCATIONS\tV2ENHANCEDLOCATIONS\tV1PERSONS\tV2ENHANCEDPERSONS\tV1ORGANISATIONS\tV2ENHANCEDORGANISATIONS\tV1.5TONE\tV2.1ENHANCEDDATES\tV2GCAM\tV2.1SHARINGIMAGE\tV2.1RELATEDIMAGES\tV2.1SOCIALVIDEOEMBEDS\tV2.1SOCIALVIDEOEMBEDS\tV2.1QUOTATIONS\tV2.1ALLNAMES\tV2.1AMOUNTS\tV2.1TRANSLATIONIFO\tV2EXTRASXML",
        "mentions" => "",
        _ => "",
    };
    
    line
}

    /// Extracts a ZIP file to the current directory.
    ///
    /// This function uses the `zip` crate to extract the ZIP file.
    /// It creates a new directory for the extracted files if necessary.
    ///
    /// # Arguments
    ///
    /// * `zip_path` - The path to the ZIP file.
    ///
    /// # Returns
    ///
    /// A `std::io::Result` indicating the status of the extraction.
    async fn extract_zip(zip_path: &str) -> std::io::Result<()> {
        // Convert the zip path to a string
        let zip_path = zip_path.to_string();

        // Spawn a blocking task to extract the ZIP file
        task::spawn_blocking(move || {
            // Open the ZIP file
            let std_file = std::fs::File::open(&zip_path)?;

            // Create a new ZIP archive
            let mut archive = ZipArchive::new(std_file)?;

            // Get the parent directory of the ZIP file
            let zip_directory = Path::new(&zip_path).parent().unwrap_or(Path::new(""));

            // Iterate over the files in the ZIP archive
            for i in 0..archive.len() {
                // Get the file at the current index
                let mut file = archive.by_index(i)?;

                // Get the file name and create a new path for the extracted file
                let mut outpath = PathBuf::from(zip_directory).join(file.enclosed_name().unwrap());

                // Convert the file name to lowercase
                if let Some(filename) = outpath.file_name() {
                    let lowercase_filename = filename.to_string_lossy().to_lowercase();
                    outpath.set_file_name(lowercase_filename);
                }

                // If the file is a directory, create it
                if file.name().ends_with('/') {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    // If the file is not a directory, create its parent directory if necessary
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(&p)?;
                        }
                    }

                    // Read the file contents
                    let mut buffer = Vec::new();
                    std::io::copy(&mut file, &mut buffer)?;

                    // Extract the full extension
                    let full_extension = outpath.file_name()
                        .and_then(|name| name.to_str())
                        .and_then(|name| name.split('.').skip(1).collect::<Vec<&str>>().join(".").into());

                    // Determine the file type and prepend the appropriate header
                    let header = match full_extension.as_deref() {
                        Some("export.csv") => "GlobalEventsID\tDay\tMonthYear\tYear\tFractionDate\tActor1Code\tActor1Name\tActor1CountryCode\tActor1KnownGroupCode\tActor1EthnicCode\tActor1Religion1Code\tActor1Religion2Code\tActor1Type1Code\tActor1Type2Code\tActor1Type3Code\tActor2Code\tActor2Name\tActor2CountryCode\tActor2KnownGroupCode\tActor2EthnicCode\tActor2Religion1Code\tActor2Religion2Code\tActor2Type1Code\tActor2Type2Code\tActor2Type3Code\tisRootEvent\tEventCode\tEventBaseCode\tEventRootCode\tQuadClass\tGoldstien\tNumMentions\tNumSources\tNumArticles\tAvgTone\tActor1GeoType\tActor1Geo_Fullname\tActor1Geo_CountryCode\tActor1GeoADM1Code\tActor1Geo_ADM2Code\tActor1Geo_Lat\tActor1Geo_Long\tActor1GeoFeatureID\tActor2GeoType\tActor2Geo_Fullname\tActor2Geo_CountryCode\tActor2GeoADM1Code\tActor2Geo_ADM2Code\tActor2Geo_Lat\tActor2Geo_Long\tActor2GeoFeatureID\tActionGeoType\tActionGeo_Fullname\tActionGeo_CountryCode\tActionGeoADM1Code\tActionGeo_ADM2Code\tActionGeo_Lat\tActionGeo_Long\tActionGeoFeatureID\tDateAdded\tSourceURL\n",
                        Some("gkg.csv") => "GKGRECORD\tV2.1DATE\tV2SOURCECOLLECTIONIDENTIFIER\tV2SOURCECOMMONNAME\tV2DOCUMENTIDENTIFIER\tV1COUNTS\tV2.1COUNTS\tV1THEMES\tV2ENHANCEDTHEMES\tV1LOCATIONS\tV2ENHANCEDLOCATIONS\tV1PERSONS\tV2ENHANCEDPERSONS\tV1ORGANISATIONS\tV2ENHANCEDORGANISATIONS\tV1.5TONE\tV2.1ENHANCEDDATES\tV2GCAM\tV2.1SHARINGIMAGE\tV2.1RELATEDIMAGES\tV2.1SOCIALVIDEOEMBEDS\tV2.1SOCIALVIDEOEMBEDS\tV2.1QUOTATIONS\tV2.1ALLNAMES\tV2.1AMOUNTS\tV2.1TRANSLATIONIFO\tV2EXTRASXML\n",
                        Some("mentions.csv") => "GLOBALEVENTID\tEventTimeDate\tMentionTimeDate\tMentionType\tMentionSourceName\tMentionIdentifier\tSentenceID\tActor1CharOffset\tActor2CharOffset\tActionCharOffset\tInRawText\tConfidence\tMentionDocLen\tMentionDocTone\tMentionDocTranslationInfo\tExtras\n",
                        _ => "",
                    };

                    // Prepend the header to the buffer
                    let mut output_buffer = header.as_bytes().to_vec();
                    output_buffer.extend_from_slice(&buffer);

                    // Write the buffer to the extracted file
                    std::fs::write(outpath, output_buffer)?;
                }
            }

            // Remove the ZIP file
            std::fs::remove_file(&zip_path)?;

            // Return Ok if the extraction was successful
            Ok(())
        })
        .await?
    }
    
    /// Checks if a file exists.
    ///
    /// This function uses the `std::path::Path` trait to check if the file exists.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the file exists.
    async fn file_exists(file_path: &str) -> bool {
        // Create a new path from the file path
        Path::new(file_path).exists()
    }
    
    /// Returns the current time as a string formatted for the GDELT project.
    ///
    /// This function uses the `chrono` crate to get the current time and format it as a string.
    ///
    /// # Returns
    ///
    /// A string representing the current time.
    pub fn last_update_string() -> String {
        // Get the current time
        let now = Local::now().with_timezone(&Utc).with_second(0).unwrap();
        
        // Round the minute to the nearest 15-minute interval
        let minute = now.minute();
        let rounded_minute = minute - (minute % 15);
        
        // Create a new time with the rounded minute
        let update_time = now.with_minute(rounded_minute).unwrap();
        
        // Format the time as a string
        format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}",
            update_time.year(),
            update_time.month(),
            update_time.day(),
            update_time.hour(),
            update_time.minute(),
            update_time.second()
        )
    }
    
    /// Fetches a file from the given URL.
    ///
    /// This function uses the `reqwest` crate to send a GET request to the URL and download the file.
    /// It then verifies the downloaded file using the `verify_download` function.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to fetch the file from.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch(&mut self, url: &str) -> Result<Verification, VerificationError> {
        // Get the file name from the URL
        let file_name = url.split('/').last().unwrap_or("download");

        // Lock the mutex to access the shared resource
        let download_path = DOWNLOAD_PATH_FOLDER.lock().await;
        let file_path = download_path.join(file_name);

        // Check if the file already exists
        if Self::file_exists(file_path.to_str().unwrap()).await {
            return Ok(Verification::Ok);
        }

        // Send a GET request to the URL
        let reqwest_result = self.request().get(url).build();

        // Check if the request was successful
        let reqwest = match reqwest_result {
            Ok(req) => self.request().execute(req).await?,
            Err(e) => return Err(VerificationError::Setup(e.to_string())),
        };

        // Create a new file and write the downloaded contents to it
        let mut file = File::create(&file_path).await?;
        let mut content = Cursor::new(
            reqwest
                .bytes()
                .await
                .map_err(|e| VerificationError::Setup(e.to_string()))?,
        );
        tokio::io::copy(&mut content, &mut file).await?;

        // Verify the downloaded file
        let verification = Self::verify_download(file_path.clone()).await;

        // Check the verification result
        match verification {
            Verification::Ok => {
                // If the verification was successful, extract the ZIP file
                let zip_path = file_path.to_str().unwrap();
                Self::extract_zip(zip_path).await?;
                Ok(Verification::Ok)
            }
            Verification::Failed => {
                // If the verification failed, print an error message and return `VerificationError::Verification`
                println!("Verification failed");
                Err(VerificationError::Verification(Verification::Failed))
            }
            Verification::NotVerified => {
                // If the verification was not verified, print an error message and return `VerificationError::Verification`
                println!("Verification not verified");
                Err(VerificationError::Verification(Verification::NotVerified))
            }
        }
    }
    
    /// Fetches event data for the given date.
    ///
    /// This function uses the `fetch` function to download the event data for the given date.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to fetch the event data for.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch_events(&mut self, date: &str) -> Result<Verification, VerificationError> {
        // Create a new URL for the event data
        let url = format!("{}{}.export.CSV.zip", *BASE_URL, date);
        
        // Fetch the event data
        self.fetch(url.as_str()).await
    }
    
    /// Fetches mention data for the given date.
    ///
    /// This function uses the `fetch` function to download the mention data for the given date.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to fetch the mention data for.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch_mentions(&mut self, date: &str) -> Result<Verification, VerificationError> {
        // Create a new URL for the mention data
        let url = format!("{}{}.mentions.CSV.zip", *BASE_URL, date);
        
        // Fetch the mention data
        self.fetch(url.as_str()).await
    }
    
    /// Fetches GKG data for the given date.
    ///
    /// This function uses the `fetch` function to download the GKG data for the given date.
    ///
    /// # Arguments
    ///
    /// * `date` - The date to fetch the GKG data for.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch_gkg(&mut self, date: &str) -> Result<Verification, VerificationError> {
        // Create a new URL for the GKG data
        let url = format!("{}{}.gkg.csv.zip", *BASE_URL, date);
        
        // Fetch the GKG data
        self.fetch(url.as_str()).await
    }
    
    /// Fetches data of the specified type for the given date.
    ///
    /// This function uses the `fetch_events`, `fetch_mentions`, and `fetch_gkg` functions to download the data of the specified type for the given date.
    ///
    /// # Arguments
    ///
    /// * `fetch_type` - The type of data to fetch.
    /// * `date` - The date to fetch the data for.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch_type(
        &mut self,
        fetch_type: FetchType,
        date: &str,
    ) -> Result<Verification, VerificationError> {
        // Check the fetch type
        match fetch_type {
            FetchType::EVENTS => self.fetch_events(date).await,
            FetchType::MENTIONS => self.fetch_mentions(date).await,
            FetchType::GKG => self.fetch_gkg(date).await,
        }
    }
    
    /// Fetches the latest data of the specified type.
    ///
    /// This function uses the `last_update_string` function to get the current time and then uses the `fetch_type` function to download the data of the specified type for the current time.
    ///
    /// # Arguments
    ///
    /// * `fetch_type` - The type of data to fetch.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Verification` or a `VerificationError`.
    pub async fn fetch_latest(
        &mut self,
        fetch_type: FetchType,
    ) -> Result<Verification, VerificationError> {
        // Get the current time
        let date = Self::last_update_string();
        
        // Fetch the latest data
        self.fetch_type(fetch_type, &date).await
    }
    // endregion
}
// endregionn
