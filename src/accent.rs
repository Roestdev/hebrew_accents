//! Main entry point for Hebrew Accent information

// Crate‑internal (local modules)
use crate::accent_data::{
    BHS_POETRY_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE,
};

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HebrewAccent {
    /// Prose variant
    Prose(ProseAccent),
    /// Poetry variant
    Poetry(PoetryAccent),
    /// Pseudo variant
    Pseudo(PseudoAccent),
}
impl HebrewAccent {
    /// Returns a reference to the inner [`ProseAccent`] if this is a Prose variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Prose variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Silluq);
    /// assert!(matches!(accent.as_prose(), Some(ProseAccent::Silluq)));
    /// ```
    ///
    /// Returns `None` for non-Prose variants:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
    /// assert_eq!(accent.as_prose(), None);
    /// ```
    ///
    /// The returned reference can be used multiple times without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Segolta);
    ///
    /// // First usage
    /// let name1 = accent.as_prose().unwrap().english_name();
    ///
    /// // Second usage - original still available
    /// let name2 = accent.as_prose().unwrap().hebrew_concept();
    ///
    /// assert_eq!(name1, "Segolta");
    /// ```
    pub fn as_prose(&self) -> Option<&ProseAccent> {
        match self {
            Self::Prose(p) => Some(p),
            _ => None,
        }
    }

    /// Returns a reference to the inner [`PoetryAccent`] if this is a Poetry variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Poetry variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
    /// assert!(matches!(accent.as_poetry(), Some(PoetryAccent::OlehWeYored)));
    /// ```
    ///
    /// Failing to extract from a Prose variant (returns `None`):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Atnach);
    /// assert_eq!(accent.as_poetry(), None);
    /// ```
    ///
    /// Accessing methods on the borrowed inner value without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
    ///
    /// // Can access properties through the reference
    /// if let Some(poetry) = accent.as_poetry() {
    ///     println!("English name: {}", poetry.english_name());
    ///     println!("Relative strength: {}", poetry.relative_strength());
    /// }
    ///
    /// // Original accent remains usable after inspection
    /// assert!(accent.as_poetry().is_some());
    /// ```
    pub fn as_poetry(&self) -> Option<&PoetryAccent> {
        match self {
            Self::Poetry(p) => Some(p),
            _ => None,
        }
    }

    /// Returns a reference to the inner [`PseudoAccent`] if this is a Pseudo variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Pseudo variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PseudoAccent, Accent};
    ///
    /// let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::SophPasuq)));
    /// ```
    ///
    /// Failing to extract from Prose or Poetry variants (returns `None`):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent, PoetryAccent};
    ///
    /// let prose = HebrewAccent::Prose(ProseAccent::Silluq);
    /// let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
    ///
    /// assert_eq!(prose.as_pseudo(), None);
    /// assert_eq!(poetry.as_pseudo(), None);
    /// ```
    ///
    /// Inspecting PseudoAccent properties without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{Accent,HebrewAccent, PseudoAccent};
    ///
    /// let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);
    ///
    /// // Check the accent type exists before accessing its data
    /// if let Some(pseudo) = accent.as_pseudo() {
    ///     println!("English name: {}", pseudo.english_name());      // "Maqqeph"
    ///     println!("Concept: {}", pseudo.hebrew_concept());         // "binder"
    /// }
    ///
    /// // The original accent remains usable after inspection
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::Maqqeph)));
    /// ```
    pub fn as_pseudo(&self) -> Option<&PseudoAccent> {
        match self {
            Self::Pseudo(p) => Some(p),
            _ => None,
        }
    }
}

impl std::fmt::Display for HebrewAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prose(p) => write!(f, "Prose: {}", p),
            Self::Poetry(p) => write!(f, "Poetry: {}", p),
            Self::Pseudo(p) => write!(f, "Pseudo: {}", p),
        }
    }
}

