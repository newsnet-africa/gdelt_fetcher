use subcategories::{
    Aid, Assault, Coercion, Consultation, Cooperation, DiplomaticCooperation, Disapproval, Fight,
    ForcePosture, InternationalInvolvement, Investigation, MassViolence, MaterialCooperation,
    PoliticalReform, Protest, PublicStatement, Rejection, Relations, ReturnRelease, Threat,
    Yieldable,
};
use top_level_actions::Verb;

use crate::components::codes::verb::CAMEOVerbCode;

pub mod top_level_actions {
    use super::subcategories::{
        Aid, Assault, Coercion, Consultation, Cooperation, DiplomaticCooperation, Disapproval,
        Fight, ForcePosture, Investigation, MassViolence, MaterialCooperation, Protest,
        PublicStatement, Rejection, Relations, Threat, Yieldable,
    };

    pub enum Verb {
        MakePublicStatement(PublicStatement),
        Appeal(Cooperation),
        IntentionToCooperate(Cooperation),
        Consult(Consultation),
        EngageInDiplomaticCooperation(DiplomaticCooperation),
        EngageInMaterialCooperation(MaterialCooperation),
        ProvideAid(Aid),
        Yield(Yieldable),
        Investigate(Investigation),
        Demand(Cooperation),
        Disapprove(Disapproval),
        Reject(Rejection),
        Threaten(Threat),
        Protest(Protest),
        ExhibitForcePosture(ForcePosture),
        ReduceRelations(Relations),
        Coerce(Coercion),
        Assault(Assault),
        Fight(Fight),
        UseUnconventionalMassViolence(MassViolence),
    }
}

pub mod subcategories {
    pub enum PublicStatement {
        Unspecified,
        DeclineToComment,
        MakePessamisticComment,
        MakeOptimisticComment,
        ConsiderPolicyOption,
        AcknowledgeOrClaimResponsibility,
        DenyResponsibility,
        EngageInSymbolicAct,
        MakeEmpatheticComment,
        ExpressAccord,
    }

    pub enum Cooperation {
        Unspecified,
        MaterialCooperation(MaterialCooperation),
        DiplomaticCooperation,
        Aid(Aid),
        PoliticalReform(PoliticalReform),
        Yield(Yieldable),
        ToMeetOrNegotiate,
        SettleDispute,
        AcceptMediation,
        Mediate,
        Ceasefire,
    }

    pub enum Consultation {
        Unspecified,
        DiscussByTelephone,
        MakeAVisit,
        HostAVisit,
        MeetAtThirdLocation,
        Mediate,
        EngageInNegotiation,
    }

    pub enum DiplomaticCooperation {
        Unspecified,
        PraiseOrEndorse,
        DefendVerbally,
        RallySupportOnBehalfOf,
        GrantDiplomaticRecognition,
        Apologise,
        Forgive,
        SignFormalAgreement,
    }

    pub enum MaterialCooperation {
        Unspecified,
        Economic,
        Military,
        Judicial,
        ShareIntelligenceOrInformation,
    }

    pub enum Aid {
        Unspecified,
        Economic,
        Military,
        Humanitarian,
        MilitaryProtectionOrPeaceKeeping,
        GrantAsylum,
    }

    pub enum ReturnRelease {
        Unspecified,
        Person,
        Property,
    }

    pub enum Yieldable {
        Unspecified,
        AdministrativeSanctions(AdministrativeSanctions),
        PoliticalDissent,
        PoliticalReform(PoliticalReform),
        ReturnRelease(ReturnRelease),
        EconomicSanctions,
        InternationalInvolvement(InternationalInvolvement),
        DeEscelateMilitaryEngagement(MilitaryEngagement),
    }

    pub enum Investigation {
        Unspecified,
        CrimeCorruption,
        HumanRightsAbuses,
        MilitaryAction,
        WarCrimes,
    }

    pub enum Disapproval {
        Unspecified,
        CriticiseOrDenounce,
        Accuse(Investigation),
        RallyOppositionAgainst,
        ComplainOfficially,
        BringLawsuitAgainst,
        FindGuiltyOrLiable,
    }

