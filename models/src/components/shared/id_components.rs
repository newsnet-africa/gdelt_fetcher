use serde::{Deserialize, Serialize};
use url::Url;

use crate::gdelt_schema::{SchemaKey, event::Event, gkg::GlobalKnowledgeGraph, mention::Mention};

#[repr(transparent)]
pub struct GlobalEventID(u128);

#[repr(transparent)]
#[derive(Debug, Serialize, Deserialize)]
pub struct GDGRecordID(String);

#[repr(transparent)]
pub struct GEGRecordID(String);

#[repr(transparent)]
pub struct GQGRecordID(String);

#[repr(transparent)]
pub struct GRGRecordID(String);

#[repr(transparent)]
pub struct OrganisationCode([u8; 3]);

#[repr(transparent)]
pub struct GKGRecordID(String);

pub enum MentionIdentifier {
    Url(Url),
    Citation(String),
    DOI(String),
    Other(String),
}

impl<'a> SchemaKey<'a, Event> for GlobalEventID {
    type RawType = u128;

    fn raw_key(&self) -> &Self::RawType {
        todo!()
    }
}

impl From<Vec<u8>> for GlobalEventID {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for GlobalEventID {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

impl<'a> SchemaKey<'a, Mention> for MentionIdentifier {
    type RawType = u128;

    fn raw_key(&self) -> &Self::RawType {
        todo!()
    }
}

impl From<Vec<u8>> for MentionIdentifier {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for MentionIdentifier {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

impl<'a> SchemaKey<'a, GlobalKnowledgeGraph> for GKGRecordID {
    type RawType = u128;

    fn raw_key(&self) -> &Self::RawType {
        todo!()
    }
}

impl From<Vec<u8>> for GKGRecordID {
    fn from(value: Vec<u8>) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for GKGRecordID {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
