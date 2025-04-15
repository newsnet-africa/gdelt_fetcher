use crate::components::base_components::location::{
    Administration1, Administration2, Coord, GeoCountry, GeoFullname, geo_feature::GeoFeature,
    geo_type::GeoType,
};

pub struct Geography {
    pub geo_type: GeoType,
    pub geo_name: GeoFullname,
    pub country: GeoCountry,
    pub admin_1: Administration1,
    pub admin_2: Administration2,
    pub coord: Coord,
    pub feature: GeoFeature,
}

pub struct EventGeography {
    pub actor_1: Geography,
    pub actor_2: Geography,
    pub action: Geography,
}
