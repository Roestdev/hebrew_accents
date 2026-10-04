//! Main entry point for Hebrew Accent information
use crate::accent_mark::TraditionNames;
use crate::accent_mark::Utf8CodePoint;
use crate::AccentCategory;
use crate::AccentKind;
use crate::CompoundType;

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) struct AccentMetaData {
    /// Official Hebrew name of the accent according to BHS
    pub(crate) hebrew_name: &'static str,
    /// Semantic meaning of the Hebrew term
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according `SBL Simplified`
    pub(crate) english_name: &'static str,
    /// Transliterated according `SBL academic`
    pub(crate) sbl_academic: &'static str,
    /// Optional alternate identifiers for hebrew_name, hebrew_concept, english_name
    /// Only the ones that are noted in the BHS
    pub(crate) alternate_names: Option<PrivAlternateNames>,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Indicates the accent accenttype (Primary, Secondary),
    pub(crate) kind: Option<AccentKind>,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) category: Option<AccentCategory>,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,
    /// Contextual notes or scholarly commentary
    pub(crate) notes: Option<&'static str>,
    /// Compound Type of the accent
    pub(crate) compound_type: Option<CompoundType>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePoint,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePoint>,
}

/// Optional alternate naming for an accent in the BHS
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct PrivAlternateNames {
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according the file `TRANSLITERATION.md`
    pub(crate) english_name: &'static str,
    /// Transliterated according `SBL Academic`
    pub(crate) sbl_academic: &'static str,
}
