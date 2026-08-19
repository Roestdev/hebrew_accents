/// Canonical position representation for Hebrew cantillation marks.
///
/// Encodes both spatial positions (above/below × left/center/right)
/// and special semantic markers (Maqqaf, Sof Pasuq, Paseq).
///
/// ## Usage
/// - **Grid positions (0-5)**: Standard placement relative to base consonant
/// - **Special positions (6-8)**: Semantic overrides that bypass normal grid
///
/// All values are compile-time constants in static registries.
/// No heap allocation or runtime construction required.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[repr(u8)]
pub(crate) enum CodePointPosition {
    /// Above the baseline, left of the consonant
    AboveLeft = 0,
    /// Above the baseline, centered on the consonant
    #[default]
    AboveCenter = 1,
    /// Above the baseline, right of the consonant
    AboveRight = 2,
    // Below the baseline, left of the consonant
    // BelowLeft = 3,
    /// Below the baseline, centered on the consonant
    BelowCenter = 4,
    /// Below the baseline, right of the consonant
    BelowRight = 5,
    /// Inter-word connector (Maqqaf — ֿ)
    Maqqaf = 6,
    /// End-of-verse marker (Sof Pasuq — ׃)
    SofPasuq = 7,
    /// Visual separator (Paseq — ֊)
    Paseq = 8,
}
