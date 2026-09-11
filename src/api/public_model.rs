use crate::accent_mark::CodePointPosition;
use crate::accent_mark::StressPosition;

/// Hebrew Accent kind — (absence is expressed via `Option<T>`)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentKind {
    #[default]
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

/// Hebrew Accent category — (absence is expressed via `Option<T>`)
///
/// # Examples
///
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, AccentCategory};
///
/// // Disjunctive accents return Some(disjunctive)
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.category(), Some(AccentCategory::Disjunctive));
///
/// // Conjunctive accents return Some(conjunctive)
/// let munach: HebrewAccent = ProseAccent::Munach.into();
/// assert_eq!(munach.category(), Some(AccentCategory::Conjunctive));
/// ```
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentCategory {
    #[default]
    /// accents that separate words
    Disjunctive,
    /// accents that connect words
    Conjunctive,
}

/// Disjunctive accent hierarchy group level following Futato's classification system.
///
/// Ranges from Tier1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// In Prose accents, there are 4 group levels.
/// Poetry has 3 levels.
///
/// # Example
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, GroupLevel};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.group_level(), Some(GroupLevel::Tier1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.group_level(), None);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GroupLevel {
    /// Primary disjunctive tier — creates major clause/phrasal breaks
    Tier1 = 1, // value represents group number for extension in future
    /// Secondary disjunctive tier — subordinate phrase boundaries
    Tier2,
    /// Tertiary disjunctive tier — minor phrasal divisions
    Tier3,
    /// Quaternary disjunctive tier — fine-grained subdivisions
    Tier4,
}

impl GroupLevel {
    /// Raw numeric strength value (1 = strongest disjunctive)
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Human-readable description of hierarchy tier
    pub const fn description(self) -> &'static str {
        match self {
            Self::Tier1 => "Primary disjunctive (major clause break)",
            Self::Tier2 => "Secondary disjunctive (phrase boundary)",
            Self::Tier3 => "Tertiary disjunctive (minor division)",
            Self::Tier4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
}

impl std::fmt::Display for GroupLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tier1 => write!(f, "Tier 1 (Primary disjunctive)"),
            Self::Tier2 => write!(f, "Tier 2 (Secondary disjunctive)"),
            Self::Tier3 => write!(f, "Tier 3 (Tertiary disjunctive)"),
            Self::Tier4 => write!(f, "Tier 4 (Quaternary disjunctive)"),
        }
    }
}

impl TryFrom<u8> for GroupLevel {
    type Error = u8;
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            4 => Ok(Self::Tier4),
            _ => Err(v),
        }
    }
}

// TODO better text: add all 2 wordspan accents /accenten met paseq

/// How many consecutive words a single accent sign can cover.
///
/// Almost every ta'am of Scripture is confined to the word it belongs to.
/// Exactly three accents — ʿoleh we-yored, tsinnorit, and maḥpaḥ — are
/// also able to stretch across the interword space and tie two adjacent
/// words together. This enum records that ability: whether the accent is
/// always single-word, or one of the exceptions that may span two.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum WordSpan {
    /// The accent sign is always confined to a single word.
    ///
    /// Every taʿam falls into this category except
    /// ʿoleh we-yored, tsinnorit, and maḥpaḥ.
    #[default]
    OneWord,
    /// The accent sign covers either one word or two consecutive words.
    ///
    /// Only three accents have this capability:
    ///
    /// - **ʿOleh we-yored** (עולה ויורד) — when the two strokes occur
    ///   together, one rises at the end of the first word and the other
    ///   descends onto the beginning of the next.
    /// - **Tsinnorit** (צינורית) — a horizontal line stretched over the
    ///   interword space, like a channel connecting the two words.
    /// - **Maḥpaḥ** (מחפך) — its clasp-shaped sign can reach from the
    ///   first word over to the second.
    OneOrTwoWords,
}

