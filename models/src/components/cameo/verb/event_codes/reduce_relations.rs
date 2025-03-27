use crate::components::cameo::verb::base_action::aid::Aid;

use super::DiplomaticAction;

pub enum ReduceRelations {
    NotSpecified,
    DiplomaticRelations,
    Aid(Aid),
    ImposeEmbargoBoycottOrSanctions,
    HaltDiplomaticAction(DiplomaticAction),
    ExpelOrWithdraw(ExpelOrWithdraw),
}

pub enum ExpelOrWithdraw {
    Peacekeepers,
    Inspectors,
    Observers,
    AidAgencies,
}
