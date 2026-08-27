//! Main entry point for Hebrew Accent information

use crate::accent::Category;
use crate::accent::Kind;
use crate::accent::WordSpan;
use crate::accent_mark::TraditionNames;
use crate::accent_mark::Utf8CodePoint;

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AccentMetaData {
    /// Official Hebrew name of the accent according to BHS
    pub(crate) hebrew_name: &'static str,
    /// Semantic meaning of the Hebrew term
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according `SLB Simplified`
    pub(crate) english_name: &'static str,
    /// Transliterated according `SLB academic`
    pub(crate) sbl_academic: &'static str,
    /// Optional alternate identifiers for hebrew_name, hebrew_concept, english_name
    pub(crate) alternate_names: Option<AlternateNames>,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Indicates the accent accenttype (Primary, Secondary),
    pub(crate) kind: Kind,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) category: Category,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,
    /// Contextual notes or scholarly commentary
    pub(crate) notes: Option<&'static str>,
    /// Maximum word span of the accent
    pub(crate) word_span: WordSpan,
}

/// Optional alternate representations for an accent.
/// As indicated in the BHS
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AlternateNames {
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according the file `TRANSLITERATION.md`
    pub(crate) english_name: &'static str,
    /// Transliterated according `SLB Academic`
    pub(crate) sbl_academic: &'static str,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePoint,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePoint>,
}
