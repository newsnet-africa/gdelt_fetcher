use crate::components::cameo::verb::base_action::political_reform::PoliticalReform;

pub enum Protest {
    NotSpecified,
    DemonstrateOrRally(PoliticalReform),
    HungerStrike(PoliticalReform),
    StrikeBoycott(PoliticalReform),
    ObstructPassageBlock(PoliticalReform),
    ViolentProtest(PoliticalReform),
}
