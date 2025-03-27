use crate::Schema;

pub mod event;
pub mod gdg;
pub mod geg;
pub mod ggg;
pub mod gkg;
pub mod gqg;
pub mod grg;
pub mod mention;

pub trait SchemaKey<'a, ParentSchema: Schema<'a>>: From<Vec<u8>> + Into<Vec<u8>> {
    type RawType;
    fn raw_key(&self) -> &Self::RawType;
}
