use anyhow::anyhow;

use crate::types::event_table::event_geography::GeographyTypeCode;

pub enum GeographyType {
    Unspecifed,
    Country,
    State,
    City,
}

impl TryFrom<GeographyTypeCode> for GeographyType {
    type Error = anyhow::Error;

    fn try_from(value: GeographyTypeCode) -> anyhow::Result<Self> {
        match value.0 {
            1 => Ok(Self::Country),
            2 | 5 => Ok(Self::State),
            3 | 4 => Ok(Self::City),
            _ => Err(anyhow!("Invalid Geography Code")),
        }
    }
}
