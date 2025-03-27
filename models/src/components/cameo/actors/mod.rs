use countries::Country;
use international_organisations::InternationalOrganisationCode;
use primary_roles::PrimaryRole;

pub mod countries;
pub mod ethnic_groups;
pub mod international_organisations;
pub mod primary_roles;
pub mod religious_codes;

pub enum ActorCode {
    NotSpecified,
    Country(Country, PrimaryRole),
    International(InternationalOrganisationCode),
}
