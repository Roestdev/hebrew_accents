/// Describes the context of a sentence (poetic or prosaic)
///
/// Default value is Prosaic
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Context {
    /// The sentence follows a poetic structure (e.g., Psalms, Job, Proverbs).
    /// Typically characterized by parallelism, meter, and specific poetry-exclusive accents
    /// such as Oleh WeYored, Dechi, Illuy, Tsinnorit Merkha/Mahpakh.
    Poetic,

    /// The sentence follows ordinary prose conventions (e.g., Genesis, Exodus narrative).
    /// Characterized by prose-exclusive accents such as Segolta, Zaqeph Qatan/Gadol,
    /// Pashta, Tevir, Yetiv, etc. Used as the default when context cannot be determined.
    #[default]
    Prosaic,
}
