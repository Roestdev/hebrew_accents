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
    /// Indication of how an strong an accent is (relativly speaking)
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
    /// Indicates a level of importancy
    #[inline]
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
    /// Indicates a level of importancy
    #[inline]
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
    //#[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn hierarchical_group(self) -> Option<HierarchicalGroup> {
        None
    }

    #[inline]
    fn code_points(self) -> u8 {
        1 // always one
          // if self.details().code_points.secondary.is_none() {
          //     1
          // } else {
          //     2
          // }
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
mod pseudo_accent_tests {
    use super::*;

    // ========================================================================
    // 1. Basic Trait Method Tests
    // ========================================================================

    /// Test that `details()` returns valid AccentInformation for all variants
    #[test]
    fn test_pseudo_accent_details_returns_valid_info() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            let info = variant.details();

            // Verify the info is not null/empty
            assert!(
                !info.english_name.is_empty(),
                "Variant {:?} has empty english_name",
                variant
            );
            assert!(
                !info.hebrew_name.is_empty(),
                "Variant {:?} has empty hebrew_name",
                variant
            );
            assert!(
                !info.meaning.is_empty(),
                "Variant {:?} has empty meaning",
                variant
            );
        }
    }

    /// Test that `english_name()` returns correct values
    #[test]
    fn test_pseudo_accent_english_name() {
        assert_eq!(PseudoAccent::SophPasuq.english_name(), "Soph Pasuq");
        assert_eq!(PseudoAccent::Maqqeph.english_name(), "Maqqeph");
        assert_eq!(PseudoAccent::Paseq.english_name(), "Paseq");
    }

    /// Test that `hebrew_name()` returns correct values
    #[test]
    fn test_pseudo_accent_hebrew_name() {
        //assert_eq!(PseudoAccent::SophPasuq.hebrew_name(), "סוֹף פָּסוּק");
        //assert_eq!(PseudoAccent::Maqqeph.hebrew_name(), "מַקֵּף");
        //assert_eq!(PseudoAccent::Paseq.hebrew_name(), "פָּסֵק");
    }

    /// Test that `meaning()` returns correct values
    #[test]
    fn test_pseudo_accent_meaning() {
        assert_eq!(PseudoAccent::SophPasuq.meaning(), "end of verse");
        assert_eq!(PseudoAccent::Maqqeph.meaning(), "binder");
        assert_eq!(
            PseudoAccent::Paseq.meaning(),
            "to pause, to stop or to interrupt"
        );
    }

    /// Test that relative_strength is consistent across calls
    #[test]
    fn test_pseudo_accent_relative_strength_consistency() {
        let variant = PseudoAccent::SophPasuq;

        let strength1 = variant.relative_strength();
        let strength2 = variant.relative_strength();
        let strength3 = variant.relative_strength();

        assert_eq!(strength1, strength2, "First two calls should match");
        assert_eq!(strength2, strength3, "All three calls should match");
    }

    // ========================================================================
    // 3. hierarchical_group Tests
    // ========================================================================

    /// Test that hierarchical_group returns None for all PseudoAccent variants
    #[test]
    fn test_pseudo_accent_hierarchical_group_is_none() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            assert_eq!(
                variant.hierarchical_group(),
                None,
                "PseudoAccent {:?} should have None hierarchical_group",
                variant
            );
        }
    }

    // ========================================================================
    // 4. code_points Tests
    // ========================================================================

    /// Test that code_points returns 1 for all PseudoAccent variants
    #[test]
    fn test_pseudo_accent_code_points_is_one() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            assert_eq!(
                variant.code_points(),
                1,
                "PseudoAccent {:?} should have 1 code point",
                variant
            );
        }
    }

    /// Test that code_points is consistent
    #[test]
    fn test_pseudo_accent_code_points_consistency() {
        let variant = PseudoAccent::Maqqeph;

        assert_eq!(variant.code_points(), variant.code_points());
        assert_eq!(variant.code_points(), 1);
    }

    // ========================================================================
    // 5. Optional Field Tests (accent_type, category, word_stress)
    // ========================================================================

    /// Test that accent_type returns None for PseudoAccent (since additional is None)
    #[test]
    fn test_pseudo_accent_accent_type_is_none() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            assert_eq!(
                variant.accent_type(),
                None,
                "PseudoAccent {:?} should have None accent_type",
                variant
            );
        }
    }

    /// Test that category returns None for PseudoAccent
    #[test]
    fn test_pseudo_accent_category_is_none() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            assert_eq!(
                variant.category(),
                None,
                "PseudoAccent {:?} should have None category",
                variant
            );
        }
    }

    /// Test that word_stress returns None for PseudoAccent
    #[test]
    fn test_pseudo_accent_word_stress_is_none() {
        let variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in variants {
            assert_eq!(
                variant.word_stress(),
                None,
                "PseudoAccent {:?} should have None word_stress",
                variant
            );
        }
    }

    // ========================================================================
    // 6. Integration Tests with HebrewAccent
    // ========================================================================

    /// Test that PseudoAccent through HebrewAccent still works correctly
    #[test]
    fn test_pseudo_accent_via_hebrew_accent_trait() {
        let pseudo = PseudoAccent::SophPasuq;
        let hebrew: HebrewAccent = pseudo.into();

        // Now test the trait methods through HebrewAccent
        assert_eq!(hebrew.english_name(), "Soph Pasuq");
        //assert_eq!(hebrew.hebrew_name(), "סוֹף פָּסוּק");
        assert_eq!(hebrew.meaning(), "end of verse");
        assert_eq!(hebrew.hierarchical_group(), None);
        assert_eq!(hebrew.code_points(), 1);
    }

    /// Test that all PseudoAccent variants work through HebrewAccent wrapper
    #[test]
    fn test_all_pseudo_variants_via_hebrew_accent() {
        let variants = vec![
            (PseudoAccent::SophPasuq, "Soph Pasuq"),
            (PseudoAccent::Maqqeph, "Maqqeph"),
            (PseudoAccent::Paseq, "Paseq"),
        ];

        for (pseudo, expected_name) in variants {
            let hebrew: HebrewAccent = pseudo.into();

            assert_eq!(hebrew.english_name(), expected_name);
            assert!(matches!(hebrew, HebrewAccent::Pseudo(_)));
        }
    }

    // ========================================================================
    // 7. Edge Case & Property Tests
    // ========================================================================

    /// Test that trait methods are idempotent (same input = same output)
    #[test]
    fn test_pseudo_accent_trait_methods_idempotent() {
        let variant = PseudoAccent::Paseq;

        // All methods should return the same value on repeated calls
        assert_eq!(variant.details(), variant.details());
        assert_eq!(variant.english_name(), variant.english_name());
        assert_eq!(variant.hebrew_name(), variant.hebrew_name());
        assert_eq!(variant.meaning(), variant.meaning());
        assert_eq!(variant.code_points(), variant.code_points());
        assert_eq!(variant.hierarchical_group(), variant.hierarchical_group());
    }

    /// Test that details() returns static reference (no allocation)
    #[test]
    fn test_pseudo_accent_details_returns_static_reference() {
        let info1 = PseudoAccent::SophPasuq.details();
        let info2 = PseudoAccent::SophPasuq.details();

        // They should be the same static reference
        assert_eq!(info1 as *const _, info2 as *const _);
    }

    /// Test that all convenience helpers work together
    #[test]
    fn test_pseudo_accent_all_helpers_consistent() {
        let variant = PseudoAccent::Maqqeph;
        let info = variant.details();

        // Verify helpers match the underlying details
        assert_eq!(variant.english_name(), info.english_name);
        assert_eq!(variant.hebrew_name(), info.hebrew_name);
        assert_eq!(variant.meaning(), info.meaning);
    }

    // ========================================================================
    // 8. Stress Test
    // ========================================================================

    /// Rapidly call trait methods to ensure no memory issues
    #[test]
    fn test_pseudo_accent_trait_stress() {
        let iterations = 1000;

        for _ in 0..iterations {
            let _ = PseudoAccent::SophPasuq.details();
            let _ = PseudoAccent::Maqqeph.english_name();
            let _ = PseudoAccent::Paseq.code_points();
            let _ = PseudoAccent::SophPasuq.hierarchical_group();
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    // Assuming these are exported from your accent_data module
    // Adjust imports based on your actual module structure
    use crate::accent::{PoetryAccent, ProseAccent, PseudoAccent};
    use crate::accent_data::{
        PROSE_ACCENT_TABLE,
        //PROSE_ACCENT_TABLE, POETRY_ACCENT_TABLE, PSEUDO_ACCENT_TABLE,
    };
    //use crate::accent::{ProseAccent, PoetryAccent, PseudoAccent, AccentInformation};

    // ========================================================================
    // 1. Table Consistency & Round-Trip Tests
    // ========================================================================

    /// Validates that every item in the PROSE table can be converted to HebrewAccent
    /// and back (conceptually) without losing identity.
    #[test]
    fn test_prose_table_round_trip_conversion() {
        for &info_ptr in PROSE_ACCENT_TABLE.iter() {
            // We need to map the info back to the enum variant.
            // Since we don't have a direct "Info -> Enum" function yet,
            // we verify that the English name matches a known variant.
            // In a real scenario, you might have a `from_english_name` helper.

            // For now, we test the structural integrity:
            // 1. Get the enum variant associated with this info (mocked via index or helper)
            // 2. Convert to HebrewAccent
            // 3. Verify it matches the expected variant

            // NOTE: This test assumes you can iterate over the enum variants.
            // If you have a `variants()` method on the enums, use that.
            // Otherwise, we test the *existence* of the conversion for known variants.

            // Let's test a specific known mapping: Silluq
            if info_ptr.english_name == "Silluq" {
                let prose_variant = ProseAccent::Silluq;
                let hebrew_accent: HebrewAccent = prose_variant.into();

                match hebrew_accent {
                    HebrewAccent::Prose(p) => {
                        assert_eq!(p, ProseAccent::Silluq);
                        // Verify the info matches
                        assert_eq!(p as usize, 0); // Assuming Silluq is index 0
                    }
                    _ => panic!("Silluq should be Prose variant"),
                }
            }
        }
    }

    /// Ensures that converting a specific enum variant produces a HebrewAccent
    /// that, when matched, yields the correct inner variant.
    #[test]
    fn test_all_prose_variants_via_table_index() {
        // Iterate through the table and verify the conversion logic holds
        // This acts as a sanity check that the table order matches the enum definition order
        let _expected_names = [
            "Silluq",
            "Atnach",
            "Segolta",
            "Shalshelet",
            "Zaqeph Qaton",
            "Zaqeph Gadol",
            "Revia",
            "Tiphcha",
            "Zarqa",
            "Pashta",
            "Yetiv",
            "Tevir",
            "Geresh",
            "Gershayim",
            "Pazer",
            "Pazer Gadol",
            "Telisha Gedolah",
            "Legarmeh",
            "Munach",
            "Mahpakh",
            "Merkha",
            "Merkha Kephulah",
            "Darga",
            "Azla",
            "Telisha Qetannah",
            "Galgal",
            "Mayela",
            "Meteg",
        ];

        for (i, &info_ptr) in PROSE_ACCENT_TABLE.iter().enumerate() {
            // We can't easily reconstruct the enum from the pointer without a reverse map,
            // so we verify the *data* consistency instead.
            // However, we can test that the *conversion* works for the known variant at index i.

            // This part requires a helper to get the variant from index,
            // or we just trust the unit tests covered the individual variants.
            // Instead, let's test the *integration* with the table data:
            assert!(
                !info_ptr.english_name.is_empty(),
                "Table entry {} has empty name",
                i
            );

            // Verify the conversion logic is sound by checking a sample
            if i == 0 {
                let variant = ProseAccent::Silluq;
                let accent: HebrewAccent = variant.into();
                assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Silluq)));
            }
        }
    }

    // ========================================================================
    // 2. Cross-Type Discrimination Tests
    // ========================================================================

    /// Ensures that accents with the same name in different categories (Prose vs Poetry)
    /// remain distinct after conversion.
    #[test]
    fn test_cross_category_name_collision_distinction() {
        // Silluq exists in both Prose and Poetry
        let prose_silluq: HebrewAccent = ProseAccent::Silluq.into();
        let poetry_silluq: HebrewAccent = PoetryAccent::Silluq.into();

        assert_ne!(
            prose_silluq, poetry_silluq,
            "Prose and Poetry Silluq should be distinct variants"
        );

        // Verify they are in the correct outer variants
        assert!(matches!(prose_silluq, HebrewAccent::Prose(_)));
        assert!(matches!(poetry_silluq, HebrewAccent::Poetry(_)));
    }

    /// Tests that Pseudo accents are distinct from Prose/Poetry even if names overlap
    /// (though in this dataset, they likely don't overlap in names).
    #[test]
    fn test_pseudo_distinct_from_prose_and_poetry() {
        let soph_pasuq: HebrewAccent = PseudoAccent::SophPasuq.into();
        let silluq: HebrewAccent = ProseAccent::Silluq.into();

        // Soph Pasuq and Silluq are semantically similar (end of verse) but structurally distinct
        assert_ne!(soph_pasuq, silluq);
        assert!(matches!(soph_pasuq, HebrewAccent::Pseudo(_)));
    }

    // ========================================================================
    // 3. Data Flow & Usage Scenario Tests
    // ========================================================================

    /// Simulates a real-world scenario: Processing a list of mixed accent types.
    #[test]
    fn test_mixed_accent_processing_pipeline() {
        // Create a mixed list of accents
        let raw_accents: Vec<HebrewAccent> = vec![
            ProseAccent::Silluq.into(),
            PoetryAccent::Atnach.into(),
            PseudoAccent::Maqqeph.into(),
            ProseAccent::Munach.into(),
            PoetryAccent::ReviaGadol.into(),
        ];

        // Process them (e.g., count by category)
        let mut prose_count = 0;
        let mut poetry_count = 0;
        let mut pseudo_count = 0;

        for accent in raw_accents {
            match accent {
                HebrewAccent::Prose(_) => prose_count += 1,
                HebrewAccent::Poetry(_) => poetry_count += 1,
                HebrewAccent::Pseudo(_) => pseudo_count += 1,
            }
        }

        assert_eq!(prose_count, 2);
        assert_eq!(poetry_count, 2);
        assert_eq!(pseudo_count, 1);
    }

    /// Tests that the conversion preserves the ability to access underlying data
    /// (simulating a scenario where you need to look up info after conversion).
    #[test]
    fn test_conversion_preserves_access_to_underlying_data() {
        let prose_variant = ProseAccent::Shalshelet;
        let accent: HebrewAccent = prose_variant.into();

        // Extract the inner value
        let extracted = match accent {
            HebrewAccent::Prose(p) => p,
            _ => panic!("Expected Prose variant"),
        };

        // Verify we can still access the original variant
        assert_eq!(extracted, ProseAccent::Shalshelet);

        // In a real app, you might now look up the info:
        assert_eq!(extracted.meaning(), "chain or link");
    }

    // ========================================================================
    // 4. Edge Cases & Error Scenarios
    // ========================================================================

    /// Tests that the conversion is idempotent (converting twice yields same result)
    #[test]
    fn test_conversion_idempotency() {
        let original = ProseAccent::Meteg;
        let first_convert: HebrewAccent = original.into();

        // We can't convert HebrewAccent back to ProseAccent directly without a TryFrom
        // But we can verify the state is stable
        let second_convert: HebrewAccent = match first_convert {
            HebrewAccent::Prose(p) => p.into(),
            _ => panic!("Should be Prose"),
        };

        assert_eq!(first_convert, second_convert);
    }

    /// Tests that the conversion works correctly with `Option` and `Result` wrappers
    #[test]
    fn test_conversion_with_option_wrappers() {
        let opt_prose: Option<ProseAccent> = Some(ProseAccent::Atnach);
        let opt_hebrew: Option<HebrewAccent> = opt_prose.map(|a| a.into());

        assert!(opt_hebrew.is_some());
        assert!(matches!(opt_hebrew.unwrap(), HebrewAccent::Prose(_)));
    }

    #[test]
    fn test_conversion_with_result_wrappers() {
        let res_prose: Result<ProseAccent, ()> = Ok(ProseAccent::Zarqa);
        let res_hebrew: Result<HebrewAccent, ()> = res_prose.map(|a| a.into());

        assert!(res_hebrew.is_ok());
        assert!(!matches!(res_hebrew.unwrap(), HebrewAccent::Poetry(_))); // Wait, Zarqa is Prose!
                                                                          // Correction:
        assert!(matches!(res_hebrew.unwrap(), HebrewAccent::Prose(_)));
    }

    // ========================================================================
    // 5. Performance/Stress Test (Optional)
    // ========================================================================

    /// Rapidly converts all variants to ensure no panics or memory issues
    #[test]
    fn test_rapid_conversion_stress() {
        let iterations = 1000;

        for _ in 0..iterations {
            let _p: HebrewAccent = ProseAccent::Silluq.into();
            let _po: HebrewAccent = PoetryAccent::Atnach.into();
            let _ps: HebrewAccent = PseudoAccent::SophPasuq.into();
        }

        // If we reach here, no panics occurred
    }
}

