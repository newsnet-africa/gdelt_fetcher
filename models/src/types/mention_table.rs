use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use super::{DatabaseTable, event_table::GlobalEventID, lookup_types::mention_type::MentionType};

#[derive(Debug)]
pub struct MentionTypeCode(pub u8);

impl TryFrom<Option<&str>> for MentionTypeCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u8>()
                .map(MentionTypeCode)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing MentionTypeCode")),
        }
    }
}

#[derive(Debug)]
pub struct MentionSourceName(pub String);

impl TryFrom<Option<&str>> for MentionSourceName {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => Ok(MentionSourceName(s.to_string())),
            None => Err(anyhow::anyhow!("missing MentionSourceName")),
        }
    }
}

#[derive(Debug)]
pub struct MentionIdentifier(pub String);

impl TryFrom<Option<&str>> for MentionIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => Ok(MentionIdentifier(s.to_string())),
            None => Err(anyhow::anyhow!("missing MentionIdentifier")),
        }
    }
}

#[derive(Debug)]
pub struct SentenceID(pub u128);

impl TryFrom<Option<&str>> for SentenceID {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u128>()
                .map(SentenceID)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing SentenceID")),
        }
    }
}

#[derive(Debug)]
pub struct CharOffset(pub u128);

impl TryFrom<Option<&str>> for CharOffset {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            None | Some("-1") => Err(anyhow::anyhow!("missing CharOffset")),
            Some(s) => s
                .parse::<u128>()
                .map(CharOffset)
                .map_err(|e| anyhow::anyhow!(e)),
        }
    }
}

#[derive(Debug)]
pub struct InRawText(pub bool);

impl TryFrom<Option<&str>> for InRawText {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => match s {
                "true" | "1" => Ok(InRawText(true)),
                "false" | "0" => Ok(InRawText(false)),
                _ => Err(anyhow::anyhow!("invalid InRawText value")),
            },
            None => Err(anyhow::anyhow!("missing InRawText")),
        }
    }
}

#[derive(Debug)]
pub struct Confidence(pub u8);

impl TryFrom<Option<&str>> for Confidence {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u8>()
                .map(Confidence)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing Confidence")),
        }
    }
}

#[derive(Debug)]
pub struct MentionDocLength(pub u128);

impl TryFrom<Option<&str>> for MentionDocLength {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<u128>()
                .map(MentionDocLength)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing MentionDocLength")),
        }
    }
}

#[derive(Debug)]
pub struct MentionDocTone(pub f32);

impl TryFrom<Option<&str>> for MentionDocTone {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => s
                .parse::<f32>()
                .map(MentionDocTone)
                .map_err(|e| anyhow::anyhow!(e)),
            None => Err(anyhow::anyhow!("missing MentionDocTone")),
        }
    }
}

#[derive(Debug)]
pub struct SourceLanguageCode(pub [u8; 3]);

impl TryFrom<Option<&str>> for SourceLanguageCode {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => {
                if s.len() != 3 {
                    return Err(anyhow::anyhow!("SourceLanguageCode must be 3 bytes long"));
                }
                let bytes = s.as_bytes();
                Ok(SourceLanguageCode([bytes[0], bytes[1], bytes[2]]))
            }
            None => Err(anyhow::anyhow!("missing SourceLanguageCode")),
        }
    }
}

#[derive(Debug)]
pub struct Engine(pub String);

impl TryFrom<Option<&str>> for Engine {
    type Error = anyhow::Error;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            Some(s) => Ok(Engine(s.to_string())),
            None => Err(anyhow::anyhow!("missing Engine")),
        }
    }
}

#[derive(Debug)]
pub struct MentionTable {
    pub global_event_id: GlobalEventID,
    pub event_date: DateTime<Utc>,
    pub mention_date: DateTime<Utc>,
    pub mention_type: MentionType,
    pub mention_source_name: MentionSourceName,
    pub sentence_id: SentenceID,
    pub actor_1_char_offset: Option<CharOffset>,
    pub actor_2_char_offset: Option<CharOffset>,
    pub action_char_offset: Option<CharOffset>,
    pub in_raw_text: InRawText,
    pub confidence: Confidence,
    pub mention_doc_len: MentionDocLength,
    pub mention_doc_tone: MentionDocTone,
    pub mention_doc_translation_info: (Option<SourceLanguageCode>, Option<Engine>),
    pub extras: String,
}

impl DatabaseTable for MentionTable {}

impl TryFrom<csv::StringRecord> for MentionTable {
    type Error = anyhow::Error;

