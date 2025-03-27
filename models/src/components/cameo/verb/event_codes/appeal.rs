use crate::components::cameo::verb::base_action::{
    aid::Aid, material_cooperation::MaterialCooperation, political_reform::PoliticalReform,
    r#yield::Yield,
};

pub enum Appeal {
    NotSpecified,
    MaterialCooperation(MaterialCooperation),
    DiplomaticCooperation,
    Aid(Aid),
    PoliticalReform(PoliticalReform),
    Yield(Yield),
    MeetOrNegotiate,
    SettleDispute,
    EngageOrAcceptMediation,
}
