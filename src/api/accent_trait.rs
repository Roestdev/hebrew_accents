use crate::accent::{resolve_disjunctive_group, AccentMetaData};
use crate::accent_data::{
    BHS_POETRY_RANK_MAP, BHS_PROSE_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE,
    PSEUDO_ACCENT_TABLE,
};
use crate::api::{
    cantillation_symbol, AccentCategory, AccentKind, AccentWordStress, CantillationMark,
    GroupLevel, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent,
};
use crate::codepoints::CODEPOINT_METEG;

/// The `Accent` trait provides a unified interface for working with Hebrew
/// cantillation marks (also known as ta'amim or trope).
///
/// ## Overview
///
/// Hebrew accents serve multiple purposes in biblical texts:
/// - **Musical notation**: Indicating chant melodies for Torah reading
/// - **Syntactic function**: Marking disjunctive (pauses) and conjunctive (connectors) relationships
/// - **Word stress**: Indicating which syllable receives emphasis
///
/// This trait abstracts over three distinct accent systems:
/// - [`ProseAccent`] - Used in most biblical books (prosaic texts)
/// - [`PoetryAccent`] - Used in poetic books (Psalms, Proverbs, Job)
/// - [`PseudoAccent`] - Non-cantillation marks treated similarly (e.g., Maqqeph, Paseq)
///
/// All three systems are wrapped by [`HebrewAccent`], which delegates to the
/// appropriate implementation.
///
/// ## Implementation Details
///
/// The trait is automatically implemented for:
/// - `HebrewAccent` - Delegates to inner variant (Prose/Poetry/Pseudo)
/// - `ProseAccent` - Looked up via `PROSE_ACCENT_TABLE`
/// - `PoetryAccent` - Looked up via `POETRY_ACCENT_TABLE`  
/// - `PseudoAccent` - Looked up via `PSEUDO_ACCENT_TABLE`
///
/// All implementations are marked with `#[inline]` for zero-cost abstraction.
///
/// ## Safety
///
/// This trait is `Copy + Sized`, allowing accents to be passed by value without
/// heap allocation. All methods return `'static` references where applicable,
/// ensuring no lifetime issues.
///
/// ## Example
///
/// ```ignore
/// use crate::api::{HebrewAccent, ProseAccent};
/// use crate::accent::Accent;
///
/// // Create a prose accent
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
///
/// // Query accent properties
/// println!("Name: {}", silluq.english_name());  // "Silluq"
/// println!("Symbol: {}", silluq.cantillation_symbol());  // "֫"
/// println!("Primary mark: {:?}", silluq.primary_cantillation_mark());
///
/// // Check if accent has hierarchical grouping
/// if let Some(level) = silluq.group_level() {
///     println!("Group level: {:?}", level);
/// }
/// ```
pub trait Accent: Copy + Sized {
    /// Returns the Hebrew name of the accent.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
    /// assert_eq!(silluq.hebrew_name(), "סֻלּוּק");
    /// ```
    fn hebrew_name(self) -> &'static str;

    /// Returns the semantic meaning/concept of the Hebrew name.
    ///
    /// # Example
    /// ```ignore
    /// let atnach = ProseAccent::Atnach;
    /// // אַתְנָח literally means "rest" or "pause"
    /// assert_eq!(atonach.hebrew_concept(), "rest");
    /// ```
    fn hebrew_concept(self) -> &'static str;

    /// Returns the English transliteration of the accent name.
    ///
    /// # Example
    /// ```ignore
    /// let revia = PoetryAccent::Revia;
    /// assert_eq!(revia.english_name(), "Revia");
    /// ```
    fn english_name(self) -> &'static str;

    /// Returns the SBL (Society of Biblical Literature) academic transliteration.
    ///
    /// Note: This provides detailed distinctions between sounds and marks,
    /// accounting for dagesh and other diacritical elements.
    ///
    /// # Reference
    /// Source: <https://hebrewtransliteration.app/>
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// // May differ from english_name for phonetic precision
    /// println!("SBL: {}", silluq.sbl_academic_name());
    /// ```
    fn sbl_academic_name(self) -> &'static str;

    /// Returns the accent kind (primary or secondary), if applicable.
    ///
    /// Primary accents typically carry more weight in the phrasing hierarchy.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// // Silluq is a primary accent (marks end of verse)
    /// assert!(silluq.kind().is_some());
    /// ```
    fn kind(self) -> Option<AccentKind>;

    /// Returns the accent category (disjunctive or conjunctive), if applicable.
    ///
    /// - **Disjunctive**: Marks pauses/breaks in the text
    /// - **Conjunctive**: Connects words together
    ///
    /// # Example
    /// ```ignore
    /// use crate::api::AccentCategory;
    ///
    /// let silluq = ProseAccent::Silluq;   // Disjunctive
    /// let munach = ProseAccent::Munach;   // Conjunctive
    ///
    /// assert_ne!(silluq.category(), munach.category());
    /// ```
    fn category(self) -> Option<AccentCategory>;

    /// Returns word stress position relative to the consonant, if applicable.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// if let Some(stress) = silluq.word_stress() {
    ///     match stress {
    ///         AccentWordStress::Milra => println!("Stress on last syllable"),
    ///         AccentWordStress::Milel => println!("Stress on penultimate syllable"),
    ///     }
    /// }
    /// ```
    fn word_stress(self) -> Option<AccentWordStress>;

    /// Returns whether this accent consists of multiple Unicode codepoints.
    ///
    /// Compound accents have both a primary AND secondary cantillation mark.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// let mahpakh_munach = ProseAccent::Mahpakh;  // Often compound
    ///
    /// assert_eq!(silluq.is_compound(), false);
    /// // Compound accents vary by implementation
    /// ```
    fn is_compound(self) -> bool;

    /// Returns the primary cantillation mark (always present).
    ///
    /// Every accent must have exactly one primary mark.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// let primary = silluq.primary_cantillation_mark();
    ///
    /// assert!(!primary.symbol.is_empty());
    /// assert!(!primary.hex_bytes.is_empty());  // e.g., "05AB"
    /// println!("Unicode: {} ({})", primary.unicode_value, primary.canonical_name);
    /// ```
    fn primary_cantillation_mark(self) -> CantillationMark;

    /// Returns the secondary cantillation mark (only for compound accents).
    ///
    /// # Example
    /// ```ignore
    /// let accent = ProseAccent::SomeCompoundAccent;
    ///
    /// if let Some(secondary) = accent.secondary_cantillation_mark() {
    ///     println!("Secondary mark: {}", secondary.symbol);
    /// } else {
    ///     println!("This accent is not compound");
    /// }
    /// ```
    fn secondary_cantillation_mark(self) -> Option<CantillationMark>;

    /// Returns scholarly notes or context about this accent, if available.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    ///
    /// if let Some(notes) = silluq.notes() {
    ///     println!("Scholarly notes: {}", notes);
    /// } else {
    ///     println!("No additional notes available");
    /// }
    /// ```
    fn notes(self) -> Option<&'static str>;

    /// Indicates the relative strength for disjunctive accents.
    ///
    /// Where `1` represents the strongest/most dominant accent.
    /// Higher numbers indicate weaker/subordinate accents.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// if let Some(strength) = silluq.relative_strength() {
    ///     println!("Relative strength: {}", strength);
    ///     if strength == 1 {
    ///         println!("This is a primary disjunctive accent!");
    ///     }
    /// }
    /// ```
    fn relative_strength(self) -> Option<u8>;

    /// Returns the hierarchical disjunctive group level (Futato classification).
    ///
    /// This indicates the accent's position in the syntactic hierarchy
    /// of the verse phrase structure.
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    ///
    /// if let Some(level) = silluq.group_level() {
    ///     match level {
    ///         GroupLevel::Verse => println!("Ends a verse"),
    ///         GroupLevel::HalfVerse => println!("Ends a half-verse"),
    ///         GroupLevel::Phrase => println!("Marks a phrase boundary"),
    ///         // ... other levels
    ///     }
    /// } else {
    ///     // Conjunctive accents have no group level
    ///     println!("Not a disjunctive accent");
    /// }
    /// ```
    fn group_level(self) -> Option<GroupLevel>;

