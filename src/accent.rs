// Standard library

// External crates

// Local modules / crate‑internal
use crate::accent_data::*;

pub trait Accent: Copy + Sized {
    /// Return the *static* metadata for this concrete accent.
    fn details(self) -> &'static AccentInfo;

    /// Convenience wrappers.
    #[inline]
    fn category(self) -> AccentCategory {
        self.details().category
    }
    #[inline]
    fn accent_type(self) -> AccentType {
        self.details().accent_type
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
        }
    }
    fn category(self) -> AccentCategory {
        match self {
            HebrewAccent::Prose(p) => p.details().category,
            HebrewAccent::Poetry(p) => p.details().category,
        }
    }
    fn accent_type(self) -> AccentType {
        match self {
            HebrewAccent::Prose(p) => p.details().accent_type,
            HebrewAccent::Poetry(p) => p.details().accent_type,
        }
    }
    fn relative_strength(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
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

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
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
/// All variants of the Hebrew Prose Accents
/// 18 Disjunctives and 11 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum ProseAccent {
    // Disjunctives
    #[default]
    Silluq,
    Atnach,
    Segolta,
    Shalshelet,
    ZaqephQaton,
    ZaqephGadol,
    Revia,
    Tiphcha,
    Zarqa,
    Pashta,
    Yetiv,
    Tevir,
    Geresh,
    Gershayim,
    Pazer,
    PazerGadol,
    TelishaGedolah,
    Legarmeh,
    // Conjunctives
    Munach,
    Mahpakh,
    Merkha,
    MerkhaKephulah,
    Darga,
    Azla,
    TelishaQetannah,
    Galgal,
    Mayela,
    Meteg,
    Maqqeph,
}

impl ProseAccent {
    pub const COUNT: usize = 29;
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
    // Disjunctives
    #[default]
    Silluq,
    OlehWeYored,
    Atnach,
    ReviaGadol,
    ReviaMugrash,
    ShalsheletGadol,
    Tsinnor,
    ReviaQaton,
    Dechi,
    Pazer,
    MehuppakhLegarmeh,
    AzlaLegarmeh,
    // Conjunctives
    Munach,
    Merkha,
    Illuy,
    Tarkha,
    Galgal,
    Mehuppakh,
    Azla,
    ShalsheletQetannah,
    TsinnoritMerkha,
    TsinnoritMahpakh,
    Meteg,
    Maqqeph,
}

impl PoetryAccent {
    pub const COUNT: usize = 24;
    #[inline]
    pub fn relative_strength(self) -> u8 {
        // Discriminants start at 0; we want 1‑based relative_strengths.
        BHS_POETRY_RANK_MAP[self as usize]
    }
}
/// (non)technical details of a Hebrew Accent like category, type, UTF8 Unicode code-point(s etc.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AccentInfo {
    /// Primary identifiers – always present
    pub english_name: &'static str,
    pub hebrew_name: &'static str,
    /// Meaning of the hebrew name he accent
    pub meaning: &'static str,
    /// Optional alternate identifiers
    pub alternates: Option<Alternates>,
    /// Indicates the accent type (Primary, Secundary)
    pub accent_type: AccentType,
    /// Optional alternate identifiers
    pub category: AccentCategory,
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
    pub english_name: &'static str,
    pub hebrew_name: &'static str,
    pub meaning: &'static str,
}
/// Lists one or two UTF-8 code-point(s) from which the accent is constructed
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CodePoints {
    pub primary: &'static Utf8CodePointInfo,
    pub secondary: Option<&'static Utf8CodePointInfo>,
}

/// Details on a specific UTF8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Utf8CodePointInfo {
    /// UTF8 code-point, e.g. U+0591
    pub code_point: &'static str,
    /// hex value of the UTF8 code-point
    pub hex_value: &'static str,
    /// name of the UTF8 code-point
    pub name: &'static str,
    /// symbol of the UTF8 code-point
    pub symbol: &'static str,
    /// position of the code-point in relation to the consonant
    pub position: CodePointPosition,
    /// array containing information of various Jewish traditions
    pub traditions: &'static [Tradition],
}
/// Names according one of four Hebrew Traditions
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub enum Tradition {
    Ashkenazi {
        hebrew: &'static str,
        name: &'static str,
    },
    Sephardi {
        hebrew: &'static str,
        name: &'static str,
    },
    Italian {
        hebrew: &'static str,
        name: &'static str,
    },
    Yemenite {
        hebrew: &'static str,
        name: &'static str,
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
    #[default]
    /// Accent is located above the consonant
    Above,
    /// Accent is located under the consonant
    Under,
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
}

/// WordStress, indicating the location of the accent in relation to the consonant
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[non_exhaustive]
pub enum WordStress {
    #[default]
    /// The accent is located above the stressed syllable
    ImPositive,
    /// The accent is NOT located above the stressed syllable, but at the very end of the word
    PostPositive,
    /// Accent is NOT located above the stressed syllable, but at the very beginning of the word
    PrePositive,
    ///
    NotApplicable,
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
    /*23 */ 23, // Maqqeph
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
        assert_eq!(ha.category(), AccentCategory::Disjunctive);
        assert_eq!(ha.accent_type(), AccentType::Primary);
        assert_eq!(ha.details().meaning, "great (long) detached");

        // Poetry variant
        let ha: HebrewAccent = PoetryAccent::MehuppakhLegarmeh.into();
        assert_eq!(ha.details().english_name, "Mehuppakh Legarmeh");
        assert_eq!(ha.category(), AccentCategory::Disjunctive);
        assert_eq!(ha.accent_type(), AccentType::Primary);
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