#[cfg(test)]
mod test_to_and_from_trait {
    use super::*;

    // ========================================================================
    // ProseAccent -> HebrewAccent Tests
    // ========================================================================

    #[test]
    fn test_from_prose_silluq() {
        let prose = ProseAccent::Silluq;
        let accent: HebrewAccent = prose.into();

        assert_eq!(accent, HebrewAccent::Prose(ProseAccent::Silluq));
        match accent {
            HebrewAccent::Prose(p) => assert_eq!(p, ProseAccent::Silluq),
            _ => panic!("Expected Prose variant"),
        }
    }

    #[test]
    fn test_from_prose_atnach() {
        let prose = ProseAccent::Atnach;
        let accent: HebrewAccent = prose.into();

        assert_eq!(accent, HebrewAccent::Prose(ProseAccent::Atnach));
    }

    #[test]
    fn test_from_prose_all_variants() {
        // Test all ProseAccent variants to ensure complete coverage
        let prose_variants = vec![
            ProseAccent::Silluq,
            ProseAccent::Atnach,
            ProseAccent::Segolta,
            ProseAccent::Shalshelet,
            ProseAccent::ZaqephQatan,
            ProseAccent::ZaqephGadol,
            ProseAccent::Revia,
            ProseAccent::Tiphcha,
            ProseAccent::Zarqa,
            ProseAccent::Pashta,
            ProseAccent::Yetiv,
            ProseAccent::Tevir,
            ProseAccent::Geresh,
            ProseAccent::Gershayim,
            ProseAccent::Pazer,
            ProseAccent::PazerGadol,
            ProseAccent::TelishaGedolah,
            ProseAccent::Legarmeh,
            ProseAccent::Munach,
            ProseAccent::Mahpakh,
            ProseAccent::Merkha,
            ProseAccent::MerkhaKephulah,
            ProseAccent::Darga,
            ProseAccent::Azla,
            ProseAccent::TelishaQetannah,
            ProseAccent::Galgal,
            ProseAccent::Mayela,
            ProseAccent::Meteg,
        ];

        for variant in prose_variants {
            let accent: HebrewAccent = variant.into();
            assert!(matches!(accent, HebrewAccent::Prose(_)));

            // Verify the inner value is preserved
            if let HebrewAccent::Prose(inner) = accent {
                assert_eq!(inner, variant);
            } else {
                panic!("Variant {:?} did not convert to Prose variant", variant);
            }
        }
    }