    pub enum Rejection {
        Unspecified,
        Cooperation(Cooperation),
        DefyNorms,
        Veto,
    }

    pub enum NonForce {
        Unspecified,
        ReduceOrStopAid,
        SanctionsBoycottEmbargo,
        ReduceOrBreakRelations,
    }

    pub enum Threat {
        Unspecified,
        NonForce(NonForce),
        AdministrativeSanctions(AdministrativeSanctions),
        PoliticalDissentOrProtest,
        HaltNegotiations,
        HaltMediation,
        HaltInternationalInvolvement,
        Repression,
        MilitaryForce(MilitaryForce),
        Ultimatum,
    }

    pub enum MilitaryForce {
        Unspecified,
        Blockade,
        Occupation,
        UnconventionalViolence,
        ConventionalAttack,
        WMD,
    }

    pub enum MilitaryEngagement {
        Unspecified,
        DeclareTruceCeasefire,
        MilitaryBlockade,
        ArmedForces,
        RetreatSurrender,
    }

    pub enum Change {
        Leadership,
        Policy,
        Rights,
        Institution,
    }

    pub enum Protest {
        Unspecified,
        DemonstrateOrRally(Change),
        HungerStrike(Change),
        StrikeBoycott(Change),
        PassageBlock(Change),
        ViolentRiot(Change),
    }

    pub enum ForcePosture {
        Unspecified,
        IncreasePoliceAlertStatus,
        IncreaseMilitaryAlertStatus,
        MobilizeOrIncreasePolicePower,
        MobilizeOrIncreaseArmedForces,
        MobilizeOrIncreaseCyberForces,
    }

    pub enum Relations {
        Unspecified,
        Diplomatic,
        MaterialAid(Aid),
        ImposeEmbargoBoycottSanction,
        Negotiations,
        Mediation,
        ExpelWithdraw(InternationalInvolvement),
    }

    pub enum InternationalInvolvement {
        Unspecified,
        PeaceKeepers,
        InspectorsObservers,
        Aid,
    }

    pub enum SeizeDamageProperty {
        Confiscate,
        Destroy,
    }

    pub enum Coercion {
        Unspecified,
        WithProperty(SeizeDamageProperty),
        AdministrativeSanctions(AdministrativeSanctions),
        ArrestDetainOrCharge,
        ExpelDeport,
        ViolentRepression,
        CyberneticAttack,
    }

    pub enum PhysicalAssault {
        Unspecified,
        Sexual,
        Torture,
        Kill,
    }

    pub enum Bombing {
        Unspecified,
        Suicide,
        Vehicular,
        Roadside,
        Location,
    }

    pub enum Assault {
        Unspecified,
        AbductHijackTakeHostage,
        Physically(PhysicalAssault),
        Bombing(Bombing),
        UseAsHumanShield,
        AttemptToAssasinate,
        Assasinate,
    }

    pub enum ArialWeapons {
        PrecisionGuided,
        RemotelyPiloted,
    }

    pub enum Fight {
        Unspecified,
        ImposeBlockade,
        OccupyTerritory,
        SmallArmsLightWeapons,
        ArtilleryAndTanks,
        Arial(ArialWeapons),
        ViolateCeasefire,
    }

    pub enum WMD {
        Unspecified,
        ChemicalBiologicalRadiological,
        Nuclear,
    }

    pub enum MassViolence {
        Unspecified,
        MassExpulsions,
        MassKillings,
        EthnicCleansing,
        WeaponsOfMassDistruction(WMD),
    }

    pub enum AdministrativeSanctions {
        Unspecified,
        PoliticalFreedoms,
        BanPoliticalPartiesOrPoliticians,
        Curfew,
        StateOfEmergencyOrMartialLaw,
    }

    pub enum PoliticalReform {
        Unspecified,
        Leadership,
        Policy,
        Rights,
        InstitutionRegime,
    }
}

