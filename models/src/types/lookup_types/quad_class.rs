use anyhow::anyhow;

use crate::types::event_table::event_action::QuadClassCode;

pub enum QuadClass {
    Invalid,
    Cooperation(Manner),
    Conflict(Manner),
}

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
