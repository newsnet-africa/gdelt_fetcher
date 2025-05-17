use anyhow::{Error, Result};
use chrono::NaiveDateTime;
use log::info;
use regex::Regex;

pub fn extract_date(url: &str) -> Result<NaiveDateTime> {
    let re = Regex::new(r"/(\d{14})\.\w+\.(csv|CSV)\.zip$")?;
    if let Some(captures) = re.captures(url) {
        if let Some(date_str) = captures.get(1) {
            let date = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y%m%d%H%M%S")?;
            return Ok(date);
        }
    }
    Err(Error::msg("Failed to extract date"))
}

pub fn extract_db_type(url: &str) -> Result<String> {
    let re = Regex::new(r"/\d{14}\.(?P<variant>[^.]+)\.(csv|CSV)\.zip$")?;
    if let Some(caps) = re.captures(url) {
        let variant = &caps["variant"];
        info!("Extracted variant: {}", variant);
        return Ok(variant.to_string());
    }
    Err(Error::msg("Failed to extract database type"))
}
pub fn extract_gdelt_version(url: &str) -> Result<String> {
    let re = Regex::new(r"/(?P<version>gdeltv\d+)/")?;
    if let Some(caps) = re.captures(url) {
        let version = &caps["version"];
        info!("Extracted GDELT version: {}", version);
        return Ok(version.to_string());
    }
    Err(Error::msg("Failed to extract GDELT version"))
}

#[cfg(test)]
mod test {
    use log::info;

    use crate::utils::extract_gdelt_version;

    use super::{extract_date, extract_db_type};

    #[test]
    fn extract_gdelt_version_test() {
        let url = "http://data.gdeltproject.org/gdeltv2/20250321184500.gkg.csv.zip";

        let version = extract_gdelt_version(url);
        info!("Extracted GDELT version: {:?}", version);

        assert!(version.is_ok());
        assert_eq!(version.unwrap(), "gdeltv2");
    }

    #[test]
    fn extract_date_test() {
        let url = "http://data.gdeltproject.org/gdeltv2/20250321184500.gkg.csv.zip";

        let date = extract_date(url);
        info!("Extracted date: {:?}", date);

        assert!(date.is_ok());
    }

    #[test]
    fn extract_db_type_test() {
        let url = "http://data.gdeltproject.org/gdeltv2/20250321184500.gkg.CSV.zip";

        let db_type = extract_db_type(url);
        info!("Extracted database type: {:?}", db_type);

        assert!(db_type.is_ok());
        assert_eq!(db_type.unwrap(), "gkg");
    }

    #[test]
    fn extract_date_invalid_url_test() {
        let url = "http://data.gdeltproject.org/gdeltv2/invalid_url.csv.zip";

        let date = extract_date(url);
        info!("Extracted date from invalid URL: {:?}", date);

        assert!(date.is_err());
    }

    #[test]
    fn extract_db_type_invalid_url_test() {
        let url = "http://data.gdeltproject.org/gdeltv2/invalid_url.csv.zip";

        let db_type = extract_db_type(url);
        info!("Extracted database type from invalid URL: {:?}", db_type);

        assert!(db_type.is_err());
    }
}
