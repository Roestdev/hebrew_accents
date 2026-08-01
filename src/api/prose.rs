//! # Hebrew Prose Accents
//!
//! Enumerates all cantillation marks (ta'amim) used in the **prose books** of the
//! Hebrew Bible: the Torah (Five Books of Moses), the Former Prophets, the Latter
//! Prophets, and most of the Writings.
//!
//! ## System Overview
//!
//! The prose accent system is the primary cantillation system of the Tanakh,
//! covering approximately 80% of the biblical text. It employs a rich hierarchy
//! of disjunctive and conjunctive accents that govern both musical chanting
//! and syntactic phrasing.
//!
//! ## Composition
//!
//! | Category | Count | Role |
//! |----------|-------|------|
//! | Disjunctive | 18 | Mark phrase boundaries and pauses |
//! | Conjunctive | 10 | Connect words within a phrase |
//! | **Total** | **28** | |
//!
//! ## Hierarchy
//!
//! The disjunctive hierarchy in the prose system follows this structure:
//!
//! ```text
//! Verse (Silluq)
//!  └─ Half-verse (Atnach)
//!     └─ Major phrase (Segolta, Shalshelet, Zaqeph Qatan/Gadol, Revia, ...)
//!        └─ Minor phrase (Tiphcha, Zarqa, Pashta, Yetiv, Tevir, ...)
//!           └─ Sub-phrase (Geresh, Gershayim, Pazer, PazerGadol, ...)
//!              └─ Micro-phrase (Telisha Gedolah, Legarmeh)
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! use crate::api::ProseAccent;
//! use crate::Accent;
//!
//! let accent = ProseAccent::Atnach;
//! println!("{}", accent); // "Atnach (אַתְנָח), meaning: rest"
//! println!("Strength: {:?}", accent.relative_strength());
//! ```

use crate::Accent;
use strum_macros::EnumIter;

