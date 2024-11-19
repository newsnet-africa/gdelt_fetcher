use super::json_types::JSONType;
use super::rss_types::RSSType;
use super::ToRequestLink;

pub enum OutputFormat {
    Html,
    CSV,
    RSS(RSSType),
    JSON(JSONType),
}

impl ToRequestLink for OutputFormat {
    fn to_request_link(&self) -> String {
        match self {
            Self::Html => "html".to_string(),
            Self::CSV => "csv".to_string(),
            Self::RSS(rss_type) => format!("rss={}", rss_type.to_request_link()),
            Self::JSON(json_type) => format!("json={}", json_type.to_request_link()),
        }
    }
}
