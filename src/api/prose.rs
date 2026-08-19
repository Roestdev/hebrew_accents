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
/// ```ignore
/// use crate::api::ProseAccent;
/// use crate::Accent;
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
