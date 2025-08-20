use serde::{Deserialize, Serialize};
use std::fmt;

/// Language codes used in GCAM data
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "eng")]
    English,
    #[serde(rename = "ara")]
    Arabic,
    #[serde(rename = "chi")]
    Chinese,
    #[serde(rename = "fra")]
    French,
    #[serde(rename = "ger")]
    German,
    #[serde(rename = "hin")]
    Hindi,
    #[serde(rename = "jpn")]
    Japanese,
    #[serde(rename = "por")]
    Portuguese,
    #[serde(rename = "rus")]
    Russian,
    #[serde(rename = "spa")]
    Spanish,
    Other(String),
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::English => write!(f, "eng"),
            Language::Arabic => write!(f, "ara"),
            Language::Chinese => write!(f, "chi"),
            Language::French => write!(f, "fra"),
            Language::German => write!(f, "ger"),
            Language::Hindi => write!(f, "hin"),
            Language::Japanese => write!(f, "jpn"),
            Language::Portuguese => write!(f, "por"),
            Language::Russian => write!(f, "rus"),
            Language::Spanish => write!(f, "spa"),
            Language::Other(code) => write!(f, "{}", code),
        }
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "eng" => Language::English,
            "ara" => Language::Arabic,
            "chi" => Language::Chinese,
            "fra" => Language::French,
            "ger" => Language::German,
            "hin" => Language::Hindi,
            "jpn" => Language::Japanese,
            "por" => Language::Portuguese,
            "rus" => Language::Russian,
            "spa" => Language::Spanish,
            _ => Language::Other(s.to_string()),
        }
    }
}

/// Dictionary names from the GCAM Master Codebook
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Dictionary {
    ForestValues,
    GdeltGlobalKnowledgeGraphThemes,
    GdeltGkgThemes,
    GeneralInquirerV102,
    LexicoderSentimentDictionary,
    LexicoderTopicDictionaries,
    LinguisticInquiryAndWordCount,
    LoughranAndMcDonaldFinancialSentiment,
    OpinionObserver,
    RegressiveImageryDictionary,
    RogetsThesaurus1911Edition,
    SentiWordNet30,
    SentiWords,
    SubjectivityLexicon,
    BodyBoundaryDictionary,
    WordNetAffect10,
    WordNetAffect11,
    WordNetDomains32,
    WordNet31LexicalCategories,
    /// Catch-all for any other dictionary not explicitly defined
    Other(String),
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dictionary::ForestValues => write!(f, "Forest Values"),
            Dictionary::GdeltGlobalKnowledgeGraphThemes => {
                write!(f, "GDELT Global Knowledge Graph Themes")
            }
            Dictionary::GdeltGkgThemes => write!(f, "GDELT GKG Themes"),
            Dictionary::GeneralInquirerV102 => write!(
                f,
                "General Inquirer V1.02 (Harvard IV-4 Psychosocial Dictionary / NamenWirth & Weber's Lasswell Dictionary)"
            ),
            Dictionary::LexicoderSentimentDictionary => write!(f, "Lexicoder Sentiment Dictionary"),
            Dictionary::LexicoderTopicDictionaries => write!(f, "Lexicoder Topic Dictionaries"),
            Dictionary::LinguisticInquiryAndWordCount => {
                write!(f, "Linguistic Inquiry and Word Count (LIWC)")
            }
            Dictionary::LoughranAndMcDonaldFinancialSentiment => {
                write!(f, "Loughran and McDonald Financial Sentiment Dictionaries")
            }
            Dictionary::OpinionObserver => write!(f, "Opinion Observer"),
            Dictionary::RegressiveImageryDictionary => write!(f, "Regressive Imagery Dictionary"),
            Dictionary::RogetsThesaurus1911Edition => write!(f, "Roget's Thesaurus 1911 Edition"),
            Dictionary::SentiWordNet30 => write!(f, "SentiWordNet 3.0"),
            Dictionary::SentiWords => write!(f, "SentiWords"),
            Dictionary::SubjectivityLexicon => write!(f, "Subjectivity Lexicon"),
            Dictionary::BodyBoundaryDictionary => write!(f, "Body Boundary Dictionary"),
            Dictionary::WordNetAffect10 => write!(f, "WordNet Affect 1.0"),
            Dictionary::WordNetAffect11 => write!(f, "WordNet Affect 1.1"),
            Dictionary::WordNetDomains32 => write!(f, "WordNet Domains 3.2"),
            Dictionary::WordNet31LexicalCategories => write!(f, "WordNet 3.1 Lexical Categories"),
            Dictionary::Other(name) => write!(f, "{}", name),
        }
    }
}