    // ========================================================================
    // PoetryAccent -> HebrewAccent Tests
    // ========================================================================

    #[test]
    fn test_from_poetry_silluq() {
        let poetry = PoetryAccent::Silluq;
        let accent: HebrewAccent = poetry.into();

        assert_eq!(accent, HebrewAccent::Poetry(PoetryAccent::Silluq));
    }

    #[test]
    fn test_from_poetry_atnach() {
        let poetry = PoetryAccent::Atnach;
        let accent: HebrewAccent = poetry.into();

        assert_eq!(accent, HebrewAccent::Poetry(PoetryAccent::Atnach));
    }

    #[test]
    fn test_from_poetry_all_variants() {
        // Test all PoetryAccent variants to ensure complete coverage
        let poetry_variants = vec![
            PoetryAccent::Silluq,
            PoetryAccent::OlehWeYored,
            PoetryAccent::Atnach,
            PoetryAccent::ReviaGadol,
            PoetryAccent::ReviaMugrash,
            PoetryAccent::ShalsheletGadol,
            PoetryAccent::Tsinnor,
            PoetryAccent::ReviaQaton,
            PoetryAccent::Dechi,
            PoetryAccent::Pazer,
            PoetryAccent::MehuppakhLegarmeh,
            PoetryAccent::AzlaLegarmeh,
            PoetryAccent::Munach,
            PoetryAccent::Merkha,
            PoetryAccent::Illuy,
            PoetryAccent::Tarcha,
            PoetryAccent::Galgal,
            PoetryAccent::Mehuppakh,
            PoetryAccent::Azla,
            PoetryAccent::ShalsheletQetannah,
            PoetryAccent::TsinnoritMerkha,
            PoetryAccent::TsinnoritMahpakh,
            PoetryAccent::Meteg,
        ];

        for variant in poetry_variants {
            let accent: HebrewAccent = variant.into();
            assert!(matches!(accent, HebrewAccent::Poetry(_)));

            // Verify the inner value is preserved
            if let HebrewAccent::Poetry(inner) = accent {
                assert_eq!(inner, variant);
            } else {
                panic!("Variant {:?} did not convert to Poetry variant", variant);
            }
        }
    }

    // ========================================================================
    // PseudoAccent -> HebrewAccent Tests
    // ========================================================================

    #[test]
    fn test_from_pseudo_soph_pasuq() {
        let pseudo = PseudoAccent::SophPasuq;
        let accent: HebrewAccent = pseudo.into();

        assert_eq!(accent, HebrewAccent::Pseudo(PseudoAccent::SophPasuq));
    }

    #[test]
    fn test_from_pseudo_maqqeph() {
        let pseudo = PseudoAccent::Maqqeph;
        let accent: HebrewAccent = pseudo.into();

        assert_eq!(accent, HebrewAccent::Pseudo(PseudoAccent::Maqqeph));
    }

    #[test]
    fn test_from_pseudo_paseq() {
        let pseudo = PseudoAccent::Paseq;
        let accent: HebrewAccent = pseudo.into();

        assert_eq!(accent, HebrewAccent::Pseudo(PseudoAccent::Paseq));
    }

    #[test]
    fn test_from_pseudo_all_variants() {
        // Test all PseudoAccent variants
        let pseudo_variants = vec![
            PseudoAccent::SophPasuq,
            PseudoAccent::Maqqeph,
            PseudoAccent::Paseq,
        ];

        for variant in pseudo_variants {
            let accent: HebrewAccent = variant.into();
            assert!(matches!(accent, HebrewAccent::Pseudo(_)));

            // Verify the inner value is preserved
            if let HebrewAccent::Pseudo(inner) = accent {
                assert_eq!(inner, variant);
            } else {
                panic!("Variant {:?} did not convert to Pseudo variant", variant);
            }
        }
    }

    // ========================================================================
    // Integration & Edge Case Tests
    // ========================================================================

    #[test]
    fn test_from_trait_preserves_equality() {
        // Ensure that converting and comparing works correctly
        let prose = ProseAccent::Silluq;
        let accent: HebrewAccent = prose.into();

        assert_eq!(accent, HebrewAccent::Prose(prose));
        assert_ne!(accent, HebrewAccent::Poetry(PoetryAccent::Silluq));
        assert_ne!(accent, HebrewAccent::Pseudo(PseudoAccent::SophPasuq));
    }

    #[test]
    fn test_from_trait_different_variants_not_equal() {
        // Different accent types should not be equal even if they share similar names
        let prose_silluq: HebrewAccent = ProseAccent::Silluq.into();
        let poetry_silluq: HebrewAccent = PoetryAccent::Silluq.into();

        assert_ne!(prose_silluq, poetry_silluq);
    }

    #[test]
    fn test_from_trait_explicit_type_annotation() {
        // Test that explicit type annotation works
        let prose: ProseAccent = ProseAccent::Munach;
        let accent: HebrewAccent = From::from(prose);

        assert!(matches!(accent, HebrewAccent::Prose(_)));
    }

