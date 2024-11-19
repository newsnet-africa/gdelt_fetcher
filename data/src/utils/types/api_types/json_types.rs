use super::ToRequestLink;

pub enum JSONType {
    Callback(String),
    Feed,
}

impl ToRequestLink for JSONType {
    fn to_request_link(&self) -> String {
        match self {
            Self::Callback(callback) => format!("callback={}", callback),
            Self::Feed => "feed".to_string(),
        }
    }
}
