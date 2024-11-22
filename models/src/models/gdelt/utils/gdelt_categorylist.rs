#![feature(repr64)]

use crate::generated::gdelt_categorylist::GdeltCategoryList;
use crate::models::gdelt::{CellItem, GDELTObject, ToProto};
use std::cell::RefCell;
use std::fmt::Display;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// The `GDELTCategoryList` enum represents the list of categories that can be assigned to a GDELT event.
/// Each category is represented by a number, and the enum provides a conversion from `u16` to `GDELTCategoryList`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, EnumString, Display)]
#[repr(u64)]
pub enum GDELTCategoryList {
    UNKNOWN = 0,
    AFFECT = 1,
    AGRICULTURE = 2,
    ALLIANCE = 3,
    APPOINTMENT = 4,
    ARMEDCONFLICT = 5,
    ARREST = 6,
    ASSASSINATION = 7,
    AUSTERITY = 8,
    AVIATION_INCIDENT = 9,
    BAN = 10,
    BLACK_MARKET = 11,
    BLOCKADE = 12,
    BORDER = 13,
    BULLYING = 14,
    CEASEFIRE = 15,
    CHARASMATIC_LEADERSHIP = 16,
    CHECKPOINT = 17,
    CLAIM_CREDIT = 18,
    CLOSURE = 19,
    CONFISCATION = 20,
    CONSTITUTIONAL = 21,
    CORRUPTION = 22,
    CRIME_CARTELS = 23,
    CRIME_COMMON_ROBBERY = 24,
    CRIME_ILLEGAL_DRUGS = 25,
    CURFEW = 26,
    CYBER_ATTACK = 27,
    DEATH_PENALTY = 28,
    DEFECTION = 29,
    DELAY = 30,
    DEMOCRACY = 31,
    DISABILITY = 32,
    DISCRIMINATION = 33,
    DISPLACED = 34,
    DRONE = 35,
    DRUG_TRADE = 36,
    ECON_BANKRUPTCY = 37,
    ECON_BOYCOTT = 38,
    ECON_COST_OF_LIVING = 39,
    ECON_CURRENCY_EXCHANGE_RATE = 40,
    ECON_CURRENCY_RESERVES = 41,
    ECON_CUTOUTLOOK = 42,
    ECON_DEBT = 43,
    ECON_DEREGULATION = 44,
    ECON_EARNINGSREPORT = 45,
    ECON_ENTREPRENEURSHIP = 46,
    ECON_FOREIGNINVEST = 47,
    ECON_FREETRADE = 48,
    ECON_HOUSING_PRICES = 49,
    ECON_INFORMAL_ECONOMY = 50,
    ECON_INTEREST_RATES = 51,
    ECON_IPO = 52,
    ECON_MONOPOLY = 53,
    ECON_MOU = 54,
    ECON_NATIONALIZE = 55,
    ECON_PRICECONTROL = 56,
    ECON_REMITTANCE = 57,
    ECON_STOCKMARKET = 58,
    ECON_SUBSIDIES = 59,
    ECON_TAXATION = 60,
    ECON_TRADE_DISPUTE = 61,
    ECON_UNIONS = 62,
    EDUCATION = 63,
    ELECTION = 64,
    ELECTION_FRAUD = 65,
    ENV_BIOFUEL = 66,
    ENV_CARBONCAPTURE = 67,
    ENV_CLIMATECHANGE = 68,
    ENV_COAL = 69,
    ENV_DEFORESTATION = 70,
    ENV_FISHERY = 71,
    ENV_FORESTRY = 72,
    ENV_GEOTHERMAL = 73,
    ENV_GREEN = 74,
    ENV_HYDRO = 75,
    ENV_METALS = 76,
    ENV_MINING = 77,
    ENV_NATURALGAS = 78,
    ENV_NUCLEARPOWER = 79,
    ENV_OIL = 80,
    ENV_OVERFISH = 81,
    ENV_POACHING = 82,
    ENV_SOLAR = 83,
    ENV_SPECIESENDANGERED = 84,
    ENV_SPECIESEXTINCT = 85,
    ENV_WATERWAYS = 86,
    ENV_WINDPOWER = 87,
    ETH_INDIGINOUS = 88,
    EVACUATION = 89,
    EXHUMATION = 90,
    EXILE = 91,
    EXTREMISM = 92,
    FIREARM_OWNERSHIP = 93,
    FOOD_SECURITY = 94,
    FOOD_STAPLE = 95,
    FREESPEECH = 96,
    FUELPRICES = 97,
    GEN_HOLIDAY = 98,
    GENDER_VIOLENCE = 99,
    GENERAL_GOVERNMENT = 100,
    GENERAL_HEALTH = 101,
    GENTRIFICATION = 102,
    GOV_DISSOLVEGOV = 103,
    GOV_DIVISIONOFPOWER = 104,
    GOV_INTERGOVERNMENTAL = 105,
    GOV_REFORM = 106,
    GOV_REPATRIATION = 107,
    GRIEVANCES = 108,
    HARASSMENT = 109,
    HATE_SPEECH = 110,
    HEALTH_PANDEMIC = 111,
    HEALTH_SEXTRANSDISEASE = 112,
    HEALTH_VACCINATION = 113,
    HUMAN_TRAFFICKING = 114,
    IDEOLOGY = 115,
    IMMIGRATION = 116,
    IMPEACHMENT = 117,
    INFO_HOAX = 118,
    INFO_RUMOR = 119,
    INFRASTRUCTURE_BAD_ROADS = 120,
    INSURGENCY = 121,
    INTERNET_BLACKOUT = 122,
    INTERNET_CENSORSHIP = 123,
    JIHAD = 124,
    KIDNAP = 125,
    KILL = 126,
    LANDMINE = 127,
    LEADER = 128,
    LEGALIZE = 129,
    LEGISLATION = 130,
    LGBT = 131,
    LITERACY = 132,
    LOCUSTS = 133,
    MANMADE_DISASTER = 134,
    MANMADE_DISASTER_IMPLIED = 135,
    MARITIME = 136,
    MARITIME_INCIDENT = 137,
    MARITIME_INCIDENT_IMPLIED = 138,
    MARITIME_INCIDENT_SELF_IDENTIFIED = 139,
    MARITIME_PIRACY = 140,
    MEDIA_CENSORSHIP = 141,
    MEDIA_MSM = 142,
    MEDIA_SOCIAL = 143,
    MEDICAL = 144,
    MEDICAL_SECURITY = 145,
    MIL_SELF_IDENTIFIED_ARMS_DEAL = 146,
    MIL_WEAPONS_PROLIFERATION = 147,
    MILITARY = 148,
    MILITARY_COOPERATION = 149,
    MOVEMENT_ENVIRONMENTAL = 150,
    MOVEMENT_GENERAL = 151,
    MOVEMENT_OTHER = 152,
    MOVEMENT_SOCIAL = 153,
    MOVEMENT_WOMENS = 154,
    NATURAL_DISASTER = 155,
    NEGOTIATIONS = 156,
    NEW_CONSTRUCTION = 157,
    ORGANIZED_CRIME = 158,
    PEACEKEEPING = 159,
    PERSECUTION = 160,
    PHONE_OUTAGE = 161,
    PIPELINE_INCIDENT = 162,
    PIRACY = 163,
    POL_HOSTVISIT = 164,
    POLITICAL_PRISONER = 165,
    POLITICAL_TURMOIL = 166,
    POPULATION_DENSITY = 167,
    POVERTY = 168,
    POWER_OUTAGE = 169,
    PRIVATIZATION = 170,
    PROPAGANDA = 171,
    PROPERTY_RIGHTS = 172,
    PROTEST = 173,
    PUBLIC_TRANSPORT = 174,
    RAIL_INCIDENT = 175,
    RAPE = 176,
    RATIFY = 177,
    REBELLION = 178,
    REBELS = 179,
    RECRUITMENT = 180,
    REFUGEES = 181,
    REL_ANTISEMITISM = 182,
    RELATIONS = 183,
    RELEASE_HOSTAGE = 184,
    RELEASE_PRISON = 185,
    RELIGION = 186,
    RESIGNATION = 187,
    RETALIATE = 188,
    RETIREMENT = 189,
    RETIREMENTS = 190,
    ROAD_INCIDENT = 191,
    RURAL = 192,
    SANCTIONS = 193,
    SANITATION = 194,
    SCANDAL = 195,
    SCIENCE = 196,
    SECURITY_SERVICES = 197,
    SEIGE = 198,
    SEIZE = 199,
    SELF_IDENTIFIED_ATROCITY = 200,
    SELF_IDENTIFIED_ENVIRON_DISASTER = 201,
    SELF_IDENTIFIED_HUMAN_RIGHTS = 202,
    SELF_IDENTIFIED_HUMANITARIAN_CRISIS = 203,
    SEPARATISTS = 204,
    SHORTAGE = 205,
    SICKENED = 206,
    SLFID_CAPACITY_BUILDING = 207,
    SLFID_CIVIL_LIBERTIES = 208,
    SLFID_DICTATORSHIP = 209,
    SLFID_ECONOMIC_DEVELOPMENT = 210,
    SLFID_ECONOMIC_POWER = 211,
    SLFID_MILITARY_BUILDUP = 212,
    SLFID_MILITARY_READINESS = 213,
    SLFID_MILITARY_SPENDING = 214,
    SLFID_MINERAL_RESOURCES = 215,
    SLFID_NATURAL_RESOURCES = 216,
    SLFID_PEACE_BUILDING = 217,
    SLFID_POLITICAL_BOUNDARIES = 218,
    SLFID_RULE_OF_LAW = 219,
    SLUMS = 220,
    SMUGGLING = 221,
    SOC_DIPLOMCOOP = 222,
    SOC_ECONCOOP = 223,
    SOC_EXPRESSREGRET = 224,
    SOC_EXPRESSSUPPORT = 225,
    SOC_FORCEDRELOCATION = 226,
    SOC_GENERALCRIME = 227,
    SOC_INTELSHARING = 228,
    SOC_JUDICIALCOOP = 229,
    SOC_MASSMIGRATION = 230,
    SOC_PARDON = 231,
    SOC_SUICIDE = 232,
    SOC_SUSPICIOUSACTIVITIES = 233,
    SOC_SUSPICIOUSPEOPLE = 234,
    SOC_TRAFFICACCIDENT = 235,
    SOVEREIGNTY = 236,
    STATE_OF_EMERGENCY = 237,
    STRIKE = 238,
    SUICIDE_ATTACK = 239,
    SURVEILLANCE = 240,
    TAKE_OFFICE = 241,
    TAX_CARTELS = 242,
    TAX_DISEASE = 243,
    TAX_ETHNICITY = 244,
    TAX_FNCACT = 245,
    TAX_FOODSTAPLES = 246,
    TAX_MILITARY_TITLE = 247,
    TAX_POLITICAL_PARTY = 248,
    TAX_RELIGION = 249,
    TAX_SPECIAL_ISSUES = 250,
    TAX_SPECIALDEATH = 251,
    TAX_TERROR_GROUP = 252,
    TAX_WEAPONS = 253,
    TERROR = 254,
    TORTURE = 255,
    TOURISM = 256,
    TRAFFIC = 257,
    TRANSPARENCY = 258,
    TREASON = 259,
    TRIAL = 260,
    UNEMPLOYMENT = 261,
    UNGOVERNED = 262,
    UNREST_CHECKPOINT = 263,
    UNREST_CLOSINGBORDER = 264,
    UNREST_HUNGERSTRIKE = 265,
    UNREST_MOLOTOVCOCKTAIL = 266,
    UNREST_POLICEBRUTALITY = 267,
    UNREST_STONETHROWING = 268,
    UNREST_STONING = 269,
    UNSAFE_WORK_ENVIRONMENT = 270,
    URBAN = 271,
    URBAN_SPRAWL = 272,
    VANDALIZE = 273,
    VETO = 274,
    VIOLENT_UNREST = 275,
    WATER_SECURITY = 276,
    WHISTLEBLOWER = 277,
    WMD = 278,
    WOUND = 279,
}

