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
mod tests {
    use super::*;

    #[test]
    fn test_group_level_error_message() {
        let err = GroupLevel::try_from(7u8).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid GroupLevel value: 7 (valid range: 1-4)"
        );
    }

    #[test]
    fn test_group_level_valid_conversions() {
        // All valid conversions should succeed
        assert_eq!(GroupLevel::try_from(1u8), Ok(GroupLevel::Tier1));
        assert_eq!(GroupLevel::try_from(2u8), Ok(GroupLevel::Tier2));
        assert_eq!(GroupLevel::try_from(3u8), Ok(GroupLevel::Tier3));
        assert_eq!(GroupLevel::try_from(4u8), Ok(GroupLevel::Tier4));
    }

    #[test]
    fn test_group_level_value_method() {
        assert_eq!(GroupLevel::Tier1.value(), 1);
        assert_eq!(GroupLevel::Tier2.value(), 2);
        assert_eq!(GroupLevel::Tier3.value(), 3);
        assert_eq!(GroupLevel::Tier4.value(), 4);
    }

    #[test]
    fn test_group_level_description_method() {
        assert_eq!(
            GroupLevel::Tier1.description(),
            "Primary disjunctive (major clause break)"
        );
        assert_eq!(
            GroupLevel::Tier2.description(),
            "Secondary disjunctive (phrase boundary)"
        );
        assert_eq!(
            GroupLevel::Tier3.description(),
            "Tertiary disjunctive (minor division)"
        );
        assert_eq!(
            GroupLevel::Tier4.description(),
            "Quaternary disjunctive (fine subdivision)"
        );
    }

    #[test]
    fn test_group_level_alt_description_method() {
        assert_eq!(GroupLevel::Tier1.alt_description(), "Emperor");
        assert_eq!(GroupLevel::Tier2.alt_description(), "Kings");
        assert_eq!(GroupLevel::Tier3.alt_description(), "Dukes");
        assert_eq!(GroupLevel::Tier4.alt_description(), "Officers");
    }

    #[test]
    fn test_group_level_max_methods() {
        assert_eq!(GroupLevel::max_poetry_tier(), GroupLevel::Tier3);
        assert_eq!(GroupLevel::max_prose_tier(), GroupLevel::Tier4);
    }

    #[test]
    fn test_group_level_is_poetry_available() {
        // Poetry has tiers 1-3
        assert!(GroupLevel::Tier1.is_poetry_available());
        assert!(GroupLevel::Tier2.is_poetry_available());
        assert!(GroupLevel::Tier3.is_poetry_available());
        // Tier4 is prose-only
        assert!(!GroupLevel::Tier4.is_poetry_available());
    }

    #[test]
    fn test_group_level_display() {
        assert_eq!(
            format!("{}", GroupLevel::Tier1),
            "Tier 1 (Primary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier2),
            "Tier 2 (Secondary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier3),
            "Tier 3 (Tertiary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier4),
            "Tier 4 (Quaternary disjunctive)"
        );
    }

    #[test]
    fn test_group_level_debug_trait() {
        let debug_output = format!("{:?}", GroupLevel::Tier1);
        assert!(debug_output.contains("Tier1"));
    }

    #[test]
    fn test_group_level_copy_clone_trait() {
        let original = GroupLevel::Tier1;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original, GroupLevel::Tier1);
    }

    #[test]
    fn test_group_level_eq_partial_eq_hash() {
        let l1 = GroupLevel::Tier1;
        let l2 = GroupLevel::Tier1;
        let l3 = GroupLevel::Tier2;

        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
        assert!(l1.eq(&l2));
    }

    // ===== ACCENT KIND TESTS =====

    #[test]
    fn test_accent_kind_variants() {
        assert_eq!(format!("{}", AccentKind::Primary), "primary");
        assert_eq!(format!("{}", AccentKind::Secondary), "secondary");
    }

    #[test]
    fn test_accent_kind_debug_trait() {
        let debug = format!("{:?}", AccentKind::Primary);
        assert!(debug.contains("Primary"));
    }

    #[test]
    fn test_accent_kind_copy_clone_eq_hash() {
        let k1 = AccentKind::Primary;
        let k2 = AccentKind::Primary;

        assert_eq!(k1, k2);
        assert_eq!(k1.clone(), k2);
    }

    // ===== ACCENT CATEGORY TESTS =====

    #[test]
    fn test_accent_category_variants() {
        assert_eq!(format!("{}", AccentCategory::Disjunctive), "disjunctive");
        assert_eq!(format!("{}", AccentCategory::Conjunctive), "conjunctive");
    }

    #[test]
    fn test_accent_category_debug_trait() {
        let debug = format!("{:?}", AccentCategory::Disjunctive);
        assert!(debug.contains("Disjunctive"));
    }

    #[test]
    fn test_accent_category_copy_clone_eq_hash() {
        let c1 = AccentCategory::Conjunctive;
        let c2 = AccentCategory::Conjunctive;

        assert_eq!(c1, c2);
        assert_eq!(c1.clone(), c2);
    }

    // ===== CANTILLATION MARK PLACEMENT TESTS =====

    #[test]
    fn test_cantillation_mark_placement_display_all() {
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveCenter),
            "Above Center"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveLeft),
            "Above Left"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveRight),
            "Above Right"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::BelowCenter),
            "Below Center"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::BelowRight),
            "Below Right"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::SofPasuq),
            "End of Verse (Sof Pasuq)"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::Maqqaph),
            "Between Words (Maqqaph)"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::Paseq),
            "Word Separator (Paseq)"
        );
    }

    #[test]
    fn test_cantillation_mark_placement_debug_trait() {
        let debug = format!("{:?}", CantillationMarkPlacement::AboveCenter);
        assert!(debug.contains("AboveCenter"));
    }

    #[test]
    fn test_cantillation_mark_placement_copy_clone_eq_hash() {
        let p1 = CantillationMarkPlacement::AboveCenter;
        let p2 = CantillationMarkPlacement::AboveCenter;

        assert_eq!(p1, p2);
        assert_eq!(p1.clone(), p2);
    }

    // ===== CANTILLATION MARK STRESS POSITION TESTS =====

    #[test]
    fn test_cantillation_mark_stress_position_display() {
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Impositive),
            "impositive"
        );
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Prepositive),
            "prepositive"
        );
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Postpositive),
            "postpositive"
        );
    }

    #[test]
    fn test_cantillation_mark_stress_position_debug_trait() {
        let debug = format!("{:?}", CantillationMarkStressPosition::Impositive);
        assert!(debug.contains("Impositive"));
    }

    #[test]
    fn test_cantillation_mark_stress_position_copy_eq_hash() {
        let s1 = CantillationMarkStressPosition::Impositive;
        let s2 = CantillationMarkStressPosition::Impositive;

        assert_eq!(s1, s2);
        assert_eq!(s1.clone(), s2);
    }

    // ===== CONVERSION IMPL TESTS =====

    #[test]
    fn test_from_codepoint_position_to_placement() {
        use crate::accent_mark::CodePointPosition;

        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::AboveLeft),
            CantillationMarkPlacement::AboveLeft
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::AboveCenter),
            CantillationMarkPlacement::AboveCenter
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::AboveRight),
            CantillationMarkPlacement::AboveRight
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::BelowCenter),
            CantillationMarkPlacement::BelowCenter
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::BelowRight),
            CantillationMarkPlacement::BelowRight
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::SofPasuq),
            CantillationMarkPlacement::SofPasuq
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::Maqqaph),
            CantillationMarkPlacement::Maqqaph
        );
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::Paseq),
            CantillationMarkPlacement::Paseq
        );
    }

    #[test]
    fn test_from_stress_position_to_option_stress_position() {
        use crate::accent_mark::StressPosition;

        assert_eq!(
            Option::<CantillationMarkStressPosition>::from(StressPosition::Impositive),
            Some(CantillationMarkStressPosition::Impositive)
        );
        assert_eq!(
            Option::<CantillationMarkStressPosition>::from(StressPosition::Prepositive),
            Some(CantillationMarkStressPosition::Prepositive)
        );
        assert_eq!(
            Option::<CantillationMarkStressPosition>::from(StressPosition::Postpositive),
            Some(CantillationMarkStressPosition::Postpositive)
        );
        assert_eq!(
            Option::<CantillationMarkStressPosition>::from(StressPosition::NotApplicable),
            None
        );
    }

    // ===== CANTILLATION MARK STRUCT TESTS =====

    #[test]
    fn test_cantillation_mark_struct_fields() {
        let mark = CantillationMark {
            symbol: '֑',
            placement: CantillationMarkPlacement::BelowCenter,
            stress_position: Some(CantillationMarkStressPosition::Impositive),
        };

        assert_eq!(mark.symbol, '֑');
        assert_eq!(mark.placement, CantillationMarkPlacement::BelowCenter);
        assert_eq!(
            mark.stress_position,
            Some(CantillationMarkStressPosition::Impositive)
        );
    }

    #[test]
    fn test_cantillation_mark_none_stress() {
        let mark = CantillationMark {
            symbol: '׀',
            placement: CantillationMarkPlacement::Paseq,
            stress_position: None,
        };

        assert_eq!(mark.stress_position, None);
    }

    #[test]
    fn test_cantillation_mark_debug_trait() {
        let mark = CantillationMark {
            symbol: '֑',
            placement: CantillationMarkPlacement::BelowCenter,
            stress_position: Some(CantillationMarkStressPosition::Impositive),
        };

        let debug = format!("{:?}", mark);
        assert!(debug.contains("CantillationMark"));
        assert!(debug.contains("BelowCenter"));
    }

    #[test]
    fn test_cantillation_mark_copy_clone_eq_hash() {
        let m1 = CantillationMark {
            symbol: '֑',
            placement: CantillationMarkPlacement::BelowCenter,
            stress_position: Some(CantillationMarkStressPosition::Impositive),
        };
        let m2 = CantillationMark {
            symbol: '֑',
            placement: CantillationMarkPlacement::BelowCenter,
            stress_position: Some(CantillationMarkStressPosition::Impositive),
        };

        assert_eq!(m1, m2);
        assert_eq!(m1.clone(), m2);
    }
}

