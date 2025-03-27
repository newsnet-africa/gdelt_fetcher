use crate::components::cameo::verb::base_action::{
    aid::Aid, material_cooperation::MaterialCooperation, political_reform::PoliticalReform,
    r#yield::Yield,
};

use super::DiplomaticAction;

pub enum Reject {
    NotSpecified,
    MaterialCooperation(MaterialCooperation),
    Aid(Aid),
    PoliticalReform(PoliticalReform),
    Yield(Yield),
    DiplomaticAction(DiplomaticAction),
    DefyNorms,
    Veto,
}
