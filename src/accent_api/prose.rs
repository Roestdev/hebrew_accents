use crate::Accent;
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// All variants of the Hebrew Prose Accents
/// 18 Disjunctives and 10 Conjunctives.
#[repr(u8)]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum ProseAccent {
    #[default]
    /// Primary disjunctive prose accent Silluq
    Silluq,
    /// Primary disjunctive prose accent accent Atnach
    Atnach,
    /// Primary disjunctive prose accent Segolta
    Segolta,
    /// Primary disjunctive prose accent Shalshelet
    Shalshelet,
    /// Primary disjunctive prose accent Zaqeph Qaton
    ZaqephQatan,
    /// Primary disjunctive prose accent Zaqeph Gadol
    ZaqephGadol,
    /// Primary disjunctive prose accent Revia
    Revia,
    /// Primary disjunctive prose accent Tiphcha,
    Tiphcha,
    /// Primary disjunctive prose accent Zarqa
    Zarqa,
    /// Primary disjunctive prose accent Pashta
    Pashta,
    /// Primary disjunctive prose accent Yetiv
    Yetiv,
    /// Primary disjunctive prose accent Tevir
    Tevir,
    /// Primary disjunctive prose accent Geresh
    Geresh,
    /// Primary disjunctive prose accent Gershayim
    Gershayim,
    /// Primary disjunctive prose accent Pazer
    Pazer,
    /// Primary disjunctive prose accent Pazer Gadol
    PazerGadol,
    /// Primary disjunctive prose accent Telisha Gedolah
    TelishaGedolah,
    /// Primary disjunctive prose accent Legarmeh
    Legarmeh,
    /// Primary conjunctive prose accent Munach
    Munach,
    /// Primary conjunctive prose accent Mahpakh
    Mahpakh,
    /// Primary conjunctive prose accent Merkha
    Merkha,
    /// Primary conjunctive prose accent Merkha Kephulah
    MerkhaKephulah,
    /// Primary conjunctive prose accent Darga
    Darga,
    /// Primary conjunctive prose accent Azla
    Azla,
    /// Primary conjunctive prose accent Telisha Qetannah
    TelishaQetannah,
    /// Primary conjunctive prose accent Galgal
    Galgal,
    /// Secondary conjunctive prose accent Meayla
    Meayla,
    /// Secondary conjunctive prose accent Meteg
    Meteg,
}

impl ProseAccent {
    /// The total number of prose accents
    pub const LEN: usize = 28;
}

impl std::fmt::Display for ProseAccent {
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
    use strum::IntoEnumIterator;

    #[test]
    fn test_enum_iter_count() {
        // Verify that Iter yields exactly LEN items
        assert_eq!(ProseAccent::iter().count(), ProseAccent::LEN);
        assert_eq!(ProseAccent::iter().count(), 28);
    }

    #[test]
    fn test_enum_iter_all_variants_present() {
        // Ensure every defined variant appears in iteration
        let mut found = std::collections::HashSet::new();

        for accent in ProseAccent::iter() {
            // Insert returns false if already present (duplicate detection)
            assert!(found.insert(accent), "Duplicate");
        }
        assert_eq!(found.len(), ProseAccent::LEN);
    }
    // ── Variant count ──────────────────────────────────────────────

    #[test]
    fn len_constant_matches_variant_count() {
        // 18 disjunctives + 10 conjunctives = 28
        assert_eq!(ProseAccent::LEN, 28);
    }

    #[test]
    fn discriminant_values_are_sequential() {
        // Because of #[repr(u8)] the discriminants start at 0 and increment.
        assert_eq!(ProseAccent::Silluq as u8, 0);
        assert_eq!(ProseAccent::Atnach as u8, 1);
        assert_eq!(ProseAccent::Legarmeh as u8, 17);
        assert_eq!(ProseAccent::Munach as u8, 18);
        assert_eq!(ProseAccent::Meteg as u8, 27);
    }

    // ── Default ────────────────────────────────────────────────────

    #[test]
    fn default_is_silluq() {
        let default = ProseAccent::default();
        assert_eq!(default, ProseAccent::Silluq);
    }

