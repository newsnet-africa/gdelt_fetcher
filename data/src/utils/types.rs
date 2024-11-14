/// Represents different types of databases.
#[derive(Clone, Debug)]
pub enum DatabaseType {
    /// Database type for export.
    Export,
    /// Database type for mentions.
    Mentions,
    /// Database type for GKG.
    GKG,
}

/// Represents different JSON types in Rust.
#[derive(Clone, Debug)]
pub enum JsonRustTypes {
    /// No JSON type.
    None,
    /// Short JSON type.
    Short,
    /// String JSON type.
    String,
    /// Number JSON type.
    Number,
    /// Boolean JSON type.
    Boolean,
    /// Object JSON type.
    Object,
    /// Array JSON type.
    Array,
}

pub mod api_types {
    pub enum QueryType {
        Quote(String),
        BooleanOR(Vec<String>),
        Exclude(Vec<QueryType>),
        Domain(String),
        DomainIs(String),
        ImageFaceTone(Operation, f32),
        ImageNumFaces(Operation, u8),
        ImageORCMeta(String),
        ImageTags(Vec<String>),
        ImageWebCount(Operation, u128),
        Near {
            distance: u8,
            root_word: String,
            near_word: String,
        },
        Repeat(String),
        SourceCountry(Country),
        SourceLang(Language),
        Theme(GKGTheme),
        Tone(Operation, f32),
        ToneAbs(Operation, f32),
    }

    pub enum OutputMode {
        ArtList,
        ArtGallery,
        ImageCollage,
        ImageCollageInfo,
        ImageGallery,
        ImageCollageShare,
        TimelineVol,
        TimelineVolRow,
        TimelineVolInfo,
        TimelineTone,
        TimelineLang,
        TimelineSourceCountry,
        ToneChart,
        WordCloudImageTags,
        WordCloudImageWebTags,
    }

    pub enum OutputFormat {
        Html,
        CSV,
        RSS(RSSType),
        JSON(JSONType),
    }

    pub enum RSSType {
        Default,
        Archive,
    }

    pub enum JSONType {
        Callback(String),
        Feed,
    }

    pub enum Translator {
        Google,
    }

    pub enum SortType {
        Date(bool),
        Tone(bool),
        HybridRel,
    }
}