impl Dictionary {
    pub fn from_human_name(name: &str) -> Self {
        match name {
            "Forest Values" => Dictionary::ForestValues,
            "GDELT Global Knowledge Graph Themes" => Dictionary::GdeltGlobalKnowledgeGraphThemes,
            "GDELT GKG Themes" => Dictionary::GdeltGkgThemes,
            name if name.starts_with("General Inquirer V1.02") => Dictionary::GeneralInquirerV102,
            "Lexicoder Sentiment Dictionary" => Dictionary::LexicoderSentimentDictionary,
            "Lexicoder Topic Dictionaries" => Dictionary::LexicoderTopicDictionaries,
            "Linguistic Inquiry and Word Count (LIWC)" => Dictionary::LinguisticInquiryAndWordCount,
            "Loughran and McDonald Financial Sentiment Dictionaries" => {
                Dictionary::LoughranAndMcDonaldFinancialSentiment
            }
            "Opinion Observer" => Dictionary::OpinionObserver,
            "Regressive Imagery Dictionary" => Dictionary::RegressiveImageryDictionary,
            "Roget's Thesaurus 1911 Edition" => Dictionary::RogetsThesaurus1911Edition,
            "SentiWordNet 3.0" => Dictionary::SentiWordNet30,
            "SentiWords" => Dictionary::SentiWords,
            "Subjectivity Lexicon" => Dictionary::SubjectivityLexicon,
            "Body Boundary Dictionary" => Dictionary::BodyBoundaryDictionary,
            "WordNet Affect 1.0" => Dictionary::WordNetAffect10,
            "WordNet Affect 1.1" => Dictionary::WordNetAffect11,
            "WordNet Domains 3.2" => Dictionary::WordNetDomains32,
            "WordNet 3.1 Lexical Categories" => Dictionary::WordNet31LexicalCategories,
            // Handle all other dictionaries without warning
            _ => Dictionary::Other(name.to_string()),
        }
    }

    pub fn get_citation(&self) -> &'static str {
        match self {
            Dictionary::ForestValues => {
                "Bengston, D, & Xu, Z. (1995). Changing national forest values: A content analysis. St. Paul, Minn.: North Central Forest Experiment Station, Forest Service, U.S. Dept. of Agriculture."
            }
            Dictionary::GdeltGlobalKnowledgeGraphThemes => {
                "Kalev Hannes Leetaru. (2013). 'The GDELT Global Knowledge Graph (GKG)'. Available http://gdeltproject.org/"
            }
            Dictionary::GdeltGkgThemes => {
                "Kalev Hannes Leetaru. (2013). 'The GDELT Global Knowledge Graph (GKG)'. Available http://gdeltproject.org/"
            }
            Dictionary::GeneralInquirerV102 => {
                "Philip J. Stone, Robert F. Bales, Zvi Namenwirth, & Daniel M. Ogilvie (1962). The General Inquirer: A computer system for content analysis and retrieval based on the sentence as a unit of information. Behavioral Science, 7(4), 484-498"
            }
            Dictionary::LexicoderSentimentDictionary => {
                "Lori Young and Stuart Soroka. 2012. Affective News: The Automated Coding of Sentiment in Political Texts, Political Communication 29: 205-231. Available at http://lexicoder.com/"
            }
            Dictionary::LexicoderTopicDictionaries => {
                "Albugh, Quinn, Julie Sevenans and Stuart Soroka. 2013. Lexicoder Topic Dictionaries, June 2013 versions, McGill University, Montreal, Canada. Available at http://lexicoder.com/"
            }
            Dictionary::LinguisticInquiryAndWordCount => {
                "Pennebaker, J. W., Booth, R. J., & Francis, M. E. (2007). Linguistic Inquiry and Word Count: LIWC [Computer software]. Austin, TX. Available at http://www.liwc.net/"
            }
            Dictionary::LoughranAndMcDonaldFinancialSentiment => {
                "Tim Loughran and Bill McDonald, 2011, \"When is a Liability not a Liability,\" Journal of Finance, V66, pp. 35-65."
            }
            Dictionary::OpinionObserver => {
                "Bing Liu, Minqing Hu and Junsheng Cheng. \"Opinion Observer: Analyzing and Comparing Opinions on the Web.\" Proceedings of the 14th International World Wide Web conference (WWW-2005), May 10-14, 2005, Chiba, Japan."
            }
            Dictionary::RegressiveImageryDictionary => {
                "Martindale C. (1987). Narrative pattern analysis: A quantitative method for inferring the symbolic meaning of narratives. In Literary discourse: Aspects of cognitive and social psychological approaches Halasz L. (ed) pp167–181, Berlin: de Gruyter"
            }
            Dictionary::RogetsThesaurus1911Edition => {
                "Peter Mark Roget. (1911). Roget's Thesaurus of English Words and Phrases. New York: TY Crowell Company."
            }
            Dictionary::SentiWordNet30 => {
                "Andrea Esuli Stefano Baccianella and Fabrizio Sebastiani. (2010). Sentiwordnet 3.0: An enhanced lexical resource for sentiment analysis and opinion mining. In LREC."
            }
            Dictionary::SentiWords => {
                "Guerini M., Gatti L. & Turchi M. \"Sentiment Analysis: How to Derive Prior Polarities from SentiWordNet\". In Proceedings of the 2013 Conference on Empirical Methods in Natural Language Processing (EMNLP'13), pp 1259-1269. Seattle, Washington, USA. 2013."
            }
            Dictionary::SubjectivityLexicon => {
                "Theresa Wilson, Janyce Wiebe, and Paul Hoffmann (2005). Recognizing Contextual Polarity in Phrase-Level Sentiment Analysis. Proc. of HLT-EMNLP-2005."
            }
            Dictionary::BodyBoundaryDictionary => {
                "Andrew Wilson. (2006). Development and application of a content analysis dictionary for body boundary research. Literary and Linguistic Computing, 21, 105-110."
            }
            Dictionary::WordNetAffect10 => {
                "Carlo Strapparava and Alessandro Valitutti. \"WordNet-Affect: an Affective Extension of WordNet\", in Proceedings of the 4th International Conference on Language Resources and Evaluation (LREC 2004), Lisbon, May 2004, pp. 1083-1086."
            }
            Dictionary::WordNetAffect11 => {
                "Carlo Strapparava and Alessandro Valitutti. \"WordNet-Affect: an Affective Extension of WordNet\", in Proceedings of the 4th International Conference on Language Resources and Evaluation (LREC 2004), Lisbon, May 2004, pp. 1083-1086."
            }
            Dictionary::WordNetDomains32 => {
                "Bernardo Magnini and Gabriela Cavaglià. \"Integrating Subject Field Codes into WordNet\". In Gavrilidou M., Crayannis G., Markantonatu S., Piperidis S. and Stainhaouer G. (Eds.) Proceedings of LREC-2000, Second International Conference on Language Resources and Evaluation, Athens, Greece, 31 May – 2 June, 2000, pp. 1413-1418."
            }
            Dictionary::WordNet31LexicalCategories => {
                "George A. Miller (1995). WordNet: A Lexical Database for English. Communications of the ACM Vol. 38, No. 11: 39-41."
            }
            Dictionary::Other(_) => "Citation not available for this dictionary.",
        }
    }
}

