use super::raw_types::*;
pub mod actor_type;
pub mod ethnicity;
pub mod known_group;
pub mod religion;

pub struct RawActor<'a> {
    pub code: CAMEOActorTypeCode,
    pub name: RawActorName<'a>,
    pub country_code: CAMEOCountryCode,
    pub known_group_code: CAMEOKnownGroupCode,
    pub ethnic_code: CAMEOEthnicCode,
    pub religion_1_code: CAMEOReligionCode,
    pub religion_2_code: CAMEOReligionCode,
    pub type_1_code: CAMEOActorTypeCode,
    pub type_2_code: CAMEOActorTypeCode,
    pub type_3_code: CAMEOActorTypeCode,
}
