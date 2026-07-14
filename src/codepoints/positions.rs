/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum CodePointPosition {
    /// UTF-8 code point is located above the consonant
    Above,
    /// UTF-8 code point is located after the consonant
    /// Used for punctuation marks that follow the word boundary
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
    /// UTF-8 code point is located in between two words
    /// Used for separators between two words/clauses
    InBetween,
    /// UTF-8 code point is located under the consonant
    #[default]
    Under,
}
