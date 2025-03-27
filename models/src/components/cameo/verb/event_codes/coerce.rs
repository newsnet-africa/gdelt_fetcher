use crate::components::cameo::verb::base_action::administrative_sanctions::AdministrativeSanctions;

pub enum Coerce {
    NotSpecified,
    SeizeOrDamageProperty,
    ImposeSanctions(AdministrativeSanctions),
    ArrestDetainOrCharge,
    ExpelOrDeportIndividuals,
    ViolentRepression,
    CyberneticAttack,
}

pub enum SeizeOrDamageProperty {
    NotSpecified,
    Confiscate,
    Destroy,
}
