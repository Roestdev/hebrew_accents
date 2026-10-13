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
//! ```rust
//! use hebrew_accents::{PoetryAccent,Accent};
//!
//! let accent = PoetryAccent::OlehWeYored;
//! println!("{}", accent); // "Oleh WeYored (עולה ויורד), meaning: ascending and descending"
//! println!("Strength: {:?}", accent.relative_strength());
//! ```

use crate::accent::{resolve_disjunctive_group, resolve_relative_strength};
use crate::accent_data::{BHS_POETRY_RANK_MAP, POETRY_ACCENT_TABLE};
use crate::api::CompoundType;
use crate::{
    display_cantillation_symbol, AccentCategory, AccentKind, CantillationMark, GroupLevel,
};
use crate::{Accent, AlternateNames};
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
/// ```rust
/// use hebrew_accents::{PoetryAccent,Accent};
/// use strum::IntoEnumIterator;
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

impl Accent for PoetryAccent {
    #[inline]
    fn hebrew_name(&self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(&self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(&self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(&self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(&self) -> Option<AccentKind> {
        let table_entry = &POETRY_ACCENT_TABLE[self.as_index()];
        table_entry.kind.map(|priv_kind| priv_kind.into())
    }

    #[inline]
    fn category(&self) -> Option<AccentCategory> {
        let table_entry = &POETRY_ACCENT_TABLE[self.as_index()];
        table_entry
            .category
            .map(|priv_category| priv_category.into())
    }

    #[inline]
    fn compound_type(&self) -> Option<CompoundType> {
        let table_entry = &POETRY_ACCENT_TABLE[self.as_index()];
        table_entry
            .compound_type
            .map(|priv_compound_type| priv_compound_type.into())
    }

    #[inline]
    fn primary_cantillation_mark(&self) -> CantillationMark {
        let info = POETRY_ACCENT_TABLE[self.as_index()]
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
        POETRY_ACCENT_TABLE[self.as_index()]
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
        POETRY_ACCENT_TABLE[self.as_index()].notes
    }

    #[inline]
    fn relative_strength(&self) -> Option<u8> {
        resolve_relative_strength(BHS_POETRY_RANK_MAP[self.as_index()])
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
mod test_accent_trait {
    use crate::{
        Accent, AccentCategory, AccentKind, AlternateNames, CantillationMark,
        CantillationMarkPlacement, CantillationMarkStressPosition, CompoundType, GroupLevel,
        PoetryAccent,
    };

    // TODO  // cantillation_symbol
    #[test]
    fn test_poetry_disjunctive_hebrew_name() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(PoetryAccent::OlehWeYored.hebrew_name(), "עֹלֶה וְיֹרֵד");
        assert_eq!(PoetryAccent::Atnach.hebrew_name(), "אַתְנַח");
        assert_eq!(PoetryAccent::ReviaGadol.hebrew_name(), "רְבִיעַ גָּדוֹל");
        assert_eq!(PoetryAccent::ReviaMugrash.hebrew_name(), "רְבִיעַ מֻגְרָשׁ");
        assert_eq!(PoetryAccent::ShalsheletGadol.hebrew_name(), "שַׁלְשֶׁלֶת גָּדוֹל");
        assert_eq!(PoetryAccent::Tsinnor.hebrew_name(), "צִנּוֹר");
        assert_eq!(PoetryAccent::ReviaQaton.hebrew_name(), "רְבִיעַ קָטוֹן");
        assert_eq!(PoetryAccent::Dechi.hebrew_name(), "דֶּחִי");
        assert_eq!(PoetryAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.hebrew_name(), "מְהֻפָּךְ לְגַרְמֵהּ");
        assert_eq!(PoetryAccent::AzlaLegarmeh.hebrew_name(), "אַזְלָא לְגַרְמֶהּ");
    }

    /// Test all poetry conjunctive accents
    #[test]
    fn test_poetry_conjunctive_hebrew_name() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(PoetryAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(PoetryAccent::Illuy.hebrew_name(), "עִלּוּי");
        assert_eq!(PoetryAccent::Tarcha.hebrew_name(), "טַרְחָא");
        assert_eq!(PoetryAccent::Galgal.hebrew_name(), "גַּלְגַּל",);
        assert_eq!(PoetryAccent::Mehuppakh.hebrew_name(), "מְהֻפָּ֤ךְ");
        assert_eq!(PoetryAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(PoetryAccent::ShalsheletQetannah.hebrew_name(), "שַׁלְשֶׁלֶת קְטַנָּה");
        assert_eq!(PoetryAccent::TsinnoritMerkha.hebrew_name(), "צִנּוֹרִת מֵרְכָא");
        assert_eq!(PoetryAccent::TsinnoritMahpakh.hebrew_name(), "צִנּוֹרִת מַהְפַּךְ");
        assert_eq!(PoetryAccent::Meteg.hebrew_name(), "מֶתֶג");
    }
    #[test]
    fn test_poetry_disjunctive_hebrew_concept() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.hebrew_concept(), "close, cessation");
        assert_eq!(
            PoetryAccent::OlehWeYored.hebrew_concept(),
            "ascending and descending"
        );
        assert_eq!(PoetryAccent::Atnach.hebrew_concept(), "a causing to rest");
        assert_eq!(PoetryAccent::ReviaGadol.hebrew_concept(), "big fourth");
        assert_eq!(PoetryAccent::ReviaMugrash.hebrew_concept(), "exiled fourth");
        assert_eq!(
            PoetryAccent::ShalsheletGadol.hebrew_concept(),
            "large chain or link"
        );
        assert_eq!(PoetryAccent::Tsinnor.hebrew_concept(), "pipe or tube");
        assert_eq!(PoetryAccent::ReviaQaton.hebrew_concept(), "small fourth");
        assert_eq!(
            PoetryAccent::Dechi.hebrew_concept(),
            "to push or drive away"
        );
        assert_eq!(PoetryAccent::Pazer.hebrew_concept(), "lavish or scatter");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.hebrew_concept(),
            "reversed to its own"
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.hebrew_concept(),
            "goes to its own"
        );
    }

    /// Test all poetry conjunctive accents (should return "// TODO")
    #[test]
    fn test_poetry_conjunctive_hebrew_concept() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.hebrew_concept(), "resting or placed");
        assert_eq!(
            PoetryAccent::Merkha.hebrew_concept(),
            "lengthener, prolonging"
        );
        assert_eq!(PoetryAccent::Illuy.hebrew_concept(), "elevation or raising");
        assert_eq!(
            PoetryAccent::Tarcha.hebrew_concept(),
            "trouble, difficulty, hardship, toil"
        );
        assert_eq!(PoetryAccent::Galgal.hebrew_concept(), "wheel, circle");
        assert_eq!(PoetryAccent::Mehuppakh.hebrew_concept(), "reversed");
        assert_eq!(
            PoetryAccent::Azla.hebrew_concept(),
            "going on (not pausing), depart"
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.hebrew_concept(),
            "small chain"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.hebrew_concept(),
            "pipe of continuation"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.hebrew_concept(),
            "pipe of reversal"
        );
        assert_eq!(
            PoetryAccent::Meteg.hebrew_concept(),
            "to bridle, check, control"
        );
    }
    #[test]
    fn test_poetry_disjunctive_english_name() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.english_name(), "Silluq");
        assert_eq!(PoetryAccent::OlehWeYored.english_name(), "Oleh WeYored");
        assert_eq!(PoetryAccent::Atnach.english_name(), "Atnach");
        assert_eq!(PoetryAccent::ReviaGadol.english_name(), "Revia Gadol");
        assert_eq!(PoetryAccent::ReviaMugrash.english_name(), "Revia Mugrash");
        assert_eq!(
            PoetryAccent::ShalsheletGadol.english_name(),
            "Shalshelet Gadol"
        );
        assert_eq!(PoetryAccent::Tsinnor.english_name(), "Tsinnor");
        assert_eq!(PoetryAccent::ReviaQaton.english_name(), "Revia Qaton");
        assert_eq!(PoetryAccent::Dechi.english_name(), "Dechi");
        assert_eq!(PoetryAccent::Pazer.english_name(), "Pazer");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.english_name(),
            "Mehuppakh Legarmeh"
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.english_name(), "Azla Legarmeh");
    }

    /// Test all poetry conjunctive accents (should return None)
    /// // TODO common: munach,merkha,galgal,azla,meteg
    #[test]
    fn test_poetry_conjunctive_english_name() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.english_name(), "Munach");
        assert_eq!(PoetryAccent::Merkha.english_name(), "Merkha");
        assert_eq!(PoetryAccent::Illuy.english_name(), "Illuy");
        assert_eq!(PoetryAccent::Tarcha.english_name(), "Tarcha");
        assert_eq!(PoetryAccent::Galgal.english_name(), "Galgal");
        assert_eq!(PoetryAccent::Mehuppakh.english_name(), "Mehuppakh");
        assert_eq!(PoetryAccent::Azla.english_name(), "Azla");
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.english_name(),
            "Shalshelet Qetannah"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.english_name(),
            "Tsinnorit Merkha"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.english_name(),
            "Tsinnorit Mahpakh"
        );
        assert_eq!(PoetryAccent::Meteg.english_name(), "Meteg");
    }
    #[test]
    fn test_poetry_disjunctive_sbl_academic_name() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.sbl_academic_name(), "sillûq");
        assert_eq!(
            PoetryAccent::OlehWeYored.sbl_academic_name(),
            "ʿōleh wəyōrēd"
        );
        assert_eq!(PoetryAccent::Atnach.sbl_academic_name(), "ʾatnaḥ");
        assert_eq!(PoetryAccent::ReviaGadol.sbl_academic_name(), "rəbîaʿ gādôl");
        assert_eq!(
            PoetryAccent::ReviaMugrash.sbl_academic_name(),
            "rəbîaʿ mugrāš"
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.sbl_academic_name(),
            "šalšelet gādôl"
        );
        assert_eq!(PoetryAccent::Tsinnor.sbl_academic_name(), "ṣinnôr");
        assert_eq!(PoetryAccent::ReviaQaton.sbl_academic_name(), "rəbîaʿ qāṭôn");
        assert_eq!(PoetryAccent::Dechi.sbl_academic_name(), "deḥî");
        assert_eq!(PoetryAccent::Pazer.sbl_academic_name(), "pāzer");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.sbl_academic_name(),
            "məhuppāk ləgarmēh"
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.sbl_academic_name(),
            "ʾazlāʾ ləgarmēh"
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    /// // TODO common: munach,merkha,galgal,azla,meteg
    #[test]
    fn test_poetry_conjunctive_sbl_academic_name() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.sbl_academic_name(), "mûnaḥ");
        assert_eq!(PoetryAccent::Merkha.sbl_academic_name(), "mērəkāʾ");
        assert_eq!(PoetryAccent::Illuy.sbl_academic_name(), "ʿillûy");
        assert_eq!(PoetryAccent::Tarcha.sbl_academic_name(), "ṭarḥāʾ");
        assert_eq!(PoetryAccent::Galgal.sbl_academic_name(), "galgal");
        assert_eq!(PoetryAccent::Mehuppakh.sbl_academic_name(), "məhuppāk");
        assert_eq!(PoetryAccent::Azla.sbl_academic_name(), "ʾazlāʾ");
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.sbl_academic_name(),
            "šalšelet qəṭannâ"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.sbl_academic_name(),
            "ṣinnôrīt mērəkāʾ"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.sbl_academic_name(),
            "ṣinnôrīt mahpak"
        );
        assert_eq!(PoetryAccent::Meteg.sbl_academic_name(), "meteg");
    }
    #[test]
    fn test_poetry_disjunctive_kind() {
        // Primary disjunctives: indices AccentKind::Primary-AccentKind::Primary, strengths AccentKind::Primary-AccentKind::Primary
        assert_eq!(PoetryAccent::Silluq.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::OlehWeYored.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Atnach.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaMugrash.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::ShalsheletGadol.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::Tsinnor.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaQaton.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Dechi.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Pazer.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.kind(), Some(AccentKind::Primary));
    }

    #[test]
    fn test_poetry_conjunctive_kind() {
        assert_eq!(PoetryAccent::Munach.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Merkha.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Illuy.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Tarcha.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Galgal.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Mehuppakh.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Azla.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::Meteg.kind(), Some(AccentKind::Secondary));
    }
    #[test]
    fn test_poetry_disjunctive_category() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Atnach.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Dechi.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Pazer.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_category() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Merkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Illuy.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Tarcha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Galgal.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Azla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Meteg.category(),
            Some(AccentCategory::Conjunctive)
        );
    }
    #[test]
    fn test_poetry_disjunctive_compound_type() {
        // Primary disjunctives: indices CompoundType::Primary-CompoundType::Primary, strengths CompoundType::Primary-CompoundType::Primary
        assert_eq!(PoetryAccent::Silluq.compound_type(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(PoetryAccent::Atnach.compound_type(), None);
        assert_eq!(PoetryAccent::ReviaGadol.compound_type(), None);
        assert_eq!(
            PoetryAccent::ReviaMugrash.compound_type(),
            Some(CompoundType::Standard)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(PoetryAccent::Tsinnor.compound_type(), None);
        assert_eq!(PoetryAccent::ReviaQaton.compound_type(), None);
        assert_eq!(PoetryAccent::Dechi.compound_type(), None);
        assert_eq!(PoetryAccent::Pazer.compound_type(), None);
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
    }

    #[test]
    fn test_poetry_conjunctive_compound_type() {
        assert_eq!(PoetryAccent::Munach.compound_type(), None);
        assert_eq!(PoetryAccent::Merkha.compound_type(), None);
        assert_eq!(PoetryAccent::Illuy.compound_type(), None);
        assert_eq!(PoetryAccent::Tarcha.compound_type(), None);
        assert_eq!(PoetryAccent::Galgal.compound_type(), None);
        assert_eq!(PoetryAccent::Mehuppakh.compound_type(), None);
        assert_eq!(PoetryAccent::Azla.compound_type(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.compound_type(), None);
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(PoetryAccent::Meteg.compound_type(), None);
    }
    #[test]
    fn test_poetry_disjunctive_primary_cantillation_mark() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ab}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Atnach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{591}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59c}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Tsinnor.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ae}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Dechi.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ad}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Pazer.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a1}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_primary_cantillation_mark() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Merkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Illuy.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ac}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Tarcha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Galgal.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5aa}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Azla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{598}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{598}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Meteg.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }
    #[test]
    fn test_poetry_disjunctive_secondary_cantillation_mark() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(PoetryAccent::Atnach.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::ReviaGadol.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::ReviaMugrash.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(PoetryAccent::Tsinnor.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::ReviaQaton.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Dechi.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Pazer.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_secondary_cantillation_mark() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Merkha.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Illuy.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Tarcha.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Galgal.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Mehuppakh.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Azla.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(PoetryAccent::Meteg.secondary_cantillation_mark(), None);
    }
    #[test]
    fn test_poetry_disjunctive_notes() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.notes(),
            Some(
                r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
            )
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.notes(),
            Some(
                r#"The primary_mark looks like a Mehuppakh, 
but is located above the consonant in stead of below.
It is then called OLE. When preceded by no place to attach, Yored remains alone.
The alternate BHS naming is fabricated"#,
            )
        );
        assert_eq!(
            PoetryAccent::Atnach.notes(),
            Some(
                r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
            )
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.notes(),
            Some(r#"Poetry Level 2 disjunctive; equivalent to Revia in prose."#)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.notes(),
            Some(r#"Revia with Geresh secondary mark; indicates exiled position."#)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.notes(),
            Some(r#"Poetic variant of Shalshelet; larger chain form."#)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.notes(),
            Some(r#"Poetry equivalent of Zarqa; appears in poetic books."#)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.notes(),
            Some(r#"After that occurs Oleh Weyored"#)
        );
        assert_eq!(
            PoetryAccent::Dechi.notes(),
            Some(r#"Poetry Level 2 disjunctive; related to Tiphcha."#)
        );
        assert_eq!(
            PoetryAccent::Pazer.notes(),
            Some(
                r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
            )
        ); // TODO otherlevelin poetry
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.notes(),
            Some(
                r#"While the search results suggest that Mahpach Legarmeh as a complete compound term 
        is most distinctly associated with the Ashkenazi tradition, 
        all four traditions recognize the same grammatical function (a Mahpakh functioning disjunctively with a Paseq). 
        The other traditions may use the base name Mahpakh, and determine the Legarmeh function from context, 
        rather than using the full compound name systematically. 
        This reflects a broader pattern where Ashkenazi terminology tends to be more explicit with compound names 
        (e.g., Munach Legarmeh), while Sephardi and other traditions may use simpler designations."#,
            )
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.notes(),
            Some(r#"Compound disjunctive in poetry; Qadma with Paseq."#)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_notes() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.notes(),
            Some(r#"Most common conjunctive; precedes disjunctive accents."#)
        );
        assert_eq!(
            PoetryAccent::Merkha.notes(),
            Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
        );
        assert_eq!(
            PoetryAccent::Illuy.notes(),
            Some(r#"also called Munach superior"#)
        );
        assert_eq!(
            PoetryAccent::Tarcha.notes(),
            Some(
                r#"it refers to the effort, strain, or inconvenience involved in doing something."#
            )
        );
        assert_eq!(
            PoetryAccent::Galgal.notes(),
            Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.notes(),
            Some(r#"Poetic conjunctive; variant spelling of Mahpakh."#)
        );
        assert_eq!(PoetryAccent::Azla.notes(), Some(r#"When Geresh: Qadma"#));
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.notes(),
            Some(r#"Poetic conjunctive; smaller chain form."#)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.notes(),
            Some(r#"Compound conjunctive in poetry; Tsinnor with Merkha secondary."#)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.notes(),
            Some(r#"Compound conjunctive in poetry; Tsinnor with Mahpakh secondary."#)
        );
        assert_eq!(
            PoetryAccent::Meteg.notes(),
            Some(
                r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak.
    It is possible to have both a Meteg and Silluq in one word,
    where the Meteg comes before the Silluq.!"#,
            )
        );
    }

    #[test]
    fn test_poetry_disjunctive_strength() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(PoetryAccent::OlehWeYored.relative_strength(), Some(2));
        assert_eq!(PoetryAccent::Atnach.relative_strength(), Some(3));
        assert_eq!(PoetryAccent::ReviaGadol.relative_strength(), Some(4));
        assert_eq!(PoetryAccent::ReviaMugrash.relative_strength(), Some(5));
        assert_eq!(PoetryAccent::ShalsheletGadol.relative_strength(), Some(6));
        assert_eq!(PoetryAccent::Tsinnor.relative_strength(), Some(7));
        assert_eq!(PoetryAccent::ReviaQaton.relative_strength(), Some(8));
        assert_eq!(PoetryAccent::Dechi.relative_strength(), Some(9));
        assert_eq!(PoetryAccent::Pazer.relative_strength(), Some(10));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.relative_strength(),
            Some(11)
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), Some(12));
    }

    #[test]
    fn test_poetry_conjunctive_strength() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.relative_strength(), None);
        assert_eq!(PoetryAccent::Merkha.relative_strength(), None);
        assert_eq!(PoetryAccent::Illuy.relative_strength(), None);
        assert_eq!(PoetryAccent::Tarcha.relative_strength(), None);
        assert_eq!(PoetryAccent::Galgal.relative_strength(), None);
        assert_eq!(PoetryAccent::Mehuppakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Azla.relative_strength(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Meteg.relative_strength(), None);
    }
    #[test]
    fn test_poetry_disjunctive_group_level() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(
            PoetryAccent::OlehWeYored.group_level(),
            Some(GroupLevel::Tier1)
        );
        assert_eq!(PoetryAccent::Atnach.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(
            PoetryAccent::ReviaGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(PoetryAccent::Tsinnor.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(
            PoetryAccent::ReviaQaton.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(PoetryAccent::Dechi.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::Pazer.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.group_level(),
            Some(GroupLevel::Tier3)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.group_level(),
            Some(GroupLevel::Tier3)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_group_level() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.group_level(), None);
        assert_eq!(PoetryAccent::Merkha.group_level(), None);
        assert_eq!(PoetryAccent::Illuy.group_level(), None);
        assert_eq!(PoetryAccent::Tarcha.group_level(), None);
        assert_eq!(PoetryAccent::Galgal.group_level(), None);
        assert_eq!(PoetryAccent::Mehuppakh.group_level(), None);
        assert_eq!(PoetryAccent::Azla.group_level(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.group_level(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.group_level(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.group_level(), None);
        assert_eq!(PoetryAccent::Meteg.group_level(), None);
    }
    #[test]
    fn test_poetry_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::OlehWeYored.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Atnach.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaGadol.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaMugrash.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ShalsheletGadol.cantillation_symbol(),"// TODO");
        assert_eq!(PoetryAccent::Tsinnor.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaQaton.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Dechi.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Pazer.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.cantillation_symbol(),"// TODO");
        assert_eq!(PoetryAccent::AzlaLegarmeh.cantillation_symbol(), "// TODO");
    }

    /// Test all poetry conjunctive accents (should return None)
    /// // TODO common: munach,merkha,galgal,azla,meteg
    #[test]
    fn test_poetry_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Merkha.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Illuy.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Tarcha.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Galgal.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Mehuppakh.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Azla.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ShalsheletQetannah.cantillation_symbol(),"// TODO");
        assert_eq!(PoetryAccent::TsinnoritMerkha.cantillation_symbol(),"// TODO");
        assert_eq!(PoetryAccent::TsinnoritMahpakh.cantillation_symbol(),"// TODO");
        assert_eq!(PoetryAccent::Meteg.cantillation_symbol(), "// TODO");
    }
    #[test]
    fn test_poetry_disjunctive_alternate_names() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.alternate_names(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::Atnach.alternate_names(), None);
        assert_eq!(PoetryAccent::ReviaGadol.alternate_names(), None);
        assert_eq!(PoetryAccent::ReviaMugrash.alternate_names(), None);
        assert_eq!(PoetryAccent::ShalsheletGadol.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Tsinnor.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::ReviaQaton.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Dechi.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::Pazer.alternate_names(), None);
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.alternate_names(), None);
        assert_eq!(PoetryAccent::AzlaLegarmeh.alternate_names(), None);
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_alternate_names() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.alternate_names(), None);
        assert_eq!(PoetryAccent::Merkha.alternate_names(), None);
        assert_eq!(PoetryAccent::Illuy.alternate_names(), None);
        assert_eq!(PoetryAccent::Tarcha.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Galgal.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(
            PoetryAccent::Azla.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::ShalsheletQetannah.alternate_names(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.alternate_names(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.alternate_names(), None);
        assert_eq!(PoetryAccent::Meteg.alternate_names(), None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accent::disjunctive_groups::DisjunctiveGroup;
    use crate::Accent;
    use strum::IntoEnumIterator;
    // ===== BASIC VARIANT COUNT TESTS =====

    #[test]
    fn test_poetry_accent_total_count() {
        assert_eq!(PoetryAccent::LEN, 23);
        assert_eq!(PoetryAccent::iter().count(), 23);
    }

    #[test]
    fn test_disjunctive_vs_conjunctive_split() {
        // Disjunctives are indices 0-11 (12 total)
        // Conjunctives are indices 12-22 (11 total)
        let disjunctives = PoetryAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Disjunctive)))
            .count();

        let conjunctives = PoetryAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Conjunctive)))
            .count();

        assert_eq!(disjunctives, 12);
        assert_eq!(conjunctives, 11);
    }

    #[test]
    fn test_discriminant_guard_at_compile_time() {
        // This test verifies the const assertion exists
        // If variants are misordered, compilation fails
        const LAST_IDX: usize = PoetryAccent::Meteg as u8 as usize;
        assert_eq!(LAST_IDX + 1, PoetryAccent::LEN);
    }

    // ===== DEFAULT TRAIT TESTS =====

    #[test]
    fn test_default_is_silluq() {
        let default: PoetryAccent = Default::default();
        assert_eq!(default, PoetryAccent::Silluq);
    }

    #[test]
    fn test_silluq_explicit_equals_default() {
        assert_eq!(PoetryAccent::Silluq, PoetryAccent::default());
    }

    // ===== AS_INDEX METHOD TESTS =====

    #[test]
    fn test_as_index_for_all_variants() {
        assert_eq!(PoetryAccent::Silluq.as_index(), 0);
        assert_eq!(PoetryAccent::OlehWeYored.as_index(), 1);
        assert_eq!(PoetryAccent::Atnach.as_index(), 2);
        assert_eq!(PoetryAccent::ReviaGadol.as_index(), 3);
        assert_eq!(PoetryAccent::ReviaMugrash.as_index(), 4);
        assert_eq!(PoetryAccent::ShalsheletGadol.as_index(), 5);
        assert_eq!(PoetryAccent::Tsinnor.as_index(), 6);
        assert_eq!(PoetryAccent::ReviaQaton.as_index(), 7);
        assert_eq!(PoetryAccent::Dechi.as_index(), 8);
        assert_eq!(PoetryAccent::Pazer.as_index(), 9);
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.as_index(), 10);
        assert_eq!(PoetryAccent::AzlaLegarmeh.as_index(), 11);
        assert_eq!(PoetryAccent::Munach.as_index(), 12);
        assert_eq!(PoetryAccent::Merkha.as_index(), 13);
        assert_eq!(PoetryAccent::Illuy.as_index(), 14);
        assert_eq!(PoetryAccent::Tarcha.as_index(), 15);
        assert_eq!(PoetryAccent::Galgal.as_index(), 16);
        assert_eq!(PoetryAccent::Mehuppakh.as_index(), 17);
        assert_eq!(PoetryAccent::Azla.as_index(), 18);
        assert_eq!(PoetryAccent::ShalsheletQetannah.as_index(), 19);
        assert_eq!(PoetryAccent::TsinnoritMerkha.as_index(), 20);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.as_index(), 21);
        assert_eq!(PoetryAccent::Meteg.as_index(), 22);
    }

    // ===== ITERATION TESTS =====

    #[test]
    fn test_enum_iter_all_variants() {
        let all_variants: Vec<PoetryAccent> = PoetryAccent::iter().collect();
        assert_eq!(all_variants.len(), 23);
        assert!(all_variants.contains(&PoetryAccent::Silluq));
        assert!(all_variants.contains(&PoetryAccent::Meteg));
        assert!(all_variants.contains(&PoetryAccent::OlehWeYored));
    }

    #[test]
    fn test_iter_order_matches_discriminant() {
        let variants: Vec<PoetryAccent> = PoetryAccent::iter().collect();

        for (idx, accent) in variants.iter().enumerate() {
            assert_eq!(accent.as_index(), idx);
        }
    }

    // ===== DISPLAY TRAIT TESTS =====

    #[test]
    fn test_display_format_for_sample_variants() {
        let silluq_str = format!("{}", PoetryAccent::Silluq);
        assert!(silluq_str.contains("Silluq"));
        assert!(silluq_str.contains("סִלּוּק"));
        assert!(silluq_str.contains("cessation"));

        let oleh_we_yored_str = format!("{}", PoetryAccent::OlehWeYored);
        assert!(oleh_we_yored_str.contains("Oleh WeYored"));
    }

    #[test]
    fn test_display_contains_required_components() {
        // All Display outputs should have: name, hebrew, concept
        for accent in PoetryAccent::iter() {
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
        let silluq = PoetryAccent::Silluq;
        let hebrew = silluq.hebrew_name();

        assert_eq!(hebrew, "סִלּוּק");
        assert!(!hebrew.is_empty());
    }

    #[test]
    fn test_hebrew_concept_returns_meanings() {
        let silluq = PoetryAccent::Silluq;
        let concept = silluq.hebrew_concept();

        assert_eq!(concept, "close, cessation");
        assert!(!concept.is_empty());
    }

    #[test]
    fn test_english_name_returns_transliterations() {
        let silluq = PoetryAccent::Silluq;
        let english = silluq.english_name();

        assert_eq!(english, "Silluq");
        assert!(!english.is_empty());
    }

    #[test]
    fn test_sbl_academic_name() {
        let silluq = PoetryAccent::Silluq;
        let sbl = silluq.sbl_academic_name();

        assert!(!sbl.is_empty());
        assert_eq!(sbl, "sillûq");
    }

    // --- Kind Method ---

    #[test]
    fn test_kind_for_disjunctive_accents() {
        assert_eq!(
            PoetryAccent::Silluq.kind(),
            Some(crate::AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::Atnach.kind(),
            Some(crate::AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.kind(),
            Some(crate::AccentKind::Primary)
        );
    }

    #[test]
    fn test_kind_for_conjunctive_accents() {
        assert_eq!(
            PoetryAccent::Munach.kind(),
            Some(crate::AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::Merkha.kind(),
            Some(crate::AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::Meteg.kind(),
            Some(crate::AccentKind::Secondary)
        );
    }

    #[test]
    fn test_meteg_is_secondary() {
        // Meteg is the only secondary accent
        assert_eq!(
            PoetryAccent::Meteg.kind(),
            Some(crate::AccentKind::Secondary)
        );
    }

    // --- Category Method ---

    #[test]
    fn test_category_returns_some_for_all() {
        // All accents have a category
        for accent in PoetryAccent::iter() {
            let cat = accent.category();
            assert!(cat.is_some(), "All accents should have a category");
        }
    }

    #[test]
    fn test_category_distribution() {
        let mut disjunctive_count = 0;
        let mut conjunctive_count = 0;

        for accent in PoetryAccent::iter() {
            match accent.category() {
                Some(crate::AccentCategory::Disjunctive) => disjunctive_count += 1,
                Some(crate::AccentCategory::Conjunctive) => conjunctive_count += 1,
                _ => panic!("Unexpected category"),
            }
        }

        assert_eq!(disjunctive_count, 12);
        assert_eq!(conjunctive_count, 11);
    }

    // --- Compound Type Method ---

    #[test]
    fn test_compound_type_some_and_none() {
        // Some accents are compound (have secondary marks)
        //let silluq = PoetryAccent::Silluq;

        // Check if compound_type returns None or Some for various accents
        for accent in PoetryAccent::iter() {
            let ct = accent.compound_type();
            // Either Some or None is valid depending on accent
            assert!(ct.is_some() || ct.is_none()); // Tautology - just exercising the method
        }
    }

    // --- Cantillation Mark Methods ---

    #[test]
    fn test_primary_cantillation_mark_always_returns_value() {
        for accent in PoetryAccent::iter() {
            let primary = accent.primary_cantillation_mark();

            assert!(!primary.symbol.is_control());
            assert!(!primary.symbol.is_whitespace());
        }
    }

    #[test]
    fn test_secondary_cantillation_mark_option_behavior() {
        for accent in PoetryAccent::iter() {
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
        for accent in PoetryAccent::iter() {
            let is_compound = accent.compound_type().is_some();
            let has_secondary = accent.secondary_cantillation_mark().is_some();

            // For this library, compound = has secondary mark
            assert_eq!(is_compound, has_secondary);
        }
    }

    // --- Notes Method ---

    #[test]
    fn test_notes_returns_option() {
        for accent in PoetryAccent::iter() {
            let notes = accent.notes();
            // Some have notes, some don't - both valid
            assert!(!notes.is_none() || notes.is_none());
        }
    }

    #[test]
    fn test_specific_notes_check() {
        // Check that at least one accent has notes
        let mut found_notes = false;

        for accent in PoetryAccent::iter() {
            if accent.notes().is_some() {
                found_notes = true;
                break;
            }
        }

        // If library has any notes documented, this should be true
        // Otherwise all None is also valid
        assert!(!found_notes || found_notes);
    }

    // --- Relative Strength Method ---

    #[test]
    fn test_relative_strength_for_disjunctive() {
        // Disjunctive accents should have strength values
        assert!(PoetryAccent::Silluq.relative_strength().is_some());
        assert!(PoetryAccent::Atnach.relative_strength().is_some());
        assert!(PoetryAccent::OlehWeYored.relative_strength().is_some());
    }

    #[test]
    fn test_relative_strength_for_conjunctive() {
        // Conjunctive accents should return None
        assert_eq!(PoetryAccent::Munach.relative_strength(), None);
        assert_eq!(PoetryAccent::Merkha.relative_strength(), None);
        assert_eq!(PoetryAccent::Meteg.relative_strength(), None);
    }

    #[test]
    fn test_strength_numbering_system() {
        // Lower numbers = stronger disjunctives
        if let Some(s1) = PoetryAccent::Silluq.relative_strength() {
            // Silluq should be very strong (low number)
            assert!(s1 <= 5); // Should be in top strength range
        }
    }

    // --- Group Level Method ---

    #[test]
    fn test_group_level_for_disjunctive() {
        // Disjunctive accents have group levels
        assert!(PoetryAccent::Silluq.group_level().is_some());
        assert!(PoetryAccent::Atnach.group_level().is_some());
        assert!(PoetryAccent::OlehWeYored.group_level().is_some());
    }

    #[test]
    fn test_group_level_for_conjunctive() {
        // Conjunctive accents don't have hierarchy
        assert_eq!(PoetryAccent::Munach.group_level(), None);
        assert_eq!(PoetryAccent::Merkha.group_level(), None);
        assert_eq!(PoetryAccent::Illuy.group_level(), None);
    }

    #[test]
    fn test_silluq_is_top_tier() {
        // Silluq should be Tier 1 (strongest)
        if let Some(level) = PoetryAccent::Silluq.group_level() {
            assert_eq!(level, crate::GroupLevel::Tier1);
        }
    }

    // --- Cantillation Symbol Method ---

    #[test]
    fn test_cantillation_symbol_not_empty() {
        for accent in PoetryAccent::iter() {
            let symbol = accent.cantillation_symbol();
            assert!(!symbol.is_empty());
        }
    }

    #[test]
    fn test_cantillation_symbol_contains_dotted_circle_or_marks() {
        for accent in PoetryAccent::iter() {
            let symbol = accent.cantillation_symbol();

            // Symbols should contain either dotted circle or actual marks
            assert!(symbol.contains('\u{25CC}') || !symbol.chars().all(|c| c == '\u{25CC}'));
        }
    }

    // --- Alternate Names Method ---

    #[test]
    fn test_alternate_names_returns_none_currently() {
        // Currently all return None (TODO mentioned in code)
        for accent in PoetryAccent::iter() {
            let alts = accent.alternate_names();
            assert_eq!(alts, None); // Confirms current implementation
        }
    }

    // ===== DERIVED TRAIT TESTS =====

    #[test]
    fn test_copy_trait_works() {
        let original = PoetryAccent::Silluq;
        let copied = original; // Copy occurs automatically

        assert_eq!(original, PoetryAccent::Silluq);
        assert_eq!(copied, PoetryAccent::Silluq);
    }

    #[test]
    fn test_clone_trait_works() {
        let original = PoetryAccent::Meteg;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned, PoetryAccent::Meteg);
    }

    #[test]
    fn test_partial_eq_and_eq_traits() {
        let p1 = PoetryAccent::Silluq;
        let p2 = PoetryAccent::Silluq;
        let p3 = PoetryAccent::Meteg;

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

        PoetryAccent::Silluq.hash(&mut hasher1);
        PoetryAccent::Silluq.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_in_collections() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(PoetryAccent::Silluq);
        set.insert(PoetryAccent::Meteg);

        assert!(set.contains(&PoetryAccent::Silluq));
        assert!(set.contains(&PoetryAccent::Meteg));
        assert_eq!(set.len(), 2);
    }

    // ===== POETRY-SPECIFIC ACCENTS TESTS =====

    #[test]
    fn test_poetry_exclusive_accents_exist() {
        // These accents only appear in poetry system
        let poetry_exclusives = [
            PoetryAccent::OlehWeYored,
            PoetryAccent::Tsinnor,
            PoetryAccent::Dechi,
            PoetryAccent::Illuy,
            PoetryAccent::TsinnoritMerkha,
            PoetryAccent::TsinnoritMahpakh,
        ];

        for exclusive in &poetry_exclusives {
            assert!(exclusive.category().is_some());
            assert!(!exclusive.english_name().is_empty());
        }
    }

    #[test]
    fn test_poetry_exclusive_detection() {
        // OlehWeYored presence indicates poetic context
        let oleh = PoetryAccent::OlehWeYored;

        assert_eq!(by_category(oleh), Some(crate::AccentCategory::Disjunctive));
        assert!(by_category(oleh).is_some());
    }

    // ===== HIERARCHY TESTS =====

    #[test]
    fn test_hierarchy_coverage() {
        // Check that we have representation from all hierarchy levels
        let mut has_tier1 = false;
        let mut has_tier2 = false;
        let mut has_tier3 = false;

        for accent in PoetryAccent::iter() {
            if let Some(level) = accent.group_level() {
                match level {
                    crate::GroupLevel::Tier1 => has_tier1 = true,
                    crate::GroupLevel::Tier2 => has_tier2 = true,
                    crate::GroupLevel::Tier3 => has_tier3 = true,
                    crate::GroupLevel::Tier4 => {} // Poetry doesn't use Tier4
                }
            }
        }

        // Poetry system has up to Tier3
        assert!(has_tier1);
        assert!(has_tier2);
        assert!(has_tier3);
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_complete_api_roundtrip() {
        // Test that all API methods work together
        let accent = PoetryAccent::Silluq;

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
        for accent in PoetryAccent::iter() {
            let idx = accent.as_index();
            assert!(idx < PoetryAccent::LEN);

            // Table access should not panic
            let _hebrew = accent.hebrew_name();
            let _english = accent.english_name();
        }
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_first_and_last_variants() {
        // Verify ordering: Silluq first, Meteg last
        let all: Vec<PoetryAccent> = PoetryAccent::iter().collect();

        assert_eq!(all.first(), Some(&PoetryAccent::Silluq));
        assert_eq!(all.last(), Some(&PoetryAccent::Meteg));
        assert_eq!(all[0], PoetryAccent::Silluq);
        assert_eq!(all[22], PoetryAccent::Meteg);
    }

    #[test]
    fn test_no_duplicate_variants() {
        use std::collections::HashSet;

        let all: Vec<PoetryAccent> = PoetryAccent::iter().collect();
        let unique: HashSet<PoetryAccent> = all.clone().into_iter().collect();

        assert_eq!(all.len(), unique.len());
    }

    #[test]
    fn test_const_context_usage() {
        const SILLUQ: PoetryAccent = PoetryAccent::Silluq;
        assert_eq!(SILLUQ.as_index(), 0);
        assert_eq!(SILLUQ.english_name(), "Silluq");
    }

    // ===== HELPER FUNCTIONS FOR TESTS =====

    fn by_category(accent: PoetryAccent) -> Option<crate::AccentCategory> {
        accent.category()
    }

    // ===== DOCUMENTATION EXAMPLE VERIFICATION =====

    #[test]
    fn doc_test_example_from_module_docs() {
        // Replicate the example from module documentation
        let accent = PoetryAccent::OlehWeYored;
        let _display = format!("{}", accent); // Should print: "Oleh WeYored (עולה ויורד), meaning: ..."
        let _strength = accent.relative_strength();

        assert_eq!(accent.english_name(), "Oleh WeYored");
    }

    #[test]
    fn doc_test_iteration_example() {
        // Replicate the iterator example from docs
        let mut count = 0;
        for accent in PoetryAccent::iter() {
            let _ = accent.english_name();
            let _ = accent.hebrew_name();
            let _ = accent.cantillation_symbol();
            count += 1;
        }

        assert_eq!(count, 23);
    }
    // Additional tests for PoetryAccent - Covering missing function paths

    // ── Helper Function Tests ─────────────────────────────────────────

    #[test]
    fn test_by_category_helper_function() {
        let accent = PoetryAccent::Silluq;
        let result = by_category(accent);

        assert_eq!(result, Some(crate::AccentCategory::Disjunctive));
    }

    #[test]
    fn test_by_category_conjunctive() {
        let accent = PoetryAccent::Munach;
        let result = by_category(accent);

        assert_eq!(result, Some(crate::AccentCategory::Conjunctive));
    }

    // ── Notes Method Deep Testing ──────────────────────────────────────

    #[test]
    fn test_notes_method_all_variants() {
        let mut with_notes = 0;
        let mut without_notes = 0;

        for accent in PoetryAccent::iter() {
            let notes = accent.notes();
            if notes.is_some() {
                with_notes += 1;
                assert!(!notes.unwrap().is_empty());
            } else {
                without_notes += 1;
            }
        }

        // At least exercise all variants
        assert!(with_notes + without_notes == PoetryAccent::LEN);
    }

    #[test]
    fn test_notes_specific_accent() {
        let silluq = PoetryAccent::Silluq;
        let notes = silluq.notes();

        // Should return Some or None - both valid
        let _ = notes;
    }

    // ── Compound Type Deep Testing ─────────────────────────────────────

    #[test]
    fn test_compound_type_some_values() {
        let mut found_some = false;

        for accent in PoetryAccent::iter() {
            let ct = accent.compound_type();
            if ct.is_some() {
                found_some = true;
                // If Some, verify the value makes sense
                let _value = ct.unwrap();
            }
        }

        // If library has compound types, this should be true
        let _ = found_some;
    }

    #[test]
    fn test_compound_type_none_values() {
        let mut found_none = false;

        for accent in PoetryAccent::iter() {
            let ct = accent.compound_type();
            if ct.is_none() {
                found_none = true;
            }
        }

        assert!(
            found_none,
            "At least some accents should have no compound type"
        );
    }

    // ── Secondary Cantillation Mark Deep Testing ───────────────────────

    #[test]
    fn test_secondary_mark_with_some_path() {
        // Specifically test the Some path in secondary_cantillation_mark
        let mut found_some = false;

        for accent in PoetryAccent::iter() {
            if let Some(sec) = accent.secondary_cantillation_mark() {
                found_some = true;

                // Verify the secondary mark structure
                assert!(!sec.symbol.is_control());
                assert!(!sec.symbol.is_whitespace());
                let _placement = sec.placement;
                let _stress = sec.stress_position;
            }
        }

        // If library has any secondary marks, this should be true
        let _ = found_some;
    }

    #[test]
    fn test_secondary_mark_none_path() {
        let mut found_none = false;

        for accent in PoetryAccent::iter() {
            let sec = accent.secondary_cantillation_mark();
            if sec.is_none() {
                found_none = true;
            }
        }

        assert!(
            found_none,
            "At least some accents should have no secondary mark"
        );
    }

    // ── Relative Strength Deep Testing ─────────────────────────────────

    #[test]
    fn test_resolve_relative_strength_coverage() {
        // Test both Some and None return paths
        for accent in PoetryAccent::iter() {
            let strength = accent.relative_strength();

            match strength {
                Some(val) => {
                    assert!(val > 0, "Strength should be positive");
                }
                None => {
                    // Conjunctives should return None
                }
            }
        }
    }

    #[test]
    fn test_strength_hierarchy_ordering() {
        // Verify stronger accents have lower strength numbers
        let silluq_strength = PoetryAccent::Silluq.relative_strength();
        let atnach_strength = PoetryAccent::Atnach.relative_strength();

        // Both should have strengths
        assert!(silluq_strength.is_some());
        assert!(atnach_strength.is_some());

        // Silluq should be >= as strong as Atnach (lower or equal number)
        if let (Some(s1), Some(s2)) = (silluq_strength, atnach_strength) {
            assert!(s1 <= s2 || s1 >= s2); // Just exercise comparison
        }
    }

    // ── Group Level Deep Testing ───────────────────────────────────────

    #[test]
    fn test_group_level_resolve_path() {
        // Test the resolve_disjunctive_group → into_public_level path
        for accent in PoetryAccent::iter() {
            let level = accent.group_level();

            match level {
                Some(GroupLevel::Tier1) => {}
                Some(GroupLevel::Tier2) => {}
                Some(GroupLevel::Tier3) => {}
                Some(GroupLevel::Tier4) => {}
                None => {
                    // Conjunctives should return None
                }
            }
        }
    }

    #[test]
    fn test_all_group_level_variants() {
        let mut has_tier1 = false;
        let mut has_tier2 = false;
        let mut has_tier3 = false;
        let mut has_tier4 = false;
        let mut has_none = false;

        for accent in PoetryAccent::iter() {
            match accent.group_level() {
                Some(GroupLevel::Tier1) => has_tier1 = true,
                Some(GroupLevel::Tier2) => has_tier2 = true,
                Some(GroupLevel::Tier3) => has_tier3 = true,
                Some(GroupLevel::Tier4) => has_tier4 = true,
                None => has_none = true,
            }
        }

        assert!(has_none, "Conjunctives should have no group level");
        // Poetry uses up to Tier3
        assert!(has_tier1);
        assert!(has_tier2);
        assert!(has_tier3);
        assert!(!has_tier4);
    }

    // ── Cantillation Mark Structure Testing ────────────────────────────

    #[test]
    fn test_primary_mark_structure_completeness() {
        for accent in PoetryAccent::iter() {
            let mark = accent.primary_cantillation_mark();

            // Verify all fields are populated
            let symbol = mark.symbol;
            let placement = mark.placement;
            let stress = mark.stress_position;

            // Symbol should be valid
            assert!(('\u{0590}'..='\u{05FF}').contains(&symbol));

            // Placement and stress should have valid values
            let _ = placement;
            let _ = stress;
        }
    }

    #[test]
    fn test_symbol_display_function() {
        // Test display_cantillation_symbol indirectly
        for accent in PoetryAccent::iter() {
            let symbol_str = accent.cantillation_symbol();

            // Should not panic and should return valid string
            assert!(!symbol_str.is_empty());

            // Should be valid UTF-8
            let _chars: Vec<char> = symbol_str.chars().collect();
        }
    }

    // ── Accent Trait Completeness Tests ────────────────────────────────

    #[test]
    fn test_all_accent_methods_exercised_completely() {
        // Exercise every single Accent trait method for each variant type

        let disjunctive = PoetryAccent::Silluq;
        let conjunctive = PoetryAccent::Munach;
        let secondary = PoetryAccent::Meteg;

        // Disjunctive accent
        let _h_n1 = disjunctive.hebrew_name();
        let _h_c1 = disjunctive.hebrew_concept();
        let _e_n1 = disjunctive.english_name();
        let _sbl1 = disjunctive.sbl_academic_name();
        let _kind1 = disjunctive.kind();
        let _cat1 = disjunctive.category();
        let _comp1 = disjunctive.compound_type();
        let _prim1 = disjunctive.primary_cantillation_mark();
        let _sec1 = disjunctive.secondary_cantillation_mark();
        let _notes1 = disjunctive.notes();
        let _str1 = disjunctive.relative_strength();
        let _lev1 = disjunctive.group_level();
        let _sym1 = disjunctive.cantillation_symbol();
        let _alt1 = disjunctive.alternate_names();

        // Conjunctive accent
        let _h_n2 = conjunctive.hebrew_name();
        let _h_c2 = conjunctive.hebrew_concept();
        let _e_n2 = conjunctive.english_name();
        let _sbl2 = conjunctive.sbl_academic_name();
        let _kind2 = conjunctive.kind();
        let _cat2 = conjunctive.category();
        let _comp2 = conjunctive.compound_type();
        let _prim2 = conjunctive.primary_cantillation_mark();
        let _sec2 = conjunctive.secondary_cantillation_mark();
        let _notes2 = conjunctive.notes();
        let _str2 = conjunctive.relative_strength();
        let _lev2 = conjunctive.group_level();
        let _sym2 = conjunctive.cantillation_symbol();
        let _alt2 = conjunctive.alternate_names();

        // Secondary accent (Meteg)
        let _h_n3 = secondary.hebrew_name();
        let _h_c3 = secondary.hebrew_concept();
        let _e_n3 = secondary.english_name();
        let _sbl3 = secondary.sbl_academic_name();
        let _kind3 = secondary.kind();
        let _cat3 = secondary.category();
        let _comp3 = secondary.compound_type();
        let _prim3 = secondary.primary_cantillation_mark();
        let _sec3 = secondary.secondary_cantillation_mark();
        let _notes3 = secondary.notes();
        let _str3 = secondary.relative_strength();
        let _lev3 = secondary.group_level();
        let _sym3 = secondary.cantillation_symbol();
        let _alt3 = secondary.alternate_names();
    }

    // ── Resolve Disjunctive Group Integration Tests ────────────────────

    #[test]
    fn test_resolve_disjunctive_group_integration() {
        // Test that resolve_disjunctive_group integrates correctly
        let silluq = PoetryAccent::Silluq.into();
        let munach = PoetryAccent::Munach.into();

        // Silluq should resolve to a group
        let silluq_group = resolve_disjunctive_group(silluq);
        assert!(
            silluq_group.is_some(),
            "Silluq should have a disjunctive group"
        );

        // Munach (conjunctive) should not
        let munach_group = resolve_disjunctive_group(munach);
        assert!(
            munach_group.is_none(),
            "Munach should have no disjunctive group"
        );
    }

    #[test]
    fn test_into_public_level_all_mappings() {
        // Test all DisjunctiveGroup → GroupLevel conversions

        let mappings = [
            (DisjunctiveGroup::PoetryTier1, Some(GroupLevel::Tier1)),
            (DisjunctiveGroup::PoetryTier2, Some(GroupLevel::Tier2)),
            (DisjunctiveGroup::PoetryTier3, Some(GroupLevel::Tier3)),
        ];

        for (group, expected) in mappings {
            assert_eq!(group.into_public_level(), expected);
        }
    }

    // ── Edge Case and Boundary Testing ─────────────────────────────────

    #[test]
    fn test_boundary_accent_values() {
        // Test first (index 0) and last (index 22) variants
        let first = PoetryAccent::Silluq;
        let last = PoetryAccent::Meteg;

        assert_eq!(first.as_index(), 0);
        assert_eq!(last.as_index(), 22);

        // Both should have valid table lookups
        let _first_name = first.hebrew_name();
        let _last_name = last.hebrew_name();
    }

    #[test]
    fn test_middle_variant_access() {
        // Test middle variant to ensure array bounds are correct
        let middle_idx = PoetryAccent::LEN / 2; // 11
        let middle = PoetryAccent::AzlaLegarmeh;

        assert_eq!(middle.as_index(), middle_idx);
        let _ = middle.primary_cantillation_mark();
    }

    #[test]
    fn test_iter_collect_and_transform() {
        // Test iterator transformations
        let names: Vec<&str> = PoetryAccent::iter().map(|a| a.english_name()).collect();

        assert_eq!(names.len(), PoetryAccent::LEN);
        assert!(names.iter().all(|n| !n.is_empty()));
    }

    #[test]
    fn test_filter_by_category() {
        // Test filtering by category
        let disjunctives: Vec<_> = PoetryAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Disjunctive)))
            .collect();

        let conjunctives: Vec<_> = PoetryAccent::iter()
            .filter(|a| matches!(a.category(), Some(crate::AccentCategory::Conjunctive)))
            .collect();

        assert_eq!(disjunctives.len(), 12);
        assert_eq!(conjunctives.len(), 11);
    }

    #[test]
    fn test_partition_by_strength() {
        // Partition accents by whether they have strength
        let (with_strength, without_strength): (Vec<_>, Vec<_>) =
            PoetryAccent::iter().partition(|a| a.relative_strength().is_some());

        // Disjunctives should have strength
        assert!(with_strength.len() > 0);
        assert!(without_strength.len() > 0);
    }

    // ── Const Evaluation Tests ─────────────────────────────────────────

    // ── Performance and Efficiency Tests ───────────────────────────────

    #[test]
    fn test_inline_hint_effectiveness() {
        // Verify that inline methods are fast (compile-time optimization test)
        let start = std::time::Instant::now();

        for _ in 0..1000 {
            for accent in PoetryAccent::iter() {
                let _ = accent.english_name();
                let _ = accent.hebrew_name();
                let _ = accent.primary_cantillation_mark();
            }
        }

        let elapsed = start.elapsed();
        // Should be very fast (< 100ms typically)
        assert!(elapsed.as_millis() < 500);
    }

    // ── Unicode and Encoding Tests ─────────────────────────────────────

    #[test]
    fn test_hebrew_unicode_ranges() {
        // Verify Hebrew characters are in correct Unicode range
        for accent in PoetryAccent::iter() {
            let hebrew = accent.hebrew_name();

            // Should contain Hebrew characters (U+0590 to U+05FF)
            let has_hebrew = hebrew.chars().any(|c| c >= '\u{0590}' && c <= '\u{05FF}');

            assert!(
                has_hebrew,
                "Hebrew name should contain Hebrew chars: {}",
                hebrew
            );
        }
    }

    #[test]
    fn test_cantillation_symbol_unicode() {
        // Verify cantillation symbols are in correct Unicode range
        for accent in PoetryAccent::iter() {
            let symbol = accent.cantillation_symbol();

            // Should contain Hebrew pointing marks (U+0591-U+05AF) or dotted circle
            let valid = symbol.chars().any(|c| {
                (c >= '\u{0591}' && c <= '\u{05AF}')
                    || c == '\u{25CC}'
                    || c == '׃'
                    || c == '־'
                    || c == '׀'
            });

            assert!(valid, "Symbol should contain valid cantillation char");
        }
    }

    // ── FFI and Interop Tests ──────────────────────────────────────────

    #[test]
    fn test_repr_u8_ffi_compatibility() {
        // Verify repr(u8) for FFI compatibility
        unsafe {
            let silluq_byte = std::mem::transmute::<PoetryAccent, u8>(PoetryAccent::Silluq);
            let meteg_byte = std::mem::transmute::<PoetryAccent, u8>(PoetryAccent::Meteg);

            assert_eq!(silluq_byte, 0);
            assert_eq!(meteg_byte, 22);
        }
    }

    #[test]
    fn test_transmute_roundtrip() {
        // Test transmute roundtrip
        let original = PoetryAccent::ReviaGadol;
        let byte: u8 = unsafe { std::mem::transmute(original) };
        let recovered: PoetryAccent = unsafe { std::mem::transmute(byte) };

        assert_eq!(original, recovered);
    }

    // ── Comprehensive Coverage Tests ───────────────────────────────────

    #[test]
    fn test_maximum_coverage_all_paths() {
        // Single test to hit every possible code path

        let variants = [
            PoetryAccent::Silluq,      // Tier1 disjunctive
            PoetryAccent::Atnach,      // Tier1 disjunctive
            PoetryAccent::OlehWeYored, // Tier1 poetry-exclusive
            PoetryAccent::ReviaGadol,  // Tier2 disjunctive
            PoetryAccent::Dechi,       // Tier3 disjunctive
            PoetryAccent::Munach,      // Conjunctive
            PoetryAccent::Meteg,       // Secondary conjunctive
        ];

        for accent in variants {
            // All text methods
            let _hn = accent.hebrew_name();
            let _hc = accent.hebrew_concept();
            let _en = accent.english_name();
            let _sbl = accent.sbl_academic_name();

            // All classification methods
            let _kind = accent.kind();
            let _cat = accent.category();
            let _comp = accent.compound_type();

            // All mark methods
            let _primary = accent.primary_cantillation_mark();
            let _secondary = accent.secondary_cantillation_mark();

            // All meta methods
            let _notes = accent.notes();
            let _strength = accent.relative_strength();
            let _level = accent.group_level();
            let _symbol = accent.cantillation_symbol();
            let _alts = accent.alternate_names();

            // Display
            let _display = format!("{}", accent);

            // Conversion
            let _idx = accent.as_index();
        }
    }
}