    #[test]
    fn test_from_trait_infer_type() {
        // Test that type inference works
        let accent: HebrewAccent = ProseAccent::Meteg.into();

        assert!(matches!(accent, HebrewAccent::Prose(_)));
    }

    #[test]
    fn test_from_trait_chained_conversions() {
        // Test that conversions can be chained in expressions
        let result: HebrewAccent = ProseAccent::Atnach.into();
        let result2: HebrewAccent = PoetryAccent::Atnach.into();
        let result3: HebrewAccent = PseudoAccent::SophPasuq.into();

        assert!(matches!(result, HebrewAccent::Prose(_)));
        assert!(matches!(result2, HebrewAccent::Poetry(_)));
        assert!(matches!(result3, HebrewAccent::Pseudo(_)));
    }

    // ========================================================================
    // Property-Based Tests (if using proptest or similar)
    // ========================================================================

    #[test]
    fn test_from_trait_idempotent_for_same_variant() {
        // Converting the same variant multiple times should produce equal results
        let variant = ProseAccent::Revia;
        let accent1: HebrewAccent = variant.into();
        let accent2: HebrewAccent = variant.into();

        assert_eq!(accent1, accent2);
    }
}

#[cfg(test)]
mod test_relative_strength {
    use crate::*;
    // relative_strength
    #[test]
    fn testing_hebrew_prose_accent_relative_strengths() {
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Zarqa).relative_strength(),
            9
        );
    }
    #[test]
    fn testing_prose_accent_relative_strengths() {
        // Disjunctives
        assert_eq!(ProseAccent::Silluq.relative_strength(), 1);
        // assert_eq!(ProseAccent::Atnach.relative_strength(), 2);
        // assert_eq!(ProseAccent::Segolta.relative_strength(), 3);
        // assert_eq!(ProseAccent::Shalshelet.relative_strength(), 4);
        // assert_eq!(ProseAccent::ZaqephQatan.relative_strength(), 5);
        // assert_eq!(ProseAccent::ZaqephGadol.relative_strength(), 6);
        // assert_eq!(ProseAccent::Revia.relative_strength(), 7);
        // assert_eq!(ProseAccent::Tiphcha.relative_strength(), 8);
        // assert_eq!(ProseAccent::Zarqa.relative_strength(), 9);
        // assert_eq!(ProseAccent::Pashta.relative_strength(), 10);
        // assert_eq!(ProseAccent::Yetiv.relative_strength(), 11);
        // assert_eq!(ProseAccent::Tevir.relative_strength(), 12);
        // assert_eq!(ProseAccent::Geresh.relative_strength(), 13);
        // assert_eq!(ProseAccent::Gershayim.relative_strength(), 14);
        // assert_eq!(ProseAccent::Pazer.relative_strength(), 15);
        // assert_eq!(ProseAccent::PazerGadol.relative_strength(), 16);
        // assert_eq!(ProseAccent::TelishaGedolah.relative_strength(), 17);
        // assert_eq!(ProseAccent::Legarmeh.relative_strength(), 18);
        // Conjunctives
        // assert_eq!(ProseAccent::Munach.relative_strength(), 19);
        // assert_eq!(ProseAccent::Mahpakh.relative_strength(), 20);
        // assert_eq!(ProseAccent::Merkha.relative_strength(), 21);
        // assert_eq!(ProseAccent::MerkhaKephulah.relative_strength(), 22);
        // assert_eq!(ProseAccent::Darga.relative_strength(), 23);
        // assert_eq!(ProseAccent::Azla.relative_strength(), 24);
        // assert_eq!(ProseAccent::TelishaQetannah.relative_strength(), 25);
        // assert_eq!(ProseAccent::Galgal.relative_strength(), 26);
        // assert_eq!(ProseAccent::Mayela.relative_strength(), 27);
        // assert_eq!(ProseAccent::Mayela.relative_strength(), 27);
        // assert_eq!(ProseAccent::Meteg.relative_strength(), 28);
    }

    #[test]
    fn testing_hebrew_poetry_accent_relative_strengths() {
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Illuy).relative_strength(),
            15
        );
    }

    #[test]
    fn testing_poetry_accent_relative_strengths() {
        // Disjunctives
        // assert_eq!(PoetryAccent::Silluq.relative_strength(), 1);
        // assert_eq!(PoetryAccent::OlehWeYored.relative_strength(), 2,);
        // assert_eq!(PoetryAccent::Atnach.relative_strength(), 3);
        // assert_eq!(PoetryAccent::ReviaGadol.relative_strength(), 4);
        // assert_eq!(PoetryAccent::ReviaMugrash.relative_strength(), 5);
        // assert_eq!(PoetryAccent::ShalsheletGadol.relative_strength(), 6);
        // assert_eq!(PoetryAccent::Tsinnor.relative_strength(), 7);
        // assert_eq!(PoetryAccent::ReviaQaton.relative_strength(), 8);
        // assert_eq!(PoetryAccent::Dechi.relative_strength(), 9);
        // assert_eq!(PoetryAccent::Pazer.relative_strength(), 10);
        // assert_eq!(PoetryAccent::MehuppakhLegarmeh.relative_strength(), 11);
        assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), 12);
        // Conjunctives
        // assert_eq!(PoetryAccent::Munach.relative_strength(), 13);
        // assert_eq!(PoetryAccent::Merkha.relative_strength(), 14);
        // assert_eq!(PoetryAccent::Illuy.relative_strength(), 15);
        // assert_eq!(PoetryAccent::Tarcha.relative_strength(), 16);
        // assert_eq!(PoetryAccent::Galgal.relative_strength(), 17);
        // assert_eq!(PoetryAccent::Mehuppakh.relative_strength(), 18);
        // assert_eq!(PoetryAccent::Azla.relative_strength(), 19);
        // assert_eq!(PoetryAccent::ShalsheletQetannah.relative_strength(), 20);
        assert_eq!(PoetryAccent::TsinnoritMerkha.relative_strength(), 21);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.relative_strength(), 21);
        assert_eq!(PoetryAccent::Meteg.relative_strength(), 22);
    }

    #[test]
    fn testing_hebrew_pseudo_accent_relative_strengths() {
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).relative_strength(),
            1
        );
    }

    #[test]
    fn testing_pseudo_accent_relative_strengths() {
        assert_eq!(PseudoAccent::SophPasuq.relative_strength(), 1);
        assert_eq!(PseudoAccent::Maqqeph.relative_strength(), 2,);
        assert_eq!(PseudoAccent::Paseq.relative_strength(), 3,);
    }
}

