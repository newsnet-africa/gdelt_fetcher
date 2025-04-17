pub mod administration;
pub mod geo_feature;
pub mod geo_type;

use administration::{PrimaryAdministration, SecondaryAdministration};

use super::actor::country::CountryZone;

pub struct GeoFullname(String);
pub struct GeoCountry(CountryZone);
pub struct Administration1(PrimaryAdministration);
pub struct Administration2(SecondaryAdministration);
pub struct Coord(f64, f64);
