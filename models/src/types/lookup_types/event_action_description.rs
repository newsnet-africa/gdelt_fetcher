use crate::types::event_table::event_action::CAMEOEventCode;
use anyhow::anyhow;
use subcategories::{
    AdministrativeSanctions, Aid, ArialWeapons, Assault, Bombing, Change, Coercion, Consultation,
    Cooperation, DiplomaticCooperation, Disapproval, Fight, ForcePosture, InternationalInvolvement,
    Investigation, MassViolence, MaterialCooperation, MilitaryEngagement, MilitaryForce, NonForce,
    PhysicalAssault, PoliticalReform, Protest, PublicStatement, Rejection, Relations,
    ReturnRelease, SeizeDamageProperty, Threat, WMD, Yieldable,
};
use top_level_actions::EventActionDescription;

pub mod top_level_actions {
    use super::subcategories::{
        Aid, Assault, Coercion, Consultation, Cooperation, DiplomaticCooperation, Disapproval,
        Fight, ForcePosture, Investigation, MassViolence, MaterialCooperation, Protest,
        PublicStatement, Rejection, Relations, Threat, Yieldable,
    };

    #[derive(Debug)]
    pub enum EventActionDescription {
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
    #[derive(Debug)]
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

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum Consultation {
        Unspecified,
        DiscussByTelephone,
        MakeAVisit,
        HostAVisit,
        MeetAtThirdLocation,
        Mediate,
        EngageInNegotiation,
    }

    #[derive(Debug)]
    pub enum DiplomaticCooperation {
        Unspecified,
        PraiseOrEndorse,
        DefendEventActionDescriptionally,
        RallySupportOnBehalfOf,
        GrantDiplomaticRecognition,
        Apologise,
        Forgive,
        SignFormalAgreement,
    }

    #[derive(Debug)]
    pub enum MaterialCooperation {
        Unspecified,
        Economic,
        Military,
        Judicial,
        ShareIntelligenceOrInformation,
        Aid(Aid),
    }

    #[derive(Debug)]
    pub enum Aid {
        Unspecified,
        Economic,
        Military,
        Humanitarian,
        MilitaryProtectionOrPeaceKeeping,
        GrantAsylum,
    }

    #[derive(Debug)]
    pub enum ReturnRelease {
        Unspecified,
        Person,
        Property,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum Investigation {
        Unspecified,
        CrimeCorruption,
        HumanRightsAbuses,
        MilitaryAction,
        WarCrimes,
        EspionageTreason,
        Aggression,
    }

    #[derive(Debug)]
    pub enum Disapproval {
        Unspecified,
        CriticiseOrDenounce,
        Accuse(Investigation),
        RallyOppositionAgainst,
        ComplainOfficially,
        BringLawsuitAgainst,
        FindGuiltyOrLiable,
    }

    #[derive(Debug)]
    pub enum Rejection {
        Unspecified,
        Cooperation(Cooperation),
        DefyNorms,
        Veto,
    }

    #[derive(Debug)]
    pub enum NonForce {
        Unspecified,
        ReduceOrStopAid,
        SanctionsBoycottEmbargo,
        ReduceOrBreakRelations,
    }

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub enum MilitaryForce {
        Unspecified,
        Blockade,
        Occupation,
        UnconventionalViolence,
        ConventionalAttack,
        WMD,
    }

    #[derive(Debug)]
    pub enum MilitaryEngagement {
        Unspecified,
        DeclareTruceCeasefire,
        MilitaryBlockade,
        ArmedForces,
        RetreatSurrender,
    }

    #[derive(Debug)]
    pub enum Change {
        Unspecified,
        Leadership,
        Policy,
        Rights,
        Institution,
    }

    #[derive(Debug)]
    pub enum Protest {
        Unspecified,
        DemonstrateOrRally(Change),
        HungerStrike(Change),
        StrikeBoycott(Change),
        PassageBlock(Change),
        ViolentRiot(Change),
    }

    #[derive(Debug)]
    pub enum ForcePosture {
        Unspecified,
        IncreasePoliceAlertStatus,
        IncreaseMilitaryAlertStatus,
        MobilizeOrIncreasePolicePower,
        MobilizeOrIncreaseArmedForces,
        MobilizeOrIncreaseCyberForces,
    }

    #[derive(Debug)]
    pub enum Relations {
        Unspecified,
        Diplomatic,
        MaterialAid(Aid),
        ImposeEmbargoBoycottSanction,
        Negotiations,
        Mediation,
        ExpelWithdraw(InternationalInvolvement),
    }

    #[derive(Debug)]
    pub enum InternationalInvolvement {
        Unspecified,
        PeaceKeepers,
        InspectorsObservers,
        Aid(Aid),
    }

    #[derive(Debug)]
    pub enum SeizeDamageProperty {
        Unspecified,
        Confiscate,
        Destroy,
    }

    #[derive(Debug)]
    pub enum Coercion {
        Unspecified,
        WithProperty(SeizeDamageProperty),
        AdministrativeSanctions(AdministrativeSanctions),
        ArrestDetainOrCharge,
        ExpelDeport,
        ViolentRepression,
        CyberneticAttack,
    }