#[cfg(test)]
mod test_hierarchical_group {
    use crate::{Accent, HebrewAccent, HierarchicalGroup, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_hebrew_prose_accent_hierarchical_group() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Silluq).hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup1)
        );
    }

    #[test]
    fn testing_prose_accent_hierarchical_group() {
        // Disjunctives
        assert_eq!(
            ProseAccent::Silluq.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup1)
        );
        assert_eq!(
            ProseAccent::Atnach.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup1)
        );
        assert_eq!(
            ProseAccent::Segolta.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup2)
        );
        assert_eq!(
            ProseAccent::Shalshelet.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup2)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup2)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup2)
        );
        assert_eq!(
            ProseAccent::Revia.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup3)
        );
        assert_eq!(
            ProseAccent::Tiphcha.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup2)
        );
        assert_eq!(
            ProseAccent::Zarqa.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup3)
        );
        assert_eq!(
            ProseAccent::Pashta.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup3)
        );
        assert_eq!(
            ProseAccent::Yetiv.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup3)
        );
        assert_eq!(
            ProseAccent::Tevir.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup3)
        );
        assert_eq!(
            ProseAccent::Geresh.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        assert_eq!(
            ProseAccent::Gershayim.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        assert_eq!(
            ProseAccent::Pazer.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        assert_eq!(
            ProseAccent::PazerGadol.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        assert_eq!(
            ProseAccent::Legarmeh.hierarchical_group(),
            Some(HierarchicalGroup::ProseGroup4)
        );
        // Conjunctives
        assert_eq!(ProseAccent::Munach.hierarchical_group(), None);
        assert_eq!(ProseAccent::Mahpakh.hierarchical_group(), None);
        assert_eq!(ProseAccent::Merkha.hierarchical_group(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.hierarchical_group(), None);
        assert_eq!(ProseAccent::Darga.hierarchical_group(), None);
        assert_eq!(ProseAccent::Azla.hierarchical_group(), None);
        assert_eq!(ProseAccent::TelishaQetannah.hierarchical_group(), None);
        assert_eq!(ProseAccent::Galgal.hierarchical_group(), None);
        assert_eq!(ProseAccent::Mayela.hierarchical_group(), None);
        assert_eq!(ProseAccent::Meteg.hierarchical_group(), None);
    }

    #[test]
    fn testing_hebrew_poetry_accent_hierarchical_group() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Silluq).hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup1)
        );
    }
    #[test]
    fn testing_poetry_accent_hierarchical_group() {
        // Disjunctives
        assert_eq!(
            PoetryAccent::Silluq.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup1)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup1)
        );
        assert_eq!(
            PoetryAccent::Atnach.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup1)
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::Dechi.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup2)
        );
        assert_eq!(
            PoetryAccent::Pazer.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup3)
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup3)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.hierarchical_group(),
            Some(HierarchicalGroup::PoetryGroup3)
        );
        // Conjunctives
        assert_eq!(PoetryAccent::Munach.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Merkha.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Illuy.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Tarcha.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Galgal.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Mehuppakh.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Azla.hierarchical_group(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.hierarchical_group(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.hierarchical_group(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.hierarchical_group(), None);
        assert_eq!(PoetryAccent::Meteg.hierarchical_group(), None);
    }

    #[test]
    fn testing_hebrew_pseudo_accent_hierarchical_group() {
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).hierarchical_group(),
            None
        );
        //assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqeph).hierarchical_group(), None);
        //assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).hierarchical_group(), None);
    }

    #[test]
    fn testing_pseudo_accent_hierarchical_group() {
        assert_eq!(PseudoAccent::SophPasuq.hierarchical_group(), None);
        assert_eq!(PseudoAccent::Maqqeph.hierarchical_group(), None);
        assert_eq!(PseudoAccent::Paseq.hierarchical_group(), None);
    }
}

#[cfg(test)]
mod test_details {
    use crate::accent_codepoints::{CP_GERESH, CP_MERKHA, CP_OLE, CP_REVIA, CP_SHALSHELET};
    use crate::accent_codepoints::{CP_GERSHAYIM, CP_SEGOL, CP_TIPEHA};
    use crate::accent_codepoints::{CP_MAQAF, CP_PASEQ, CP_SOPH_PASUQ};
    use crate::{
        Accent, AccentCategory, AccentInformation, AccentType, Additional, Alternates, CodePoints,
        WordStress,
    };
    use crate::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_prose_accent_details() {
        let expected_result = AccentInformation {
            english_name: "Segolta",
            hebrew_name: "סְגֹולְתָּא",
            meaning: "a little grape-bunch",
            code_points: CodePoints {
                primary: &CP_SEGOL,
                secondary: None,
            },
            comment: None,
            additional: Some(Additional {
                accent_type: AccentType::Primary,
                category: AccentCategory::Disjunctive,
                word_stress: Some(WordStress::PostPositive),
                hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
                alternates: None,
            }),
        };
        assert_eq!(ProseAccent::Segolta.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Segolta).details(),
            &expected_result
        );
        //
        let expected_result = AccentInformation {
            english_name: "Gershayim",
            hebrew_name: "גֵּרְשַׁיִם",
            meaning: "double of expulsion, driving out, divorce",
            code_points: CodePoints {
                primary: &CP_GERSHAYIM,
                secondary: None,
            },
            comment: None,
            additional: Some(Additional {
                accent_type: AccentType::Primary,
                category: AccentCategory::Disjunctive,
                word_stress: Some(WordStress::ImPositive),
                hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
                alternates: None,
            }),
        };

        assert_eq!(ProseAccent::Gershayim.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Gershayim).details(),
            &expected_result
        );
        //
        let expected_result = AccentInformation {
            english_name: "Mayela",
            hebrew_name: "מָאיְלָא",
            meaning: "to be raised or elevated",
            code_points: CodePoints {
                primary: &CP_TIPEHA,
                secondary: None,
            },
            comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq"),
            additional: Some(Additional {
                accent_type: AccentType::Secondary,
                category: AccentCategory::Conjunctive,
                word_stress: None,
                hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
                alternates: Some(Alternates {
                    english_name: "Meayyela",
                    hebrew_name: "מְאַיְּלָא",
                    meaning: "todo",
                }),
            }),
        };
        assert_eq!(ProseAccent::Mayela.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Mayela).details(),
            &expected_result
        );
    }

    #[test]
    fn testing_poetry_accent_details() {
        let expected_result = AccentInformation {
    english_name: "Oleh We Yored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    meaning: "ascending and descending",
    code_points: CodePoints {
        primary: &CP_OLE,
        secondary: Some(&CP_MERKHA),
    },
    comment: Some("The primary CodePoint is Mehuppakh, but located above the consonant. It is then called OLE."),    
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
        alternates: None,
    }),
};

        assert_eq!(PoetryAccent::OlehWeYored.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored).details(),
            &expected_result
        );
        //
        let expected_result = AccentInformation {
            english_name: "Revia Mugrash",
            hebrew_name: "רְבִיעַ מֻגְרָשׁ",
            meaning: "exiled fourth",
            code_points: CodePoints {
                primary: &CP_GERESH,
                secondary: Some(&CP_REVIA),
            },
            comment: None,
            additional: Some(Additional {
                accent_type: AccentType::Primary,
                category: AccentCategory::Disjunctive,
                word_stress: Some(WordStress::ImPositive),
                hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
                alternates: None,
            }),
        };

        assert_eq!(PoetryAccent::ReviaMugrash.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).details(),
            &expected_result
        );
        //
        let expected_result = AccentInformation {
            english_name: "Shalshelet Qetannah",
            hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
            meaning: "small chain",
            code_points: CodePoints {
                primary: &CP_SHALSHELET,
                secondary: None,
            },
            comment: None,
            additional: Some(Additional {
                accent_type: AccentType::Primary,
                category: AccentCategory::Conjunctive,
                word_stress: Some(WordStress::ImPositive),
                hierarchical_group: Some(crate::HierarchicalGroup::PoetryGroup2),
                alternates: None,
            }),
        };

        assert_eq!(PoetryAccent::ShalsheletQetannah.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).details(),
            &expected_result
        );
    }

    #[test]
    fn testing_pseudo_accent_details() {
        let expected_result = AccentInformation {
        english_name: "Soph Pasuq",
        hebrew_name: "סוֹף פָּסוּק",
        meaning: "end of verse",
        code_points: CodePoints {
            primary: &CP_SOPH_PASUQ,
            secondary: None,
        },
        comment: Some(
            "it doesn’t carry any theological or interpretive meaning beyond marking a boundary",
        ),
        additional: None,
    };
        assert_eq!(PseudoAccent::SophPasuq.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).details(),
            &expected_result
        );
        //
        let expected_result = AccentInformation {
    english_name: "Maqqeph",
    hebrew_name: "מַקֵּף",
    meaning:"binder",
    code_points: CodePoints {
        primary: &CP_MAQAF,
        secondary: None,
    },
    comment: Some("Can link two (or more) short words together, after which they function as a single compound word bearing a single Hebrew accent."),
    additional:  None,
};

        assert_eq!(PseudoAccent::Maqqeph.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph).details(),
            &expected_result
        );
        //
        let expected_result =  AccentInformation {
    english_name: "Paseq",
    hebrew_name: "פָּסֵק",
    meaning: "to pause, to stop or to interrupt",
    code_points: CodePoints {
        primary: &CP_PASEQ,
        secondary: None,
    },
    comment: Some(
        "It’s indicating that someone or something is stopping temporarily or creating a pause.",
    ),
    additional: None,
};

        assert_eq!(PseudoAccent::Paseq.details(), &expected_result);
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::Paseq).details(),
            &expected_result
        );
    }
}

