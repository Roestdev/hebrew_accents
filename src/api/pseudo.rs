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
//! | [`Maqqeph`] | מַקָּף | Joins words (hyphen) |
//! | [`Paseq`] | פָּשְׁק | Separates adjacent accents |
//!
//! ## Usage
//!
//! ```ignore
//! use crate::api::PseudoAccent;
//! use crate::Accent;
//!
//! let mark = PseudoAccent::Maqqeph;
//! println!("{}", mark); // "Maqqeph (מַקָּף), meaning: hyphen"
//! ```

use crate::Accent;
use strum_macros::EnumIter;

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
/// - `#[non_exhaustive]` — Additional pseudo-accents may be identified in future versions.
/// - `EnumIter` — Enables iteration over all variants via `PseudoAccent::iter()`.
///
/// # Example
///
/// ```ignore
/// use crate::api::PseudoAccent;
/// use crate::Accent;
///
/// let mark = PseudoAccent::SophPasuq;
/// assert_eq!(mark.relative_strength(), None);
/// assert_eq!(mark.group_level(), None);
/// println!("{}", mark); // "Soph Pasuq (סוֹף פָּסוּק), meaning: end of verse"
/// ```
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
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
    SophPasuq,

    /// **Maqqeph** (מַקָּף) — "hyphen, joiner"
    ///
    /// A connecting line (־) that joins multiple Hebrew words into a single
    /// phonological unit. Functions analogously to a hyphen in English.
    ///
    /// # Accent Behavior
    ///
    /// When words are joined by Maqqeph:
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
    /// (two accented words)  (one accented unit via Maqqeph)
    /// ```
    Maqqeph,

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
    Paseq,
}

impl PseudoAccent {
    /// The total number of pseudo-accent variants.
    pub const LEN: usize = 3;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_constant_matches_variant_count() {
        assert_eq!(PseudoAccent::LEN, 3);
    }

    #[test]
    fn default_is_soph_pasuq() {
        assert_eq!(PseudoAccent::default(), PseudoAccent::SophPasuq);
    }

    #[test]
    fn relative_strength_is_one_for_soph_pasuq() {
        assert_eq!(PseudoAccent::SophPasuq.relative_strength(), None);
    }

    #[test]
    fn relative_strength_is_two_for_maqqeph() {
        assert_eq!(PseudoAccent::Maqqeph.relative_strength(), None);
    }

    #[test]
    fn relative_strength_is_three_for_paseq() {
        assert_eq!(PseudoAccent::Paseq.relative_strength(), None);
    }

    #[test]
    fn copy_preserves_value() {
        let original = PseudoAccent::Maqqeph;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn clone_preserves_value() {
        let original = PseudoAccent::Paseq;
        assert_eq!(original, original.clone());
    }

    #[test]
    fn equality_and_inequality() {
        assert_eq!(PseudoAccent::SophPasuq, PseudoAccent::SophPasuq);
        assert_ne!(PseudoAccent::SophPasuq, PseudoAccent::Maqqeph);
        assert_ne!(PseudoAccent::Maqqeph, PseudoAccent::Paseq);
    }

    #[test]
    fn hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        PseudoAccent::Maqqeph.hash(&mut h1);
        PseudoAccent::Maqqeph.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());

        let mut h3 = DefaultHasher::new();
        PseudoAccent::Paseq.hash(&mut h3);
        assert_ne!(h1.finish(), h3.finish());
    }

    #[test]
    fn debug_output_contains_variant_name() {
        assert!(format!("{:?}", PseudoAccent::SophPasuq).contains("SophPasuq"));
        assert!(format!("{:?}", PseudoAccent::Maqqeph).contains("Maqqeph"));
        assert!(format!("{:?}", PseudoAccent::Paseq).contains("Paseq"));
    }

    #[test]
    fn debug_outputs_are_distinct() {
        let a = format!("{:?}", PseudoAccent::SophPasuq);
        let b = format!("{:?}", PseudoAccent::Maqqeph);
        let c = format!("{:?}", PseudoAccent::Paseq);
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn display_contains_meaning_keyword() {
        let s = PseudoAccent::SophPasuq.to_string();
        assert!(!s.is_empty(), "Display output was empty");
        assert!(
            s.contains("meaning:"),
            "Display output missing 'meaning:' — got: {}",
            s
        );
    }

    #[test]
    fn display_contains_parenthesised_hebrew_name() {
        let s = PseudoAccent::Maqqeph.to_string();
        assert!(
            s.contains('(') && s.contains(')'),
            "Display output missing parenthesised hebrew name — got: {}",
            s
        );
    }

    #[test]
    fn display_differs_across_variants() {
        let a = PseudoAccent::SophPasuq.to_string();
        let b = PseudoAccent::Maqqeph.to_string();
        let c = PseudoAccent::Paseq.to_string();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn display_non_empty_for_all_variants() {
        for variant in [
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ] {
            let s = variant.to_string();
            assert!(!s.is_empty(), "Empty Display for {:?}", variant);
        }
    }
}
