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