#[cfg(test)]
mod test_english_name {
    use crate::{Accent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_prose_accent_english_name() {
        // Disjunctives
        assert_eq!(ProseAccent::Silluq.english_name(), "Silluq");
        assert_eq!(ProseAccent::Atnach.english_name(), "Atnach");
        assert_eq!(ProseAccent::Segolta.english_name(), "Segolta");
        assert_eq!(ProseAccent::Shalshelet.english_name(), "Shalshelet");
        assert_eq!(ProseAccent::ZaqephQatan.english_name(), "Zaqeph Qaton");
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
        // Conjunctives
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
        assert_eq!(ProseAccent::Mayela.english_name(), "Mayela");
        assert_eq!(ProseAccent::Meteg.english_name(), "Meteg");
    }

    #[test]
    fn testing_poetry_accent_english_name() {
        // Disjunctives
        assert_eq!(PoetryAccent::Silluq.english_name(), "Silluq");
        assert_eq!(PoetryAccent::OlehWeYored.english_name(), "Oleh We Yored");
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
        // Conjunctives
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
    fn testing_pseudo_accent_english_name() {
        assert_eq!(PseudoAccent::SophPasuq.english_name(), "Soph Pasuq");
        assert_eq!(PseudoAccent::Maqqeph.english_name(), "Maqqeph");
        assert_eq!(PseudoAccent::Paseq.english_name(), "Paseq");
    }
}
#[cfg(test)]
mod test_hebrew_name {
    use crate::{Accent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_prose_accent_hebrew_name() {
        // Disjunctives
        assert_eq!(ProseAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(ProseAccent::Atnach.hebrew_name(), "אתְנָח");
        assert_eq!(ProseAccent::Segolta.hebrew_name(), "סְגֹולְתָּא");
        assert_eq!(ProseAccent::Shalshelet.hebrew_name(), "שַׁלְשֶׁלֶת");
        assert_eq!(ProseAccent::ZaqephQatan.hebrew_name(), "זָקֵף קָטוֹן");
        assert_eq!(ProseAccent::ZaqephGadol.hebrew_name(), "זָקֵף גָּדוֹל");
        assert_eq!(ProseAccent::Revia.hebrew_name(), "רְבִיעַ");
        assert_eq!(ProseAccent::Tiphcha.hebrew_name(), "טִפְחָא");
        assert_eq!(ProseAccent::Zarqa.hebrew_name(), "זַרְקָא");
        assert_eq!(ProseAccent::Pashta.hebrew_name(), "פַּשְׁטָא");
        assert_eq!(ProseAccent::Yetiv.hebrew_name(), "יְתִיב");
        assert_eq!(ProseAccent::Tevir.hebrew_name(), "תְּבִיר");
        assert_eq!(ProseAccent::Geresh.hebrew_name(), "גֵּרֵישׁ");
        assert_eq!(ProseAccent::Gershayim.hebrew_name(), "גֵּרְשַׁיִם");
        assert_eq!(ProseAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(ProseAccent::PazerGadol.hebrew_name(), "פָּזֶר גּדוֹל");
        assert_eq!(ProseAccent::TelishaGedolah.hebrew_name(), "תְּלִישָׁא גְּדוֹלָה");
        assert_eq!(ProseAccent::Legarmeh.hebrew_name(), "לְגַרְמֶהּ");
        // Conjunctives
        assert_eq!(ProseAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(ProseAccent::Mahpakh.hebrew_name(), "מַהְפַּךְ");
        assert_eq!(ProseAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(ProseAccent::MerkhaKephulah.hebrew_name(), "מֵרְכָא כְּפוּלָה");
        assert_eq!(ProseAccent::Darga.hebrew_name(), "דַּרְגָּא");
        assert_eq!(ProseAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(ProseAccent::TelishaQetannah.hebrew_name(), "תְּלִישָא קְטַנָּה");
        assert_eq!(ProseAccent::Galgal.hebrew_name(), "גַּלְגַּל");
        assert_eq!(ProseAccent::Mayela.hebrew_name(), "מָאיְלָא");
        assert_eq!(ProseAccent::Meteg.hebrew_name(), "מֶתֶג");
    }

    #[test]
    fn testing_poetry_accent_hebrew_name() {
        // Disjunctives
        assert_eq!(PoetryAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(PoetryAccent::OlehWeYored.hebrew_name(), "עוֹלֶה וְיוֹרֵד");
        assert_eq!(PoetryAccent::Atnach.hebrew_name(), "אתְנָח");
        assert_eq!(PoetryAccent::ReviaGadol.hebrew_name(), "רְבִיעַ גּדוֹל");
        assert_eq!(PoetryAccent::ReviaMugrash.hebrew_name(), "רְבִיעַ מֻגְרָשׁ");
        assert_eq!(PoetryAccent::ShalsheletGadol.hebrew_name(), "שַׁלְשֶׁלֶת גָּדוֹל");
        assert_eq!(PoetryAccent::Tsinnor.hebrew_name(), "צִנּוֹר");
        assert_eq!(PoetryAccent::ReviaQaton.hebrew_name(), "רְבִיעַ קָטוֹן");
        assert_eq!(PoetryAccent::Dechi.hebrew_name(), "דֶּחִי");
        assert_eq!(PoetryAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.hebrew_name(), "מְהֻפָּךְ לְגַרְמֵהּ");
        assert_eq!(PoetryAccent::AzlaLegarmeh.hebrew_name(), "אַזְלָא לְגַרְמֶהּ");
        // Conjunctives
        assert_eq!(PoetryAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(PoetryAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(PoetryAccent::Illuy.hebrew_name(), "עִלּוּי");
        assert_eq!(PoetryAccent::Tarcha.hebrew_name(), "טַרְחָא");
        assert_eq!(PoetryAccent::Galgal.hebrew_name(), "גַּלְגַּל");
        assert_eq!(PoetryAccent::Mehuppakh.hebrew_name(), "מְהֻפָּ֤ךְ");
        assert_eq!(PoetryAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(PoetryAccent::ShalsheletQetannah.hebrew_name(), "שַׁלְשֶׁלֶת קְטַנָּה");
        assert_eq!(PoetryAccent::TsinnoritMerkha.hebrew_name(), "צִנּוֹרִת מֵרְכָא");
        assert_eq!(PoetryAccent::TsinnoritMahpakh.hebrew_name(), "צִנּוֹרִת מַהְפַּךְ");
        assert_eq!(PoetryAccent::Meteg.hebrew_name(), "מֶתֶג");
    }

    #[test]
    fn testing_pseudo_accent_hebrew_name() {
        assert_eq!(PseudoAccent::SophPasuq.hebrew_name(), "סוֹף פָּסוּק");
        assert_eq!(PseudoAccent::Maqqeph.hebrew_name(), "מַקֵּף");
        assert_eq!(PseudoAccent::Paseq.hebrew_name(), "פָּסֵק");
    }
}
#[cfg(test)]
mod test_meaning {
    use crate::{Accent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_prose_accent_meaning() {
        // Disjunctives
        assert_eq!(ProseAccent::Silluq.meaning(), "close, cessation");
        assert_eq!(ProseAccent::Atnach.meaning(), "a causing to rest");
        assert_eq!(ProseAccent::Segolta.meaning(), "a little grape-bunch");
        assert_eq!(ProseAccent::Shalshelet.meaning(), "chain or link");
        assert_eq!(ProseAccent::ZaqephQatan.meaning(), "small upright");
        assert_eq!(ProseAccent::ZaqephGadol.meaning(), "large upright");
        assert_eq!(ProseAccent::Revia.meaning(), "fourth [in a sequence]");
        assert_eq!(ProseAccent::Tiphcha.meaning(), "handbreadth or diagonal");
        assert_eq!(ProseAccent::Zarqa.meaning(), "to sprinkle, scatter");
        assert_eq!(
            ProseAccent::Pashta.meaning(),
            "extending, stretching out in length"
        );
        assert_eq!(ProseAccent::Yetiv.meaning(), "resting or sitting");
        assert_eq!(ProseAccent::Tevir.meaning(), "broken, downward tumble");
        assert_eq!(
            ProseAccent::Geresh.meaning(),
            "expulsion, driving out, divorce"
        );
        assert_eq!(
            ProseAccent::Gershayim.meaning(),
            "double of expulsion, driving out, divorce"
        );
        assert_eq!(ProseAccent::Pazer.meaning(), "lavish or scatter");
        assert_eq!(ProseAccent::PazerGadol.meaning(), "large lavish or scatter");
        assert_eq!(
            ProseAccent::TelishaGedolah.meaning(),
            "great (long) detached"
        );
        assert_eq!(
            ProseAccent::Legarmeh.meaning(),
            "for or by itself, independant"
        );
        // Conjunctives
        assert_eq!(ProseAccent::Munach.meaning(), "resting or placed");
        assert_eq!(ProseAccent::Mahpakh.meaning(), "turning round");
        assert_eq!(ProseAccent::Merkha.meaning(), "lengthener, prolonging");
        assert_eq!(ProseAccent::MerkhaKephulah.meaning(), "double lengthener");
        assert_eq!(ProseAccent::Darga.meaning(), "stairstep");
        assert_eq!(
            ProseAccent::Azla.meaning(),
            "going on (not pausing), depart"
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.meaning(),
            "small (short) detached"
        );
        assert_eq!(ProseAccent::Galgal.meaning(), "wheel, circle");
        assert_eq!(ProseAccent::Mayela.meaning(), "to be raised or elevated");
        assert_eq!(ProseAccent::Meteg.meaning(), "accent or mark");
    }

    #[test]
    fn testing_poetry_accent_meaning() {
        // Disjunctives
        assert_eq!(PoetryAccent::Silluq.meaning(), "close, cessation");
        assert_eq!(
            PoetryAccent::OlehWeYored.meaning(),
            "ascending and descending"
        );
        assert_eq!(PoetryAccent::Atnach.meaning(), "a causing to rest");
        assert_eq!(PoetryAccent::ReviaGadol.meaning(), "big fourth");
        assert_eq!(PoetryAccent::ReviaMugrash.meaning(), "exiled fourth");
        assert_eq!(
            PoetryAccent::ShalsheletGadol.meaning(),
            "large chain or link"
        );
        assert_eq!(PoetryAccent::Tsinnor.meaning(), "pipe or tube");
        assert_eq!(PoetryAccent::ReviaQaton.meaning(), "small fourth");
        assert_eq!(PoetryAccent::Dechi.meaning(), "to push or drive away");
        assert_eq!(PoetryAccent::Pazer.meaning(), "lavish or scatter");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.meaning(),
            "reversed to its own"
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.meaning(), "goes to its own");
        // Conjunctives
        assert_eq!(PoetryAccent::Munach.meaning(), "resting or placed");
        assert_eq!(PoetryAccent::Merkha.meaning(), "lengthener, prolonging");
        assert_eq!(PoetryAccent::Illuy.meaning(), "elevation or raising");
        assert_eq!(
            PoetryAccent::Tarcha.meaning(),
            "trouble, difficulty, hardship, toil"
        );
        assert_eq!(PoetryAccent::Galgal.meaning(), "wheel, circle");
        assert_eq!(PoetryAccent::Mehuppakh.meaning(), "reversed");
        assert_eq!(
            PoetryAccent::Azla.meaning(),
            "going on (not pausing), depart"
        );
        assert_eq!(PoetryAccent::ShalsheletQetannah.meaning(), "small chain");
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.meaning(),
            "pipe of continuation"
        );
        assert_eq!(PoetryAccent::TsinnoritMahpakh.meaning(), "pipe of reversal");
        assert_eq!(PoetryAccent::Meteg.meaning(), "accent or mark");
    }

    #[test]
    fn testing_pseudo_accent_meaning() {
        assert_eq!(PseudoAccent::SophPasuq.meaning(), "end of verse");
        assert_eq!(PseudoAccent::Maqqeph.meaning(), "binder");
        assert_eq!(
            PseudoAccent::Paseq.meaning(),
            "to pause, to stop or to interrupt"
        );
    }
}
#[cfg(test)]
mod test_accent_type {
    use crate::{Accent, AccentType, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_hebrew_prose_accent_types() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Silluq).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Atnach).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Segolta).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Shalshelet).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::ZaqephQatan).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::ZaqephGadol).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Revia).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Tiphcha).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Zarqa).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Pashta).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Yetiv).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Tevir).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Geresh).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Gershayim).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Pazer).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::PazerGadol).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::TelishaGedolah).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Legarmeh).accent_type(),
            Some(AccentType::Primary)
        );
        // Conjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Munach).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Mahpakh).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Merkha).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Darga).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Azla).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::TelishaQetannah).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Galgal).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Mayela).accent_type(),
            Some(AccentType::Secondary)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Meteg).accent_type(),
            Some(AccentType::Secondary)
        );
    }

    #[test]
    fn testing_prose_accent_types() {
        // Disjunctives
        assert_eq!(ProseAccent::Silluq.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Atnach.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::Segolta.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            ProseAccent::Shalshelet.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Revia.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::Tiphcha.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Zarqa.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Pashta.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Yetiv.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Tevir.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Geresh.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::Gershayim.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Pazer.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::PazerGadol.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            ProseAccent::Legarmeh.accent_type(),
            Some(AccentType::Primary)
        );
        // Conjunctives
        assert_eq!(ProseAccent::Munach.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::Mahpakh.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Merkha.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::MerkhaKephulah.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Darga.accent_type(), Some(AccentType::Primary));
        assert_eq!(ProseAccent::Azla.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::TelishaQetannah.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(ProseAccent::Galgal.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            ProseAccent::Mayela.accent_type(),
            Some(AccentType::Secondary)
        );
        assert_eq!(
            ProseAccent::Meteg.accent_type(),
            Some(AccentType::Secondary)
        );
    }

    #[test]
    fn testing_hebrew_poetry_accent_types() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Silluq).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Atnach).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Tsinnor).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Dechi).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Pazer).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).accent_type(),
            Some(AccentType::Primary)
        );
        // Conjunctives
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Munach).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Merkha).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Illuy).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Tarcha).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Galgal).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Azla).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Meteg).accent_type(),
            Some(AccentType::Secondary)
        );
    }

    #[test]
    fn testing_poetry_accent_types() {
        // Disjunctives
        assert_eq!(
            PoetryAccent::Silluq.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Atnach.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(PoetryAccent::Dechi.accent_type(), Some(AccentType::Primary));
        assert_eq!(PoetryAccent::Pazer.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.accent_type(),
            Some(AccentType::Primary)
        );
        // Conjunctives
        assert_eq!(
            PoetryAccent::Munach.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Merkha.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(PoetryAccent::Illuy.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            PoetryAccent::Tarcha.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Galgal.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(PoetryAccent::Azla.accent_type(), Some(AccentType::Primary));
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.accent_type(),
            Some(AccentType::Primary)
        );
        assert_eq!(
            PoetryAccent::Meteg.accent_type(),
            Some(AccentType::Secondary)
        );
    }

    #[test]
    fn testing_hebrew_pseudo_accent_types() {
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).accent_type(),
            None
        );
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph).accent_type(),
            None
        );
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::Paseq).accent_type(),
            None
        );
    }
    #[test]
    fn testing_pseudo_accent_types() {
        assert_eq!(PseudoAccent::SophPasuq.accent_type(), None);
        assert_eq!(PseudoAccent::Maqqeph.accent_type(), None);
        assert_eq!(PseudoAccent::Paseq.accent_type(), None);
    }
}

