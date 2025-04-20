use subcategories::{
    AdministrativeSanctions, Aid, ArialWeapons, Assault, Bombing, Change, Coercion, Consultation,
    Cooperation, DiplomaticCooperation, Disapproval, Fight, ForcePosture, InternationalInvolvement,
    Investigation, MassViolence, MaterialCooperation, MilitaryEngagement, MilitaryForce, NonForce,
    PhysicalAssault, PoliticalReform, Protest, PublicStatement, Rejection, Relations,
    ReturnRelease, SeizeDamageProperty, Threat, WMD, Yieldable,
};
use top_level_actions::Verb;

use crate::components::base_components::raw_types::RawCAMEOEventCode;

pub mod top_level_actions {
    use super::subcategories::{
        Aid, Assault, Coercion, Consultation, Cooperation, DiplomaticCooperation, Disapproval,
        Fight, ForcePosture, Investigation, MassViolence, MaterialCooperation, Protest,
        PublicStatement, Rejection, Relations, Threat, Yieldable,
    };

    pub enum Verb {
        Unspecified,
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
        Withdraw,
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
        Aid(Aid),
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
        EspionageTreason,
        Aggression,
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
        Unspecified,
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
        Aid(Aid),
    }

    pub enum SeizeDamageProperty {
        Unspecified,
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
        Unspecified,
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

impl From<RawCAMEOEventCode> for Verb {
    fn from(value: RawCAMEOEventCode) -> Self {
        let str_value = std::str::from_utf8(&value.0).expect("Invalid CAMEO Code format");
        match &str_value[..2] {
            "01" => match str_value.chars().nth(2) {
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
                None | Some(_) => Verb::MakePublicStatement(PublicStatement::Unspecified),
            },
            "02" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
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
                    None | Some(_) => Verb::Appeal(Cooperation::MaterialCooperation(
                        MaterialCooperation::Unspecified,
                    )),
                },
                Some('2') => Verb::Appeal(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Appeal(Cooperation::Aid(Aid::Economic)),
                    Some('2') => Verb::Appeal(Cooperation::Aid(Aid::Military)),
                    Some('3') => Verb::Appeal(Cooperation::Aid(Aid::Humanitarian)),
                    Some('4') => {
                        Verb::Appeal(Cooperation::Aid(Aid::MilitaryProtectionOrPeaceKeeping))
                    }
                    None | Some(_) => Verb::Appeal(Cooperation::Aid(Aid::Unspecified)),
                },
                Some('4') => match str_value.chars().nth(3) {
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
                    None | Some(_) => {
                        Verb::Appeal(Cooperation::PoliticalReform(PoliticalReform::Unspecified))
                    }
                },
                Some('5') => match str_value.chars().nth(3) {
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
                    None | Some(_) => Verb::Appeal(Cooperation::Yield(Yieldable::Unspecified)),
                },
                Some('6') => Verb::Appeal(Cooperation::ToMeetOrNegotiate),
                Some('7') => Verb::Appeal(Cooperation::SettleDispute),
                Some('8') => Verb::Appeal(Cooperation::AcceptMediation),
                None | Some(_) => Verb::Appeal(Cooperation::Unspecified),
            },
            "03" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
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
                    None | Some(_) => Verb::IntentionToCooperate(Cooperation::MaterialCooperation(
                        MaterialCooperation::Unspecified,
                    )),
                },
                Some('2') => Verb::IntentionToCooperate(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Economic)),
                    Some('2') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Military)),
                    Some('3') => Verb::IntentionToCooperate(Cooperation::Aid(Aid::Humanitarian)),
                    Some('4') => Verb::IntentionToCooperate(Cooperation::Aid(
                        Aid::MilitaryProtectionOrPeaceKeeping,
                    )),
                    None | Some(_) => {
                        Verb::IntentionToCooperate(Cooperation::Aid(Aid::Unspecified))
                    }
                },
                Some('4') => match str_value.chars().nth(3) {
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
                    None | Some(_) => Verb::IntentionToCooperate(Cooperation::PoliticalReform(
                        PoliticalReform::Unspecified,
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
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
                    None | Some(_) => {
                        Verb::IntentionToCooperate(Cooperation::Yield(Yieldable::Unspecified))
                    }
                },
                Some('6') => Verb::IntentionToCooperate(Cooperation::ToMeetOrNegotiate),
                Some('7') => Verb::IntentionToCooperate(Cooperation::SettleDispute),
                Some('8') => Verb::IntentionToCooperate(Cooperation::AcceptMediation),
                Some('9') => Verb::IntentionToCooperate(Cooperation::Mediate),
                None | Some(_) => Verb::IntentionToCooperate(Cooperation::Unspecified),
            },
            "04" => match str_value.chars().nth(2) {
                Some('1') => Verb::Consult(Consultation::DiscussByTelephone),
                Some('2') => Verb::Consult(Consultation::MakeAVisit),
                Some('3') => Verb::Consult(Consultation::HostAVisit),
                Some('4') => Verb::Consult(Consultation::MeetAtThirdLocation),
                Some('5') => Verb::Consult(Consultation::Mediate),
                Some('6') => Verb::Consult(Consultation::EngageInNegotiation),
                None | Some(_) => Verb::Consult(Consultation::Unspecified),
            },
            "05" => match str_value.chars().nth(2) {
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
                None | Some(_) => {
                    Verb::EngageInDiplomaticCooperation(DiplomaticCooperation::Unspecified)
                }
            },
            "06" => match str_value.chars().nth(2) {
                Some('1') => Verb::EngageInMaterialCooperation(MaterialCooperation::Economic),
                Some('2') => Verb::EngageInMaterialCooperation(MaterialCooperation::Military),
                Some('3') => Verb::EngageInMaterialCooperation(MaterialCooperation::Judicial),
                Some('4') => Verb::EngageInMaterialCooperation(
                    MaterialCooperation::ShareIntelligenceOrInformation,
                ),
                None | Some(_) => {
                    Verb::EngageInMaterialCooperation(MaterialCooperation::Unspecified)
                }
            },
            "07" => match str_value.chars().nth(2) {
                Some('1') => Verb::ProvideAid(Aid::Economic),
                Some('2') => Verb::ProvideAid(Aid::Military),
                Some('3') => Verb::ProvideAid(Aid::Humanitarian),
                Some('4') => Verb::ProvideAid(Aid::MilitaryProtectionOrPeaceKeeping),
                Some('5') => Verb::ProvideAid(Aid::GrantAsylum),
                None | Some(_) => Verb::ProvideAid(Aid::Unspecified),
            },
            "08" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Yield(Yieldable::AdministrativeSanctions(
                        AdministrativeSanctions::PoliticalFreedoms,
                    )),
                    Some('2') => Verb::Yield(Yieldable::AdministrativeSanctions(
                        AdministrativeSanctions::Curfew,
                    )),
                    Some('3') => Verb::Yield(Yieldable::AdministrativeSanctions(
                        AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                    )),
                    None | Some(_) => Verb::Yield(Yieldable::AdministrativeSanctions(
                        AdministrativeSanctions::Unspecified,
                    )),
                },
                Some('2') => Verb::Yield(Yieldable::PoliticalDissent),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => {
                        Verb::Yield(Yieldable::PoliticalReform(PoliticalReform::Leadership))
                    }
                    Some('2') => Verb::Yield(Yieldable::PoliticalReform(PoliticalReform::Policy)),
                    Some('3') => Verb::Yield(Yieldable::PoliticalReform(PoliticalReform::Rights)),
                    Some('4') => Verb::Yield(Yieldable::PoliticalReform(
                        PoliticalReform::InstitutionRegime,
                    )),
                    None | Some(_) => {
                        Verb::Yield(Yieldable::PoliticalReform(PoliticalReform::Unspecified))
                    }
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Yield(Yieldable::ReturnRelease(ReturnRelease::Person)),
                    Some('2') => Verb::Yield(Yieldable::ReturnRelease(ReturnRelease::Property)),
                    None | Some(_) => {
                        Verb::Yield(Yieldable::ReturnRelease(ReturnRelease::Unspecified))
                    }
                },
                Some('5') => Verb::Yield(Yieldable::EconomicSanctions),
                Some('6') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Yield(Yieldable::InternationalInvolvement(
                        InternationalInvolvement::PeaceKeepers,
                    )),
                    Some('2') => Verb::Yield(Yieldable::InternationalInvolvement(
                        InternationalInvolvement::InspectorsObservers,
                    )),
                    Some('3') => Verb::Yield(Yieldable::InternationalInvolvement(
                        InternationalInvolvement::Aid(Aid::Unspecified),
                    )),
                    None | Some(_) => Verb::Yield(Yieldable::InternationalInvolvement(
                        InternationalInvolvement::Unspecified,
                    )),
                },
                Some('7') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(
                        MilitaryEngagement::DeclareTruceCeasefire,
                    )),
                    Some('2') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(
                        MilitaryEngagement::MilitaryBlockade,
                    )),
                    Some('3') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(
                        MilitaryEngagement::ArmedForces,
                    )),
                    Some('4') => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(
                        MilitaryEngagement::RetreatSurrender,
                    )),
                    None | Some(_) => Verb::Yield(Yieldable::DeEscelateMilitaryEngagement(
                        MilitaryEngagement::Unspecified,
                    )),
                },
                None | Some(_) => Verb::Yield(Yieldable::Unspecified),
            },
            "09" => match str_value.chars().nth(2) {
                Some('1') => Verb::Investigate(Investigation::CrimeCorruption),
                Some('2') => Verb::Investigate(Investigation::HumanRightsAbuses),
                Some('3') => Verb::Investigate(Investigation::MilitaryAction),
                Some('4') => Verb::Investigate(Investigation::WarCrimes),
                None | Some(_) => Verb::Investigate(Investigation::Unspecified),
            },
            "10" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Demand(Cooperation::MaterialCooperation(
                        MaterialCooperation::Economic,
                    )),
                    Some('2') => Verb::Demand(Cooperation::MaterialCooperation(
                        MaterialCooperation::Military,
                    )),
                    Some('3') => Verb::Demand(Cooperation::MaterialCooperation(
                        MaterialCooperation::Judicial,
                    )),
                    Some('4') => Verb::Demand(Cooperation::MaterialCooperation(
                        MaterialCooperation::ShareIntelligenceOrInformation,
                    )),
                    None | Some(_) => Verb::Demand(Cooperation::MaterialCooperation(
                        MaterialCooperation::Unspecified,
                    )),
                },
                Some('2') => Verb::Demand(Cooperation::DiplomaticCooperation),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Demand(Cooperation::Aid(Aid::Economic)),
                    Some('2') => Verb::Demand(Cooperation::Aid(Aid::Military)),
                    Some('3') => Verb::Demand(Cooperation::Aid(Aid::Humanitarian)),
                    Some('4') => {
                        Verb::Demand(Cooperation::Aid(Aid::MilitaryProtectionOrPeaceKeeping))
                    }
                    None | Some(_) => Verb::Demand(Cooperation::Aid(Aid::Unspecified)),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => {
                        Verb::Demand(Cooperation::PoliticalReform(PoliticalReform::Leadership))
                    }
                    Some('2') => {
                        Verb::Demand(Cooperation::PoliticalReform(PoliticalReform::Policy))
                    }
                    Some('3') => {
                        Verb::Demand(Cooperation::PoliticalReform(PoliticalReform::Rights))
                    }
                    Some('4') => Verb::Demand(Cooperation::PoliticalReform(
                        PoliticalReform::InstitutionRegime,
                    )),
                    None | Some(_) => {
                        Verb::Demand(Cooperation::PoliticalReform(PoliticalReform::Unspecified))
                    }
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Demand(Cooperation::Yield(
                        Yieldable::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    )),
                    Some('2') => Verb::Demand(Cooperation::Yield(Yieldable::PoliticalDissent)),
                    Some('3') => Verb::Demand(Cooperation::Yield(Yieldable::ReturnRelease(
                        ReturnRelease::Unspecified,
                    ))),
                    Some('4') => Verb::Demand(Cooperation::Yield(Yieldable::EconomicSanctions)),
                    Some('5') => Verb::Demand(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    )),
                    Some('6') => Verb::Demand(Cooperation::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(MilitaryEngagement::Unspecified),
                    )),
                    None | Some(_) => Verb::Demand(Cooperation::Yield(Yieldable::Unspecified)),
                },
                Some('6') => Verb::Demand(Cooperation::Withdraw),
                Some('7') => Verb::Demand(Cooperation::Ceasefire),
                Some('8') => Verb::Demand(Cooperation::ToMeetOrNegotiate),
                None | Some(_) => Verb::Demand(Cooperation::Unspecified),
            },
            "11" => match str_value.chars().nth(2) {
                Some('1') => Verb::Disapprove(Disapproval::CriticiseOrDenounce),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => {
                        Verb::Disapprove(Disapproval::Accuse(Investigation::CrimeCorruption))
                    }
                    Some('2') => {
                        Verb::Disapprove(Disapproval::Accuse(Investigation::HumanRightsAbuses))
                    }
                    Some('3') => Verb::Disapprove(Disapproval::Accuse(Investigation::Aggression)),
                    Some('4') => Verb::Disapprove(Disapproval::Accuse(Investigation::WarCrimes)),
                    Some('5') => {
                        Verb::Disapprove(Disapproval::Accuse(Investigation::EspionageTreason))
                    }
                    None | Some(_) => {
                        Verb::Disapprove(Disapproval::Accuse(Investigation::Unspecified))
                    }
                },
                Some('3') => Verb::Disapprove(Disapproval::RallyOppositionAgainst),
                Some('4') => Verb::Disapprove(Disapproval::ComplainOfficially),
                Some('5') => Verb::Disapprove(Disapproval::BringLawsuitAgainst),
                Some('6') => Verb::Disapprove(Disapproval::FindGuiltyOrLiable),
                None | Some(_) => Verb::Disapprove(Disapproval::Unspecified),
            },
            "12" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    )),
                    Some('2') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    )),
                    None | Some(_) => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    )),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    )),
                    Some('2') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    )),
                    Some('3') => {
                        Verb::Reject(Rejection::Cooperation(Cooperation::MaterialCooperation(
                            MaterialCooperation::Aid(Aid::Humanitarian),
                        )))
                    }
                    Some('4') => {
                        Verb::Reject(Rejection::Cooperation(Cooperation::MaterialCooperation(
                            MaterialCooperation::Aid(Aid::MilitaryProtectionOrPeaceKeeping),
                        )))
                    }
                    None | Some(_) => Verb::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    )),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Leadership),
                    )),
                    Some('2') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Policy),
                    )),
                    Some('3') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Rights),
                    )),
                    Some('4') => Verb::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::InstitutionRegime),
                    )),
                    None | Some(_) => Verb::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Unspecified),
                    )),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    ))),
                    Some('2') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::PoliticalDissent,
                    ))),
                    Some('3') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::ReturnRelease(ReturnRelease::Unspecified),
                    ))),
                    Some('4') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::EconomicSanctions,
                    ))),
                    Some('5') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    ))),
                    Some('6') => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(MilitaryEngagement::Unspecified),
                    ))),
                    None | Some(_) => Verb::Reject(Rejection::Cooperation(Cooperation::Yield(
                        Yieldable::Unspecified,
                    ))),
                },
                Some('5') => Verb::Reject(Rejection::Cooperation(Cooperation::ToMeetOrNegotiate)),
                Some('6') => Verb::Reject(Rejection::Cooperation(Cooperation::AcceptMediation)),
                Some('7') => Verb::Reject(Rejection::Cooperation(Cooperation::SettleDispute)),
                Some('8') => Verb::Reject(Rejection::DefyNorms),
                Some('9') => Verb::Reject(Rejection::Veto),
                None | Some(_) => Verb::Reject(Rejection::Unspecified),
            },
            "13" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Threaten(Threat::NonForce(NonForce::ReduceOrStopAid)),
                    Some('2') => {
                        Verb::Threaten(Threat::NonForce(NonForce::SanctionsBoycottEmbargo))
                    }
                    Some('3') => Verb::Threaten(Threat::NonForce(NonForce::ReduceOrBreakRelations)),
                    None | Some(_) => {
                        Verb::Threaten(Threat::NonForce(subcategories::NonForce::Unspecified))
                    }
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Threaten(Threat::AdministrativeSanctions(
                        AdministrativeSanctions::PoliticalFreedoms,
                    )),
                    Some('2') => Verb::Threaten(Threat::AdministrativeSanctions(
                        AdministrativeSanctions::BanPoliticalPartiesOrPoliticians,
                    )),
                    Some('3') => Verb::Threaten(Threat::AdministrativeSanctions(
                        AdministrativeSanctions::Curfew,
                    )),
                    Some('4') => Verb::Threaten(Threat::AdministrativeSanctions(
                        AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                    )),
                    None | Some(_) => Verb::Threaten(Threat::AdministrativeSanctions(
                        AdministrativeSanctions::Unspecified,
                    )),
                },
                Some('3') => Verb::Threaten(Threat::PoliticalDissentOrProtest),
                Some('4') => Verb::Threaten(Threat::HaltNegotiations),
                Some('5') => Verb::Threaten(Threat::HaltMediation),
                Some('6') => Verb::Threaten(Threat::HaltInternationalInvolvement),
                Some('7') => Verb::Threaten(Threat::Repression),
                Some('8') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Threaten(Threat::MilitaryForce(MilitaryForce::Blockade)),
                    Some('2') => Verb::Threaten(Threat::MilitaryForce(MilitaryForce::Occupation)),
                    Some('3') => {
                        Verb::Threaten(Threat::MilitaryForce(MilitaryForce::UnconventionalViolence))
                    }
                    Some('4') => {
                        Verb::Threaten(Threat::MilitaryForce(MilitaryForce::ConventionalAttack))
                    }
                    Some('5') => Verb::Threaten(Threat::MilitaryForce(MilitaryForce::WMD)),
                    None | Some(_) => {
                        Verb::Threaten(Threat::MilitaryForce(MilitaryForce::Unspecified))
                    }
                },
                Some('9') => Verb::Threaten(Threat::Ultimatum),
                None | Some(_) => Verb::Threaten(Threat::Unspecified),
            },
            "14" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Protest(Protest::DemonstrateOrRally(Change::Leadership)),
                    Some('2') => Verb::Protest(Protest::DemonstrateOrRally(Change::Policy)),
                    Some('3') => Verb::Protest(Protest::DemonstrateOrRally(Change::Rights)),
                    Some('4') => Verb::Protest(Protest::DemonstrateOrRally(Change::Institution)),
                    None | Some(_) => {
                        Verb::Protest(Protest::DemonstrateOrRally(Change::Unspecified))
                    }
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Protest(Protest::HungerStrike(Change::Leadership)),
                    Some('2') => Verb::Protest(Protest::HungerStrike(Change::Policy)),
                    Some('3') => Verb::Protest(Protest::HungerStrike(Change::Rights)),
                    Some('4') => Verb::Protest(Protest::HungerStrike(Change::Institution)),
                    None | Some(_) => Verb::Protest(Protest::HungerStrike(Change::Unspecified)),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Protest(Protest::StrikeBoycott(Change::Leadership)),
                    Some('2') => Verb::Protest(Protest::StrikeBoycott(Change::Policy)),
                    Some('3') => Verb::Protest(Protest::StrikeBoycott(Change::Rights)),
                    Some('4') => Verb::Protest(Protest::StrikeBoycott(Change::Institution)),
                    None | Some(_) => Verb::Protest(Protest::StrikeBoycott(Change::Unspecified)),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Protest(Protest::PassageBlock(Change::Leadership)),
                    Some('2') => Verb::Protest(Protest::PassageBlock(Change::Policy)),
                    Some('3') => Verb::Protest(Protest::PassageBlock(Change::Rights)),
                    Some('4') => Verb::Protest(Protest::PassageBlock(Change::Institution)),
                    None | Some(_) => Verb::Protest(Protest::PassageBlock(Change::Unspecified)),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Protest(Protest::ViolentRiot(Change::Leadership)),
                    Some('2') => Verb::Protest(Protest::ViolentRiot(Change::Policy)),
                    Some('3') => Verb::Protest(Protest::ViolentRiot(Change::Rights)),
                    Some('4') => Verb::Protest(Protest::ViolentRiot(Change::Institution)),
                    None | Some(_) => Verb::Protest(Protest::ViolentRiot(Change::Unspecified)),
                },
                None | Some(_) => Verb::Protest(Protest::Unspecified),
            },
            "15" => match str_value.chars().nth(2) {
                Some('1') => Verb::ExhibitForcePosture(ForcePosture::IncreasePoliceAlertStatus),
                Some('2') => Verb::ExhibitForcePosture(ForcePosture::IncreaseMilitaryAlertStatus),
                Some('3') => Verb::ExhibitForcePosture(ForcePosture::MobilizeOrIncreasePolicePower),
                Some('4') => Verb::ExhibitForcePosture(ForcePosture::MobilizeOrIncreaseArmedForces),
                None | Some(_) => Verb::ExhibitForcePosture(ForcePosture::Unspecified),
            },
            "16" => match str_value.chars().nth(2) {
                Some('1') => Verb::ReduceRelations(Relations::Diplomatic),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::ReduceRelations(Relations::MaterialAid(Aid::Economic)),
                    Some('2') => Verb::ReduceRelations(Relations::MaterialAid(Aid::Military)),
                    Some('3') => Verb::ReduceRelations(Relations::MaterialAid(Aid::Humanitarian)),
                    None | Some(_) => {
                        Verb::ReduceRelations(Relations::MaterialAid(Aid::Unspecified))
                    }
                },
                Some('3') => Verb::ReduceRelations(Relations::ImposeEmbargoBoycottSanction),
                Some('4') => Verb::ReduceRelations(Relations::Negotiations),
                Some('5') => Verb::ReduceRelations(Relations::Mediation),
                Some('6') => match str_value.chars().nth(3) {
                    Some('1') => Verb::ReduceRelations(Relations::ExpelWithdraw(
                        InternationalInvolvement::PeaceKeepers,
                    )),
                    Some('2') => Verb::ReduceRelations(Relations::ExpelWithdraw(
                        InternationalInvolvement::InspectorsObservers,
                    )),
                    Some('3') => Verb::ReduceRelations(Relations::ExpelWithdraw(
                        InternationalInvolvement::Aid(Aid::Humanitarian),
                    )),
                    None | Some(_) => Verb::ReduceRelations(Relations::ExpelWithdraw(
                        InternationalInvolvement::Unspecified,
                    )),
                },
                None | Some(_) => Verb::ReduceRelations(Relations::Unspecified),
            },
            "17" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => {
                        Verb::Coerce(Coercion::WithProperty(SeizeDamageProperty::Confiscate))
                    }
                    Some('2') => Verb::Coerce(Coercion::WithProperty(SeizeDamageProperty::Destroy)),
                    None | Some(_) => {
                        Verb::Coerce(Coercion::WithProperty(SeizeDamageProperty::Unspecified))
                    }
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Coerce(Coercion::AdministrativeSanctions(
                        AdministrativeSanctions::PoliticalFreedoms,
                    )),
                    Some('2') => Verb::Coerce(Coercion::AdministrativeSanctions(
                        AdministrativeSanctions::BanPoliticalPartiesOrPoliticians,
                    )),
                    Some('3') => Verb::Coerce(Coercion::AdministrativeSanctions(
                        AdministrativeSanctions::Curfew,
                    )),
                    Some('4') => Verb::Coerce(Coercion::AdministrativeSanctions(
                        AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                    )),
                    None | Some(_) => Verb::Coerce(Coercion::AdministrativeSanctions(
                        AdministrativeSanctions::Unspecified,
                    )),
                },
                Some('3') => Verb::Coerce(Coercion::ArrestDetainOrCharge),
                Some('4') => Verb::Coerce(Coercion::ExpelDeport),
                Some('5') => Verb::Coerce(Coercion::ViolentRepression),
                None | Some(_) => Verb::Coerce(Coercion::Unspecified),
            },
            "18" => match str_value.chars().nth(2) {
                Some('1') => Verb::Assault(Assault::AbductHijackTakeHostage),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Assault(Assault::Physically(PhysicalAssault::Sexual)),
                    Some('2') => Verb::Assault(Assault::Physically(PhysicalAssault::Torture)),
                    Some('3') => Verb::Assault(Assault::Physically(PhysicalAssault::Kill)),
                    None | Some(_) => {
                        Verb::Assault(Assault::Physically(PhysicalAssault::Unspecified))
                    }
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Verb::Assault(Assault::Bombing(Bombing::Suicide)),
                    Some('2') => Verb::Assault(Assault::Bombing(Bombing::Vehicular)),
                    Some('3') => Verb::Assault(Assault::Bombing(Bombing::Roadside)),
                    None | Some(_) => Verb::Assault(Assault::Bombing(Bombing::Unspecified)),
                },
                Some('4') => Verb::Assault(Assault::UseAsHumanShield),
                Some('5') => Verb::Assault(Assault::AttemptToAssasinate),
                Some('6') => Verb::Assault(Assault::Assasinate),
                None | Some(_) => Verb::Assault(Assault::Unspecified),
            },
            "19" => match str_value.chars().nth(2) {
                Some('1') => Verb::Fight(Fight::ImposeBlockade),
                Some('2') => Verb::Fight(Fight::OccupyTerritory),
                Some('3') => Verb::Fight(Fight::SmallArmsLightWeapons),
                Some('4') => Verb::Fight(Fight::ArtilleryAndTanks),
                Some('5') => Verb::Fight(Fight::Arial(ArialWeapons::Unspecified)),
                Some('6') => Verb::Fight(Fight::ViolateCeasefire),
                None | Some(_) => Verb::Fight(Fight::Unspecified),
            },
            "20" => match str_value.chars().nth(2) {
                Some('1') => Verb::UseUnconventionalMassViolence(MassViolence::MassExpulsions),
                Some('2') => Verb::UseUnconventionalMassViolence(MassViolence::MassKillings),
                Some('3') => Verb::UseUnconventionalMassViolence(MassViolence::EthnicCleansing),
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => {
                        Verb::UseUnconventionalMassViolence(MassViolence::WeaponsOfMassDistruction(
                            subcategories::WMD::ChemicalBiologicalRadiological,
                        ))
                    }
                    Some('2') => Verb::UseUnconventionalMassViolence(
                        MassViolence::WeaponsOfMassDistruction(subcategories::WMD::Nuclear),
                    ),
                    None | Some(_) => Verb::UseUnconventionalMassViolence(
                        MassViolence::WeaponsOfMassDistruction(WMD::Unspecified),
                    ),
                },
                None | Some(_) => Verb::UseUnconventionalMassViolence(MassViolence::Unspecified),
            },

            _ => Verb::Unspecified,
        }
    }
}
