use super::ToRequestLink;

pub enum RSSType {
    Default,
    Archive,
}

impl ToRequestLink for RSSType {
    fn to_request_link(&self) -> String {
        match self {
            Self::Default => "default".to_string(),
            Self::Archive => "archive".to_string(),
        }
    }
}
