use super::ToRequestLink;

pub enum Operation {
    MoreThan(f32),
    LessThan(f32),
    Equal(f32),
}

impl ToRequestLink for Operation {
    fn to_request_link(&self) -> String {
        match self {
            Self::MoreThan(value) => format!(">{}", value),
            Self::LessThan(value) => format!("<{}", value),
            Self::Equal(value) => format!("={}", value),
        }
    }
}
