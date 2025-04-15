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
        PoliticalFreedoms,
        BanPoliticalPartiesOrPoliticians,
        Curfew,
        StateOfEmergencyOrMartialLaw,
    }

    pub enum PoliticalReform {
        Leadership,
        Policy,
        Rights,
        InstitutionRegime,
    }
}
