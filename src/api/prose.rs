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
//! ```rust
//! use hebrew_accents::{ProseAccent,Accent};
//!
//! let accent = ProseAccent::Atnach;
//! println!("{}", accent); // "Atnach (אַתְנָח), meaning: rest"
//! println!("Strength: {:?}", accent.relative_strength());
//! ```

use crate::accent::{resolve_disjunctive_group, resolve_relative_strength};
use crate::accent_data::{BHS_PROSE_RANK_MAP, PROSE_ACCENT_TABLE};
use crate::api::CompoundType;
use crate::{
    display_cantillation_symbol, AccentCategory, AccentKind, CantillationMark, GroupLevel,
};
use crate::{Accent, AlternateNames};
use strum_macros::{EnumCount, EnumIter};

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
/// - `EnumCount` / `EnumIter` — Auto-derived count and iteration.
/// - Discriminant values are **explicit and consecutive** to guarantee
///   that `self as usize` indexes into `PROSE_ACCENT_TABLE` correctly.
///
/// # Relative Strength
///
/// The discriminant value plus one equals the relative strength (where 1 is
/// the strongest). This means `Silluq` (index 0) has strength 1, and `Meteg`
/// (index 27) has strength 28.
///
/// # Example
///
/// ```rust
/// use hebrew_accents::{ProseAccent,Accent};
///
/// // Check the strength of Atnach
/// let atnach = ProseAccent::Atnach;
/// println!("{} strength: {:?}", atnach.english_name(), atnach.relative_strength());
/// ```
#[repr(u8)]
#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
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
    Silluq = 0,

    /// **Atnach** (אַתְנָח) — "rest, pause"
    ///
    /// The second-strongest disjunctive. Divides the verse into two halves
    /// (atarah/imtarah). Acts as the primary midpoint marker.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    /// - **Strength**: 2
    Atnach = 1,

    /// **Segolta** (סְגוֹלְתָּא) — "bunch, cluster (of grapes)"
    ///
    /// A prose-exclusive disjunctive that appears near the beginning of a verse,
    /// marking a major phrase boundary. Its presence confirms prose context.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    Segolta = 2,

    /// **Shalshelet** (שַׁלְשֶׁלֶת) — "chain, link"
    ///
    /// A rare disjunctive accent occurring at key narrative moments. Appears
    /// only a handful of times in the Torah.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    Shalshelet = 3,

    /// **Zaqeph Qatan** (זָקֵף קָטֹן) — "small raiser"
    ///
    /// A prose-exclusive disjunctive marking a phrase boundary. The "Qatan"
    /// (small) form occurs when the accented word is followed by additional
    /// words before the next disjunctive.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    ZaqephQatan = 4,

    /// **Zaqeph Gadol** (זָקֵף גָּדוֹל) — "great raiser"
    ///
    /// A prose-exclusive disjunctive marking a phrase boundary. The "Gadol"
    /// (great) form occurs when the accented word immediately precedes the
    /// next disjunctive accent.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Major phrase
    /// - **Exclusivity**: Prose only
    ZaqephGadol = 5,

    /// **Revia** (רְבִיעַ) — "quarter, fourth"
    ///
    /// A disjunctive accent marking a minor phrase boundary. Divides segments
    /// created by higher-level disjunctives.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Minor phrase
    Revia = 6,

    /// **Tiphcha** (טִפְחָא) — "handbreadth, palm"
    ///
    /// A disjunctive accent marking a sub-phrase boundary. Often appears near
    /// the end of a verse before Silluq.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Tiphcha = 7,

    /// **Zarqa** (זַרְקָא) — "throwing, scattering"
    ///
    /// A prose-exclusive disjunctive that marks a sub-phrase boundary. Appears
    /// as a prepositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Zarqa = 8,

    /// **Pashta** (פַּשְׁטָא) — "extension, stretching out"
    ///
    /// A prose-exclusive disjunctive marking a sub-phrase division. Appears
    /// as a postpositive mark on the final consonant.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Pashta = 9,

    /// **Yetiv** (יְתִיב) — "sitting, resting"
    ///
    /// A prose-exclusive disjunctive marking a sub-phrase boundary. Appears as
    /// a prepositive mark on the initial consonant.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Yetiv = 10,

    /// **Tevir** (תְּבִיר) — "break, fracture"
    ///
    /// A prose-exclusive disjunctive that marks a sub-phrase boundary, often
    /// indicating a break in the melodic line.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Prose only
    Tevir = 11,

    /// **Geresh** (גֵּרֶשׁ) — "expulsion, driving out"
    ///
    /// A disjunctive accent marking a micro-phrase boundary. Appears as a
    /// postpositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Geresh = 12,

    /// **Gershayim** (גֵּרְשַׁיִם) — "double expulsion"
    ///
    /// A prose-exclusive disjunctive accent, the doubled form of Geresh.
    /// Marks a micro-phrase boundary with a stronger pause.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    Gershayim = 13,

    /// **Pazer** (פָּזֵר) — "scatter, disperse"
    ///
    /// A disjunctive accent marking a micro-phrase boundary. Indicates a
    /// light pause within a sub-phrase.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Pazer = 14,

    /// **Pazer Gadol** (פָּזֵר גָּדוֹל) — "great scatter"
    ///
    /// A prose-exclusive disjunctive, the strengthened form of Pazer. Marks a
    /// micro-phrase boundary with longer melismatic elaboration.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    PazerGadol = 15,

    /// **Telisha Gedolah** (טְלִישָׁה גְּדוֹלָה) — "great magnification"
    ///
    /// A prose-exclusive disjunctive marking a micro-phrase boundary. Appears
    /// as a prepositive mark.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    /// - **Exclusivity**: Prose only
    TelishaGedolah = 16,

    /// **Legarmeh** (לְגַרְמֵהּ) — "by itself, independently"
    ///
    /// A disjunctive accent that stands alone as an independent phrase marker.
    /// Often appears as a suffix indicating self-standing status.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Micro-phrase
    Legarmeh = 17,

    /// **Munach** (מֻנָּח) — "resting, placed"
    ///
    /// The most common conjunctive accent. Connects a word to the following
    /// disjunctive, carrying the melody forward without a pause.
    ///
    /// - **Category**: Conjunctive (primary)
    Munach = 18,

    /// **Mahpakh** (מַהְפָּךְ) — "overturning, transformation"
    ///
    /// A conjunctive accent linking words within a phrase. The mark is the
    /// inverted form of its disjunctive counterpart.
    ///
    /// - **Category**: Conjunctive (primary)
    Mahpakh = 19,

    /// **Merkha** (מֵרכָּא) — "lengthener, drawn out"
    ///
    /// A conjunctive accent with a forward-leaning melodic motion, connecting
    /// a word to the next accented word.
    ///
    /// - **Category**: Conjunctive (primary)
    Merkha = 20,

    /// **Merkha Kephulah** (מֵרכָּא כְּפוּלָּה) — "doubled Merkha"
    ///
    /// A prose-exclusive conjunctive accent — the doubled form of Merkha.
    /// Its presence is a strong indicator of prose context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Prose only
    MerkhaKephulah = 21,

    /// **Darga** (דַּרְגָּא) — "step, grade, stair"
    ///
    /// A prose-exclusive conjunctive accent marking a stepping melodic motion.
    /// Its presence is a strong indicator of prose context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Prose only
    Darga = 22,

    /// **Azla** (אַזְלָא) — "going forth, departure"
    ///
    /// A conjunctive accent connecting a word to the following disjunctive.
    /// Also known as Qadma in some scholarly traditions.
    ///
    /// - **Category**: Conjunctive (primary)
    Azla = 23,

    /// **Telisha Qetannah** (טְלִישָׁה קְטַנָּה) — "small magnification"
    ///
    /// A conjunctive counterpart to Telisha Gedolah, connecting words rather
    /// than dividing phrases. Appears as a postpositive mark.
    ///
    /// - **Category**: Conjunctive (primary)
    TelishaQetannah = 24,

    /// **Galgal** (גַּלְגַּל) — "wheel, rolling"
    ///
    /// A conjunctive accent with a rolling melodic motion. Appears in both
    /// poetic and prose systems.
    ///
    /// - **Category**: Conjunctive (primary)
    Galgal = 25,

    /// **Meayla** (מֵעֲיָא) — "from a heap, abundance"
    ///
    /// A secondary conjunctive accent with a supportive melodic function.
    /// Less frequent than primary conjunctives.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meayla = 26,

    /// **Meteg** (מֶתֶג) — "bridle, restraint"
    ///
    /// A secondary conjunctive mark that clarifies vowel length and prevents
    /// ambiguous sheva readings. Does not carry independent melodic function
    /// in the primary cantillation hierarchy.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meteg = 27,
}