    // ── relative_strength ──────────────────────────────────────────

    #[test]
    fn relative_strength_starts_at_one_for_first_variant() {
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
    }

    #[test]
    fn relative_strength_is_discriminant_plus_one() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            assert_eq!(accent.relative_strength(), Some(v + 1));
        }
    }

    #[test]
    fn relative_strength_strongest_is_len() {
        assert_eq!(ProseAccent::Meteg.relative_strength(), Some(28));
    }

    #[test]
    fn relative_strength_monotonically_increasing() {
        let mut prev: u8 = 0;
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let strength = accent.relative_strength();
            assert!(
                strength > Some(prev),
                "Strength not monotonic at variant {}",
                v
            );
            prev = strength.unwrap();
        }
    }

    #[test]
    fn silluq_is_strongest_disjunctive() {
        // Silluq (variant 0) has strength 1 — the weakest by this scheme.
        // The doc comment says "The strongest accent has a relative strength of 1".
        // So lower == stronger. Confirm that Silluq indeed gets strength 1.
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
    }

    // ── Trait derivations ──────────────────────────────────────────

    #[test]
    fn clone_produces_equal_value() {
        let accent = ProseAccent::ZaqephGadol;
        assert_eq!(accent, accent.clone());
    }

    #[test]
    fn copy_works_without_clone_explicit() {
        let original = ProseAccent::Pashta;
        let copied = original; // relies on Copy
        assert_eq!(original, copied); // original still usable
    }

    #[test]
    fn equality_and_inequality() {
        assert_eq!(ProseAccent::Revia, ProseAccent::Revia);
        assert_ne!(ProseAccent::Revia, ProseAccent::Geresh);
    }

    #[test]
    fn hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        ProseAccent::Pazer.hash(&mut h1);
        ProseAccent::Pazer.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());

        let mut h3 = DefaultHasher::new();
        ProseAccent::PazerGadol.hash(&mut h3);
        assert_ne!(h1.finish(), h3.finish());
    }

    // ── Debug trait ────────────────────────────────────────────────

    #[test]
    fn debug_output_is_non_empty() {
        let dbg = format!("{:?}", ProseAccent::Tevir);
        assert!(!dbg.is_empty());
        assert!(dbg.contains("Tevir"));
    }

    // ── Display ────────────────────────────────────────────────────
    // Display delegates to english_name(), hebrew_name(), hebrew_concept().
    // These methods aren't in this file, but if they compile we can smoke-test
    // the format string structure.

    #[test]
    fn display_contains_parentheses_pattern() {
        // The format is "{english} ({hebrew}), meaning: {concept}"
        // At minimum the string should be non-empty and contain "meaning:".
        let s = ProseAccent::Silluq.to_string();
        assert!(!s.is_empty(), "Display output was empty");
        assert!(
            s.contains("meaning:"),
            "Display output missing 'meaning:' — got: {}",
            s
        );
        assert!(
            s.contains('(') && s.contains(')'),
            "Display output missing parenthesised hebrew name — got: {}",
            s
        );
    }

    #[test]
    fn display_differs_across_variants() {
        let a = ProseAccent::Atnach.to_string();
        let b = ProseAccent::Segolta.to_string();
        assert_ne!(a, b, "Two different variants produced identical Display");
    }

    // ── Exhaustive checks over all variants ───────────────────────

    #[test]
    fn all_variants_have_distinct_relative_strength() {
        let mut strengths: Vec<Option<u8>> = (0..ProseAccent::LEN as u8)
            .map(|v| {
                let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
                accent.relative_strength()
            })
            .collect();
        strengths.sort_unstable();
        strengths.dedup();
        assert_eq!(
            strengths.len(),
            ProseAccent::LEN,
            "Duplicate relative_strength values found"
        );
    }

    #[test]
    fn all_variants_produce_non_empty_display() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let s = accent.to_string();
            assert!(!s.is_empty(), "Empty Display for variant {}", v);
        }
    }

    #[test]
    fn all_variants_produce_non_empty_debug() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let s = format!("{:?}", accent);
            assert!(!s.is_empty(), "Empty Debug for variant {}", v);
        }
    }
}