impl From<ProseAccent> for HebrewAccent {
    /// Converts a [`ProseAccent`] into a [`HebrewAccent`] wrapper.
    ///
    /// # Examples
    ///
    /// Basic conversion using `.into()`:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let prose = ProseAccent::Silluq;
    /// let accent: HebrewAccent = prose.into();
    ///
    /// assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Silluq)));
    /// ```
    ///
    /// Using explicit `From::from()` syntax:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::from(ProseAccent::Atnach);
    ///
    /// assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Atnach)));
    /// ```
    ///
    /// Type inference works when the target type is clear from context:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// fn accept_hebrew(accent: impl Into<HebrewAccent>) {
    ///     // Function body...
    /// }
    ///
    /// // No need to call .into() explicitly here!
    /// accept_hebrew(ProseAccent::Segolta);
    /// ```
    ///
    /// Multiple conversions in sequence (e.g., collecting into vectors):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let prose_accents = vec![
    ///     ProseAccent::Silluq,
    ///     ProseAccent::Atnach,
    ///     ProseAccent::Revia,
    /// ];
    ///
    /// let hebrew_accents: Vec<HebrewAccent> = prose_accents
    ///     .into_iter()
    ///     .map(|p| p.into())
    ///     .collect();
    ///
    /// assert_eq!(hebrew_accents.len(), 3);
    /// ```
    fn from(a: ProseAccent) -> Self {
        HebrewAccent::Prose(a)
    }
}

/// Converts a [`PoetryAccent`] into a [`HebrewAccent`] by wrapping it in the `Poetry` variant.
///
/// # Examples
///
/// Wrapping a poetry accent:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PoetryAccent};
///
/// let poetry = PoetryAccent::Atnach;
/// let accent: HebrewAccent = poetry.into();
///
/// assert!(matches!(accent, HebrewAccent::Poetry(PoetryAccent::Atnach)));
/// ```
///
/// Using explicit `From` trait:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PoetryAccent};
/// use std::convert::From;
///
/// let poetry = PoetryAccent::Silluq;
/// let accent = HebrewAccent::from(poetry);
///
/// assert!(matches!(accent, HebrewAccent::Poetry(_)));
/// ```
impl From<PoetryAccent> for HebrewAccent {
    fn from(a: PoetryAccent) -> Self {
        HebrewAccent::Poetry(a)
    }
}

/// Converts a [`PseudoAccent`] into a [`HebrewAccent`] by wrapping it in the `Pseudo` variant.
/// # Examples
///
/// Using `.into()` for implicit conversion:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// let pseudo = PseudoAccent::SophPasuq;
/// let accent: HebrewAccent = pseudo.into();
///
/// assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::SophPasuq)));
/// ```
///
/// Using `From::from()` explicitly:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// let accent = HebrewAccent::from(PseudoAccent::Maqqeph);
///
/// assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::Maqqeph)));
/// ```
///
/// Automatic type coercion in function arguments:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// fn accepts_hebrew(accent: impl Into<HebrewAccent>) {}
///
/// // No explicit .into() needed at call site
/// accepts_hebrew(PseudoAccent::Paseq);
/// ```
impl From<PseudoAccent> for HebrewAccent {
    fn from(a: PseudoAccent) -> Self {
        HebrewAccent::Pseudo(a)
    }
}

/// All variants of the Hebrew Prose Accents
/// 18 Disjunctives and 11 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
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
    pub const LEN: usize = 28;
    /// Indication of how an strong an accent is (relative speaking)
    ///
    /// The stronger, the longer the pause/break when reading
    /// The strongest accent has a relative strength of 1
    ///
    /// Note: For now all Hebrew Accents have this property
    ///       However it is only valid for DISJUNCTIVE accents!
    #[inline]
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

impl std::fmt::Display for ProseAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}), meaning: {}", self.english_name(), self.hebrew_name(), self.hebrew_concept())
    }
}


/// All variants of the Hebrew Poetry Accents
/// 12 Disjunctives and 12 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
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
    pub const LEN: usize = 23;
    /// Indication of how an strong an accent is (relative speaking)
    ///
    /// The stronger, the longer the pause/break when reading
    /// The strongest accent has a relative strength of 1
    ///
    /// Note: For now all Hebrew Accents have this property
    ///       However it is only valid for DISJUNCTIVE accents!
    #[inline]
    pub fn relative_strength(self) -> u8 {
        // Discriminants start at 0; we want 1‑based relative_strengths.
        BHS_POETRY_RANK_MAP[self as usize]
    }
}

impl std::fmt::Display for PoetryAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}), meaning: {}", self.english_name(), self.hebrew_name(), self.hebrew_concept())
    }
}