    #[derive(Debug)]
    pub enum PhysicalAssault {
        Unspecified,
        Sexual,
        Torture,
        Kill,
    }

    #[derive(Debug)]
    pub enum Bombing {
        Unspecified,
        Suicide,
        Vehicular,
        Roadside,
        Location,
    }

    #[derive(Debug)]
    pub enum Assault {
        Unspecified,
        AbductHijackTakeHostage,
        Physically(PhysicalAssault),
        Bombing(Bombing),
        UseAsHumanShield,
        AttemptToAssasinate,
        Assasinate,
    }

    #[derive(Debug)]
    pub enum ArialWeapons {
        Unspecified,
        PrecisionGuided,
        RemotelyPiloted,
    }

    #[derive(Debug)]
    pub enum Fight {
        Unspecified,
        ImposeBlockade,
        OccupyTerritory,
        SmallArmsLightWeapons,
        ArtilleryAndTanks,
        Arial(ArialWeapons),
        ViolateCeasefire,
    }

    #[derive(Debug)]
    pub enum WMD {
        Unspecified,
        ChemicalBiologicalRadiological,
        Nuclear,
    }

    #[derive(Debug)]
    pub enum MassViolence {
        Unspecified,
        MassExpulsions,
        MassKillings,
        EthnicCleansing,
        WeaponsOfMassDistruction(WMD),
    }

    #[derive(Debug)]
    pub enum AdministrativeSanctions {
        Unspecified,
        PoliticalFreedoms,
        BanPoliticalPartiesOrPoliticians,
        Curfew,
        StateOfEmergencyOrMartialLaw,
    }

    #[derive(Debug)]
    pub enum PoliticalReform {
        Unspecified,
        Leadership,
        Policy,
        Rights,
        InstitutionRegime,
    }
}

impl TryFrom<CAMEOEventCode> for EventActionDescription {
    type Error = anyhow::Error;