impl From<CAMEOVerbCode> for Verb {
    fn from(value: CAMEOVerbCode) -> Self {
        let str_value = std::str::from_utf8(&value.0).expect("Invalid CAMEO Code format");
        match &str_value[..2] {
            "01" => match str_value.chars().nth(2) {
                Some('0') => Verb::MakePublicStatement(PublicStatement::Unspecified),
                Some('1') => Verb::MakePublicStatement(PublicStatement::DeclineToComment),
                Some('2') => Verb::MakePublicStatement(PublicStatement::MakePessamisticComment),
                Some('3') => Verb::MakePublicStatement(PublicStatement::MakeOptimisticComment),
                Some('4') => Verb::MakePublicStatement(PublicStatement::ConsiderPolicyOption),
                Some('5') => {
                    Verb::MakePublicStatement(PublicStatement::AcknowledgeOrClaimResponsibility)
                }
                Some('6') => Verb::MakePublicStatement(PublicStatement::DenyResponsibility),
                Some('7') => Verb::MakePublicStatement(PublicStatement::EngageInSymbolicAct),
                Some('8') => Verb::MakePublicStatement(PublicStatement::MakeEmpatheticComment),
                Some('9') => Verb::MakePublicStatement(PublicStatement::ExpressAccord),
            },
            "02" => match str_value.chars().nth(2) {
                Some('0') => Verb::Appeal(Cooperation::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    None | Some(_) => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::Unspecified,
                    )),
                    Some('1') => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::Economic,
                    )),
                    Some('2') => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::Military,
                    )),
                    Some('3') => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::Judicial,
                    )),
                    Some('4') => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::ShareIntelligenceOrInformation,
                    )),
                },
                Some('2') => Verb::Appeal(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    None | Some(_) => Verb::Appeal(Cooperation::Aid(Aid::Unspecified)),
                    Some('1') => Verb::Appeal(Cooperation::Aid(Aid::Economic)),
                    Some('2') => Verb::Appeal(Cooperation::Aid(Aid::Military)),
                    Some('3') => Verb::Appeal(Cooperation::Aid(Aid::Humanitarian)),
                    Some('4') => {
                        Verb::Appeal(Cooperation::Aid(Aid::MilitaryProtectionOrPeaceKeeping))
                    }
                },
                Some('4') => match str_value.chars().nth(3) {
                    None | Some(_) => {
                        Verb::Appeal(Cooperation::PoliticalReform(PoliticalReform::Unspecified))
                    }
                    Some('1') => {
                        Verb::Appeal(Cooperation::PoliticalReform(PoliticalReform::Leadership))
                    }
                    Some('2') => {
                        Verb::Appeal(Cooperation::PoliticalReform(PoliticalReform::Policy))
                    }
                    Some('3') => {
                        Verb::Appeal(Cooperation::PoliticalReform(PoliticalReform::Rights))
                    }
                    Some('4') => Verb::Appeal(Cooperation::PoliticalReform(
                        PoliticalReform::InstitutionRegime,
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
                    None | Some(_) => Verb::Appeal(Cooperation::Yield(Yieldable::Unspecified)),
                    Some('1') => {
                        Verb::Appeal(Cooperation::Yield(Yieldable::AdministrativeSanctions(
                            subcategories::AdministrativeSanctions::Unspecified,
                        )))
                    }
                    Some('2') => Verb::Appeal(Cooperation::Yield(Yieldable::PoliticalDissent)),
                    Some('3') => Verb::Appeal(Cooperation::Yield(Yieldable::ReturnRelease(
                        ReturnRelease::Unspecified,
                    ))),
                    Some('4') => Verb::Appeal(Cooperation::Yield(Yieldable::EconomicSanctions)),
                    Some('5') => Verb::Appeal(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    )),
                    Some('6') => {
                        Verb::Appeal(Cooperation::Yield(Yieldable::DeEscelateMilitaryEngagement(
                            subcategories::MilitaryEngagement::Unspecified,
                        )))
                    }
                },
                Some('6') => Verb::Appeal(Cooperation::ToMeetOrNegotiate),
                Some('7') => Verb::Appeal(Cooperation::SettleDispute),
                Some('8') => Verb::Appeal(Cooperation::AcceptMediation),
            },
            "03" => match str_value.chars().nth(2) {
                Some('0') => Verb::IntentionToCooperate(Cooperation::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    None | Some(_) => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::Unspecified,
                    )),
                    Some('1') => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::Economic,
                    )),
                    Some('2') => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::Military,
                    )),
                    Some('3') => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::Judicial,
                    )),
                    Some('4') => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::ShareIntelligenceOrInformation,
                    )),
                },
                Some('2') => Verb::IntentionToCooperate(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    None | Some(_) => {
                        Verb::IntentionToCooperate(Cooperation::Aid(Aid::Unspecified))
                    }
                    Some('1') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Economic)),
                    Some('2') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Military)),
                    Some('3') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Humanitarian)),
                    Some('4') => Verb::IntentionToCooperate(Cooperation::Aid(
                        Aid::MilitaryProtectionOrPeaceKeeping,
                    )),
                },
                Some('4') => match str_value.chars().nth(3) {
                    None | Some(_) => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::Unspecified,
                    )),
                    Some('1') => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::Leadership,
                    )),
                    Some('2') => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::Policy,
                    )),
                    Some('3') => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::Rights,
                    )),
                    Some('4') => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::InstitutionRegime,
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
                    None | Some(_) => {
                        Verb::IntentionToCooperate(Cooperation::Yield(Yieldable::Unspecified))
                    }
                    Some('1') => Verb::IntentionToCooperate(Cooperation::Yield(
                        Yieldable::AdministrativeSanctions(
                            subcategories::AdministrativeSanctions::Unspecified,
                        ),
                    )),
                    Some('2') => {
                        Verb::IntentionToCooperate(Cooperation::Yield(Yieldable::PoliticalDissent))
                    }
                    Some('3') => Verb::IntentionToCooperate(Cooperation::Yield(
                        Yieldable::ReturnRelease(ReturnRelease::Unspecified),
                    )),
                    Some('4') => {
                        Verb::IntentionToCooperate(Cooperation::Yield(Yieldable::EconomicSanctions))
                    }
                    Some('5') => Verb::IntentionToCooperate(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    )),
                    Some('6') => Verb::IntentionToCooperate(Cooperation::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(
                            subcategories::MilitaryEngagement::Unspecified,
                        ),
                    )),
                },
                Some('6') => Verb::IntentionToCooperate(Cooperation::ToMeetOrNegotiate),
                Some('7') => Verb::IntentionToCooperate(Cooperation::SettleDispute),
                Some('8') => Verb::IntentionToCooperate(Cooperation::AcceptMediation),
                Some('9') => Verb::IntentionToCooperate(Cooperation::Mediate),
            },
            "04" => match str_value.chars().nth(2) {
                Some('0') => Verb::Consult(Consultation::Unspecified),
                Some('1') => Verb::Consult(Consultation::DiscussByTelephone),
                Some('2') => Verb::Consult(Consultation::MakeAVisit),
                Some('3') => Verb::Consult(Consultation::HostAVisit),
                Some('4') => Verb::Consult(Consultation::MeetAtThirdLocation),
                Some('5') => Verb::Consult(Consultation::Mediate),
                Some('6') => Verb::Consult(Consultation::EngageInNegotiation),
            },
            "05" => match str_value.chars().nth(2) {
                Some('0') => {
                    Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::Unspecified)
                }
                Some('1') => {
                    Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::PraiseOrEndorse)
                }
                Some('2') => {
                    Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::DefendVerbally)
                }
                Some('3') => Verb::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::RallySupportOnBehalfOf,
                ),
                Some('4') => Verb::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::GrantDiplomaticRecognition,
                ),
                Some('5') => Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::Apologise),
                Some('6') => Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::Forgive),
                Some('7') => {
                    Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::SignFormalAgreement)
                }
            },
            "06" => match str_value.chars().nth(2) {
                Some('0') => Verb::EngageInMaterialCooperation(MaterialCooperation::Unspecified),
                Some('1') => Verb::EngageInMaterialCooperation(MaterialCooperation::Economic),
                Some('2') => Verb::EngageInMaterialCooperation(MaterialCooperation::Military),
                Some('3') => Verb::EngageInMaterialCooperation(MaterialCooperation::Judicial),
                Some('4') => Verb::EngageInMaterialCooperation(
                    MaterialCooperation::ShareIntelligenceOrInformation,
                ),
            },
            "07" => match str_value.chars().nth(2) {
                Some('0') => Verb::ProvideAid(Aid::Unspecified),
                Some('1') => Verb::ProvideAid(Aid::Economic),
                Some('2') => Verb::ProvideAid(Aid::Military),
                Some('3') => Verb::ProvideAid(Aid::Humanitarian),
                Some('4') => Verb::ProvideAid(Aid::MilitaryProtectionOrPeaceKeeping),
                Some('5') => Verb::ProvideAid(Aid::GrantAsylum),
            },
            "08" => match str_value.chars().nth(2) {
                Some('0') => Verb::Yield(Yieldable::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('1') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('2') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('3') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('4') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('5') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('6') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('7') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('8') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                    Some('9') => Verb::Yield(Yieldable::AdministrativeSanctions(())),
                },
                Some('2') => Verb::Yield(Yieldable::PoliticalDissent),
                Some('3') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('1') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('2') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('3') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('4') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('5') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('6') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('7') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('8') => Verb::Yield(Yieldable::PoliticalReform(())),
                    Some('9') => Verb::Yield(Yieldable::PoliticalReform(())),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('1') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('2') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('3') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('4') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('5') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('6') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('7') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('8') => Verb::Yield(Yieldable::ReturnRelease(())),
                    Some('9') => Verb::Yield(Yieldable::ReturnRelease(())),
                },
                Some('5') => Verb::Yield(Yieldable::EconomicSanctions),
                Some('6') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('1') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('2') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('3') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('4') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('5') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('6') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('7') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('8') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                    Some('9') => Verb::Yield(Yieldable::InternationalInvolvement(())),
                },
                Some('7') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('1') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('2') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('3') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('4') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('5') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('6') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('7') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('8') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                    Some('9') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(())),
                },
            },
            "09" => match str_value.chars().nth(2) {
                Some('0') => Verb::Investigate(Investigation::Unspecified),
                Some('1') => Verb::Investigate(Investigation::CrimeCorruption),
                Some('2') => Verb::Investigate(Investigation::HumanRightsAbuses),
                Some('3') => Verb::Investigate(Investigation::MilitaryAction),
                Some('4') => Verb::Investigate(Investigation::WarCrimes),
            },
            "10" => match str_value.chars().nth(2) {
                Some('0') => Verb::Demand(Cooperation::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('1') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('2') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('3') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('4') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('5') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('6') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('7') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('8') => Verb::Demand(Cooperation::MaterialCooperation(())),
                    Some('9') => Verb::Demand(Cooperation::MaterialCooperation(())),
                },
                Some('2') => Verb::Demand(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Demand(Cooperation::Aid(())),
                    Some('1') => Verb::Demand(Cooperation::Aid(())),
                    Some('2') => Verb::Demand(Cooperation::Aid(())),
                    Some('3') => Verb::Demand(Cooperation::Aid(())),
                    Some('4') => Verb::Demand(Cooperation::Aid(())),
                    Some('5') => Verb::Demand(Cooperation::Aid(())),
                    Some('6') => Verb::Demand(Cooperation::Aid(())),
                    Some('7') => Verb::Demand(Cooperation::Aid(())),
                    Some('8') => Verb::Demand(Cooperation::Aid(())),
                    Some('9') => Verb::Demand(Cooperation::Aid(())),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('1') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('2') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('3') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('4') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('5') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('6') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('7') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('8') => Verb::Demand(Cooperation::PoliticalReform(())),
                    Some('9') => Verb::Demand(Cooperation::PoliticalReform(())),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Demand(Cooperation::Yield(())),
                    Some('1') => Verb::Demand(Cooperation::Yield(())),
                    Some('2') => Verb::Demand(Cooperation::Yield(())),
                    Some('3') => Verb::Demand(Cooperation::Yield(())),
                    Some('4') => Verb::Demand(Cooperation::Yield(())),
                    Some('5') => Verb::Demand(Cooperation::Yield(())),
                    Some('6') => Verb::Demand(Cooperation::Yield(())),
                    Some('7') => Verb::Demand(Cooperation::Yield(())),
                    Some('8') => Verb::Demand(Cooperation::Yield(())),
                    Some('9') => Verb::Demand(Cooperation::Yield(())),
                },
                Some('7') => Verb::Demand(Cooperation::Ceasefire),
                Some('8') => Verb::Demand(Cooperation::ToMeetOrNegotiate),
            },
            "11" => match str_value.chars().nth(2) {
                Some('0') => Verb::Disapprove(Disapproval::Unspecified),
                Some('1') => Verb::Disapprove(Disapproval::CriticiseOrDenounce),
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('1') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('2') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('3') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('4') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('5') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('6') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('7') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('8') => Verb::Disapprove(Disapproval::Accuse(())),
                    Some('9') => Verb::Disapprove(Disapproval::Accuse(())),
                },
                Some('3') => Verb::Disapprove(Disapproval::RallyOppositionAgainst),
                Some('4') => Verb::Disapprove(Disapproval::ComplainOfficially),
                Some('5') => Verb::Disapprove(Disapproval::BringLawsuitAgainst),
                Some('6') => Verb::Disapprove(Disapproval::FindGuiltyOrLiable),
            },
            "12" => match str_value.chars().nth(2) {
                Some('0') => Verb::Reject(Rejection::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('6') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('7') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Reject(Rejection::Cooperation(())),
                    Some('1') => Verb::Reject(Rejection::Cooperation(())),
                    Some('2') => Verb::Reject(Rejection::Cooperation(())),
                    Some('3') => Verb::Reject(Rejection::Cooperation(())),
                    Some('4') => Verb::Reject(Rejection::Cooperation(())),
                    Some('5') => Verb::Reject(Rejection::Cooperation(())),
                    Some('6') => Verb::Reject(Rejection::Cooperation(())),
                    Some('7') => Verb::Reject(Rejection::Cooperation(())),
                    Some('8') => Verb::Reject(Rejection::Cooperation(())),
                    Some('9') => Verb::Reject(Rejection::Cooperation(())),
                },
                Some('8') => Verb::Reject(Rejection::DefyNorms),
                Some('9') => Verb::Reject(Rejection::Veto),
            },
            "13" => match str_value.chars().nth(2) {
                Some('0') => Verb::Threaten(Threat::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Threaten(Threat::NonForce(())),
                    Some('1') => Verb::Threaten(Threat::NonForce(())),
                    Some('2') => Verb::Threaten(Threat::NonForce(())),
                    Some('3') => Verb::Threaten(Threat::NonForce(())),
                    Some('4') => Verb::Threaten(Threat::NonForce(())),
                    Some('5') => Verb::Threaten(Threat::NonForce(())),
                    Some('6') => Verb::Threaten(Threat::NonForce(())),
                    Some('7') => Verb::Threaten(Threat::NonForce(())),
                    Some('8') => Verb::Threaten(Threat::NonForce(())),
                    Some('9') => Verb::Threaten(Threat::NonForce(())),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('1') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('2') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('3') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('4') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('5') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('6') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('7') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('8') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                    Some('9') => Verb::Threaten(Threat::AdministrativeSanctions(())),
                },
                Some('3') => Verb::Threaten(Threat::PoliticalDissentOrProtest),
                Some('4') => Verb::Threaten(Threat::HaltNegotiations),
                Some('5') => Verb::Threaten(Threat::HaltMediation),
                Some('6') => Verb::Threaten(Threat::HaltInternationalInvolvement),
                Some('7') => Verb::Threaten(Threat::Repression),
                Some('8') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('1') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('2') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('3') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('4') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('5') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('6') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('7') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('8') => Verb::Threaten(Threat::MilitaryForce(())),
                    Some('9') => Verb::Threaten(Threat::MilitaryForce(())),
                },
                Some('9') => Verb::Threaten(Threat::Ultimatum),
            },
            "14" => match str_value.chars().nth(2) {
                Some('0') => Verb::Protest(Protest::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('1') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('2') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('3') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('4') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('5') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('6') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('7') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('8') => Verb::Protest(Protest::DemonstrateOrRally(())),
                    Some('9') => Verb::Protest(Protest::DemonstrateOrRally(())),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Protest(Protest::HungerStrike(())),
                    Some('1') => Verb::Protest(Protest::HungerStrike(())),
                    Some('2') => Verb::Protest(Protest::HungerStrike(())),
                    Some('3') => Verb::Protest(Protest::HungerStrike(())),
                    Some('4') => Verb::Protest(Protest::HungerStrike(())),
                    Some('5') => Verb::Protest(Protest::HungerStrike(())),
                    Some('6') => Verb::Protest(Protest::HungerStrike(())),
                    Some('7') => Verb::Protest(Protest::HungerStrike(())),
                    Some('8') => Verb::Protest(Protest::HungerStrike(())),
                    Some('9') => Verb::Protest(Protest::HungerStrike(())),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('1') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('2') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('3') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('4') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('5') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('6') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('7') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('8') => Verb::Protest(Protest::StrikeBoycott(())),
                    Some('9') => Verb::Protest(Protest::StrikeBoycott(())),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Protest(Protest::PassageBlock(())),
                    Some('1') => Verb::Protest(Protest::PassageBlock(())),
                    Some('2') => Verb::Protest(Protest::PassageBlock(())),
                    Some('3') => Verb::Protest(Protest::PassageBlock(())),
                    Some('4') => Verb::Protest(Protest::PassageBlock(())),
                    Some('5') => Verb::Protest(Protest::PassageBlock(())),
                    Some('6') => Verb::Protest(Protest::PassageBlock(())),
                    Some('7') => Verb::Protest(Protest::PassageBlock(())),
                    Some('8') => Verb::Protest(Protest::PassageBlock(())),
                    Some('9') => Verb::Protest(Protest::PassageBlock(())),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('1') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('2') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('3') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('4') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('5') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('6') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('7') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('8') => Verb::Protest(Protest::ViolentRiot(())),
                    Some('9') => Verb::Protest(Protest::ViolentRiot(())),
                },
            },
            "15" => match str_value.chars().nth(2) {
                Some('0') => Verb::ExhibitForcePosture(ForcePosture::Unspecified),
                Some('1') => Verb::ExhibitForcePosture(ForcePosture::IncreasePoliceAlertStatus),
                Some('2') => Verb::ExhibitForcePosture(ForcePosture::IncreaseMilitaryAlertStatus),
                Some('3') => Verb::ExhibitForcePosture(ForcePosture::MobilizeOrIncreasePolicePower),
                Some('4') => Verb::ExhibitForcePosture(ForcePosture::MobilizeOrIncreaseArmedForces),
            },
            "16" => match str_value.chars().nth(2) {
                Some('0') => Verb::ReduceRelations(Relations::Unspecified),
                Some('1') => Verb::ReduceRelations(Relations::Diplomatic),
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('1') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('2') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('3') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('4') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('5') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('6') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('7') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('8') => Verb::ReduceRelations(Relations::MaterialAid(())),
                    Some('9') => Verb::ReduceRelations(Relations::MaterialAid(())),
                },
                Some('3') => Verb::ReduceRelations(Relations::ImposeEmbargoBoycottSanction),
                Some('4') => Verb::ReduceRelations(Relations::Negotiations),
                Some('5') => Verb::ReduceRelations(Relations::Mediation),
                Some('6') => match str_value.chars().nth(3) {
                    Some('0') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('1') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('2') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('3') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('4') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('5') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('6') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('7') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('8') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                    Some('9') => Verb::ReduceRelations(Relations::ExpelWithdraw(())),
                },
            },
            "17" => match str_value.chars().nth(2) {
                Some('0') => Verb::Coerce(Coercion::Unspecified),
                Some('1') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('1') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('2') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('3') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('4') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('5') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('6') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('7') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('8') => Verb::Coerce(Coercion::WithProperty(())),
                    Some('9') => Verb::Coerce(Coercion::WithProperty(())),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('1') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('2') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('3') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('4') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('5') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('6') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('7') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('8') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                    Some('9') => Verb::Coerce(Coercion::AdministrativeSanctions(())),
                },
                Some('3') => Verb::Coerce(Coercion::ArrestDetainOrCharge),
                Some('4') => Verb::Coerce(Coercion::ExpelDeport),
                Some('5') => Verb::Coerce(Coercion::ViolentRepression),
            },
            "18" => match str_value.chars().nth(2) {
                Some('0') => Verb::Assault(Assault::Unspecified),
                Some('1') => Verb::Assault(Assault::AbductHijackTakeHostage),
                Some('2') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Assault(Assault::Physically(())),
                    Some('1') => Verb::Assault(Assault::Physically(())),
                    Some('2') => Verb::Assault(Assault::Physically(())),
                    Some('3') => Verb::Assault(Assault::Physically(())),
                    Some('4') => Verb::Assault(Assault::Physically(())),
                    Some('5') => Verb::Assault(Assault::Physically(())),
                    Some('6') => Verb::Assault(Assault::Physically(())),
                    Some('7') => Verb::Assault(Assault::Physically(())),
                    Some('8') => Verb::Assault(Assault::Physically(())),
                    Some('9') => Verb::Assault(Assault::Physically(())),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Assault(Assault::Bombing(())),
                    Some('1') => Verb::Assault(Assault::Bombing(())),
                    Some('2') => Verb::Assault(Assault::Bombing(())),
                    Some('3') => Verb::Assault(Assault::Bombing(())),
                    Some('4') => Verb::Assault(Assault::Bombing(())),
                    Some('5') => Verb::Assault(Assault::Bombing(())),
                    Some('6') => Verb::Assault(Assault::Bombing(())),
                    Some('7') => Verb::Assault(Assault::Bombing(())),
                    Some('8') => Verb::Assault(Assault::Bombing(())),
                    Some('9') => Verb::Assault(Assault::Bombing(())),
                },
                Some('4') => Verb::Assault(Assault::UseAsHumanShield),
                Some('5') => Verb::Assault(Assault::AttemptToAssasinate),
                Some('6') => Verb::Assault(Assault::Assasinate),
            },
            "19" => match str_value.chars().nth(2) {
                Some('0') => Verb::Fight(Fight::Unspecified),
                Some('1') => Verb::Fight(Fight::ImposeBlockade),
                Some('2') => Verb::Fight(Fight::OccupyTerritory),
                Some('3') => Verb::Fight(Fight::SmallArmsLightWeapons),
                Some('4') => Verb::Fight(Fight::ArtilleryAndTanks),
                Some('5') => match str_value.chars().nth(3) {
                    Some('0') => Verb::Fight(Fight::Arial(())),
                    Some('1') => Verb::Fight(Fight::Arial(())),
                    Some('2') => Verb::Fight(Fight::Arial(())),
                    Some('3') => Verb::Fight(Fight::Arial(())),
                    Some('4') => Verb::Fight(Fight::Arial(())),
                    Some('5') => Verb::Fight(Fight::Arial(())),
                    Some('6') => Verb::Fight(Fight::Arial(())),
                    Some('7') => Verb::Fight(Fight::Arial(())),
                    Some('8') => Verb::Fight(Fight::Arial(())),
                    Some('9') => Verb::Fight(Fight::Arial(())),
                },
                Some('6') => Verb::Fight(Fight::ViolateCeasefire),
            },
            "20" => match str_value.chars().nth(2) {
                Some('0') => Verb::UseUnconventionalMassViolence(MassViolence::Unspecified),
                Some('1') => Verb::UseUnconventionalMassViolence(MassViolence::MassExpulsions),
                Some('2') => Verb::UseUnconventionalMassViolence(MassViolence::MassKillings),
                Some('3') => Verb::UseUnconventionalMassViolence(MassViolence::EthnicCleansing),
                Some('4') => {
                    Verb::UseUnconventionalMassViolence(MassViolence::WeaponsOfMassDistruction(()))
                }
            },
        }
    }
}
