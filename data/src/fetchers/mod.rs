// GDELT fetchers module
pub mod gdelt;

// Old fetcher API (non-WASM only, requires file system)
#[cfg(not(target_arch = "wasm32"))]
use models::types::DatabaseTable;
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
use url::Url;

#[cfg(not(target_arch = "wasm32"))]
pub mod big_query;

#[cfg(not(target_arch = "wasm32"))]
use chrono::DateTime;
#[cfg(not(target_arch = "wasm32"))]
use chrono::TimeZone;

#[cfg(not(target_arch = "wasm32"))]
pub enum DataSource {
    BigQuery,
    Http(HttpDatatypes),
}

#[cfg(not(target_arch = "wasm32"))]
pub enum HttpDatatypes {
    CSV,
    JSON,
}

#[cfg(not(target_arch = "wasm32"))]
pub trait DataFetcher<Datatype: DatabaseTable, RawDataFileFormat>: RawDataFetcher {
    type FetchBatch: Iterator<Item = Datatype>;
    fn url_link(&self) -> anyhow::Result<Url>;
    fn fetch_latest(&self) -> anyhow::Result<Self::FetchBatch>;
    fn last_fetch(&self) -> Option<DateTime<impl TimeZone>>;
    fn fetch_date(&self, date: DateTime<impl TimeZone>) -> anyhow::Result<Self::FetchBatch>;
}

#[cfg(not(target_arch = "wasm32"))]
pub trait RawDataFetcher {
    type RawDataFileFormat;
    fn file_path(&self) -> anyhow::Result<PathBuf>;
    fn fetched_file_paths(&self) -> anyhow::Result<Vec<PathBuf>>;
    fn fetch_latest_raw(&self, datasource: DataSource) -> anyhow::Result<Self::RawDataFileFormat>;
    fn fetch_date_raw(
        &self,
        date: DateTime<impl TimeZone>,
        datasource: DataSource,
    ) -> anyhow::Result<Self::RawDataFileFormat>;
    fn source(&self) -> DataSource;
}