/// Implement the conversion from `u16` to `GDELTCategoryList`.
impl From<u16> for GDELTCategoryList {
    fn from(value: u16) -> Self {
        match value {
            0 => GDELTCategoryList::UNKNOWN,
            1 => GDELTCategoryList::AFFECT,
            2 => GDELTCategoryList::AGRICULTURE,
            3 => GDELTCategoryList::ALLIANCE,
            4 => GDELTCategoryList::APPOINTMENT,
            5 => GDELTCategoryList::ARMEDCONFLICT,
            6 => GDELTCategoryList::ARREST,
            7 => GDELTCategoryList::ASSASSINATION,
            8 => GDELTCategoryList::AUSTERITY,
            9 => GDELTCategoryList::AVIATION_INCIDENT,
            10 => GDELTCategoryList::BAN,
            11 => GDELTCategoryList::BLACK_MARKET,
            12 => GDELTCategoryList::BLOCKADE,
            13 => GDELTCategoryList::BORDER,
            14 => GDELTCategoryList::BULLYING,
            15 => GDELTCategoryList::CEASEFIRE,
            16 => GDELTCategoryList::CHARASMATIC_LEADERSHIP,
            17 => GDELTCategoryList::CHECKPOINT,
            18 => GDELTCategoryList::CLAIM_CREDIT,
            19 => GDELTCategoryList::CLOSURE,
            20 => GDELTCategoryList::CONFISCATION,
            21 => GDELTCategoryList::CONSTITUTIONAL,
            22 => GDELTCategoryList::CORRUPTION,
            23 => GDELTCategoryList::CRIME_CARTELS,
            24 => GDELTCategoryList::CRIME_COMMON_ROBBERY,
            25 => GDELTCategoryList::CRIME_ILLEGAL_DRUGS,
            26 => GDELTCategoryList::CURFEW,
            27 => GDELTCategoryList::CYBER_ATTACK,
            28 => GDELTCategoryList::DEATH_PENALTY,
            29 => GDELTCategoryList::DEFECTION,
            30 => GDELTCategoryList::DELAY,
            31 => GDELTCategoryList::DEMOCRACY,
            32 => GDELTCategoryList::DISABILITY,
            33 => GDELTCategoryList::DISCRIMINATION,
            34 => GDELTCategoryList::DISPLACED,
            35 => GDELTCategoryList::DRONE,
            36 => GDELTCategoryList::DRUG_TRADE,
            37 => GDELTCategoryList::ECON_BANKRUPTCY,
            38 => GDELTCategoryList::ECON_BOYCOTT,
            39 => GDELTCategoryList::ECON_COST_OF_LIVING,
            40 => GDELTCategoryList::ECON_CURRENCY_EXCHANGE_RATE,
            41 => GDELTCategoryList::ECON_CURRENCY_RESERVES,
            42 => GDELTCategoryList::ECON_CUTOUTLOOK,
            43 => GDELTCategoryList::ECON_DEBT,
            44 => GDELTCategoryList::ECON_DEREGULATION,
            45 => GDELTCategoryList::ECON_EARNINGSREPORT,
            46 => GDELTCategoryList::ECON_ENTREPRENEURSHIP,
            47 => GDELTCategoryList::ECON_FOREIGNINVEST,
            48 => GDELTCategoryList::ECON_FREETRADE,
            49 => GDELTCategoryList::ECON_HOUSING_PRICES,
            50 => GDELTCategoryList::ECON_INFORMAL_ECONOMY,
            51 => GDELTCategoryList::ECON_INTEREST_RATES,
            52 => GDELTCategoryList::ECON_IPO,
            53 => GDELTCategoryList::ECON_MONOPOLY,
            54 => GDELTCategoryList::ECON_MOU,
            55 => GDELTCategoryList::ECON_NATIONALIZE,
            56 => GDELTCategoryList::ECON_PRICECONTROL,
            57 => GDELTCategoryList::ECON_REMITTANCE,
            58 => GDELTCategoryList::ECON_STOCKMARKET,
            59 => GDELTCategoryList::ECON_SUBSIDIES,
            60 => GDELTCategoryList::ECON_TAXATION,
            61 => GDELTCategoryList::ECON_TRADE_DISPUTE,
            62 => GDELTCategoryList::ECON_UNIONS,
            63 => GDELTCategoryList::EDUCATION,
            64 => GDELTCategoryList::ELECTION,
            65 => GDELTCategoryList::ELECTION_FRAUD,
            66 => GDELTCategoryList::ENV_BIOFUEL,
            67 => GDELTCategoryList::ENV_CARBONCAPTURE,
            68 => GDELTCategoryList::ENV_CLIMATECHANGE,
            69 => GDELTCategoryList::ENV_COAL,
            70 => GDELTCategoryList::ENV_DEFORESTATION,
            71 => GDELTCategoryList::ENV_FISHERY,
            72 => GDELTCategoryList::ENV_FORESTRY,
            73 => GDELTCategoryList::ENV_GEOTHERMAL,
            74 => GDELTCategoryList::ENV_GREEN,
            75 => GDELTCategoryList::ENV_HYDRO,
            76 => GDELTCategoryList::ENV_METALS,
            77 => GDELTCategoryList::ENV_MINING,
            78 => GDELTCategoryList::ENV_NATURALGAS,
            79 => GDELTCategoryList::ENV_NUCLEARPOWER,
            80 => GDELTCategoryList::ENV_OIL,
            81 => GDELTCategoryList::ENV_OVERFISH,
            82 => GDELTCategoryList::ENV_POACHING,
            83 => GDELTCategoryList::ENV_SOLAR,
            84 => GDELTCategoryList::ENV_SPECIESENDANGERED,
            85 => GDELTCategoryList::ENV_SPECIESEXTINCT,
            86 => GDELTCategoryList::ENV_WATERWAYS,
            87 => GDELTCategoryList::ENV_WINDPOWER,
            88 => GDELTCategoryList::ETH_INDIGINOUS,
            89 => GDELTCategoryList::EVACUATION,
            90 => GDELTCategoryList::EXHUMATION,
            91 => GDELTCategoryList::EXILE,
            92 => GDELTCategoryList::EXTREMISM,
            93 => GDELTCategoryList::FIREARM_OWNERSHIP,
            94 => GDELTCategoryList::FOOD_SECURITY,
            95 => GDELTCategoryList::FOOD_STAPLE,
            96 => GDELTCategoryList::FREESPEECH,
            97 => GDELTCategoryList::FUELPRICES,
            98 => GDELTCategoryList::GEN_HOLIDAY,
            99 => GDELTCategoryList::GENDER_VIOLENCE,
            100 => GDELTCategoryList::GENERAL_GOVERNMENT,
            101 => GDELTCategoryList::GENERAL_HEALTH,
            102 => GDELTCategoryList::GENTRIFICATION,
            103 => GDELTCategoryList::GOV_DISSOLVEGOV,
            104 => GDELTCategoryList::GOV_DIVISIONOFPOWER,
            105 => GDELTCategoryList::GOV_INTERGOVERNMENTAL,
            106 => GDELTCategoryList::GOV_REFORM,
            107 => GDELTCategoryList::GOV_REPATRIATION,
            108 => GDELTCategoryList::GRIEVANCES,
            109 => GDELTCategoryList::HARASSMENT,
            110 => GDELTCategoryList::HATE_SPEECH,
            111 => GDELTCategoryList::HEALTH_PANDEMIC,
            112 => GDELTCategoryList::HEALTH_SEXTRANSDISEASE,
            113 => GDELTCategoryList::HEALTH_VACCINATION,
            114 => GDELTCategoryList::HUMAN_TRAFFICKING,
            115 => GDELTCategoryList::IDEOLOGY,
            116 => GDELTCategoryList::IMMIGRATION,
            117 => GDELTCategoryList::IMPEACHMENT,
            118 => GDELTCategoryList::INFO_HOAX,
            119 => GDELTCategoryList::INFO_RUMOR,
            120 => GDELTCategoryList::INFRASTRUCTURE_BAD_ROADS,
            121 => GDELTCategoryList::INSURGENCY,
            122 => GDELTCategoryList::INTERNET_BLACKOUT,
            123 => GDELTCategoryList::INTERNET_CENSORSHIP,
            124 => GDELTCategoryList::JIHAD,
            125 => GDELTCategoryList::KIDNAP,
            126 => GDELTCategoryList::KILL,
            127 => GDELTCategoryList::LANDMINE,
            128 => GDELTCategoryList::LEADER,
            129 => GDELTCategoryList::LEGALIZE,
            130 => GDELTCategoryList::LEGISLATION,
            131 => GDELTCategoryList::LGBT,
            132 => GDELTCategoryList::LITERACY,
            133 => GDELTCategoryList::LOCUSTS,
            134 => GDELTCategoryList::MANMADE_DISASTER,
            135 => GDELTCategoryList::MANMADE_DISASTER_IMPLIED,
            136 => GDELTCategoryList::MARITIME,
            137 => GDELTCategoryList::MARITIME_INCIDENT,
            138 => GDELTCategoryList::MARITIME_INCIDENT_IMPLIED,
            139 => GDELTCategoryList::MARITIME_INCIDENT_SELF_IDENTIFIED,
            140 => GDELTCategoryList::MARITIME_PIRACY,
            141 => GDELTCategoryList::MEDIA_CENSORSHIP,
            142 => GDELTCategoryList::MEDIA_MSM,
            143 => GDELTCategoryList::MEDIA_SOCIAL,
            144 => GDELTCategoryList::MEDICAL,
            145 => GDELTCategoryList::MEDICAL_SECURITY,
            146 => GDELTCategoryList::MIL_SELF_IDENTIFIED_ARMS_DEAL,
            147 => GDELTCategoryList::MIL_WEAPONS_PROLIFERATION,
            148 => GDELTCategoryList::MILITARY,
            149 => GDELTCategoryList::MILITARY_COOPERATION,
            150 => GDELTCategoryList::MOVEMENT_ENVIRONMENTAL,
            151 => GDELTCategoryList::MOVEMENT_GENERAL,
            152 => GDELTCategoryList::MOVEMENT_OTHER,
            153 => GDELTCategoryList::MOVEMENT_SOCIAL,
            154 => GDELTCategoryList::MOVEMENT_WOMENS,
            155 => GDELTCategoryList::NATURAL_DISASTER,
            156 => GDELTCategoryList::NEGOTIATIONS,
            157 => GDELTCategoryList::NEW_CONSTRUCTION,
            158 => GDELTCategoryList::ORGANIZED_CRIME,
            159 => GDELTCategoryList::PEACEKEEPING,
            160 => GDELTCategoryList::PERSECUTION,
            161 => GDELTCategoryList::PHONE_OUTAGE,
            162 => GDELTCategoryList::PIPELINE_INCIDENT,
            163 => GDELTCategoryList::PIRACY,
            164 => GDELTCategoryList::POL_HOSTVISIT,
            165 => GDELTCategoryList::POLITICAL_PRISONER,
            166 => GDELTCategoryList::POLITICAL_TURMOIL,
            167 => GDELTCategoryList::POPULATION_DENSITY,
            168 => GDELTCategoryList::POVERTY,
            169 => GDELTCategoryList::POWER_OUTAGE,
            170 => GDELTCategoryList::PRIVATIZATION,
            171 => GDELTCategoryList::PROPAGANDA,
            172 => GDELTCategoryList::PROPERTY_RIGHTS,
            173 => GDELTCategoryList::PROTEST,
            174 => GDELTCategoryList::PUBLIC_TRANSPORT,
            175 => GDELTCategoryList::RAIL_INCIDENT,
            176 => GDELTCategoryList::RAPE,
            177 => GDELTCategoryList::RATIFY,
            178 => GDELTCategoryList::REBELLION,
            179 => GDELTCategoryList::REBELS,
            180 => GDELTCategoryList::RECRUITMENT,
            181 => GDELTCategoryList::REFUGEES,
            182 => GDELTCategoryList::REL_ANTISEMITISM,
            183 => GDELTCategoryList::RELATIONS,
            184 => GDELTCategoryList::RELEASE_HOSTAGE,
            185 => GDELTCategoryList::RELEASE_PRISON,
            186 => GDELTCategoryList::RELIGION,
            187 => GDELTCategoryList::RESIGNATION,
            188 => GDELTCategoryList::RETALIATE,
            189 => GDELTCategoryList::RETIREMENT,
            190 => GDELTCategoryList::RETIREMENTS,
            191 => GDELTCategoryList::ROAD_INCIDENT,
            192 => GDELTCategoryList::RURAL,
            193 => GDELTCategoryList::SANCTIONS,
            194 => GDELTCategoryList::SANITATION,
            195 => GDELTCategoryList::SCANDAL,
            196 => GDELTCategoryList::SCIENCE,
            197 => GDELTCategoryList::SECURITY_SERVICES,
            198 => GDELTCategoryList::SEIGE,
            199 => GDELTCategoryList::SEIZE,
            200 => GDELTCategoryList::SELF_IDENTIFIED_ATROCITY,
            201 => GDELTCategoryList::SELF_IDENTIFIED_ENVIRON_DISASTER,
            202 => GDELTCategoryList::SELF_IDENTIFIED_HUMAN_RIGHTS,
            203 => GDELTCategoryList::SELF_IDENTIFIED_HUMANITARIAN_CRISIS,
            204 => GDELTCategoryList::SEPARATISTS,
            205 => GDELTCategoryList::SHORTAGE,
            206 => GDELTCategoryList::SICKENED,
            207 => GDELTCategoryList::SLFID_CAPACITY_BUILDING,
            208 => GDELTCategoryList::SLFID_CIVIL_LIBERTIES,
            209 => GDELTCategoryList::SLFID_DICTATORSHIP,
            210 => GDELTCategoryList::SLFID_ECONOMIC_DEVELOPMENT,
            211 => GDELTCategoryList::SLFID_ECONOMIC_POWER,
            212 => GDELTCategoryList::SLFID_MILITARY_BUILDUP,
            213 => GDELTCategoryList::SLFID_MILITARY_READINESS,
            214 => GDELTCategoryList::SLFID_MILITARY_SPENDING,
            215 => GDELTCategoryList::SLFID_MINERAL_RESOURCES,
            216 => GDELTCategoryList::SLFID_NATURAL_RESOURCES,
            217 => GDELTCategoryList::SLFID_PEACE_BUILDING,
            218 => GDELTCategoryList::SLFID_POLITICAL_BOUNDARIES,
            219 => GDELTCategoryList::SLFID_RULE_OF_LAW,
            220 => GDELTCategoryList::SLUMS,
            221 => GDELTCategoryList::SMUGGLING,
            222 => GDELTCategoryList::SOC_DIPLOMCOOP,
            223 => GDELTCategoryList::SOC_ECONCOOP,
            224 => GDELTCategoryList::SOC_EXPRESSREGRET,
            225 => GDELTCategoryList::SOC_EXPRESSSUPPORT,
            226 => GDELTCategoryList::SOC_FORCEDRELOCATION,
            227 => GDELTCategoryList::SOC_GENERALCRIME,
            228 => GDELTCategoryList::SOC_INTELSHARING,
            229 => GDELTCategoryList::SOC_JUDICIALCOOP,
            230 => GDELTCategoryList::SOC_MASSMIGRATION,
            231 => GDELTCategoryList::SOC_PARDON,
            232 => GDELTCategoryList::SOC_SUICIDE,
            233 => GDELTCategoryList::SOC_SUSPICIOUSACTIVITIES,
            234 => GDELTCategoryList::SOC_SUSPICIOUSPEOPLE,
            235 => GDELTCategoryList::SOC_TRAFFICACCIDENT,
            236 => GDELTCategoryList::SOVEREIGNTY,
            237 => GDELTCategoryList::STATE_OF_EMERGENCY,
            238 => GDELTCategoryList::STRIKE,
            239 => GDELTCategoryList::SUICIDE_ATTACK,
            240 => GDELTCategoryList::SURVEILLANCE,
            241 => GDELTCategoryList::TAKE_OFFICE,
            242 => GDELTCategoryList::TAX_CARTELS,
            243 => GDELTCategoryList::TAX_DISEASE,
            244 => GDELTCategoryList::TAX_ETHNICITY,
            245 => GDELTCategoryList::TAX_FNCACT,
            246 => GDELTCategoryList::TAX_FOODSTAPLES,
            247 => GDELTCategoryList::TAX_MILITARY_TITLE,
            248 => GDELTCategoryList::TAX_POLITICAL_PARTY,
            249 => GDELTCategoryList::TAX_RELIGION,
            250 => GDELTCategoryList::TAX_SPECIAL_ISSUES,
            251 => GDELTCategoryList::TAX_SPECIALDEATH,
            252 => GDELTCategoryList::TAX_TERROR_GROUP,
            253 => GDELTCategoryList::TAX_WEAPONS,
            254 => GDELTCategoryList::TERROR,
            255 => GDELTCategoryList::TORTURE,
            256 => GDELTCategoryList::TOURISM,
            257 => GDELTCategoryList::TRAFFIC,
            258 => GDELTCategoryList::TRANSPARENCY,
            259 => GDELTCategoryList::TREASON,
            260 => GDELTCategoryList::TRIAL,
            261 => GDELTCategoryList::UNEMPLOYMENT,
            262 => GDELTCategoryList::UNGOVERNED,
            263 => GDELTCategoryList::UNREST_CHECKPOINT,
            264 => GDELTCategoryList::UNREST_CLOSINGBORDER,
            265 => GDELTCategoryList::UNREST_HUNGERSTRIKE,
            266 => GDELTCategoryList::UNREST_MOLOTOVCOCKTAIL,
            267 => GDELTCategoryList::UNREST_POLICEBRUTALITY,
            268 => GDELTCategoryList::UNREST_STONETHROWING,
            269 => GDELTCategoryList::UNREST_STONING,
            270 => GDELTCategoryList::UNSAFE_WORK_ENVIRONMENT,
            271 => GDELTCategoryList::URBAN,
            272 => GDELTCategoryList::URBAN_SPRAWL,
            273 => GDELTCategoryList::VANDALIZE,
            274 => GDELTCategoryList::VETO,
            275 => GDELTCategoryList::VIOLENT_UNREST,
            276 => GDELTCategoryList::WATER_SECURITY,
            277 => GDELTCategoryList::WHISTLEBLOWER,
            278 => GDELTCategoryList::WMD,
            279 => GDELTCategoryList::WOUND,
            _ => GDELTCategoryList::UNKNOWN, // Default case for unknown values
        }
    }
}

