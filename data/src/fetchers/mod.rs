use models::types::DatabaseTable;
use std::path::PathBuf;
use url::Url;

pub mod big_query;
pub mod gdelt;

use chrono::DateTime;
use chrono::TimeZone;

pub enum DataSource {
    BigQuery,
    Http(HttpDatatypes),
}

pub enum HttpDatatypes {
    CSV,
    JSON,
}

pub trait DataFetcher<Datatype: DatabaseTable, RawDataFileFormat>: RawDataFetcher {
    type FetchBatch: Iterator<Item = Datatype>;
    fn url_link(&self) -> anyhow::Result<Url>;
    fn fetch_latest(&self) -> anyhow::Result<Self::FetchBatch>;
    fn last_fetch(&self) -> Option<DateTime<impl TimeZone>>;
    fn fetch_date(&self, date: DateTime<impl TimeZone>) -> anyhow::Result<Self::FetchBatch>;
}

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
