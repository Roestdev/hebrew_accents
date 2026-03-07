//! Main entry point for Hebrew Accent information

// Standard library
// N/A

// External crates
// N/A

// Crate‑internal (local modules)
use crate::accent_data::{
    BHS_POETRY_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE,
};

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HebrewAccent {
    /// Prose variant
    Prose(ProseAccent),
    /// Poetry variant
    Poetry(PoetryAccent),
    /// Pseudo variant
    Pseudo(PseudoAccent),
}

impl From<ProseAccent> for HebrewAccent {
    fn from(a: ProseAccent) -> Self {
        HebrewAccent::Prose(a)
    }
}
impl From<PoetryAccent> for HebrewAccent {
    fn from(a: PoetryAccent) -> Self {
        HebrewAccent::Poetry(a)
    }
}
impl From<PseudoAccent> for HebrewAccent {
    fn from(a: PseudoAccent) -> Self {
        HebrewAccent::Pseudo(a)
    }
}
/// All variants of the Hebrew Prose Accents
/// 18 Disjunctives and 11 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
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
    /// Secondary conjunctive prose accent Mayela
    Mayela,
    /// Secondary conjunctive prose accent Meteg
    Meteg,
}

impl ProseAccent {
    /// The total number of prose accents
    pub const COUNT: usize = 28;
    /// Indication of how an strong an accent is relativly speaking
    ///
    /// The stronger, the longer the pause/break when reading
    /// The strongest accent has a relative strength of 1
    ///
    /// Note: For now all Hebrew Accents have this property for now
    ///    It is only valid for disjunctive accents.
    #[inline]
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

/// All variants of the Hebrew Poetry Accents
/// 12 Disjunctives and 12 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum PoetryAccent {
    #[default]
    /// Primary disjunctive poetry accent Silluq
    Silluq,
    /// Primary disjunctive poetry accent Oleh We Yored
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
    pub const COUNT: usize = 23;
    #[inline]
    /// Indicates a level of importancy
    pub fn relative_strength(self) -> u8 {
        // Discriminants start at 0; we want 1‑based relative_strengths.
        BHS_POETRY_RANK_MAP[self as usize]
    }
}

/// Hebrew marks that are close related to the Hebrew accents
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum PseudoAccent {
    #[default]
    /// Marks the end of a sentence (like the period), but not always
    SophPasuq,
    /// Joins multiple words (like a hyphen), mostly leaving 1 accent for the joined words
    Maqqeph,
    /// Can be part of a Hebrew Accent, but not as an individual accent
    Paseq,
}

impl PseudoAccent {
    /// Total count of all pseudo accents
    pub const COUNT: usize = 3;
    #[inline]
    /// Indicates a level of importancy
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

/// Used for retrieving information
pub trait Accent: Copy + Sized {
    /// indicates the relative_strength of a selected accent (1 is the strongest)
    fn relative_strength(self) -> u8;
    /// indicates the relative_strength of a selected accent (1 is the strongest)
    fn hierarchical_group(self) -> Option<HierarchicalGroup>;
    /// Return the *static* metadata for this concrete accent.
    fn details(self) -> &'static AccentInformation;

    /* ---------- Convenience helpers (default impls) ---------- */
    /// English name of the Hebrew Accent
    #[inline]
    fn english_name(self) -> &'static str {
        self.details().english_name
    }
    /// Hebrew name of the Hebrew Accent
    #[inline]
    fn hebrew_name(self) -> &'static str {
        self.details().hebrew_name
    }
    /// meaning of the Hebrew name
    #[inline]
    fn meaning(self) -> &'static str {
        self.details().meaning
    }
    /// Hebrew Accent type
    #[inline]
    fn accent_type(self) -> Option<AccentType> {
        self.details().additional.map(|add| add.accent_type)
    }
    /// category of the Hebrew Accent
    #[inline]
    fn category(self) -> Option<AccentCategory> {
        self.details().additional.map(|add| add.category)
    }
    /// word-stress of the Hebrew Accent
    #[inline]
    fn word_stress(self) -> Option<WordStress> {
        self.details().additional.and_then(|add| add.word_stress)
    }

    /// number of UTF-8 code points of the Hebrew Accent
    fn code_points(self) -> u8;
}

