use chrono::{DateTime, TimeZone, Utc};
use std::fmt;

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

impl fmt::Display for MentionTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Mention for Event #{}", self.global_event_id.0)?;
        writeln!(f, "  Event Date: {}", self.event_date)?;
        writeln!(f, "  Mention Date: {}", self.mention_date)?;
        writeln!(f, "  Mention Type: {:?}", self.mention_type)?;
        writeln!(f, "  Source Name: {:?}", self.mention_source_name)?;
        writeln!(f, "  Sentence ID: {}", self.sentence_id.0)?;

        if let Some(ref offset) = self.actor_1_char_offset {
            writeln!(f, "  Actor 1 Character Offset: {}", offset.0)?;
        }

        if let Some(ref offset) = self.actor_2_char_offset {
            writeln!(f, "  Actor 2 Character Offset: {}", offset.0)?;
        }

        if let Some(ref offset) = self.action_char_offset {
            writeln!(f, "  Action Character Offset: {}", offset.0)?;
        }

        writeln!(f, "  In Raw Text: {}", self.in_raw_text.0)?;
        writeln!(f, "  Confidence: {}", self.confidence.0)?;
        writeln!(f, "  Document Length: {}", self.mention_doc_len.0)?;
        writeln!(f, "  Document Tone: {}", self.mention_doc_tone.0)?;

        let (source_lang, engine) = &self.mention_doc_translation_info;
        if source_lang.is_some() || engine.is_some() {
            writeln!(
                f,
                "  Translation: Source={:?}, Engine={:?}",
                source_lang, engine
            )?;
        }

        if !self.extras.is_empty() {
            writeln!(f, "  Extras: {}", self.extras)?;
        }

        Ok(())
    }
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

        fn parse_translation_info(
            translation_info: &str,
        ) -> (Option<SourceLanguageCode>, Option<Engine>) {
            if translation_info.is_empty() {
                return (None, None);
            }

            let mut source_lang = None;
            let mut engine = None;

            // Parse format like "srclc:fra; eng:Moses 2.1.1 / MosesCore Europarl fr-en / GT-FRA 1.0"
            for part in translation_info.split(';') {
                let part = part.trim();
                if let Some(lang_part) = part.strip_prefix("srclc:") {
                    let lang_code = lang_part.trim();
                    if lang_code.len() == 3 {
                        source_lang = SourceLanguageCode::try_from(Some(lang_code)).ok();
                    }
                } else if let Some(eng_part) = part.strip_prefix("eng:") {
                    let eng_str = eng_part.trim();
                    if !eng_str.is_empty() {
                        engine = Engine::try_from(Some(eng_str)).ok();
                    }
                }
            }

            (source_lang, engine)
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
            mention_doc_translation_info: parse_translation_info(fields[14]),
            extras: fields[15].to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::StringRecord;

    fn init_logger() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = env_logger::try_init();
        });
    }

    fn make_record(input: &str) -> StringRecord {
        StringRecord::from(input.split('\t').collect::<Vec<_>>())
    }

    #[test]
    fn test_debug_mention_table_data_structure() {
        init_logger();

        println!("=== MentionTable Debug Test ===");
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);

        println!("Total fields: {}", record.len());
        println!("First 10 fields:");
        for (i, field) in record.iter().take(10).enumerate() {
            println!("  Field {}: '{}'", i, field);
        }

        // Test parsing
        match MentionTable::try_from(record) {
            Ok(mention) => {
                println!("✅ Parsing successful!");
                println!("Global Event ID: {}", mention.global_event_id.0);
                println!("Source Name: {}", mention.mention_source_name.0);
                println!("Confidence: {}", mention.confidence.0);
            }
            Err(e) => {
                println!("❌ Parsing failed: {}", e);
            }
        }
    }

    #[test]
    fn test_mention_table_try_from_valid_input() {
        init_logger();
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
        init_logger();
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
        init_logger();
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
        init_logger();
        let input = "1233696063\t20250322164500\t20250322180000\t999\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_err(),
            "Should fail due to invalid MentionTypeCode"
        );
    }

    #[test]
    fn test_mention_table_translation_info_parsing() {
        init_logger();
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t1\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tsrclc:eng; eng:TestEngine\tExtras";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        assert!(
            mention_table_result.is_ok(),
            "Should parse successfully with proper translation info format"
        );
        let mention = mention_table_result.unwrap();
        assert_eq!(mention.extras, "Extras");
    }

    #[test]
    fn test_mention_table_try_from_invalid_source_language_code() {
        init_logger();
        // Test with invalid language code (too short)
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t1\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tsrclc:x; eng:TestEngine\tExtras";
        let record = make_record(input);
        let mention_table_result = MentionTable::try_from(record);
        // Should still succeed because invalid language codes are handled gracefully
        // (they just result in None for the language code)
        assert!(
            mention_table_result.is_ok(),
            "Should handle invalid language codes gracefully"
        );

        let mention = mention_table_result.unwrap();
        assert!(
            mention.mention_doc_translation_info.0.is_none(),
            "Should have None for invalid language code"
        );
    }

    #[test]
    fn test_mention_table_try_from_wrong_field_count() {
        init_logger();
        // Test with too few fields
        let record = make_record("field1\tfield2\tfield3");
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with wrong field count");
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Expected 16 fields")
        );

        // Test with too many fields
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName\textra_field";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with too many fields");
    }

    #[test]
    fn test_mention_table_try_from_invalid_dates() {
        init_logger();
        let input = "1233696063\tinvalid_date\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid event date");

        let input = "1233696063\t20250322164500\tinvalid_mention_date\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid mention date");
    }

    #[test]
    fn test_mention_table_try_from_invalid_numeric_fields() {
        init_logger();
        // Invalid sentence ID
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\tnot_number\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid sentence ID");

        // Invalid confidence
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/news/national/turkey-braces-for-fourth-night-of-protests-as-police-quiz-mayor/article_5cf163b7-4383-5dd1-9343-68d3caf61293.html\t8\t-1\t1562\t1620\t0\t20\t3569\tnot_number\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid confidence");
    }

    #[test]
    fn test_mention_table_try_from_invalid_url() {
        init_logger();
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\tinvalid_url\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Should fail with invalid URL");
    }

    #[test]
    fn test_mention_table_mock_data_parsing() {
        init_logger();
        let input1 = "1000000001\t20250322164500\t20250322180000\t1\texample.com\thttps://example.com/article1\t1\t100\t200\t300\t1\t5\t1000\t50\tsrclc:eng; eng:TestEngine\tExtras1";
        let record1 = make_record(input1);
        let result1 = MentionTable::try_from(record1);
        assert!(
            result1.is_ok(),
            "First mock record should parse successfully"
        );
        let mention1 = result1.unwrap();
        assert_eq!(mention1.global_event_id.0, 1000000001);
        assert_eq!(mention1.mention_source_name.0, "example.com");
        assert_eq!(mention1.confidence.0, 5);
        assert_eq!(mention1.extras, "Extras1");

        let input2 = "1000000002\t20250322164600\t20250322180100\t2\ttest.org\thttps://test.org/article2\t2\t-1\t-1\t-1\t0\t200\t500\t25\tsrclc:spa; eng:AnotherEngine\tExtras2";
        let record2 = make_record(input2);
        let result2 = MentionTable::try_from(record2);
        assert!(
            result2.is_ok(),
            "Second mock record should parse successfully"
        );
        let mention2 = result2.unwrap();
        assert_eq!(mention2.global_event_id.0, 1000000002);
        assert_eq!(mention2.confidence.0, 200);
        assert_eq!(mention2.extras, "Extras2");
    }

    #[test]
    fn test_mention_table_edge_cases() {
        init_logger();
        // Test with minimum values
        let input =
            "0\t20250101000000\t20250101000000\t1\tm\thttps://m.co\t0\t-1\t-1\t-1\t0\t0\t0\t0\t\tE";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_ok(), "Minimum values should parse successfully");
        let mention = result.unwrap();
        assert_eq!(mention.global_event_id.0, 0);
        assert_eq!(mention.confidence.0, 0);

        // Test with maximum reasonable values
        let input = "999999999999\t20251231235959\t20251231235959\t6\tveryverylongsourcename.verylongdomain.com\thttps://verylongurlname.verylongdomain.com/very/long/path/to/article\t999999\t99999\t99999\t99999\t1\t255\t999999\t999.999\tsrclc:eng; eng:VeryLongEngineName\tVeryLongExtras";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_ok(), "Maximum values should parse successfully");
        let mention = result.unwrap();
        assert_eq!(mention.global_event_id.0, 999999999999);
        assert_eq!(mention.confidence.0, 255);
    }

    #[test]
    fn test_mention_table_unicode_handling() {
        init_logger();
        let input = "1233696063\t20250322164500\t20250322180000\t1\t测试新闻.com\thttps://测试新闻.com/文章/新闻\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tZHO\t中文引擎";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_ok(), "Unicode content should parse successfully");
        let mention = result.unwrap();
        assert_eq!(mention.mention_source_name.0, "测试新闻.com");
    }

    #[test]
    fn test_mention_table_optional_char_offsets() {
        init_logger();
        // Test with -1 values (indicating missing offsets)
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t-1\t-1\t-1\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(
            result.is_ok(),
            "Missing char offsets should parse successfully"
        );
        let mention = result.unwrap();
        assert!(mention.actor_1_char_offset.is_none());
        assert!(mention.actor_2_char_offset.is_none());
        assert!(mention.action_char_offset.is_none());

        // Test with valid positive offsets
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t100\t200\t300\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(
            result.is_ok(),
            "Valid char offsets should parse successfully"
        );
        let mention = result.unwrap();
        assert!(mention.actor_1_char_offset.is_some());
        assert!(mention.actor_2_char_offset.is_some());
        assert!(mention.action_char_offset.is_some());
        assert_eq!(mention.actor_1_char_offset.unwrap().0, 100);
    }

    #[test]
    fn test_mention_table_different_mention_types() {
        init_logger();
        // Test all valid mention type codes (1-6)
        for mention_type in 1u8..=6u8 {
            let input = format!(
                "1233696063\t20250322164500\t20250322180000\t{}\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tsrclc:eng; eng:EngineName\tExtras",
                mention_type
            );
            let record = make_record(&input);
            let result = MentionTable::try_from(record);
            assert!(
                result.is_ok(),
                "Mention type {} should parse successfully",
                mention_type
            );
        }

        // Test invalid mention type
        let input = "1233696063\t20250322164500\t20250322180000\t99\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(result.is_err(), "Invalid mention type should fail");
    }

    #[test]
    fn test_mention_table_language_codes() {
        init_logger();
        // Test various language codes
        let languages = vec![
            "ENG", "SPA", "FRA", "DEU", "ITA", "POR", "RUS", "ZHO", "JPN", "ARA",
        ];

        for lang in languages {
            let input = format!(
                "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\t{}\tEngineName",
                lang
            );
            let record = make_record(&input);
            let result = MentionTable::try_from(record);
            assert!(
                result.is_ok(),
                "Language code {} should parse successfully",
                lang
            );
        }

        // Test invalid language code (wrong length) - should parse successfully but with None language code
        let input = "1233696063\t20250322164500\t20250322180000\t1\twyomingnewsnow.tv\thttps://www.wyomingnewsnow.tv/article.html\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tEN\tEngineName";
        let record = make_record(input);
        let result = MentionTable::try_from(record);
        assert!(
            result.is_ok(),
            "Should parse successfully even with invalid language code"
        );
        let mention = result.unwrap();
        assert!(
            mention.mention_doc_translation_info.0.is_none(),
            "Invalid language code should result in None"
        );
    }

    #[test]
    fn test_mention_table_performance_simulation() {
        init_logger();
        // Simulate parsing multiple records
        let mut successful_parses = 0;
        for i in 0..100 {
            let input = format!(
                "{}\t20250322164500\t20250322180000\t1\texample{}.com\thttps://example{}.com/article\t8\t-1\t1562\t1620\t0\t20\t3569\t-7.2790294627383\tENG\tEngine{}",
                1000000000 + i,
                i,
                i,
                i
            );
            let record = make_record(&input);
            let result = MentionTable::try_from(record);
            if result.is_ok() {
                successful_parses += 1;
            }
        }

        assert_eq!(
            successful_parses, 100,
            "All 100 simulated records should parse successfully"
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
