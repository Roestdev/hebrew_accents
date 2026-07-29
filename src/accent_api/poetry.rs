use crate::Accent;
//use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// All variants of the Hebrew Poetry Accents
/// 12 Disjunctives and 11 Conjunctives.
#[repr(u8)]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PoetryAccent {
    #[default]
    /// Primary disjunctive poetry accent Silluq
    Silluq,
    /// Primary disjunctive poetry accent Oleh Weyored
    OlehWeYored,
    /// Primary disjunctive poetry accent Atnach
    Atnach,
    /// Primary disjunctive poetry accent Revia Gadol
    ReviaGadol,
    /// Primary disjunctive poetry accent Revia Mugrash,
    ReviaMugrash,
    /// Primary disjunctive poetry accent ShalsheletGadol
    ShalsheletGadol,
    /// Primary disjunctive poetry accent Tsinnor
    Tsinnor,
    /// Primary disjunctive poetry accent Revia Qaton
    ReviaQaton,
    /// Primary disjunctive poetry accent Dechi,
    Dechi,
    /// Primary disjunctive poetry accent Pazer
    Pazer,
    /// Primary disjunctive poetry accent MehuppakhLegarmeh
    MehuppakhLegarmeh,
    /// Primary disjunctive poetry accent AzlaLegarmeh
    AzlaLegarmeh,
    /// Primary conjunctive poetry accent Munach
    Munach,
    /// Primary conjunctive poetry accent Merkha
    Merkha,
    /// Primary conjunctive poetry accent Illuy,
    Illuy,
    /// Primary conjunctive poetry accent Tarcha
    Tarcha,
    /// Primary conjunctive poetry accent Galgal
    Galgal,
    /// Primary conjunctive poetry accent Mehuppakh
    Mehuppakh,
    /// Primary conjunctive poetry accent Azla
    Azla,
    /// Primary conjunctive poetry accent Shalshelet Qetannah
    ShalsheletQetannah,
    /// Primary conjunctive poetry accent Tsinnorit Merkha
    TsinnoritMerkha,
    /// Primary conjunctive poetry accent Tsinnorit Mahpakh
    TsinnoritMahpakh,
    /// Secondary conjunctive poetry accent Meteg
    Meteg,
}

impl PoetryAccent {
    /// Total count of all poetry accents,including some 'non-accents'
    pub const LEN: usize = 23;
}

impl std::fmt::Display for PoetryAccent {
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
    use crate::accent_data::BHS_POETRY_RANK_MAP;

    // ── Variant count ──────────────────────────────────────────────

    #[test]
    fn len_constant_matches_variant_count() {
        // 12 disjunctives + 11 conjunctives = 23
        assert_eq!(PoetryAccent::LEN, 23);
    }

    #[test]
    fn discriminant_values_are_sequential() {
        assert_eq!(PoetryAccent::Silluq as u8, 0);
        assert_eq!(PoetryAccent::OlehWeYored as u8, 1);
        assert_eq!(PoetryAccent::AzlaLegarmeh as u8, 11);
        assert_eq!(PoetryAccent::Munach as u8, 12);
        assert_eq!(PoetryAccent::Meteg as u8, 22);
    }

    // ── Default ────────────────────────────────────────────────────

    #[test]
    fn default_is_silluq() {
        assert_eq!(PoetryAccent::default(), PoetryAccent::Silluq);
    }

    // ── relative_strength ──────────────────────────────────────────

