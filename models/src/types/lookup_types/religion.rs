use crate::types::event_table::actor::CAMEOReligionCode;
use anyhow::anyhow;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Religion {
    Unspecified,
    AfricanDiasporicReligion,
    Alewi,
    Agnostic,
    BahaiFaith,
    Buddhism,
    Christianity,
    Confucianism,
    Coptic,
    Catholic,
    Orthodox,
    Druze,
    Hinduism,
    Hasidic,
    IndigenousTribalReligion,
    Jainism,
    Judaism,
    JehovahsWitness,
    LatterDaySaints,
    Muslim,
    Maronite,
    NewReligiousMovement,
    Pagan,
    Protestant,
    Sufi,
    Shia,
    OldShintoSchool,
    Sikh,
    Sunni,
    Taoist,
    UltraOrthodox,
    Zoroastrianism,
}

impl TryFrom<CAMEOReligionCode> for Religion {
    type Error = anyhow::Error;

    fn try_from(value: CAMEOReligionCode) -> Result<Self, Self::Error> {
        let str_value = std::str::from_utf8(&value.0).expect("Invalid CAMEO Code format");
        match std::str::from_utf8(&value.0)? {
            "ADR" => Ok(Self::AfricanDiasporicReligion),
            "ALE" => Ok(Self::Alewi),
            "ATH" => Ok(Self::Agnostic),
            "BAH" => Ok(Self::BahaiFaith),
            "BUD" => Ok(Self::Buddhism),
            "CHR" => Ok(Self::Christianity),
            "CON" => Ok(Self::Confucianism),
            "CPT" => Ok(Self::Coptic),
            "CTH" => Ok(Self::Catholic),
            "DOX" => Ok(Self::Orthodox),
            "DRZ" => Ok(Self::Druze),
            "HIN" => Ok(Self::Hinduism),
            "HSD" => Ok(Self::Hasidic),
            "ITR" => Ok(Self::IndigenousTribalReligion),
            "JAN" => Ok(Self::Jainism),
            "JEW" => Ok(Self::Judaism),
            "JHW" => Ok(Self::JehovahsWitness),
            "LDS" => Ok(Self::LatterDaySaints),
            "MOS" => Ok(Self::Muslim),
            "MRN" => Ok(Self::Maronite),
            "NRM" => Ok(Self::NewReligiousMovement),
            "PAG" => Ok(Self::Pagan),
            "PRO" => Ok(Self::Protestant),
            "SFI" => Ok(Self::Sufi),
            "SHI" => Ok(Self::Shia),
            "SHN" => Ok(Self::OldShintoSchool),
            "SIK" => Ok(Self::Sikh),
            "SUN" => Ok(Self::Sunni),
            "TAO" => Ok(Self::Taoist),
            "UDX" => Ok(Self::UltraOrthodox),
            "ZRO" => Ok(Self::Zoroastrianism),
            _ => Err(anyhow!("Invalid Religion Code")),
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
    fn test_cameo_religion_code_try_from_valid_codes() {
        init_logger();

        let valid_code_str = "CHR"; // Christianity
        info!("Testing valid CAMEOReligionCode: {:?}", valid_code_str);
        let code = CAMEOReligionCode::try_from(Some(valid_code_str));
        assert!(code.is_ok());
        assert_eq!(code.unwrap().0, *b"CHR");
    }

    #[test]
    fn test_cameo_religion_code_try_from_invalid_codes() {
        init_logger();

        let invalid_code_str = "XXXX"; // Invalid code
        info!("Testing invalid CAMEOReligionCode: {:?}", invalid_code_str);
        let code = CAMEOReligionCode::try_from(Some(invalid_code_str));
        assert!(code.is_err());
    }

    #[test]
    fn test_religion_try_from_valid_codes() {
        init_logger();

        let valid_code = CAMEOReligionCode(*b"CHR"); // Christianity
        info!("Testing valid Religion code: {:?}", valid_code);
        let religion = Religion::try_from(valid_code);
        assert!(religion.is_ok());
        assert_eq!(religion.unwrap(), Religion::Christianity);
    }

    #[test]
    fn test_religion_try_from_invalid_codes() {
        init_logger();

        let invalid_code = CAMEOReligionCode(*b"XXX"); // Invalid code
        info!("Testing invalid Religion code: {:?}", invalid_code);
        let religion = Religion::try_from(invalid_code);
        assert!(religion.is_err());
    }
}