impl WordSpan {
    /// Whether the accent sign can ever cross a word boundary.
    /// Only ʿoleh we-yored, tsinnorit, and maḥpaḥ
    /// ([`WordSpan::OneOrTwoWords`]) can.
    pub const fn can_span_word_boundary(self) -> bool {
        matches!(self, WordSpan::OneOrTwoWords)
    }

    /// The number of words the sign covers, in either the minimum or
    /// maximum case.
    pub const fn word_count_bounds(self) -> (usize, usize) {
        match self {
            WordSpan::OneWord => (1, 1),
            WordSpan::OneOrTwoWords => (1, 2),
        }
    }
}

/// Hebrew Accent wordstress — (absence is expressed via `Option<T>`)
///
/// 'StressPosition', indicating the location of the accent mark relative to
/// the stressed syllable of the word.
///
/// # Important Distinction: Mark Type vs. Word Instance
///
/// **`stress_position` is a FIXED property of each cantillation MARK TYPE**,
/// not a variable property of individual word instances. Each accent type
/// (Pashta, Qadma, Silluq, etc.) has a predetermined stress relationship
/// that never changes, regardless of which word it appears on.
///
/// | Cantonation Mark Type | Fixed `stress_position` | Visual Placement |
/// |----------------------|-------------------------|------------------|
/// | **Qadma** | `Impositive` | On stressed syllable |
/// | **Pashta** | `Postpositive` | On final consonant |
/// | **Silluq** | `Impositive` | On stressed syllable |
/// | **Tevir** | `Impositive` | On stressed syllable |
/// | **Munach** | `Postpositive` | On pre-tonic syllable |
///
/// # How This Handles Shnei Pashtin (Double Pashta)
///
/// When a word has **non-final stress** (stress not on the last syllable)
/// AND requires a Pashta mark, the system creates a **compound accent**:
///
/// 1. **Primary mark**: Pashta (U+05A8) → `stress_position: Impositive` (on stressed syllable)
/// 2. **Secondary mark**: Pashta (U+0599) → `stress_position: Postpositive` (on final consonant)
///
/// This does NOT mean Pashta's `stress_position` varies! Rather:
/// - **Two different Pashta variants** exist in the compound
/// - Each retains its **fixed, type-level** stress position
/// - Together they signal: "word has non-final stress, yet Pashta is required"
///
/// # Example Usage
///
/// ```rust
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, CantillationMarkStressPosition};
///
/// // Silluq always has Impositive (on stressed syllable)
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// match silluq.primary_cantillation_mark().stress_position {
///     Some(CantillationMarkStressPosition::Impositive) => {
///         println!("Silluq sits on the stressed syllable");
///     }
///     _ => unreachable!("Silluq's stress_position never varies by word!")
/// }
///
/// // Pashta always has Postpositive (on final consonant)
/// let pashta = HebrewAccent::Prose(ProseAccent::Pashta);
/// assert_eq!(
///     pashta.primary_cantillation_mark().stress_position,
///     Some(CantillationMarkStressPosition::Postpositive)
/// );
///
/// // Shnei Pashtin (double Pashta) has BOTH positions via compound marks
/// if let Some(shalshelet) = HebrewAccent::Prose(ProseAccent::Shalshelet).secondary_cantillation_mark() {
///     // Primary mark: Impositive (stressed syllable)
///     // Secondary mark: Postpositive OR Paseq separator (depends on accent type)
/// }
/// ```
///
/// # Relationship to `CantillationMarkPlacement`
///
/// Do not confuse `stress_position` with `placement`:
///
/// | Field | Describes | Per-Type or Per-Word? |
/// |-------|-----------|----------------------|
/// | `stress_position` | Syllable relationship | **Fixed by mark type** |
/// | `placement` | Visual position on letter | Fixed by mark type |
///
/// Both are compile-time properties of the accent type, not runtime properties
/// of individual word instances.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CantillationMarkStressPosition {
    #[default]
    /// On the stressed syllable
    /// The accent mark sits directly on the stressed syllable
    Impositive,
    /// Before the stressed syllable
    /// The accent mark is placed on a syllable preceding the stressed one
    Prepositive,
    /// After the stressed syllable
    /// The accent mark is placed on a syllable following the stressed one
    Postpositive,
}