/// Represents a single Hebrew prose cantillation mark.
///
/// Each variant corresponds to one accent in the **prose books** system (Torah,
/// Prophets, most Writings). Variants are ordered by their position in the
/// disjunctive hierarchy: disjunctive accents first (indices 0–17), followed
/// by conjunctive accents (indices 18–27).
///
/// # Layout
///
/// | Index Range | Category | Count |
/// |-------------|----------|-------|
/// | 0–17 | Disjunctive | 18 |
/// | 18–27 | Conjunctive | 10 |
///
/// # Representation
///
/// - `#[repr(u8)]` — Each variant is stored as a single byte for efficient
///   table lookups and FFI compatibility.
/// - `#[non_exhaustive]` — New accent variants may be added in future versions.
/// - `EnumIter` — Enables iteration over all variants via `ProseAccent::iter()`.
///
/// # Relative Strength
///
/// The discriminant value plus one equals the relative strength (where 1 is
/// the strongest). This means `Silluq` (index 0) has strength 1, and `Meteg`
/// (index 27) has strength 28.
///
/// # Example
///
/// ```ignore
/// use crate::api::ProseAccent;
/// use crate::Accent;
///
/// // Check the strength of Atnach
/// let atnach = ProseAccent::Atnach;
/// println!("{} strength: {:?}", atnach.english_name(), atnach.relative_strength());
/// ```
#[repr(u8)]
#[derive(EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum ProseAccent {
    /// **Silluq** (סִלּוּק) — "cessation, ending"
    ///
    /// The strongest disjunctive accent. Marks the end of a complete verse.
    /// Every verse in the prose books concludes with Silluq.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Verse
    /// - **Strength**: 1 (strongest)
    #[default]
    Silluq,

    /// **Atnach** (אַתְנָח) — "rest, pause"
    ///
    /// The second-strongest disjunctive. Divides the verse into two halves
    /// (atarah/imtarah). Acts as the primary midpoint marker.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    /// - **Strength**: 2
    Atnach,

    /// **Segolta** (סְגוֹלְתָּא) — "bunch, cluster (of grapes)"
    ///
    /// A prose-exclusive disjunctive that appears near the beginning of a verse,
    /// marking a major phrase boundary. Its presence confirms prose context.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    Segolta,

    /// **Shalshelet** (שַׁלְשֶׁלֶת) — "chain, link"
    ///
    /// A rare disjunctive accent occurring at key narrative moments. Appears
    /// only a handful of times in the Torah.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    Shalshelet,

    /// **Zaqeph Qatan** (זָקֵף קָטֹן) — "small raiser"
    ///
    /// A prose-exclusive disjunctive marking a phrase boundary. The "Qatan"
    /// (small) form occurs when the accented word is followed by additional
    /// words before the next disjunctive.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    ZaqephQatan,

    /// **Zaqeph Gadol** (זָקֵף גָּדוֹל) — "great raiser"
    ///
    /// A prose-exclusive disjunctive marking a phrase boundary. The "Gadol"
    /// (great) form occurs when the accented word immediately precedes the
    /// next disjunctive accent.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    ZaqephGadol,

    /// **Revia** (רְבִיעַ) — "quarter, fourth"
    ///
    /// A disjunctive accent marking a minor phrase boundary. Divides segments
    /// created by higher-level disjunctives.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Minor phrase
    Revia,

    /// **Tiphcha** (טִפְחָא) — "handbreadth, palm"
    ///
    /// A disjunctive accent marking a sub-phrase boundary. Often appears near
    /// the end of a verse before Silluq.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Tiphcha,

    /// **Zarqa** (זַרְקָא) — "throwing, scattering"
    ///
    /// A prose-exclusive disjunctive that marks a sub-phrase boundary. Appears
    /// as a prepositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Zarqa,

    /// **Pashta** (פַּשְׁטָא) — "extension, stretching out"
    ///
    /// A prose-exclusive disjunctive marking a sub-phrase division. Appears
    /// as a postpositive mark on the final consonant.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Pashta,

    /// **Yetiv** (יְתִיב) — "sitting, resting"
    ///
    /// A prose-exclusive disjunctive marking a sub-phrase boundary. Appears as
    /// a prepositive mark on the initial consonant.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Yetiv,

    /// **Tevir** (תְּבִיר) — "break, fracture"
    ///
    /// A prose-exclusive disjunctive that marks a sub-phrase boundary, often
    /// indicating a break in the melodic line.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Tevir,

    /// **Geresh** (גֵּרֶשׁ) — "expulsion, driving out"
    ///
    /// A disjunctive accent marking a micro-phrase boundary. Appears as a
    /// postpositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Geresh,

    /// **Gershayim** (גֵּרְשַׁיִם) — "double expulsion"
    ///
    /// A prose-exclusive disjunctive accent, the doubled form of Geresh.
    /// Marks a micro-phrase boundary with a stronger pause.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    Gershayim,

    /// **Pazer** (פָּזֵר) — "scatter, disperse"
    ///
    /// A disjunctive accent marking a micro-phrase boundary. Indicates a
    /// light pause within a sub-phrase.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Pazer,

    /// **Pazer Gadol** (פָּזֵר גָּדוֹל) — "great scatter"
    ///
    /// A prose-exclusive disjunctive, the strengthened form of Pazer. Marks a
    /// micro-phrase boundary with longer melismatic elaboration.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    PazerGadol,

    /// **Telisha Gedolah** (טְלִישָׁה גְּדוֹלָה) — "great magnification"
    ///
    /// A prose-exclusive disjunctive marking a micro-phrase boundary. Appears
    /// as a prepositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    TelishaGedolah,

    /// **Legarmeh** (לְגַרְמֵהּ) — "by itself, independently"
    ///
    /// A disjunctive accent that stands alone as an independent phrase marker.
    /// Often appears as a suffix indicating self-standing status.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Legarmeh,

    /// **Munach** (מֻנָּח) — "resting, placed"
    ///
    /// The most common conjunctive accent. Connects a word to the following
    /// disjunctive, carrying the melody forward without a pause.
    ///
    /// - **Category**: Conjunctive (primary)
    Munach,

    /// **Mahpakh** (מַהְפָּךְ) — "overturning, transformation"
    ///
    /// A conjunctive accent linking words within a phrase. The mark is the
    /// inverted form of its disjunctive counterpart.
    ///
    /// - **Category**: Conjunctive (primary)
    Mahpakh,

    /// **Merkha** (מֵרכָּא) — "lengthener, drawn out"
    ///
    /// A conjunctive accent with a forward-leaning melodic motion, connecting
    /// a word to the next accented word.
    ///
    /// - **Category**: Conjunctive (primary)
    Merkha,

    /// **Merkha Kephulah** (מֵרכָּא כְּפוּלָּה) — "doubled Merkha"
    ///
    /// A prose-exclusive conjunctive accent — the doubled form of Merkha.
    /// Its presence is a strong indicator of prose context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Prose only
    MerkhaKephulah,

    /// **Darga** (דַּרְגָּא) — "step, grade, stair"
    ///
    /// A prose-exclusive conjunctive accent marking a stepping melodic motion.
    /// Its presence is a strong indicator of prose context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Prose only
    Darga,

    /// **Azla** (אַזְלָא) — "going forth, departure"
    ///
    /// A conjunctive accent connecting a word to the following disjunctive.
    /// Also known as Qadma in some scholarly traditions.
    ///
    /// - **Category**: Conjunctive (primary)
    Azla,

    /// **Telisha Qetannah** (טְלִישָׁה קְטַנָּה) — "small magnification"
    ///
    /// A conjunctive counterpart to Telisha Gedolah, connecting words rather
    /// than dividing phrases. Appears as a postpositive mark.
    ///
    /// - **Category**: Conjunctive (primary)
    TelishaQetannah,

    /// **Galgal** (גַּלְגַּל) — "wheel, rolling"
    ///
    /// A conjunctive accent with a rolling melodic motion. Appears in both
    /// poetic and prose systems.
    ///
    /// - **Category**: Conjunctive (primary)
    Galgal,

    /// **Meayla** (מֵעֲיָא) — "from a heap, abundance"
    ///
    /// A secondary conjunctive accent with a supportive melodic function.
    /// Less frequent than primary conjunctives.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meayla,

    /// **Meteg** (מֶתֶג) — "bridle, restraint"
    ///
    /// A secondary conjunctive mark that clarifies vowel length and prevents
    /// ambiguous sheva readings. Does not carry independent melodic function
    /// in the primary cantillation hierarchy.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meteg,
}