#[cfg(test)]
mod test_category {
    use crate::{Accent, AccentCategory, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_hebrew_prose_accent_categories() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Silluq).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Atnach).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Segolta).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Shalshelet).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::ZaqephQatan).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::ZaqephGadol).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Revia).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Tiphcha).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Zarqa).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Pashta).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Yetiv).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Tevir).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Geresh).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Gershayim).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Pazer).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::PazerGadol).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::TelishaGedolah).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Legarmeh).category(),
            Some(AccentCategory::Disjunctive)
        );
        // Conjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Munach).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Mahpakh).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Merkha).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Darga).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Azla).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::TelishaQetannah).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Galgal).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Mayela).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Meteg).category(),
            Some(AccentCategory::Conjunctive)
        );
    }

    #[test]
    fn testing_prose_accent_categories() {
        // Disjunctives
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
        // Conjunctives
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
            ProseAccent::Mayela.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Meteg.category(),
            Some(AccentCategory::Conjunctive)
        );
    }

    #[test]
    fn testing_hebrew_poetry_accent_categories() {
        // Disjunctives
        // assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqeph).category(), None);
        // HebrewAccent::Prose(ProseAccent::Silluq).category(),
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Silluq).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Atnach).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Tsinnor).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Dechi).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Pazer).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).category(),
            Some(AccentCategory::Disjunctive)
        );
        // Conjunctives
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Munach).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Merkha).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Illuy).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Tarcha).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Galgal).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Azla).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::Meteg).category(),
            Some(AccentCategory::Conjunctive)
        );
    }
    #[test]
    fn testing_poetry_accent_categories() {
        // Disjunctives
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
        // Conjunctives
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
    fn testing_hebrew_pseudo_accent_categories() {
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).category(),
            None
        );
        assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqeph).category(), None);
        assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).category(), None);
    }
    #[test]
    fn testing_pseudo_accent_categories() {
        assert_eq!(PseudoAccent::SophPasuq.category(), None);
        assert_eq!(PseudoAccent::Maqqeph.category(), None);
        assert_eq!(PseudoAccent::Paseq.category(), None);
    }
}
#[cfg(test)]
mod test_word_stress {
    use crate::{Accent, PoetryAccent, ProseAccent, PseudoAccent, WordStress};