/// Syntactic markers associated with biblical Hebrew cantillation but distinct from true accents.
///
/// `PseudoAccent` values represent structural symbols that influence accent placement without
/// carrying independent melodic contour. They govern phrase boundaries, word grouping, and
/// punctuation within the Masoretic text tradition.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PseudoAccent {
    #[default]
    /// Marks the end of a verse or sentence (Hebrew: סוֹף פָּסוּק).
    /// Equivalent to a terminal period; signals final pause despite lacking its own melody.
    ///
    /// **Note:** Contrary to intuition, `Silluq` and not `SophPasuq` designates official verse endings
    /// in standard BHS texts. `SophPasuq` may be absent even at valid verse boundaries in some rare cases.
    SophPasuq,

    /// Joins multiple words into a single phonological unit (Hebrew: מַקָּף).
    /// Functions as a hyphen: suppresses independent accents on joined words, causing
    /// accent shifts to the rightmost constituent.
    Maqqeph,

    /// Separates adjacent cantillation marks (Hebrew: פָּשְׁק).
    /// Prevents conflation of neighboring accents where disambiguation is required;
    /// never appears as an standalone accent.
    Paseq,
}

impl PseudoAccent {
    /// Total count of all pseudo accents
    pub const LEN: usize = 3;
    /// Indicates a level of importance
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

impl std::fmt::Display for PseudoAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}), meaning: {}", self.english_name(), self.hebrew_name(), self.hebrew_concept())
    }
}
/// Used for retrieving information
pub trait Accent: Copy + Sized {
    /// Hebrew name of the Hebrew Accent
    fn hebrew_name(self) -> &'static str;
    /// hebrew_concept of the Hebrew name
    fn hebrew_concept(self) -> &'static str;
    /// English name of the Hebrew Accent
    fn english_name(self) -> &'static str;
    /// Hebrew Accent type
    //fn accent_type(self) -> Option<AccentType> ;
    /// category of the Hebrew Accent
    //fn category(self) -> Option<AccentCategory> ;
    /// word-stress of the Hebrew Accent
    //fn word_stress(self) -> Option<WordStress> ;
    //self.details().accent_meta_data.and_then(|add| add.word_stress)
    /// number of UTF-8 code points of the Hebrew Accent
    fn number_of_symbols(self) -> u8;
    /// Returns any accent_meta_data notes or context about this accent, if available.
    fn notes(self) -> Option<&'static str>;

    /// Indicates the relative strength where 1 represents the strongest accent.
    fn relative_strength(self) -> u8;

    /// indicates the relative_strength of a selected accent (1 is the strongest)
    fn group_level(self) -> Option<GroupLevel>;
}

impl Accent for HebrewAccent {
    fn hebrew_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_name(),
            HebrewAccent::Poetry(p) => p.hebrew_name(),
            HebrewAccent::Pseudo(p) => p.hebrew_name(),
        }
    }

    fn hebrew_concept(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_concept(),
            HebrewAccent::Poetry(p) => p.hebrew_concept(),
            HebrewAccent::Pseudo(p) => p.hebrew_concept(),
        }
    }

    fn english_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.english_name(),
            HebrewAccent::Poetry(p) => p.english_name(),
            HebrewAccent::Pseudo(p) => p.english_name(),
        }
    }

    fn number_of_symbols(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.number_of_symbols(),
            HebrewAccent::Poetry(p) => p.number_of_symbols(),
            HebrewAccent::Pseudo(p) => p.number_of_symbols(),
        }
    }
    fn notes(self) -> Option<&'static str> {
        match self {
            HebrewAccent::Prose(p) => p.notes(),
            HebrewAccent::Poetry(p) => p.notes(),
            HebrewAccent::Pseudo(p) => p.notes(),
        }
    }
    // #[inline]
    fn relative_strength(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }

    // #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self).and_then(|g| g.into_public_level())
    }
}

impl Accent for ProseAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        PROSE_ACCENT_TABLE[self as usize].english_name
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PROSE_ACCENT_TABLE[self as usize].hebrew_name
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PROSE_ACCENT_TABLE[self as usize].hebrew_concept
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        if PROSE_ACCENT_TABLE[self as usize]
            .cantillation_symbol
            .secondary_mark
            .is_none()
        {
            1
        } else {
            2
        }
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PROSE_ACCENT_TABLE[self as usize].notes
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
}

