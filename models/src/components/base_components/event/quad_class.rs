use crate::components::base_components::raw_types::RawQuadClass;

pub enum QuadClass {
    Invalid,
    Cooperation(Manner),
    Conflict(Manner),
}

pub enum Manner {
    Verbal,
    Material,
}

impl From<RawQuadClass> for QuadClass {
    fn from(value: RawQuadClass) -> Self {
        match value.0 {
            1 => QuadClass::Cooperation(Manner::Verbal),
            2 => QuadClass::Cooperation(Manner::Material),
            3 => QuadClass::Conflict(Manner::Verbal),
            4 => QuadClass::Conflict(Manner::Material),
            _ => QuadClass::Invalid,
        }
    }
}
