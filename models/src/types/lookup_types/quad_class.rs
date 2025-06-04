use anyhow::anyhow;

use crate::types::event_table::event_action::QuadClassCode;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum QuadClass {
    Invalid,
    Cooperation(Manner),
    Conflict(Manner),
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Manner {
    Verbal,
    Material,
}

impl TryFrom<QuadClassCode> for QuadClass {
    type Error = anyhow::Error;

    fn try_from(value: QuadClassCode) -> anyhow::Result<Self> {
        match value.0 {
            1 => Ok(QuadClass::Cooperation(Manner::Verbal)),
            2 => Ok(QuadClass::Cooperation(Manner::Material)),
            3 => Ok(QuadClass::Conflict(Manner::Verbal)),
            4 => Ok(QuadClass::Conflict(Manner::Material)),
            _ => Err(anyhow!("Invalid Quad Class")),
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
    fn test_quad_class_code_try_from_valid_codes() {
        init_logger();

        let valid_code_str = "1"; // Cooperation Verbal
        info!("Testing valid QuadClassCode: {:?}", valid_code_str);
        let code = QuadClassCode::try_from(Some(valid_code_str));
        assert!(code.is_ok());
        assert_eq!(code.unwrap().0, 1);
    }

    #[test]
    fn test_quad_class_try_from_valid_codes() {
        init_logger();

        let valid_code = QuadClassCode(1); // Cooperation Verbal
        info!("Testing valid QuadClass: {:?}", valid_code);
        let quad_class = QuadClass::try_from(valid_code);
        assert!(quad_class.is_ok());
        assert!(matches!(quad_class.unwrap(), QuadClass::Cooperation(Manner::Verbal)));
    }

    #[test]
    fn test_quad_class_try_from_invalid_codes() {
        init_logger();

        let invalid_code = QuadClassCode(99); // Invalid code
        info!("Testing invalid QuadClass: {:?}", invalid_code);
        let quad_class = QuadClass::try_from(invalid_code);
        assert!(quad_class.is_err());
    }
}