    fn try_from(value: CAMEOEventCode) -> Result<Self, Self::Error> {
        let str_value = std::str::from_utf8(&value.0).expect("Invalid CAMEO Code format");
        match &str_value[..2] {
            "01" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::DeclineToComment,
                )),
                Some('2') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::MakePessamisticComment,
                )),
                Some('3') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::MakeOptimisticComment,
                )),
                Some('4') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::ConsiderPolicyOption,
                )),
                Some('5') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::AcknowledgeOrClaimResponsibility,
                )),
                Some('6') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::DenyResponsibility,
                )),
                Some('7') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::EngageInSymbolicAct,
                )),
                Some('8') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::MakeEmpatheticComment,
                )),
                Some('9') => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::ExpressAccord,
                )),
                None | Some(_) => Ok(EventActionDescription::MakePublicStatement(
                    PublicStatement::Unspecified,
                )),
            },
            "02" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Appeal(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    )),
                    Some('2') => Ok(EventActionDescription::Appeal(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    )),
                    Some('3') => Ok(EventActionDescription::Appeal(
                        Cooperation::MaterialCooperation(MaterialCooperation::Judicial),
                    )),
                    Some('4') => Ok(EventActionDescription::Appeal(
                        Cooperation::MaterialCooperation(
                            MaterialCooperation::ShareIntelligenceOrInformation,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Appeal(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    )),
                },
                Some('2') => Ok(EventActionDescription::Appeal(
                    Cooperation::DiplomaticCooperation,
                )),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Appeal(Cooperation::Aid(
                        Aid::Economic,
                    ))),
                    Some('2') => Ok(EventActionDescription::Appeal(Cooperation::Aid(
                        Aid::Military,
                    ))),
                    Some('3') => Ok(EventActionDescription::Appeal(Cooperation::Aid(
                        Aid::Humanitarian,
                    ))),
                    Some('4') => Ok(EventActionDescription::Appeal(Cooperation::Aid(
                        Aid::MilitaryProtectionOrPeaceKeeping,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Appeal(Cooperation::Aid(
                        Aid::Unspecified,
                    ))),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Appeal(
                        Cooperation::PoliticalReform(PoliticalReform::Leadership),
                    )),
                    Some('2') => Ok(EventActionDescription::Appeal(
                        Cooperation::PoliticalReform(PoliticalReform::Policy),
                    )),
                    Some('3') => Ok(EventActionDescription::Appeal(
                        Cooperation::PoliticalReform(PoliticalReform::Rights),
                    )),
                    Some('4') => Ok(EventActionDescription::Appeal(
                        Cooperation::PoliticalReform(PoliticalReform::InstitutionRegime),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Appeal(
                        Cooperation::PoliticalReform(PoliticalReform::Unspecified),
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::AdministrativeSanctions(
                            subcategories::AdministrativeSanctions::Unspecified,
                        ),
                    ))),
                    Some('2') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::PoliticalDissent,
                    ))),
                    Some('3') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::ReturnRelease(ReturnRelease::Unspecified),
                    ))),
                    Some('4') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::EconomicSanctions,
                    ))),
                    Some('5') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    ))),
                    Some('6') => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(
                            subcategories::MilitaryEngagement::Unspecified,
                        ),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Appeal(Cooperation::Yield(
                        Yieldable::Unspecified,
                    ))),
                },
                Some('6') => Ok(EventActionDescription::Appeal(
                    Cooperation::ToMeetOrNegotiate,
                )),
                Some('7') => Ok(EventActionDescription::Appeal(Cooperation::SettleDispute)),
                Some('8') => Ok(EventActionDescription::Appeal(Cooperation::AcceptMediation)),
                None | Some(_) => Ok(EventActionDescription::Appeal(Cooperation::Unspecified)),
            },
            "03" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    )),
                    Some('2') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    )),
                    Some('3') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::MaterialCooperation(MaterialCooperation::Judicial),
                    )),
                    Some('4') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::MaterialCooperation(
                            MaterialCooperation::ShareIntelligenceOrInformation,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    )),
                },
                Some('2') => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::DiplomaticCooperation,
                )),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Aid(Aid::Economic),
                    )),
                    Some('2') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Aid(Aid::Military),
                    )),
                    Some('3') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Aid(Aid::Humanitarian),
                    )),
                    Some('4') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Aid(Aid::MilitaryProtectionOrPeaceKeeping),
                    )),
                    None | Some(_) => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Aid(Aid::Unspecified),
                    )),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::PoliticalReform(PoliticalReform::Leadership),
                    )),
                    Some('2') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::PoliticalReform(PoliticalReform::Policy),
                    )),
                    Some('3') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::PoliticalReform(PoliticalReform::Rights),
                    )),
                    Some('4') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::PoliticalReform(PoliticalReform::InstitutionRegime),
                    )),
                    None | Some(_) => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::PoliticalReform(PoliticalReform::Unspecified),
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::AdministrativeSanctions(
                            subcategories::AdministrativeSanctions::Unspecified,
                        )),
                    )),
                    Some('2') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::PoliticalDissent),
                    )),
                    Some('3') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::ReturnRelease(ReturnRelease::Unspecified)),
                    )),
                    Some('4') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::EconomicSanctions),
                    )),
                    Some('5') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::InternationalInvolvement(
                            InternationalInvolvement::Unspecified,
                        )),
                    )),
                    Some('6') => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::DeEscelateMilitaryEngagement(
                            subcategories::MilitaryEngagement::Unspecified,
                        )),
                    )),
                    None | Some(_) => Ok(EventActionDescription::IntentionToCooperate(
                        Cooperation::Yield(Yieldable::Unspecified),
                    )),
                },
                Some('6') => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::ToMeetOrNegotiate,
                )),
                Some('7') => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::SettleDispute,
                )),
                Some('8') => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::AcceptMediation,
                )),
                Some('9') => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::Mediate,
                )),
                None | Some(_) => Ok(EventActionDescription::IntentionToCooperate(
                    Cooperation::Unspecified,
                )),
            },
            "04" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::Consult(
                    Consultation::DiscussByTelephone,
                )),
                Some('2') => Ok(EventActionDescription::Consult(Consultation::MakeAVisit)),
                Some('3') => Ok(EventActionDescription::Consult(Consultation::HostAVisit)),
                Some('4') => Ok(EventActionDescription::Consult(
                    Consultation::MeetAtThirdLocation,
                )),
                Some('5') => Ok(EventActionDescription::Consult(Consultation::Mediate)),
                Some('6') => Ok(EventActionDescription::Consult(
                    Consultation::EngageInNegotiation,
                )),
                None | Some(_) => Ok(EventActionDescription::Consult(Consultation::Unspecified)),
            },
            "05" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::PraiseOrEndorse,
                )),
                Some('2') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::DefendEventActionDescriptionally,
                )),
                Some('3') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::RallySupportOnBehalfOf,
                )),
                Some('4') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::GrantDiplomaticRecognition,
                )),
                Some('5') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::Apologise,
                )),
                Some('6') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::Forgive,
                )),
                Some('7') => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::SignFormalAgreement,
                )),
                None | Some(_) => Ok(EventActionDescription::EngageInDiplomaticCooperation(
                    DiplomaticCooperation::Unspecified,
                )),
            },
            "06" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::EngageInMaterialCooperation(
                    MaterialCooperation::Economic,
                )),
                Some('2') => Ok(EventActionDescription::EngageInMaterialCooperation(
                    MaterialCooperation::Military,
                )),
                Some('3') => Ok(EventActionDescription::EngageInMaterialCooperation(
                    MaterialCooperation::Judicial,
                )),
                Some('4') => Ok(EventActionDescription::EngageInMaterialCooperation(
                    MaterialCooperation::ShareIntelligenceOrInformation,
                )),
                None | Some(_) => Ok(EventActionDescription::EngageInMaterialCooperation(
                    MaterialCooperation::Unspecified,
                )),
            },
            "07" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::ProvideAid(Aid::Economic)),
                Some('2') => Ok(EventActionDescription::ProvideAid(Aid::Military)),
                Some('3') => Ok(EventActionDescription::ProvideAid(Aid::Humanitarian)),
                Some('4') => Ok(EventActionDescription::ProvideAid(
                    Aid::MilitaryProtectionOrPeaceKeeping,
                )),
                Some('5') => Ok(EventActionDescription::ProvideAid(Aid::GrantAsylum)),
                None | Some(_) => Ok(EventActionDescription::ProvideAid(Aid::Unspecified)),
            },
            "08" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Yield(
                        Yieldable::AdministrativeSanctions(
                            AdministrativeSanctions::PoliticalFreedoms,
                        ),
                    )),
                    Some('2') => Ok(EventActionDescription::Yield(
                        Yieldable::AdministrativeSanctions(AdministrativeSanctions::Curfew),
                    )),
                    Some('3') => Ok(EventActionDescription::Yield(
                        Yieldable::AdministrativeSanctions(
                            AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Yield(
                        Yieldable::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    )),
                },
                Some('2') => Ok(EventActionDescription::Yield(Yieldable::PoliticalDissent)),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Yield(Yieldable::PoliticalReform(
                        PoliticalReform::Leadership,
                    ))),
                    Some('2') => Ok(EventActionDescription::Yield(Yieldable::PoliticalReform(
                        PoliticalReform::Policy,
                    ))),
                    Some('3') => Ok(EventActionDescription::Yield(Yieldable::PoliticalReform(
                        PoliticalReform::Rights,
                    ))),
                    Some('4') => Ok(EventActionDescription::Yield(Yieldable::PoliticalReform(
                        PoliticalReform::InstitutionRegime,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Yield(
                        Yieldable::PoliticalReform(PoliticalReform::Unspecified),
                    )),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Yield(Yieldable::ReturnRelease(
                        ReturnRelease::Person,
                    ))),
                    Some('2') => Ok(EventActionDescription::Yield(Yieldable::ReturnRelease(
                        ReturnRelease::Property,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Yield(Yieldable::ReturnRelease(
                        ReturnRelease::Unspecified,
                    ))),
                },
                Some('5') => Ok(EventActionDescription::Yield(Yieldable::EconomicSanctions)),
                Some('6') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::PeaceKeepers),
                    )),
                    Some('2') => Ok(EventActionDescription::Yield(
                        Yieldable::InternationalInvolvement(
                            InternationalInvolvement::InspectorsObservers,
                        ),
                    )),
                    Some('3') => Ok(EventActionDescription::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Aid(
                            Aid::Unspecified,
                        )),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    )),
                },
                Some('7') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(
                            MilitaryEngagement::DeclareTruceCeasefire,
                        ),
                    )),
                    Some('2') => Ok(EventActionDescription::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(
                            MilitaryEngagement::MilitaryBlockade,
                        ),
                    )),
                    Some('3') => Ok(EventActionDescription::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(MilitaryEngagement::ArmedForces),
                    )),
                    Some('4') => Ok(EventActionDescription::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(
                            MilitaryEngagement::RetreatSurrender,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(MilitaryEngagement::Unspecified),
                    )),
                },
                None | Some(_) => Ok(EventActionDescription::Yield(Yieldable::Unspecified)),
            },
            "09" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::Investigate(
                    Investigation::CrimeCorruption,
                )),
                Some('2') => Ok(EventActionDescription::Investigate(
                    Investigation::HumanRightsAbuses,
                )),
                Some('3') => Ok(EventActionDescription::Investigate(
                    Investigation::MilitaryAction,
                )),
                Some('4') => Ok(EventActionDescription::Investigate(
                    Investigation::WarCrimes,
                )),
                None | Some(_) => Ok(EventActionDescription::Investigate(
                    Investigation::Unspecified,
                )),
            },
            "10" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Demand(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    )),
                    Some('2') => Ok(EventActionDescription::Demand(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    )),
                    Some('3') => Ok(EventActionDescription::Demand(
                        Cooperation::MaterialCooperation(MaterialCooperation::Judicial),
                    )),
                    Some('4') => Ok(EventActionDescription::Demand(
                        Cooperation::MaterialCooperation(
                            MaterialCooperation::ShareIntelligenceOrInformation,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Demand(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    )),
                },
                Some('2') => Ok(EventActionDescription::Demand(
                    Cooperation::DiplomaticCooperation,
                )),
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Demand(Cooperation::Aid(
                        Aid::Economic,
                    ))),
                    Some('2') => Ok(EventActionDescription::Demand(Cooperation::Aid(
                        Aid::Military,
                    ))),
                    Some('3') => Ok(EventActionDescription::Demand(Cooperation::Aid(
                        Aid::Humanitarian,
                    ))),
                    Some('4') => Ok(EventActionDescription::Demand(Cooperation::Aid(
                        Aid::MilitaryProtectionOrPeaceKeeping,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Demand(Cooperation::Aid(
                        Aid::Unspecified,
                    ))),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Demand(
                        Cooperation::PoliticalReform(PoliticalReform::Leadership),
                    )),
                    Some('2') => Ok(EventActionDescription::Demand(
                        Cooperation::PoliticalReform(PoliticalReform::Policy),
                    )),
                    Some('3') => Ok(EventActionDescription::Demand(
                        Cooperation::PoliticalReform(PoliticalReform::Rights),
                    )),
                    Some('4') => Ok(EventActionDescription::Demand(
                        Cooperation::PoliticalReform(PoliticalReform::InstitutionRegime),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Demand(
                        Cooperation::PoliticalReform(PoliticalReform::Unspecified),
                    )),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    ))),
                    Some('2') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::PoliticalDissent,
                    ))),
                    Some('3') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::ReturnRelease(ReturnRelease::Unspecified),
                    ))),
                    Some('4') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::EconomicSanctions,
                    ))),
                    Some('5') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::InternationalInvolvement(InternationalInvolvement::Unspecified),
                    ))),
                    Some('6') => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::DeEscelateMilitaryEngagement(MilitaryEngagement::Unspecified),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Demand(Cooperation::Yield(
                        Yieldable::Unspecified,
                    ))),
                },
                Some('6') => Ok(EventActionDescription::Demand(Cooperation::Withdraw)),
                Some('7') => Ok(EventActionDescription::Demand(Cooperation::Ceasefire)),
                Some('8') => Ok(EventActionDescription::Demand(
                    Cooperation::ToMeetOrNegotiate,
                )),
                None | Some(_) => Ok(EventActionDescription::Demand(Cooperation::Unspecified)),
            },
            "11" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::Disapprove(
                    Disapproval::CriticiseOrDenounce,
                )),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::CrimeCorruption,
                    ))),
                    Some('2') => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::HumanRightsAbuses,
                    ))),
                    Some('3') => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::Aggression,
                    ))),
                    Some('4') => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::WarCrimes,
                    ))),
                    Some('5') => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::EspionageTreason,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Disapprove(Disapproval::Accuse(
                        Investigation::Unspecified,
                    ))),
                },
                Some('3') => Ok(EventActionDescription::Disapprove(
                    Disapproval::RallyOppositionAgainst,
                )),
                Some('4') => Ok(EventActionDescription::Disapprove(
                    Disapproval::ComplainOfficially,
                )),
                Some('5') => Ok(EventActionDescription::Disapprove(
                    Disapproval::BringLawsuitAgainst,
                )),
                Some('6') => Ok(EventActionDescription::Disapprove(
                    Disapproval::FindGuiltyOrLiable,
                )),
                None | Some(_) => Ok(EventActionDescription::Disapprove(Disapproval::Unspecified)),
            },
            "12" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    ))),
                    Some('2') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    ))),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Economic),
                    ))),
                    Some('2') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Military),
                    ))),
                    Some('3') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Aid(
                            Aid::Humanitarian,
                        )),
                    ))),
                    Some('4') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Aid(
                            Aid::MilitaryProtectionOrPeaceKeeping,
                        )),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::MaterialCooperation(MaterialCooperation::Unspecified),
                    ))),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Leadership),
                    ))),
                    Some('2') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Policy),
                    ))),
                    Some('3') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Rights),
                    ))),
                    Some('4') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::InstitutionRegime),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::PoliticalReform(PoliticalReform::Unspecified),
                    ))),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::AdministrativeSanctions(
                            AdministrativeSanctions::Unspecified,
                        )),
                    ))),
                    Some('2') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::PoliticalDissent),
                    ))),
                    Some('3') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::ReturnRelease(ReturnRelease::Unspecified)),
                    ))),
                    Some('4') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::EconomicSanctions),
                    ))),
                    Some('5') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::InternationalInvolvement(
                            InternationalInvolvement::Unspecified,
                        )),
                    ))),
                    Some('6') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::DeEscelateMilitaryEngagement(
                            MilitaryEngagement::Unspecified,
                        )),
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                        Cooperation::Yield(Yieldable::Unspecified),
                    ))),
                },
                Some('5') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                    Cooperation::ToMeetOrNegotiate,
                ))),
                Some('6') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                    Cooperation::AcceptMediation,
                ))),
                Some('7') => Ok(EventActionDescription::Reject(Rejection::Cooperation(
                    Cooperation::SettleDispute,
                ))),
                Some('8') => Ok(EventActionDescription::Reject(Rejection::DefyNorms)),
                Some('9') => Ok(EventActionDescription::Reject(Rejection::Veto)),
                None | Some(_) => Ok(EventActionDescription::Reject(Rejection::Unspecified)),
            },
            "13" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Threaten(Threat::NonForce(
                        NonForce::ReduceOrStopAid,
                    ))),
                    Some('2') => Ok(EventActionDescription::Threaten(Threat::NonForce(
                        NonForce::SanctionsBoycottEmbargo,
                    ))),
                    Some('3') => Ok(EventActionDescription::Threaten(Threat::NonForce(
                        NonForce::ReduceOrBreakRelations,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Threaten(Threat::NonForce(
                        subcategories::NonForce::Unspecified,
                    ))),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Threaten(
                        Threat::AdministrativeSanctions(AdministrativeSanctions::PoliticalFreedoms),
                    )),
                    Some('2') => Ok(EventActionDescription::Threaten(
                        Threat::AdministrativeSanctions(
                            AdministrativeSanctions::BanPoliticalPartiesOrPoliticians,
                        ),
                    )),
                    Some('3') => Ok(EventActionDescription::Threaten(
                        Threat::AdministrativeSanctions(AdministrativeSanctions::Curfew),
                    )),
                    Some('4') => Ok(EventActionDescription::Threaten(
                        Threat::AdministrativeSanctions(
                            AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Threaten(
                        Threat::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    )),
                },
                Some('3') => Ok(EventActionDescription::Threaten(
                    Threat::PoliticalDissentOrProtest,
                )),
                Some('4') => Ok(EventActionDescription::Threaten(Threat::HaltNegotiations)),
                Some('5') => Ok(EventActionDescription::Threaten(Threat::HaltMediation)),
                Some('6') => Ok(EventActionDescription::Threaten(
                    Threat::HaltInternationalInvolvement,
                )),
                Some('7') => Ok(EventActionDescription::Threaten(Threat::Repression)),
                Some('8') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::Blockade,
                    ))),
                    Some('2') => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::Occupation,
                    ))),
                    Some('3') => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::UnconventionalViolence,
                    ))),
                    Some('4') => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::ConventionalAttack,
                    ))),
                    Some('5') => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::WMD,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Threaten(Threat::MilitaryForce(
                        MilitaryForce::Unspecified,
                    ))),
                },
                Some('9') => Ok(EventActionDescription::Threaten(Threat::Ultimatum)),
                None | Some(_) => Ok(EventActionDescription::Threaten(Threat::Unspecified)),
            },
            "14" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Protest(
                        Protest::DemonstrateOrRally(Change::Leadership),
                    )),
                    Some('2') => Ok(EventActionDescription::Protest(
                        Protest::DemonstrateOrRally(Change::Policy),
                    )),
                    Some('3') => Ok(EventActionDescription::Protest(
                        Protest::DemonstrateOrRally(Change::Rights),
                    )),
                    Some('4') => Ok(EventActionDescription::Protest(
                        Protest::DemonstrateOrRally(Change::Institution),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Protest(
                        Protest::DemonstrateOrRally(Change::Unspecified),
                    )),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Protest(Protest::HungerStrike(
                        Change::Leadership,
                    ))),
                    Some('2') => Ok(EventActionDescription::Protest(Protest::HungerStrike(
                        Change::Policy,
                    ))),
                    Some('3') => Ok(EventActionDescription::Protest(Protest::HungerStrike(
                        Change::Rights,
                    ))),
                    Some('4') => Ok(EventActionDescription::Protest(Protest::HungerStrike(
                        Change::Institution,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Protest(Protest::HungerStrike(
                        Change::Unspecified,
                    ))),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Protest(Protest::StrikeBoycott(
                        Change::Leadership,
                    ))),
                    Some('2') => Ok(EventActionDescription::Protest(Protest::StrikeBoycott(
                        Change::Policy,
                    ))),
                    Some('3') => Ok(EventActionDescription::Protest(Protest::StrikeBoycott(
                        Change::Rights,
                    ))),
                    Some('4') => Ok(EventActionDescription::Protest(Protest::StrikeBoycott(
                        Change::Institution,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Protest(Protest::StrikeBoycott(
                        Change::Unspecified,
                    ))),
                },
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Protest(Protest::PassageBlock(
                        Change::Leadership,
                    ))),
                    Some('2') => Ok(EventActionDescription::Protest(Protest::PassageBlock(
                        Change::Policy,
                    ))),
                    Some('3') => Ok(EventActionDescription::Protest(Protest::PassageBlock(
                        Change::Rights,
                    ))),
                    Some('4') => Ok(EventActionDescription::Protest(Protest::PassageBlock(
                        Change::Institution,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Protest(Protest::PassageBlock(
                        Change::Unspecified,
                    ))),
                },
                Some('5') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Protest(Protest::ViolentRiot(
                        Change::Leadership,
                    ))),
                    Some('2') => Ok(EventActionDescription::Protest(Protest::ViolentRiot(
                        Change::Policy,
                    ))),
                    Some('3') => Ok(EventActionDescription::Protest(Protest::ViolentRiot(
                        Change::Rights,
                    ))),
                    Some('4') => Ok(EventActionDescription::Protest(Protest::ViolentRiot(
                        Change::Institution,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Protest(Protest::ViolentRiot(
                        Change::Unspecified,
                    ))),
                },
                None | Some(_) => Ok(EventActionDescription::Protest(Protest::Unspecified)),
            },
            "15" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::ExhibitForcePosture(
                    ForcePosture::IncreasePoliceAlertStatus,
                )),
                Some('2') => Ok(EventActionDescription::ExhibitForcePosture(
                    ForcePosture::IncreaseMilitaryAlertStatus,
                )),
                Some('3') => Ok(EventActionDescription::ExhibitForcePosture(
                    ForcePosture::MobilizeOrIncreasePolicePower,
                )),
                Some('4') => Ok(EventActionDescription::ExhibitForcePosture(
                    ForcePosture::MobilizeOrIncreaseArmedForces,
                )),
                None | Some(_) => Ok(EventActionDescription::ExhibitForcePosture(
                    ForcePosture::Unspecified,
                )),
            },
            "16" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::ReduceRelations(
                    Relations::Diplomatic,
                )),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::ReduceRelations(
                        Relations::MaterialAid(Aid::Economic),
                    )),
                    Some('2') => Ok(EventActionDescription::ReduceRelations(
                        Relations::MaterialAid(Aid::Military),
                    )),
                    Some('3') => Ok(EventActionDescription::ReduceRelations(
                        Relations::MaterialAid(Aid::Humanitarian),
                    )),
                    None | Some(_) => Ok(EventActionDescription::ReduceRelations(
                        Relations::MaterialAid(Aid::Unspecified),
                    )),
                },
                Some('3') => Ok(EventActionDescription::ReduceRelations(
                    Relations::ImposeEmbargoBoycottSanction,
                )),
                Some('4') => Ok(EventActionDescription::ReduceRelations(
                    Relations::Negotiations,
                )),
                Some('5') => Ok(EventActionDescription::ReduceRelations(
                    Relations::Mediation,
                )),
                Some('6') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::ReduceRelations(
                        Relations::ExpelWithdraw(InternationalInvolvement::PeaceKeepers),
                    )),
                    Some('2') => Ok(EventActionDescription::ReduceRelations(
                        Relations::ExpelWithdraw(InternationalInvolvement::InspectorsObservers),
                    )),
                    Some('3') => Ok(EventActionDescription::ReduceRelations(
                        Relations::ExpelWithdraw(InternationalInvolvement::Aid(Aid::Humanitarian)),
                    )),
                    None | Some(_) => Ok(EventActionDescription::ReduceRelations(
                        Relations::ExpelWithdraw(InternationalInvolvement::Unspecified),
                    )),
                },
                None | Some(_) => Ok(EventActionDescription::ReduceRelations(
                    Relations::Unspecified,
                )),
            },
            "17" => match str_value.chars().nth(2) {
                Some('1') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Coerce(Coercion::WithProperty(
                        SeizeDamageProperty::Confiscate,
                    ))),
                    Some('2') => Ok(EventActionDescription::Coerce(Coercion::WithProperty(
                        SeizeDamageProperty::Destroy,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Coerce(Coercion::WithProperty(
                        SeizeDamageProperty::Unspecified,
                    ))),
                },
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Coerce(
                        Coercion::AdministrativeSanctions(
                            AdministrativeSanctions::PoliticalFreedoms,
                        ),
                    )),
                    Some('2') => Ok(EventActionDescription::Coerce(
                        Coercion::AdministrativeSanctions(
                            AdministrativeSanctions::BanPoliticalPartiesOrPoliticians,
                        ),
                    )),
                    Some('3') => Ok(EventActionDescription::Coerce(
                        Coercion::AdministrativeSanctions(AdministrativeSanctions::Curfew),
                    )),
                    Some('4') => Ok(EventActionDescription::Coerce(
                        Coercion::AdministrativeSanctions(
                            AdministrativeSanctions::StateOfEmergencyOrMartialLaw,
                        ),
                    )),
                    None | Some(_) => Ok(EventActionDescription::Coerce(
                        Coercion::AdministrativeSanctions(AdministrativeSanctions::Unspecified),
                    )),
                },
                Some('3') => Ok(EventActionDescription::Coerce(
                    Coercion::ArrestDetainOrCharge,
                )),
                Some('4') => Ok(EventActionDescription::Coerce(Coercion::ExpelDeport)),
                Some('5') => Ok(EventActionDescription::Coerce(Coercion::ViolentRepression)),
                None | Some(_) => Ok(EventActionDescription::Coerce(Coercion::Unspecified)),
            },
            "18" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::Assault(
                    Assault::AbductHijackTakeHostage,
                )),
                Some('2') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Assault(Assault::Physically(
                        PhysicalAssault::Sexual,
                    ))),
                    Some('2') => Ok(EventActionDescription::Assault(Assault::Physically(
                        PhysicalAssault::Torture,
                    ))),
                    Some('3') => Ok(EventActionDescription::Assault(Assault::Physically(
                        PhysicalAssault::Kill,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Assault(Assault::Physically(
                        PhysicalAssault::Unspecified,
                    ))),
                },
                Some('3') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::Assault(Assault::Bombing(
                        Bombing::Suicide,
                    ))),
                    Some('2') => Ok(EventActionDescription::Assault(Assault::Bombing(
                        Bombing::Vehicular,
                    ))),
                    Some('3') => Ok(EventActionDescription::Assault(Assault::Bombing(
                        Bombing::Roadside,
                    ))),
                    None | Some(_) => Ok(EventActionDescription::Assault(Assault::Bombing(
                        Bombing::Unspecified,
                    ))),
                },
                Some('4') => Ok(EventActionDescription::Assault(Assault::UseAsHumanShield)),
                Some('5') => Ok(EventActionDescription::Assault(
                    Assault::AttemptToAssasinate,
                )),
                Some('6') => Ok(EventActionDescription::Assault(Assault::Assasinate)),
                None | Some(_) => Ok(EventActionDescription::Assault(Assault::Unspecified)),
            },
            "19" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::Fight(Fight::ImposeBlockade)),
                Some('2') => Ok(EventActionDescription::Fight(Fight::OccupyTerritory)),
                Some('3') => Ok(EventActionDescription::Fight(Fight::SmallArmsLightWeapons)),
                Some('4') => Ok(EventActionDescription::Fight(Fight::ArtilleryAndTanks)),
                Some('5') => Ok(EventActionDescription::Fight(Fight::Arial(
                    ArialWeapons::Unspecified,
                ))),
                Some('6') => Ok(EventActionDescription::Fight(Fight::ViolateCeasefire)),
                None | Some(_) => Ok(EventActionDescription::Fight(Fight::Unspecified)),
            },
            "20" => match str_value.chars().nth(2) {
                Some('1') => Ok(EventActionDescription::UseUnconventionalMassViolence(
                    MassViolence::MassExpulsions,
                )),
                Some('2') => Ok(EventActionDescription::UseUnconventionalMassViolence(
                    MassViolence::MassKillings,
                )),
                Some('3') => Ok(EventActionDescription::UseUnconventionalMassViolence(
                    MassViolence::EthnicCleansing,
                )),
                Some('4') => match str_value.chars().nth(3) {
                    Some('1') => Ok(EventActionDescription::UseUnconventionalMassViolence(
                        MassViolence::WeaponsOfMassDistruction(
                            subcategories::WMD::ChemicalBiologicalRadiological,
                        ),
                    )),
                    Some('2') => Ok(EventActionDescription::UseUnconventionalMassViolence(
                        MassViolence::WeaponsOfMassDistruction(subcategories::WMD::Nuclear),
                    )),
                    None | Some(_) => Ok(EventActionDescription::UseUnconventionalMassViolence(
                        MassViolence::WeaponsOfMassDistruction(WMD::Unspecified),
                    )),
                },
                None | Some(_) => Ok(EventActionDescription::UseUnconventionalMassViolence(
                    MassViolence::Unspecified,
                )),
            },

            _ => Err(anyhow!("Invalid Action code")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            env_logger::init();
        });
    }

    #[test]
    fn test_event_action_description_try_from_valid_codes() {
        init_logger();

        let valid_code = Some("0101"); // Example: MakePublicStatement
        info!("Testing valid EventActionDescription code: {:?}", valid_code);
        let event_action = EventActionDescription::try_from(CAMEOEventCode::try_from(valid_code).unwrap());
        assert!(event_action.is_ok());
        assert!(matches!(event_action.unwrap(), EventActionDescription::MakePublicStatement(_)));
    }

    #[test]
    fn test_event_action_description_try_from_invalid_codes() {
        init_logger();

        let invalid_code = Some("9999"); // Invalid code
        info!("Testing invalid EventActionDescription code: {:?}", invalid_code);
        let event_action = EventActionDescription::try_from(CAMEOEventCode::try_from(invalid_code).unwrap());
        assert!(event_action.is_err());
    }

    #[test]
    fn test_event_action_description_try_from_edge_cases() {
        init_logger();

        let empty_code = Some(""); // Empty code
        info!("Testing empty EventActionDescription code: {:?}", empty_code);
        let empty_result = CAMEOEventCode::try_from(empty_code);
        assert!(empty_result.is_err());

        let short_code = Some("01"); // Short code
        info!("Testing short EventActionDescription code: {:?}", short_code);
        let short_result = CAMEOEventCode::try_from(short_code);
        assert!(short_result.is_err());
    }

    #[test]
    fn test_subcategories_try_from_valid_codes() {
        init_logger();

        let valid_code = Some("0101"); // Example: PublicStatement
        info!("Testing valid subcategory code: {:?}", valid_code);
        let event_action = EventActionDescription::try_from(CAMEOEventCode::try_from(valid_code).unwrap());
        assert!(event_action.is_ok());
        assert!(matches!(event_action.unwrap(), EventActionDescription::MakePublicStatement(PublicStatement::Unspecified)));
    }

    #[test]
    fn test_subcategories_try_from_invalid_codes() {
        init_logger();

        let invalid_code = Some("9999"); // Invalid code
        info!("Testing invalid subcategory code: {:?}", invalid_code);
        let event_action = EventActionDescription::try_from(CAMEOEventCode::try_from(invalid_code).unwrap());
        assert!(event_action.is_err());
    }

    #[test]
    fn test_subcategories_try_from_edge_cases() {
        init_logger();

        let empty_code = Some(""); // Empty code
        info!("Testing empty subcategory code: {:?}", empty_code);
        let empty_result = CAMEOEventCode::try_from(empty_code);
        assert!(empty_result.is_err());

        let short_code = Some("01"); // Short code
        info!("Testing short subcategory code: {:?}", short_code);
        let short_result = CAMEOEventCode::try_from(short_code);
        assert!(short_result.is_err());
    }
}