#[cfg(test)]
mod additional_function_coverage_tests {
    use super::*;

    // ============================================================
    // FROM TRAIT IMPLEMENTATIONS - FULL COVERAGE
    // ============================================================

    #[test]
    fn test_from_codepoint_position_all_variants() {
        use crate::accent_mark::CodePointPosition;

        // Test every match arm including the one that's currently commented
        let _above_left = CantillationMarkPlacement::from(CodePointPosition::AboveLeft);
        let _above_center = CantillationMarkPlacement::from(CodePointPosition::AboveCenter);
        let _above_right = CantillationMarkPlacement::from(CodePointPosition::AboveRight);
        let _below_center = CantillationMarkPlacement::from(CodePointPosition::BelowCenter);
        let _below_right = CantillationMarkPlacement::from(CodePointPosition::BelowRight);
        let _sof_pasuq = CantillationMarkPlacement::from(CodePointPosition::SofPasuq);
        let _maqaph = CantillationMarkPlacement::from(CodePointPosition::Maqqaph);
        let _paseq = CantillationMarkPlacement::from(CodePointPosition::Paseq);

        // Verify they convert correctly
        assert_eq!(
            CantillationMarkPlacement::from(CodePointPosition::AboveLeft),
            CantillationMarkPlacement::AboveLeft
        );
    }

