use super::ToRequestLink;

pub enum SortType {
    Date(bool),
    Tone(bool),
    HybridRel,
}

impl ToRequestLink for SortType {
    fn to_request_link(&self) -> String {
        match self {
            Self::Date(asc) => {
                if *asc {
                    "dateasc".to_string()
                } else {
                    "datedesc".to_string()
                }
            }
            Self::Tone(asc) => {
                if *asc {
                    "toneasc".to_string()
                } else {
                    "tonedesc".to_string()
                }
            }
            Self::HybridRel => "hybridrel".to_string(),
        }
    }
}
