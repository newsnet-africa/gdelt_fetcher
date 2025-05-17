pub mod administration;
pub mod country;
pub mod location_type;

use super::raw_types::location::RawV1Location;

pub type RawGeography<'a> = RawV1Location<'a>;

pub struct RawEventGeography<'a> {
    actor_1: RawGeography<'a>,
    actor_2: RawGeography<'a>,
    action: RawGeography<'a>,
}
