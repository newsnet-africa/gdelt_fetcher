use super::{
    administrative_sanctions::AdministrativeSanctions,
    international_involvement::InternationalInvolvement, political_reform::PoliticalReform,
};

pub enum Yield {
    NotSpecified,
    AdministrativeSanctions(AdministrativeSanctions),
    PopularDissent,
    PoliticalReform(PoliticalReform),
    Release(PersonProperty),
    EconomicSanctionsBoycottsOrEmbargo,
    AllowInternationalInvolvement(InternationalInvolvement),
    DeescalateMilitaryEngagement,
}

pub enum PersonProperty {
    Person,
    Property,
}
