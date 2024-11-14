use super::json_types::JSONType;
use super::rss_types::RSSType;

pub enum OutputFormat {
    Html,
    CSV,
    RSS(RSSType),
    JSON(JSONType),
}