    #[test]
    fn test_from_stress_position_all_variants() {
        use crate::accent_mark::StressPosition;

        // Test Impositive
        let impositive: Option<CantillationMarkStressPosition> = StressPosition::Impositive.into();
        assert_eq!(impositive, Some(CantillationMarkStressPosition::Impositive));

        // Test Prepositive
        let prepositive: Option<CantillationMarkStressPosition> =
            StressPosition::Prepositive.into();
        assert_eq!(
            prepositive,
            Some(CantillationMarkStressPosition::Prepositive)
        );

        // Test Postpositive
        let postpositive: Option<CantillationMarkStressPosition> =
            StressPosition::Postpositive.into();
        assert_eq!(
            postpositive,
            Some(CantillationMarkStressPosition::Postpositive)
        );

        // Test NotApplicable -> None (THIS IS THE KEY MISSING PATH)
        let not_applicable: Option<CantillationMarkStressPosition> =
            StressPosition::NotApplicable.into();
        assert_eq!(not_applicable, None);
    }

    #[test]
    fn test_from_stress_position_explicit_type_annotation() {
        use crate::accent_mark::StressPosition;

        // Ensure compiler knows we're converting to Option<CantillationMarkStressPosition>
        let result: Option<CantillationMarkStressPosition> = StressPosition::NotApplicable.into();
        assert!(result.is_none());
    }