/// Implement the conversion from `GDELTCategoryList` to `u16`.
impl From<GDELTCategoryList> for u16 {
    fn from(value: GDELTCategoryList) -> u16 {
        match value {
            GDELTCategoryList::UNKNOWN => 0,
            GDELTCategoryList::AFFECT => 1,
            GDELTCategoryList::AGRICULTURE => 2,
            GDELTCategoryList::ALLIANCE => 3,
            GDELTCategoryList::APPOINTMENT => 4,
            GDELTCategoryList::ARMEDCONFLICT => 5,
            GDELTCategoryList::ARREST => 6,
            GDELTCategoryList::ASSASSINATION => 7,
            GDELTCategoryList::AUSTERITY => 8,
            GDELTCategoryList::AVIATION_INCIDENT => 9,
            GDELTCategoryList::BAN => 10,
            GDELTCategoryList::BLACK_MARKET => 11,
            GDELTCategoryList::BLOCKADE => 12,
            GDELTCategoryList::BORDER => 13,
            GDELTCategoryList::BULLYING => 14,
            GDELTCategoryList::CEASEFIRE => 15,
            GDELTCategoryList::CHARASMATIC_LEADERSHIP => 16,
            GDELTCategoryList::CHECKPOINT => 17,
            GDELTCategoryList::CLAIM_CREDIT => 18,
            GDELTCategoryList::CLOSURE => 19,
            GDELTCategoryList::CONFISCATION => 20,
            GDELTCategoryList::CONSTITUTIONAL => 21,
            GDELTCategoryList::CORRUPTION => 22,
            GDELTCategoryList::CRIME_CARTELS => 23,
            GDELTCategoryList::CRIME_COMMON_ROBBERY => 24,
            GDELTCategoryList::CRIME_ILLEGAL_DRUGS => 25,
            GDELTCategoryList::CURFEW => 26,
            GDELTCategoryList::CYBER_ATTACK => 27,
            GDELTCategoryList::DEATH_PENALTY => 28,
            GDELTCategoryList::DEFECTION => 29,
            GDELTCategoryList::DELAY => 30,
            GDELTCategoryList::DEMOCRACY => 31,
            GDELTCategoryList::DISABILITY => 32,
            GDELTCategoryList::DISCRIMINATION => 33,
            GDELTCategoryList::DISPLACED => 34,
            GDELTCategoryList::DRONE => 35,
            GDELTCategoryList::DRUG_TRADE => 36,
            GDELTCategoryList::ECON_BANKRUPTCY => 37,
            GDELTCategoryList::ECON_BOYCOTT => 38,
            GDELTCategoryList::ECON_COST_OF_LIVING => 39,
            GDELTCategoryList::ECON_CURRENCY_EXCHANGE_RATE => 40,
            GDELTCategoryList::ECON_CURRENCY_RESERVES => 41,
            GDELTCategoryList::ECON_CUTOUTLOOK => 42,
            GDELTCategoryList::ECON_DEBT => 43,
            GDELTCategoryList::ECON_DEREGULATION => 44,
            GDELTCategoryList::ECON_EARNINGSREPORT => 45,
            GDELTCategoryList::ECON_ENTREPRENEURSHIP => 46,
            GDELTCategoryList::ECON_FOREIGNINVEST => 47,
            GDELTCategoryList::ECON_FREETRADE => 48,
            GDELTCategoryList::ECON_HOUSING_PRICES => 49,
            GDELTCategoryList::ECON_INFORMAL_ECONOMY => 50,
            GDELTCategoryList::ECON_INTEREST_RATES => 51,
            GDELTCategoryList::ECON_IPO => 52,
            GDELTCategoryList::ECON_MONOPOLY => 53,
            GDELTCategoryList::ECON_MOU => 54,
            GDELTCategoryList::ECON_NATIONALIZE => 55,
            GDELTCategoryList::ECON_PRICECONTROL => 56,
            GDELTCategoryList::ECON_REMITTANCE => 57,
            GDELTCategoryList::ECON_STOCKMARKET => 58,
            GDELTCategoryList::ECON_SUBSIDIES => 59,
            GDELTCategoryList::ECON_TAXATION => 60,
            GDELTCategoryList::ECON_TRADE_DISPUTE => 61,
            GDELTCategoryList::ECON_UNIONS => 62,
            GDELTCategoryList::EDUCATION => 63,
            GDELTCategoryList::ELECTION => 64,
            GDELTCategoryList::ELECTION_FRAUD => 65,
            GDELTCategoryList::ENV_BIOFUEL => 66,
            GDELTCategoryList::ENV_CARBONCAPTURE => 67,
            GDELTCategoryList::ENV_CLIMATECHANGE => 68,
            GDELTCategoryList::ENV_COAL => 69,
            GDELTCategoryList::ENV_DEFORESTATION => 70,
            GDELTCategoryList::ENV_FISHERY => 71,
            GDELTCategoryList::ENV_FORESTRY => 72,
            GDELTCategoryList::ENV_GEOTHERMAL => 73,
            GDELTCategoryList::ENV_GREEN => 74,
            GDELTCategoryList::ENV_HYDRO => 75,
            GDELTCategoryList::ENV_METALS => 76,
            GDELTCategoryList::ENV_MINING => 77,
            GDELTCategoryList::ENV_NATURALGAS => 78,
            GDELTCategoryList::ENV_NUCLEARPOWER => 79,
            GDELTCategoryList::ENV_OIL => 80,
            GDELTCategoryList::ENV_OVERFISH => 81,
            GDELTCategoryList::ENV_POACHING => 82,
            GDELTCategoryList::ENV_SOLAR => 83,
            GDELTCategoryList::ENV_SPECIESENDANGERED => 84,
            GDELTCategoryList::ENV_SPECIESEXTINCT => 85,
            GDELTCategoryList::ENV_WATERWAYS => 86,
            GDELTCategoryList::ENV_WINDPOWER => 87,
            GDELTCategoryList::ETH_INDIGINOUS => 88,
            GDELTCategoryList::EVACUATION => 89,
            GDELTCategoryList::EXHUMATION => 90,
            GDELTCategoryList::EXILE => 91,
            GDELTCategoryList::EXTREMISM => 92,
            GDELTCategoryList::FIREARM_OWNERSHIP => 93,
            GDELTCategoryList::FOOD_SECURITY => 94,
            GDELTCategoryList::FOOD_STAPLE => 95,
            GDELTCategoryList::FREESPEECH => 96,
            GDELTCategoryList::FUELPRICES => 97,
            GDELTCategoryList::GEN_HOLIDAY => 98,
            GDELTCategoryList::GENDER_VIOLENCE => 99,
            GDELTCategoryList::GENERAL_GOVERNMENT => 100,
            GDELTCategoryList::GENERAL_HEALTH => 101,
            GDELTCategoryList::GENTRIFICATION => 102,
            GDELTCategoryList::GOV_DISSOLVEGOV => 103,
            GDELTCategoryList::GOV_DIVISIONOFPOWER => 104,
            GDELTCategoryList::GOV_INTERGOVERNMENTAL => 105,
            GDELTCategoryList::GOV_REFORM => 106,
            GDELTCategoryList::GOV_REPATRIATION => 107,
            GDELTCategoryList::GRIEVANCES => 108,
            GDELTCategoryList::HARASSMENT => 109,
            GDELTCategoryList::HATE_SPEECH => 110,
            GDELTCategoryList::HEALTH_PANDEMIC => 111,
            GDELTCategoryList::HEALTH_SEXTRANSDISEASE => 112,
            GDELTCategoryList::HEALTH_VACCINATION => 113,
            GDELTCategoryList::HUMAN_TRAFFICKING => 114,
            GDELTCategoryList::IDEOLOGY => 115,
            GDELTCategoryList::IMMIGRATION => 116,
            GDELTCategoryList::IMPEACHMENT => 117,
            GDELTCategoryList::INFO_HOAX => 118,
            GDELTCategoryList::INFO_RUMOR => 119,
            GDELTCategoryList::INFRASTRUCTURE_BAD_ROADS => 120,
            GDELTCategoryList::INSURGENCY => 121,
            GDELTCategoryList::INTERNET_BLACKOUT => 122,
            GDELTCategoryList::INTERNET_CENSORSHIP => 123,
            GDELTCategoryList::JIHAD => 124,
            GDELTCategoryList::KIDNAP => 125,
            GDELTCategoryList::KILL => 126,
            GDELTCategoryList::LANDMINE => 127,
            GDELTCategoryList::LEADER => 128,
            GDELTCategoryList::LEGALIZE => 129,
            GDELTCategoryList::LEGISLATION => 130,
            GDELTCategoryList::LGBT => 131,
            GDELTCategoryList::LITERACY => 132,
            GDELTCategoryList::LOCUSTS => 133,
            GDELTCategoryList::MANMADE_DISASTER => 134,
            GDELTCategoryList::MANMADE_DISASTER_IMPLIED => 135,
            GDELTCategoryList::MARITIME => 136,
            GDELTCategoryList::MARITIME_INCIDENT => 137,
            GDELTCategoryList::MARITIME_INCIDENT_IMPLIED => 138,
            GDELTCategoryList::MARITIME_INCIDENT_SELF_IDENTIFIED => 139,
            GDELTCategoryList::MARITIME_PIRACY => 140,
            GDELTCategoryList::MEDIA_CENSORSHIP => 141,
            GDELTCategoryList::MEDIA_MSM => 142,
            GDELTCategoryList::MEDIA_SOCIAL => 143,
            GDELTCategoryList::MEDICAL => 144,
            GDELTCategoryList::MEDICAL_SECURITY => 145,
            GDELTCategoryList::MIL_SELF_IDENTIFIED_ARMS_DEAL => 146,
            GDELTCategoryList::MIL_WEAPONS_PROLIFERATION => 147,
            GDELTCategoryList::MILITARY => 148,
            GDELTCategoryList::MILITARY_COOPERATION => 149,
            GDELTCategoryList::MOVEMENT_ENVIRONMENTAL => 150,
            GDELTCategoryList::MOVEMENT_GENERAL => 151,
            GDELTCategoryList::MOVEMENT_OTHER => 152,
            GDELTCategoryList::MOVEMENT_SOCIAL => 153,
            GDELTCategoryList::MOVEMENT_WOMENS => 154,
            GDELTCategoryList::NATURAL_DISASTER => 155,
            GDELTCategoryList::NEGOTIATIONS => 156,
            GDELTCategoryList::NEW_CONSTRUCTION => 157,
            GDELTCategoryList::ORGANIZED_CRIME => 158,
            GDELTCategoryList::PEACEKEEPING => 159,
            GDELTCategoryList::PERSECUTION => 160,
            GDELTCategoryList::PHONE_OUTAGE => 161,
            GDELTCategoryList::PIPELINE_INCIDENT => 162,
            GDELTCategoryList::PIRACY => 163,
            GDELTCategoryList::POL_HOSTVISIT => 164,
            GDELTCategoryList::POLITICAL_PRISONER => 165,
            GDELTCategoryList::POLITICAL_TURMOIL => 166,
            GDELTCategoryList::POPULATION_DENSITY => 167,
            GDELTCategoryList::POVERTY => 168,
            GDELTCategoryList::POWER_OUTAGE => 169,
            GDELTCategoryList::PRIVATIZATION => 170,
            GDELTCategoryList::PROPAGANDA => 171,
            GDELTCategoryList::PROPERTY_RIGHTS => 172,
            GDELTCategoryList::PROTEST => 173,
            GDELTCategoryList::PUBLIC_TRANSPORT => 174,
            GDELTCategoryList::RAIL_INCIDENT => 175,
            GDELTCategoryList::RAPE => 176,
            GDELTCategoryList::RATIFY => 177,
            GDELTCategoryList::REBELLION => 178,
            GDELTCategoryList::REBELS => 179,
            GDELTCategoryList::RECRUITMENT => 180,
            GDELTCategoryList::REFUGEES => 181,
            GDELTCategoryList::REL_ANTISEMITISM => 182,
            GDELTCategoryList::RELATIONS => 183,
            GDELTCategoryList::RELEASE_HOSTAGE => 184,
            GDELTCategoryList::RELEASE_PRISON => 185,
            GDELTCategoryList::RELIGION => 186,
            GDELTCategoryList::RESIGNATION => 187,
            GDELTCategoryList::RETALIATE => 188,
            GDELTCategoryList::RETIREMENT => 189,
            GDELTCategoryList::RETIREMENTS => 190,
            GDELTCategoryList::ROAD_INCIDENT => 191,
            GDELTCategoryList::RURAL => 192,
            GDELTCategoryList::SANCTIONS => 193,
            GDELTCategoryList::SANITATION => 194,
            GDELTCategoryList::SCANDAL => 195,
            GDELTCategoryList::SCIENCE => 196,
            GDELTCategoryList::SECURITY_SERVICES => 197,
            GDELTCategoryList::SEIGE => 198,
            GDELTCategoryList::SEIZE => 199,
            GDELTCategoryList::SELF_IDENTIFIED_ATROCITY => 200,
            GDELTCategoryList::SELF_IDENTIFIED_ENVIRON_DISASTER => 201,
            GDELTCategoryList::SELF_IDENTIFIED_HUMAN_RIGHTS => 202,
            GDELTCategoryList::SELF_IDENTIFIED_HUMANITARIAN_CRISIS => 203,
            GDELTCategoryList::SEPARATISTS => 204,
            GDELTCategoryList::SHORTAGE => 205,
            GDELTCategoryList::SICKENED => 206,
            GDELTCategoryList::SLFID_CAPACITY_BUILDING => 207,
            GDELTCategoryList::SLFID_CIVIL_LIBERTIES => 208,
            GDELTCategoryList::SLFID_DICTATORSHIP => 209,
            GDELTCategoryList::SLFID_ECONOMIC_DEVELOPMENT => 210,
            GDELTCategoryList::SLFID_ECONOMIC_POWER => 211,
            GDELTCategoryList::SLFID_MILITARY_BUILDUP => 212,
            GDELTCategoryList::SLFID_MILITARY_READINESS => 213,
            GDELTCategoryList::SLFID_MILITARY_SPENDING => 214,
            GDELTCategoryList::SLFID_MINERAL_RESOURCES => 215,
            GDELTCategoryList::SLFID_NATURAL_RESOURCES => 216,
            GDELTCategoryList::SLFID_PEACE_BUILDING => 217,
            GDELTCategoryList::SLFID_POLITICAL_BOUNDARIES => 218,
            GDELTCategoryList::SLFID_RULE_OF_LAW => 219,
            GDELTCategoryList::SLUMS => 220,
            GDELTCategoryList::SMUGGLING => 221,
            GDELTCategoryList::SOC_DIPLOMCOOP => 222,
            GDELTCategoryList::SOC_ECONCOOP => 223,
            GDELTCategoryList::SOC_EXPRESSREGRET => 224,
            GDELTCategoryList::SOC_EXPRESSSUPPORT => 225,
            GDELTCategoryList::SOC_FORCEDRELOCATION => 226,
            GDELTCategoryList::SOC_GENERALCRIME => 227,
            GDELTCategoryList::SOC_INTELSHARING => 228,
            GDELTCategoryList::SOC_JUDICIALCOOP => 229,
            GDELTCategoryList::SOC_MASSMIGRATION => 230,
            GDELTCategoryList::SOC_PARDON => 231,
            GDELTCategoryList::SOC_SUICIDE => 232,
            GDELTCategoryList::SOC_SUSPICIOUSACTIVITIES => 233,
            GDELTCategoryList::SOC_SUSPICIOUSPEOPLE => 234,
            GDELTCategoryList::SOC_TRAFFICACCIDENT => 235,
            GDELTCategoryList::SOVEREIGNTY => 236,
            GDELTCategoryList::STATE_OF_EMERGENCY => 237,
            GDELTCategoryList::STRIKE => 238,
            GDELTCategoryList::SUICIDE_ATTACK => 239,
            GDELTCategoryList::SURVEILLANCE => 240,
            GDELTCategoryList::TAKE_OFFICE => 241,
            GDELTCategoryList::TAX_CARTELS => 242,
            GDELTCategoryList::TAX_DISEASE => 243,
            GDELTCategoryList::TAX_ETHNICITY => 244,
            GDELTCategoryList::TAX_FNCACT => 245,
            GDELTCategoryList::TAX_FOODSTAPLES => 246,
            GDELTCategoryList::TAX_MILITARY_TITLE => 247,
            GDELTCategoryList::TAX_POLITICAL_PARTY => 248,
            GDELTCategoryList::TAX_RELIGION => 249,
            GDELTCategoryList::TAX_SPECIAL_ISSUES => 250,
            GDELTCategoryList::TAX_SPECIALDEATH => 251,
            GDELTCategoryList::TAX_TERROR_GROUP => 252,
            GDELTCategoryList::TAX_WEAPONS => 253,
            GDELTCategoryList::TERROR => 254,
            GDELTCategoryList::TORTURE => 255,
            GDELTCategoryList::TOURISM => 256,
            GDELTCategoryList::TRAFFIC => 257,
            GDELTCategoryList::TRANSPARENCY => 258,
            GDELTCategoryList::TREASON => 259,
            GDELTCategoryList::TRIAL => 260,
            GDELTCategoryList::UNEMPLOYMENT => 261,
            GDELTCategoryList::UNGOVERNED => 262,
            GDELTCategoryList::UNREST_CHECKPOINT => 263,
            GDELTCategoryList::UNREST_CLOSINGBORDER => 264,
            GDELTCategoryList::UNREST_HUNGERSTRIKE => 265,
            GDELTCategoryList::UNREST_MOLOTOVCOCKTAIL => 266,
            GDELTCategoryList::UNREST_POLICEBRUTALITY => 267,
            GDELTCategoryList::UNREST_STONETHROWING => 268,
            GDELTCategoryList::UNREST_STONING => 269,
            GDELTCategoryList::UNSAFE_WORK_ENVIRONMENT => 270,
            GDELTCategoryList::URBAN => 271,
            GDELTCategoryList::URBAN_SPRAWL => 272,
            GDELTCategoryList::VANDALIZE => 273,
            GDELTCategoryList::VETO => 274,
            GDELTCategoryList::VIOLENT_UNREST => 275,
            GDELTCategoryList::WATER_SECURITY => 276,
            GDELTCategoryList::WHISTLEBLOWER => 277,
            GDELTCategoryList::WMD => 278,
            GDELTCategoryList::WOUND => 279,
        }
    }
}

