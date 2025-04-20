use crate::components::base_components::{
    location::{country::CountryZone, location_type::LocationType},
    raw_types::location::{RawLatitude, RawLongitude},
};

pub struct Geography {
    pub geo_type: LocationType,
    pub geo_name: LocationName,
    pub country: CountryZone,
    pub admin_1: Administration1,
    pub admin_2: Administration2,
    pub coord: Coordinates,
    pub feature: GeoFeature,
}

pub struct EventGeography {
    pub actor_1: Geography,
    pub actor_2: Geography,
    pub action: Geography,
}

pub struct LocationName(String);
pub struct Coordinates(RawLatitude, RawLongitude);
