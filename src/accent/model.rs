//! Main entry point for Hebrew Accent information


// Crate‑internal (local modules)
use crate::accent::public_model::AccentCategory;
use crate::accent::public_model::AccentKind;
use crate::accent::public_model::AccentWordStress;
use crate::accent::public_model::GroupLevel;
use crate::accent::HebrewAccent;
use crate::accent::PoetryAccent;
use crate::accent::ProseAccent;
use crate::codepoints::{TraditionNames,CodePointPosition};

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AccentInformation {
    /// Official Hebrew name of the accent according to BHS
    pub(crate) hebrew_name: &'static str,
    /// Semantic meaning of the Hebrew term.
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated English name of the accent.
    pub(crate) english_name: &'static str,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Optional alternate identifiers for hebrew_name, hebrew_concept, english_name
    pub(crate) alternate_names: Option<AlternateNames>,
    /// Indicates the accent accenttype (Primary, Secondary)
    pub(crate) kind: Kind,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) accent_category: Category,
    /// Indicates if the accent is on the stressed syllable
    pub(crate) word_stress: WordStress,
    /// Contextual notes or scholarly commentary.
    pub(crate) notes: Option<&'static str>,
}

/// Optional alternate representations for an accent.
/// Used by some scholars
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AlternateNames {
    /// Transliterated English name for the hebrew name
    pub(crate) english_name: &'static str,
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
}

/// Struct containing the cantillation symbol of a Hebrew Accent
/// Which may consist of one or (max) two cantillation marks
/// One cantilation mark is one UTF8 code point
/// A cantillation mark is a diacritical symbol attached to a letter or a word. 
/// It is characterised by its shape and its position relative to the letter or 
/// word it is attached to.
/// 
/// A cantillation symbol consists of one or two cantillation marks. 
/// It is characterised by its meaning and by the rules for its usage in a given context.

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePointInfo,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePointInfo>,
}

/// Details on a specific UTF-8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Utf8CodePointInfo {
    /// UTF-8 code-point id, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_value: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub(crate) name: &'static str,
    /// The symbol of the UTF-8 code-point
    pub(crate) symbol: &'static str,
    /// The position of the code-point in relation to the consonant
    pub(crate) position: CodePointPosition,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,  // ← Changed from &'static [Tradition]
}

/// Hebrew Accent category (either Conjunctive or Disjunctive)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Category {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
    /// Only applicable for PseudoAccent's
    None,
}

impl Category {
    pub(crate) const fn to_public(self) -> Option<AccentCategory> {
        match self {
            Category::Conjunctive => Some(AccentCategory::Conjunctive),
            Category::Disjunctive => Some(AccentCategory::Disjunctive),
            Category::None => None,
        }
    }
}

/// Internal type — has None variant for pseudo accents
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Kind {
    #[default]
    Primary,
    Secondary,
    None,
}

impl Kind {
    pub(crate) const fn to_public(self) -> Option<super::public_model::AccentKind> {
        match self {
            Kind::Primary => Some(AccentKind::Primary),
            Kind::Secondary => Some(AccentKind::Secondary),
            Kind::None => None,
        }
    }
}

impl WordStress {
    pub(crate) const fn to_public(self) -> Option<AccentWordStress> {
        match self {
            WordStress::Im => Some(AccentWordStress::ImPositive),
            WordStress::Post => Some(AccentWordStress::PostPositive),
            WordStress::Pre => Some(AccentWordStress::PrePositive),
            WordStress::None => None,
        }
    }
}

/// WordStress, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum WordStress {
    #[default]
    /// ImPositive: The accent is located above the stressed syllable
    Im,
    /// PostPositive: The accent is NOT located above the stressed syllable, but at the very end of the word
    Post,
    /// PrePositive: Accent is NOT located above the stressed syllable, but at the very beginning of the word
    Pre,
    /// internal use only
    None,
}

/// Full Futato hierarchy classification with prose/poetry distinction.
///
/// **Internal use only**—do not rely on this accenttype publicly as it may change
/// without semver warning. Use [`super::GroupLevel`] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum DisjunctiveGroup {
    ProseLevel1, // Fixed variants, no number needed
    ProseLevel2,
    ProseLevel3,
    ProseLevel4,
    PoetryLevel1,
    PoetryLevel2,
    PoetryLevel3, // Max tier differs between systems
}

impl DisjunctiveGroup {
    /// Convert to simplified public GroupLevel
    pub(crate) const fn into_public_level(self) -> Option<GroupLevel> {
        match self {
            Self::ProseLevel1 | Self::PoetryLevel1 => Some(GroupLevel::Level1),
            Self::ProseLevel2 | Self::PoetryLevel2 => Some(GroupLevel::Level2),
            Self::ProseLevel3 | Self::PoetryLevel3 => Some(GroupLevel::Level3),
            Self::ProseLevel4 => Some(GroupLevel::Level4),
        }
    }
}

/// Lookup logic for accent hierarchy (private function)
pub(crate) fn resolve_disjunctive_group(accent: HebrewAccent) -> Option<DisjunctiveGroup> {
    match accent {
        HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Prose(ProseAccent::Atnach) => {
            Some(DisjunctiveGroup::ProseLevel1)
        }

        HebrewAccent::Prose(ProseAccent::Segolta)
        | HebrewAccent::Prose(ProseAccent::Shalshelet)
        | HebrewAccent::Prose(ProseAccent::ZaqephQatan)
        | HebrewAccent::Prose(ProseAccent::ZaqephGadol)
        | HebrewAccent::Prose(ProseAccent::Tiphcha) => Some(DisjunctiveGroup::ProseLevel2),

        HebrewAccent::Prose(ProseAccent::Revia)
        | HebrewAccent::Prose(ProseAccent::Zarqa)
        | HebrewAccent::Prose(ProseAccent::Pashta)
        | HebrewAccent::Prose(ProseAccent::Tevir)
        | HebrewAccent::Prose(ProseAccent::Yetiv) => Some(DisjunctiveGroup::ProseLevel3),

        HebrewAccent::Prose(ProseAccent::Geresh)
        | HebrewAccent::Prose(ProseAccent::Gershayim)
        | HebrewAccent::Prose(ProseAccent::Pazer)
        | HebrewAccent::Prose(ProseAccent::PazerGadol)
        | HebrewAccent::Prose(ProseAccent::TelishaGedolah)
        | HebrewAccent::Prose(ProseAccent::Legarmeh) => Some(DisjunctiveGroup::ProseLevel4),

        HebrewAccent::Poetry(PoetryAccent::Silluq)
        | HebrewAccent::Poetry(PoetryAccent::OlehWeYored)
        | HebrewAccent::Poetry(PoetryAccent::Atnach) => Some(DisjunctiveGroup::PoetryLevel1),

        HebrewAccent::Poetry(PoetryAccent::ReviaGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)
        | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaQaton)
        | HebrewAccent::Poetry(PoetryAccent::Tsinnor)
        | HebrewAccent::Poetry(PoetryAccent::Dechi) => Some(DisjunctiveGroup::PoetryLevel2),

        HebrewAccent::Poetry(PoetryAccent::Pazer)
        | HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
        | HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => Some(DisjunctiveGroup::PoetryLevel3),
        _ => None, // conjunctives and pseudo-accents lack hierarchy
    }
}