/// The `GDELTObject` trait defines a method for creating an instance of a type from a string.
impl GDELTObject for GDELTCategoryList {
    /// Creates an instance of `GDELTCategoryList` from a string representation of its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `record` - A string slice that holds the string representation of the unique identifier.
    ///
    /// # Returns
    ///
    /// An `Option<Self>` which is `Some` if the string can be parsed into a `u16` and converted to a `GDELTCategoryList`.
    /// Returns `None` if the parsing fails.
    fn from_strings(record: &str) -> Option<Self> {
        let split = record.split(",").collect::<Vec<&str>>();

        match split.len() {
            1 => match split.get(0) {
                None => None,
                Some(val) => match GDELTCategoryList::from_str(val) {
                    Ok(oval) => Some(oval),
                    _ => None,
                },
            },
            2 => match split.get(1) {
                None => Some(GDELTCategoryList::from(record.parse::<u16>().ok()?)),
                Some(offset) => match offset.parse::<u64>() {
                    Ok(offset_u128) => match GDELTCategoryList::from_str(split.get(0)?) {
                        Ok(val) => Some(GDELTCategoryList::from(val)),
                        _ => None,
                    },
                    _ => None,
                },
            },
            _ => None,
        }
    }

    // TODO: Fix the above fuckery i literally dont understand why i did it but this was designed to hold an offset. Maybe make another enum to support V2 Themes? idk.

