use crate::components::{
    Coordinates, GeoFeature, LocationName,
    base_components::{
        location::{
            RawEventGeography, RawGeography, country::CountryZone, location_type::LocationType,
        },
        raw_types::location::*,
    },
};

pub struct Geography {
    pub geo_type: LocationType,
    pub geo_name: LocationName,
    pub country: CountryZone,
    pub coord: Coordinates,
    pub feature: GeoFeature,
}

impl<'a> From<RawGeography<'a>> for Geography {
    fn from(value: RawGeography<'a>) -> Self {
        Self {
            geo_type: LocationType::from(value.0),
            geo_name: LocationName::from(value.1),
            country: CountryZone::from(value.2),
            coord: Coordinates(value.4, value.5),
            feature: GeoFeature::from(value.6),
        }
    }
}

pub struct EventGeography {
    pub actor_1: Geography,
    pub actor_2: Geography,
    pub action: Geography,
}

impl<'a> From<RawEventGeography<'a>> for EventGeography {
    fn from(value: RawEventGeography<'a>) -> Self {}
}