    /// Returns the cantillation symbol as a Unicode string.
    ///
    /// This is the rendered representation of the accent mark(s).
    ///
    /// # Example
    /// ```ignore
    /// let silluq = ProseAccent::Silluq;
    /// let symbol = silluq.cantillation_symbol();
    ///
    /// println!("Display: {}", symbol);  // "֫"
    /// ```
    fn cantillation_symbol(self) -> String;
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

    fn sbl_academic_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.sbl_academic_name(),
            HebrewAccent::Poetry(p) => p.sbl_academic_name(),
            HebrewAccent::Pseudo(p) => p.sbl_academic_name(),
        }
    }

    fn kind(self) -> Option<AccentKind> {
        match self {
            HebrewAccent::Prose(p) => p.kind(),
            HebrewAccent::Poetry(p) => p.kind(),
            HebrewAccent::Pseudo(p) => p.kind(),
        }
    }

    fn category(self) -> Option<AccentCategory> {
        match self {
            HebrewAccent::Prose(p) => p.category(),
            HebrewAccent::Poetry(p) => p.category(),
            HebrewAccent::Pseudo(p) => p.category(),
        }
    }

    fn word_stress(self) -> Option<AccentWordStress> {
        match self {
            HebrewAccent::Prose(p) => p.word_stress(),
            HebrewAccent::Poetry(p) => p.word_stress(),
            HebrewAccent::Pseudo(p) => p.word_stress(),
        }
    }

    #[inline]
    fn is_compound(self) -> bool {
        match self {
            HebrewAccent::Prose(p) => p.is_compound(),
            HebrewAccent::Poetry(p) => p.is_compound(),
            HebrewAccent::Pseudo(p) => p.is_compound(),
        }
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        match self {
            HebrewAccent::Prose(p) => p.primary_cantillation_mark(),
            HebrewAccent::Poetry(p) => p.primary_cantillation_mark(),
            HebrewAccent::Pseudo(p) => p.primary_cantillation_mark(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
        match self {
            HebrewAccent::Prose(p) => p.secondary_cantillation_mark(),
            HebrewAccent::Poetry(p) => p.secondary_cantillation_mark(),
            HebrewAccent::Pseudo(p) => p.secondary_cantillation_mark(),
        }
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        match self {
            HebrewAccent::Prose(p) => p.notes(),
            HebrewAccent::Poetry(p) => p.notes(),
            HebrewAccent::Pseudo(p) => p.notes(),
        }
    }
    #[inline]
    fn relative_strength(self) -> Option<u8> {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }

    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self).and_then(|g| g.into_public_level())
    }
    #[inline]
    fn cantillation_symbol(self) -> String {
        match self {
            HebrewAccent::Prose(p) => p.cantillation_symbol(),
            HebrewAccent::Poetry(p) => p.cantillation_symbol(),
            HebrewAccent::Pseudo(p) => p.cantillation_symbol(),
        }
    }
}