    // ============================================================
    // GROUP LEVEL TRYFROM EDGE CASES
    // ============================================================

    #[test]
    fn test_try_from_u8_zero() {
        // Test boundary case: 0 should fail
        let result = GroupLevel::try_from(0u8);
        assert!(matches!(
            result,
            Err(crate::GroupLevelError::InvalidGroupLevel(0))
        ));
    }

    #[test]
    fn test_try_from_u8_boundary_min() {
        // Test minimum valid value
        let result = GroupLevel::try_from(1u8);
        assert_eq!(result, Ok(GroupLevel::Tier1));
    }

    #[test]
    fn test_try_from_u8_boundary_max() {
        // Test maximum valid value
        let result = GroupLevel::try_from(4u8);
        assert_eq!(result, Ok(GroupLevel::Tier4));
    }

    #[test]
    fn test_try_from_u8_above_max() {
        // Test above maximum
        let result = GroupLevel::try_from(5u8);
        assert!(matches!(
            result,
            Err(crate::GroupLevelError::InvalidGroupLevel(5))
        ));
    }

    #[test]
    fn test_try_from_u8_large_value() {
        // Test very large value
        let result = GroupLevel::try_from(255u8);
        assert!(matches!(
            result,
            Err(crate::GroupLevelError::InvalidGroupLevel(255))
        ));
    }

    // ============================================================
    // DISPLAY TRAIT - INDIVIDUAL VARIANT COVERAGE
    // ============================================================

    #[test]
    fn test_accent_kind_display_exact_output() {
        assert_eq!(format!("{}", AccentKind::Primary), "primary");
        assert_eq!(format!("{}", AccentKind::Secondary), "secondary");
    }

    #[test]
    fn test_accent_category_display_exact_output() {
        assert_eq!(format!("{}", AccentCategory::Disjunctive), "disjunctive");
        assert_eq!(format!("{}", AccentCategory::Conjunctive), "conjunctive");
    }

    #[test]
    fn test_group_level_display_exact_output() {
        assert_eq!(
            format!("{}", GroupLevel::Tier1),
            "Tier 1 (Primary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier2),
            "Tier 2 (Secondary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier3),
            "Tier 3 (Tertiary disjunctive)"
        );
        assert_eq!(
            format!("{}", GroupLevel::Tier4),
            "Tier 4 (Quaternary disjunctive)"
        );
    }

