use base_action::{
    aid::Aid, diplomatic_cooperation::DiplomaticCooperation,
    material_cooperation::MaterialCooperation, military_engagement::MilitaryEngagement,
    r#yield::Yield,
};
use event_codes::{
    appeal::Appeal, assault::Assault, coerce::Coerce, consult::Consult, demand::Demand,
    disaprove::Disaprove, exhibit_force_posture::ExhibitForcePosture,
    express_intent_to_cooperate::ExpressIntentToCooperate, fight::Fight, investigate::Investigate,
    make_public_statement::MakePublicStatement, protest::Protest,
    reduce_relations::ReduceRelations, reject::Reject, threaten::Threaten,
    use_unconventional_mass_violence::UnconventionalMassViolence,
};

pub mod base_action;
pub mod event_codes;

pub enum EventCode {
    MakePublicStatement(MakePublicStatement),
    Appeal(Appeal),
    ExpressIntentToCooperate(ExpressIntentToCooperate),
    Consult(Consult),
    EngageInDiplomaticCooperation(DiplomaticCooperation),
    EngageInMilitaryCooperation(MaterialCooperation),
    ProvideAid(Aid),
    Yield(Yield),
    Investigate(Investigate),
    Demand(Demand),
    Disapprove(Disaprove),
    Reject(Reject),
    Threaten(Threaten),
    Protest(Protest),
    ExhibitForcePosture(ExhibitForcePosture),
    ReduceRelations(ReduceRelations),
    Coerce(Coerce),
    Assault(Assault),
    Fight(Fight),
    UseUnconventionalMassViolence(UnconventionalMassViolence),
}