impl ProseAccent {
    /// The total number of prose accent variants.
    ///
    /// Composed of 18 disjunctive + 10 conjunctive accents.
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
        assert_eq!(ProseAccent::iter().count(), ProseAccent::LEN);
        assert_eq!(ProseAccent::iter().count(), 28);
    }

    #[test]
    fn test_enum_iter_all_variants_present() {
        let mut found = std::collections::HashSet::new();
        for accent in ProseAccent::iter() {
            assert!(found.insert(accent), "Duplicate");
        }
        assert_eq!(found.len(), ProseAccent::LEN);
    }

    #[test]
    fn len_constant_matches_variant_count() {
        assert_eq!(ProseAccent::LEN, 28);
    }

    #[test]
    fn discriminant_values_are_sequential() {
        assert_eq!(ProseAccent::Silluq as u8, 0);
        assert_eq!(ProseAccent::Atnach as u8, 1);
        assert_eq!(ProseAccent::Legarmeh as u8, 17);
        assert_eq!(ProseAccent::Munach as u8, 18);
        assert_eq!(ProseAccent::Meteg as u8, 27);
    }

    #[test]
    fn default_is_silluq() {
        let default = ProseAccent::default();
        assert_eq!(default, ProseAccent::Silluq);
    }

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
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
    }

    #[test]
    fn clone_produces_equal_value() {
        let accent = ProseAccent::ZaqephGadol;
        assert_eq!(accent, accent.clone());
    }

    #[test]
    fn copy_works_without_clone_explicit() {
        let original = ProseAccent::Pashta;
        let copied = original;
        assert_eq!(original, copied);
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

    #[test]
    fn debug_output_is_non_empty() {
        let dbg = format!("{:?}", ProseAccent::Tevir);
        assert!(!dbg.is_empty());
        assert!(dbg.contains("Tevir"));
    }

    #[test]
    fn display_contains_parentheses_pattern() {
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
        assert_ne!(a, b);
    }

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
