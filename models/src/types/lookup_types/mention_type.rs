use anyhow::anyhow;
use url::Url;

use crate::types::mention_table::{MentionIdentifier, MentionTypeCode};

pub enum MentionType {
    Web(Url),
    CitationOnly(MentionIdentifier),
    Core(MentionIdentifier),
    DTIC(MentionIdentifier),
    JSTOR(MentionIdentifier),
    NonTextualSource(Url),
}

impl TryFrom<(MentionTypeCode, MentionIdentifier)> for MentionType {
    type Error = anyhow::Error;

    fn try_from(value: (MentionTypeCode, MentionIdentifier)) -> anyhow::Result<Self> {
        match value.0.0 {
            1 => Ok(Self::Web(Url::parse(&value.1.0)?)),
            2 => Ok(Self::CitationOnly(value.1)),
            3 => Ok(Self::Core(value.1)),
            4 => Ok(Self::DTIC(value.1)),
            5 => Ok(Self::JSTOR(value.1)),
            6 => Ok(Self::NonTextualSource(Url::parse(&value.1.0)?)),
            _ => Err(anyhow!("Invalid Mention Source")),
        }
    }
}
