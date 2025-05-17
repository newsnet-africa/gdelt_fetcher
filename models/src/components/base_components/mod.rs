use raw_types::CharOffset;
pub mod actor;
pub mod event;

pub mod global_difference_graph;
pub mod global_knowledge_graph;
pub mod location;
pub mod mention;

pub fn parse_slice_to_array<const N: usize>(input: &[u8]) -> [u8; N] {
    let mut output = [0; N]; // Initialize the output array with zeros
    output[..input.len().min(N)].copy_from_slice(&input[..input.len().min(N)]);
    output
}

pub fn chunk_string_to_2d_array<const ROWS: usize, const COLS: usize>(
    input: &str,
) -> [[u8; COLS]; ROWS] {
    let mut output = [[0; COLS]; ROWS]; // Initialize the output array with zeros
    let bytes = input.as_bytes();
    let len = bytes.len();

    // Validate the input length
    if len < 1 || len > ROWS * COLS || len % COLS != 0 {
        panic!(
            "Input string must be between 1 and {} characters and a multiple of {}.",
            ROWS * COLS,
            COLS
        );
    }

    for (i, chunk) in bytes.chunks(COLS).enumerate() {
        if i >= ROWS {
            break; // We only need the first ROWS chunks
        }
        output[i][..chunk.len()].copy_from_slice(chunk);
    }

    output
}

pub fn split_char_offset<const DELIMITER: char>(source: &str) -> (&str, CharOffset) {
    let mut split = source.split(DELIMITER);

    let first = split.next().unwrap_or(""); // Get the first part or an empty string if none
    let second = raw_types::CharOffset::from(split.next_back().unwrap_or_default());

    (first, second)
}

pub mod raw_types {
    use count::{CountValue, RawCountObject};
    #[repr(transparent)]
    pub struct GlobalEventID(pub u128);
    #[repr(transparent)]
    pub struct Day(pub u32);
    #[repr(transparent)]
    pub struct MonthYear(pub u32);
    #[repr(transparent)]
    pub struct Year(pub u16);
    #[repr(transparent)]
    pub struct FractionDate(pub f64);

    #[repr(transparent)]
    pub struct CAMEOActorCode(pub [[u8; 3]; 5]);
    #[repr(transparent)]
    pub struct RawActorName<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct CAMEOCountryCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct CAMEOKnownGroupCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct CAMEOEthnicCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct CAMEOReligionCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct CAMEOActorTypeCode(pub [u8; 3]);

    #[repr(transparent)]
    pub struct IsRootEvent(pub bool);
    #[repr(transparent)]
    pub struct CAMEOEventCode(pub [u8; 4]);
    #[repr(transparent)]
    pub struct CAMEOEventBaseCode(pub [u8; 3]);
    #[repr(transparent)]
    pub struct CAMEOEventRootCode(pub [u8; 2]);
    #[repr(transparent)]
    pub struct RawQuadClass(pub u8);
    #[repr(transparent)]
    pub struct GoldsteinScale(pub f32);
    #[repr(transparent)]
    pub struct NumberOfMentions(pub u128);
    #[repr(transparent)]
    pub struct NumberOfSources(pub u128);
    #[repr(transparent)]
    pub struct NumberOfArticles(pub u128);
    #[repr(transparent)]
    pub struct DateAdded(pub u64);