    #[test]
    fn testing_prose_accent_word_stress() {
        assert_eq!(
            ProseAccent::Silluq.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Atnach.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Segolta.word_stress(),
            Some(WordStress::PostPositive)
        );
        assert_eq!(
            ProseAccent::Shalshelet.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Revia.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Tiphcha.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Zarqa.word_stress(),
            Some(WordStress::PostPositive)
        );
        assert_eq!(
            ProseAccent::Pashta.word_stress(),
            Some(WordStress::PostPositive)
        );
        assert_eq!(
            ProseAccent::Yetiv.word_stress(),
            Some(WordStress::PrePositive)
        );
        assert_eq!(
            ProseAccent::Tevir.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Geresh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Gershayim.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Pazer.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::PazerGadol.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.word_stress(),
            Some(WordStress::PrePositive)
        );
        assert_eq!(
            ProseAccent::Legarmeh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Munach.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Mahpakh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Merkha.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Darga.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::Azla.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.word_stress(),
            Some(WordStress::PostPositive)
        );
        assert_eq!(
            ProseAccent::Galgal.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(ProseAccent::Mayela.word_stress(), None);
        assert_eq!(ProseAccent::Meteg.word_stress(), None);
    }

    #[test]
    fn testing_poetry_accent_word_stress() {
        assert_eq!(
            PoetryAccent::Silluq.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Atnach.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.word_stress(),
            Some(WordStress::PostPositive)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Dechi.word_stress(),
            Some(WordStress::PrePositive)
        );
        assert_eq!(
            PoetryAccent::Pazer.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Munach.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Merkha.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Illuy.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Tarcha.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Galgal.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::Azla.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.word_stress(),
            Some(WordStress::ImPositive)
        );
        assert_eq!(PoetryAccent::Meteg.word_stress(), None);
    }

    #[test]
    fn testing_pseudo_accent_word_stress() {
        assert_eq!(PseudoAccent::SophPasuq.word_stress(), None);
        assert_eq!(PseudoAccent::Maqqeph.word_stress(), None);
        assert_eq!(PseudoAccent::Paseq.word_stress(), None);
    }
}
#[cfg(test)]
mod test_code_points {
    use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    #[test]
    fn testing_prose_accent_code_points() {
        // Disjunctives
        assert_eq!(HebrewAccent::Prose(ProseAccent::Silluq).code_points(), 1);
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Shalshelet).code_points(),
            2
        );

        assert_eq!(ProseAccent::Silluq.code_points(), 1);
        assert_eq!(ProseAccent::Atnach.code_points(), 1);
        assert_eq!(ProseAccent::Segolta.code_points(), 1);
        assert_eq!(ProseAccent::Shalshelet.code_points(), 2);
        assert_eq!(ProseAccent::ZaqephQatan.code_points(), 1);
        assert_eq!(ProseAccent::ZaqephGadol.code_points(), 1);
        assert_eq!(ProseAccent::Revia.code_points(), 1);
        assert_eq!(ProseAccent::Tiphcha.code_points(), 1);
        assert_eq!(ProseAccent::Zarqa.code_points(), 1);
        assert_eq!(ProseAccent::Pashta.code_points(), 1);
        assert_eq!(ProseAccent::Yetiv.code_points(), 1);
        assert_eq!(ProseAccent::Tevir.code_points(), 1);
        assert_eq!(ProseAccent::Geresh.code_points(), 1);
        assert_eq!(ProseAccent::Gershayim.code_points(), 1);
        assert_eq!(ProseAccent::Pazer.code_points(), 1);
        assert_eq!(ProseAccent::PazerGadol.code_points(), 1);
        assert_eq!(ProseAccent::TelishaGedolah.code_points(), 1);
        assert_eq!(ProseAccent::Legarmeh.code_points(), 2);
        // Conjunctives
        assert_eq!(ProseAccent::Munach.code_points(), 1);
        assert_eq!(ProseAccent::Mahpakh.code_points(), 1);
        assert_eq!(ProseAccent::Merkha.code_points(), 1);
        assert_eq!(ProseAccent::MerkhaKephulah.code_points(), 1);
        assert_eq!(ProseAccent::Darga.code_points(), 1);
        assert_eq!(ProseAccent::Azla.code_points(), 1);
        assert_eq!(ProseAccent::TelishaQetannah.code_points(), 1);
        assert_eq!(ProseAccent::Galgal.code_points(), 1);
        assert_eq!(ProseAccent::Mayela.code_points(), 1);
        assert_eq!(ProseAccent::Meteg.code_points(), 1);
    }

    #[test]
    fn testing_poetry_accent_code_points() {
        // Disjunctives
        assert_eq!(HebrewAccent::Poetry(PoetryAccent::Silluq).code_points(), 1);
        assert_eq!(
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored).code_points(),
            2
        );

        assert_eq!(PoetryAccent::Silluq.code_points(), 1);
        assert_eq!(PoetryAccent::OlehWeYored.code_points(), 2,);
        assert_eq!(PoetryAccent::Atnach.code_points(), 1);
        assert_eq!(PoetryAccent::ReviaGadol.code_points(), 1);
        assert_eq!(PoetryAccent::ReviaMugrash.code_points(), 2);
        assert_eq!(PoetryAccent::ShalsheletGadol.code_points(), 2);
        assert_eq!(PoetryAccent::Tsinnor.code_points(), 1);
        assert_eq!(PoetryAccent::ReviaQaton.code_points(), 1);
        assert_eq!(PoetryAccent::Dechi.code_points(), 1);
        assert_eq!(PoetryAccent::Pazer.code_points(), 1);
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.code_points(), 2);
        assert_eq!(PoetryAccent::AzlaLegarmeh.code_points(), 2);
        // Conjunctives
        assert_eq!(PoetryAccent::Munach.code_points(), 1);
        assert_eq!(PoetryAccent::Merkha.code_points(), 1);
        assert_eq!(PoetryAccent::Illuy.code_points(), 1);
        assert_eq!(PoetryAccent::Tarcha.code_points(), 1);
        assert_eq!(PoetryAccent::Galgal.code_points(), 1);
        assert_eq!(PoetryAccent::Mehuppakh.code_points(), 1);
        assert_eq!(PoetryAccent::Azla.code_points(), 1);
        assert_eq!(PoetryAccent::ShalsheletQetannah.code_points(), 1);
        assert_eq!(PoetryAccent::TsinnoritMerkha.code_points(), 2);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.code_points(), 2);
        assert_eq!(PoetryAccent::Meteg.code_points(), 1);
    }

    #[test]
    fn testing_pseudo_accent_code_points() {
        assert_eq!(
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).code_points(),
            1
        );
        assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqeph).code_points(), 1);
        assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).code_points(), 1);

        assert_ne!(PseudoAccent::SophPasuq.code_points(), 2);
        assert_eq!(PseudoAccent::Maqqeph.code_points(), 1,);
        assert_eq!(PseudoAccent::Paseq.code_points(), 1,);
    }
}