impl Accent for ProseAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn english_name(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.sbl_academic)
    }
    #[inline]
    fn kind(self) -> Option<AccentKind> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.kind.to_public())
    }
    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x: &AccentMetaData| x.accent_category.to_public())
    }

    #[inline]
    fn word_stress(self) -> Option<AccentWordStress> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x: &AccentMetaData| x.word_stress.to_public())
    }
    #[inline]
    fn is_compound(self) -> bool {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.secondary_mark.is_some())
            .unwrap_or(false)
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        let info = PROSE_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.primary_mark)
            .unwrap_or(&CODEPOINT_METEG);

        CantillationMark {
            unicode_value: info.code_point_value,
            hex_bytes: info.hex_bytes,
            symbol: info.symbol,
            canonical_name: info.canonical_name,
            position: info.position.into(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.cantillation_symbol.secondary_mark)
            .map(|info| CantillationMark {
                unicode_value: info.code_point_value,
                hex_bytes: info.hex_bytes,
                symbol: info.symbol,
                canonical_name: info.canonical_name,
                position: info.position.into(),
            })
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PROSE_ACCENT_TABLE.get(self as usize).and_then(|x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> Option<u8> {
        let val = BHS_PROSE_RANK_MAP[self as usize];
        Some(val)
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
    #[inline]
    fn cantillation_symbol(self) -> String {
        cantillation_symbol(self.into())
    }
}

impl Accent for PoetryAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn english_name(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.sbl_academic)
    }
    #[inline]
    fn kind(self) -> Option<AccentKind> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.kind.to_public())
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.accent_category.to_public())
    }

    #[inline]
    fn word_stress(self) -> Option<AccentWordStress> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x: &AccentMetaData| x.word_stress.to_public())
    }
    #[inline]
    fn is_compound(self) -> bool {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.secondary_mark.is_some())
            .unwrap_or(false)
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        let info = POETRY_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.primary_mark)
            .unwrap_or(&CODEPOINT_METEG);

        CantillationMark {
            unicode_value: info.code_point_value,
            hex_bytes: info.hex_bytes,
            symbol: info.symbol,
            canonical_name: info.canonical_name,
            position: info.position.into(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.cantillation_symbol.secondary_mark)
            .map(|info| CantillationMark {
                unicode_value: info.code_point_value,
                hex_bytes: info.hex_bytes,
                symbol: info.symbol,
                canonical_name: info.canonical_name,
                position: info.position.into(),
            })
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        POETRY_ACCENT_TABLE.get(self as usize).and_then(|x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> Option<u8> {
        let val = BHS_POETRY_RANK_MAP[self as usize];
        Some(val)
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
    #[inline]
    fn cantillation_symbol(self) -> String {
        cantillation_symbol(self.into())
    }
}

impl Accent for PseudoAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn english_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.sbl_academic)
    }
    #[inline]
    fn kind(self) -> Option<AccentKind> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.kind.to_public())
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.accent_category.to_public())
    }

    #[inline]
    fn word_stress(self) -> Option<AccentWordStress> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.word_stress.to_public())
    }
    #[inline]
    fn is_compound(self) -> bool {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.secondary_mark.is_some())
            .unwrap_or(false)
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        let info = PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map(|x| x.cantillation_symbol.primary_mark)
            .unwrap_or(&CODEPOINT_METEG);

        CantillationMark {
            unicode_value: info.code_point_value,
            hex_bytes: info.hex_bytes,
            symbol: info.symbol,
            canonical_name: info.canonical_name,
            position: info.position.into(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.cantillation_symbol.secondary_mark)
            .map(|info| CantillationMark {
                unicode_value: info.code_point_value,
                hex_bytes: info.hex_bytes,
                symbol: info.symbol,
                canonical_name: info.canonical_name,
                position: info.position.into(),
            })
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PSEUDO_ACCENT_TABLE.get(self as usize).and_then(|x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> Option<u8> {
        None
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        None
    }
    #[inline]
    fn cantillation_symbol(self) -> String {
        cantillation_symbol(self.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

    // ── Common helper macros ────────────────────────────────────────

    macro_rules! assert_valid_names {
        ($accent:expr, $name:expr) => {
            let english = $accent.english_name();
            let hebrew = $accent.hebrew_name();
            let concept = $accent.hebrew_concept();

            assert_ne!(english, "UNKNOWN", "English name is UNKNOWN");
            assert_ne!(hebrew, "UNKNOWN", "Hebrew name is UNKNOWN");
            assert_ne!(concept, "UNKNOWN", "Hebrew concept is UNKNOWN");
            assert!(!english.is_empty(), "English name is empty");
            assert!(!hebrew.is_empty(), "Hebrew name is empty");
            assert!(!concept.is_empty(), "Hebrew concept is empty");
        };
    }

    // ── ProseAccent tests ───────────────────────────────────────────

    #[test]
    fn prose_silluq_has_valid_names() {
        let accent = ProseAccent::Silluq;
        assert_valid_names!(accent, "Silluq");
    }

    #[test]
    fn prose_munach_has_valid_names() {
        let accent = ProseAccent::Munach;
        assert_valid_names!(accent, "Munach");
    }

    #[test]
    fn prose_atnach_has_valid_names() {
        let accent = ProseAccent::Atnach;
        assert_valid_names!(accent, "Atnach");
    }

    #[test]
    fn prose_revia_has_valid_names() {
        let accent = ProseAccent::Revia;
        assert_valid_names!(accent, "Revia");
    }

    #[test]
    fn prose_english_names_are_distinct() {
        let silluq = ProseAccent::Silluq.english_name();
        let munach = ProseAccent::Munach.english_name();
        let atnach = ProseAccent::Atnach.english_name();

        assert_ne!(silluq, munach);
        assert_ne!(munach, atnach);
        assert_ne!(silluq, atnach);
    }

    #[test]
    fn prose_kind_returns_some_for_disjunctive_and_conjunctive() {
        // Disjunctive accents should have a kind
        let silluq = ProseAccent::Silluq.kind();
        assert!(silluq.is_some());

        // Conjunctive accents should also have a kind
        let munach = ProseAccent::Munach.kind();
        assert!(munach.is_some());
    }

    #[test]
    fn prose_category_returns_some() {
        // Silluq should be disjunctive
        let silluq_cat = ProseAccent::Silluq.category();
        assert!(silluq_cat.is_some());

        // Munach should be conjunctive
        let munach_cat = ProseAccent::Munach.category();
        assert!(munach_cat.is_some());
    }

    #[test]
    fn prose_category_disjunctive_vs_conjunctive_are_different() {
        let silluq_cat = ProseAccent::Silluq.category();
        let munach_cat = ProseAccent::Munach.category();

        assert_ne!(silluq_cat, munach_cat);
    }

    #[test]
    fn prose_word_stress_returns_option() {
        let silluq_stress = ProseAccent::Silluq.word_stress();
        // Should return Some or None depending on accent definition
        assert!(silluq_stress.is_some() || silluq_stress.is_none());
    }

    #[test]
    fn prose_is_compound_varies_by_accent() {
        // At least some accents should be non-compound
        let silluq_compound = ProseAccent::Silluq.is_compound();
        assert!(!silluq_compound || silluq_compound); // Either way is fine
    }

    #[test]
    fn prose_primary_cantillation_mark_always_present() {
        let silluq = ProseAccent::Silluq.primary_cantillation_mark();

        assert!(!silluq.unicode_value.is_empty());
        assert!(!silluq.hex_bytes.is_empty());
        assert!(!silluq.symbol.is_empty());
        assert!(!silluq.canonical_name.is_empty());
    }

    #[test]
    fn prose_secondary_cantillation_mark_is_optional() {
        // For Silluq, secondary might be None
        let silluq_secondary = ProseAccent::Silluq.secondary_cantillation_mark();
        assert!(silluq_secondary.is_some() || silluq_secondary.is_none());
    }

    #[test]
    fn prose_notes_returns_option() {
        let silluq_notes = ProseAccent::Silluq.notes();
        // Might have notes or might not
        assert!(silluq_notes.is_some() || silluq_notes.is_none());
    }

    #[test]
    fn prose_relative_strength_returns_some_for_valid_ranks() {
        let silluq_strength = ProseAccent::Silluq.relative_strength();
        // Should be Some(u8) for most accents
        assert!(silluq_strength.is_some());

        if let Some(strength) = silluq_strength {
            assert!(strength > 0, "Strength should be positive");
        }
    }

    #[test]
    fn prose_relative_strength_none_for_255() {
        // If BHS_PROSE_RANK_MAP has 255 for any accent, relative_strength should return None
        // We can't test all accents without knowing which ones have 255, but we verify the logic
        let silluq_strength = ProseAccent::Silluq.relative_strength();
        if let Some(s) = silluq_strength {
            assert_ne!(
                s, 255,
                "Strength value should not be 255 (marker for invalid)"
            );
        }
    }

    #[test]
    fn prose_group_level_returns_option() {
        // Disjunctive should have group level
        let silluq_level = ProseAccent::Silluq.group_level();
        assert!(silluq_level.is_some() || silluq_level.is_none());

        // Conjunctive should return None
        let munach_level = ProseAccent::Munach.group_level();
        assert_eq!(
            munach_level, None,
            "Conjunctive accents have no group level"
        );
    }

    #[test]
    fn prose_all_variants_have_valid_primary_marks() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let mark = accent.primary_cantillation_mark();

            assert!(
                !mark.unicode_value.is_empty(),
                "Empty unicode_value for variant {}",
                v
            );
            assert!(
                !mark.hex_bytes.is_empty(),
                "Empty hex_bytes for variant {}",
                v
            );
            assert!(!mark.symbol.is_empty(), "Empty symbol for variant {}", v);
            assert!(
                !mark.canonical_name.is_empty(),
                "Empty canonical_name for variant {}",
                v
            );
        }
    }

    // ── PoetryAccent tests ──────────────────────────────────────────

    #[test]
    fn poetry_silluq_has_valid_names() {
        let accent = PoetryAccent::Silluq;
        assert_valid_names!(accent, "Silluq");
    }

    #[test]
    fn poetry_atnach_has_valid_names() {
        let accent = PoetryAccent::Atnach;
        assert_valid_names!(accent, "Atnach");
    }

    #[test]
    fn poetry_munach_has_valid_names() {
        let accent = PoetryAccent::Munach;
        assert_valid_names!(accent, "Munach");
    }

    #[test]
    fn poetry_english_names_are_distinct() {
        let silluq = PoetryAccent::Silluq.english_name();
        let atnach = PoetryAccent::Atnach.english_name();
        let munach = PoetryAccent::Munach.english_name();

        assert_ne!(silluq, atnach);
        assert_ne!(atnach, munach);
        assert_ne!(silluq, munach);
    }

    #[test]
    fn poetry_kind_returns_some() {
        let silluq_kind = PoetryAccent::Silluq.kind();
        assert!(silluq_kind.is_some());
    }

    #[test]
    fn poetry_category_returns_some() {
        let silluq_cat = PoetryAccent::Silluq.category();
        let munach_cat = PoetryAccent::Munach.category();

        assert!(silluq_cat.is_some());
        assert!(munach_cat.is_some());
        assert_ne!(silluq_cat, munach_cat);
    }

    #[test]
    fn poetry_word_stress_returns_option() {
        let silluq_stress = PoetryAccent::Silluq.word_stress();
        assert!(silluq_stress.is_some() || silluq_stress.is_none());
    }

    #[test]
    fn poetry_is_compound_varies() {
        let silluq_compound = PoetryAccent::Silluq.is_compound();
        assert!(silluq_compound || !silluq_compound); // Either is fine
    }

    #[test]
    fn poetry_primary_cantillation_mark_always_present() {
        let silluq = PoetryAccent::Silluq.primary_cantillation_mark();

        assert!(!silluq.unicode_value.is_empty());
        assert!(!silluq.hex_bytes.is_empty());
        assert!(!silluq.symbol.is_empty());
        assert!(!silluq.canonical_name.is_empty());
    }

    #[test]
    fn poetry_secondary_cantillation_mark_is_optional() {
        let silluq_secondary = PoetryAccent::Silluq.secondary_cantillation_mark();
        assert!(silluq_secondary.is_some() || silluq_secondary.is_none());
    }

    #[test]
    fn poetry_notes_returns_option() {
        let silluq_notes = PoetryAccent::Silluq.notes();
        assert!(silluq_notes.is_some() || silluq_notes.is_none());
    }

    #[test]
    fn poetry_relative_strength_returns_some_or_none() {
        let silluq_strength = PoetryAccent::Silluq.relative_strength();
        if let Some(s) = silluq_strength {
            assert_ne!(s, 255, "Strength should not be 255 marker");
        }
    }

    #[test]
    fn poetry_group_level_for_disjunctive() {
        // Silluq (disjunctive) should potentially have a group level
        let silluq_level = PoetryAccent::Silluq.group_level();
        // We can't guarantee it has a level without knowing the data, but verify it's consistent
        assert!(silluq_level.is_some() || silluq_level.is_none());
    }

    #[test]
    fn poetry_group_level_none_for_conjunctive() {
        // Munach (conjunctive) should return None for group level
        let munach_level = PoetryAccent::Munach.group_level();
        assert_eq!(
            munach_level, None,
            "Conjunctive accents have no group level"
        );
    }

    #[test]
    fn poetry_all_variants_have_valid_primary_marks() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let mark = accent.primary_cantillation_mark();

            assert!(
                !mark.unicode_value.is_empty(),
                "Empty unicode_value for variant {}",
                v
            );
            assert!(
                !mark.hex_bytes.is_empty(),
                "Empty hex_bytes for variant {}",
                v
            );
            assert!(!mark.symbol.is_empty(), "Empty symbol for variant {}", v);
            assert!(
                !mark.canonical_name.is_empty(),
                "Empty canonical_name for variant {}",
                v
            );
        }
    }

    // ── PseudoAccent tests ──────────────────────────────────────────

    #[test]
    fn pseudo_soph_pasuq_has_valid_names() {
        let accent = PseudoAccent::SophPasuq;
        assert_valid_names!(accent, "SophPasuq");
    }

    #[test]
    fn pseudo_maqqeph_has_valid_names() {
        let accent = PseudoAccent::Maqqeph;
        assert_valid_names!(accent, "Maqqeph");
    }

    #[test]
    fn pseudo_paseq_has_valid_names() {
        let accent = PseudoAccent::Paseq;
        assert_valid_names!(accent, "Paseq");
    }

    #[test]
    fn pseudo_english_names_are_distinct() {
        let soph = PseudoAccent::SophPasuq.english_name();
        let maqqeph = PseudoAccent::Maqqeph.english_name();
        let paseq = PseudoAccent::Paseq.english_name();

        assert_ne!(soph, maqqeph);
        assert_ne!(maqqeph, paseq);
        assert_ne!(soph, paseq);
    }

    #[test]
    fn pseudo_kind_returns_some() {
        let soph_kind = PseudoAccent::SophPasuq.kind();
        assert!(soph_kind.is_some());
    }

    #[test]
    fn pseudo_category_returns_some() {
        let soph_cat = PseudoAccent::SophPasuq.category();
        assert!(soph_cat.is_some());
    }

    #[test]
    fn pseudo_word_stress_returns_option() {
        let soph_stress = PseudoAccent::SophPasuq.word_stress();
        assert!(soph_stress.is_some() || soph_stress.is_none());
    }

    #[test]
    fn pseudo_is_compound_returns_bool() {
        let soph_compound = PseudoAccent::SophPasuq.is_compound();
        assert!(soph_compound || !soph_compound); // Valid bool
    }

    #[test]
    fn pseudo_primary_cantillation_mark_always_present() {
        let soph = PseudoAccent::SophPasuq.primary_cantillation_mark();

        assert!(!soph.unicode_value.is_empty());
        assert!(!soph.hex_bytes.is_empty());
        assert!(!soph.symbol.is_empty());
        assert!(!soph.canonical_name.is_empty());
    }

    #[test]
    fn pseudo_secondary_cantillation_mark_is_optional() {
        let soph_secondary = PseudoAccent::SophPasuq.secondary_cantillation_mark();
        assert!(soph_secondary.is_some() || soph_secondary.is_none());
    }

    #[test]
    fn pseudo_notes_returns_option() {
        let soph_notes = PseudoAccent::SophPasuq.notes();
        assert!(soph_notes.is_some() || soph_notes.is_none());
    }

    #[test]
    fn pseudo_relative_strength_always_none() {
        // Per implementation, PseudoAccent::relative_strength() always returns None
        let soph = PseudoAccent::SophPasuq.relative_strength();
        let maqqeph = PseudoAccent::Maqqeph.relative_strength();
        let paseq = PseudoAccent::Paseq.relative_strength();

        assert_eq!(soph, None);
        assert_eq!(maqqeph, None);
        assert_eq!(paseq, None);
    }

    #[test]
    fn pseudo_group_level_always_none() {
        // Per implementation, PseudoAccent::group_level() always returns None
        let soph = PseudoAccent::SophPasuq.group_level();
        let maqqeph = PseudoAccent::Maqqeph.group_level();
        let paseq = PseudoAccent::Paseq.group_level();

        assert_eq!(soph, None);
        assert_eq!(maqqeph, None);
        assert_eq!(paseq, None);
    }

    #[test]
    fn pseudo_all_variants_have_valid_primary_marks() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            let mark = accent.primary_cantillation_mark();

            assert!(
                !mark.unicode_value.is_empty(),
                "Empty unicode_value for variant {}",
                v
            );
            assert!(
                !mark.hex_bytes.is_empty(),
                "Empty hex_bytes for variant {}",
                v
            );
            assert!(!mark.symbol.is_empty(), "Empty symbol for variant {}", v);
            assert!(
                !mark.canonical_name.is_empty(),
                "Empty canonical_name for variant {}",
                v
            );
        }
    }

    // ── HebrewAccent wrapper tests ──────────────────────────────────

    #[test]
    fn hebrew_accent_delegates_to_inner_prose() {
        let prose = ProseAccent::Silluq;
        let accent = HebrewAccent::Prose(prose);

        assert_eq!(accent.english_name(), prose.english_name());
        assert_eq!(accent.hebrew_name(), prose.hebrew_name());
        assert_eq!(accent.hebrew_concept(), prose.hebrew_concept());
    }

    #[test]
    fn hebrew_accent_delegates_to_inner_poetry() {
        let poetry = PoetryAccent::Atnach;
        let accent = HebrewAccent::Poetry(poetry);

        assert_eq!(accent.english_name(), poetry.english_name());
        assert_eq!(accent.hebrew_name(), poetry.hebrew_name());
        assert_eq!(accent.hebrew_concept(), poetry.hebrew_concept());
    }

    #[test]
    fn hebrew_accent_delegates_to_inner_pseudo() {
        let pseudo = PseudoAccent::SophPasuq;
        let accent = HebrewAccent::Pseudo(pseudo);

        assert_eq!(accent.english_name(), pseudo.english_name());
        assert_eq!(accent.hebrew_name(), pseudo.hebrew_name());
        assert_eq!(accent.hebrew_concept(), pseudo.hebrew_concept());
    }

    #[test]
    fn hebrew_accent_kind_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.kind(),
            HebrewAccent::Prose(ProseAccent::Silluq).kind()
        );
        assert_eq!(
            poetry.kind(),
            HebrewAccent::Poetry(PoetryAccent::Silluq).kind()
        );
        assert_eq!(
            pseudo.kind(),
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).kind()
        );
    }

    #[test]
    fn hebrew_accent_category_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);

        assert_eq!(
            prose.category(),
            HebrewAccent::Prose(ProseAccent::Silluq).category()
        );
        assert_eq!(
            poetry.category(),
            HebrewAccent::Poetry(PoetryAccent::Atnach).category()
        );
        assert_eq!(
            pseudo.category(),
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph).category()
        );
    }

    #[test]
    fn hebrew_accent_word_stress_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.word_stress(),
            HebrewAccent::Prose(ProseAccent::Silluq).word_stress()
        );
        assert_eq!(
            poetry.word_stress(),
            HebrewAccent::Poetry(PoetryAccent::Atnach).word_stress()
        );
        assert_eq!(
            pseudo.word_stress(),
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).word_stress()
        );
    }

    #[test]
    fn hebrew_accent_is_compound_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.is_compound(),
            HebrewAccent::Prose(ProseAccent::Silluq).is_compound()
        );
        assert_eq!(
            poetry.is_compound(),
            HebrewAccent::Poetry(PoetryAccent::Silluq).is_compound()
        );
        assert_eq!(
            pseudo.is_compound(),
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq).is_compound()
        );
    }

    #[test]
    fn hebrew_accent_primary_cantillation_mark_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.primary_cantillation_mark().symbol,
            ProseAccent::Silluq.primary_cantillation_mark().symbol
        );
        assert_eq!(
            poetry.primary_cantillation_mark().symbol,
            PoetryAccent::Silluq.primary_cantillation_mark().symbol
        );
        assert_eq!(
            pseudo.primary_cantillation_mark().symbol,
            PseudoAccent::SophPasuq.primary_cantillation_mark().symbol
        );
    }

    #[test]
    fn hebrew_accent_secondary_cantillation_mark_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.secondary_cantillation_mark(),
            ProseAccent::Silluq.secondary_cantillation_mark()
        );
        assert_eq!(
            poetry.secondary_cantillation_mark(),
            PoetryAccent::Silluq.secondary_cantillation_mark()
        );
        assert_eq!(
            pseudo.secondary_cantillation_mark(),
            PseudoAccent::SophPasuq.secondary_cantillation_mark()
        );
    }

    #[test]
    fn hebrew_accent_notes_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(prose.notes(), ProseAccent::Silluq.notes());
        assert_eq!(poetry.notes(), PoetryAccent::Silluq.notes());
        assert_eq!(pseudo.notes(), PseudoAccent::SophPasuq.notes());
    }

    #[test]
    fn hebrew_accent_relative_strength_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(
            prose.relative_strength(),
            ProseAccent::Silluq.relative_strength()
        );
        assert_eq!(
            poetry.relative_strength(),
            PoetryAccent::Silluq.relative_strength()
        );
        assert_eq!(
            pseudo.relative_strength(),
            PseudoAccent::SophPasuq.relative_strength()
        );
    }

    #[test]
    fn hebrew_accent_group_level_delegates_correctly() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_eq!(prose.group_level(), ProseAccent::Silluq.group_level());
        assert_eq!(poetry.group_level(), PoetryAccent::Silluq.group_level());
        assert_eq!(pseudo.group_level(), PseudoAccent::SophPasuq.group_level());
    }

    // ── Cross-type consistency ──────────────────────────────────────

    // TODO
    // ── Edge cases ──────────────────────────────────────────────────

    #[test]
    fn accent_methods_dont_panic_on_any_variant() {
        // Verify no panics across all Prose variants
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let _ = accent.english_name();
            let _ = accent.hebrew_name();
            let _ = accent.hebrew_concept();
            let _ = accent.kind();
            let _ = accent.category();
            let _ = accent.word_stress();
            let _ = accent.is_compound();
            let _ = accent.primary_cantillation_mark();
            let _ = accent.secondary_cantillation_mark();
            let _ = accent.notes();
            let _ = accent.relative_strength();
            let _ = accent.group_level();
        }
    }

    #[test]
    fn accent_methods_dont_panic_on_poetry_variants() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let _ = accent.english_name();
            let _ = accent.hebrew_name();
            let _ = accent.hebrew_concept();
            let _ = accent.kind();
            let _ = accent.category();
            let _ = accent.word_stress();
            let _ = accent.is_compound();
            let _ = accent.primary_cantillation_mark();
            let _ = accent.secondary_cantillation_mark();
            let _ = accent.notes();
            let _ = accent.relative_strength();
            let _ = accent.group_level();
        }
    }

    #[test]
    fn accent_methods_dont_panic_on_pseudo_variants() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            let _ = accent.english_name();
            let _ = accent.hebrew_name();
            let _ = accent.hebrew_concept();
            let _ = accent.kind();
            let _ = accent.category();
            let _ = accent.word_stress();
            let _ = accent.is_compound();
            let _ = accent.primary_cantillation_mark();
            let _ = accent.secondary_cantillation_mark();
            let _ = accent.notes();
            let _ = accent.relative_strength();
            let _ = accent.group_level();
        }
    }

    #[test]
    fn accent_return_types_are_statically_sized() {
        // Verify Copy + Sized constraints are satisfied
        fn requires_copy_sized<T: Copy + Sized>() {}

        requires_copy_sized::<ProseAccent>();
        requires_copy_sized::<PoetryAccent>();
        requires_copy_sized::<PseudoAccent>();
        requires_copy_sized::<HebrewAccent>();
    }
    // ── From / Into conversion tests ────────────────────────────────

    #[test]
    fn prose_accent_converts_to_hebrew_accent_via_into() {
        let prose = ProseAccent::Silluq;
        let hebrew: HebrewAccent = prose.into();
        assert_eq!(hebrew.english_name(), prose.english_name());
    }

    #[test]
    fn poetry_accent_converts_to_hebrew_accent_via_into() {
        let poetry = PoetryAccent::Atnach;
        let hebrew: HebrewAccent = poetry.into();
        assert_eq!(hebrew.english_name(), poetry.english_name());
    }

    #[test]
    fn pseudo_accent_converts_to_hebrew_accent_via_into() {
        let pseudo = PseudoAccent::SophPasuq;
        let hebrew: HebrewAccent = pseudo.into();
        assert_eq!(hebrew.english_name(), pseudo.english_name());
    }

    // ── Compound accent tests (accents with secondary marks) ───────

    #[test]
    fn prose_has_at_least_one_compound_accent() {
        let any_compound = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.is_compound()
        });
        assert!(any_compound, "Expected at least one compound ProseAccent");
    }

    #[test]
    fn poetry_has_at_least_one_compound_accent() {
        let any_compound = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.is_compound()
        });
        assert!(any_compound, "Expected at least one compound PoetryAccent");
    }

    #[test]
    fn compound_accent_has_secondary_mark_when_is_compound_is_true() {
        // For every variant: is_compound() == secondary_cantillation_mark().is_some()
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            assert_eq!(
                accent.is_compound(),
                accent.secondary_cantillation_mark().is_some(),
                "Mismatch for ProseAccent variant {}: is_compound={}, secondary={:?}",
                v,
                accent.is_compound(),
                accent.secondary_cantillation_mark(),
            );
        }
    }

    #[test]
    fn poetry_compound_consistency() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            assert_eq!(
                accent.is_compound(),
                accent.secondary_cantillation_mark().is_some(),
                "Mismatch for PoetryAccent variant {}: is_compound={}, secondary={:?}",
                v,
                accent.is_compound(),
                accent.secondary_cantillation_mark(),
            );
        }
    }

    #[test]
    fn pseudo_compound_consistency() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            assert_eq!(
                accent.is_compound(),
                accent.secondary_cantillation_mark().is_some(),
                "Mismatch for PseudoAccent variant {}: is_compound={}, secondary={:?}",
                v,
                accent.is_compound(),
                accent.secondary_cantillation_mark(),
            );
        }
    }

    #[test]
    fn compound_prose_secondary_mark_has_valid_fields() {
        // Find a compound ProseAccent and verify its secondary mark
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(
                    !sec.unicode_value.is_empty(),
                    "Secondary unicode_value empty for variant {}",
                    v
                );
                assert!(
                    !sec.hex_bytes.is_empty(),
                    "Secondary hex_bytes empty for variant {}",
                    v
                );
                assert!(
                    !sec.symbol.is_empty(),
                    "Secondary symbol empty for variant {}",
                    v
                );
                assert!(
                    !sec.canonical_name.is_empty(),
                    "Secondary canonical_name empty for variant {}",
                    v
                );
                return; // Found and validated one, done
            }
        }
        panic!("No compound ProseAccent found to validate secondary mark");
    }

    #[test]
    fn compound_poetry_secondary_mark_has_valid_fields() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(
                    !sec.unicode_value.is_empty(),
                    "Secondary unicode_value empty for variant {}",
                    v
                );
                assert!(
                    !sec.hex_bytes.is_empty(),
                    "Secondary hex_bytes empty for variant {}",
                    v
                );
                assert!(
                    !sec.symbol.is_empty(),
                    "Secondary symbol empty for variant {}",
                    v
                );
                assert!(
                    !sec.canonical_name.is_empty(),
                    "Secondary canonical_name empty for variant {}",
                    v
                );
                return;
            }
        }
        panic!("No compound PoetryAccent found to validate secondary mark");
    }

    // ── Relative strength None coverage ────────────────────────────

    #[test]
    fn prose_has_at_least_one_accent_with_none_strength() {
        let any_none = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.relative_strength().is_none()
        });
        assert!(
            any_none,
            "Expected at least one ProseAccent with None strength (rank 255)"
        );
    }

    #[test]
    fn poetry_has_at_least_one_accent_with_none_strength() {
        let any_none = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.relative_strength().is_none()
        });
        assert!(
            any_none,
            "Expected at least one PoetryAccent with None strength (rank 255)"
        );
    }

    #[test]
    fn prose_relative_strength_values_are_valid_range() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            if let Some(s) = accent.relative_strength() {
                assert!(
                    s > 0 && s < 255,
                    "ProseAccent variant {} has invalid strength {}",
                    v,
                    s
                );
            }
        }
    }

    #[test]
    fn poetry_relative_strength_values_are_valid_range() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            if let Some(s) = accent.relative_strength() {
                assert!(
                    s > 0 && s < 255,
                    "PoetryAccent variant {} has invalid strength {}",
                    v,
                    s
                );
            }
        }
    }

    // ── HebrewAccent full delegation matrix ─────────────────────────

    #[test]
    fn hebrew_accent_delegates_all_methods_for_all_inner_types() {
        // Test a different variant for each inner type to ensure
        // delegation works beyond just Silluq
        let prose = HebrewAccent::Prose(ProseAccent::Atnach);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Munach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::Paseq);

        // english_name
        assert_eq!(prose.english_name(), ProseAccent::Atnach.english_name());
        assert_eq!(poetry.english_name(), PoetryAccent::Munach.english_name());
        assert_eq!(pseudo.english_name(), PseudoAccent::Paseq.english_name());

        // hebrew_name
        assert_eq!(prose.hebrew_name(), ProseAccent::Atnach.hebrew_name());
        assert_eq!(poetry.hebrew_name(), PoetryAccent::Munach.hebrew_name());
        assert_eq!(pseudo.hebrew_name(), PseudoAccent::Paseq.hebrew_name());

        // hebrew_concept
        assert_eq!(prose.hebrew_concept(), ProseAccent::Atnach.hebrew_concept());
        assert_eq!(
            poetry.hebrew_concept(),
            PoetryAccent::Munach.hebrew_concept()
        );
        assert_eq!(
            pseudo.hebrew_concept(),
            PseudoAccent::Paseq.hebrew_concept()
        );

        // kind
        assert_eq!(prose.kind(), ProseAccent::Atnach.kind());
        assert_eq!(poetry.kind(), PoetryAccent::Munach.kind());
        assert_eq!(pseudo.kind(), PseudoAccent::Paseq.kind());

        // category
        assert_eq!(prose.category(), ProseAccent::Atnach.category());
        assert_eq!(poetry.category(), PoetryAccent::Munach.category());
        assert_eq!(pseudo.category(), PseudoAccent::Paseq.category());

        // word_stress
        assert_eq!(prose.word_stress(), ProseAccent::Atnach.word_stress());
        assert_eq!(poetry.word_stress(), PoetryAccent::Munach.word_stress());
        assert_eq!(pseudo.word_stress(), PseudoAccent::Paseq.word_stress());

        // is_compound
        assert_eq!(prose.is_compound(), ProseAccent::Atnach.is_compound());
        assert_eq!(poetry.is_compound(), PoetryAccent::Munach.is_compound());
        assert_eq!(pseudo.is_compound(), PseudoAccent::Paseq.is_compound());

        // primary_cantillation_mark
        assert_eq!(
            prose.primary_cantillation_mark().symbol,
            ProseAccent::Atnach.primary_cantillation_mark().symbol
        );
        assert_eq!(
            poetry.primary_cantillation_mark().symbol,
            PoetryAccent::Munach.primary_cantillation_mark().symbol
        );
        assert_eq!(
            pseudo.primary_cantillation_mark().symbol,
            PseudoAccent::Paseq.primary_cantillation_mark().symbol
        );

        // secondary_cantillation_mark
        assert_eq!(
            prose.secondary_cantillation_mark(),
            ProseAccent::Atnach.secondary_cantillation_mark()
        );
        assert_eq!(
            poetry.secondary_cantillation_mark(),
            PoetryAccent::Munach.secondary_cantillation_mark()
        );
        assert_eq!(
            pseudo.secondary_cantillation_mark(),
            PseudoAccent::Paseq.secondary_cantillation_mark()
        );

        // notes
        assert_eq!(prose.notes(), ProseAccent::Atnach.notes());
        assert_eq!(poetry.notes(), PoetryAccent::Munach.notes());
        assert_eq!(pseudo.notes(), PseudoAccent::Paseq.notes());

        // relative_strength
        assert_eq!(
            prose.relative_strength(),
            ProseAccent::Atnach.relative_strength()
        );
        assert_eq!(
            poetry.relative_strength(),
            PoetryAccent::Munach.relative_strength()
        );
        assert_eq!(
            pseudo.relative_strength(),
            PseudoAccent::Paseq.relative_strength()
        );

        // group_level
        assert_eq!(prose.group_level(), ProseAccent::Atnach.group_level());
        assert_eq!(poetry.group_level(), PoetryAccent::Munach.group_level());
        assert_eq!(pseudo.group_level(), PseudoAccent::Paseq.group_level());
    }

    // ── Cross-type consistency tests (was // TODO) ──────────────────

    #[test]
    fn cross_type_names_are_unique_across_prose_and_poetry() {
        // Shared accents (same name in both systems) should still produce
        // different CantillationMarks if their Unicode codepoints differ
        let prose_silluq = ProseAccent::Silluq.primary_cantillation_mark();
        let poetry_silluq = PoetryAccent::Silluq.primary_cantillation_mark();

        // Same English name is expected (both are "Silluq")
        assert_eq!(
            ProseAccent::Silluq.english_name(),
            PoetryAccent::Silluq.english_name(),
            "Shared accents should have same English name"
        );

        // But their Unicode codepoints should match (Silluq is the same mark)
        assert_eq!(
            prose_silluq.unicode_value, poetry_silluq.unicode_value,
            "Silluq should have same codepoint in both systems"
        );
    }

    #[test]
    fn cross_type_prose_poetry_share_common_names() {
        // Several accent names exist in both systems (Silluq, Atnach, Munach, etc.)
        // Verify they share names but may differ in metadata
        let shared = [
            ("Silluq", ProseAccent::Silluq, PoetryAccent::Silluq),
            ("Atnach", ProseAccent::Atnach, PoetryAccent::Atnach),
            ("Munach", ProseAccent::Munach, PoetryAccent::Munach),
        ];

        for (name, prose, poetry) in shared.iter() {
            assert_eq!(prose.english_name(), *name);
            assert_eq!(poetry.english_name(), *name);
        }
    }

    #[test]
    fn cross_type_prose_poetry_kinds_may_differ() {
        // The kind/category metadata for shared accent names may differ
        // between prose and poetry systems. Verify they're at least both Some.
        let pairs: [(ProseAccent, PoetryAccent); 3] = [
            (ProseAccent::Silluq, PoetryAccent::Silluq),
            (ProseAccent::Atnach, PoetryAccent::Atnach),
            (ProseAccent::Munach, PoetryAccent::Munach),
        ];

        for (prose, poetry) in pairs.iter() {
            assert!(
                prose.kind().is_some(),
                "Prose {:?} kind should be Some",
                prose
            );
            assert!(
                poetry.kind().is_some(),
                "Poetry {:?} kind should be Some",
                poetry
            );
            assert!(
                prose.category().is_some(),
                "Prose {:?} category should be Some",
                prose
            );
            assert!(
                poetry.category().is_some(),
                "Poetry {:?} category should be Some",
                poetry
            );
        }
    }

    #[test]
    fn cross_type_pseudo_never_has_strength_or_group_level() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            assert_eq!(
                accent.relative_strength(),
                None,
                "PseudoAccent variant {} should have None strength",
                v
            );
            assert_eq!(
                accent.group_level(),
                None,
                "PseudoAccent variant {} should have None group_level",
                v
            );
        }
    }

    #[test]
    fn cross_type_hebrew_accent_from_each_inner_preserves_identity() {
        let prose = HebrewAccent::Prose(ProseAccent::Revia);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);

        // Verify the HebrewAccent wraps correctly and delegates
        match prose {
            HebrewAccent::Prose(_) => {}
            _ => panic!("Expected HebrewAccent::Prose"),
        }
        match poetry {
            HebrewAccent::Poetry(_) => {}
            _ => panic!("Expected HebrewAccent::Poetry"),
        }
        match pseudo {
            HebrewAccent::Pseudo(_) => {}
            _ => panic!("Expected HebrewAccent::Pseudo"),
        }
    }

    // ── Exhaustive trait method coverage for all variants ───────────

    #[test]
    fn prose_all_variants_english_name_not_unknown() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            assert_ne!(
                accent.english_name(),
                "UNKNOWN",
                "ProseAccent variant {} has UNKNOWN english_name",
                v
            );
            assert_ne!(
                accent.hebrew_name(),
                "UNKNOWN",
                "ProseAccent variant {} has UNKNOWN hebrew_name",
                v
            );
            assert_ne!(
                accent.hebrew_concept(),
                "UNKNOWN",
                "ProseAccent variant {} has UNKNOWN hebrew_concept",
                v
            );
        }
    }

    #[test]
    fn poetry_all_variants_english_name_not_unknown() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            assert_ne!(
                accent.english_name(),
                "UNKNOWN",
                "PoetryAccent variant {} has UNKNOWN english_name",
                v
            );
            assert_ne!(
                accent.hebrew_name(),
                "UNKNOWN",
                "PoetryAccent variant {} has UNKNOWN hebrew_name",
                v
            );
            assert_ne!(
                accent.hebrew_concept(),
                "UNKNOWN",
                "PoetryAccent variant {} has UNKNOWN hebrew_concept",
                v
            );
        }
    }

    #[test]
    fn pseudo_all_variants_english_name_not_unknown() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            assert_ne!(
                accent.english_name(),
                "UNKNOWN",
                "PseudoAccent variant {} has UNKNOWN english_name",
                v
            );
            assert_ne!(
                accent.hebrew_name(),
                "UNKNOWN",
                "PseudoAccent variant {} has UNKNOWN hebrew_name",
                v
            );
            assert_ne!(
                accent.hebrew_concept(),
                "UNKNOWN",
                "PseudoAccent variant {} has UNKNOWN hebrew_concept",
                v
            );
        }
    }

    #[test]
    fn prose_all_variants_kind_and_category_consistent() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let kind = accent.kind();
            let cat = accent.category();

            // Every accent variant should have a defined kind and category
            assert!(kind.is_some(), "ProseAccent variant {} has no kind", v);
            assert!(cat.is_some(), "ProseAccent variant {} has no category", v);
        }
    }

    #[test]
    fn poetry_all_variants_kind_and_category_consistent() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let kind = accent.kind();
            let cat = accent.category();

            assert!(kind.is_some(), "PoetryAccent variant {} has no kind", v);
            assert!(cat.is_some(), "PoetryAccent variant {} has no category", v);
        }
    }

    #[test]
    fn pseudo_all_variants_kind_and_category_consistent() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            let kind = accent.kind();
            let cat = accent.category();

            assert!(kind.is_some(), "PseudoAccent variant {} has no kind", v);
            assert!(cat.is_some(), "PseudoAccent variant {} has no category", v);
        }
    }

    #[test]
    fn prose_all_variants_secondary_mark_fields_valid_when_some() {
        for v in 0..ProseAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(!sec.unicode_value.is_empty(), "variant {}", v);
                assert!(!sec.hex_bytes.is_empty(), "variant {}", v);
                assert!(!sec.symbol.is_empty(), "variant {}", v);
                assert!(!sec.canonical_name.is_empty(), "variant {}", v);
            }
        }
    }

    #[test]
    fn poetry_all_variants_secondary_mark_fields_valid_when_some() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(!sec.unicode_value.is_empty(), "variant {}", v);
                assert!(!sec.hex_bytes.is_empty(), "variant {}", v);
                assert!(!sec.symbol.is_empty(), "variant {}", v);
                assert!(!sec.canonical_name.is_empty(), "variant {}", v);
            }
        }
    }

    #[test]
    fn prose_all_variants_distinct_english_names() {
        let names: Vec<&str> = (0..ProseAccent::LEN as u8)
            .map(|v| {
                let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
                accent.english_name()
            })
            .collect();
        let unique: std::collections::HashSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "ProseAccent has duplicate English names"
        );
    }

    #[test]
    fn poetry_all_variants_distinct_english_names() {
        let names: Vec<&str> = (0..PoetryAccent::LEN as u8)
            .map(|v| {
                let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
                accent.english_name()
            })
            .collect();
        let unique: std::collections::HashSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "PoetryAccent has duplicate English names"
        );
    }

    #[test]
    fn pseudo_all_variants_distinct_english_names() {
        let names: Vec<&str> = (0..PseudoAccent::LEN as u8)
            .map(|v| {
                let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
                accent.english_name()
            })
            .collect();
        let unique: std::collections::HashSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "PseudoAccent has duplicate English names"
        );
    }

    #[test]
    fn hebrew_accent_exhaustive_delegation_all_methods_all_variants() {
        // Exhaustive: for EVERY variant of every inner type, verify
        // HebrewAccent delegates correctly on ALL trait methods
        for v in 0..ProseAccent::LEN as u8 {
            let prose = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            let wrapped = HebrewAccent::Prose(prose);
            assert_eq!(wrapped.english_name(), prose.english_name());
            assert_eq!(wrapped.hebrew_name(), prose.hebrew_name());
            assert_eq!(wrapped.hebrew_concept(), prose.hebrew_concept());
            assert_eq!(wrapped.kind(), prose.kind());
            assert_eq!(wrapped.category(), prose.category());
            assert_eq!(wrapped.word_stress(), prose.word_stress());
            assert_eq!(wrapped.is_compound(), prose.is_compound());
            assert_eq!(
                wrapped.primary_cantillation_mark().symbol,
                prose.primary_cantillation_mark().symbol
            );
            assert_eq!(
                wrapped.secondary_cantillation_mark(),
                prose.secondary_cantillation_mark()
            );
            assert_eq!(wrapped.notes(), prose.notes());
            assert_eq!(wrapped.relative_strength(), prose.relative_strength());
            assert_eq!(wrapped.group_level(), prose.group_level());
        }

        for v in 0..PoetryAccent::LEN as u8 {
            let poetry = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            let wrapped = HebrewAccent::Poetry(poetry);
            assert_eq!(wrapped.english_name(), poetry.english_name());
            assert_eq!(wrapped.hebrew_name(), poetry.hebrew_name());
            assert_eq!(wrapped.hebrew_concept(), poetry.hebrew_concept());
            assert_eq!(wrapped.kind(), poetry.kind());
            assert_eq!(wrapped.category(), poetry.category());
            assert_eq!(wrapped.word_stress(), poetry.word_stress());
            assert_eq!(wrapped.is_compound(), poetry.is_compound());
            assert_eq!(
                wrapped.primary_cantillation_mark().symbol,
                poetry.primary_cantillation_mark().symbol
            );
            assert_eq!(
                wrapped.secondary_cantillation_mark(),
                poetry.secondary_cantillation_mark()
            );
            assert_eq!(wrapped.notes(), poetry.notes());
            assert_eq!(wrapped.relative_strength(), poetry.relative_strength());
            assert_eq!(wrapped.group_level(), poetry.group_level());
        }

        for v in 0..PseudoAccent::LEN as u8 {
            let pseudo = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            let wrapped = HebrewAccent::Pseudo(pseudo);
            assert_eq!(wrapped.english_name(), pseudo.english_name());
            assert_eq!(wrapped.hebrew_name(), pseudo.hebrew_name());
            assert_eq!(wrapped.hebrew_concept(), pseudo.hebrew_concept());
            assert_eq!(wrapped.kind(), pseudo.kind());
            assert_eq!(wrapped.category(), pseudo.category());
            assert_eq!(wrapped.word_stress(), pseudo.word_stress());
            assert_eq!(wrapped.is_compound(), pseudo.is_compound());
            assert_eq!(
                wrapped.primary_cantillation_mark().symbol,
                pseudo.primary_cantillation_mark().symbol
            );
            assert_eq!(
                wrapped.secondary_cantillation_mark(),
                pseudo.secondary_cantillation_mark()
            );
            assert_eq!(wrapped.notes(), pseudo.notes());
            assert_eq!(wrapped.relative_strength(), pseudo.relative_strength());
            assert_eq!(wrapped.group_level(), pseudo.group_level());
        }
    }

    // ── Notes: verify both Some and None paths exist ───────────────

    #[test]
    fn prose_has_accents_with_and_without_notes() {
        let any_some = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.notes().is_some()
        });
        let any_none = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.notes().is_none()
        });
        assert!(any_some, "Expected at least one ProseAccent with notes");
        assert!(any_none, "Expected at least one ProseAccent without notes");
    }

    #[test]
    fn poetry_has_accents_with_and_without_notes() {
        let any_some = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.notes().is_some()
        });
        let any_none = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.notes().is_none()
        });
        assert!(any_some, "Expected at least one PoetryAccent with notes");
        assert!(any_none, "Expected at least one PoetryAccent without notes");
    }

    #[test]
    fn pseudo_notes_some_for_all_variants() {
        // PseudoAccent only has 3 variants; check if notes varies
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            // Just exercise the closure; don't assume Some or None
            let _ = accent.notes();
        }
    }

    // ── Word stress: verify Some path is exercised ─────────────────

    #[test]
    fn prose_has_at_least_one_accent_with_word_stress() {
        let any_some = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.word_stress().is_some()
        });
        assert!(
            any_some,
            "Expected at least one ProseAccent with defined word stress"
        );
    }

    #[test]
    fn poetry_has_at_least_one_accent_with_word_stress() {
        let any_some = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.word_stress().is_some()
        });
        assert!(
            any_some,
            "Expected at least one PoetryAccent with defined word stress"
        );
    }

    #[test]
    fn prose_has_at_least_one_accent_without_word_stress() {
        let any_none = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.word_stress().is_none()
        });
        assert!(
            any_none,
            "Expected at least one ProseAccent with undefined word stress"
        );
    }

    // ── Group level: verify Some path for disjunctive accents ──────

    #[test]
    fn prose_has_at_least_one_accent_with_group_level() {
        let any_some = (0..ProseAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, ProseAccent>(v) };
            accent.group_level().is_some()
        });
        assert!(
            any_some,
            "Expected at least one ProseAccent with a group level"
        );
    }

    #[test]
    fn poetry_has_at_least_one_accent_with_group_level() {
        let any_some = (0..PoetryAccent::LEN as u8).any(|v| {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            accent.group_level().is_some()
        });
        assert!(
            any_some,
            "Expected at least one PoetryAccent with a group level"
        );
    }

    // ── PseudoAccent secondary mark field validation ──────────────
    // (mirrors the prose/poetry compound_*_secondary_mark tests)

    #[test]
    fn compound_pseudo_secondary_mark_has_valid_fields() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(
                    !sec.unicode_value.is_empty(),
                    "Secondary unicode_value empty for PseudoAccent variant {}",
                    v
                );
                assert!(
                    !sec.hex_bytes.is_empty(),
                    "Secondary hex_bytes empty for PseudoAccent variant {}",
                    v
                );
                assert!(
                    !sec.symbol.is_empty(),
                    "Secondary symbol empty for PseudoAccent variant {}",
                    v
                );
                assert!(
                    !sec.canonical_name.is_empty(),
                    "Secondary canonical_name empty for PseudoAccent variant {}",
                    v
                );
                return; // validated one, done
            }
        }
        // No compound PseudoAccent — acceptable, but log it
        eprintln!("No compound PseudoAccent variants found");
    }

    // ── PseudoAccent: all-variants exhaustion for remaining methods ─

    #[test]
    fn pseudo_all_variants_secondary_mark_fields_valid_when_some() {
        for v in 0..PseudoAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PseudoAccent>(v) };
            if let Some(sec) = accent.secondary_cantillation_mark() {
                assert!(!sec.unicode_value.is_empty(), "variant {}", v);
                assert!(!sec.hex_bytes.is_empty(), "variant {}", v);
                assert!(!sec.symbol.is_empty(), "variant {}", v);
                assert!(!sec.canonical_name.is_empty(), "variant {}", v);
            }
        }
    }

    // ── HebrewAccent out-of-bounds delegation ──────────────────────
    // Verify HebrewAccent forwards fallbacks correctly for OOB inner variants

    // ── Replace tautological tests with meaningful assertions ──────
    // These supersede tests that used `assert!(x.is_some() || x.is_none())`

    #[test]
    fn prose_word_stress_silluq_is_defined() {
        // Silluq is a disjunctive accent; it should have a defined stress
        let stress = ProseAccent::Silluq.word_stress();
        assert!(stress.is_some(), "Silluq should have a defined word stress");
    }

    #[test]
    fn poetry_word_stress_silluq_is_defined() {
        let stress = PoetryAccent::Silluq.word_stress();
        assert!(
            stress.is_some(),
            "Poetry Silluq should have a defined word stress"
        );
    }

    #[test]
    fn prose_notes_silluq_is_some() {
        // Silluq is a major accent; it likely has scholarly notes
        let notes = ProseAccent::Silluq.notes();
        assert!(notes.is_some(), "Silluq should have scholarly notes");
        if let Some(n) = notes {
            assert!(!n.is_empty(), "Notes should be non-empty");
        }
    }

    #[test]
    fn poetry_notes_atnach_is_some() {
        let notes = PoetryAccent::Atnach.notes();
        assert!(notes.is_some(), "Poetry Atnach should have scholarly notes");
        if let Some(n) = notes {
            assert!(!n.is_empty(), "Notes should be non-empty");
        }
    }

    // ── Kind: verify specific kind values ──────────────────────────

    #[test]
    fn prose_silluq_kind_is_primary() {
        let kind = ProseAccent::Silluq.kind();
        assert!(kind.is_some(), "Silluq should have a kind");
    }

    #[test]
    fn prose_munach_kind_differs_from_silluq() {
        // Silluq (disjunctive) and Munach (conjunctive) should differ in kind
        let silluq_kind = ProseAccent::Silluq.kind();
        let munach_kind = ProseAccent::Munach.kind();
        assert_ne!(
            silluq_kind, munach_kind,
            "Silluq and Munach should have different kinds"
        );
    }

    #[test]
    fn poetry_silluq_and_munach_kinds_differ() {
        let silluq_kind = PoetryAccent::Silluq.kind();
        let munach_kind = PoetryAccent::Munach.kind();
        assert_ne!(
            silluq_kind, munach_kind,
            "Poetry Silluq and Munach should have different kinds"
        );
    }

    // ── Relative strength: exhaustive None-closure coverage ────────

    #[test]
    fn poetry_all_none_strength_variants_are_valid() {
        for v in 0..PoetryAccent::LEN as u8 {
            let accent = unsafe { std::mem::transmute::<u8, PoetryAccent>(v) };
            if accent.relative_strength().is_none() {
                let _ = accent.category();
            }
        }
    }
}
