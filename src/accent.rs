
use crate::accent_data::*;

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
}

/// All variants of the Hebrew Prose Accents
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum ProseAccent {
    /// Disjunctives (18x)
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
    /// Conjunctives (11x)
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

/// All variants of the Hebrew Poetry Accents
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum PoetryAccent {
    /// Disjunctives (12)
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
    /// Conjunctives (12)
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

/// (non)technical details of a Hebrew Accent like category, type, UTF8 Unicode code-point(s etc.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct AccentInfo {
    /// Primary identifiers – always present.
    pub english_name: &'static str,
    pub hebrew_name: &'static str,
    pub meaning: &'static str,
    /// Optional alternate identifiers.
    pub alternates: Option<Alternates>,
    /// Indicates the accent type (Primary, Secundary)
    pub accent_type: AccentType,
    /// Optional alternate identifiers.
    pub category: AccentCategory,
    /// Unicode code‑point data.
    pub code_points: CodePoints,
    /// Free‑form comment (may be omitted).
    pub comment: Option<&'static str>,
}

/// Optional alternate representations for an accent.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
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
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentCategory {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that seperate words
    Disjunctive,
}

/// Hebrew Accent types (Primary, secondary, None)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentType {
    #[default]
    Primary,
    /// used for Meayla and Meteg
    Secondary,
    /// used for Maqqeph
    None,
}

/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum CodePointPosition {
    #[default]
    /// Accent is located above the consonant
    Above,
    /// Accent is located above and after the consonant
    AbovePostPositive,
    /// Accent is located above and before the consonant
    AbovePrePositive,
    /// Accent is located end of the consonant (used for Paseq, Soph Pasuq and Maqqeph)
    End,
    /// Accent is located under the consonant
    Under,
    /// Accent is located under and before the consonant
    UnderPrePositive,
}

pub trait Accent: Copy + Sized {
    /// Return the *static* metadata for this concrete accent.
    fn info(self) -> &'static AccentInfo;

    // Convenience wrappers that expose the fields you used most.
    // #[inline] fn rank(self)        -> u8                 { self.details().rank }
    #[inline]
    fn category(self) -> AccentCategory {
        self.info().category
    }
    #[inline]
    fn accent_type(self) -> AccentType {
        self.info().accent_type
    }
    //#[inline] fn traditions(self)  -> &'static [Tradition] {
    //   self.info().code_points.primary.traditions
    //}
}

impl Accent for HebrewAccent {
    #[inline]
    fn info(self) -> &'static AccentInfo {
        match self {
            HebrewAccent::Prose(p) => p.info(),
            HebrewAccent::Poetry(p) => p.info(),
        }
    }
    fn category(self) -> AccentCategory {
        match self {
            HebrewAccent::Prose(p) => p.info().category,
            HebrewAccent::Poetry(p) => p.info().category,
        }
    }
    fn accent_type(self) -> AccentType {
        match self {
            HebrewAccent::Prose(p) => p.info().accent_type,
            HebrewAccent::Poetry(p) => p.info().accent_type,
        }
    }
}

impl Accent for ProseAccent {
    fn info(self) -> &'static AccentInfo {
        PROSE_ACCENT_TABLE[self as usize] // panics if the index is out of range
    }
}

impl Accent for PoetryAccent {
    fn info(self) -> &'static AccentInfo {
        POETRY_ACCENT_TABLE[self as usize] // panics if the index is out of range
    }
}

impl ProseAccent {
    pub const COUNT: usize = 29;
    #[inline]
    pub fn rank(self) -> u8 {
        // Discriminants start at 0; we want 1‑based ranks.
        self as u8 + 1
    }
}