    fn try_from(record: csv::StringRecord) -> Result<Self, Self::Error> {
        if record.len() != 16 {
            return Err(anyhow::anyhow!(
                "Expected 16 fields for MentionTable, got {}",
                record.len()
            ));
        }
        let fields: [&str; 16] = record
            .iter()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert record to fixed-size array"))?;

        fn convert_to_utc_datetime(
            date_time_str: &str,
        ) -> anyhow::Result<chrono::DateTime<chrono::Utc>> {
            chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y%m%d%H%M%S")
                .or_else(|_| chrono::NaiveDateTime::parse_from_str(date_time_str, "%Y%m%d"))
                .map(|ndt| chrono::Utc.from_utc_datetime(&ndt))
                .map_err(|e| anyhow::anyhow!("Invalid datetime: {}", e))
        }

        let mention_type_code = MentionTypeCode::try_from(Some(fields[3]))?;
        let mention_identifier = MentionIdentifier::try_from(Some(fields[5]))?;
        let mention_type = MentionType::try_from(Some((mention_type_code, mention_identifier)))?;

        Ok(MentionTable {
            global_event_id: GlobalEventID::try_from(Some(fields[0]))?,
            event_date: convert_to_utc_datetime(fields[1])?,
            mention_date: convert_to_utc_datetime(fields[2])?,
            mention_type,
            mention_source_name: MentionSourceName::try_from(Some(fields[4]))?,
            sentence_id: SentenceID::try_from(Some(fields[6]))?,
            actor_1_char_offset: CharOffset::try_from(Some(fields[7])).ok(),
            actor_2_char_offset: CharOffset::try_from(Some(fields[8])).ok(),
            action_char_offset: CharOffset::try_from(Some(fields[9])).ok(),
            in_raw_text: InRawText::try_from(Some(fields[10]))?,
            confidence: Confidence::try_from(Some(fields[11]))?,
            mention_doc_len: MentionDocLength::try_from(Some(fields[12]))?,
            mention_doc_tone: MentionDocTone::try_from(Some(fields[13]))?,
            mention_doc_translation_info: (
                SourceLanguageCode::try_from(Some(fields[14])).ok(),
                Engine::try_from(Some(fields[15])).ok(),
            ),
            extras: "".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;

    fn make_record(input: &str) -> StringRecord {
        StringRecord::from(input.split('\t').collect::<Vec<_>>())
    }

    #[test]
    fn test_mention_table_try_from_valid_input() {
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_ok(),
            "Failed to create MentionTable from valid input"
        );
        let mention_table = mention_table_result.unwrap();
        assert_eq!(mention_table.global_event_id.0, 1233696063);
        assert_eq!(mention_table.mention_source_name.0, "wyomingnewsnow.tv");
    }

    #[test]
    fn test_mention_table_try_from_invalid_input_length() {
        let input = "1233696063\t20250322164500\t20250322180000";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_err(),
            "Should fail due to invalid input length"
        );
    }

    #[test]
    fn test_mention_table_try_from_invalid_global_event_id() {
        let input = "invalid_id\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_err(),
            "Should fail due to invalid GlobalEventID"
        );
    }

    #[test]
    fn test_mention_table_try_from_invalid_mention_type_code() {
        let input = "1233696063\t20250322164500\t20250322180000\t999\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_err(),
            "Should fail due to invalid MentionTypeCode"
        );
    }

    #[test]
    fn test_mention_table_try_from_invalid_source_language_code() {
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tEN\tEngineName";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_err(),
            "Should fail due to invalid SourceLanguageCode"
        );
    }
}
#[cfg(test)]
mod newtype_tests {
    use super::*;

    #[test]
    fn test_mention_type_code() {
        assert_eq!(MentionTypeCode::try_from(Some("1")).unwrap().0, 1);
        assert!(MentionTypeCode::try_from(Some("bad")).is_err());
        assert!(MentionTypeCode::try_from(None).is_err());
    }

    #[test]
    fn test_mention_source_name() {
        assert_eq!(
            MentionSourceName::try_from(Some("source")).unwrap().0,
            "source"
        );
        assert!(MentionSourceName::try_from(None).is_err());
    }

    #[test]
    fn test_mention_identifier() {
        assert_eq!(
            MentionIdentifier::try_from(Some("id123")).unwrap().0,
            "id123"
        );
        assert!(MentionIdentifier::try_from(None).is_err());
    }

    #[test]
    fn test_sentence_id() {
        assert_eq!(SentenceID::try_from(Some("42")).unwrap().0, 42);
        assert!(SentenceID::try_from(Some("bad")).is_err());
        assert!(SentenceID::try_from(None).is_err());
    }

    #[test]
    fn test_char_offset() {
        assert_eq!(CharOffset::try_from(Some("100")).unwrap().0, 100);
        assert!(CharOffset::try_from(Some("-1")).is_err());
        assert!(CharOffset::try_from(Some("bad")).is_err());
        assert!(CharOffset::try_from(None).is_err());
    }

    #[test]
    fn test_in_raw_text() {
        assert!(InRawText::try_from(Some("1")).unwrap().0);
        assert!(!InRawText::try_from(Some("0")).unwrap().0);
        assert!(InRawText::try_from(Some("bad")).is_err());
        assert!(InRawText::try_from(None).is_err());
    }

    #[test]
    fn test_confidence() {
        assert_eq!(Confidence::try_from(Some("5")).unwrap().0, 5);
        assert!(Confidence::try_from(Some("bad")).is_err());
        assert!(Confidence::try_from(None).is_err());
    }

    #[test]
    fn test_mention_doc_length() {
        assert_eq!(MentionDocLength::try_from(Some("123")).unwrap().0, 123);
        assert!(MentionDocLength::try_from(Some("bad")).is_err());
        assert!(MentionDocLength::try_from(None).is_err());
    }

    #[test]
    fn test_mention_doc_tone() {
        assert_eq!(MentionDocTone::try_from(Some("1.5")).unwrap().0, 1.5);
        assert!(MentionDocTone::try_from(Some("bad")).is_err());
        assert!(MentionDocTone::try_from(None).is_err());
    }

    #[test]
    fn test_source_language_code() {
        assert_eq!(
            SourceLanguageCode::try_from(Some("ENG")).unwrap().0,
            [69, 78, 71]
        );
        assert!(SourceLanguageCode::try_from(Some("EN")).is_err());
        assert!(SourceLanguageCode::try_from(None).is_err());
    }

    #[test]
    fn test_engine() {
        assert_eq!(Engine::try_from(Some("EngineX")).unwrap().0, "EngineX");
        assert!(Engine::try_from(None).is_err());
    }
}
