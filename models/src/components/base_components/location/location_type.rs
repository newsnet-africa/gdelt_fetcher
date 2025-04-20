use crate::components::base_components::raw_types::location::RawLocationType;

pub enum LocationType {
    Unspecifed,
    Country,
    State,
    City,
}

impl From<RawLocationType> for LocationType {
    fn from(value: RawLocationType) -> Self {
        match value.0 {
            1 => Self::Country,
            2 | 5 => Self::State,
            3 | 4 => Self::City,
            _ => Self::Unspecifed,
        }
    }
}
