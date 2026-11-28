// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use std::fmt;

use crate::mark::StressPosition;

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
/// # How This Handles Shene Pashtin (Double Pashta)
///
/// When a word has **non-final stress** (stress not on the last syllable)
/// AND requires a Pashta mark, the system creates a **compound accent**:
///
/// 1. **Primary mark**: Qadma (U+05A8) → `stress_position: Impositive` (on stressed syllable)
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
/// // Shene Pashtin (double Pashta) has BOTH positions via compound marks
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

#[cfg(test)]
mod tests {
    use super::*;

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
        use crate::mark::MarkPlacement;

        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::AboveLeft),
            CantillationMarkPlacement::AboveLeft
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::AboveCenter),
            CantillationMarkPlacement::AboveCenter
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::AboveRight),
            CantillationMarkPlacement::AboveRight
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::BelowCenter),
            CantillationMarkPlacement::BelowCenter
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::BelowRight),
            CantillationMarkPlacement::BelowRight
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::SofPasuq),
            CantillationMarkPlacement::SofPasuq
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::Maqqaph),
            CantillationMarkPlacement::Maqqaph
        );
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::Paseq),
            CantillationMarkPlacement::Paseq
        );
    }

    #[test]
    fn test_from_stress_position_to_option_stress_position() {
        use crate::mark::StressPosition;

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
        use crate::mark::MarkPlacement;

        // Test every match arm including the one that's currently commented
        let _above_left = CantillationMarkPlacement::from(MarkPlacement::AboveLeft);
        let _above_center = CantillationMarkPlacement::from(MarkPlacement::AboveCenter);
        let _above_right = CantillationMarkPlacement::from(MarkPlacement::AboveRight);
        let _below_center = CantillationMarkPlacement::from(MarkPlacement::BelowCenter);
        let _below_right = CantillationMarkPlacement::from(MarkPlacement::BelowRight);
        let _sof_pasuq = CantillationMarkPlacement::from(MarkPlacement::SofPasuq);
        let _maqaph = CantillationMarkPlacement::from(MarkPlacement::Maqqaph);
        let _paseq = CantillationMarkPlacement::from(MarkPlacement::Paseq);

        // Verify they convert correctly
        assert_eq!(
            CantillationMarkPlacement::from(MarkPlacement::AboveLeft),
            CantillationMarkPlacement::AboveLeft
        );
    }

    #[test]
    fn test_from_stress_position_all_variants() {
        use crate::mark::StressPosition;

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
        use crate::mark::StressPosition;

        // Ensure compiler knows we're converting to Option<CantillationMarkStressPosition>
        let result: Option<CantillationMarkStressPosition> = StressPosition::NotApplicable.into();
        assert!(result.is_none());
    }

    // ============================================================
    // DISPLAY TRAIT - INDIVIDUAL VARIANT COVERAGE
    // ============================================================

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
    // GROUP LEVEL - CONTEXT-AWARE METHODS
    // ============================================================

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