    #[test]
    fn relative_strength_delegates_to_rank_map() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            assert_eq!(
                accent.relative_strength().unwrap(),
                BHS_POETRY_RANK_MAP[v as usize],
                "Mismatch at variant {}",
                v
            );
        }
    }

    #[test]
    fn relative_strength_is_nonzero_for_all_variants() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let strength = accent.relative_strength();
            assert!(
                strength.unwrap() > 0,
                "relative_strength is 0 for variant {} ({:?})",
                v,
                accent
            );
        }
    }

    #[test]
    fn relative_strength_fits_in_u8() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let _ = accent.relative_strength(); // should not panic
        }
    }

    #[test]
    fn rank_map_length_matches_len() {
        assert_eq!(BHS_POETRY_RANK_MAP.len(), PoetryAccent::LEN);
    }

    // ── Derived traits ──────────────────────────────────────────────

    #[test]
    fn copy_preserves_value() {
        let original = PoetryAccent::Tsinnor;
        let copied = original; // relies on Copy
        assert_eq!(original, copied);
    }

    #[test]
    fn clone_preserves_value() {
        let original = PoetryAccent::Dechi;
        assert_eq!(original, original.clone());
    }

    #[test]
    fn equality_and_inequality() {
        assert_eq!(PoetryAccent::Munach, PoetryAccent::Munach);
        assert_ne!(PoetryAccent::Munach, PoetryAccent::Merkha);
        assert_ne!(PoetryAccent::ReviaGadol, PoetryAccent::ReviaQaton);
        assert_ne!(
            PoetryAccent::ShalsheletGadol,
            PoetryAccent::ShalsheletQetannah
        );
    }

    #[test]
    fn hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        PoetryAccent::Pazer.hash(&mut h1);
        PoetryAccent::Pazer.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());

        let mut h3 = DefaultHasher::new();
        PoetryAccent::Azla.hash(&mut h3);
        assert_ne!(h1.finish(), h3.finish());
    }

    // ── Debug trait ────────────────────────────────────────────────

    #[test]
    fn debug_output_contains_variant_name() {
        assert!(format!("{:?}", PoetryAccent::Silluq).contains("Silluq"));
        assert!(format!("{:?}", PoetryAccent::OlehWeYored).contains("OlehWeYored"));
        assert!(format!("{:?}", PoetryAccent::TsinnoritMerkha).contains("TsinnoritMerkha"));
        assert!(format!("{:?}", PoetryAccent::TsinnoritMahpakh).contains("TsinnoritMahpakh"));
    }

    #[test]
    fn debug_outputs_are_distinct() {
        let a = format!("{:?}", PoetryAccent::Illuy);
        let b = format!("{:?}", PoetryAccent::Tarcha);
        assert_ne!(a, b);
    }

    // ── Display ────────────────────────────────────────────────────
    // Display delegates to english_name(), hebrew_name(), hebrew_concept().
    // These methods aren't in this file, but if they compile we can smoke-test
    // the format-string structure.

    #[test]
    fn display_contains_meaning_keyword() {
        let s = PoetryAccent::Silluq.to_string();
        assert!(!s.is_empty(), "Display output was empty");
        assert!(
            s.contains("meaning:"),
            "Display output missing 'meaning:' — got: {}",
            s
        );
    }

    #[test]
    fn display_contains_parenthesised_hebrew_name() {
        let s = PoetryAccent::Atnach.to_string();
        assert!(
            s.contains('(') && s.contains(')'),
            "Display output missing parenthesised hebrew name — got: {}",
            s
        );
    }

    #[test]
    fn display_differs_across_variants() {
        let a = PoetryAccent::Silluq.to_string();
        let b = PoetryAccent::Atnach.to_string();
        let c = PoetryAccent::Meteg.to_string();
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }

    #[test]
    fn display_non_empty_for_all_variants() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let s = accent.to_string();
            assert!(
                !s.is_empty(),
                "Empty Display for variant {} ({:?})",
                v,
                accent
            );
        }
    }

    // ── Exhaustive checks ──────────────────────────────────────────

    #[test]
    fn all_variants_produce_non_empty_debug() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let s = format!("{:?}", accent);
            assert!(!s.is_empty(), "Empty Debug for variant {}", v);
        }
    }

    #[test]
    fn all_debug_outputs_are_distinct() {
        let mut seen = std::collections::HashSet::new();
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let s = format!("{:?}", accent);
            assert!(seen.insert(s), "Duplicate Debug output at variant {}", v);
        }
        assert_eq!(seen.len(), PoetryAccent::LEN);
    }

    #[test]
    fn all_display_outputs_are_distinct() {
        let mut seen = std::collections::HashSet::new();
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let s = accent.to_string();
            assert!(
                seen.insert(s.clone()),
                "Duplicate Display output at variant {}: {}",
                v,
                s
            );
        }
        assert_eq!(seen.len(), PoetryAccent::LEN);
    }
}
