pub mod appeal;
pub mod assault;
pub mod coerce;
pub mod consult;
pub mod demand;
pub mod disaprove;
pub mod exhibit_force_posture;
pub mod express_intent_to_cooperate;
pub mod fight;
pub mod investigate;
pub mod make_public_statement;
pub mod protest;
pub mod reduce_relations;
pub mod reject;
pub mod threaten;
pub mod use_unconventional_mass_violence;

pub enum DiplomaticAction {
    MeetOrNegotiate,
    SettleDispute,
    AcceptMediation,
    Mediate,
}
