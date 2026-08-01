//! # Hebrew Poetry Accents
//!
//! Enumerates all cantillation marks (ta'amim) used in the **three poetic books**
//! of the Hebrew Bible: Psalms (תְּהִלִּים), Proverbs (מִשְׁלֵי), and Job (אִיּוֹב).
//!
//! ## System Overview
//!
//! The poetic accent system is distinct from the prose system, employing a different
//! set of disjunctive and conjunctive marks with unique melodic traditions. While some
//! accent *names* are shared between systems (e.g., Silluq, Atnach, Munach), their
//! hierarchical roles and associated melodies often differ.
//!
//! ## Composition
//!
//! | Category | Count | Role |
//! |----------|-------|------|
//! | Disjunctive | 12 | Mark phrase boundaries and pauses |
//! | Conjunctive | 11 | Connect words within a phrase |
//! | **Total** | **23** | |
//!
//! ## Hierarchy
//!
//! Disjunctive accents form a nested hierarchy from the verse-level down:
//!
//! ```text
//! Verse (Silluq)
//!  └─ Half-verse (Atnach / Oleh WeYored)
//!     └─ Phrase levels (Revia Gadol, Revia Mugrash, ...)
//!        └─ Sub-phrase (Dechi, Pazer, ...)
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use crate::api::PoetryAccent;
//! use crate::Accent;
//!
//! let accent = PoetryAccent::OlehWeYored;
//! println!("{}", accent); // "Oleh WeYored (עולה ויורד), meaning: ascending and descending"
//! println!("Strength: {:?}", accent.relative_strength());
//! ```

use crate::Accent;
use strum_macros::EnumIter;