impl Accent for HebrewAccent {
    #[inline]
    fn details(self) -> &'static AccentInformation {
        match self {
            HebrewAccent::Prose(p) => p.details(),
            HebrewAccent::Poetry(p) => p.details(),
            HebrewAccent::Pseudo(p) => p.details(),
        }
    }

    #[inline]
    fn relative_strength(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }

    #[inline]
    fn hierarchical_group(self) -> Option<HierarchicalGroup> {
        match self {
            HebrewAccent::Prose(p) => p.hierarchical_group(),
            HebrewAccent::Poetry(p) => p.hierarchical_group(),
            HebrewAccent::Pseudo(_) => None,
        }
    }

    #[inline]
    fn code_points(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.code_points(),
            HebrewAccent::Poetry(p) => p.code_points(),
            HebrewAccent::Pseudo(p) => p.code_points(),
        }
    }
}

impl Accent for ProseAccent {
    #[inline]
    fn details(self) -> &'static AccentInformation {
        PROSE_ACCENT_TABLE[self as usize]
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn hierarchical_group(self) -> Option<HierarchicalGroup> {
        prose_hierarchical_group(self)
    }
    #[inline]
    fn code_points(self) -> u8 {
        if self.details().code_points.secondary.is_none() {
            1
        } else {
            2
        }
    }
}

impl Accent for PoetryAccent {
    #[inline]
    fn details(self) -> &'static AccentInformation {
        POETRY_ACCENT_TABLE[self as usize]
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        BHS_POETRY_RANK_MAP[self as usize]
    }
    #[inline]
    fn hierarchical_group(self) -> Option<HierarchicalGroup> {
        poetry_hierarchical_group(self)
    }
    #[inline]
    fn code_points(self) -> u8 {
        if self.details().code_points.secondary.is_none() {
            1
        } else {
            2
        }
    }
}

impl Accent for PseudoAccent {
    #[inline]
    fn details(self) -> &'static AccentInformation {
        PSEUDO_ACCENT_TABLE[self as usize]
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn hierarchical_group(self) -> Option<HierarchicalGroup> {
        None
    }

    #[inline]
    fn code_points(self) -> u8 {
        if self.details().code_points.secondary.is_none() {
            1
        } else {
            2
        }
    }
}

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AccentInformation {
    /// Hebrew name of the accent
    pub hebrew_name: &'static str,
    /// The meaning of the Hebrew the accent
    pub meaning: &'static str,
    /// English name, transliterated
    pub english_name: &'static str,
    /// Unicode code‑point data.
    pub code_points: CodePoints,
    /// Free‑form comment
    pub comment: Option<&'static str>,
    /// Additional accent information (for PseudeoAccents, additional=None)
    pub additional: Option<Additional>,
}

/// Additional information for the accents used in Prose and Poetry
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Additional {
    /// Indicates the accent type (Primary, Secundary)
    pub accent_type: AccentType,
    /// Optional alternate identifiers
    pub category: AccentCategory,
    /// Indicates if the accent is on the stressed syllable
    pub word_stress: Option<WordStress>,
    /// TODO
    pub hierarchical_group: Option<HierarchicalGroup>,
    /// Optional alternate identifiers
    pub alternates: Option<Alternates>,
}

/// Optional alternate representations for an accent.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Alternates {
    /// Transliterated English name for the hebrew name
    pub english_name: &'static str,
    /// Hebrew name of the accent
    pub hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub meaning: &'static str,
}
/// Lists one or two UTF-8 code-point(s) from which the accent is constructed
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CodePoints {
    /// Primary UTF-8 code point
    pub primary: &'static Utf8CodePointInfo,
    /// Secondary UTF-8 code point, if applicable
    pub secondary: Option<&'static Utf8CodePointInfo>,
}