/// Mapping from the enum discriminant (as `usize`) to the logical rank.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variant, extend this
/// array accordingly – the `static_assertions` check below will remind you.
pub const BHS_POETRY_RANK_MAP:[u8; PoetryAccent::COUNT]= [
    // ---- Disjunctives ----------------------------------------------------
    /* 0 */  1,  // Silluq
    /* 1 */  2,  // OlehWeYored
    /* 2 */  3,  // Atnach
    /* 3 */  4,  // ReviaGadol
    /* 4 */  5,  // ReviaMugrash
    /* 5 */  6,  // ShalsheletGadol
    /* 6 */  7,  // Tsinnor
    /* 7 */  8,  // ReviaQaton
    /* 8 */  9,  // Dechi
    /* 9 */ 10,  // Pazer
    /*10 */ 11,  // MehuppakhLegarmeh
    /*11 */ 12,  // AzlaLegarmeh
    // ---- Conjunctives ----------------------------------------------------
    /*12 */ 13,  // Munach
    /*13 */ 14,  // Merkha
    /*14 */ 15,  // Illuy
    /*15 */ 16,  // Tarkha
    /*16 */ 17,  // Galgal
    /*17 */ 18,  // Mehuppakh
    /*18 */ 19,  // Azla
    /*19 */ 20,  // ShalsheletQetannah
    /*20 */ 21,  // TsinnoritMerkha
    /*21 */ 21,  // TsinnoritMahpakh
    /*22 */ 22,  // Meteg
    /*23 */ 23,  // Maqqeph
];

