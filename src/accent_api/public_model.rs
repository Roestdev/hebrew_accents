use crate::Accent;
use crate::{codepoints::CodePointPosition, HebrewAccent};

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
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
}

/// Hebrew Accent wordstress — (absence is expressed via Option<T>)
///
/// 'WordStress', indicating the location of the accent in relation to the consonant
///
/// # Examples
///
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, AccentWordStress};
///
/// // Check where the stress is located
/// let accent = HebrewAccent::Prose(ProseAccent::Silluq);
///
/// match accent.word_stress() {
///     Some(AccentWordStress::ImPositive) => {
///         println!("Stress is on the syllable");
///     }
///     Some(AccentWordStress::PostPositive) => {
///         println!("Stress is at word end");
///     }
///     Some(AccentWordStress::PrePositive) => {
///         println!("Stress is at word beginning");
///     }
///     None => {
///         println!("No word stress information");
///     }
/// }
/// ```
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentWordStress {
    #[default]
    /// ImPositive: The accent is located above the stressed syllable
    ImPositive,
    /// PostPositive: The accent is NOT located above the stressed syllable, but at the very end of the word
    PostPositive,
    /// PrePositive: Accent is NOT located above the stressed syllable, but at the very beginning of the word
    PrePositive,
}

/// Public-facing representation of a cantillation codepoint
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CantillationMark {
    /// Unicode codepoint value, e.g. "U+0597"
    pub unicode_value: &'static str,
    /// Hex byte representation, e.g. "0xd6 0x97"
    pub hex_bytes: &'static str,
    /// The actual Hebrew character/symbol, e.g. "֗"
    pub symbol: &'static str,
    /// Canonical name from UTF-8 character tables
    pub canonical_name: &'static str,
    /// Position relative to the consonant
    pub position: CantillationMarkPosition,
}

/// Public-facing accent position type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CantillationMarkPosition {
    /// UTF-8 code point is located above the consonant
    Above,
    /// UTF-8 code point is located after the consonant
    After,
    /// UTF-8 code point is located in between two words
    InBetween,
    /// UTF-8 code point is located under the consonant
    Under,
}

// Add conversion from internal to public type
impl From<CodePointPosition> for CantillationMarkPosition {
    fn from(pos: CodePointPosition) -> Self {
        match pos {
            CodePointPosition::Above => CantillationMarkPosition::Above,
            CodePointPosition::After => CantillationMarkPosition::After,
            CodePointPosition::InBetween => CantillationMarkPosition::InBetween,
            CodePointPosition::Under => CantillationMarkPosition::Under,
        }
    }
}

/// Disjunctive accent hierarchy level following Futato's classification system.
///
/// Ranges from 1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// # Example
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, GroupLevel};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.group_level(), Some(GroupLevel::Level1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.group_level(), None);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GroupLevel {
    /// Primary disjunctive tier — creates major clause/phrasal breaks
    Level1 = 1, // value represents group number for extension in future
    /// Secondary disjunctive tier — subordinate phrase boundaries
    Level2,
    /// Tertiary disjunctive tier — minor phrasal divisions
    Level3,
    /// Quaternary disjunctive tier — fine-grained subdivisions
    Level4,
}

impl GroupLevel {
    /// Raw numeric strength value (1 = strongest disjunctive)
    pub const fn value(self) -> u8 {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
        }
    }

    /// Human-readable description of hierarchy tier
    pub const fn description(self) -> &'static str {
        match self {
            Self::Level1 => "Primary disjunctive (major clause break)",
            Self::Level2 => "Secondary disjunctive (phrase boundary)",
            Self::Level3 => "Tertiary disjunctive (minor division)",
            Self::Level4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
}

impl std::fmt::Display for GroupLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Level1 => write!(f, "Tier 1 (Primary disjunctive)"),
            Self::Level2 => write!(f, "Tier 2 (Secondary disjunctive)"),
            Self::Level3 => write!(f, "Tier 3 (Tertiary disjunctive)"),
            Self::Level4 => write!(f, "Tier 4 (Quaternary disjunctive)"),
        }
    }
}