    pub enum RawSourceUrl<'a> {
        URL(&'a str),
        Citation(&'a str),
    }
    #[repr(transparent)]
    pub struct SentenceID(pub u128);
    #[repr(transparent)]
    pub struct InRawText(pub bool);
    #[repr(transparent)]
    pub struct Confidence(pub f32);
    #[repr(transparent)]
    pub struct DocLength(pub u128);

    #[repr(transparent)]
    pub struct RawGlobalKnowledgeGraphRecordID<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct V2Date(pub u64);
    #[repr(transparent)]
    pub struct V2SourceCollectionIdentifier(pub u8);
    #[repr(transparent)]
    pub struct RawV2SourceCommonName<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawV2DocumentIdentifier<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct CharOffset(pub u128);
    #[repr(transparent)]
    pub struct RawV1Theme<'a>(pub &'a str);
    pub struct RawV2Theme<'a>(pub &'a str, pub CharOffset);
    #[repr(transparent)]
    pub struct RawV1Person<'a>(pub &'a str);
    pub struct RawV2Person<'a>(pub &'a str, pub CharOffset);
    #[repr(transparent)]
    pub struct RawV1Organisation<'a>(pub &'a str);
    pub struct RawV2Organisation<'a>(pub &'a str, pub CharOffset);
    #[repr(transparent)]
    pub struct RawV2SharingImage<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawV2RelatedImage<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawV2SocialMediaEmbed<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawV2SocialVideoEmbed<'a>(pub &'a str);
    pub struct RawV2AllName<'a>(pub &'a str, pub CharOffset);
    pub struct RawV2Amount<'a>(pub CountValue, pub RawCountObject<'a>, pub CharOffset);

    #[repr(transparent)]
    pub struct RawPageURL<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawPageTitle<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawPageDomainFull<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawPageDomainRoot<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawPageLanguage<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawFetchDateOriginal<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawFetchDateCheck<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawHTTPCode<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct HTTPSize(pub u128);
    #[repr(transparent)]
    pub struct RawRedirectURL<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct RawTitleNew<'a>(pub &'a str);
    #[repr(transparent)]
    pub struct NumberOfChanges(pub u128);
    pub enum RawChangeUnit {
        Word,
        Char,
    }
    #[repr(transparent)]
    pub struct FromNumberOfChars(pub u128);
    #[repr(transparent)]
    pub struct ToNumberOfChars(pub u128);
    #[repr(transparent)]
    pub struct FromNumberOfChangedChars(pub u128);
    #[repr(transparent)]
    pub struct ToNumberOfChangedChars(pub u128);
    #[repr(transparent)]
    pub struct TotalChangedChars(pub u128);
    #[repr(transparent)]
    pub struct PercentChangedChars(pub u128);
    pub mod changes {
        pub struct RawChanges<'a>(
            pub RawOriginalTextBlock<'a>,
            pub RawNewTextBlock<'a>,
            pub FromRange,
            pub ToRange,
        );
        #[repr(transparent)]
        pub struct RawOriginalTextBlock<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct RawNewTextBlock<'a>(pub &'a str);
        pub struct FromRange(pub u128, pub u128);
        pub struct ToRange(pub u128, pub u128);
    }

    pub mod extras_xml {
        pub mod cited_reference {
            pub struct RawCitedReference<'a>(
                pub RawAuthor<'a>,
                pub RawTitle<'a>,
                pub RawBookTitle<'a>,
                pub RawDate<'a>,
                pub RawJournal<'a>,
                pub RawVolume<'a>,
                pub RawIssue<'a>,
                pub RawPages<'a>,
                pub RawInstitution<'a>,
                pub RawPublisher<'a>,
                pub RawCiteLocation<'a>,
            );
            #[repr(transparent)]
            pub struct RawAuthor<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawTitle<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawBookTitle<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawDate<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawJournal<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawVolume<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawIssue<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawPages<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawInstitution<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawPublisher<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawCiteLocation<'a>(pub &'a str);
            #[repr(transparent)]
            pub struct RawMarker<'a>(pub &'a str);
        }
    }
    pub mod language {
        pub struct RawTranslationInfo<'a>(
            pub SourceLanguageCode,
            pub (RawTranslationEngine<'a>, RawTranslationModel<'a>),
        );
        #[repr(transparent)]
        pub struct SourceLanguageCode(pub [u8; 3]);
        #[repr(transparent)]
        pub struct RawTranslationEngine<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct RawTranslationModel<'a>(pub &'a str);
    }
    pub mod quotation {
        use super::CharOffset;

        pub struct RawV2Quotation<'a>(
            pub CharOffset,
            pub QuotationLength,
            pub RawQuotationVerb<'a>,
            pub RawQuoteValue<'a>,
        );
        #[repr(transparent)]
        pub struct QuotationLength(pub u128);
        #[repr(transparent)]
        pub struct RawQuotationVerb<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct RawQuoteValue<'a>(pub &'a str);
    }
    pub mod date {
        pub struct V2EnhancedDate(pub DateResolution, pub Month, pub Day, pub Year);
        #[repr(transparent)]
        pub struct DateResolution(pub u8);
        #[repr(transparent)]
        pub struct Month(pub u8);
        #[repr(transparent)]
        pub struct Day(pub u8);
        #[repr(transparent)]
        pub struct Year(pub u16);
    }

    pub mod tone {
        pub struct V1Tone(
            pub ToneValue,
            pub PositiveScore,
            pub NegativeScore,
            pub Polarity,
            pub ActivityReferenceDensity,
            pub PronounReferenceDensity,
            pub WordCount,
        );
        #[repr(transparent)]
        pub struct ToneValue(pub f32); //TODO: Maybe use a crate that shrinks these values cause the range is only from -10 to +10. Do this for all other f32s in this crate I guess
        #[repr(transparent)]
        pub struct PositiveScore(pub f32);
        #[repr(transparent)]
        pub struct NegativeScore(pub f32);
        #[repr(transparent)]
        pub struct Polarity(pub f32);
        #[repr(transparent)]
        pub struct ActivityReferenceDensity(pub f32);
        #[repr(transparent)]
        pub struct PronounReferenceDensity(pub f32);
        #[repr(transparent)]
        pub struct WordCount(pub u128);
    }

    pub mod location {
        use super::CharOffset;

        pub struct RawV1Location<'a>(
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub FIPSCountryCode,
            pub FIPSAdministrationCode,
            pub Latitude,
            pub Longitude,
            pub RawFeatureID<'a>,
        );
        pub struct RawV2Location<'a>(
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub FIPSCountryCode,
            pub FIPSAdministrationCode,
            pub Latitude,
            pub Longitude,
            pub RawFeatureID<'a>,
            pub CharOffset,
        );
        #[repr(transparent)]
        pub struct RawLocationType(pub u8);
        #[repr(transparent)]
        pub struct RawLocationName<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct FIPSCountryCode(pub [u8; 2]);
        pub struct FIPSAdministrationCode(pub FIPSCountryCode, pub [u8; 2]);
        #[repr(transparent)]
        pub struct Latitude(pub f64);
        #[repr(transparent)]
        pub struct Longitude(pub f64);
        pub enum RawFeatureID<'a> {
            String(&'a str),
            Integer(i128),
        }
    }
    pub mod count {
        use super::{
            CharOffset,
            location::{
                FIPSAdministrationCode, FIPSCountryCode, Latitude, Longitude, RawFeatureID,
                RawLocationName, RawLocationType,
            },
        };

        pub struct RawV1Count<'a>(
            pub RawCountType<'a>,
            pub CountValue,
            pub RawCountObject<'a>,
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub FIPSCountryCode,
            pub FIPSAdministrationCode,
            pub Longitude,
            pub Latitude,
            pub RawFeatureID<'a>,
        );
        pub struct RawV2Count<'a>(
            pub RawCountType<'a>,
            pub CountValue,
            pub RawCountObject<'a>,
            pub RawLocationType,
            pub RawLocationName<'a>,
            pub FIPSCountryCode,
            pub FIPSAdministrationCode,
            pub Longitude,
            pub Latitude,
            pub RawFeatureID<'a>,
            pub CharOffset,
        );
        #[repr(transparent)]
        pub struct RawCountType<'a>(pub &'a str);
        #[repr(transparent)]
        pub struct CountValue(pub u128);
        #[repr(transparent)]
        pub struct RawCountObject<'a>(pub &'a str);
    }

    pub mod from_str {
        use super::count::*;
        use super::date::*;
        use super::extras_xml::cited_reference::*;
        use super::language::*;
        use super::location::*;
        use super::quotation::*;
        use super::tone::*;
        use super::*;

        impl From<&str> for CharOffset {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid CharOffset"))
            }
        }

        impl From<&str> for CAMEOActorCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [[0; 3]; 5];
                for (i, chunk) in bytes.chunks(3).enumerate() {
                    code[i][..chunk.len()].copy_from_slice(chunk);
                }
                Self(code)
            }
        }

        impl<'a> From<&'a str> for RawActorName<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl From<&str> for CAMEOCountryCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl<'a> From<&'a str> for RawV2Person<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 2, "Invalid RawV2Person format");
                Self(parts[0], CharOffset::from(parts[1]))
            }
        }

        impl<'a> From<&'a str> for RawV2Amount<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 3, "Invalid RawV2Amount format");
                Self(
                    CountValue(parts[0].parse().expect("Invalid RawCountValue")),
                    RawCountObject(parts[1]),
                    CharOffset::from(parts[2]),
                )
            }
        }

        impl From<&str> for ToneValue {
            fn from(value: &str) -> Self {
                Self(value.parse::<f32>().expect("Invalid RawToneValue"))
            }
        }

        impl From<&str> for V1Tone {
            fn from(value: &str) -> Self {
                let parts: Vec<&str> = value.split(',').collect();
                assert_eq!(parts.len(), 7, "Invalid RawV1Tone format");
                Self(
                    ToneValue(parts[0].parse().expect("Invalid RawToneValue")),
                    PositiveScore(parts[1].parse().expect("Invalid RawPositiveScore")),
                    NegativeScore(parts[2].parse().expect("Invalid RawNegativeScore")),
                    Polarity(parts[3].parse().expect("Invalid RawPolarity")),
                    ActivityReferenceDensity(
                        parts[4]
                            .parse()
                            .expect("Invalid RawActivityReferenceDensity"),
                    ),
                    PronounReferenceDensity(
                        parts[5]
                            .parse()
                            .expect("Invalid RawPronounReferenceDensity"),
                    ),
                    WordCount(parts[6].parse().expect("Invalid RawWordCount")),
                )
            }
        }

        impl From<&str> for CAMEOKnownGroupCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOEthnicCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOReligionCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOActorTypeCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOEventCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 4];
                code[..bytes.len().min(4)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOEventBaseCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for CAMEOEventRootCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 2];
                code[..bytes.len().min(2)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for RawQuadClass {
            fn from(value: &str) -> Self {
                Self(value.parse::<u8>().expect("Invalid RawQuadClass"))
            }
        }

        impl From<&str> for GoldsteinScale {
            fn from(value: &str) -> Self {
                Self(value.parse::<f32>().expect("Invalid RawGoldsteinScale"))
            }
        }

        impl From<&str> for NumberOfMentions {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid RawNumberOfMentions"))
            }
        }

        impl From<&str> for NumberOfSources {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid RawNumberOfSources"))
            }
        }

        impl From<&str> for NumberOfArticles {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid RawNumberOfArticles"))
            }
        }

        impl From<&str> for DateAdded {
            fn from(value: &str) -> Self {
                Self(value.parse::<u64>().expect("Invalid RawDateAdded"))
            }
        }

        impl From<&str> for SentenceID {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid RawSentenceID"))
            }
        }

        impl From<&str> for InRawText {
            fn from(value: &str) -> Self {
                Self(value.parse::<bool>().expect("Invalid RawInRawText"))
            }
        }

        impl From<&str> for Confidence {
            fn from(value: &str) -> Self {
                Self(value.parse::<f32>().expect("Invalid RawConfidence"))
            }
        }

        impl From<&str> for DocLength {
            fn from(value: &str) -> Self {
                Self(value.parse::<u128>().expect("Invalid RawDocLength"))
            }
        }

        impl From<&str> for V2Date {
            fn from(value: &str) -> Self {
                Self(value.parse::<u64>().expect("Invalid RawV2Date"))
            }
        }

        impl From<&str> for V2SourceCollectionIdentifier {
            fn from(value: &str) -> Self {
                Self(
                    value
                        .parse::<u8>()
                        .expect("Invalid RawV2SourceCollectionIdentifier"),
                )
            }
        }

        impl<'a> From<&'a str> for RawV2SourceCommonName<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2DocumentIdentifier<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV1Theme<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2Theme<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 2, "Invalid RawV2Theme format");
                Self(parts[0], CharOffset::from(parts[1]))
            }
        }

        impl<'a> From<&'a str> for RawV1Person<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV1Organisation<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2Organisation<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 2, "Invalid RawV2Organisation format");
                Self(parts[0], CharOffset::from(parts[1]))
            }
        }

        impl<'a> From<&'a str> for RawV2SharingImage<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2RelatedImage<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2SocialMediaEmbed<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2SocialVideoEmbed<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV2AllName<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 2, "Invalid RawV2AllName format");
                Self(parts[0], CharOffset::from(parts[1]))
            }
        }

        impl From<&str> for DateResolution {
            fn from(value: &str) -> Self {
                Self(value.parse::<u8>().expect("Invalid RawDateResolution"))
            }
        }

        impl From<&str> for Month {
            fn from(value: &str) -> Self {
                Self(value.parse::<u8>().expect("Invalid RawMonth"))
            }
        }

        impl From<&str> for super::Day {
            fn from(value: &str) -> Self {
                Self(value.parse::<u32>().expect("Invalid RawDay"))
            }
        }
        impl From<&str> for super::date::Day {
            fn from(value: &str) -> Self {
                Self(value.parse::<u8>().expect("Invalid RawDay"))
            }
        }

        impl From<&str> for super::date::Year {
            fn from(value: &str) -> Self {
                Self(value.parse::<u16>().expect("Invalid RawYear"))
            }
        }
        impl From<&str> for super::Year {
            fn from(value: &str) -> Self {
                Self(value.parse::<u16>().expect("Invalid RawYear"))
            }
        }

        impl From<&str> for V2EnhancedDate {
            fn from(value: &str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 4, "Invalid RawV2EnhancedDate format");
                Self(
                    DateResolution::from(parts[0]),
                    Month::from(parts[1]),
                    super::date::Day::from(parts[2]),
                    super::date::Year::from(parts[3]),
                )
            }
        }

        impl<'a> From<&'a str> for RawPageURL<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawPageTitle<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawPageDomainFull<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawPageDomainRoot<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawPageLanguage<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawFetchDateOriginal<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawFetchDateCheck<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawHTTPCode<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawGlobalKnowledgeGraphRecordID<'a> {
            fn from(value: &'a str) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a str> for RawV1Location<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 9, "Invalid RawV1Location format");
                Self(
                    RawLocationType(parts[0].parse().expect("Invalid RawLocationType")),
                    RawLocationName(parts[1]),
                    FIPSCountryCode::from(parts[2]),
                    FIPSAdministrationCode::from(parts[3]),
                    Latitude(parts[4].parse().expect("Invalid RawLatitude")),
                    Longitude(parts[5].parse().expect("Invalid RawLongitude")),
                    RawFeatureID::String(parts[6]),
                )
            }
        }

        impl<'a> From<&'a str> for RawV2Location<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 10, "Invalid RawV2Location format");
                Self(
                    RawLocationType(parts[0].parse().expect("Invalid RawLocationType")),
                    RawLocationName(parts[1]),
                    FIPSCountryCode::from(parts[2]),
                    FIPSAdministrationCode::from(parts[3]),
                    Latitude(parts[4].parse().expect("Invalid RawLatitude")),
                    Longitude(parts[5].parse().expect("Invalid RawLongitude")),
                    RawFeatureID::String(parts[6]),
                    CharOffset::from(parts[7]),
                )
            }
        }

        impl<'a> From<&'a str> for RawV2Quotation<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 4, "Invalid RawV2Quotation format");
                Self(
                    CharOffset::from(parts[0]),
                    QuotationLength(parts[1].parse().expect("Invalid RawQuotationLength")),
                    RawQuotationVerb(parts[2]),
                    RawQuoteValue(parts[3]),
                )
            }
        }

        impl<'a> From<&'a str> for RawTranslationInfo<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 3, "Invalid RawTranslationInfo format");
                Self(
                    SourceLanguageCode::from(parts[0]),
                    (
                        RawTranslationEngine(parts[1]),
                        RawTranslationModel(parts[2]),
                    ),
                )
            }
        }

        impl<'a> From<&'a str> for RawCitedReference<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 11, "Invalid RawCitedReference format");
                Self(
                    RawAuthor(parts[0]),
                    RawTitle(parts[1]),
                    RawBookTitle(parts[2]),
                    RawDate(parts[3]),
                    RawJournal(parts[4]),
                    RawVolume(parts[5]),
                    RawIssue(parts[6]),
                    RawPages(parts[7]),
                    RawInstitution(parts[8]),
                    RawPublisher(parts[9]),
                    RawCiteLocation(parts[10]),
                )
            }
        }

        impl<'a> From<&'a str> for RawV1Count<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 9, "Invalid RawV1Count format");
                Self(
                    RawCountType(parts[0]),
                    CountValue(parts[1].parse().expect("Invalid RawCountValue")),
                    RawCountObject(parts[2]),
                    RawLocationType(parts[3].parse().expect("Invalid RawLocationType")),
                    RawLocationName(parts[4]),
                    FIPSCountryCode::from(parts[5]),
                    FIPSAdministrationCode::from(parts[6]),
                    Longitude(parts[7].parse().expect("Invalid RawLongitude")),
                    Latitude(parts[8].parse().expect("Invalid RawLatitude")),
                    RawFeatureID::String(parts[9]),
                )
            }
        }

        impl<'a> From<&'a str> for RawV2Count<'a> {
            fn from(value: &'a str) -> Self {
                let parts: Vec<&str> = value.split(';').collect();
                assert_eq!(parts.len(), 10, "Invalid RawV2Count format");
                Self(
                    RawCountType(parts[0]),
                    CountValue(parts[1].parse().expect("Invalid RawCountValue")),
                    RawCountObject(parts[2]),
                    RawLocationType(parts[3].parse().expect("Invalid RawLocationType")),
                    RawLocationName(parts[4]),
                    FIPSCountryCode::from(parts[5]),
                    FIPSAdministrationCode::from(parts[6]),
                    Longitude(parts[7].parse().expect("Invalid RawLongitude")),
                    Latitude(parts[8].parse().expect("Invalid RawLatitude")),
                    RawFeatureID::String(parts[9]),
                    CharOffset::from(parts[10]),
                )
            }
        }

        impl From<&str> for FIPSCountryCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 2];
                code[..bytes.len().min(2)].copy_from_slice(bytes);
                Self(code)
            }
        }

        impl From<&str> for FIPSAdministrationCode {
            fn from(value: &str) -> Self {
                let parts: Vec<&str> = value.split('-').collect();
                assert_eq!(parts.len(), 2, "Invalid RawFIPSAdministrationCode format");
                Self(
                    FIPSCountryCode::from(parts[0]),
                    parts[1]
                        .as_bytes()
                        .try_into()
                        .expect("Invalid administration code"),
                )
            }
        }

        impl From<&str> for SourceLanguageCode {
            fn from(value: &str) -> Self {
                let bytes = value.as_bytes();
                let mut code = [0; 3];
                code[..bytes.len().min(3)].copy_from_slice(bytes);
                Self(code)
            }
        }
    }
}
