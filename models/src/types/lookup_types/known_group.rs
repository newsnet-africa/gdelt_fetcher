use crate::types::event_table::actor::CAMEOKnownGroupCode;
use anyhow::anyhow;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum KnownGroup {
    Unspecified,
    AlAqsaMartyrsBrigade,
    ArabBankforEconomicDevelopmentinAfrica,
    ArabCooperationCouncil,
    AsianDevelopmentBank,
    ArabEconomicUnityCouncil,
    AfricanDevelopmentBank,
    AlQaeda,
    ArabMonetaryFundforEconomicandSocialDevelopment,
    AmalMilitia,
    AmnestyInternational,
    ArabMaghrebUnion,
    AbuNidalOrganization,
    OrgofArabPetroleumExportingCountriesOAPEC,
    ArabLeague,
    SouthLebanonArmy,
    AssociationofSoutheastAsianNationsASEAN,
    EasternandSouthernAfricanTradeandDevelopmentBank,
    BankofCentralAfricanStatesBEAC,
    BankforInternationalSettlements,
    BaathParty,
    CommonMarketforEasternandSouthernAfrica,
    MonetaryandEconomicCommunityofCentralAfrica,
    FrancZoneFinancialCommunityofAfrica,
    CommonwealthofIndependentStates,
    Communist,
    CouncilofEurope,
    CocoaProducersAlliance,
    AssociationofCoffeeProducingCountries,
    InternationalFedofRedCrossandRedCrescentICRC,
    CommunityofSahelSaharanStatesCENSAD,
    CommonwealthofNations,
    DemocraticFrontfortheLibofPalestineDFLP,
    EuropeanBankforReconstructionandDevelopment,
    EconomicCommunityofCentralAfricanStates,
    EuropeanUnion,
    EuropeanFreeTradeAssociation,
    EnnahdaMovement,
    UnitedNationsFoodandAgricultureOrganization,
    InternationalFederationofHumanRightsFIDH,
    IslamicSalvationArmy,
    NationalLiberationFrontFLN,
    Fatah,
    GulfCooperationCouncil,
    ArmedIslamicGroupGIA,
    GroupofEightG8G7plusRussia,
    GroupofSevenG7,
    SalafistGroup,
    GroupofSeventySevenG77,
    UNHighCommissionforHumanRights,
    UNHighCommissionforRefugees,
    Hezbullah,
    HighlyIndebtedPoorCountriesHIPC,
    Hamas,
    HumanRightsWatch,
    InterAfricanCoffeeOrganizationIACO,
    IntergovernmentalAuthorityonDevelopmentIGAD,
    InternationalAtomicEnergyAgencyIAEA,
    IslamicActionFront,
    InternationalCriminalCourt,
    InternationalCrisisGroup,
    InternationalCourtofJusticeICJ,
    InternationalCocoaOrganizationICCO,
    IslamicDevelopmentBank,
    InternationalGrainsCouncil,
    InternationalHelsinkiFederationforHumanRights,
    InternationalLaborOrganization,
    InternationalMonetaryFundIMF,
    InternationalOrganizationforMigration,
    InterParliamentaryUnion,
    RedCross,
    PalestinianIslamicJihad,
    Interpol,
    InternationalCommissionofJurists,
    KurdishDemocraticPartyKDP,
    UnitedNationsChildrensFundUNICEF,
    IsraeliLaborParty,
    LikudParty,
    MuslimBrotherhood,
    MeretzParty,
    MedecinsSansFrontieresDoctorsWithoutBorders,
    MovementoftheSocietyforPeace,
    NorthAtlanticTreatyOrganizationNATO,
    NewEconomicPartnershipforAfricasDevelopment,
    OrganizationofNonAlignedCountries,
    OrganizationofAmericanStates,
    OrganizationofAfricanUnityOAU,
    OrganizationofIslamicConferencesOIC,
    OrganizationofPetroleumExportingCountriesOPEC,
    PanAfricanParliament,
    PeoplesFrontfortheLiberationofPalestinePFLP,
    PalestineLiberationFront,
    PalestineLiberationOrganization,
    PolisarioGuerillas,
    PeoplesMujahedeen,
    ParisClub,
    OccupiedPalestinianTerritories,
    RedCrescent,
    DemocraticNationalRally,
    SouthAsianAssociation,
    SouthernAfricanDevelopmentCommunity,
    CouncilofSecurityandCooperationinEuropeOSCE,
    ShasParty,
    SoutheastAsiaCollectiveDefenseTreatySEATO,
    Taliban,
    EconomicandMonetaryUnionofWestAfricaUEMOA,
    UnitedNations,
    WestAfricaDevelopmentBank,
    WestAfricaMonetaryandEconomicUnion,
    EconomicCommunityofWestAfricanStatesECOWAS,
    WorldBank,
    InternationalWarCrimesTribunals,
    WorldEconomicForum,
    WorldFoodProgram,
    WorldHealthOrganization,
    WorldTradeOrganization,
    WorldTradeOrganizationWTO,
    Oxfam,
}

