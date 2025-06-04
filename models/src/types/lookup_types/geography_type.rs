use anyhow::anyhow;

use crate::types::event_table::event_geography::GeographyTypeCode;

#[derive(Debug, PartialOrd, PartialEq)]
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
#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn test_geography_type_try_from_valid_codes() {
        init_logger();

        let country_code = GeographyTypeCode(1);
        let state_code = GeographyTypeCode(2);
        let city_code = GeographyTypeCode(3);

        info!("Testing valid GeographyType code: {:?}", country_code);
        let country = GeographyType::try_from(country_code);
        assert!(country.is_ok());
        assert_eq!(country.unwrap(), GeographyType::Country);

        info!("Testing valid GeographyType code: {:?}", state_code);
        let state = GeographyType::try_from(state_code);
        assert!(state.is_ok());
        assert_eq!(state.unwrap(), GeographyType::State);

        info!("Testing valid GeographyType code: {:?}", city_code);
        let city = GeographyType::try_from(city_code);
        assert!(city.is_ok());
        assert_eq!(city.unwrap(), GeographyType::City);
    }

    #[test]
    fn test_geography_type_try_from_invalid_codes() {
        init_logger();

        let invalid_code = GeographyTypeCode(99);

        info!("Testing invalid GeographyType code: {:?}", invalid_code);
        let result = GeographyType::try_from(invalid_code);
        assert!(result.is_err());
    }
}