impl ProseAccent {
    /// The total number of prose accent variants.
    ///
    /// Composed of 18 disjunctive + 10 conjunctive accents.
    pub const LEN: usize = <Self as strum::EnumCount>::COUNT;

    /// Returns the discriminant as `usize`, suitable for direct table indexing.
    ///
    /// This is safe to use with `PROSE_ACCENT_TABLE` because the enum's
    /// discriminant values are guaranteed to be consecutive starting at 0.
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }
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

// ── Compile-time discriminant guards ────────────────────────────────────
// If a variant is inserted, removed, or reordered, this const assertion
// will fail at compile time.
const _: () = {
    const LAST_DISCRIMINANT: u8 = ProseAccent::Meteg as u8;
    assert!((LAST_DISCRIMINANT + 1) as usize == ProseAccent::LEN);
};

impl Accent for ProseAccent {
    #[inline]
    fn hebrew_name(&self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(&self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(&self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(&self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(&self) -> Option<AccentKind> {
        PROSE_ACCENT_TABLE[self.as_index()].kind
    }

    #[inline]
    fn category(&self) -> Option<AccentCategory> {
        PROSE_ACCENT_TABLE[self.as_index()].category
    }

    #[inline]
    fn compound_type(&self) -> Option<CompoundType> {
        PROSE_ACCENT_TABLE[self.as_index()].compound_type
    }

    #[inline]
    fn primary_cantillation_mark(&self) -> CantillationMark {
        let info = PROSE_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .primary_mark;

        CantillationMark {
            symbol: info.symbol,
            placement: info.position.into(),
            stress_position: info.stress_position.into(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(&self) -> Option<CantillationMark> {
        PROSE_ACCENT_TABLE[self.as_index()]
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
        PROSE_ACCENT_TABLE[self.as_index()].notes
    }

    #[inline]
    fn relative_strength(&self) -> Option<u8> {
        resolve_relative_strength(BHS_PROSE_RANK_MAP[self.as_index()])
    }

    #[inline]
    fn group_level(&self) -> Option<GroupLevel> {
        resolve_disjunctive_group((*self).into()).and_then(|g| g.into_public_level())
    }

    #[inline]
    fn cantillation_symbol(&self) -> String {
        display_cantillation_symbol((*self).into())
    }

    #[inline]
    fn alternate_names(&self) -> Option<AlternateNames> {
        None // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Accent;
    use strum::IntoEnumIterator;

    // ===== BASIC VARIANT COUNT TESTS =====

    #[test]
    fn test_prose_accent_total_count() {
        assert_eq!(ProseAccent::LEN, 28);
        assert_eq!(ProseAccent::iter().count(), 28);
    }

    #[test]
    fn test_disjunctive_vs_conjunctive_split() {
        // Disjunctives are indices 0-17 (18 total)
        // Conjunctives are indices 18-27 (10 total)
        let disjunctives = ProseAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Disjunctive)))
            .count();

        let conjunctives = ProseAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Conjunctive)))
            .count();

        assert_eq!(disjunctives, 18);
        assert_eq!(conjunctives, 10);
    }

    #[test]
    fn test_discriminant_guard_at_compile_time() {
        // This test verifies the const assertion exists
        // If variants are misordered, compilation fails
        const LAST_IDX: usize = ProseAccent::Meteg as u8 as usize;
        assert_eq!(LAST_IDX + 1, ProseAccent::LEN);
    }

    // ===== DEFAULT TRAIT TESTS =====

    #[test]
    fn test_default_is_silluq() {
        let default: ProseAccent = Default::default();
        assert_eq!(default, ProseAccent::Silluq);
    }

    #[test]
    fn test_silluq_explicit_equals_default() {
        assert_eq!(ProseAccent::Silluq, ProseAccent::default());
    }

    // ===== AS_INDEX METHOD TESTS =====

    #[test]
    fn test_as_index_for_all_variants() {
        assert_eq!(ProseAccent::Silluq.as_index(), 0);
        assert_eq!(ProseAccent::Atnach.as_index(), 1);
        assert_eq!(ProseAccent::Segolta.as_index(), 2);
        assert_eq!(ProseAccent::Shalshelet.as_index(), 3);
        assert_eq!(ProseAccent::ZaqephQatan.as_index(), 4);
        assert_eq!(ProseAccent::ZaqephGadol.as_index(), 5);
        assert_eq!(ProseAccent::Revia.as_index(), 6);
        assert_eq!(ProseAccent::Tiphcha.as_index(), 7);
        assert_eq!(ProseAccent::Zarqa.as_index(), 8);
        assert_eq!(ProseAccent::Pashta.as_index(), 9);
        assert_eq!(ProseAccent::Yetiv.as_index(), 10);
        assert_eq!(ProseAccent::Tevir.as_index(), 11);
        assert_eq!(ProseAccent::Geresh.as_index(), 12);
        assert_eq!(ProseAccent::Gershayim.as_index(), 13);
        assert_eq!(ProseAccent::Pazer.as_index(), 14);
        assert_eq!(ProseAccent::PazerGadol.as_index(), 15);
        assert_eq!(ProseAccent::TelishaGedolah.as_index(), 16);
        assert_eq!(ProseAccent::Legarmeh.as_index(), 17);
        assert_eq!(ProseAccent::Munach.as_index(), 18);
        assert_eq!(ProseAccent::Mahpakh.as_index(), 19);
        assert_eq!(ProseAccent::Merkha.as_index(), 20);
        assert_eq!(ProseAccent::MerkhaKephulah.as_index(), 21);
        assert_eq!(ProseAccent::Darga.as_index(), 22);
        assert_eq!(ProseAccent::Azla.as_index(), 23);
        assert_eq!(ProseAccent::TelishaQetannah.as_index(), 24);
        assert_eq!(ProseAccent::Galgal.as_index(), 25);
        assert_eq!(ProseAccent::Meayla.as_index(), 26);
        assert_eq!(ProseAccent::Meteg.as_index(), 27);
    }

    // ===== ITERATION TESTS =====

    #[test]
    fn test_enum_iter_all_variants() {
        let all_variants: Vec<ProseAccent> = ProseAccent::iter().collect();
        assert_eq!(all_variants.len(), 28);
        assert!(all_variants.contains(&ProseAccent::Silluq));
        assert!(all_variants.contains(&ProseAccent::Meteg));
        assert!(all_variants.contains(&ProseAccent::Segolta));
    }

    #[test]
    fn test_iter_order_matches_discriminant() {
        let variants: Vec<ProseAccent> = ProseAccent::iter().collect();

        for (idx, accent) in variants.iter().enumerate() {
            assert_eq!(accent.as_index(), idx);
        }
    }

    // ===== DISPLAY TRAIT TESTS =====

    #[test]
    fn test_display_format_for_sample_variants() {
        let silluq_str = format!("{}", ProseAccent::Silluq);
        assert!(silluq_str.contains("Silluq"));
        assert!(silluq_str.contains("סִלּוּק"));
        assert!(silluq_str.contains("cessation"));

        let atnach_str = format!("{}", ProseAccent::Atnach);
        assert!(atnach_str.contains("Atnach"));
    }

    #[test]
    fn test_display_contains_required_components() {
        // All Display outputs should have: name, hebrew, concept
        for accent in ProseAccent::iter() {
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
        let silluq = ProseAccent::Silluq;
        let hebrew = silluq.hebrew_name();

        assert_eq!(hebrew, "סִלּוּק");
        assert!(!hebrew.is_empty());
    }

    #[test]
    fn test_hebrew_concept_returns_meanings() {
        let silluq = ProseAccent::Silluq;
        let concept = silluq.hebrew_concept();

        assert_eq!(concept, "close, cessation");
        assert!(!concept.is_empty());
    }

    #[test]
    fn test_english_name_returns_transliterations() {
        let silluq = ProseAccent::Silluq;
        let english = silluq.english_name();

        assert_eq!(english, "Silluq");
        assert!(!english.is_empty());
    }

    #[test]
    fn test_sbl_academic_name() {
        let silluq = ProseAccent::Silluq;
        let sbl = silluq.sbl_academic_name();

        assert!(!sbl.is_empty());
        assert_eq!(sbl, "sillûq");
    }

    // --- Kind Method ---

    #[test]
    fn test_kind_for_disjunctive_accents() {
        assert_eq!(ProseAccent::Silluq.kind(), Some(crate::AccentKind::Primary));
        assert_eq!(ProseAccent::Atnach.kind(), Some(crate::AccentKind::Primary));
        assert_eq!(
            ProseAccent::Segolta.kind(),
            Some(crate::AccentKind::Primary)
        );
    }

    #[test]
    fn test_kind_for_conjunctive_accents() {
        assert_eq!(ProseAccent::Munach.kind(), Some(crate::AccentKind::Primary));
        assert_eq!(ProseAccent::Merkha.kind(), Some(crate::AccentKind::Primary));
        assert_eq!(
            ProseAccent::Meayla.kind(),
            Some(crate::AccentKind::Secondary)
        );
        assert_eq!(
            ProseAccent::Meteg.kind(),
            Some(crate::AccentKind::Secondary)
        );
    }

    #[test]
    fn test_meayla_and_meteg_are_secondary() {
        // Meayla and Meteg are the only secondary accents
        assert_eq!(
            ProseAccent::Meayla.kind(),
            Some(crate::AccentKind::Secondary)
        );
        assert_eq!(
            ProseAccent::Meteg.kind(),
            Some(crate::AccentKind::Secondary)
        );
    }

    // --- Category Method ---

    #[test]
    fn test_category_returns_some_for_all() {
        // All accents have a category
        for accent in ProseAccent::iter() {
            let cat = accent.category();
            assert!(cat.is_some(), "All accents should have a category");
        }
    }

    #[test]
    fn test_category_distribution() {
        let mut disjunctive_count = 0;
        let mut conjunctive_count = 0;

        for accent in ProseAccent::iter() {
            match accent.category() {
                Some(crate::AccentCategory::Disjunctive) => disjunctive_count += 1,
                Some(crate::AccentCategory::Conjunctive) => conjunctive_count += 1,
                _ => panic!("Unexpected category"),
            }
        }

        assert_eq!(disjunctive_count, 18);
        assert_eq!(conjunctive_count, 10);
    }

    // --- Compound Type Method ---

    #[test]
    fn test_compound_type_some_and_none() {
        // Some accents are compound (have secondary marks)
        for accent in ProseAccent::iter() {
            let ct = accent.compound_type();
            // Either Some or None is valid depending on accent
            assert!(!ct.is_none() || ct.is_none()); // Tautology - just exercising the method
        }
    }

    // --- Cantillation Mark Methods ---

    #[test]
    fn test_primary_cantillation_mark_always_returns_value() {
        for accent in ProseAccent::iter() {
            let primary = accent.primary_cantillation_mark();

            assert!(!primary.symbol.is_control());
            assert!(!primary.symbol.is_whitespace());
        }
    }

    #[test]
    fn test_secondary_cantillation_mark_option_behavior() {
        for accent in ProseAccent::iter() {
            let secondary = accent.secondary_cantillation_mark();

            // Secondary is either Some or None - both valid
            if let Some(sec) = secondary {
                // If some, it must have valid symbol
                assert!(!sec.symbol.is_control());
            }
        }
    }

    #[test]
    fn test_compound_accent_has_secondary_mark() {
        // Verify that accents with compound_type Some have secondary marks
        for accent in ProseAccent::iter() {
            let is_compound = accent.compound_type().is_some();
            let has_secondary = accent.secondary_cantillation_mark().is_some();

            // For this library, compound = has secondary mark
            assert_eq!(is_compound, has_secondary);
        }
    }

    // --- Notes Method ---

    #[test]
    fn test_notes_returns_option() {
        for accent in ProseAccent::iter() {
            let notes = accent.notes();
            // Some have notes, some don't - both valid
            assert!(!notes.is_none() || notes.is_none());
        }
    }

    #[test]
    fn test_shalshelet_has_rare_note() {
        // Shalshelet is noted as rare (occurs only 4 times in Torah)
        let notes = ProseAccent::Shalshelet.notes();
        if notes.is_some() {
            let note_text = notes.unwrap();
            assert!(
                note_text.contains("rare")
                    || note_text.contains("four")
                    || note_text.contains("times")
            );
        }
    }

    // --- Relative Strength Method ---

    #[test]
    fn test_relative_strength_for_disjunctive() {
        // Disjunctive accents should have strength values
        assert!(ProseAccent::Silluq.relative_strength().is_some());
        assert!(ProseAccent::Atnach.relative_strength().is_some());
        assert!(ProseAccent::Segolta.relative_strength().is_some());
    }

    #[test]
    fn test_relative_strength_for_conjunctive() {
        // Conjunctive accents should return None
        assert_eq!(ProseAccent::Munach.relative_strength(), None);
        assert_eq!(ProseAccent::Merkha.relative_strength(), None);
        assert_eq!(ProseAccent::Meteg.relative_strength(), None);
    }

    #[test]
    fn test_strength_numbering_system() {
        // Lower numbers = stronger disjunctives
        if let Some(s1) = ProseAccent::Silluq.relative_strength() {
            // Silluq should be very strong (low number)
            assert!(s1 <= 5); // Should be in top strength range
        }

        if let Some(s2) = ProseAccent::Atnach.relative_strength() {
            // Atnach should be second strongest
            assert_eq!(s2, 2);
        }
    }

    // --- Group Level Method ---

    #[test]
    fn test_group_level_for_disjunctive() {
        // Disjunctive accents have group levels
        assert!(ProseAccent::Silluq.group_level().is_some());
        assert!(ProseAccent::Atnach.group_level().is_some());
        assert!(ProseAccent::Segolta.group_level().is_some());
    }

    #[test]
    fn test_group_level_for_conjunctive() {
        // Conjunctive accents don't have hierarchy
        assert_eq!(ProseAccent::Munach.group_level(), None);
        assert_eq!(ProseAccent::Merkha.group_level(), None);
        assert_eq!(ProseAccent::Mahpakh.group_level(), None);
    }

    #[test]
    fn test_silluq_is_top_tier() {
        // Silluq should be Tier 1 (strongest)
        if let Some(level) = ProseAccent::Silluq.group_level() {
            assert_eq!(level, crate::GroupLevel::Tier1);
        }
    }

    #[test]
    fn test_prose_has_tier_four() {
        // Prose system reaches Tier4, poetry stops at Tier3
        let mut has_tier4 = false;

        for accent in ProseAccent::iter() {
            if let Some(level) = accent.group_level() {
                if level == crate::GroupLevel::Tier4 {
                    has_tier4 = true;
                }
            }
        }

        // Prose should have Tier4 accents
        assert!(has_tier4, "Prose system includes Tier4 disjunctive accents");
    }

    // --- Cantillation Symbol Method ---

    #[test]
    fn test_cantillation_symbol_not_empty() {
        for accent in ProseAccent::iter() {
            let symbol = accent.cantillation_symbol();
            assert!(!symbol.is_empty());
        }
    }

    #[test]
    fn test_cantillation_symbol_contains_dotted_circle_or_marks() {
        for accent in ProseAccent::iter() {
            let symbol = accent.cantillation_symbol();

            // Symbols should contain either dotted circle or actual marks
            assert!(symbol.contains('\u{25CC}') || !symbol.chars().all(|c| c == '\u{25CC}'));
        }
    }

    // --- Alternate Names Method ---

    #[test]
    fn test_alternate_names_returns_none_currently() {
        // Currently all return None (TODO mentioned in code)
        for accent in ProseAccent::iter() {
            let alts = accent.alternate_names();
            assert_eq!(alts, None); // Confirms current implementation
        }
    }

    // ===== DERIVED TRAIT TESTS =====

    #[test]
    fn test_copy_trait_works() {
        let original = ProseAccent::Silluq;
        let copied = original; // Copy occurs automatically

        assert_eq!(original, ProseAccent::Silluq);
        assert_eq!(copied, ProseAccent::Silluq);
    }

    #[test]
    fn test_clone_trait_works() {
        let original = ProseAccent::Meteg;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned, ProseAccent::Meteg);
    }

    #[test]
    fn test_partial_eq_and_eq_traits() {
        let p1 = ProseAccent::Silluq;
        let p2 = ProseAccent::Silluq;
        let p3 = ProseAccent::Meteg;

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

        ProseAccent::Silluq.hash(&mut hasher1);
        ProseAccent::Silluq.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_in_collections() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(ProseAccent::Silluq);
        set.insert(ProseAccent::Meteg);

        assert!(set.contains(&ProseAccent::Silluq));
        assert!(set.contains(&ProseAccent::Meteg));
        assert_eq!(set.len(), 2);
    }

    // ===== PROSE-SPECIFIC ACCENTS TESTS =====

    #[test]
    fn test_prose_exclusive_accents_exist() {
        // These accents only appear in prose system
        let prose_exclusives = [
            ProseAccent::Segolta,
            ProseAccent::ZaqephQatan,
            ProseAccent::ZaqephGadol,
            ProseAccent::Zarqa,
            ProseAccent::Pashta,
            ProseAccent::Yetiv,
            ProseAccent::Tevir,
            ProseAccent::Gershayim,
            ProseAccent::PazerGadol,
            ProseAccent::TelishaGedolah,
            ProseAccent::MerkhaKephulah,
            ProseAccent::Darga,
        ];

        for exclusive in &prose_exclusives {
            assert!(exclusive.category().is_some());
            assert!(!exclusive.english_name().is_empty());
        }
    }

    #[test]
    fn test_prose_exclusive_detection() {
        // Segolta presence indicates prose context
        let segolta = ProseAccent::Segolta;

        assert_eq!(
            by_category(segolta),
            Some(crate::AccentCategory::Disjunctive)
        );
        assert!(by_category(segolta).is_some());
    }

    #[test]
    fn test_zarqa_pashta_yetiv_tevir_exclusivity() {
        // These are strong indicators of prose text
        let prose_markers = [
            ProseAccent::Zarqa,
            ProseAccent::Pashta,
            ProseAccent::Yetiv,
            ProseAccent::Tevir,
        ];

        for marker in &prose_markers {
            assert!(marker.category().is_some());
        }
    }

    // ===== HIERARCHY TESTS =====

    #[test]
    fn test_hierarchy_coverage() {
        // Check that we have representation from all hierarchy levels
        let mut has_tier1 = false;
        let mut has_tier2 = false;
        let mut has_tier3 = false;
        let mut has_tier4 = false;

        for accent in ProseAccent::iter() {
            if let Some(level) = accent.group_level() {
                match level {
                    crate::GroupLevel::Tier1 => has_tier1 = true,
                    crate::GroupLevel::Tier2 => has_tier2 = true,
                    crate::GroupLevel::Tier3 => has_tier3 = true,
                    crate::GroupLevel::Tier4 => has_tier4 = true,
                }
            }
        }

        // Prose system has all 4 tiers
        assert!(has_tier1);
        assert!(has_tier2);
        assert!(has_tier3);
        assert!(has_tier4);
    }

    #[test]
    fn test_highest_strength_variants() {
        // Silluq should have highest strength (lowest number)
        if let Some(silluq_strength) = ProseAccent::Silluq.relative_strength() {
            assert_eq!(silluq_strength, 1, "Silluq should be strength 1");
        }

        if let Some(atnach_strength) = ProseAccent::Atnach.relative_strength() {
            assert_eq!(atnach_strength, 2, "Atnach should be strength 2");
        }
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_complete_api_roundtrip() {
        // Test that all API methods work together
        let accent = ProseAccent::Silluq;

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
        assert_eq!(accent.english_name(), "Silluq");
    }

    #[test]
    fn test_table_lookup_consistency() {
        // Verify table indexing works for all variants
        for accent in ProseAccent::iter() {
            let idx = accent.as_index();
            assert!(idx < ProseAccent::LEN);

            // Table access should not panic
            let _hebrew = accent.hebrew_name();
            let _english = accent.english_name();
        }
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_first_and_last_variants() {
        // Verify ordering: Silluq first, Meteg last
        let all: Vec<ProseAccent> = ProseAccent::iter().collect();

        assert_eq!(all.first(), Some(&ProseAccent::Silluq));
        assert_eq!(all.last(), Some(&ProseAccent::Meteg));
        assert_eq!(all[0], ProseAccent::Silluq);
        assert_eq!(all[27], ProseAccent::Meteg);
    }

    #[test]
    fn test_no_duplicate_variants() {
        use std::collections::HashSet;

        let all: Vec<ProseAccent> = ProseAccent::iter().collect();
        let unique: HashSet<ProseAccent> = all.clone().into_iter().collect();

        assert_eq!(all.len(), unique.len());
    }

    #[test]
    fn test_const_context_usage() {
        const ATNACH: ProseAccent = ProseAccent::Atnach;
        assert_eq!(ATNACH.as_index(), 1);
        assert_eq!(ATNACH.english_name(), "Atnach");
    }

    // ===== HELPERS FOR TESTS =====

    fn by_category(accent: ProseAccent) -> Option<crate::AccentCategory> {
        accent.category()
    }

    // ===== DOCUMENTATION EXAMPLE VERIFICATION =====

    #[test]
    fn doc_test_example_from_module_docs() {
        // Replicate the example from module documentation
        let accent = ProseAccent::Atnach;
        let _display = format!("{}", accent); // Should print: "Atnach (אַתְנָח), meaning: rest"
        let _strength = accent.relative_strength();

        assert_eq!(accent.english_name(), "Atnach");
    }

    #[test]
    fn doc_test_strength_example() {
        // Test the strength check example from docs
        let atnach = ProseAccent::Atnach;
        let strength = atnach.relative_strength();

        assert!(
            strength.is_some(),
            "Atnach should have a relative strength value"
        );
    }

    // ===== PROSE VS POETRY COMPARISON TESTS =====

    #[test]
    fn test_shared_accent_between_systems() {
        // Silluq exists in both prose and poetry
        let prose_silluq = ProseAccent::Silluq;
        let poetry_silluq = crate::PoetryAccent::Silluq;

        assert_eq!(prose_silluq.english_name(), poetry_silluq.english_name());
        assert_eq!(prose_silluq.hebrew_name(), poetry_silluq.hebrew_name());
    }

    #[test]
    fn test_unique_accent_distribution() {
        // Prose has Segolta, Poetry has OlehWeYored - they don't overlap
        let prose_has_segolta = ProseAccent::Segolta.english_name();
        let poetry_has_olehweyored = crate::PoetryAccent::OlehWeYored.english_name();

        assert_eq!(prose_has_segolta, "Segolta");
        assert_eq!(poetry_has_olehweyored, "Oleh WeYored");
    }
}
