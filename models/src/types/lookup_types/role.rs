use anyhow::anyhow;

use crate::types::event_table::actor::CAMEORoleCode;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ActorRole {
    Unspecified,
    Policeforces,
    Government,
    Insurgents,
    Judiciary,
    Military,
    PoliticalOpposition,
    Rebels,
    SeparatistRebels,
    StateIntelligence,
    UnalignedArmedForces,
    Agriculture,
    Business,
    Criminal,
    Civilian,
    Development,
    Education,
    Elites,
    Environmental,
    Health,
    HumanRights,
    Labor,
    Legislature,
    Media,
    Refugees,
    Moderate,
    Radical,
    AmnestyInternational,
    RedCross,
    Greenpeace,
    UnitedNations,
    Peacekeepers,
    UnidentifiedStateActor,
    InterGovernmentalOrganization,
    InternationalMilitarizedGroup,
    InternationalTransnationalGeneric,
    MultinationalCorporation,
    NonGovernmentalMovement,
    NonGovernmentalOrganization,
    Settler,
}

impl TryFrom<CAMEORoleCode> for ActorRole {
    type Error = anyhow::Error;

    fn try_from(value: CAMEORoleCode) -> anyhow::Result<Self> {
        let str_value = std::str::from_utf8(&value.0)?;
        match str_value {
            "COP" => Ok(Self::Policeforces),
            "GOV" => Ok(Self::Government),
            "INS" => Ok(Self::Insurgents),
            "JUD" => Ok(Self::Judiciary),
            "MIL" => Ok(Self::Military),
            "OPP" => Ok(Self::PoliticalOpposition),
            "REB" => Ok(Self::Rebels),
            "SEP" => Ok(Self::SeparatistRebels),
            "SPY" => Ok(Self::StateIntelligence),
            "UAF" => Ok(Self::UnalignedArmedForces),
            "AGR" => Ok(Self::Agriculture),
            "BUS" => Ok(Self::Business),
            "CRM" => Ok(Self::Criminal),
            "CVL" => Ok(Self::Civilian),
            "DEV" => Ok(Self::Development),
            "EDU" => Ok(Self::Education),
            "ELI" => Ok(Self::Elites),
            "ENV" => Ok(Self::Environmental),
            "HLH" => Ok(Self::Health),
            "HRI" => Ok(Self::HumanRights),
            "LAB" => Ok(Self::Labor),
            "LEG" => Ok(Self::Legislature),
            "MED" => Ok(Self::Media),
            "REF" => Ok(Self::Refugees),
            "MOD" => Ok(Self::Moderate),
            "RAD" => Ok(Self::Radical),
            "AMN" => Ok(Self::AmnestyInternational),
            "IRC" => Ok(Self::RedCross),
            "GRP" => Ok(Self::Greenpeace),
            "UNO" => Ok(Self::UnitedNations),
            "PKO" => Ok(Self::Peacekeepers),
            "IGO" => Ok(Self::InterGovernmentalOrganization),
            "IMG" => Ok(Self::InternationalMilitarizedGroup),
            "INT" => Ok(Self::InternationalTransnationalGeneric),
            "MNC" => Ok(Self::MultinationalCorporation),
            "NGM" => Ok(Self::NonGovernmentalMovement),
            "NGO" => Ok(Self::NonGovernmentalOrganization),
            "UIS" => Ok(Self::UnidentifiedStateActor),
            "SET" => Ok(Self::Settler),
            _ => Err(anyhow!("Invalid CAMEO Country Code")),
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
    fn test_cameo_role_code_try_from_valid_codes() {
        init_logger();

        let valid_code_str = "COP"; // Policeforces
        info!("Testing valid CAMEORoleCode: {:?}", valid_code_str);
        let code = CAMEORoleCode::try_from(Some(valid_code_str));
        assert!(code.is_ok());
        assert_eq!(std::str::from_utf8(&code.unwrap().0).unwrap(), "COP");
    }


    #[test]
    fn test_actor_role_try_from_valid_codes() {
        init_logger();

        let valid_code = CAMEORoleCode(*b"COP"); // Policeforces
        info!("Testing valid ActorRole: {:?}", valid_code);
        let actor_role = ActorRole::try_from(valid_code);
        assert!(actor_role.is_ok());
        assert_eq!(actor_role.unwrap(), ActorRole::Policeforces);
    }

    #[test]
    fn test_actor_role_try_from_invalid_codes() {
        init_logger();

        let invalid_code = CAMEORoleCode(*b"XXX"); // Invalid code
        info!("Testing invalid ActorRole: {:?}", invalid_code);
        let actor_role = ActorRole::try_from(invalid_code);
        assert!(actor_role.is_err());
    }
}