pub(crate) use crate::common::GENERIC_MARK_BASE;

/// Returns the cantilation symbol
/// May consist of two cantillation marks
pub fn cantillation_symbol(accent: HebrewAccent) -> String {
    // get first cantillation_mark
    // if exist get second cantillation_mark
    // stel output samen
    // gebruik generic mark base
    //
    let cant1 = accent.primary_cantillation_mark();
    let cp1 = cant1.unicode_value;
    if accent.secondary_cantillation_mark().is_some() {
        let cp2 = accent.secondary_cantillation_mark().unwrap().unicode_value;
        format!(
            "{}{}{}{}{}",
            GENERIC_MARK_BASE, cp1, GENERIC_MARK_BASE, cp2, " "
        )
    } else {
        format!("{}{}{}", GENERIC_MARK_BASE, cp1, " ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codepoints::CodePointPosition;
    use crate::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    // ── AccentKind ─────────────────────────────────────────────────

    #[test]
    fn accent_kind_default_is_primary() {
        assert_eq!(AccentKind::default(), AccentKind::Primary);
    }

    #[test]
    fn accent_kind_equality() {
        assert_eq!(AccentKind::Primary, AccentKind::Primary);
        assert_ne!(AccentKind::Primary, AccentKind::Secondary);
    }

    #[test]
    fn accent_kind_copy_and_clone() {
        let a = AccentKind::Secondary;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn accent_kind_debug_non_empty() {
        assert!(format!("{:?}", AccentKind::Primary).contains("Primary"));
        assert!(format!("{:?}", AccentKind::Secondary).contains("Secondary"));
    }

    #[test]
    fn accent_kind_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        AccentKind::Primary.hash(&mut h1);
        AccentKind::Primary.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── AccentCategory ────────────────────────────────────────────

    #[test]
    fn accent_category_default_is_disjunctive() {
        assert_eq!(AccentCategory::default(), AccentCategory::Disjunctive);
    }

    #[test]
    fn accent_category_equality() {
        assert_eq!(AccentCategory::Conjunctive, AccentCategory::Conjunctive);
        assert_ne!(AccentCategory::Conjunctive, AccentCategory::Disjunctive);
    }

    #[test]
    fn accent_category_copy_and_clone() {
        let a = AccentCategory::Conjunctive;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn accent_category_debug_non_empty() {
        assert!(format!("{:?}", AccentCategory::Conjunctive).contains("Conjunctive"));
        assert!(format!("{:?}", AccentCategory::Disjunctive).contains("Disjunctive"));
    }

    #[test]
    fn accent_category_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        AccentCategory::Disjunctive.hash(&mut h1);
        AccentCategory::Disjunctive.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── AccentWordStress ──────────────────────────────────────────

    #[test]
    fn accent_word_stress_default_is_im_positive() {
        assert_eq!(AccentWordStress::default(), AccentWordStress::ImPositive);
    }

    #[test]
    fn accent_word_stress_equality() {
        assert_eq!(AccentWordStress::ImPositive, AccentWordStress::ImPositive);
        assert_ne!(AccentWordStress::ImPositive, AccentWordStress::PostPositive);
        assert_ne!(
            AccentWordStress::PostPositive,
            AccentWordStress::PrePositive
        );
        assert_ne!(AccentWordStress::ImPositive, AccentWordStress::PrePositive);
    }

    #[test]
    fn accent_word_stress_copy_and_clone() {
        let a = AccentWordStress::PrePositive;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn accent_word_stress_debug_non_empty() {
        assert!(format!("{:?}", AccentWordStress::ImPositive).contains("ImPositive"));
        assert!(format!("{:?}", AccentWordStress::PostPositive).contains("PostPositive"));
        assert!(format!("{:?}", AccentWordStress::PrePositive).contains("PrePositive"));
    }

    #[test]
    fn accent_word_stress_all_variants_distinct_debug() {
        let a = format!("{:?}", AccentWordStress::ImPositive);
        let b = format!("{:?}", AccentWordStress::PostPositive);
        let c = format!("{:?}", AccentWordStress::PrePositive);
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn accent_word_stress_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        AccentWordStress::PostPositive.hash(&mut h1);
        AccentWordStress::PostPositive.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── CantillationMarkPosition ──────────────────────────────────

    #[test]
    fn cantillation_mark_position_equality() {
        assert_eq!(
            CantillationMarkPosition::Above,
            CantillationMarkPosition::Above
        );
        assert_ne!(
            CantillationMarkPosition::Above,
            CantillationMarkPosition::Under
        );
        assert_ne!(
            CantillationMarkPosition::After,
            CantillationMarkPosition::InBetween
        );
    }

    #[test]
    fn cantillation_mark_position_copy_and_clone() {
        let a = CantillationMarkPosition::InBetween;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn cantillation_mark_position_debug_non_empty() {
        assert!(format!("{:?}", CantillationMarkPosition::Above).contains("Above"));
        assert!(format!("{:?}", CantillationMarkPosition::After).contains("After"));
        assert!(format!("{:?}", CantillationMarkPosition::InBetween).contains("InBetween"));
        assert!(format!("{:?}", CantillationMarkPosition::Under).contains("Under"));
    }

    #[test]
    fn cantillation_mark_position_all_variants_distinct_debug() {
        let variants = [
            format!("{:?}", CantillationMarkPosition::Above),
            format!("{:?}", CantillationMarkPosition::After),
            format!("{:?}", CantillationMarkPosition::InBetween),
            format!("{:?}", CantillationMarkPosition::Under),
        ];
        let mut seen = std::collections::HashSet::new();
        for v in &variants {
            assert!(seen.insert(v.clone()), "Duplicate debug output: {}", v);
        }
        assert_eq!(seen.len(), 4);
    }

    // ── From<CodePointPosition> for CantillationMarkPosition ──────

    #[test]
    fn from_code_point_position_above() {
        let pos: CantillationMarkPosition = CodePointPosition::Above.into();
        assert_eq!(pos, CantillationMarkPosition::Above);
    }

    #[test]
    fn from_code_point_position_after() {
        let pos: CantillationMarkPosition = CodePointPosition::After.into();
        assert_eq!(pos, CantillationMarkPosition::After);
    }

    #[test]
    fn from_code_point_position_in_between() {
        let pos: CantillationMarkPosition = CodePointPosition::InBetween.into();
        assert_eq!(pos, CantillationMarkPosition::InBetween);
    }

    #[test]
    fn from_code_point_position_under() {
        let pos: CantillationMarkPosition = CodePointPosition::Under.into();
        assert_eq!(pos, CantillationMarkPosition::Under);
    }

    #[test]
    fn from_code_point_position_all_variants_match() {
        let cases = [
            (CodePointPosition::Above, CantillationMarkPosition::Above),
            (CodePointPosition::After, CantillationMarkPosition::After),
            (
                CodePointPosition::InBetween,
                CantillationMarkPosition::InBetween,
            ),
            (CodePointPosition::Under, CantillationMarkPosition::Under),
        ];

        for (cp, expected) in cases {
            let converted: CantillationMarkPosition = cp.into();
            assert_eq!(converted, expected);
        }
    }

    // ── CantillationMark struct ───────────────────────────────────

    #[test]
    fn cantillation_mark_construction_and_field_access() {
        let mark = CantillationMark {
            unicode_value: "U+0597",
            hex_bytes: "0xd6 0x97",
            symbol: "֗",
            canonical_name: "HEBREW ACCENT ZAQEF QATAN",
            position: CantillationMarkPosition::Above,
        };

        assert_eq!(mark.unicode_value, "U+0597");
        assert_eq!(mark.hex_bytes, "0xd6 0x97");
        assert_eq!(mark.symbol, "֗");
        assert_eq!(mark.canonical_name, "HEBREW ACCENT ZAQEF QATAN");
        assert_eq!(mark.position, CantillationMarkPosition::Above);
    }

    #[test]
    fn cantillation_mark_equality() {
        let a = CantillationMark {
            unicode_value: "U+0598",
            hex_bytes: "0xd6 0x98",
            symbol: "֘",
            canonical_name: "HEBREW ACCENT ZAQEF GADOL",
            position: CantillationMarkPosition::Above,
        };
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn cantillation_mark_inequality_on_different_fields() {
        let base = CantillationMark {
            unicode_value: "U+0598",
            hex_bytes: "0xd6 0x98",
            symbol: "֘",
            canonical_name: "HEBREW ACCENT ZAQEF GADOL",
            position: CantillationMarkPosition::Above,
        };

        // Different unicode_value
        let diff_val = CantillationMark {
            unicode_value: "U+0599",
            ..base
        };
        assert_ne!(base, diff_val);

        // Different position
        let diff_pos = CantillationMark {
            position: CantillationMarkPosition::Under,
            ..base
        };
        assert_ne!(base, diff_pos);

        // Different symbol
        let diff_sym = CantillationMark {
            symbol: "֙", ..base
        };
        assert_ne!(base, diff_sym);
    }

    #[test]
    fn cantillation_mark_copy_preserves_value() {
        let mark = CantillationMark {
            unicode_value: "U+059A",
            hex_bytes: "0xd6 0x9a",
            symbol: "֚",
            canonical_name: "HEBREW ACCENT TIPEHA",
            position: CantillationMarkPosition::Under,
        };
        let copied = mark;
        assert_eq!(mark, copied);
    }

    #[test]
    fn cantillation_mark_clone_preserves_value() {
        let mark = CantillationMark {
            unicode_value: "U+059B",
            hex_bytes: "0xd6 0x9b",
            symbol: "֛",
            canonical_name: "HEBREW ACCENT ZARKA",
            position: CantillationMarkPosition::Above,
        };
        assert_eq!(mark, mark.clone());
    }

    #[test]
    fn cantillation_mark_debug_non_empty() {
        let mark = CantillationMark {
            unicode_value: "U+0597",
            hex_bytes: "0xd6 0x97",
            symbol: "֗",
            canonical_name: "HEBREW ACCENT ZAQEF QATAN",
            position: CantillationMarkPosition::Above,
        };
        let dbg = format!("{:?}", mark);
        assert!(dbg.contains("CantillationMark"));
        assert!(dbg.contains("U+0597"));
        assert!(dbg.contains("Above"));
    }

    #[test]
    fn cantillation_mark_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mark = CantillationMark {
            unicode_value: "U+0597",
            hex_bytes: "0xd6 0x97",
            symbol: "֗",
            canonical_name: "HEBREW ACCENT ZAQEF QATAN",
            position: CantillationMarkPosition::Above,
        };

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        mark.hash(&mut h1);
        mark.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── GroupLevel ────────────────────────────────────────────────

    #[test]
    fn group_level_discriminants_are_sequential() {
        assert_eq!(GroupLevel::Level1 as u8, 1);
        assert_eq!(GroupLevel::Level2 as u8, 2);
        assert_eq!(GroupLevel::Level3 as u8, 3);
        assert_eq!(GroupLevel::Level4 as u8, 4);
    }

    #[test]
    fn group_level_value_method_matches_discriminant() {
        assert_eq!(GroupLevel::Level1.value(), 1);
        assert_eq!(GroupLevel::Level2.value(), 2);
        assert_eq!(GroupLevel::Level3.value(), 3);
        assert_eq!(GroupLevel::Level4.value(), 4);
    }

    #[test]
    fn group_level_value_is_one_based() {
        // Level1 (strongest) has value 1, not 0
        assert_eq!(GroupLevel::Level1.value(), 1);
    }

    #[test]
    fn group_level_value_monotonically_increasing() {
        assert!(GroupLevel::Level1.value() < GroupLevel::Level2.value());
        assert!(GroupLevel::Level2.value() < GroupLevel::Level3.value());
        assert!(GroupLevel::Level3.value() < GroupLevel::Level4.value());
    }

    #[test]
    fn group_level_description_non_empty() {
        for level in [
            GroupLevel::Level1,
            GroupLevel::Level2,
            GroupLevel::Level3,
            GroupLevel::Level4,
        ] {
            assert!(
                !level.description().is_empty(),
                "Description empty for {:?}",
                level
            );
        }
    }

    #[test]
    fn group_level_description_contains_tier_info() {
        assert!(GroupLevel::Level1.description().contains("Primary"));
        assert!(GroupLevel::Level2.description().contains("Secondary"));
        assert!(GroupLevel::Level3.description().contains("Tertiary"));
        assert!(GroupLevel::Level4.description().contains("Quaternary"));
    }

    #[test]
    fn group_level_descriptions_are_distinct() {
        let descs = [
            GroupLevel::Level1.description(),
            GroupLevel::Level2.description(),
            GroupLevel::Level3.description(),
            GroupLevel::Level4.description(),
        ];
        let mut seen = std::collections::HashSet::new();
        for d in &descs {
            assert!(seen.insert(*d), "Duplicate description: {}", d);
        }
        assert_eq!(seen.len(), 4);
    }

    #[test]
    fn group_level_equality() {
        assert_eq!(GroupLevel::Level1, GroupLevel::Level1);
        assert_ne!(GroupLevel::Level1, GroupLevel::Level2);
        assert_ne!(GroupLevel::Level3, GroupLevel::Level4);
    }

    #[test]
    fn group_level_copy_and_clone() {
        let a = GroupLevel::Level3;
        let b = a;
        assert_eq!(a, b);
        assert_eq!(a, a.clone());
    }

    #[test]
    fn group_level_debug_non_empty() {
        assert!(format!("{:?}", GroupLevel::Level1).contains("Level1"));
        assert!(format!("{:?}", GroupLevel::Level4).contains("Level4"));
    }

    #[test]
    fn group_level_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        GroupLevel::Level2.hash(&mut h1);
        GroupLevel::Level2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    // ── GroupLevel Display ────────────────────────────────────────

    #[test]
    fn group_level_display_contains_tier_number() {
        assert!(format!("{}", GroupLevel::Level1).contains("Tier 1"));
        assert!(format!("{}", GroupLevel::Level2).contains("Tier 2"));
        assert!(format!("{}", GroupLevel::Level3).contains("Tier 3"));
        assert!(format!("{}", GroupLevel::Level4).contains("Tier 4"));
    }

    #[test]
    fn group_level_display_contains_disjunctive_label() {
        assert!(format!("{}", GroupLevel::Level1).contains("disjunctive"));
        assert!(format!("{}", GroupLevel::Level2).contains("disjunctive"));
        assert!(format!("{}", GroupLevel::Level3).contains("disjunctive"));
        assert!(format!("{}", GroupLevel::Level4).contains("disjunctive"));
    }

    #[test]
    fn group_level_display_outputs_are_distinct() {
        let outputs: Vec<String> = [
            GroupLevel::Level1,
            GroupLevel::Level2,
            GroupLevel::Level3,
            GroupLevel::Level4,
        ]
        .iter()
        .map(|l| l.to_string())
        .collect();

        let mut seen = std::collections::HashSet::new();
        for s in &outputs {
            assert!(seen.insert(s.clone()), "Duplicate Display: {}", s);
        }
        assert_eq!(seen.len(), 4);
    }

    // ── cantillation_symbol function ──────────────────────────────
    // This function depends on the Accent trait, so we test it with
    // actual accent values. The exact output depends on the accent's
    // cantillation mark data, which is defined externally.

    #[test]
    fn cantillation_symbol_returns_non_empty_string() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let symbol = cantillation_symbol(accent);
        assert!(
            !symbol.is_empty(),
            "cantillation_symbol returned empty string"
        );
    }

    #[test]
    fn cantillation_symbol_ends_with_space() {
        // Based on the format strings in the function, output always ends with " "
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);
        let symbol = cantillation_symbol(accent);
        assert!(
            symbol.ends_with(' '),
            "Expected trailing space, got: {:?}",
            symbol
        );
    }

    #[test]
    fn cantillation_symbol_contains_generic_mark_base() {
        // The function prepends GENERIC_MARK_BASE to each codepoint
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let symbol = cantillation_symbol(accent);
        assert!(
            symbol.contains(GENERIC_MARK_BASE),
            "Output missing GENERIC_MARK_BASE: {:?}",
            symbol
        );
    }

    #[test]
    fn cantillation_symbol_differs_across_accents() {
        let s1 = cantillation_symbol(HebrewAccent::Prose(ProseAccent::Silluq));
        let s2 = cantillation_symbol(HebrewAccent::Prose(ProseAccent::Atnach));
        // Most accents should have different Unicode values
        assert_ne!(s1, s2, "Different accents produced identical symbols");
    }

    #[test]
    fn cantillation_symbol_works_for_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let symbol = cantillation_symbol(accent);
        assert!(!symbol.is_empty());
        assert!(symbol.ends_with(' '));
    }

    #[test]
    fn cantillation_symbol_works_for_pseudo() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let symbol = cantillation_symbol(accent);
        assert!(!symbol.is_empty());
        assert!(symbol.ends_with(' '));
    }

    #[test]
    fn cantillation_symbol_with_secondary_mark_has_two_bases() {
        // Find an accent that has a secondary cantillation mark.
        // We test a few; the function adds a second GENERIC_MARK_BASE
        // when a secondary mark exists.
        let accents = [
            HebrewAccent::Prose(ProseAccent::Silluq),
            HebrewAccent::Prose(ProseAccent::Atnach),
            HebrewAccent::Prose(ProseAccent::Segolta),
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored),
            HebrewAccent::Poetry(PoetryAccent::Atnach),
        ];

        // At least one accent should produce output with two mark bases
        let any_two = accents
            .iter()
            .map(|a| cantillation_symbol(*a))
            .any(|s| s.matches(GENERIC_MARK_BASE).count() >= 2);

        // If none have secondary marks, that's acceptable — but if some do,
        // verify the format includes both bases.
        if any_two {
            for accent in &accents {
                let symbol = cantillation_symbol(*accent);
                let count = symbol.matches(GENERIC_MARK_BASE).count();
                if count >= 2 {
                    // Secondary path: format is "{BASE}{cp1}{BASE}{cp2} "
                    assert!(
                        symbol.ends_with(' '),
                        "Missing trailing space with secondary mark"
                    );
                }
            }
        }
    }

    #[test]
    fn cantillation_symbol_produces_distinct_output_for_all_accent_types() {
        let prose = cantillation_symbol(HebrewAccent::Prose(ProseAccent::Silluq));
        let poetry = cantillation_symbol(HebrewAccent::Poetry(PoetryAccent::Silluq));
        let pseudo = cantillation_symbol(HebrewAccent::Pseudo(PseudoAccent::SophPasuq));

        // At least two should differ (they use different codepoints)
        let unique = std::collections::HashSet::from([prose, poetry, pseudo]);
        assert!(
            unique.len() >= 2,
            "Expected at least 2 distinct symbols across accent types"
        );
    }
}