impl Accent for PoetryAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        POETRY_ACCENT_TABLE[self as usize].english_name
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        POETRY_ACCENT_TABLE[self as usize].hebrew_name
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        POETRY_ACCENT_TABLE[self as usize].hebrew_concept
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        if POETRY_ACCENT_TABLE[self as usize]
            .cantillation_symbol
            .secondary_mark
            .is_none()
        {
            1
        } else {
            2
        }
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        POETRY_ACCENT_TABLE[self as usize].notes
    }

    #[inline]
    fn relative_strength(self) -> u8 {
        BHS_POETRY_RANK_MAP[self as usize]
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
}

impl Accent for PseudoAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self as usize].english_name
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self as usize].hebrew_name
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self as usize].hebrew_concept
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        1
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PSEUDO_ACCENT_TABLE[self as usize].notes
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        None
    }
}

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AccentInformation {
    /// Official Hebrew name of the accent according to BHS
    pub(crate) hebrew_name: &'static str,
    /// Semantic meaning of the Hebrew term.
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated English name of the accent.
    pub(crate) english_name: &'static str,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Contextual notes or scholarly commentary.
    pub(crate) notes: Option<&'static str>,
    /// Optional alternate identifiers for hebrew_name, hebrew_concept, english_name
    pub(crate) alternate_names: Option<AlternateNames>,
    /// Indicates the accent type (Primary, Secondary)
    pub(crate) accent_type: Option<AccentType>,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) category: Option<AccentCategory>,
    /// Indicates if the accent is on the stressed syllable
    pub(crate) word_stress: Option<WordStress>,
}

/// Optional alternate representations for an accent.
/// Used by some scholars
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AlternateNames {
    /// Transliterated English name for the hebrew name
    pub(crate) english_name: &'static str,
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
}

/// Struct containing the cantillation symbol of a Hebrew Accent
/// Which may consist of one or (max) two cantillation marks
/// One cantilation mark is one UTF8 code point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePointInfo,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePointInfo>,
}

/// Details on a specific UTF-8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Utf8CodePointInfo {
    /// UTF-8 code-point id, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_value: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub(crate) name: &'static str,
    /// The symbol of the UTF-8 code-point
    pub(crate) symbol: &'static str,
    /// The position of the code-point in relation to the consonant
    pub(crate) position: CodePointPosition,
    /// An array containing information of various Jewish traditions
    pub(crate) traditions: &'static [Tradition],
}
/// Accents names according one of four Hebrew Traditions
///
/// Biblical Hebrew does not have a single, universal pronunciation.
/// It has been transmitted through four principal reading traditions,
/// each of which handles stress placement differently.
///
/// The four Traditions:
/// - Ashkenazi (the Eastern European tradition)
/// - Sephardi (the Iberian and North African tradition)
/// - Yemenite (the most archaic, from the Yemenite Jewish community)
/// - Italian (the tradition of the Italian Jewish community)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum Tradition {
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
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum AccentCategory {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
}

/// Hebrew Accent types (Primary, secondary_mark, None)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum AccentType {
    #[default]
    /// Indicates that the Accent is Primary Accent
    Primary,
    /// Secondary Accent e.g. Meayla and Meteg
    Secondary,
    // Used for Pseudo Accents
    //None,
}

/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum CodePointPosition {
    /// UTF-8 code point is located above the consonant
    Above,
    /// UTF-8 code point is located after the consonant
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
    /// UTF-8 code point is located in between two words
    InBetween,
    /// UTF-8 code point is located under the consonant
    #[default]
    Under,
}

/// WordStress, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum WordStress {
    #[default]
    /// ImPositive: The accent is located above the stressed syllable
    Im,
    /// PostPositive: The accent is NOT located above the stressed syllable, but at the very end of the word
    Post,
    /// PrePositive: Accent is NOT located above the stressed syllable, but at the very beginning of the word
    Pre,
}

/// Disjunctive accent hierarchy level following Futato's classification system.
///
/// Ranges from 1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// # Example
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, GroupLevel};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.group_level(), Some(GroupLevel::Level1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.group_level(), None);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GroupLevel {
    /// Primary disjunctive tier — creates major clause/phrasal breaks
    Level1 = 1, // value represents group number for extension in future
    /// Secondary disjunctive tier — subordinate phrase boundaries
    Level2,
    /// Tertiary disjunctive tier — minor phrasal divisions
    Level3,
    /// Quaternary disjunctive tier — fine-grained subdivisions
    Level4,
}

