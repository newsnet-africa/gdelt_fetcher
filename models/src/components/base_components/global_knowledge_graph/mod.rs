use crate::schema::mention_table::CharOffset;

pub mod counts;
pub mod gdelt_category;
pub mod tone;

pub struct GKGSourceCommonName(String);

pub struct CountValue(u128);

pub struct ObjectType(String);

pub struct AmountValue(u128);

pub struct Quotation {
    pub offset: CharOffset,
    pub length: u64,
    pub verb: String,
    pub quote: String,
}

pub struct Amount {
    pub amount: AmountValue,
    pub object: ObjectType,
    pub offset: CharOffset,
}
