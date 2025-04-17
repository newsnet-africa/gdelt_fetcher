#[repr(transparent)]
pub struct CAMEOActorCode(pub [[u8; 3]; 5]);

#[repr(transparent)]
pub struct CAMEOCountryCode(pub [u8; 3]);

#[repr(transparent)]
pub struct EthnicGroupCode(pub [u8; 3]);

#[repr(transparent)]
pub struct FIPS(pub [u8; 2]);

#[repr(transparent)]
pub struct KnownGroupCode(pub [u8; 3]);

#[repr(transparent)]
pub struct ReligionCode(pub [u8; 3]);

#[repr(transparent)]
pub struct ActorTypeCode(pub [u8; 3]);
