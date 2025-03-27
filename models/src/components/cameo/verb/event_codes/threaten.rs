use crate::components::cameo::verb::base_action::{
    administrative_sanctions::AdministrativeSanctions, military_force::MilitaryForce,
    non_force::NonForce,
};

use super::DiplomaticAction;

pub enum Threaten {
    NotSpecified,
    NonForce(NonForce),
    AdministrativeSanctions(AdministrativeSanctions),
    PoliticalDissent,
    DiplomaticAction(DiplomaticAction),
    Repression,
    Force(MilitaryForce),
    GiveUltimatum,
}
