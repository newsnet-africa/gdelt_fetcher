use super::investigate::Investigate;

pub enum Disaprove {
    NotSpecified,
    CriticiseOrDenounce,
    Accuse(Investigate),
    RallyOppositionAgainst,
    ComplainOfficially,
    BringLawsuitAgainst,
    FindGuiltyOrLiable,
}
