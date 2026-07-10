use crate::accent::Accent;

/// Syntactic markers associated with biblical Hebrew cantillation but distinct from true accents.
///
/// `PseudoAccent` values represent structural symbols that influence accent placement without
/// carrying independent melodic contour. They govern phrase boundaries, word grouping, and
/// punctuation within the Masoretic text tradition.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PseudoAccent {
    #[default]
    /// Marks the end of a verse or sentence (Hebrew: סוֹף פָּסוּק).
    /// Equivalent to a terminal period; signals final pause despite lacking its own melody.
    ///
    /// **Note:** Contrary to intuition, `Silluq` and not `SophPasuq` designates official verse endings
    /// in standard BHS texts. `SophPasuq` may be absent even at valid verse boundaries in some rare cases.
    SophPasuq,

    /// Joins multiple words into a single phonological unit (Hebrew: מַקָּף).
    /// Functions as a hyphen: suppresses independent accents on joined words, causing
    /// accent shifts to the rightmost constituent.
    Maqqeph,

    /// Separates adjacent cantillation marks (Hebrew: פָּשְׁק).
    /// Prevents conflation of neighboring accents where disambiguation is required;
    /// never appears as an standalone accent.
    Paseq,
}

impl PseudoAccent {
    /// Total count of all pseudo accents
    pub const LEN: usize = 3;
    /// Indicates a level of importance
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

impl std::fmt::Display for PseudoAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}), meaning: {}",
            self.english_name(),
            self.hebrew_name(),
            self.hebrew_concept()
        )
    }
}