impl PoetryAccent {
    pub const COUNT: usize = 24;
    #[inline]
    pub fn rank(self) -> u8 {
        // Discriminants start at 0; we want 1‑based ranks.
        BHS_POETRY_RANK_MAP[self as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper that asserts the three most common accessor methods on any
    /// `Accent` implementation give consistent results.
    fn assert_accent_consistency<A: Accent>(accent: A, expected_meaning: &str) {
        // 1️⃣  The static metadata must match the expected string.
        assert_eq!(accent.info().meaning, expected_meaning);

        // 2️⃣  Category and type are just forwarded to `info()`.
        assert_eq!(accent.category(), accent.info().category);
        assert_eq!(accent.accent_type(), accent.info().accent_type);
    }

    /// ----------------------------------------------------------------
    /// 1️⃣  Simple sanity checks for a handful of *prose* accents
    /// ----------------------------------------------------------------
    #[test]
    fn prose_accent_basic_properties() {
        // Silluq – the very first disjunctive
        let silluq = ProseAccent::Silluq;
        assert_eq!(silluq.rank(), 1);
        assert_accent_consistency(silluq, "close, cessation");

        // Atnach – second disjunctive
        let atnach = ProseAccent::Atnach;
        assert_eq!(atnach.rank(), 2);
        assert_accent_consistency(atnach, "a causing to rest");

        // Galgal – a conjunctive (wheel, circle)
        let galgal = ProseAccent::Galgal;
        assert_eq!(galgal.rank(), 26);
        assert_accent_consistency(galgal, "wheel, circle");
    }

    /// ----------------------------------------------------------------
    /// 2️⃣  Parallel checks for *poetry* accents
    /// ----------------------------------------------------------------
    #[test]
    fn poetry_accent_basic_properties() {
        // Shalshelet – a disjunctive in poetry
        let shalshelet = PoetryAccent::ShalsheletGadol;
        assert_eq!(shalshelet.rank(), 6);
        assert_accent_consistency(shalshelet, "large chain or link");

        // Illuy – a conjunctive (elevation / raising)
        let illuy = PoetryAccent::Illuy;
        assert_eq!(illuy.rank(), 15);
        assert_accent_consistency(illuy, "elevation or raising");
    }

    /// ----------------------------------------------------------------
    /// 3️⃣  The high‑level `HebrewAccent` wrapper forwards correctly
    /// ----------------------------------------------------------------
    #[test]
    fn hebrew_accent_dispatches_to_concrete_impls() {
        // Prose variant
        let ha_prose = HebrewAccent::Prose(ProseAccent::TelishaGedolah);
        assert_eq!(ha_prose.info().english_name, "Telisha Gedolah");
        assert_eq!(ha_prose.category(), AccentCategory::Disjunctive);
        assert_eq!(ha_prose.accent_type(), AccentType::Primary);
        assert_eq!(ha_prose.info().meaning, "great (long) detached");

        // Poetry variant
        let ha_poetry = HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh);
        assert_eq!(ha_poetry.info().english_name, "Mehuppakh Legarmeh");
        assert_eq!(ha_poetry.category(), AccentCategory::Disjunctive);
        assert_eq!(ha_poetry.accent_type(), AccentType::Primary);
        assert_eq!(ha_poetry.info().meaning, "reversed to its own");
    }

    /// ----------------------------------------------------------------
    /// 4️⃣  Edge‑case sanity check – ensure the tables are sized correctly
    /// ----------------------------------------------------------------
    #[test]
    fn tables_have_the_expected_length() {
        // The `COUNT` constants should match the length of the static tables.
        // If they diverge, the `info()` implementation would panic at runtime.
        assert_eq!(ProseAccent::COUNT, PROSE_ACCENT_TABLE.len());
        assert_eq!(PoetryAccent::COUNT, POETRY_ACCENT_TABLE.len());
    }

    #[test]
    fn silluq() {
        let pa = ProseAccent::Silluq;
        let pa_silluq_ord = pa.rank();
        assert_eq!(1, pa_silluq_ord);
        // let _details = a.details();
    }
    #[test]
    fn no_test_just_print_details() {
        println!("\n{:#?}", ProseAccent::Silluq.info());
        println!("\n{:#?}", ProseAccent::Atnach.info());
        println!("\n{:#?}", ProseAccent::Segolta.info());
        println!("\n{:#?}", ProseAccent::Shalshelet.info());
        println!("\n{:#?}", ProseAccent::ZaqephQaton.info());
        println!("\n{:#?}", ProseAccent::ZaqephGadol.info());
        println!("\n{:#?}", ProseAccent::Revia.info());
        println!("\n{:#?}", ProseAccent::Tiphcha.info());
        println!("\n{:#?}", ProseAccent::Zarqa.info());
        println!("\n{:#?}", ProseAccent::Pashta.info());
        println!("\n{:#?}", ProseAccent::Yetiv.info());
        println!("\n{:#?}", ProseAccent::Tevir.info());
        println!("\n{:#?}", ProseAccent::Geresh.info());
        println!("\n{:#?}", ProseAccent::Gershayim.info());
        println!("\n{:#?}", ProseAccent::Pazer.info());
        println!("\n{:#?}", ProseAccent::PazerGadol.info());
        println!("\n{:#?}", ProseAccent::TelishaGedolah.info());
        println!("\n{:#?}", ProseAccent::Legarmeh.info());
        // Conjunctives
        println!("\n{:#?}", ProseAccent::Munach.info());
        println!("\n{:#?}", ProseAccent::Mahpakh.info());
        println!("\n{:#?}", ProseAccent::Merkha.info());
        println!("\n{:#?}", ProseAccent::MerkhaKephulah.info());
        println!("\n{:#?}", ProseAccent::Darga.info());
        println!("\n{:#?}", ProseAccent::Azla.info());
        println!("\n{:#?}", ProseAccent::TelishaQetannah.info());
        println!("\n{:#?}", ProseAccent::Galgal.info());
        println!("\n{:#?}", ProseAccent::Mayela.info());
    }

    #[test]
    fn test_prose_accent_details() {
        let pa = ProseAccent::Galgal;
        assert_eq!("wheel, circle", pa.info().meaning);
    }
    #[test]
    fn test_poetry_accent_details() {
        let pa = PoetryAccent::Galgal;
        assert_eq!("wheel, circle", pa.info().meaning);
    }
    #[test]
    fn test_hebrew_accent_details() {
        let ha = HebrewAccent::Prose(ProseAccent::Galgal);
        assert_eq!("wheel, circle", ha.info().meaning);
    }
}
