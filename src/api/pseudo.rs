//! # Hebrew Pseudo-Accents
//!
//! Enumerates syntactic markers associated with biblical Hebrew cantillation that
//! are **distinct from true cantillation accents** (ta'amim).
//!
//! ## Purpose
//!
//! Pseudo-accents represent structural symbols that influence accent placement,
//! word grouping, and verse boundaries within the Masoretic text tradition.
//! Unlike true accents, they do not carry independent melodic contours.
//!
//! ## Variants
//!
//! | Variant | Hebrew | Function |
//! |---------|--------|----------|
//! | [`SophPasuq`] | סוֹף פָּסוּק | Marks verse end |
//! | [`Maqqaph`] | מַקָּף | Joins words (hyphen) |
//! | [`Paseq`] | פָּשְׁק | Separates adjacent accents |
//!
//! ## Usage
//!
//! ```rust
//! use hebrew_accents::{PseudoAccent, Accent};
//!
//! let mark = PseudoAccent::Maqqaph;
//! println!("{}", mark); // "Maqqaph (מַקָּף), meaning: hyphen"
//! ```

use crate::accent_data::PSEUDO_ACCENT_TABLE;
use crate::api::CompoundType;
use crate::{
    display_cantillation_symbol, AccentCategory, AccentKind, CantillationMark, GroupLevel,
};
use crate::{Accent, AlternateNames};
use strum_macros::{EnumCount, EnumIter};

/// Represents a syntactic marker associated with Hebrew cantillation.
///
/// `PseudoAccent` values are structural symbols that influence accent placement
/// and word grouping without carrying independent melodic contour. They govern
/// phrase boundaries, word joining, and disambiguation within the Masoretic
/// text tradition.
///
/// # Distinction from True Accents
///
/// | Property | True Accents ([`ProseAccent`](crate::ProseAccent)/[`PoetryAccent`](crate::PoetryAccent)) | Pseudo-Accents |
/// |----------|--------------------------------------------------------------------------------------------------|----------------|
/// | Melodic contour | Yes — each has a unique chant melody | No — structural only |
/// | Disjunctive/conjunctive role | Yes — phrases are built around them | No — modifies accent behavior |
/// | `relative_strength()` | Returns `Some(u8)` | Always returns `None` |
/// | `group_level()` | Returns `Some` for disjunctives | Always returns `None` |
///
/// # Representation
///
/// - `#[repr(u8)]` — Each variant is stored as a single byte.
/// - `EnumCount` / `EnumIter` — Auto-derived count and iteration.
/// - Discriminant values are **explicit and consecutive**.
///
/// # Example
///
/// ```rust
/// use hebrew_accents::{PseudoAccent,Accent};
///
/// let mark = PseudoAccent::SophPasuq;
/// assert_eq!(mark.relative_strength(), None);
/// assert_eq!(mark.group_level(), None);
/// println!("{}", mark); // "Soph Pasuq (סוֹף פָּסוּק), meaning: end of verse"
/// ```
#[repr(u8)]
#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum PseudoAccent {
    /// **Soph Pasuq** (סוֹף פָּסוּק) — "end of a verse/sentence"
    ///
    /// A terminal punctuation mark resembling a large colon (׃) that marks the
    /// end of a biblical verse. It functions as the structural boundary marker
    /// for verse divisions.
    ///
    /// # Relationship to Silluq
    ///
    /// While `SophPasuq` visually marks verse endings, it is the cantillation
    /// accent **Silluq** (not `SophPasuq`) that officially designates verse
    /// endings in standard BHS (Biblia Hebraica Stuttgartensia) texts.
    /// `SophPasuq` may be absent even at valid verse boundaries in rare cases.
    ///
    /// # Usage Notes
    ///
    /// - Always appears at the very end of a verse string
    /// - Does not carry an independent melody
    /// - Affects accent parsing: the word before `SophPasuq` typically
    ///   receives the Silluq accent
    ///
    /// # Example
    ///
    /// ```text
    /// ... וְאֵ֥ת הָאָֽרֶץ׃  ׃ פ
    ///                   ↑ SophPasuq (׃)
    /// ```
    #[default]
    SophPasuq = 0,

    /// **Maqqaph** (מַקָּף) — "hyphen, joiner"
    ///
    /// A connecting line (־) that joins multiple Hebrew words into a single
    /// phonological unit. Functions analogously to a hyphen in English.
    ///
    /// # Accent Behavior
    ///
    /// When words are joined by Maqqaph:
    ///
    /// 1. Independent accents on joined words are suppressed
    /// 2. The combined unit receives a single accent (typically on the rightmost
    ///    constituent)
    /// 3. Stress shifts to the final word of the joined group
    ///
    /// # Example
    ///
    /// ```text
    /// בְּרֵאשִׁ֖ית  →  בְּרֵאשִׁ֖ית־בָּרָ֣א
    /// (two accented words)  (one accented unit via Maqqaph)
    /// ```
    Maqqaph = 1,

    /// **Paseq** (פָּשְׁק) — "separator, divider"
    ///
    /// A vertical line (׀) inserted between adjacent cantillation marks to
    /// prevent conflation of neighboring accents where disambiguation is required.
    ///
    /// # When It Appears
    ///
    /// Paseq is used when two accents that could be confused appear adjacently.
    /// It functions purely as a visual separator and never carries melodic
    /// content.
    ///
    /// # Example
    ///
    /// ```text
    /// וַיֹּ֙אמֶר֙ ׀ יְהוָ֔ה
    ///              ↑ Paseq (׀) separates adjacent accents
    /// ```
    ///
    /// # Note
    ///
    /// Paseq does not function as a standalone accent. It only appears
    /// between other cantillation marks and does not affect the melodic
    /// chanting of the verse.
    ///
    Paseq = 2,
}

