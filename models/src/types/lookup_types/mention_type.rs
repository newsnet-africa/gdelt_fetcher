use anyhow::anyhow;
use url::Url;

use crate::types::mention_table::{MentionIdentifier, MentionTypeCode};

#[derive(Debug)]
pub enum MentionType {
    Web(Url),
    CitationOnly(MentionIdentifier),
    Core(MentionIdentifier),
    DTIC(MentionIdentifier),
    JSTOR(MentionIdentifier),
    NonTextualSource(Url),
}

impl TryFrom<Option<(MentionTypeCode, MentionIdentifier)>> for MentionType {
    type Error = anyhow::Error;

    fn try_from(value: Option<(MentionTypeCode, MentionIdentifier)>) -> anyhow::Result<Self> {
        let value = value.ok_or_else(|| anyhow!("MentionTypeCode is None"))?;
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
    fn test_mention_type_try_from_valid_cases() {
        // init_logger();

        let web_code = MentionTypeCode(1);
        let web_identifier = MentionIdentifier("https://example.com".to_string());
        info!("Testing valid MentionType: Web");
        let mention_type = MentionType::try_from(Some((web_code, web_identifier)));
        assert!(mention_type.is_ok());
        assert!(matches!(mention_type.unwrap(), MentionType::Web(_)));

        let citation_code = MentionTypeCode(2);
        let citation_identifier = MentionIdentifier("citation_id".to_string());
        info!("Testing valid MentionType: CitationOnly");
        let mention_type = MentionType::try_from(Some((citation_code, citation_identifier)));
        assert!(mention_type.is_ok());
        assert!(matches!(
            mention_type.unwrap(),
            MentionType::CitationOnly(_)
        ));

        let core_code = MentionTypeCode(3);
        let core_identifier = MentionIdentifier("core_id".to_string());
        info!("Testing valid MentionType: Core");
        let mention_type = MentionType::try_from(Some((core_code, core_identifier)));
        assert!(mention_type.is_ok());
        assert!(matches!(mention_type.unwrap(), MentionType::Core(_)));
    }

    #[test]
    fn test_mention_type_try_from_invalid_cases() {
        // init_logger();

        let invalid_code = MentionTypeCode(99);
        let identifier = MentionIdentifier("invalid_id".to_string());
        info!("Testing invalid MentionTypeCode");
        let mention_type = MentionType::try_from(Some((invalid_code, identifier)));
        assert!(mention_type.is_err());
    }

    #[test]
    fn test_mention_type_try_from_edge_cases() {
        // init_logger();

        let web_code = MentionTypeCode(1);
        let empty_identifier = MentionIdentifier("".to_string());
        info!("Testing edge case: Empty MentionIdentifier");
        let mention_type = MentionType::try_from(Some((web_code, empty_identifier)));
        assert!(mention_type.is_err());

        let malformed_code = MentionTypeCode(1);
        let malformed_identifier = MentionIdentifier("not_a_url".to_string());
        info!("Testing edge case: Malformed MentionIdentifier for Web");
        let mention_type = MentionType::try_from(Some((malformed_code, malformed_identifier)));
        assert!(mention_type.is_err());
    }
}
