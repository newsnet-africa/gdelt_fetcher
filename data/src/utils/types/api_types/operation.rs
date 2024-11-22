use std::fmt::Display;

use super::ToRequestLink;

pub enum Operation<T>
where
    T: PartialOrd + Display,
{
    MoreThan(T),
    LessThan(T),
    Equal(T),
}

impl<T: PartialOrd + Display> ToRequestLink for Operation<T> {
    fn to_request_link(&self) -> String {
        match self {
            Self::MoreThan(value) => format!(">{}", value),
            Self::LessThan(value) => format!("<{}", value),
            Self::Equal(value) => format!("={}", value),
        }
    }
}
