use std::fmt;

use crate::accent_mark::CodePointPosition;
use crate::accent_mark::StressPosition;

/// Hebrew Accent kind — (absence is expressed via `Option<T>`)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentKind {
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

impl fmt::Display for AccentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Secondary => write!(f, "secondary"),
        }
    }
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AccentCategory {
    /// accents that separate words
    Disjunctive,
    /// accents that connect words
    Conjunctive,
}

impl fmt::Display for AccentCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disjunctive => write!(f, "disjunctive"),
            Self::Conjunctive => write!(f, "conjunctive"),
        }
    }
}
/// **Disjunctive accent** hierarchy group level following Futato's classification system.
///
/// Ranges from Tier1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// Note:
///   In Prose accents, there are 4 group levels.
///   Poetry has 3 levels.
///
/// # Example
/// ```rust
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, GroupLevel};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.group_level(), Some(GroupLevel::Tier1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.group_level(), None);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    /// Human-readable description of hierarchy tier groups
    pub const fn description(self) -> &'static str {
        match self {
            Self::Tier1 => "Primary disjunctive (major clause break)",
            Self::Tier2 => "Secondary disjunctive (phrase boundary)",
            Self::Tier3 => "Tertiary disjunctive (minor division)",
            Self::Tier4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
    /// Human-readable description of hierarchy tier groups
    ///
    /// In the literature other names are sometimes used
    pub const fn alt_description(self) -> &'static str {
        match self {
            Self::Tier1 => "Emperor",
            Self::Tier2 => "Kings",
            Self::Tier3 => "Dukes",
            Self::Tier4 => "Officers",
        }
    }
    /// Returns the maximum tier available for poetry accents.
    /// Poetry accents stop at Tier3; prose can reach Tier4.
    pub const fn max_poetry_tier() -> Self {
        Self::Tier3
    }
    /// Returns the maximum tier available for prose accents.
    pub const fn max_prose_tier() -> Self {
        Self::Tier4
    }
    /// Checks if this tier is available in poetry context.
    pub const fn is_poetry_available(self) -> bool {
        matches!(self, Self::Tier1 | Self::Tier2 | Self::Tier3)
    }
}

impl fmt::Display for GroupLevel {
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
    type Error = crate::GroupLevelError; // ← Use shared error type

    fn try_from(raw_value: u8) -> Result<Self, Self::Error> {
        match raw_value {
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            4 => Ok(Self::Tier4),
            _ => Err(crate::GroupLevelError::InvalidGroupLevel(raw_value)),
        }
    }
}

/// Public-facing representation of a cantillation mark
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct CantillationMark {
    /// The actual Hebrew character/symbol, e.g. "֗"
    pub symbol: char,
    /// Position relative to the consonant
    pub placement: CantillationMarkPlacement,
    /// Stress relation
    pub stress_position: Option<CantillationMarkStressPosition>,
}

/// Placement of the cantillation mark related to the consonant
///
/// - **True cantillation marks**: have a `vertical + horizontal` component, e.g. AboveLeft
/// - **Pseudo-accents**: can not be expressed in `vertical + horizontal` components
///
/// Distinguishes from [`CantillationMarkStressPosition`] which describes
/// linguistic relationship to the stressed syllable.
///
/// **Note:** Linguistically, no Te'amim exist for BelowLeft placement.
/// See: <https://en.wikipedia.org/wiki/Te'amim>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CantillationMarkPlacement {
    // Above placements
    /// The cantillation mark is placed above the consonant in the center
    AboveCenter,
    /// The cantillation mark is placed above the consonant at the left side
    AboveLeft,
    /// The cantillation mark is placed above the consonant at the right side
    AboveRight,

    // Below placements
    /// The cantillation mark is placed below the consonant in the center
    BelowCenter,
    /// The cantillation mark is placed below the consonant at the right side
    BelowRight,

    // Between-word/special placements
    /// The cantillation mark is placed after the last word in the verse
    SofPasuq,
    /// The cantillation mark is placed between two words
    Maqqaph,
    /// The cantillation mark is placed at the left side of a word
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
            Internal::SofPasuq => Public::SofPasuq,
            Internal::Maqqaph => Public::Maqqaph,
            Internal::Paseq => Public::Paseq,
        }
    }
}

impl fmt::Display for CantillationMarkPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AboveCenter => write!(f, "Above Center"),
            Self::AboveLeft => write!(f, "Above Left"),
            Self::AboveRight => write!(f, "Above Right"),
            Self::BelowCenter => write!(f, "Below Center"),
            Self::BelowRight => write!(f, "Below Right"),
            Self::SofPasuq => write!(f, "End of Verse (Sof Pasuq)"),
            Self::Maqqaph => write!(f, "Between Words (Maqqaph)"),
            Self::Paseq => write!(f, "Word Separator (Paseq)"),
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CantillationMarkStressPosition {
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

impl fmt::Display for CantillationMarkStressPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Impositive => write!(f, "impositive"),
            Self::Prepositive => write!(f, "prepositive"),
            Self::Postpositive => write!(f, "postpositive"),
        }
    }
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
    fn test_invalid_group_level_error_type() {
        use crate::GroupLevelError;

        // Invalid values should return the correct error type
        let err5 = GroupLevel::try_from(5u8).unwrap_err();
        assert!(matches!(err5, GroupLevelError::InvalidGroupLevel(5)));

        let err0 = GroupLevel::try_from(0u8).unwrap_err();
        assert!(matches!(err0, GroupLevelError::InvalidGroupLevel(0)));

        let err255 = GroupLevel::try_from(255u8).unwrap_err();
        assert!(matches!(err255, GroupLevelError::InvalidGroupLevel(255)));
    }

    #[test]
    fn test_group_level_error_message() {
        let err = GroupLevel::try_from(7u8).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid GroupLevel value: 7 (valid range: 1-4)"
        );
    }
}
