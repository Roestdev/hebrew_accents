//! Main entry point for Hebrew Accent information
use crate::accent::TraditionNames;
use crate::accent::{
    CantillationSymbol, InternalAccentCategory, InternalAccentKind, InternalAlternateNames,
    InternalCompoundType,
};

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
    pub(crate) alternate_names: Option<InternalAlternateNames>,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Indicates the accent accenttype (Primary, Secondary),
    pub(crate) kind: Option<InternalAccentKind>,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) category: Option<InternalAccentCategory>,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,
    /// Contextual notes or scholarly commentary
    pub(crate) notes: Option<&'static str>,
    /// Compound Type of the accent
    pub(crate) compound_type: Option<InternalCompoundType>,
}

// TODO traditionnames behoren bij een mark
