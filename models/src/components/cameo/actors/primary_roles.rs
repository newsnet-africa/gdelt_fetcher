use crate::components::shared::id_components::OrganisationCode;

use super::{ethnic_groups::EthnicGroup, international_organisations::ReligionCode};

pub enum PrimaryRole {
    NotSpecified(Box<SecondaryRole>),
    Agriculture(Box<SecondarySpecialty>),
    Business(Box<SecondarySpecialty>),
    Criminal(Box<SecondarySpecialty>),
    Civilian(Box<SecondarySpecialty>),
    Development(Box<SecondarySpecialty>),
    Education(Box<SecondarySpecialty>),
    Elites(Box<SecondarySpecialty>),
    Environmental(Box<SecondarySpecialty>),
    Health(Box<SecondarySpecialty>),
    HumanRights(Box<SecondarySpecialty>),
    Labour(Box<SecondarySpecialty>),
    Legislature(Box<SecondarySpecialty>),
    Media(Box<SecondarySpecialty>),
    Refugees(Box<SecondarySpecialty>),
}
pub enum PartySpecialty {
    NotSpecified(EthnicityRole),
    Party(EthnicityRole),
    PrimarySpecialty(PrimaryRole),
}

pub enum EthnicityRole {
    NotSpecified(ReligionRole),
    Ethnicity(EthnicGroup, ReligionRole),
}

pub enum ReligionRole {
    NotSpecified(SecondaryRole),
    Religion(ReligionCode, SecondaryRole),
}

pub enum EthincReligionSpecialty {
    NotSpecified,
}

pub enum SecondaryRole {
    NotSpecified(Box<TertiaryRole>),
    Agriculture(Box<SecondarySpecialty>),
    Business(Box<SecondarySpecialty>),
    Criminal(Box<SecondarySpecialty>),
    Civilian(Box<SecondarySpecialty>),
    Development(Box<SecondarySpecialty>),
    Education(Box<SecondarySpecialty>),
    Elites(Box<SecondarySpecialty>),
    Environmental(Box<SecondarySpecialty>),
    Health(Box<SecondarySpecialty>),
    HumanRights(Box<SecondarySpecialty>),
    Labour(Box<SecondarySpecialty>),
    Legislature(Box<SecondarySpecialty>),
    Media(Box<SecondarySpecialty>),
    Refugees(Box<SecondarySpecialty>),
}

pub enum SecondarySpecialty {
    NotSpecified(Box<TertiaryRole>),
    Specialty(Box<SecondaryRole>),
}

pub enum TertiaryRole {
    NotSpecified(Option<OrganisationCode>),
    Moderate(Option<OrganisationCode>),
    Radical(Option<OrganisationCode>),
}
