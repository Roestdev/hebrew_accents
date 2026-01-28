// Standard library

// External crates

// Local modules / crate‑internal
use crate::accent_data::*;

/// Gets accent information
pub trait Accent: Copy + Sized {
    /// Return the *static* metadata for this concrete accent.
    fn details(self) -> &'static AccentInfo;
    // Convenience wrappers.
    /// accenttrypr
    #[inline]
    fn accent_type(self) -> Option<AccentType> {
        self.details().accent_type
    }
    /// accenttrypr
    #[inline]
    fn category(self) -> Option<AccentCategory> {
        self.details().category
    }
    /// accenttrypr
    #[inline]
    fn word_stress(self) -> Option<WordStress> {
        self.details().word_stress
    }
    /// indicates the relative_strength of a selected accent
    fn relative_strength(self) -> u8;
}

impl Accent for HebrewAccent {
    #[inline]
    fn details(self) -> &'static AccentInfo {
        match self {
            HebrewAccent::Prose(p) => p.details(),
            HebrewAccent::Poetry(p) => p.details(),
            HebrewAccent::Pseudo(p) => p.details(),
        }
    }
    fn accent_type(self) -> Option<AccentType> {
        match self {
            HebrewAccent::Prose(p) => p.details().accent_type,
            HebrewAccent::Poetry(p) => p.details().accent_type,
            HebrewAccent::Pseudo(p) => p.details().accent_type,
        }
    }
    fn category(self) -> Option<AccentCategory> {
        match self {
            HebrewAccent::Prose(p) => p.details().category,
            HebrewAccent::Poetry(p) => p.details().category,
            HebrewAccent::Pseudo(p) => p.details().category,
        }
    }
    fn word_stress(self) -> Option<WordStress> {
        match self {
            HebrewAccent::Prose(p) => p.details().word_stress,
            HebrewAccent::Poetry(p) => p.details().word_stress,
            HebrewAccent::Pseudo(p) => p.details().word_stress,
        }
    }
    fn relative_strength(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }
}

impl Accent for ProseAccent {
    fn details(self) -> &'static AccentInfo {
        PROSE_ACCENT_TABLE[self as usize]
    }
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

impl Accent for PoetryAccent {
    fn details(self) -> &'static AccentInfo {
        POETRY_ACCENT_TABLE[self as usize]
    }
    fn relative_strength(self) -> u8 {
        BHS_POETRY_RANK_MAP[self as usize]
    }
}

impl Accent for PseudoAccent {
    fn details(self) -> &'static AccentInfo {
        PSEUDO_ACCENT_TABLE[self as usize]
    }
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HebrewAccent {
    /// TODO
    Prose(ProseAccent),
    /// TODO
    Poetry(PoetryAccent),
    /// TODO
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
    /// Disjunctive prose accent Silluq
    Silluq,
    /// Disjunctive prose accent Atnach
    Atnach,
    /// Disjunctive prose accent Segolta
    Segolta,
    /// Disjunctive prose accent Shalshelet
    Shalshelet,
    /// Disjunctive prose accent Zaqeph Qaton
    ZaqephQaton,
    /// Disjunctive prose accent Zaqeph Gadol
    ZaqephGadol,
    /// Disjunctive prose accent Revia
    Revia,
    /// Disjunctive prose accent Tiphcha,
    Tiphcha,
    /// Disjunctive prose accent Zarqa
    Zarqa,
    /// Disjunctive prose accent Pashta
    Pashta,
    /// Disjunctive prose accent Yetiv
    Yetiv,
    /// Disjunctive prose accent Tevir
    Tevir,
    /// Disjunctive prose accent Geresh
    Geresh,
    /// Disjunctive prose accent Gershayim
    Gershayim,
    /// Disjunctive prose accent Pazer
    Pazer,
    /// Disjunctive prose accent Pazer Gadol
    PazerGadol,
    /// Disjunctive prose accent Telisha Gedolah
    TelishaGedolah,
    /// Disjunctive prose accent Legarmeh
    Legarmeh,
    /// Conjunctive prose accent Munach
    Munach,
    /// Conjunctive prose accent Mahpakh
    Mahpakh,
    /// Conjunctive prose accent Merkha
    Merkha,
    /// Conjunctive prose accent Merkha Kephulah
    MerkhaKephulah,
    /// Conjunctive prose accent Darga
    Darga,
    /// Conjunctive prose accent Azla
    Azla,
    /// Conjunctive prose accent Telisha Qetannah
    TelishaQetannah,
    /// Conjunctive prose accent Galgal
    Galgal,
    /// Conjunctive prose accent Mayela
    Mayela,
    /// Conjunctive prose accent Meteg
    Meteg,
}

