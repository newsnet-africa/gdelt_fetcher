use crate::models::gdelt::ToProto;

//region SourceCollectionIdentifier Enum
/// Represents the source collection identifier for GDELT data.
/// Each variant corresponds to a specific source collection.
#[derive(Debug, Clone)]
pub enum SourceCollectionIdentifier {
    /// Web source collection
    WEB = 1,
    /// Citation source collection
    CITATION = 2,
    /// Core source collection
    CORE = 3,
    /// DTIC source collection
    DTIC = 4,
    /// JSTOR source collection
    JSTOR = 5,
    /// Non-textual source collection
    NONTEXTUALSOURCE = 6,
    /// Unknown source collection
    UNKNOWN
}
//endregion

impl ToProto for SourceCollectionIdentifier {
    type ProtoType = u8;

    fn to_proto(&self) -> Option<Self::ProtoType> {
        Some(u8::from(self.clone()))
    }
}

//region From<u8> for SourceCollectionIdentifier
/// Implements conversion from `u8` to `SourceCollectionIdentifier`.
impl From<u8> for SourceCollectionIdentifier {
    /// Converts a `u8` to a `SourceCollectionIdentifier`.
    ///
    /// # Arguments
    ///
    /// * `num` - A `u8` representing the source collection identifier.
    ///
    /// # Returns
    ///
    /// * `SourceCollectionIdentifier` - The corresponding enum variant.
    fn from(num: u8) -> Self {
        match num {
            1 => Self::WEB,
            2 => Self::CITATION,
            3 => Self::CORE,
            4 => Self::DTIC,
            5 => Self::JSTOR,
            6 => Self::NONTEXTUALSOURCE,
            _ => Self::UNKNOWN,
        }
    }
}
//endregion

//region From<SourceCollectionIdentifier> for u8
/// Implements conversion from `SourceCollectionIdentifier` to `u8`.
impl From<SourceCollectionIdentifier> for u8 {
    /// Converts a `SourceCollectionIdentifier` to a `u8`.
    ///
    /// # Arguments
    ///
    /// * `value` - A `SourceCollectionIdentifier` enum variant.
    ///
    /// # Returns
    ///
    /// * `u8` - The corresponding `u8` value.
    fn from(value: SourceCollectionIdentifier) -> Self {
        match value {
            SourceCollectionIdentifier::UNKNOWN => 0,
            SourceCollectionIdentifier::WEB => 1,
            SourceCollectionIdentifier::CITATION => 2,
            SourceCollectionIdentifier::CORE => 3,
            SourceCollectionIdentifier::DTIC => 4,
            SourceCollectionIdentifier::JSTOR => 5,
            SourceCollectionIdentifier::NONTEXTUALSOURCE => 6,
        }
    }
}
//endregion

//region SourceCollectionIdentifier Methods
impl SourceCollectionIdentifier {
    /// Converts a string to a `SourceCollectionIdentifier`.
    ///
    /// # Arguments
    ///
    /// * `string` - A string slice that holds the source collection identifier.
    ///
    /// # Returns
    ///
    /// * `Option<SourceCollectionIdentifier>` - The corresponding enum variant wrapped in an `Option`.
    ///   Returns `None` if the string cannot be parsed to a `u8`.
    pub fn from_string(string: &str) -> Option<Self> {
        let result = string.parse::<u8>();
        
        match result {
            Ok(num) => Some(Self::from(num)),
            Err(_) => None,
        }
    }
}
//endregion}