/// Represents a single Hebrew poetry cantillation mark.
///
/// Each variant corresponds to one accent in the **poetic books** system
/// (Psalms, Proverbs, Job). Variants are ordered by their position in the
/// disjunctive hierarchy: disjunctive accents first (indices 0–11), followed
/// by conjunctive accents (indices 12–22).
///
/// # Layout
///
/// | Index Range | Category | Count |
/// |-------------|----------|-------|
/// | 0–11 | Disjunctive | 12 |
/// | 12–22 | Conjunctive | 11 |
///
/// # Representation
///
/// - `#[repr(u8)]` — Each variant is stored as a single byte for efficient
///   table lookups and FFI compatibility.
/// - `#[non_exhaustive]` — New accent variants may be added in future versions
///   without breaking semver compatibility.
/// - `EnumIter` — Enables iteration over all variants via `PoetryAccent::iter()`.
///
/// # Example
///
/// ```ignore
/// use crate::api::PoetryAccent;
/// use crate::Accent;
///
/// // Iterate over all poetry accents
/// for accent in PoetryAccent::iter() {
///     println!("{}: {} ({})",
///         accent.english_name(),
///         accent.hebrew_name(),
///         accent.cantillation_symbol(),
///     );
/// }
/// ```
#[repr(u8)]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PoetryAccent {
    /// **Silluq** (סִלּוּק) — "cessation, ending"
    ///
    /// The strongest disjunctive accent in the poetic system. Marks the end
    /// of a complete verse, functioning as the terminal accent. Every verse
    /// in the poetic books concludes with Silluq.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Verse
    /// - **Strength**: 1 (strongest)
    #[default]
    Silluq,

    /// **Oleh WeYored** (עוֹלֶה וְיוֹרֵד) — "ascending and descending"
    ///
    /// A poetry-exclusive disjunctive accent that divides the verse into two
    /// halves. It serves the same structural role as Atnach in the prose system
    /// but is unique to the poetic books.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    /// - **Exclusivity**: Poetry only — presence confirms poetic context
    OlehWeYored,

    /// **Atnach** (אַתְנָח) — "rest, pause"
    ///
    /// A major disjunctive that divides the verse into two halves. Also appears
    /// in the prose system but may carry a different melodic contour in poetry.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    Atnach,

    /// **Revia Gadol** (רְבִיעַ גָּדוֹל) — "great quarter"
    ///
    /// A primary disjunctive marking a phrase boundary within a half-verse.
    /// The "Gadol" (great) designation distinguishes it from the smaller
    /// Revia Qaton.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ReviaGadol,

    /// **Revia Mugrash** (רְבִיעַ מֻגְרָשׁ) — "quartered with Garesh"
    ///
    /// A disjunctive accent that combines the Revia mark with a preceding
    /// Garesh-like element. Occurs in specific poetic phrase structures.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ReviaMugrash,

    /// **Shalshelet Gadol** (שַׁלְשֶׁלֶת גָּדוֹל) — "great chain"
    ///
    /// A rare disjunctive accent appearing in distinctive poetic constructions.
    /// The "Gadol" form is specific to the poetic system.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ShalsheletGadol,

    /// **Tsinnor** (צִנּוֹר) — "channel, pipe"
    ///
    /// A disjunctive accent unique to the poetic system. Appears as a
    /// prepositive mark on the accented syllable.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    Tsinnor,

    /// **Revia Qaton** (רְבִיעַ קָטָן) — "small quarter"
    ///
    /// A subordinate disjunctive marking a sub-phrase boundary. The "Qaton"
    /// (small) designation indicates a weaker pause than Revia Gadol.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    ReviaQaton,

    /// **Dechi** (דְּחִי) — "pushed away"
    ///
    /// A poetry-exclusive disjunctive accent marking a minor phrase division.
    /// Its presence is a strong indicator of poetic context.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Poetry only
    Dechi,

    /// **Pazer** (פָּזֵר) — "scatter, disperse"
    ///
    /// A disjunctive accent indicating a lighter pause within a sub-phrase.
    /// Also appears in the prose system with potentially different function.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Pazer,

    /// **Mehuppakh Legarmeh** (מְהֻפָּךְ לְגַרְמֵהּ) — "inverted, alone"
    ///
    /// A disjunctive accent that combines the Mehuppakh mark with a legarmeh
    /// separator. Functions as an independent phrase boundary in poetic text.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    MehuppakhLegarmeh,

    /// **Azla Legarmeh** (אַזְלָא לְגַרְמֵהּ) — "going forth, alone"
    ///
    /// A disjunctive accent combining Azla with a legarmeh separator, marking
    /// an independent phrase division.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    AzlaLegarmeh,

    /// **Munach** (מֻנָּח) — "resting, placed"
    ///
    /// The most common conjunctive accent in both systems. Connects a word to
    /// the following disjunctive accent without introducing a pause.
    ///
    /// - **Category**: Conjunctive (primary)
    Munach,

    /// **Merkha** (מֵרכָּא) — "lengthener, drawn out"
    ///
    /// A conjunctive accent that links a word to the following accent with a
    /// forward-leaning melodic motion.
    ///
    /// - **Category**: Conjunctive (primary)
    Merkha,

    /// **Illuy** (עִלּוּי) — "elevation, rising"
    ///
    /// A poetry-exclusive conjunctive accent. Its presence is a strong
    /// indicator of poetic context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    Illuy,

    /// **Tarcha** (תַּרְחָא) — "delay, lingering"
    ///
    /// A conjunctive accent connecting words within a poetic phrase. Known
    /// in some traditions as Tipcha in the prose system.
    ///
    /// - **Category**: Conjunctive (primary)
    Tarcha,

    /// **Galgal** (גַּלְגַּל) — "wheel, rolling"
    ///
    /// A conjunctive accent with a rolling melodic motion. Appears in both
    /// poetic and prose systems.
    ///
    /// - **Category**: Conjunctive (primary)
    Galgal,

    /// **Mehuppakh** (מְהֻפָּךְ) — "inverted, overturned"
    ///
    /// A conjunctive accent linking words within a phrase. The name refers to
    /// the inverted form of the mark compared to its disjunctive counterpart.
    ///
    /// - **Category**: Conjunctive (primary)
    Mehuppakh,

    /// **Azla** (אַזְלָא) — "going forth, departure"
    ///
    /// A conjunctive accent that connects a word to the next. Also known as
    /// Qadma in some scholarly traditions.
    ///
    /// - **Category**: Conjunctive (primary)
    Azla,

    /// **Shalshelet Qetannah** (שַׁלְשֶׁלֶת קְטַנָּה) — "small chain"
    ///
    /// A conjunctive counterpart to Shalshelet Gadol, connecting words rather
    /// than dividing phrases.
    ///
    /// - **Category**: Conjunctive (primary)
    ShalsheletQetannah,

    /// **Tsinnorit Merkha** (צִנּוֹרִית מֵרכָּא) — "channel-like Merkha"
    ///
    /// A poetry-exclusive conjunctive accent. Combines Tsinnorit with Merkha,
    /// serving as a strong poetic-context indicator.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    TsinnoritMerkha,

    /// **Tsinnorit Mahpakh** (צִנּוֹרִית מַהְפָּךְ) — "channel-like Mahpakh"
    ///
    /// A poetry-exclusive conjunctive accent. Combines Tsinnorit with Mahpakh,
    /// serving as a strong poetic-context indicator.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    TsinnoritMahpakh,

    /// **Meteg** (מֶתֶג) — "bridle, restraint"
    ///
    /// A secondary conjunctive mark that clarifies vowel length and prevents
    /// misreading of sheva. Does not carry independent melodic function in the
    /// primary cantillation hierarchy.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meteg,
}

impl PoetryAccent {
    /// The total number of poetry accent variants.
    ///
    /// Composed of 12 disjunctive + 11 conjunctive accents.
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