// Conversion from internal to public type
impl From<StressPosition> for Option<CantillationMarkStressPosition> {
    fn from(pos: StressPosition) -> Self {
        use CantillationMarkStressPosition as Public;
        use StressPosition as Internal;

        match pos {
            Internal::Impositive => Some(Public::Impositive),
            Internal::Prepositive => Some(Public::Prepositive),
            Internal::Postpositive => Some(Public::Postpositive),
            Internal::NotApplicable => None,
        }
    }
}

/// Public-facing representation of a cantillation codepoint
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CantillationMark {
    /// The actual Hebrew character/symbol, e.g. "֗"
    pub symbol: char,
    /// Position relative to the consonant
    pub placement: CantillationMarkPlacement,
    /// stress realtion
    pub stress_position: Option<CantillationMarkStressPosition>,
}

/// Complete visual placement decomposed into independent dimensions.
///
/// ## Placement Priority
/// When `special` is `Some(...)`, the `vertical` and `horizontal` fields
/// should be ignored — `special` acts as a complete override.
///
/// ## Usage Patterns
/// - **True cantillation marks**: `special = None`, use `vertical + horizontal`
/// - **Pseudo-accents**: `special = Some(...)`, ignore vertical/horizontal
///
/// Distinguishes from [`CantillationMarkStressPosition`] which describes
/// linguistic relationship to the stressed syllable.
///
/// Note: Not one accent is placed 'BelowLeft'
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum CantillationMarkPlacement {
    /// todo
    #[default]
    AboveCenter,
    /// todo
    AboveLeft,
    /// todo
    AboveRight,
    /// todo
    BelowCenter,
    /// todo
    /// BelowLeft,
    /// todo
    BelowRight,
    /// todo
    Maqqaf,
    /// todo
    SofPasuq,
    /// todo
    Paseq,
}

// Conversion from internal to public type
impl From<CodePointPosition> for CantillationMarkPlacement {
    fn from(pos: CodePointPosition) -> Self {
        use CantillationMarkPlacement as Public;
        use CodePointPosition as Internal;

        match pos {
            Internal::AboveLeft => Public::AboveLeft,
            Internal::AboveCenter => Public::AboveCenter,
            Internal::AboveRight => Public::AboveRight,
            //Internal::BelowLeft => Public::BelowLeft,
            Internal::BelowCenter => Public::BelowCenter,
            Internal::BelowRight => Public::BelowRight,
            Internal::Maqqaf => Public::Maqqaf,
            Internal::SofPasuq => Public::SofPasuq,
            Internal::Paseq => Public::Paseq,
        }
    }
}

// /// Returns the cantilation symbol
// /// May consist of two cantillation marks
// pub fn cantillation_symbol(accent: HebrewAccent) -> String {
//     const GENERIC_MARK_BASE: &str = "\u{25CC}";
//     // get first cantillation_mark
//     // if exist get second cantillation_mark
//     // stel output samen
//     // gebruik generic mark base
//     //
//     let cant1 = accent.primary_cantillation_mark();
//     let cp1 = cant1.unicode_value;
//     if accent.secondary_cantillation_mark().is_some() {
//         let cp2 = accent.secondary_cantillation_mark().unwrap().unicode_value;
//         format!(
//             "{}{}{}{}{}",
//             GENERIC_MARK_BASE, cp1, GENERIC_MARK_BASE, cp2, " "
//         )
//     } else {
//         format!("{}{}{}", GENERIC_MARK_BASE, cp1, " ")
//     }
// }

#[cfg(test)]
mod group_level_tests {
    use super::*;
    #[test]
    fn round_trips_and_epithets() {
        for v in 1..=4u8 {
            let t = GroupLevel::try_from(v).unwrap();
            assert_eq!(t.value(), v);
        }
        assert_eq!(GroupLevel::try_from(5), Err(5));
    }
}
