pub mod actor;
pub mod location;
pub mod mention;
pub mod verb;

pub trait Code {
    type RawType;
}