/// Details on a specific UTF-8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Utf8CodePointInfo {
    /// UTF-8 code-point, e.g. U+0591
    pub code_point: &'static str,
    /// The hex value of the UTF-8 code-point
    pub hex_value: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub name: &'static str,
    /// The symbol of the UTF-8 code-point
    pub symbol: &'static str,
    /// The position of the code-point in relation to the consonant
    pub position: CodePointPosition,
    /// An array containing information of various Jewish traditions
    pub traditions: &'static [Tradition],
}
/// Names according one of four Hebrew Traditions
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum Tradition {
    /// Naming of the accent according Ashkenazi tradition
    Ashkenazi {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Sephardi tradition
    Sephardi {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Italian tradition
    Italian {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Yemenite tradition
    Yemenite {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
}

/// Hebrew Accent category (either Conjunctive or Disjunctive)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum AccentCategory {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that seperate words
    Disjunctive,
}

/// Hebrew Accent types (Primary, secondary, None)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum AccentType {
    #[default]
    /// Indicates that the Accent is primary
    Primary,
    /// used for Meayla and Meteg
    Secondary,
    /// used for Pseudo Accents
    None,
}

/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum CodePointPosition {
    /// UTF-8 code point is located above the consonant
    Above,
    /// UTF-8 code point is located above the consonant
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
    /// UTF-8 code point is located in between two words
    InBetween,
    /// UTF-8 code point is located under the consonant
    #[default]
    Under,
}

/// WordStress, indicating the location of the accent in relation to the consonant
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub enum WordStress {
    #[default]
    /// The accent is located above the stressed syllable
    ImPositive,
    /// The accent is NOT located above the stressed syllable, but at the very end of the word
    PostPositive,
    /// Accent is NOT located above the stressed syllable, but at the very beginning of the word
    PrePositive,
}

/// Indication of where the DISJUNCIVE accents are part o
/// Group classification is according the book from  Basics of Hebrew Accents by Mark D. Futato, Sr.
/// which is based on
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HierarchicalGroup {
    /// TODO
    ProseGroup1,
    /// TODO
    ProseGroup2,
    /// TODO
    ProseGroup3,
    /// TODO
    ProseGroup4,
    /// TODO
    PoetryGroup1,
    /// TODO
    PoetryGroup2,
    /// TODO
    PoetryGroup3,
}

pub(crate) fn prose_hierarchical_group(accent: ProseAccent) -> Option<HierarchicalGroup> {
    match accent {
        ProseAccent::Silluq | ProseAccent::Atnach => Some(HierarchicalGroup::ProseGroup1),

        ProseAccent::Segolta
        | ProseAccent::Shalshelet
        | ProseAccent::ZaqephQatan
        | ProseAccent::ZaqephGadol
        | ProseAccent::Tiphcha => Some(HierarchicalGroup::ProseGroup2),

        ProseAccent::Revia
        | ProseAccent::Zarqa
        | ProseAccent::Pashta
        | ProseAccent::Tevir
        | ProseAccent::Yetiv => Some(HierarchicalGroup::ProseGroup3),

        ProseAccent::Geresh
        | ProseAccent::Gershayim
        | ProseAccent::Pazer
        | ProseAccent::PazerGadol
        | ProseAccent::TelishaGedolah
        | ProseAccent::Legarmeh => Some(HierarchicalGroup::ProseGroup4),
        _ => None, // all conjuntive ProseAccents
    }
}
pub(crate) fn poetry_hierarchical_group(accent: PoetryAccent) -> Option<HierarchicalGroup> {
    match accent {
        PoetryAccent::Silluq | PoetryAccent::OlehWeYored | PoetryAccent::Atnach => {
            Some(HierarchicalGroup::PoetryGroup1)
        }
        PoetryAccent::ReviaGadol
        | PoetryAccent::ReviaMugrash
        | PoetryAccent::ShalsheletGadol
        | PoetryAccent::ReviaQaton
        | PoetryAccent::Tsinnor
        | PoetryAccent::Dechi => Some(HierarchicalGroup::PoetryGroup2),

        PoetryAccent::Pazer | PoetryAccent::MehuppakhLegarmeh | PoetryAccent::AzlaLegarmeh => {
            Some(HierarchicalGroup::PoetryGroup3)
        }
        _ => None, // all conjuntive PoetryAccents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that asserts the three most common accessor methods on any
    /// `Accent` implementation give consistent results.
    fn assert_accent_consistency<A: Accent>(accent: A, expected_meaning: &str) {
        // The static metadata must match the expected string.
        assert_eq!(accent.details().meaning, expected_meaning);

        // Category and type are just forwarded to `details()`.
        assert_eq!(
            accent.category(),
            Some(accent.details().additional.unwrap().category)
        );
        assert_eq!(
            accent.accent_type(),
            Some(accent.details().additional.unwrap().accent_type)
        );
    }

    /// ----------------------------------------------------------------
    /// Simple sanity checks for a handful of *prose* accents
    /// ----------------------------------------------------------------
    #[test]
    fn prose_accent_basic_properties() {
        // Silluq – the very first disjunctive
        let silluq = ProseAccent::Silluq;
        assert_eq!(silluq.relative_strength(), 1);
        assert_accent_consistency(silluq, "close, cessation");

        // Atnach – second disjunctive
        let atnach = ProseAccent::Atnach;
        assert_eq!(atnach.relative_strength(), 2);
        assert_accent_consistency(atnach, "a causing to rest");

        // Galgal – a conjunctive (wheel, circle)
        let galgal = ProseAccent::Galgal;
        assert_eq!(galgal.relative_strength(), 26);
        assert_accent_consistency(galgal, "wheel, circle");
    }

    /// ----------------------------------------------------------------
    /// Parallel checks for *poetry* accents
    /// ----------------------------------------------------------------
    #[test]
    fn poetry_accent_basic_properties() {
        // Shalshelet – a disjunctive in poetry
        let shalshelet = PoetryAccent::ShalsheletGadol;
        assert_eq!(shalshelet.relative_strength(), 6);
        assert_accent_consistency(shalshelet, "large chain or link");

        // Illuy – a conjunctive (elevation / raising)
        let illuy = PoetryAccent::Illuy;
        assert_eq!(illuy.relative_strength(), 15);
        assert_accent_consistency(illuy, "elevation or raising");
    }

    /// ----------------------------------------------------------------
    /// The high‑level `HebrewAccent` wrapper forwards correctly
    /// ----------------------------------------------------------------
    #[test]
    fn hebrew_accent_dispatches_to_concrete_impls() {
        // Prose variant
        let ha: HebrewAccent = ProseAccent::TelishaGedolah.into();
        assert_eq!(ha.details().english_name, "Telisha Gedolah");
        assert_eq!(ha.category(), Some(AccentCategory::Disjunctive));
        assert_eq!(ha.accent_type(), Some(AccentType::Primary));
        assert_eq!(ha.details().meaning, "great (long) detached");

        // Poetry variant
        let ha: HebrewAccent = PoetryAccent::MehuppakhLegarmeh.into();
        assert_eq!(ha.details().english_name, "Mehuppakh Legarmeh");
        assert_eq!(ha.category(), Some(AccentCategory::Disjunctive));
        assert_eq!(ha.accent_type(), Some(AccentType::Primary));
        assert_eq!(ha.details().meaning, "reversed to its own");
    }

    #[test]
    fn silluq() {
        let pa = ProseAccent::Silluq;
        let pa_silluq_ord = pa.relative_strength();
        assert_eq!(1, pa_silluq_ord);
    }

    #[test]
    fn test_prose_accent_details() {
        let pa = ProseAccent::Galgal;
        assert_eq!("wheel, circle", pa.details().meaning);
    }
    #[test]
    fn test_poetry_accent_details() {
        let pa = PoetryAccent::Galgal;
        assert_eq!("wheel, circle", pa.details().meaning);
    }
    #[test]
    fn test_hebrew_accent_details() {
        let ha = HebrewAccent::Prose(ProseAccent::Galgal);
        assert_eq!("wheel, circle", ha.details().meaning);
    }
}

mod pseudo {
    #[test]
    fn check() {}
}