impl TryFrom<Option<CAMEOKnownGroupCode>> for KnownGroup {
    type Error = anyhow::Error;

    fn try_from(value: Option<CAMEOKnownGroupCode>) -> anyhow::Result<Self> {
        let value = value.ok_or_else(|| anyhow!("CAMEOKnownGroupCode is None"))?;
        match std::str::from_utf8(&value.0)? {
            "AAM" => Ok(Self::AlAqsaMartyrsBrigade),
            "ABD" => Ok(Self::ArabBankforEconomicDevelopmentinAfrica),
            "ACC" => Ok(Self::ArabCooperationCouncil),
            "ADB" => Ok(Self::AsianDevelopmentBank),
            "AEU" => Ok(Self::ArabEconomicUnityCouncil),
            "AFB" => Ok(Self::AfricanDevelopmentBank),
            "ALQ" => Ok(Self::AlQaeda),
            "AMF" => Ok(Self::ArabMonetaryFundforEconomicandSocialDevelopment),
            "AML" => Ok(Self::AmalMilitia),
            "AMN" => Ok(Self::AmnestyInternational),
            "AMU" => Ok(Self::ArabMaghrebUnion),
            "ANO" => Ok(Self::AbuNidalOrganization),
            "APE" => Ok(Self::OrgofArabPetroleumExportingCountriesOAPEC),
            "ARL" => Ok(Self::ArabLeague),
            "ASL" => Ok(Self::SouthLebanonArmy),
            "ASN" => Ok(Self::AssociationofSoutheastAsianNationsASEAN),
            "ATD" => Ok(Self::EasternandSouthernAfricanTradeandDevelopmentBank),
            "BCA" => Ok(Self::BankofCentralAfricanStatesBEAC),
            "BIS" => Ok(Self::BankforInternationalSettlements),
            "BTH" => Ok(Self::BaathParty),
            "CEM" => Ok(Self::CommonMarketforEasternandSouthernAfrica),
            "CFA" => Ok(Self::FrancZoneFinancialCommunityofAfrica),
            "CIS" => Ok(Self::CommonwealthofIndependentStates),
            "CMN" => Ok(Self::Communist),
            "COE" => Ok(Self::CouncilofEurope),
            "CPA" => Ok(Self::CocoaProducersAlliance),
            "CPC" => Ok(Self::AssociationofCoffeeProducingCountries),
            "CRC" => Ok(Self::InternationalFedofRedCrossandRedCrescentICRC),
            "CSS" => Ok(Self::CommunityofSahelSaharanStatesCENSAD),
            "CWN" => Ok(Self::CommonwealthofNations),
            "DFL" => Ok(Self::DemocraticFrontfortheLibofPalestineDFLP),
            "EBR" => Ok(Self::EuropeanBankforReconstructionandDevelopment),
            "ECA" => Ok(Self::EconomicCommunityofCentralAfricanStates),
            "EEC" => Ok(Self::EuropeanUnion),
            "EFT" => Ok(Self::EuropeanFreeTradeAssociation),
            "ENN" => Ok(Self::EnnahdaMovement),
            "FAO" => Ok(Self::UnitedNationsFoodandAgricultureOrganization),
            "FID" => Ok(Self::InternationalFederationofHumanRightsFIDH),
            "FIS" => Ok(Self::IslamicSalvationArmy),
            "FLN" => Ok(Self::NationalLiberationFrontFLN),
            "FTA" => Ok(Self::Fatah),
            "GCC" => Ok(Self::GulfCooperationCouncil),
            "GIA" => Ok(Self::ArmedIslamicGroupGIA),
            "GOE" => Ok(Self::GroupofEightG8G7plusRussia),
            "GOS" => Ok(Self::GroupofSevenG7),
            "GSP" => Ok(Self::SalafistGroup),
            "GSS" => Ok(Self::GroupofSeventySevenG77),
            "HCH" => Ok(Self::UNHighCommissionforHumanRights),
            "HCR" => Ok(Self::UNHighCommissionforRefugees),
            "HEZ" => Ok(Self::Hezbullah),
            "HIP" => Ok(Self::HighlyIndebtedPoorCountriesHIPC),
            "HMS" => Ok(Self::Hamas),
            "HRW" => Ok(Self::HumanRightsWatch),
            "IAC" => Ok(Self::InterAfricanCoffeeOrganizationIACO),
            "IAD" => Ok(Self::IntergovernmentalAuthorityonDevelopmentIGAD),
            "IAE" => Ok(Self::InternationalAtomicEnergyAgencyIAEA),
            "IAF" => Ok(Self::IslamicActionFront),
            "ICC" => Ok(Self::InternationalCriminalCourt),
            "ICG" => Ok(Self::InternationalCrisisGroup),
            "ICJ" => Ok(Self::InternationalCourtofJusticeICJ),
            "ICO" => Ok(Self::InternationalCocoaOrganizationICCO),
            "IDB" => Ok(Self::IslamicDevelopmentBank),
            "IGC" => Ok(Self::InternationalGrainsCouncil),
            "IHF" => Ok(Self::InternationalHelsinkiFederationforHumanRights),
            "ILO" => Ok(Self::InternationalLaborOrganization),
            "IMF" => Ok(Self::InternationalMonetaryFundIMF),
            "IOM" => Ok(Self::InternationalOrganizationforMigration),
            "IPU" => Ok(Self::InterParliamentaryUnion),
            "IRC" => Ok(Self::RedCross),
            "ISJ" => Ok(Self::PalestinianIslamicJihad),
            "ITP" => Ok(Self::Interpol),
            "JUR" => Ok(Self::InternationalCommissionofJurists),
            "KDP" => Ok(Self::KurdishDemocraticPartyKDP),
            "KID" => Ok(Self::UnitedNationsChildrensFundUNICEF),
            "LBA" => Ok(Self::IsraeliLaborParty),
            "LKD" => Ok(Self::LikudParty),
            "MBR" => Ok(Self::MuslimBrotherhood),
            "MRZ" => Ok(Self::MeretzParty),
            "MSF" => Ok(Self::MedecinsSansFrontieresDoctorsWithoutBorders),
            "MSP" => Ok(Self::MovementoftheSocietyforPeace),
            "NAT" => Ok(Self::NorthAtlanticTreatyOrganizationNATO),
            "NEP" => Ok(Self::NewEconomicPartnershipforAfricasDevelopment),
            "NON" => Ok(Self::OrganizationofNonAlignedCountries),
            "OAS" => Ok(Self::OrganizationofAmericanStates),
            "OAU" => Ok(Self::OrganizationofAfricanUnityOAU),
            "OIC" => Ok(Self::OrganizationofIslamicConferencesOIC),
            "OPC" => Ok(Self::OrganizationofPetroleumExportingCountriesOPEC),
            "PAP" => Ok(Self::PanAfricanParliament),
            "PFL" => Ok(Self::PeoplesFrontfortheLiberationofPalestinePFLP),
            "PLF" => Ok(Self::PalestineLiberationFront),
            "PLO" => Ok(Self::PalestineLiberationOrganization),
            "PLS" => Ok(Self::PolisarioGuerillas),
            "PMD" => Ok(Self::PeoplesMujahedeen),
            "PRC" => Ok(Self::ParisClub),
            "PSE" => Ok(Self::OccupiedPalestinianTerritories),
            "RCR" => Ok(Self::RedCrescent),
            "RND" => Ok(Self::DemocraticNationalRally),
            "SAA" => Ok(Self::SouthAsianAssociation),
            "SAD" => Ok(Self::SouthernAfricanDevelopmentCommunity),
            "SCE" => Ok(Self::CouncilofSecurityandCooperationinEuropeOSCE),
            "SHA" => Ok(Self::ShasParty),
            "SOT" => Ok(Self::SoutheastAsiaCollectiveDefenseTreatySEATO),
            "TAL" => Ok(Self::Taliban),
            "UEM" => Ok(Self::EconomicandMonetaryUnionofWestAfricaUEMOA),
            "UNO" => Ok(Self::UnitedNations),
            "WAD" => Ok(Self::WestAfricaDevelopmentBank),
            "WAM" => Ok(Self::WestAfricaMonetaryandEconomicUnion),
            "WAS" => Ok(Self::EconomicCommunityofWestAfricanStatesECOWAS),
            "WBK" => Ok(Self::WorldBank),
            "WCT" => Ok(Self::InternationalWarCrimesTribunals),
            "WEF" => Ok(Self::WorldEconomicForum),
            "WFP" => Ok(Self::WorldFoodProgram),
            "WHO" => Ok(Self::WorldHealthOrganization),
            "WTO" => Ok(Self::WorldTradeOrganization),
            "WTO" => Ok(Self::WorldTradeOrganization),
            "XFM" => Ok(Self::Oxfam),
            _ => Err(anyhow!("Invalid Known Group Code")),
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
    fn test_cameo_known_group_code_try_from_valid_codes() {
        // init_logger();

        let valid_code_str = "AAM"; // AlAqsaMartyrsBrigade
        info!("Testing valid CAMEOKnownGroupCode: {:?}", valid_code_str);
        let code = CAMEOKnownGroupCode::try_from(Some(valid_code_str));
        assert!(code.is_ok());
        assert_eq!(code.unwrap().0, *b"AAM");
    }

    #[test]
    fn test_cameo_known_group_code_try_from_invalid_codes() {
        // init_logger();

        let invalid_code_str = "XXXX"; // Invalid code
        info!(
            "Testing invalid CAMEOKnownGroupCode: {:?}",
            invalid_code_str
        );
        let code = CAMEOKnownGroupCode::try_from(Some(invalid_code_str));
        assert!(code.is_err());
    }

    #[test]
    fn test_known_group_try_from_valid_codes() {
        // init_logger();

        let valid_code = Some(CAMEOKnownGroupCode(*b"AAM")); // AlAqsaMartyrsBrigade
        info!("Testing valid KnownGroup code: {:?}", valid_code);
        let known_group = KnownGroup::try_from(valid_code);
        assert!(known_group.is_ok());
        assert_eq!(known_group.unwrap(), KnownGroup::AlAqsaMartyrsBrigade);
    }

    #[test]
    fn test_known_group_try_from_invalid_codes() {
        // init_logger();

        let invalid_code = Some(CAMEOKnownGroupCode(*b"XXX")); // Invalid code
        info!("Testing invalid KnownGroup code: {:?}", invalid_code);
        let known_group = KnownGroup::try_from(invalid_code);
        assert!(known_group.is_err());
    }
}
