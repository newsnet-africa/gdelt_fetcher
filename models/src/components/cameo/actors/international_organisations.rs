use super::{primary_roles::PrimaryRole, religious_codes::Religion};

pub enum InternationalOrganisationCode {
    NotSpecified(LocationCode),
    InterGovernmentalOrganisation(LocationCode),
    InternationalMilitarisedGroup(LocationCode),
    MultiNAtionalCorporation(LocationCode),
    NonGovernmentalMovement(LocationCode),
    NonGovenmentalOrganisation(LocationCode),
    UnidentifiedStateActor(LocationCode),
}

pub enum LocationCode {
    NotSpecified(EthnicityCode),
    Africa(EthnicityCode),
    Asia(EthnicityCode),
    Balkans(EthnicityCode),
    Caribbean(EthnicityCode),
    Caucasus(EthnicityCode),
    CentralAfrica(EthnicityCode),
    CentralAsia(EthnicityCode),
    CentralEurope(EthnicityCode),
    EastIndies(EthnicityCode),
    EasternAfrica(EthnicityCode),
    EasternEurope(EthnicityCode),
    Europe(EthnicityCode),
    LatinAmerica(EthnicityCode),
    MiddleEast(EthnicityCode),
    Mediterranean(EthnicityCode),
    NorthAfrica(EthnicityCode),
    NorthAmerica(EthnicityCode),
    PersianGulf(EthnicityCode),
    Scandinavia(EthnicityCode),
    SouthAmerica(EthnicityCode),
    SouthAsia(EthnicityCode),
    SoutheastAsia(EthnicityCode),
    SouthernAfrica(EthnicityCode),
    WestAfrica(EthnicityCode),
    TheWest(EthnicityCode),
}

pub enum EthnicityCode {
    NotSpecified(Box<ReligionCode>),
    Ethnicity(Box<EthnicityCode>, Box<ReligionCode>),
}

pub enum ReligionCode {
    NotSpecified(Box<PrimaryRole>, Box<PrimaryRole>, Box<PrimaryRole>),
    ReligionCode(
        Box<Religion>,
        Box<PrimaryRole>,
        Box<PrimaryRole>,
        Box<PrimaryRole>,
    ),
}
