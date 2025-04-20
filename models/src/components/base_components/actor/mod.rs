use super::raw_types::{
    RawActorName, RawCAMEOActorCode, RawCAMEOActorTypeCode, RawCAMEOCountryCode,
    RawCAMEOEthnicCode, RawCAMEOKnownGroupCode, RawCAMEOReligionCode,
};

pub mod actor_type;
pub mod ethnicity;
pub mod known_group;
pub mod religion;

pub struct RawActor<'a> {
    pub code: RawCAMEOActorTypeCode,
    pub name: RawActorName<'a>,
    pub country_code: RawCAMEOCountryCode,
    pub known_group_code: RawCAMEOKnownGroupCode,
    pub ethnic_code: RawCAMEOEthnicCode,
    pub religion_1_code: RawCAMEOReligionCode,
    pub religion_2_code: RawCAMEOReligionCode,
    pub type_1_code: RawCAMEOActorTypeCode,
    pub type_2_code: RawCAMEOActorTypeCode,
    pub type_3_code: RawCAMEOActorTypeCode,
}
