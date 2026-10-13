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
use crate::Accent;
use crate::{
    display_cantillation_symbol, AccentCategory, AccentKind, CantillationMark, GroupLevel,
};
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
        let table_entry = &PROSE_ACCENT_TABLE[self.as_index()];
        table_entry.kind.map(|priv_kind| priv_kind.into())
    }

    #[inline]
    fn category(&self) -> Option<AccentCategory> {
        let table_entry = &PROSE_ACCENT_TABLE[self.as_index()];
        table_entry
            .category
            .map(|priv_category| priv_category.into())
    }

    #[inline]
    fn compound_type(&self) -> Option<CompoundType> {
        let table_entry = &PROSE_ACCENT_TABLE[self.as_index()];
        table_entry
            .compound_type
            .map(|priv_compound_type| priv_compound_type.into())
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
    fn alternate_names(&self) -> Option<crate::api::AlternateNames> {
        let table_entry = &PROSE_ACCENT_TABLE[self.as_index()];
        table_entry
            .alternate_names
            .map(|priv_names| priv_names.into())
    }
}

#[cfg(test)]
mod test_accent_trait {
    use crate::{
        Accent, AccentCategory, AccentKind, AlternateNames, CantillationMark,
        CantillationMarkPlacement, CantillationMarkStressPosition, CompoundType, GroupLevel,
        ProseAccent,
    };
    // TODO  // cantillation_symbol
    #[test]
    fn test_prose_disjunctive_hebrew_name() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(ProseAccent::Atnach.hebrew_name(), "אַתְנַח");
        assert_eq!(ProseAccent::Segolta.hebrew_name(), "סְגֹולְתָּא");
        assert_eq!(ProseAccent::Shalshelet.hebrew_name(), "שַׁלְשֶׁלֶת");
        assert_eq!(ProseAccent::ZaqephQatan.hebrew_name(), "זָקֵף קָטוֹן");
        assert_eq!(ProseAccent::ZaqephGadol.hebrew_name(), "זָקֵף גָּדוֹל");
        assert_eq!(ProseAccent::Revia.hebrew_name(), "רְבִיעַ");
        assert_eq!(ProseAccent::Tiphcha.hebrew_name(), "טִפְחָא");
        assert_eq!(ProseAccent::Zarqa.hebrew_name(), "זַרְקָא");
        assert_eq!(ProseAccent::Pashta.hebrew_name(), "פַּשְׁטָא");
        assert_eq!(ProseAccent::Yetiv.hebrew_name(), "יְתִיב");
        assert_eq!(ProseAccent::Tevir.hebrew_name(), "תְּבִיר");
        assert_eq!(ProseAccent::Geresh.hebrew_name(), "גֵּרֵישׁ");
        assert_eq!(ProseAccent::Gershayim.hebrew_name(), "גֵּרְשַׁיִם");
        assert_eq!(ProseAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(ProseAccent::PazerGadol.hebrew_name(), "פָּזֶר גּדוֹל");
        assert_eq!(ProseAccent::TelishaGedolah.hebrew_name(), "תְּלִישָׁא גְּדוֹלָה");
        assert_eq!(ProseAccent::Legarmeh.hebrew_name(), "לְגַרְמֶהּ");
    }

