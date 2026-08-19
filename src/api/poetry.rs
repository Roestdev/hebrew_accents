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
use strum_macros::{EnumCount, EnumIter};

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
/// - `EnumCount` / `EnumIter` — Auto-derived count and iteration.
/// - Discriminant values are **explicit and consecutive** to guarantee
///   that `self as usize` indexes into `POETRY_ACCENT_TABLE` correctly.
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
#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
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
    Silluq = 0,

    /// **Oleh WeYored** (עוֹלֶה וְיוֹרֵד) — "ascending and descending"
    ///
    /// A poetry-exclusive disjunctive accent that divides the verse into two
    /// halves. It serves the same structural role as Atnach in the prose system
    /// but is unique to the poetic books.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    /// - **Exclusivity**: Poetry only — presence confirms poetic context
    OlehWeYored = 1,

    /// **Atnach** (אַתְנָח) — "rest, pause"
    ///
    /// A major disjunctive that divides the verse into two halves. Also appears
    /// in the prose system but may carry a different melodic contour in poetry.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Half-verse
    Atnach = 2,

    /// **Revia Gadol** (רְבִיעַ גָּדוֹל) — "great quarter"
    ///
    /// A primary disjunctive marking a phrase boundary within a half-verse.
    /// The "Gadol" (great) designation distinguishes it from the smaller
    /// Revia Qaton.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ReviaGadol = 3,

    /// **Revia Mugrash** (רְבִיעַ מֻגְרָשׁ) — "quartered with Garesh"
    ///
    /// A disjunctive accent that combines the Revia mark with a preceding
    /// Garesh-like element. Occurs in specific poetic phrase structures.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ReviaMugrash = 4,

    /// **Shalshelet Gadol** (שַׁלְשֶׁלֶת גָּדוֹל) — "great chain"
    ///
    /// A rare disjunctive accent appearing in distinctive poetic constructions.
    /// The "Gadol" form is specific to the poetic system.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    ShalsheletGadol = 5,

    /// **Tsinnor** (צִנּוֹר) — "channel, pipe"
    ///
    /// A disjunctive accent unique to the poetic system. Appears as a
    /// prepositive mark on the accented syllable.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Phrase
    Tsinnor = 6,

    /// **Revia Qaton** (רְבִיעַ קָטָן) — "small quarter"
    ///
    /// A subordinate disjunctive marking a sub-phrase boundary. The "Qaton"
    /// (small) designation indicates a weaker pause than Revia Gadol.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    ReviaQaton = 7,

    /// **Dechi** (דְּחִי) — "pushed away"
    ///
    /// A poetry-exclusive disjunctive accent marking a minor phrase division.
    /// Its presence is a strong indicator of poetic context.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    /// - **Exclusivity**: Poetry only
    Dechi = 8,

    /// **Pazer** (פָּזֵר) — "scatter, disperse"
    ///
    /// A disjunctive accent indicating a lighter pause within a sub-phrase.
    /// Also appears in the prose system with potentially different function.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    Pazer = 9,

    /// **Mehuppakh Legarmeh** (מְהֻפָּךְ לְגַרְמֵהּ) — "inverted, alone"
    ///
    /// A disjunctive accent that combines the Mehuppakh mark with a legarmeh
    /// separator. Functions as an independent phrase boundary in poetic text.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    MehuppakhLegarmeh = 10,

    /// **Azla Legarmeh** (אַזְלָא לְגַרְמֵהּ) — "going forth, alone"
    ///
    /// A disjunctive accent combining Azla with a legarmeh separator, marking
    /// an independent phrase division.
    ///
    /// - **Category**: Disjunctive (primary)
    /// - **Hierarchy Level**: Sub-phrase
    AzlaLegarmeh = 11,

    /// **Munach** (מֻנָּח) — "resting, placed"
    ///
    /// The most common conjunctive accent in both systems. Connects a word to
    /// the following disjunctive accent without introducing a pause.
    ///
    /// - **Category**: Conjunctive (primary)
    Munach = 12,

    /// **Merkha** (מֵרכָּא) — "lengthener, drawn out"
    ///
    /// A conjunctive accent that links a word to the following accent with a
    /// forward-leaning melodic motion.
    ///
    /// - **Category**: Conjunctive (primary)
    Merkha = 13,

    /// **Illuy** (עִלּוּי) — "elevation, rising"
    ///
    /// A poetry-exclusive conjunctive accent. Its presence is a strong
    /// indicator of poetic context.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    Illuy = 14,

    /// **Tarcha** (תַּרְחָא) — "delay, lingering"
    ///
    /// A conjunctive accent connecting words within a poetic phrase. Known
    /// in some traditions as Tipcha in the prose system.
    ///
    /// - **Category**: Conjunctive (primary)
    Tarcha = 15,

    /// **Galgal** (גַּלְגַּל) — "wheel, rolling"
    ///
    /// A conjunctive accent with a rolling melodic motion. Appears in both
    /// poetic and prose systems.
    ///
    /// - **Category**: Conjunctive (primary)
    Galgal = 16,

    /// **Mehuppakh** (מְהֻפָּךְ) — "inverted, overturned"
    ///
    /// A conjunctive accent linking words within a phrase. The name refers to
    /// the inverted form of the mark compared to its disjunctive counterpart.
    ///
    /// - **Category**: Conjunctive (primary)
    Mehuppakh = 17,

    /// **Azla** (אַזְלָא) — "going forth, departure"
    ///
    /// A conjunctive accent that connects a word to the next. Also known as
    /// Qadma in some scholarly traditions.
    ///
    /// - **Category**: Conjunctive (primary)
    Azla = 18,

    /// **Shalshelet Qetannah** (שַׁלְשֶׁלֶת קְטַנָּה) — "small chain"
    ///
    /// A conjunctive counterpart to Shalshelet Gadol, connecting words rather
    /// than dividing phrases.
    ///
    /// - **Category**: Conjunctive (primary)
    ShalsheletQetannah = 19,

    /// **Tsinnorit Merkha** (צִנּוֹרִית מֵרכָּא) — "channel-like Merkha"
    ///
    /// A poetry-exclusive conjunctive accent. Combines Tsinnorit with Merkha,
    /// serving as a strong poetic-context indicator.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    TsinnoritMerkha = 20,

    /// **Tsinnorit Mahpakh** (צִנּוֹרִית מַהְפָּךְ) — "channel-like Mahpakh"
    ///
    /// A poetry-exclusive conjunctive accent. Combines Tsinnorit with Mahpakh,
    /// serving as a strong poetic-context indicator.
    ///
    /// - **Category**: Conjunctive (primary)
    /// - **Exclusivity**: Poetry only
    TsinnoritMahpakh = 21,

    /// **Meteg** (מֶתֶג) — "bridle, restraint"
    ///
    /// A secondary conjunctive mark that clarifies vowel length and prevents
    /// misreading of sheva. Does not carry independent melodic function in the
    /// primary cantillation hierarchy.
    ///
    /// - **Category**: Conjunctive (secondary)
    Meteg = 22,
}

impl PoetryAccent {
    /// The total number of poetry accent variants.
    ///
    /// Composed of 12 disjunctive + 11 conjunctive accents.
    pub const LEN: usize = <Self as strum::EnumCount>::COUNT;

    /// Returns the discriminant as `usize`, suitable for direct table indexing.
    ///
    /// This is safe to use with `POETRY_ACCENT_TABLE` because the enum's
    /// discriminant values are guaranteed to be consecutive starting at 0.
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }
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

// ── Compile-time discriminant guards ────────────────────────────────────
// If a variant is inserted, removed, or reordered, this const assertion
// will fail at compile time.
const _: () = {
    const LAST_DISCRIMINANT: u8 = PoetryAccent::Meteg as u8;
    assert!((LAST_DISCRIMINANT + 1) as usize == PoetryAccent::LEN);
};