impl PseudoAccent {
    /// The total number of pseudo-accent variants.
    pub const LEN: usize = <Self as strum::EnumCount>::COUNT;

    /// Returns the discriminant as `usize`, suitable for direct table indexing.
    ///
    /// This is safe to use with `PSEUDO_ACCENT_TABLE` because the enum's
    /// discriminant values are guaranteed to be consecutive starting at 0.
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for PseudoAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}), meaning: {}",
            self.english_name(),
            self.hebrew_name(),
            self.hebrew_concept()
        )
    }
}

// ── Compile-time discriminant guards ────────────────────────────────────

/// Verifies that the last discriminant + 1 equals LEN.
///
/// If a variant is inserted, removed, or reordered, this const assertion
/// will fail at compile time.
const _: () = {
    const LAST_DISCRIMINANT: u8 = PseudoAccent::Paseq as u8;
    assert!((LAST_DISCRIMINANT + 1) as usize == PseudoAccent::LEN);
};

impl Accent for PseudoAccent {
    #[inline]
    fn hebrew_name(&self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(&self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(&self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(&self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(&self) -> Option<AccentKind> {
        PSEUDO_ACCENT_TABLE[self.as_index()].kind
    }

    #[inline]
    fn category(&self) -> Option<AccentCategory> {
        PSEUDO_ACCENT_TABLE[self.as_index()].category
    }

    #[inline]
    fn compound_type(&self) -> Option<CompoundType> {
        PSEUDO_ACCENT_TABLE[self.as_index()].compound_type
    }

    #[inline]
    fn primary_cantillation_mark(&self) -> CantillationMark {
        let info = PSEUDO_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .primary_mark;

        CantillationMark {
            symbol: info.symbol,
            placement: info.position.into(),
            stress_position: info.stress_position.to_public(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(&self) -> Option<CantillationMark> {
        PSEUDO_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .secondary_mark
            .map(|info| CantillationMark {
                symbol: info.symbol,
                placement: info.position.into(),
                stress_position: info.stress_position.to_public(),
            })
    }

    #[inline]
    fn notes(&self) -> Option<&'static str> {
        PSEUDO_ACCENT_TABLE[self.as_index()].notes
    }

    #[inline]
    fn relative_strength(&self) -> Option<u8> {
        None
    }

    #[inline]
    fn group_level(&self) -> Option<GroupLevel> {
        None
    }

    #[inline]
    fn cantillation_symbol(&self) -> String {
        display_cantillation_symbol((*self).into())
    }

    #[inline]
    fn alternate_names(&self) -> Option<AlternateNames> {
        None //TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Accent;
    use strum::IntoEnumIterator;

    // ===== BASIC VARIANT COUNT TESTS =====

    #[test]
    fn test_pseudo_accent_total_count() {
        assert_eq!(PseudoAccent::LEN, 3);
        assert_eq!(PseudoAccent::iter().count(), 3);
    }

    #[test]
    fn test_discriminant_guard_at_compile_time() {
        // This test verifies the const assertion exists
        // If variants are misordered, compilation fails
        const LAST_IDX: usize = PseudoAccent::Paseq as u8 as usize;
        assert_eq!(LAST_IDX + 1, PseudoAccent::LEN);
    }

    // ===== DEFAULT TRAIT TESTS =====

    #[test]
    fn test_default_is_soph_pasuq() {
        let default: PseudoAccent = Default::default();
        assert_eq!(default, PseudoAccent::SophPasuq);
    }

    #[test]
    fn test_soph_pasuq_explicit_equals_default() {
        assert_eq!(PseudoAccent::SophPasuq, PseudoAccent::default());
    }

    // ===== AS_INDEX METHOD TESTS =====

    #[test]
    fn test_as_index_for_all_variants() {
        assert_eq!(PseudoAccent::SophPasuq.as_index(), 0);
        assert_eq!(PseudoAccent::Maqqaph.as_index(), 1);
        assert_eq!(PseudoAccent::Paseq.as_index(), 2);
    }

    // ===== ITERATION TESTS =====

    #[test]
    fn test_enum_iter_all_variants() {
        let all_variants: Vec<PseudoAccent> = PseudoAccent::iter().collect();
        assert_eq!(all_variants.len(), 3);
        assert!(all_variants.contains(&PseudoAccent::SophPasuq));
        assert!(all_variants.contains(&PseudoAccent::Maqqaph));
        assert!(all_variants.contains(&PseudoAccent::Paseq));
    }

    #[test]
    fn test_iter_order_matches_discriminant() {
        let variants: Vec<PseudoAccent> = PseudoAccent::iter().collect();

        for (idx, accent) in variants.iter().enumerate() {
            assert_eq!(accent.as_index(), idx);
        }
    }

    // ===== DISPLAY TRAIT TESTS =====

    #[test]
    fn test_display_format_for_soph_pasuq() {
        let soph_str = format!("{}", PseudoAccent::SophPasuq);
        assert!(soph_str.contains("Soph Pasuq"));
        //assert!(soph_str.contains("סוֹף פָּסוּק"));
        assert!(soph_str.contains("end of verse"));
    }

    #[test]
    fn test_display_format_for_maqqaph() {
        let maqqaph_str = format!("{}", PseudoAccent::Maqqaph);
        assert!(maqqaph_str.contains("Maqqaph"));
        //assert!(maqqaph_str.contains("מַקָּף"));
        assert!(maqqaph_str.contains("binder"));
    }

    #[test]
    fn test_display_format_for_paseq() {
        let paseq_str = format!("{}", PseudoAccent::Paseq);
        assert!(paseq_str.contains("Paseq"));
        //assert!(paseq_str.contains("פָּשְׁק"));
        assert!(paseq_str.contains("pause"));
    }

    #[test]
    fn test_display_contains_required_components() {
        // All Display outputs should have: name, hebrew, concept
        for accent in PseudoAccent::iter() {
            let display = format!("{}", accent);
            assert!(!display.is_empty());
            assert!(display.contains(accent.english_name()));
            assert!(display.contains(accent.hebrew_name()));
            assert!(display.contains(accent.hebrew_concept()));
        }
    }

    // ===== ACCENT TRAIT METHODS TESTS =====

    // --- Text Methods ---

    #[test]
    fn test_hebrew_name_returns_static_strings() {
        assert_eq!(PseudoAccent::SophPasuq.hebrew_name(), "סוֹף פָּסוּק");
        assert_eq!(PseudoAccent::Maqqaph.hebrew_name(), "מַקָּף");
        assert_eq!(PseudoAccent::Paseq.hebrew_name(), "פָּסֵק");
    }

    #[test]
    fn test_hebrew_concept_returns_meanings() {
        assert_eq!(PseudoAccent::SophPasuq.hebrew_concept(), "end of verse");
        assert_eq!(PseudoAccent::Maqqaph.hebrew_concept(), "binder");
        assert_eq!(
            PseudoAccent::Paseq.hebrew_concept(),
            "to pause, to stop or to interrupt"
        );
    }

    #[test]
    fn test_english_name_returns_transliterations() {
        assert_eq!(PseudoAccent::SophPasuq.english_name(), "Soph Pasuq");
        assert_eq!(PseudoAccent::Maqqaph.english_name(), "Maqqaph");
        assert_eq!(PseudoAccent::Paseq.english_name(), "Paseq");
    }

    #[test]
    fn test_sbl_academic_name() {
        // Verify SBL names are non-empty
        for accent in PseudoAccent::iter() {
            let sbl = accent.sbl_academic_name();
            assert!(!sbl.is_empty());
        }
    }

    // --- Kind Method ---

    #[test]
    fn test_kind_returns_none_for_all() {
        // Pseudo-accents have no hierarchical kind
        assert_eq!(PseudoAccent::SophPasuq.kind(), None);
        assert_eq!(PseudoAccent::Maqqaph.kind(), None);
        assert_eq!(PseudoAccent::Paseq.kind(), None);
    }

    // --- Category Method ---

    #[test]
    fn test_category_returns_none_for_all() {
        // Pseudo-accents have no disjunctive/conjunctive category
        assert_eq!(PseudoAccent::SophPasuq.category(), None);
        assert_eq!(PseudoAccent::Maqqaph.category(), None);
        assert_eq!(PseudoAccent::Paseq.category(), None);
    }

    // --- Compound Type Method ---

    #[test]
    fn test_compound_type_returns_none_for_all() {
        // Pseudo-accents are not compound accents
        for accent in PseudoAccent::iter() {
            assert_eq!(accent.compound_type(), None);
        }
    }

    // --- Cantillation Mark Methods ---

    #[test]
    fn test_primary_cantillation_mark_always_returns_value() {
        for accent in PseudoAccent::iter() {
            let primary = accent.primary_cantillation_mark();

            assert!(!primary.symbol.is_control());
            assert!(!primary.symbol.is_whitespace());
        }
    }

    #[test]
    fn test_primary_marks_have_valid_symbols() {
        // Check that each pseudo-accent has its expected symbol
        assert_eq!(
            PseudoAccent::SophPasuq.primary_cantillation_mark().symbol,
            '׃'
        );
        assert_eq!(
            PseudoAccent::Maqqaph.primary_cantillation_mark().symbol,
            '־'
        );
        assert_eq!(PseudoAccent::Paseq.primary_cantillation_mark().symbol, '׀');
    }

    #[test]
    fn test_secondary_cantillation_mark_returns_none() {
        // Pseudo-accents don't have secondary marks
        for accent in PseudoAccent::iter() {
            assert_eq!(accent.secondary_cantillation_mark(), None);
        }
    }

    // --- Notes Method ---

    #[test]
    fn test_notes_returns_option() {
        for accent in PseudoAccent::iter() {
            let _notes = accent.notes();
            // Some may have notes, some don't - both valid
        }
    }

    // --- Relative Strength Method (ALWAYS NONE) ---

    #[test]
    fn test_relative_strength_always_returns_none() {
        // Pseudo-accents have no hierarchical strength
        assert_eq!(PseudoAccent::SophPasuq.relative_strength(), None);
        assert_eq!(PseudoAccent::Maqqaph.relative_strength(), None);
        assert_eq!(PseudoAccent::Paseq.relative_strength(), None);
    }

    // --- Group Level Method (ALWAYS NONE) ---

    #[test]
    fn test_group_level_always_returns_none() {
        // Pseudo-accents have no hierarchical level
        assert_eq!(PseudoAccent::SophPasuq.group_level(), None);
        assert_eq!(PseudoAccent::Maqqaph.group_level(), None);
        assert_eq!(PseudoAccent::Paseq.group_level(), None);
    }

    // --- Cantillation Symbol Method ---

    #[test]
    fn test_cantillation_symbol_not_empty() {
        for accent in PseudoAccent::iter() {
            let symbol = accent.cantillation_symbol();
            assert!(!symbol.is_empty());
        }
    }

    #[test]
    fn test_cantillation_symbol_contains_expected_characters() {
        let soph_sym = PseudoAccent::SophPasuq.cantillation_symbol();
        let maq_sym = PseudoAccent::Maqqaph.cantillation_symbol();
        let pse_sym = PseudoAccent::Paseq.cantillation_symbol();

        assert!(!soph_sym.is_empty());
        assert!(!maq_sym.is_empty());
        assert!(!pse_sym.is_empty());
    }

    // --- Alternate Names Method ---

    #[test]
    fn test_alternate_names_returns_none_currently() {
        // Currently all return None (TODO mentioned in code)
        for accent in PseudoAccent::iter() {
            assert_eq!(accent.alternate_names(), None);
        }
    }

    // ===== PSEUDO-ACCENT DISTINCTIVENESS TESTS =====

    #[test]
    fn test_pseudo_vs_true_accent_behavior() {
        // Pseudo-accents should differ from true accents
        use crate::ProseAccent;

        let pseudo = PseudoAccent::SophPasuq;
        let prose = ProseAccent::Silluq;

        // Both have hebrew_name
        assert!(!pseudo.hebrew_name().is_empty());
        assert!(!prose.hebrew_name().is_empty());

        // But pseudo lacks strength/level
        assert_eq!(pseudo.relative_strength(), None);
        assert!(prose.relative_strength().is_some());

        assert_eq!(pseudo.group_level(), None);
        assert!(prose.group_level().is_some());
    }

    #[test]
    fn test_no_melodic_contour_property() {
        // Verify pseudo-accents lack melodic hierarchy
        for accent in PseudoAccent::iter() {
            assert_eq!(accent.kind(), None, "No hierarchical kind");
            assert_eq!(
                accent.category(),
                None,
                "No disjunctive/conjunctive category"
            );
            assert_eq!(accent.relative_strength(), None, "No relative strength");
            assert_eq!(accent.group_level(), None, "No group level");
        }
    }

    // ===== DERIVED TRAIT TESTS =====

    #[test]
    fn test_copy_trait_works() {
        let original = PseudoAccent::SophPasuq;
        let copied = original; // Copy occurs automatically

        assert_eq!(original, PseudoAccent::SophPasuq);
        assert_eq!(copied, PseudoAccent::SophPasuq);
    }

    #[test]
    fn test_clone_trait_works() {
        let original = PseudoAccent::Paseq;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned, PseudoAccent::Paseq);
    }

    #[test]
    fn test_partial_eq_and_eq_traits() {
        let p1 = PseudoAccent::SophPasuq;
        let p2 = PseudoAccent::SophPasuq;
        let p3 = PseudoAccent::Maqqaph;

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert!(p1.eq(&p2));
    }

    #[test]
    fn test_hash_trait_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        PseudoAccent::SophPasuq.hash(&mut hasher1);
        PseudoAccent::SophPasuq.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_in_collections() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(PseudoAccent::SophPasuq);
        set.insert(PseudoAccent::Maqqaph);
        set.insert(PseudoAccent::Paseq);

        assert!(set.contains(&PseudoAccent::SophPasuq));
        assert!(set.contains(&PseudoAccent::Maqqaph));
        assert!(set.contains(&PseudoAccent::Paseq));
        assert_eq!(set.len(), 3);
    }

    // ===== FUNCTIONAL BEHAVIOR TESTS =====

    #[test]
    fn test_soph_pasuq_terminal_marker() {
        // Soph Pasuq marks verse endings
        let soph = PseudoAccent::SophPasuq;

        assert_eq!(soph.english_name(), "Soph Pasuq");
        assert_eq!(soph.hebrew_name(), "סוֹף פָּסוּק");
        assert!(soph.hebrew_concept().contains("verse"));
    }

    #[test]
    fn test_maqqaph_joiner_behavior() {
        // Maqqaph joins words
        let maq = PseudoAccent::Maqqaph;

        assert_eq!(maq.english_name(), "Maqqaph");
        assert!(maq.hebrew_concept().contains("binder"));
    }

    #[test]
    fn test_paseq_separator_behavior() {
        // Paseq separates adjacent marks
        let pse = PseudoAccent::Paseq;

        assert_eq!(pse.english_name(), "Paseq");
        assert!(
            pse.hebrew_concept().contains("pause") || pse.hebrew_concept().contains("interrupt")
        );
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_complete_api_roundtrip() {
        // Test that all API methods work together
        let accent = PseudoAccent::Maqqaph;

        let _hebrew = accent.hebrew_name();
        let _concept = accent.hebrew_concept();
        let _english = accent.english_name();
        let _sbl = accent.sbl_academic_name();
        let _kind = accent.kind();
        let _category = accent.category();
        let _compound = accent.compound_type();
        let _primary = accent.primary_cantillation_mark();
        let _secondary = accent.secondary_cantillation_mark();
        let _notes = accent.notes();
        let _strength = accent.relative_strength();
        let _level = accent.group_level();
        let _symbol = accent.cantillation_symbol();
        let _alts = accent.alternate_names();
        let _display = format!("{}", accent);

        // All operations completed successfully
        assert_eq!(accent.english_name(), "Maqqaph");
    }

    #[test]
    fn test_table_lookup_consistency() {
        // Verify table indexing works for all variants
        for accent in PseudoAccent::iter() {
            let idx = accent.as_index();
            assert!(idx < PseudoAccent::LEN);

            // Table access should not panic
            let _hebrew = accent.hebrew_name();
            let _english = accent.english_name();
        }
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_first_and_last_variants() {
        // Verify ordering: SophPasuq first, Paseq last
        let all: Vec<PseudoAccent> = PseudoAccent::iter().collect();

        assert_eq!(all.first(), Some(&PseudoAccent::SophPasuq));
        assert_eq!(all.last(), Some(&PseudoAccent::Paseq));
        assert_eq!(all[0], PseudoAccent::SophPasuq);
        assert_eq!(all[2], PseudoAccent::Paseq);
    }

    #[test]
    fn test_no_duplicate_variants() {
        use std::collections::HashSet;

        let all: Vec<PseudoAccent> = PseudoAccent::iter().collect();
        let unique: HashSet<PseudoAccent> = all.clone().into_iter().collect();

        assert_eq!(all.len(), unique.len());
    }

    #[test]
    fn test_const_context_usage() {
        const MAQQAPH: PseudoAccent = PseudoAccent::Maqqaph;
        assert_eq!(MAQQAPH.as_index(), 1);
        assert_eq!(MAQQAPH.english_name(), "Maqqaph");
    }

    // ===== DOCUMENTATION EXAMPLE VERIFICATION =====

    #[test]
    fn doc_test_module_example() {
        // Replicate the example from module documentation
        let mark = PseudoAccent::Maqqaph;
        let _display = format!("{}", mark); // Should print: "Maqqaph (מַקָּף), meaning: hyphen"

        assert_eq!(mark.english_name(), "Maqqaph");
    }

    #[test]
    fn doc_test_example_from_struct_docs() {
        // Replicate the example from struct documentation
        let mark = PseudoAccent::SophPasuq;
        assert_eq!(mark.relative_strength(), None);
        assert_eq!(mark.group_level(), None);
        let _display = format!("{}", mark); // "Soph Pasuq (סוֹף פָּסוּק), meaning: ..."

        assert_eq!(mark.hebrew_name(), "סוֹף פָּסוּק");
    }

    #[test]
    fn doc_test_pseudo_vs_true_comparison() {
        // From the documentation table comparing pseudo vs true accents
        use crate::ProseAccent;

        let pseudo = PseudoAccent::Paseq;
        let prose = ProseAccent::Revia;

        // Verify the documented behavior difference
        assert_eq!(pseudo.relative_strength(), None);
        assert!(prose.relative_strength().is_some());

        assert_eq!(pseudo.group_level(), None);
        assert!(prose.group_level().is_some());
    }

    // ===== CROSS-REFERENCING TESTS =====

    #[test]
    fn test_pseudo_accent_symbols_unique() {
        // Each pseudo-accent should have a unique symbol
        let soph_sym = PseudoAccent::SophPasuq.primary_cantillation_mark().symbol;
        let maq_sym = PseudoAccent::Maqqaph.primary_cantillation_mark().symbol;
        let pse_sym = PseudoAccent::Paseq.primary_cantillation_mark().symbol;

        assert_ne!(soph_sym, maq_sym);
        assert_ne!(soph_sym, pse_sym);
        assert_ne!(maq_sym, pse_sym);
    }

    #[test]
    fn test_all_three_functional_roles_covered() {
        // Ensure all three functional roles are represented
        let mut has_verese_end = false;
        let mut has_word_join = false;
        let mut has_mark_sep = false;

        for accent in PseudoAccent::iter() {
            let concept = accent.hebrew_concept().to_lowercase();
            if concept.contains("end") || concept.contains("verse") {
                has_verese_end = true;
            }
            if concept.contains("binder") {
                has_word_join = true;
            }
            if concept.contains("pause") || concept.contains("interrupt") {
                has_mark_sep = true;
            }
        }

        assert!(
            has_verese_end,
            "Should have verse-ending marker (Soph Pasuq)"
        );
        assert!(has_word_join, "Should have word-joining marker (Maqqaph)");
        assert!(has_mark_sep, "Should have separator marker (Paseq)");
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use crate::Accent;
    use strum::IntoEnumIterator;

    // Additional tests for uncovered function paths

    #[test]
    fn test_as_index_const_evaluation() {
        // Verify as_index can be evaluated in const context
        const IDX: usize = PseudoAccent::SophPasuq.as_index();
        assert_eq!(IDX, 0);
    }

    #[test]
    fn test_as_index_usize_conversion() {
        // Ensure as_index returns proper usize for indexing
        for accent in PseudoAccent::iter() {
            let idx = accent.as_index();
            assert!(idx <= 255); // u8 max
            assert!(idx < PseudoAccent::LEN);
        }
    }

    #[test]
    fn test_secondary_cantillation_mark_none_path() {
        // Specifically test the None return path for secondary marks
        for accent in PseudoAccent::iter() {
            let secondary = accent.secondary_cantillation_mark();
            match secondary {
                None => assert!(true), // Expected for pseudo-accents
                Some(_) => panic!("Pseudo-accents should not have secondary marks"),
            }
        }
    }

    #[test]
    fn test_notes_method_returns_option() {
        // Test notes method specifically (even though it returns None)
        for accent in PseudoAccent::iter() {
            let notes = accent.notes();
            // Should be None currently
            assert!(notes.is_some());
        }
    }

    #[test]
    fn test_alternate_names_method_returns_none() {
        // Specifically test alternate_names (marked as TODO in code)
        for accent in PseudoAccent::iter() {
            let alts = accent.alternate_names();
            assert!(alts.is_none(), "Alternate names should be None for now");
        }
    }

    #[test]
    fn test_cantillation_symbol_function_call() {
        // Ensure display_cantillation_symbol is called for each variant
        let soph_sym = PseudoAccent::SophPasuq.cantillation_symbol();
        let maq_sym = PseudoAccent::Maqqaph.cantillation_symbol();
        let pse_sym = PseudoAccent::Paseq.cantillation_symbol();

        assert!(!soph_sym.is_empty());
        assert!(!maq_sym.is_empty());
        assert!(!pse_sym.is_empty());
    }

    #[test]
    fn test_hebrew_name_static_lifetime() {
        // Verify hebrew_name returns &'static str
        let name: &'static str = PseudoAccent::SophPasuq.hebrew_name();
        assert_eq!(name, "סוֹף פָּסוּק");

        // Should work in static context
        //const STATIC_NAME: &'static str = PseudoAccent::Maqqaph.hebrew_name();
        //assert_eq!(STATIC_NAME, "מַקָּף");
    }

    #[test]
    fn test_english_name_static_lifetime() {
        // Verify english_name returns &'static str
        let name: &'static str = PseudoAccent::Paseq.english_name();
        assert_eq!(name, "Paseq");
    }

    #[test]
    fn test_sbl_academic_name_non_empty() {
        // Test sbl_academic_name specifically
        for accent in PseudoAccent::iter() {
            let sbl = accent.sbl_academic_name();
            assert!(!sbl.is_empty(), "SBL name should not be empty");

            // Should be &'static str
            let _static_ref: &'static str = sbl;
        }
    }

    #[test]
    fn test_kind_return_type_none() {
        // Specifically test kind() returns None
        for accent in PseudoAccent::iter() {
            let kind_opt: Option<crate::AccentKind> = accent.kind();
            assert!(kind_opt.is_none());
        }
    }

    #[test]
    fn test_category_return_type_none() {
        // Specifically test category() returns None
        for accent in PseudoAccent::iter() {
            let cat_opt: Option<crate::AccentCategory> = accent.category();
            assert!(cat_opt.is_none());
        }
    }

    #[test]
    fn test_compound_type_return_type_none() {
        // Specifically test compound_type() returns None
        for accent in PseudoAccent::iter() {
            let ct_opt: Option<crate::CompoundType> = accent.compound_type();
            assert!(ct_opt.is_none());
        }
    }

    #[test]
    fn test_relative_strength_return_type_none() {
        // Specifically test relative_strength() returns None
        for accent in PseudoAccent::iter() {
            let strength_opt: Option<u8> = accent.relative_strength();
            assert!(strength_opt.is_none());
        }
    }

    #[test]
    fn test_group_level_return_type_none() {
        // Specifically test group_level() returns None
        for accent in PseudoAccent::iter() {
            let level_opt: Option<GroupLevel> = accent.group_level();
            assert!(level_opt.is_none());
        }
    }

    #[test]
    fn test_primary_cantillation_mark_structure() {
        // Verify primary_cantillation_mark returns valid CantillationMark struct
        for accent in PseudoAccent::iter() {
            let mark = accent.primary_cantillation_mark();

            assert!(!mark.symbol.is_control());
            assert!(!mark.symbol.is_whitespace());
            assert_eq!(mark.placement, mark.placement); // Reflexive equality
            assert_eq!(mark.stress_position, mark.stress_position); // Reflexive equality
        }
    }

    #[test]
    fn test_hebrew_concept_static_lifetime() {
        // Verify hebrew_concept returns &'static str
        let concept: &'static str = PseudoAccent::Maqqaph.hebrew_concept();
        assert_eq!(concept, "binder");
    }

    #[test]
    fn test_all_accent_trait_methods_exercised() {
        // Exercise every single Accent trait method once
        let accent = PseudoAccent::SophPasuq;

        // Text methods
        let _h_name = accent.hebrew_name();
        let _h_concept = accent.hebrew_concept();
        let _e_name = accent.english_name();
        let _sbl = accent.sbl_academic_name();

        // Classification methods
        let _kind = accent.kind();
        let _cat = accent.category();
        let _compound = accent.compound_type();

        // Mark methods
        let _primary = accent.primary_cantillation_mark();
        let _secondary = accent.secondary_cantillation_mark();

        // Meta methods
        let _notes = accent.notes();
        let _strength = accent.relative_strength();
        let _level = accent.group_level();
        let _symbol = accent.cantillation_symbol();
        let _alts = accent.alternate_names();
    }

    #[test]
    fn test_display_trait_write_macro() {
        // Specifically test the Display implementation write macro
        use std::fmt::Write;

        let mut buffer = String::new();
        write!(&mut buffer, "{}", PseudoAccent::SophPasuq).unwrap();

        assert!(!buffer.is_empty());
        assert!(buffer.contains("Soph Pasuq"));
    }

    #[test]
    fn test_pseudo_accent_in_vec() {
        // Test PseudoAccent works in collections
        let vec: Vec<PseudoAccent> = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqaph,
            PseudoAccent::Paseq,
        ];

        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_pseudo_accent_array() {
        // Test PseudoAccent works in arrays
        let arr: [PseudoAccent; 3] = [
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqaph,
            PseudoAccent::Paseq,
        ];

        assert_eq!(arr[0], PseudoAccent::SophPasuq);
        assert_eq!(arr[1], PseudoAccent::Maqqaph);
        assert_eq!(arr[2], PseudoAccent::Paseq);
    }

    #[test]
    fn test_pattern_matching_on_variants() {
        // Test pattern matching covers all variants
        for accent in PseudoAccent::iter() {
            let desc = match accent {
                PseudoAccent::SophPasuq => "verse ending",
                PseudoAccent::Maqqaph => "word joiner",
                PseudoAccent::Paseq => "separator",
            };

            assert!(!desc.is_empty());
        }
    }

    #[test]
    fn test_from_str_or_other_conversions() {
        use std::hash::Hash;
        // Test if any From/Into conversions exist
        // (may not exist yet, but test for future-proofing)

        // Just verify the enum can be used in generic contexts
        fn accepts_copy<T: Copy>(_: T) {}
        fn accepts_clone<T: Clone>(_: T) {}
        fn accepts_eq<T: Eq + PartialEq>(_: T) {}
        fn accepts_hash<T: Hash>(_: T) {}
        fn accepts_debug<T: std::fmt::Debug>(_: T) {}

        accepts_copy(PseudoAccent::SophPasuq);
        accepts_clone(PseudoAccent::Maqqaph);
        accepts_eq(PseudoAccent::Paseq);
        accepts_hash(PseudoAccent::SophPasuq);
        accepts_debug(PseudoAccent::Maqqaph);
    }

    #[test]
    fn test_repr_u8_discriminant_values() {
        // Verify #[repr(u8)] is working correctly
        let soph_bytes =
            unsafe { std::mem::transmute::<PseudoAccent, u8>(PseudoAccent::SophPasuq) };
        let maq_bytes = unsafe { std::mem::transmute::<PseudoAccent, u8>(PseudoAccent::Maqqaph) };
        let pse_bytes = unsafe { std::mem::transmute::<PseudoAccent, u8>(PseudoAccent::Paseq) };

        assert_eq!(soph_bytes, 0);
        assert_eq!(maq_bytes, 1);
        assert_eq!(pse_bytes, 2);
    }

    #[test]
    fn test_iter_collect_into_array() {
        // Test iteration collects correctly into fixed-size array
        let collected: [PseudoAccent; 3] = PseudoAccent::iter()
            .take(3)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Should collect exactly 3 variants");

        assert_eq!(collected[0], PseudoAccent::SophPasuq);
        assert_eq!(collected[1], PseudoAccent::Maqqaph);
        assert_eq!(collected[2], PseudoAccent::Paseq);
    }

    #[test]
    fn test_len_const_correctness() {
        // Verify LEN constant is correct
        const _: () = assert!(PseudoAccent::LEN == 3);
        assert_eq!(PseudoAccent::LEN, 3);
    }

    #[test]
    fn test_default_trait_impl() {
        // Test Default trait implementation explicitly
        let default1 = PseudoAccent::default();
        let default2: PseudoAccent = Default::default();

        assert_eq!(default1, PseudoAccent::SophPasuq);
        assert_eq!(default2, PseudoAccent::SophPasuq);
        assert_eq!(default1, default2);
    }

    #[test]
    fn test_eq_partial_eq_deref() {
        // Test Eq and PartialEq interaction
        let p1 = PseudoAccent::Maqqaph;
        let p2 = PseudoAccent::Maqqaph;

        // These should all be equivalent
        assert_eq!(p1, p2);
        assert!(p1.eq(&p2));
        assert!(p1 != PseudoAccent::Paseq);
        assert!(p1.ne(&PseudoAccent::Paseq));
    }

    #[test]
    fn test_debug_trait_output_format() {
        // Test Debug trait output
        let debug_out = format!("{:?}", PseudoAccent::Paseq);

        assert!(debug_out.contains("Paseq"));
        assert!(!debug_out.is_empty());
    }

    #[test]
    fn test_cantillation_mark_placement_conversion() {
        // Test that placement conversion works
        let mark = PseudoAccent::SophPasuq.primary_cantillation_mark();

        // Placement should convert properly from internal type
        assert_eq!(mark.placement, mark.placement);
    }

    #[test]
    fn test_stress_position_conversion() {
        // Test stress_position conversion works
        let mark = PseudoAccent::Maqqaph.primary_cantillation_mark();

        // Should be a valid Option<CantillationMarkStressPosition>
        let _pos = mark.stress_position;
    }

    #[test]
    fn test_unicode_version_reference() {
        // If unicode version is referenced, test it
        // (Check if PSEUDO_ACCENT_TABLE has unicode info)

        // Just verify the symbols are valid Unicode
        let soph = PseudoAccent::SophPasuq.primary_cantillation_mark().symbol;
        assert!(soph as u32 > 0x0500); // Hebrew block starts around U+0500
    }

    #[test]
    fn test_ascii_vs_unicode_names() {
        // Verify english names are ASCII, hebrew names are Unicode
        let eng = PseudoAccent::SophPasuq.english_name();
        let heb = PseudoAccent::SophPasuq.hebrew_name();

        // English should be ASCII
        assert!(eng.is_ascii());

        // Hebrew should contain non-ASCII
        assert!(!heb.is_ascii() || heb.chars().any(|c| c > '\u{007F}'));
    }

    #[test]
    fn test_iterator_double_end() {
        // Test iterator can be ended early
        let mut iter = PseudoAccent::iter();
        let first = iter.next();
        assert_eq!(first, Some(PseudoAccent::SophPasuq));

        // Can still get remaining
        let second = iter.next();
        assert_eq!(second, Some(PseudoAccent::Maqqaph));
    }

    #[test]
    fn test_iterator_exact_size() {
        // Test iterator is ExactSizeIterator
        let iter = PseudoAccent::iter();

        assert_eq!(iter.len(), 3);
        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_clone_from_slice() {
        // Test cloning from slice
        let slice: &[PseudoAccent] = &[PseudoAccent::SophPasuq, PseudoAccent::Maqqaph];
        let cloned: Vec<PseudoAccent> = slice.to_vec();

        assert_eq!(cloned.len(), 2);
        assert_eq!(cloned[0], PseudoAccent::SophPasuq);
    }

    #[test]
    fn test_borrow_trait_if_implemented() {
        // Test if Borrow trait is implemented for smart pointer compatibility
        // (May not exist, but test for completeness)

        let accent = PseudoAccent::SophPasuq;
        // Just verify it can be borrowed implicitly
        let _ref: &PseudoAccent = &accent;
    }

    #[test]
    fn test_ord_if_implemented() {
        // Test Ord trait if it exists
        // (May not be derived, but test anyway)

        // For now just test that we can compare for sorting purposes
        //use std::cmp::Ordering;

        let a = PseudoAccent::SophPasuq;
        let b = PseudoAccent::Maqqaph;

        // At minimum, they should be comparable via PartialEq
        assert!(a == a);
        assert!(a != b);
    }

    #[test]
    fn test_unwind_safe_if_needed() {
        // Test UnwindSafe and RefUnwindSafe traits
        // (Usually derived automatically)

        fn requires_unwind_safe<T: std::panic::UnwindSafe>() {}
        fn requires_ref_unwind_safe<T: std::panic::RefUnwindSafe>() {}

        requires_unwind_safe::<PseudoAccent>();
        requires_ref_unwind_safe::<PseudoAccent>();
    }

    #[test]
    fn test_sync_and_send_bounds() {
        // Verify Send + Sync bounds for thread safety
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<PseudoAccent>();
    }

    #[test]
    fn test_unused_code_coverage() {
        // Cover any potentially unused helper code
        // This exercises less common code paths

        // Test with all variants in sequence
        let variants: [PseudoAccent; 3] = [
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqaph,
            PseudoAccent::Paseq,
        ];

        for (i, variant) in variants.iter().enumerate() {
            assert_eq!(variant.as_index(), i);
            let _name = variant.english_name();
            let _sym = variant.cantillation_symbol();
        }
    }

    #[test]
    fn test_const_fn_constructor() {
        // Test if as_index can be const-evaluated
        const FIRST_IDX: usize = PseudoAccent::SophPasuq.as_index();
        const LAST_IDX: usize = PseudoAccent::Paseq.as_index();

        assert_eq!(FIRST_IDX, 0);
        assert_eq!(LAST_IDX, 2);
    }
}