    #[test]
    fn test_prose_conjunctive_hebrew_name() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(ProseAccent::Mahpakh.hebrew_name(), "מַהְפַּךְ");
        assert_eq!(ProseAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(ProseAccent::MerkhaKephulah.hebrew_name(), "מֵרְכָא כְּפוּלָה");
        assert_eq!(ProseAccent::Darga.hebrew_name(), "דַּרְגָּא");
        assert_eq!(ProseAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(ProseAccent::TelishaQetannah.hebrew_name(), "תְּלִישָא קְטַנָּה");
        assert_eq!(ProseAccent::Galgal.hebrew_name(), "גַּלְגַּל",);
        assert_eq!(ProseAccent::Meayla.hebrew_name(), "מְאַיְלָא");
        assert_eq!(ProseAccent::Meteg.hebrew_name(), "מֶתֶג");
    }
    #[test]
    fn test_prose_disjunctive_hebrew_concept() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.hebrew_concept(), "close, cessation");
        assert_eq!(ProseAccent::Atnach.hebrew_concept(), "a causing to rest");
        assert_eq!(
            ProseAccent::Segolta.hebrew_concept(),
            "a little grape-bunch"
        );
        assert_eq!(ProseAccent::Shalshelet.hebrew_concept(), "chain or link");
        assert_eq!(
            ProseAccent::ZaqephQatan.hebrew_concept(),
            "small upright or small standing one"
        );
        assert_eq!(ProseAccent::ZaqephGadol.hebrew_concept(), "large upright");
        assert_eq!(
            ProseAccent::Revia.hebrew_concept(),
            "fourth [in a sequence]"
        );
        assert_eq!(
            ProseAccent::Tiphcha.hebrew_concept(),
            "handbreadth or diagonal"
        );
        assert_eq!(
            ProseAccent::Zarqa.hebrew_concept(),
            "to sprinkle, to scatter"
        );
        assert_eq!(
            ProseAccent::Pashta.hebrew_concept(),
            "extending, stretching out in length"
        );
        assert_eq!(ProseAccent::Yetiv.hebrew_concept(), "resting or sitting");
        assert_eq!(
            ProseAccent::Tevir.hebrew_concept(),
            "broken, downward tumble"
        );
        assert_eq!(
            ProseAccent::Geresh.hebrew_concept(),
            "expulsion, driving out, divorce"
        );
        assert_eq!(
            ProseAccent::Gershayim.hebrew_concept(),
            "double of expulsion, driving out, divorce"
        );
        assert_eq!(ProseAccent::Pazer.hebrew_concept(), "lavish or scatter");
        assert_eq!(
            ProseAccent::PazerGadol.hebrew_concept(),
            "large lavish or scatter"
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.hebrew_concept(),
            "great (long) detached"
        );
        assert_eq!(
            ProseAccent::Legarmeh.hebrew_concept(),
            "for or by itself, independent"
        );
    }

    #[test]
    fn test_prose_conjunctive_hebrew_concept() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.hebrew_concept(), "resting or placed");
        assert_eq!(ProseAccent::Mahpakh.hebrew_concept(), "turning round");
        assert_eq!(
            ProseAccent::Merkha.hebrew_concept(),
            "lengthener, prolonging"
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.hebrew_concept(),
            "double lengthener"
        );
        assert_eq!(ProseAccent::Darga.hebrew_concept(), "step, stair-step");
        assert_eq!(
            ProseAccent::Azla.hebrew_concept(),
            "going on (not pausing), depart"
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.hebrew_concept(),
            "small (short) detached"
        );
        assert_eq!(ProseAccent::Galgal.hebrew_concept(), "wheel, circle");
        assert_eq!(ProseAccent::Meayla.hebrew_concept(), "upper, above, higher");
        assert_eq!(
            ProseAccent::Meteg.hebrew_concept(),
            "to bridle, check, control"
        );
    }
    #[test]
    fn test_prose_disjunctive_english_name() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.english_name(), "Silluq");
        assert_eq!(ProseAccent::Atnach.english_name(), "Atnach");
        assert_eq!(ProseAccent::Segolta.english_name(), "Segolta");
        assert_eq!(ProseAccent::Shalshelet.english_name(), "Shalshelet");
        // TODO assert_eq!(ProseAccent::ZaqephQatan.english_name(), "Zaqeph Qatan");
        assert_eq!(ProseAccent::ZaqephGadol.english_name(), "Zaqeph Gadol");
        assert_eq!(ProseAccent::Revia.english_name(), "Revia");
        assert_eq!(ProseAccent::Tiphcha.english_name(), "Tiphcha");
        assert_eq!(ProseAccent::Zarqa.english_name(), "Zarqa");
        assert_eq!(ProseAccent::Pashta.english_name(), "Pashta");
        assert_eq!(ProseAccent::Yetiv.english_name(), "Yetiv");
        assert_eq!(ProseAccent::Tevir.english_name(), "Tevir");
        assert_eq!(ProseAccent::Geresh.english_name(), "Geresh");
        assert_eq!(ProseAccent::Gershayim.english_name(), "Gershayim");
        assert_eq!(ProseAccent::Pazer.english_name(), "Pazer");
        assert_eq!(ProseAccent::PazerGadol.english_name(), "Pazer Gadol");
        assert_eq!(
            ProseAccent::TelishaGedolah.english_name(),
            "Telisha Gedolah"
        );
        assert_eq!(ProseAccent::Legarmeh.english_name(), "Legarmeh");
    }

    #[test]
    fn test_prose_conjunctive_english_name() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.english_name(), "Munach");
        assert_eq!(ProseAccent::Mahpakh.english_name(), "Mahpakh");
        assert_eq!(ProseAccent::Merkha.english_name(), "Merkha");
        assert_eq!(
            ProseAccent::MerkhaKephulah.english_name(),
            "Merkha Kephulah"
        );
        assert_eq!(ProseAccent::Darga.english_name(), "Darga");
        assert_eq!(ProseAccent::Azla.english_name(), "Azla");
        assert_eq!(
            ProseAccent::TelishaQetannah.english_name(),
            "Telisha Qetannah"
        );
        assert_eq!(ProseAccent::Galgal.english_name(), "Galgal");
        assert_eq!(ProseAccent::Meayla.english_name(), "Meayla");
        assert_eq!(ProseAccent::Meteg.english_name(), "Meteg");
    }
    #[test]
    fn test_prose_disjunctive_sbl_academic_name() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.sbl_academic_name(), "sillûq");
        assert_eq!(ProseAccent::Atnach.sbl_academic_name(), "ʾatnaḥ");
        assert_eq!(ProseAccent::Segolta.sbl_academic_name(), "səgôlətāʾ");
        assert_eq!(ProseAccent::Shalshelet.sbl_academic_name(), "šalšelet");
        assert_eq!(ProseAccent::ZaqephQatan.sbl_academic_name(), "zāqēp qāṭôn");
        assert_eq!(ProseAccent::ZaqephGadol.sbl_academic_name(), "zāqēp gādôl");
        assert_eq!(ProseAccent::Revia.sbl_academic_name(), "rəbîaʿ");
        assert_eq!(ProseAccent::Tiphcha.sbl_academic_name(), "ṭipḥāʾ");
        assert_eq!(ProseAccent::Zarqa.sbl_academic_name(), "zarqāʾ");
        assert_eq!(ProseAccent::Pashta.sbl_academic_name(), "pašṭāʾ");
        assert_eq!(ProseAccent::Yetiv.sbl_academic_name(), "yətîb");
        assert_eq!(ProseAccent::Tevir.sbl_academic_name(), "təbîr");
        assert_eq!(ProseAccent::Geresh.sbl_academic_name(), "gērêš");
        assert_eq!(ProseAccent::Gershayim.sbl_academic_name(), "gērəšayīm");
        assert_eq!(ProseAccent::Pazer.sbl_academic_name(), "pāzer");
        assert_eq!(ProseAccent::PazerGadol.sbl_academic_name(), "pāzer gdôl");
        assert_eq!(
            ProseAccent::TelishaGedolah.sbl_academic_name(),
            "təlîšāʾ gədôlâ"
        );
        assert_eq!(ProseAccent::Legarmeh.sbl_academic_name(), "ləgarmeh");
    }

    #[test]
    fn test_prose_conjunctive_sbl_academic_name() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.sbl_academic_name(), "mûnaḥ");
        assert_eq!(ProseAccent::Mahpakh.sbl_academic_name(), "mahpak");
        assert_eq!(ProseAccent::Merkha.sbl_academic_name(), "mērəkāʾ");
        assert_eq!(
            ProseAccent::MerkhaKephulah.sbl_academic_name(),
            "mērəkāʾ kəpûlâ"
        );
        assert_eq!(ProseAccent::Darga.sbl_academic_name(), "dargāʾ");
        assert_eq!(ProseAccent::Azla.sbl_academic_name(), "ʾazlāʾ");
        assert_eq!(
            ProseAccent::TelishaQetannah.sbl_academic_name(),
            "təlîšāʾ qəṭannâ"
        );
        assert_eq!(ProseAccent::Galgal.sbl_academic_name(), "galgal");
        assert_eq!(ProseAccent::Meayla.sbl_academic_name(), "məʾaylāʾ");
        assert_eq!(ProseAccent::Meteg.sbl_academic_name(), "meteg");
    }
    #[test]
    fn test_prose_disjunctive_kind() {
        assert_eq!(ProseAccent::Silluq.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Atnach.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Segolta.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Shalshelet.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::ZaqephQatan.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::ZaqephGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Revia.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Tiphcha.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Zarqa.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Pashta.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Yetiv.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Tevir.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Geresh.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Gershayim.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Pazer.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::PazerGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::TelishaGedolah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Legarmeh.kind(), Some(AccentKind::Primary));
    }

    #[test]
    fn test_prose_conjunctive_kind() {
        // Conjunctives: indices AccentKind::Primary-AccentKind::Primary, all should be None
        assert_eq!(ProseAccent::Munach.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Mahpakh.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Merkha.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::MerkhaKephulah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Darga.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Azla.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::TelishaQetannah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Galgal.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Meayla.kind(), Some(AccentKind::Secondary));
        assert_eq!(ProseAccent::Meteg.kind(), Some(AccentKind::Secondary));
    }
    #[test]
    fn test_prose_disjunctive_category() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Atnach.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Segolta.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Shalshelet.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Revia.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Tiphcha.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Zarqa.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Pashta.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Yetiv.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Tevir.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Geresh.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Gershayim.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Pazer.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::PazerGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Legarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
    }

    #[test]
    fn test_prose_conjunctive_category() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Mahpakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Merkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Darga.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Azla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Galgal.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Meayla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Meteg.category(),
            Some(AccentCategory::Conjunctive)
        );
    }
    #[test]
    fn test_prose_disjunctive_compound_type() {
        assert_eq!(ProseAccent::Silluq.compound_type(), None);
        assert_eq!(ProseAccent::Atnach.compound_type(), None);
        assert_eq!(ProseAccent::Segolta.compound_type(), None);
        assert_eq!(
            ProseAccent::Shalshelet.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(ProseAccent::ZaqephQatan.compound_type(), None);
        assert_eq!(ProseAccent::ZaqephGadol.compound_type(), None);
        assert_eq!(ProseAccent::Revia.compound_type(), None);
        assert_eq!(ProseAccent::Tiphcha.compound_type(), None);
        assert_eq!(ProseAccent::Zarqa.compound_type(), None);
        assert_eq!(ProseAccent::Pashta.compound_type(), None);
        assert_eq!(ProseAccent::Yetiv.compound_type(), None);
        assert_eq!(ProseAccent::Tevir.compound_type(), None);
        assert_eq!(ProseAccent::Geresh.compound_type(), None);
        assert_eq!(ProseAccent::Gershayim.compound_type(), None);
        assert_eq!(ProseAccent::Pazer.compound_type(), None);
        assert_eq!(ProseAccent::PazerGadol.compound_type(), None);
        assert_eq!(ProseAccent::TelishaGedolah.compound_type(), None);
        assert_eq!(
            ProseAccent::Legarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
    }

    #[test]
    fn test_prose_conjunctive_compound_type() {
        // Conjunctives: indices CompoundType::Primary-CompoundType::Primary, all should be None
        assert_eq!(ProseAccent::Munach.compound_type(), None);
        assert_eq!(ProseAccent::Mahpakh.compound_type(), None);
        assert_eq!(ProseAccent::Merkha.compound_type(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.compound_type(), None);
        assert_eq!(ProseAccent::Darga.compound_type(), None);
        assert_eq!(ProseAccent::Azla.compound_type(), None);
        assert_eq!(ProseAccent::TelishaQetannah.compound_type(), None);
        assert_eq!(ProseAccent::Galgal.compound_type(), None);
        assert_eq!(ProseAccent::Meayla.compound_type(), None);
        assert_eq!(ProseAccent::Meteg.compound_type(), None);
    }
    #[test]
    fn test_prose_disjunctive_primary_cantillation_mark() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Atnach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{591}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Segolta.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{592}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Shalshelet.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{594}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{595}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Revia.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Tiphcha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Zarqa.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ae}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Pashta.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{599}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Yetiv.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59a}',
                placement: CantillationMarkPlacement::BelowRight,
                stress_position: Some(CantillationMarkStressPosition::Prepositive)
            }
        );
        assert_eq!(
            ProseAccent::Tevir.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59b}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Geresh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59c}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Gershayim.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59e}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Pazer.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a1}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::PazerGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59f}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a0}',
                placement: CantillationMarkPlacement::AboveRight,
                stress_position: Some(CantillationMarkStressPosition::Prepositive)
            }
        );
        assert_eq!(
            ProseAccent::Legarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_primary_cantillation_mark() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Mahpakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Merkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a6}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Darga.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a7}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Azla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a9}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Galgal.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5aa}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Meayla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Meteg.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }
    #[test]
    fn test_prose_disjunctive_secondary_cantillation_mark() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Atnach.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Segolta.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::Shalshelet.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(ProseAccent::ZaqephQatan.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::ZaqephGadol.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Revia.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Tiphcha.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Zarqa.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Pashta.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Yetiv.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Tevir.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Geresh.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Gershayim.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Pazer.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::PazerGadol.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::TelishaGedolah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(
            ProseAccent::Legarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
    }

    #[test]
    fn test_prose_conjunctive_secondary_cantillation_mark() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Mahpakh.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Merkha.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::MerkhaKephulah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(ProseAccent::Darga.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Azla.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::TelishaQetannah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(ProseAccent::Galgal.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Meayla.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Meteg.secondary_cantillation_mark(), None);
    }
    #[test]
    fn test_prose_disjunctive_notes() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.notes(),
            Some(
                r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
            )
        );
        assert_eq!(
            ProseAccent::Atnach.notes(),
            Some(
                r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
            )
        );
        assert_eq!(
            ProseAccent::Segolta.notes(),
            Some(r#"Appears frequently in prose, preceding Atnach or Silluq."#)
        );
        assert_eq!(
            ProseAccent::Shalshelet.notes(),
            Some(r#"Rare accent occurring only four times in the Torah."#)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.notes(),
            Some(r#"Common disjunctive, often appearing before Zaqeph Gadol or Atnach."#)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.notes(),
            Some(r#"Less common than Zaqeph Qaton; appears after Qadma or Azla."#)
        );
        assert_eq!(
            ProseAccent::Revia.notes(),
            Some(r#"probably due to its four-note tune."#)
        );
        assert_eq!(
            ProseAccent::Tiphcha.notes(),
            Some(r#"before Atnach and Silluq"#)
        );
        assert_eq!(ProseAccent::Zarqa.notes(), Some(r#"before Segolta"#));
        assert_eq!(
            ProseAccent::Pashta.notes(),
            Some(r#"appears on the last syllable of the word, extending rightward."#)
        );
        assert_eq!(
            ProseAccent::Yetiv.notes(),
            Some(r#"occasionally substitutes for a Pashta"#)
        );
        assert_eq!(
            ProseAccent::Tevir.notes(),
            Some(r#"Often followed by Revia; indicates a melodic break."#)
        );
        assert_eq!(
            ProseAccent::Geresh.notes(),
            Some(r#"Functions as Level 4 disjunctive; often preceded by Azla."#)
        );
        assert_eq!(
            ProseAccent::Gershayim.notes(),
            Some(r#"Double Geresh; rarer variant appearing in specific syntactic contexts."#)
        );
        assert_eq!(
            ProseAccent::Pazer.notes(),
            Some(
                r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
            )
        );
        assert_eq!(
            ProseAccent::PazerGadol.notes(),
            Some(r#"Also known as Qarne Pharah; rare variant resembling horns."#)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.notes(),
            Some(r#"Level 4 disjunctive; distinct from Telisha Qetannah."#)
        );
        assert_eq!(
            ProseAccent::Legarmeh.notes(),
            Some(r#"Munach with Passeq; Before Revia"#)
        );
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_notes() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.notes(),
            Some(r#"Most common conjunctive; precedes disjunctive accents."#)
        );
        assert_eq!(
            ProseAccent::Mahpakh.notes(),
            Some(r#"Conjunctive accent; in poetry can function as Mehuppakh Legarmeh."#)
        );
        assert_eq!(
            ProseAccent::Merkha.notes(),
            Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.notes(),
            Some(r#"Merkha duplex"#)
        );
        assert_eq!(
            ProseAccent::Darga.notes(),
            Some(r#"Conjunctive accent; synonymous with Qadma in some traditions."#)
        );
        assert_eq!(ProseAccent::Azla.notes(), Some(r#"When Geresh: Qadma"#));
        assert_eq!(
            ProseAccent::TelishaQetannah.notes(),
            Some(r#"Short form of Telisha; conjunctive function."#)
        );
        assert_eq!(
            ProseAccent::Galgal.notes(),
            Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
        );
        assert_eq!(
            ProseAccent::Meayla.notes(),
            Some(
                r#"Name given to a Tiphcha, when in the same word as Atnach or Silluq.
    This accent is historically the same glyph as Tipcha (U+0596) but functions as a secondary conjunctive 
    in specific syntactic contexts.
    Different traditions use different names: 
        Tipcha (common), Tarḥa, or Me'ayla/Mayela (rare scholarly names)."#,
            )
        );
        assert_eq!(
            ProseAccent::Meteg.notes(),
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
    fn test_prose_disjunctive_strength() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(ProseAccent::Atnach.relative_strength(), Some(2));
        assert_eq!(ProseAccent::Segolta.relative_strength(), Some(3));
        assert_eq!(ProseAccent::Shalshelet.relative_strength(), Some(4));
        assert_eq!(ProseAccent::ZaqephQatan.relative_strength(), Some(5));
        assert_eq!(ProseAccent::ZaqephGadol.relative_strength(), Some(6));
        assert_eq!(ProseAccent::Revia.relative_strength(), Some(7));
        assert_eq!(ProseAccent::Tiphcha.relative_strength(), Some(8));
        assert_eq!(ProseAccent::Zarqa.relative_strength(), Some(9));
        assert_eq!(ProseAccent::Pashta.relative_strength(), Some(10));
        assert_eq!(ProseAccent::Yetiv.relative_strength(), Some(11));
        assert_eq!(ProseAccent::Tevir.relative_strength(), Some(12));
        assert_eq!(ProseAccent::Geresh.relative_strength(), Some(13));
        assert_eq!(ProseAccent::Gershayim.relative_strength(), Some(14));
        assert_eq!(ProseAccent::Pazer.relative_strength(), Some(15));
        assert_eq!(ProseAccent::PazerGadol.relative_strength(), Some(16));
        assert_eq!(ProseAccent::TelishaGedolah.relative_strength(), Some(17));
        assert_eq!(ProseAccent::Legarmeh.relative_strength(), Some(18));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_strength() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.relative_strength(), None);
        assert_eq!(ProseAccent::Mahpakh.relative_strength(), None);
        assert_eq!(ProseAccent::Merkha.relative_strength(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.relative_strength(), None);
        assert_eq!(ProseAccent::Darga.relative_strength(), None);
        assert_eq!(ProseAccent::Azla.relative_strength(), None);
        assert_eq!(ProseAccent::TelishaQetannah.relative_strength(), None);
        assert_eq!(ProseAccent::Galgal.relative_strength(), None);
        assert_eq!(ProseAccent::Meayla.relative_strength(), None);
        assert_eq!(ProseAccent::Meteg.relative_strength(), None);
    }
    #[test]
    fn test_prose_disjunctive_group_level() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Atnach.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Segolta.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(
            ProseAccent::Shalshelet.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(ProseAccent::Revia.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tiphcha.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::Zarqa.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Pashta.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Yetiv.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tevir.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Geresh.group_level(), Some(GroupLevel::Tier4));
        assert_eq!(
            ProseAccent::Gershayim.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(ProseAccent::Pazer.group_level(), Some(GroupLevel::Tier4));
        assert_eq!(
            ProseAccent::PazerGadol.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(ProseAccent::Legarmeh.group_level(), Some(GroupLevel::Tier4));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_group_level() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.group_level(), None);
        assert_eq!(ProseAccent::Mahpakh.group_level(), None);
        assert_eq!(ProseAccent::Merkha.group_level(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.group_level(), None);
        assert_eq!(ProseAccent::Darga.group_level(), None);
        assert_eq!(ProseAccent::Azla.group_level(), None);
        assert_eq!(ProseAccent::TelishaQetannah.group_level(), None);
        assert_eq!(ProseAccent::Galgal.group_level(), None);
        assert_eq!(ProseAccent::Meayla.group_level(), None);
        assert_eq!(ProseAccent::Meteg.group_level(), None);
    }
#[test]
    fn test_prose_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Atnach.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Segolta.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Shalshelet.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::ZaqephQatan.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::ZaqephGadol.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Revia.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Tiphcha.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Zarqa.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Pashta.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Yetiv.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Tevir.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Geresh.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Gershayim.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Pazer.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::PazerGadol.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::TelishaGedolah.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Legarmeh.cantillation_symbol(), "// TODO");
    }

    #[test]
    fn test_prose_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Mahpakh.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Merkha.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::MerkhaKephulah.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Darga.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Azla.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::TelishaQetannah.cantillation_symbol(),"// TODO");
        assert_eq!(ProseAccent::Galgal.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Meayla.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Meteg.cantillation_symbol(), "// TODO");
    }
        #[test]
    fn test_prose_disjunctive_alternate_names() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.alternate_names(), None);
        assert_eq!(ProseAccent::Atnach.alternate_names(), None);
        assert_eq!(ProseAccent::Segolta.alternate_names(), None);
        assert_eq!(ProseAccent::Shalshelet.alternate_names(), None);
        assert_eq!(ProseAccent::ZaqephQatan.alternate_names(), None);
        assert_eq!(ProseAccent::ZaqephGadol.alternate_names(), None);
        assert_eq!(ProseAccent::Revia.alternate_names(), None);
        assert_eq!(ProseAccent::Tiphcha.alternate_names(), None);
        assert_eq!(ProseAccent::Zarqa.alternate_names(), None);
        assert_eq!(ProseAccent::Pashta.alternate_names(), None);
        assert_eq!(ProseAccent::Yetiv.alternate_names(), None);
        assert_eq!(ProseAccent::Tevir.alternate_names(), None);
        assert_eq!(
            ProseAccent::Geresh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "טֶרֶס",
                hebrew_concept: "a boundary or border",
                english_name: "Teres",
                sbl_academic: "ṭeres",
            })
        );
        assert_eq!(ProseAccent::Gershayim.alternate_names(), None);
        assert_eq!(ProseAccent::Pazer.alternate_names(), None);
        assert_eq!(
            ProseAccent::PazerGadol.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "קַרְנֵי פָרָה",
                hebrew_concept: "horns of a cow",
                english_name: "Qarne Pharah",
                sbl_academic: "qarnê pārâ",
            })
        );
        assert_eq!(ProseAccent::TelishaGedolah.alternate_names(), None);
        assert_eq!(ProseAccent::Legarmeh.alternate_names(), None);
    }

    // TODO
    #[test]
    fn test_prose_conjunctive_alternate_names() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.alternate_names(), None);
        assert_eq!(
            ProseAccent::Mahpakh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(ProseAccent::Merkha.alternate_names(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.alternate_names(), None);
        assert_eq!(ProseAccent::Darga.alternate_names(), None);
        assert_eq!(ProseAccent::Azla.alternate_names(), None);
        assert_eq!(ProseAccent::TelishaQetannah.alternate_names(), None);
        assert_eq!(
            ProseAccent::Galgal.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(ProseAccent::Meayla.alternate_names(), None);
        assert_eq!(ProseAccent::Meteg.alternate_names(), None);
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
}