    #[test]
    fn test_cantillation_mark_placement_display_all_variants_explicit() {
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveCenter),
            "Above Center"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveLeft),
            "Above Left"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::AboveRight),
            "Above Right"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::BelowCenter),
            "Below Center"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::BelowRight),
            "Below Right"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::SofPasuq),
            "End of Verse (Sof Pasuq)"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::Maqqaph),
            "Between Words (Maqqaph)"
        );
        assert_eq!(
            format!("{}", CantillationMarkPlacement::Paseq),
            "Word Separator (Paseq)"
        );
    }

    #[test]
    fn test_cantillation_mark_stress_position_display_all_variants_explicit() {
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Impositive),
            "impositive"
        );
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Prepositive),
            "prepositive"
        );
        assert_eq!(
            format!("{}", CantillationMarkStressPosition::Postpositive),
            "postpositive"
        );
    }

    // ============================================================
    // CANTILLATION MARK STRUCT - FIELD ACCESSORS
    // ============================================================

    #[test]
    fn test_cantillation_mark_struct_public_fields() {
        let mark = CantillationMark {
            symbol: '֑',
            placement: CantillationMarkPlacement::AboveCenter,
            stress_position: Some(CantillationMarkStressPosition::Prepositive),
        };

        // Access all public fields explicitly
        assert_eq!(mark.symbol, '֑');
        assert_eq!(mark.placement, CantillationMarkPlacement::AboveCenter);
        assert_eq!(
            mark.stress_position,
            Some(CantillationMarkStressPosition::Prepositive)
        );
    }

    #[test]
    fn test_cantillation_mark_with_pseudo_accent_stress_none() {
        // Paseq and Maqqaph have no stress position
        let paseq = CantillationMark {
            symbol: '׀',
            placement: CantillationMarkPlacement::Paseq,
            stress_position: None,
        };

        assert_eq!(paseq.stress_position, None);
    }

    #[test]
    fn test_cantillation_mark_with_sof_pasuq() {
        let sof_pasuq = CantillationMark {
            symbol: '׃',
            placement: CantillationMarkPlacement::SofPasuq,
            stress_position: None,
        };

        assert_eq!(sof_pasuq.placement, CantillationMarkPlacement::SofPasuq);
        assert_eq!(sof_pasuq.stress_position, None);
    }

    #[test]
    fn test_cantillation_mark_with_maqqaph() {
        let maqqaph = CantillationMark {
            symbol: '־',
            placement: CantillationMarkPlacement::Maqqaph,
            stress_position: None,
        };

        assert_eq!(maqqaph.placement, CantillationMarkPlacement::Maqqaph);
    }

    // ============================================================
    // GROUP LEVEL - CONSTANT FUNCTIONS
    // ============================================================

    #[test]
    fn test_group_level_value_const_fn() {
        const TIER1_VAL: u8 = GroupLevel::Tier1.value();
        const TIER2_VAL: u8 = GroupLevel::Tier2.value();
        const TIER3_VAL: u8 = GroupLevel::Tier3.value();
        const TIER4_VAL: u8 = GroupLevel::Tier4.value();

        assert_eq!(TIER1_VAL, 1);
        assert_eq!(TIER2_VAL, 2);
        assert_eq!(TIER3_VAL, 3);
        assert_eq!(TIER4_VAL, 4);
    }

    #[test]
    fn test_group_level_description_const_fn() {
        const DESC1: &str = GroupLevel::Tier1.description();
        const DESC2: &str = GroupLevel::Tier2.description();
        const DESC3: &str = GroupLevel::Tier3.description();
        const DESC4: &str = GroupLevel::Tier4.description();

        assert_eq!(DESC1, "Primary disjunctive (major clause break)");
        assert_eq!(DESC2, "Secondary disjunctive (phrase boundary)");
        assert_eq!(DESC3, "Tertiary disjunctive (minor division)");
        assert_eq!(DESC4, "Quaternary disjunctive (fine subdivision)");
    }

    #[test]
    fn test_group_level_alt_description_const_fn() {
        const ALT1: &str = GroupLevel::Tier1.alt_description();
        const ALT2: &str = GroupLevel::Tier2.alt_description();
        const ALT3: &str = GroupLevel::Tier3.alt_description();
        const ALT4: &str = GroupLevel::Tier4.alt_description();

        assert_eq!(ALT1, "Emperor");
        assert_eq!(ALT2, "Kings");
        assert_eq!(ALT3, "Dukes");
        assert_eq!(ALT4, "Officers");
    }

    // ============================================================
    // GROUP LEVEL - CONTEXT-AWARE METHODS
    // ============================================================

    #[test]
    fn test_group_level_poetry_vs_prose_distinction() {
        // Poetry stops at Tier3
        assert_eq!(GroupLevel::max_poetry_tier(), GroupLevel::Tier3);
        assert!(GroupLevel::Tier1.is_poetry_available());
        assert!(GroupLevel::Tier2.is_poetry_available());
        assert!(GroupLevel::Tier3.is_poetry_available());
        assert!(!GroupLevel::Tier4.is_poetry_available());

        // Prose goes to Tier4
        assert_eq!(GroupLevel::max_prose_tier(), GroupLevel::Tier4);
    }

    #[test]
    fn test_group_level_tier4_prose_only() {
        // Tier4 is specifically prose-only
        assert!(!GroupLevel::Tier4.is_poetry_available());
        assert_eq!(GroupLevel::max_poetry_tier(), GroupLevel::Tier3);
    }

    // ============================================================
    // HASH AND PARTIAL_EQ TRAIT TESTING
    // ============================================================

    #[test]
    fn test_group_level_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        GroupLevel::Tier1.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        GroupLevel::Tier1.hash(&mut h2);
        let hash2 = h2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_accent_kind_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        AccentKind::Primary.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        AccentKind::Primary.hash(&mut h2);
        let hash2 = h2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_cantillation_mark_placement_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        CantillationMarkPlacement::AboveCenter.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        CantillationMarkPlacement::AboveCenter.hash(&mut h2);
        let hash2 = h2.finish();

        assert_eq!(hash1, hash2);
    }
}