impl ProseAccent {
    /// The total number of prose accents
    pub const COUNT: usize = 28;
    ///  TODO
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
    /// Disjunctive prose accent Silluq
    Silluq,
    /// Disjunctive prose accent Oleh We Yored
    OlehWeYored,
    /// Disjunctive prose accent Atnach
    Atnach,
    /// Disjunctive prose accent Revia Gadol
    ReviaGadol,
    /// Disjunctive prose accent     Revia Mugrash,
    ReviaMugrash,
    /// Disjunctive prose accent ShalsheletGadol
    ShalsheletGadol,
    /// Disjunctive prose accent Tsinnor
    Tsinnor,
    /// Disjunctive prose accent Revia Qaton
    ReviaQaton,
    /// Disjunctive prose accent     Dechi,
    Dechi,
    /// Disjunctive prose accent Pazer
    Pazer,
    /// Disjunctive prose accent MehuppakhLegarmeh
    MehuppakhLegarmeh,
    /// Disjunctive prose accent AzlaLegarmeh
    AzlaLegarmeh,
    /// Conjunctive prose accent Munach
    Munach,
    /// Conjunctive prose accent Merkha
    Merkha,
    /// Conjunctive prose accent Illuy,
    Illuy,
    /// Conjunctive prose accent Tarkha
    Tarkha,
    /// Conjunctive prose accent Galgal
    Galgal,
    /// Conjunctive prose accent Mehuppakh
    Mehuppakh,
    /// Conjunctive prose accent Azla
    Azla,
    /// Conjunctive prose accent Shalshelet Qetannah
    ShalsheletQetannah,
    /// Conjunctive prose accent Tsinnorit Merkha
    TsinnoritMerkha,
    /// Conjunctive prose accent Tsinnorit Mahpakh
    TsinnoritMahpakh,
    /// Conjunctive prose accent Meteg
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

/// Hebrew marks that are related to the Hebrew accents
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum PseudoAccent {
    #[default]
    /// TODO
    SophPasuq,
    /// TODO
    Maqqeph,
    /// TODO
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
/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AccentInfo {
    /// Primary identifiers – always present
    pub english_name: &'static str,
    /// Hebrew name of the accent – always present
    pub hebrew_name: &'static str,
    /// The meaning of the hebrew the accent – always present
    pub meaning: &'static str,
    /// Optional alternate identifiers
    pub alternates: Option<Alternates>,
    /// Indicates the accent type (Primary, Secundary)
    pub accent_type: Option<AccentType>,
    /// Optional alternate identifiers
    pub category: Option<AccentCategory>,
    /// Indicates if the accent is on the stressed syllable
    pub word_stress: Option<WordStress>,
    /// Unicode code‑point data.
    pub code_points: CodePoints,
    /// Free‑form comment (may be omitted)
    pub comment: Option<&'static str>,
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
    /// Primary UTF8 code point
    pub primary: &'static Utf8CodePointInfo,
    /// Secondary UTF8 code point, if applicable
    pub secondary: Option<&'static Utf8CodePointInfo>,
}

/// Details on a specific UTF8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Utf8CodePointInfo {
    /// UTF8 code-point, e.g. U+0591
    pub code_point: &'static str,
    /// The hex value of the UTF8 code-point
    pub hex_value: &'static str,
    /// The name of the UTF8 code-point as mentioned in the UTF8 code tables
    pub name: &'static str,
    /// The symbol of the UTF8 code-point
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
    /// Naming of the accent according Italian  tradition
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
    /// used for Maqqeph
    None,
}

/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum CodePointPosition {
    /// UTF8 code point is located above the consonant
    Above,
    /// UTF8 code point is located above the consonant
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
    /// UTF8 code point is located in between two words
    InBetween,
    /// UTF8 code point is located under the consonant
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

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variant, extend this
/// array accordingly – the `static_assertions` check below will remind you.
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::COUNT] = [
    // ---- Disjunctives ----------------------------------------------------
    /* 0 */
    1, // Silluq
    /* 1 */ 2, // OlehWeYored
    /* 2 */ 3, // Atnach
    /* 3 */ 4, // ReviaGadol
    /* 4 */ 5, // ReviaMugrash
    /* 5 */ 6, // ShalsheletGadol
    /* 6 */ 7, // Tsinnor
    /* 7 */ 8, // ReviaQaton
    /* 8 */ 9, // Dechi
    /* 9 */ 10, // Pazer
    /*10 */ 11, // MehuppakhLegarmeh
    /*11 */ 12, // AzlaLegarmeh
    // ---- Conjunctives ----------------------------------------------------
    /*12 */
    13, // Munach
    /*13 */ 14, // Merkha
    /*14 */ 15, // Illuy
    /*15 */ 16, // Tarkha
    /*16 */ 17, // Galgal
    /*17 */ 18, // Mehuppakh
    /*18 */ 19, // Azla
    /*19 */ 20, // ShalsheletQetannah
    /*20 */ 21, // TsinnoritMerkha
    /*21 */ 21, // TsinnoritMahpakh
    /*22 */ 22, // Meteg
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that asserts the three most common accessor methods on any
    /// `Accent` implementation give consistent results.
    fn assert_accent_consistency<A: Accent>(accent: A, expected_meaning: &str) {
        // The static metadata must match the expected string.
        assert_eq!(accent.details().meaning, expected_meaning);

        // Category and type are just forwarded to `details()`.
        assert_eq!(accent.category(), accent.details().category);
        assert_eq!(accent.accent_type(), accent.details().accent_type);
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