impl GroupLevel {
    /// Raw numeric strength value (1 = strongest disjunctive)
    pub const fn value(self) -> u8 {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
        }
    }

    /// Human-readable description of hierarchy tier
    pub const fn description(self) -> &'static str {
        match self {
            Self::Level1 => "Primary disjunctive (major clause break)",
            Self::Level2 => "Secondary disjunctive (phrase boundary)",
            Self::Level3 => "Tertiary disjunctive (minor division)",
            Self::Level4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
}

impl std::fmt::Display for GroupLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Level1 => write!(f, "Tier 1 (Primary disjunctive)"),
            Self::Level2 => write!(f, "Tier 2 (Secondary disjunctive)"),
            Self::Level3 => write!(f, "Tier 3 (Tertiary disjunctive)"),
            Self::Level4 => write!(f, "Tier 4 (Quaternary disjunctive)"),
        }
    }
}

/// Full Futato hierarchy classification with prose/poetry distinction.
///
/// **Internal use only**—do not rely on this type publicly as it may change
/// without semver warning. Use [`super::GroupLevel`] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum DisjunctiveGroup {
    ProseLevel1, // Fixed variants, no number needed
    ProseLevel2,
    ProseLevel3,
    ProseLevel4,
    PoetryLevel1,
    PoetryLevel2,
    PoetryLevel3, // Max tier differs between systems
}

impl DisjunctiveGroup {
    /// Convert to simplified public GroupLevel
    pub(crate) const fn into_public_level(self) -> Option<GroupLevel> {
        match self {
            Self::ProseLevel1 | Self::PoetryLevel1 => Some(GroupLevel::Level1),
            Self::ProseLevel2 | Self::PoetryLevel2 => Some(GroupLevel::Level2),
            Self::ProseLevel3 | Self::PoetryLevel3 => Some(GroupLevel::Level3),
            Self::ProseLevel4 => Some(GroupLevel::Level4),
        }
    }

    // Return just the raw group number regardless of prose/poetry origin
    // pub(crate) const fn group_number(&self) -> u8 {
    //     match self {
    //         Self::ProseGroup(n) | Self::PoetryGroup(n) => *n,
    //     }
    // }
}

/// Lookup logic for accent hierarchy (private function)
pub(crate) fn resolve_disjunctive_group(accent: HebrewAccent) -> Option<DisjunctiveGroup> {
    match accent {
        HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Prose(ProseAccent::Atnach) => {
            Some(DisjunctiveGroup::ProseLevel1)
        }

        HebrewAccent::Prose(ProseAccent::Segolta)
        | HebrewAccent::Prose(ProseAccent::Shalshelet)
        | HebrewAccent::Prose(ProseAccent::ZaqephQatan)
        | HebrewAccent::Prose(ProseAccent::ZaqephGadol)
        | HebrewAccent::Prose(ProseAccent::Tiphcha) => Some(DisjunctiveGroup::ProseLevel2),

        HebrewAccent::Prose(ProseAccent::Revia)
        | HebrewAccent::Prose(ProseAccent::Zarqa)
        | HebrewAccent::Prose(ProseAccent::Pashta)
        | HebrewAccent::Prose(ProseAccent::Tevir)
        | HebrewAccent::Prose(ProseAccent::Yetiv) => Some(DisjunctiveGroup::ProseLevel3),

        HebrewAccent::Prose(ProseAccent::Geresh)
        | HebrewAccent::Prose(ProseAccent::Gershayim)
        | HebrewAccent::Prose(ProseAccent::Pazer)
        | HebrewAccent::Prose(ProseAccent::PazerGadol)
        | HebrewAccent::Prose(ProseAccent::TelishaGedolah)
        | HebrewAccent::Prose(ProseAccent::Legarmeh) => Some(DisjunctiveGroup::ProseLevel4),

        HebrewAccent::Poetry(PoetryAccent::Silluq)
        | HebrewAccent::Poetry(PoetryAccent::OlehWeYored)
        | HebrewAccent::Poetry(PoetryAccent::Atnach) => Some(DisjunctiveGroup::PoetryLevel1),

        HebrewAccent::Poetry(PoetryAccent::ReviaGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)
        | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaQaton)
        | HebrewAccent::Poetry(PoetryAccent::Tsinnor)
        | HebrewAccent::Poetry(PoetryAccent::Dechi) => Some(DisjunctiveGroup::PoetryLevel2),

        HebrewAccent::Poetry(PoetryAccent::Pazer)
        | HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
        | HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => Some(DisjunctiveGroup::PoetryLevel3),
        _ => None, // conjunctives and pseudo-accents lack hierarchy
    }
}


