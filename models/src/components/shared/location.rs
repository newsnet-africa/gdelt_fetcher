use crate::components::cameo::actors::countries::Country;

pub struct Coordinate {
    pub long: f64,
    pub lat: f64,
}

pub enum LocationType {
    Country = 0,
    City = 4,
    State,
}
pub struct Location {
    pub location_type: LocationType,
    pub name: String,
    pub country: Country,
    pub coord: Coordinate,
}