    /// Creates a new instance of `GDELTCategoryList` from a vector of string slices.
    ///
    /// # Arguments
    ///
    /// * `fields` - A vector of string slices representing the fields to be parsed into a `GDELTCategoryList`.
    ///
    /// # Returns
    ///
    /// * `Option<Self>` - Returns `Some(GDELTCategoryList)` if the first field can be successfully parsed into a `GDELTCategoryList`.
    ///   Returns `None` if the first field is empty or if parsing fails.
    fn new(fields: Vec<&str>) -> Option<Self> {
        match fields.get(0) {
            // If the first field is empty or not present, return None.
            Some(&"") | None => None,
            // If the first field is present, attempt to parse it into a GDELTCategoryList.
            Some(value) => GDELTCategoryList::from_strings(value),
        }
    }
}

/// Implementation of the `CellItem` trait for the `GDELTCategoryList` enum.
impl CellItem for GDELTCategoryList {
    /// Converts a delimited string into a vector of `GDELTCategoryList` items.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice containing the delimited items to be converted.
    ///
    /// # Returns
    ///
    /// * `Option<Vec<Self>>` - Returns `Some(Vec<GDELTCategoryList>)` if the conversion is successful.
    ///   Returns `None` if the conversion fails or if the resulting vector is empty.
    fn vec_from_cell(string: &str) -> Option<Vec<Self>> {
        // Split the input string into a vector of items using the delimiter ";"
        let list_of_items = <Self as GDELTObject>::delimited_vector(";", string);

        // Flag to determine if the items are in version 2 format (contain a comma)
        let mut is_v2 = false;

        // Check if the first item contains a comma to set the version flag
        if list_of_items[0].contains(",") {
            is_v2 = true;
        }

        // Initialize an empty vector to store the resulting `GDELTCategoryList` items
        let mut result = Vec::new();

        // Iterate over each item in the list
        for item in list_of_items {
            if is_v2 {
                // If the item is in version 2 format, attempt to convert it to `GDELTCategoryList`
                if let Some(category) = GDELTCategoryList::from_strings(item) {
                    result.push(category);
                }
            } else {
                // If the item is not in version 2 format, attempt to convert it to `GDELTCategoryList`
                if let Some(category) = GDELTCategoryList::from_strings(item) {
                    result.push(category);
                }
            }
        }

        // Return `None` if the result vector is empty, otherwise return the result vector wrapped in `Some`
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

/// The `GDELTCategoryList` enum represents various categories in the GDELT dataset.
/// Each variant corresponds to a specific category with a unique identifier.
impl GDELTCategoryList {
    /// Returns the unique identifier associated with the `GDELTCategoryList` variant.
    ///
    /// # Returns
    ///
    /// A `u16` value representing the unique identifier of the category.
    pub fn value(&self) -> u16 {
        u16::from(self.clone())
    }

    /// Extracts the inner `u64` value from a `GDELTCategoryList` variant.
    ///
    /// # Arguments
    ///
    /// * `value` - A `GDELTCategoryList` variant from which the inner value is to be extracted.
    ///
    /// # Returns
    ///
    /// * `Option<u64>` - Returns `Some(u64)` containing the inner value if the variant contains one.
    ///   Returns `None` if the variant does not contain an inner value.
    pub(crate) fn extract_value(value: GDELTCategoryList) -> Option<u64> {
        todo!("This is broken as hell. Find a way to store Offsets for categories without keeping them in the enum")
    }
    //     match value {
    //         GDELTCategoryList::UNKNOWN
    //         | GDELTCategoryList::AFFECT
    //         | GDELTCategoryList::AGRICULTURE
    //         | GDELTCategoryList::ALLIANCE
    //         | GDELTCategoryList::APPOINTMENT
    //         | GDELTCategoryList::ARMEDCONFLICT
    //         | GDELTCategoryList::ARREST
    //         | GDELTCategoryList::ASSASSINATION
    //         | GDELTCategoryList::AUSTERITY
    //         | GDELTCategoryList::AVIATION_INCIDENT
    //         | GDELTCategoryList::BAN
    //         | GDELTCategoryList::BLACK_MARKET
    //         | GDELTCategoryList::BLOCKADE
    //         | GDELTCategoryList::BORDER
    //         | GDELTCategoryList::BULLYING
    //         | GDELTCategoryList::CEASEFIRE
    //         | GDELTCategoryList::CHARASMATIC_LEADERSHIP
    //         | GDELTCategoryList::CHECKPOINT
    //         | GDELTCategoryList::CLAIM_CREDIT
    //         | GDELTCategoryList::CLOSURE
    //         | GDELTCategoryList::CONFISCATION
    //         | GDELTCategoryList::CONSTITUTIONAL
    //         | GDELTCategoryList::CORRUPTION
    //         | GDELTCategoryList::CRIME_CARTELS
    //         | GDELTCategoryList::CRIME_COMMON_ROBBERY
    //         | GDELTCategoryList::CRIME_ILLEGAL_DRUGS
    //         | GDELTCategoryList::CURFEW
    //         | GDELTCategoryList::CYBER_ATTACK
    //         | GDELTCategoryList::DEATH_PENALTY
    //         | GDELTCategoryList::DEFECTION
    //         | GDELTCategoryList::DELAY
    //         | GDELTCategoryList::DEMOCRACY
    //         | GDELTCategoryList::DISABILITY
    //         | GDELTCategoryList::DISCRIMINATION
    //         | GDELTCategoryList::DISPLACED
    //         | GDELTCategoryList::DRONE
    //         | GDELTCategoryList::DRUG_TRADE
    //         | GDELTCategoryList::ECON_BANKRUPTCY
    //         | GDELTCategoryList::ECON_BOYCOTT
    //         | GDELTCategoryList::ECON_COST_OF_LIVING
    //         | GDELTCategoryList::ECON_CURRENCY_EXCHANGE_RATE
    //         | GDELTCategoryList::ECON_CURRENCY_RESERVES
    //         | GDELTCategoryList::ECON_CUTOUTLOOK
    //         | GDELTCategoryList::ECON_DEBT
    //         | GDELTCategoryList::ECON_DEREGULATION
    //         | GDELTCategoryList::ECON_EARNINGSREPORT
    //         | GDELTCategoryList::ECON_ENTREPRENEURSHIP
    //         | GDELTCategoryList::ECON_FOREIGNINVEST
    //         | GDELTCategoryList::ECON_FREETRADE
    //         | GDELTCategoryList::ECON_HOUSING_PRICES
    //         | GDELTCategoryList::ECON_INFORMAL_ECONOMY
    //         | GDELTCategoryList::ECON_INTEREST_RATES
    //         | GDELTCategoryList::ECON_IPO
    //         | GDELTCategoryList::ECON_MONOPOLY
    //         | GDELTCategoryList::ECON_MOU
    //         | GDELTCategoryList::ECON_NATIONALIZE
    //         | GDELTCategoryList::ECON_PRICECONTROL
    //         | GDELTCategoryList::ECON_REMITTANCE
    //         | GDELTCategoryList::ECON_STOCKMARKET
    //         | GDELTCategoryList::ECON_SUBSIDIES
    //         | GDELTCategoryList::ECON_TAXATION
    //         | GDELTCategoryList::ECON_TRADE_DISPUTE
    //         | GDELTCategoryList::ECON_UNIONS
    //         | GDELTCategoryList::EDUCATION
    //         | GDELTCategoryList::ELECTION
    //         | GDELTCategoryList::ELECTION_FRAUD
    //         | GDELTCategoryList::ENV_BIOFUEL
    //         | GDELTCategoryList::ENV_CARBONCAPTURE
    //         | GDELTCategoryList::ENV_CLIMATECHANGE
    //         | GDELTCategoryList::ENV_COAL
    //         | GDELTCategoryList::ENV_DEFORESTATION
    //         | GDELTCategoryList::ENV_FISHERY
    //         | GDELTCategoryList::ENV_FORESTRY
    //         | GDELTCategoryList::ENV_GEOTHERMAL
    //         | GDELTCategoryList::ENV_GREEN
    //         | GDELTCategoryList::ENV_HYDRO
    //         | GDELTCategoryList::ENV_METALS
    //         | GDELTCategoryList::ENV_MINING
    //         | GDELTCategoryList::ENV_NATURALGAS
    //         | GDELTCategoryList::ENV_NUCLEARPOWER
    //         | GDELTCategoryList::ENV_OIL
    //         | GDELTCategoryList::ENV_OVERFISH
    //         | GDELTCategoryList::ENV_POACHING
    //         | GDELTCategoryList::ENV_SOLAR
    //         | GDELTCategoryList::ENV_SPECIESENDANGERED
    //         | GDELTCategoryList::ENV_SPECIESEXTINCT
    //         | GDELTCategoryList::ENV_WATERWAYS
    //         | GDELTCategoryList::ENV_WINDPOWER
    //         | GDELTCategoryList::ETH_INDIGINOUS
    //         | GDELTCategoryList::EVACUATION
    //         | GDELTCategoryList::EXHUMATION
    //         | GDELTCategoryList::EXILE
    //         | GDELTCategoryList::EXTREMISM
    //         | GDELTCategoryList::FIREARM_OWNERSHIP
    //         | GDELTCategoryList::FOOD_SECURITY
    //         | GDELTCategoryList::FOOD_STAPLE
    //         | GDELTCategoryList::FREESPEECH
    //         | GDELTCategoryList::FUELPRICES
    //         | GDELTCategoryList::GEN_HOLIDAY
    //         | GDELTCategoryList::GENDER_VIOLENCE
    //         | GDELTCategoryList::GENERAL_GOVERNMENT
    //         | GDELTCategoryList::GENERAL_HEALTH
    //         | GDELTCategoryList::GENTRIFICATION
    //         | GDELTCategoryList::GOV_DISSOLVEGOV
    //         | GDELTCategoryList::GOV_DIVISIONOFPOWER
    //         | GDELTCategoryList::GOV_INTERGOVERNMENTAL
    //         | GDELTCategoryList::GOV_REFORM
    //         | GDELTCategoryList::GOV_REPATRIATION
    //         | GDELTCategoryList::GRIEVANCES
    //         | GDELTCategoryList::HARASSMENT
    //         | GDELTCategoryList::HATE_SPEECH
    //         | GDELTCategoryList::HEALTH_PANDEMIC
    //         | GDELTCategoryList::HEALTH_SEXTRANSDISEASE
    //         | GDELTCategoryList::HEALTH_VACCINATION
    //         | GDELTCategoryList::HUMAN_TRAFFICKING
    //         | GDELTCategoryList::IDEOLOGY
    //         | GDELTCategoryList::IMMIGRATION
    //         | GDELTCategoryList::IMPEACHMENT
    //         | GDELTCategoryList::INFO_HOAX
    //         | GDELTCategoryList::INFO_RUMOR
    //         | GDELTCategoryList::INFRASTRUCTURE_BAD_ROADS
    //         | GDELTCategoryList::INSURGENCY
    //         | GDELTCategoryList::INTERNET_BLACKOUT
    //         | GDELTCategoryList::INTERNET_CENSORSHIP
    //         | GDELTCategoryList::JIHAD
    //         | GDELTCategoryList::KIDNAP
    //         | GDELTCategoryList::KILL
    //         | GDELTCategoryList::LANDMINE
    //         | GDELTCategoryList::LEADER
    //         | GDELTCategoryList::LEGALIZE
    //         | GDELTCategoryList::LEGISLATION
    //         | GDELTCategoryList::LGBT
    //         | GDELTCategoryList::LITERACY
    //         | GDELTCategoryList::LOCUSTS
    //         | GDELTCategoryList::MANMADE_DISASTER
    //         | GDELTCategoryList::MANMADE_DISASTER_IMPLIED
    //         | GDELTCategoryList::MARITIME
    //         | GDELTCategoryList::MARITIME_INCIDENT
    //         | GDELTCategoryList::MARITIME_INCIDENT_IMPLIED
    //         | GDELTCategoryList::MARITIME_INCIDENT_SELF_IDENTIFIED
    //         | GDELTCategoryList::MARITIME_PIRACY
    //         | GDELTCategoryList::MEDIA_CENSORSHIP
    //         | GDELTCategoryList::MEDIA_MSM
    //         | GDELTCategoryList::MEDIA_SOCIAL
    //         | GDELTCategoryList::MEDICAL
    //         | GDELTCategoryList::MEDICAL_SECURITY
    //         | GDELTCategoryList::MIL_SELF_IDENTIFIED_ARMS_DEAL
    //         | GDELTCategoryList::MIL_WEAPONS_PROLIFERATION
    //         | GDELTCategoryList::MILITARY
    //         | GDELTCategoryList::MILITARY_COOPERATION
    //         | GDELTCategoryList::MOVEMENT_ENVIRONMENTAL
    //         | GDELTCategoryList::MOVEMENT_GENERAL
    //         | GDELTCategoryList::MOVEMENT_OTHER
    //         | GDELTCategoryList::MOVEMENT_SOCIAL
    //         | GDELTCategoryList::MOVEMENT_WOMENS
    //         | GDELTCategoryList::NATURAL_DISASTER
    //         | GDELTCategoryList::NEGOTIATIONS
    //         | GDELTCategoryList::NEW_CONSTRUCTION
    //         | GDELTCategoryList::ORGANIZED_CRIME
    //         | GDELTCategoryList::PEACEKEEPING
    //         | GDELTCategoryList::PERSECUTION
    //         | GDELTCategoryList::PHONE_OUTAGE
    //         | GDELTCategoryList::PIPELINE_INCIDENT
    //         | GDELTCategoryList::PIRACY
    //         | GDELTCategoryList::POL_HOSTVISIT
    //         | GDELTCategoryList::POLITICAL_PRISONER
    //         | GDELTCategoryList::POLITICAL_TURMOIL
    //         | GDELTCategoryList::POPULATION_DENSITY
    //         | GDELTCategoryList::POVERTY
    //         | GDELTCategoryList::POWER_OUTAGE
    //         | GDELTCategoryList::PRIVATIZATION
    //         | GDELTCategoryList::PROPAGANDA
    //         | GDELTCategoryList::PROPERTY_RIGHTS
    //         | GDELTCategoryList::PROTEST
    //         | GDELTCategoryList::PUBLIC_TRANSPORT
    //         | GDELTCategoryList::RAIL_INCIDENT
    //         | GDELTCategoryList::RAPE
    //         | GDELTCategoryList::RATIFY
    //         | GDELTCategoryList::REBELLION
    //         | GDELTCategoryList::REBELS
    //         | GDELTCategoryList::RECRUITMENT
    //         | GDELTCategoryList::REFUGEES
    //         | GDELTCategoryList::REL_ANTISEMITISM
    //         | GDELTCategoryList::RELATIONS
    //         | GDELTCategoryList::RELEASE_HOSTAGE
    //         | GDELTCategoryList::RELEASE_PRISON
    //         | GDELTCategoryList::RELIGION
    //         | GDELTCategoryList::RESIGNATION
    //         | GDELTCategoryList::RETALIATE
    //         | GDELTCategoryList::RETIREMENT
    //         | GDELTCategoryList::RETIREMENTS
    //         | GDELTCategoryList::ROAD_INCIDENT
    //         | GDELTCategoryList::RURAL
    //         | GDELTCategoryList::SANCTIONS
    //         | GDELTCategoryList::SANITATION
    //         | GDELTCategoryList::SCANDAL
    //         | GDELTCategoryList::SCIENCE
    //         | GDELTCategoryList::SECURITY_SERVICES
    //         | GDELTCategoryList::SEIGE
    //         | GDELTCategoryList::SEIZE
    //         | GDELTCategoryList::SELF_IDENTIFIED_ATROCITY
    //         | GDELTCategoryList::SELF_IDENTIFIED_ENVIRON_DISASTER
    //         | GDELTCategoryList::SELF_IDENTIFIED_HUMAN_RIGHTS
    //         | GDELTCategoryList::SELF_IDENTIFIED_HUMANITARIAN_CRISIS
    //         | GDELTCategoryList::SEPARATISTS
    //         | GDELTCategoryList::SHORTAGE
    //         | GDELTCategoryList::SICKENED
    //         | GDELTCategoryList::SLFID_CAPACITY_BUILDING
    //         | GDELTCategoryList::SLFID_CIVIL_LIBERTIES
    //         | GDELTCategoryList::SLFID_DICTATORSHIP
    //         | GDELTCategoryList::SLFID_ECONOMIC_DEVELOPMENT
    //         | GDELTCategoryList::SLFID_ECONOMIC_POWER
    //         | GDELTCategoryList::SLFID_MILITARY_BUILDUP
    //         | GDELTCategoryList::SLFID_MILITARY_READINESS
    //         | GDELTCategoryList::SLFID_MILITARY_SPENDING
    //         | GDELTCategoryList::SLFID_MINERAL_RESOURCES
    //         | GDELTCategoryList::SLFID_NATURAL_RESOURCES
    //         | GDELTCategoryList::SLFID_PEACE_BUILDING
    //         | GDELTCategoryList::SLFID_POLITICAL_BOUNDARIES
    //         | GDELTCategoryList::SLFID_RULE_OF_LAW
    //         | GDELTCategoryList::SLUMS
    //         | GDELTCategoryList::SMUGGLING
    //         | GDELTCategoryList::SOC_DIPLOMCOOP
    //         | GDELTCategoryList::SOC_ECONCOOP
    //         | GDELTCategoryList::SOC_EXPRESSREGRET
    //         | GDELTCategoryList::SOC_EXPRESSSUPPORT
    //         | GDELTCategoryList::SOC_FORCEDRELOCATION
    //         | GDELTCategoryList::SOC_GENERALCRIME
    //         | GDELTCategoryList::SOC_INTELSHARING
    //         | GDELTCategoryList::SOC_JUDICIALCOOP
    //         | GDELTCategoryList::SOC_MASSMIGRATION
    //         | GDELTCategoryList::SOC_PARDON
    //         | GDELTCategoryList::SOC_SUICIDE
    //         | GDELTCategoryList::SOC_SUSPICIOUSACTIVITIES
    //         | GDELTCategoryList::SOC_SUSPICIOUSPEOPLE
    //         | GDELTCategoryList::SOC_TRAFFICACCIDENT
    //         | GDELTCategoryList::SOVEREIGNTY
    //         | GDELTCategoryList::STATE_OF_EMERGENCY
    //         | GDELTCategoryList::STRIKE
    //         | GDELTCategoryList::SUICIDE_ATTACK
    //         | GDELTCategoryList::SURVEILLANCE
    //         | GDELTCategoryList::TAKE_OFFICE
    //         | GDELTCategoryList::TAX_CARTELS
    //         | GDELTCategoryList::TAX_DISEASE
    //         | GDELTCategoryList::TAX_ETHNICITY
    //         | GDELTCategoryList::TAX_FNCACT
    //         | GDELTCategoryList::TAX_FOODSTAPLES
    //         | GDELTCategoryList::TAX_MILITARY_TITLE
    //         | GDELTCategoryList::TAX_POLITICAL_PARTY
    //         | GDELTCategoryList::TAX_RELIGION
    //         | GDELTCategoryList::TAX_SPECIAL_ISSUES
    //         | GDELTCategoryList::TAX_SPECIALDEATH
    //         | GDELTCategoryList::TAX_TERROR_GROUP
    //         | GDELTCategoryList::TAX_WEAPONS
    //         | GDELTCategoryList::TERROR
    //         | GDELTCategoryList::TORTURE
    //         | GDELTCategoryList::TOURISM
    //         | GDELTCategoryList::TRAFFIC
    //         | GDELTCategoryList::TRANSPARENCY
    //         | GDELTCategoryList::TREASON
    //         | GDELTCategoryList::TRIAL
    //         | GDELTCategoryList::UNEMPLOYMENT
    //         | GDELTCategoryList::UNGOVERNED
    //         | GDELTCategoryList::UNREST_CHECKPOINT
    //         | GDELTCategoryList::UNREST_CLOSINGBORDER
    //         | GDELTCategoryList::UNREST_HUNGERSTRIKE
    //         | GDELTCategoryList::UNREST_MOLOTOVCOCKTAIL
    //         | GDELTCategoryList::UNREST_POLICEBRUTALITY
    //         | GDELTCategoryList::UNREST_STONETHROWING
    //         | GDELTCategoryList::UNREST_STONING
    //         | GDELTCategoryList::UNSAFE_WORK_ENVIRONMENT
    //         | GDELTCategoryList::URBAN
    //         | GDELTCategoryList::URBAN_SPRAWL
    //         | GDELTCategoryList::VANDALIZE
    //         | GDELTCategoryList::VETO
    //         | GDELTCategoryList::VIOLENT_UNREST
    //         | GDELTCategoryList::WATER_SECURITY
    //         | GDELTCategoryList::WHISTLEBLOWER
    //         | GDELTCategoryList::WMD
    //         | GDELTCategoryList::WOUND => Some(val1.clone().into_inner()),
    //     }
    // }
}

impl ToProto for GDELTCategoryList {
    type ProtoType = GdeltCategoryList;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        let temp = self.clone().to_string();
        GdeltCategoryList::from_str_name(&temp)
    }
}