/// Type of measurement for the GCAM entry
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeasurementType {
    #[serde(rename = "WORDCOUNT")]
    WordCount,
    #[serde(rename = "RATIO")]
    Ratio,
    #[serde(rename = "SCORE")]
    Score,
    Other(String),
}

impl fmt::Display for MeasurementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeasurementType::WordCount => write!(f, "WORDCOUNT"),
            MeasurementType::Ratio => write!(f, "RATIO"),
            MeasurementType::Score => write!(f, "SCORE"),
            MeasurementType::Other(t) => write!(f, "{}", t),
        }
    }
}

impl From<&str> for MeasurementType {
    fn from(s: &str) -> Self {
        match s {
            "WORDCOUNT" => MeasurementType::WordCount,
            "RATIO" => MeasurementType::Ratio,
            "SCORE" => MeasurementType::Score,
            _ => MeasurementType::Other(s.to_string()),
        }
    }
}

/// Enriched GCAM entry structure based on the GCAM Master Codebook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GCAMCodebookEntry {
    /// Variable identifier (e.g., "c1.1")
    pub variable: String,
    /// Dictionary ID (numeric identifier)
    pub dictionary_id: u32,
    /// Dimension ID within the dictionary
    pub dimension_id: u32,
    /// Type of measurement
    pub measurement_type: MeasurementType,
    /// Language code
    pub language: Language,
    /// Human-readable dictionary name
    pub dictionary: Dictionary,
    /// Human-readable dimension name
    pub dimension_name: String,
    /// Citation for the dictionary
    pub citation: String,
}

impl GCAMCodebookEntry {
    pub fn new(
        variable: String,
        dictionary_id: u32,
        dimension_id: u32,
        measurement_type: MeasurementType,
        language: Language,
        dictionary: Dictionary,
        dimension_name: String,
        citation: String,
    ) -> Self {
        Self {
            variable,
            dictionary_id,
            dimension_id,
            measurement_type,
            language,
            dictionary,
            dimension_name,
            citation,
        }
    }
}

/// Enhanced GCAM entry that combines the original key-value with enriched metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnrichedGCAMEntry {
    /// Original GCAM key
    pub key: String,
    /// Original GCAM value
    pub value: f32,
    /// Enriched metadata from codebook (if available)
    pub metadata: Option<GCAMCodebookEntry>,
}

impl EnrichedGCAMEntry {
    pub fn new(key: String, value: f32, metadata: Option<GCAMCodebookEntry>) -> Self {
        Self {
            key,
            value,
            metadata,
        }
    }

    /// Create from original GCAMEntry without metadata
    pub fn from_simple(key: String, value: f32) -> Self {
        Self {
            key,
            value,
            metadata: None,
        }
    }

    /// Get the dictionary name if metadata is available
    pub fn dictionary_name(&self) -> Option<String> {
        self.metadata.as_ref().map(|m| m.dictionary.to_string())
    }

    /// Get the dimension name if metadata is available
    pub fn dimension_name(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.dimension_name.as_str())
    }

    /// Get the citation if metadata is available
    pub fn citation(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.citation.as_str())
    }

    /// Serialize to bytes using bincode
    pub fn to_bytes(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }

    /// Deserialize from bytes using bincode
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}
