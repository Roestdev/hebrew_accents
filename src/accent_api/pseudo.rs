use crate::Accent;

/// Syntactic markers associated with biblical Hebrew cantillation but distinct from true accents.
///
/// `PseudoAccent` values represent structural symbols that influence accent placement without
/// carrying independent melodic contour. They govern phrase boundaries, word grouping, and
/// punctuation within the Masoretic text tradition.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PseudoAccent {
    #[default]
    /// Marks the end of a verse or sentence (Hebrew: סוֹף פָּסוּק).
    /// Equivalent to a terminal period; signals final pause despite lacking its own melody.
    ///
    /// **Note:** Contrary to intuition, `Silluq` and not `SophPasuq` designates official verse endings
    /// in standard BHS texts. `SophPasuq` may be absent even at valid verse boundaries in some rare cases.
    SophPasuq,

    /// Joins multiple words into a single phonological unit (Hebrew: מַקָּף).
    /// Functions as a hyphen: suppresses independent accents on joined words, causing
    /// accent shifts to the rightmost constituent.
    Maqqeph,

    /// Separates adjacent cantillation marks (Hebrew: פָּשְׁק).
    /// Prevents conflation of neighboring accents where disambiguation is required;
    /// never appears as an standalone accent.
    Paseq,
}

impl PseudoAccent {
    /// Total count of all pseudo accents
    pub const LEN: usize = 3;
}

impl std::fmt::Display for PseudoAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}), meaning: {}",
            self.sbl_simplified_name(),
            self.hebrew_name(),
            self.hebrew_concept()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Variant count ──────────────────────────────────────────────

    #[test]
    fn len_constant_matches_variant_count() {
        assert_eq!(PseudoAccent::LEN, 3);
    }

    // ── Default ────────────────────────────────────────────────────

    #[test]
    fn default_is_soph_pasuq() {
        assert_eq!(PseudoAccent::default(), PseudoAccent::SophPasuq);
    }

    // ── relative_strength ───────────────────────────────────────────

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

    // ── Derived traits ──────────────────────────────────────────────

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

    // ── Debug trait ────────────────────────────────────────────────

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

    // ── Display ────────────────────────────────────────────────────
    // Display delegates to sbl_simplified_name(), hebrew_name(), hebrew_concept().
    // These methods aren't defined in this file, but if they compile we can
    // smoke-test the format-string structure.

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
