use crate::accent::resolve_disjunctive_group;
use crate::accent_data::{
    BHS_POETRY_RANK_MAP, BHS_PROSE_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE,
    PSEUDO_ACCENT_TABLE,
};
use crate::{
    AccentCategory, AccentKind, CantillationMark, GroupLevel, HebrewAccent, MaxWordSpan,
    PoetryAccent, ProseAccent, PseudoAccent,
};

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
pub trait Accent: Copy + Sized {
    /// Returns the Hebrew name of the accent.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
    /// assert_eq!(silluq.hebrew_name(), "סִלּוּק");
    /// ```
    fn hebrew_name(self) -> &'static str;

    /// Returns the semantic meaning/concept of the Hebrew name.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    /// let atnach = ProseAccent::Atnach;
    /// // אַתְנָח literally means "rest" or "pause"
    /// assert_eq!(atnach.hebrew_concept(), "a causing to rest");
    /// ```
    fn hebrew_concept(self) -> &'static str;

    /// Returns the English transliteration of the accent name.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, PoetryAccent};
    ///
    /// let revia = PoetryAccent::ReviaGadol;
    /// assert_eq!(revia.english_name(), "Revia Gadol");
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
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    ///
    /// let silluq = ProseAccent::Silluq;
    /// // May differ from english_name for phonetic precision
    /// println!("SBL: {}", silluq.sbl_academic_name());
    /// assert_eq!(silluq.sbl_academic_name(), "sillûq");
    /// ```
    fn sbl_academic_name(self) -> &'static str;

    /// Returns the accent kind (primary or secondary), if applicable.
    ///
    /// Primary accents typically carry more weight in the phrasing hierarchy.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent,AccentKind};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// // Silluq is a primary accent (marks end of verse)
    /// assert!(silluq.kind().is_some());
    /// assert_eq!(silluq.kind(), Some(AccentKind::Primary));
    /// ```
    fn kind(self) -> Option<AccentKind>;

    /// Returns the accent category (disjunctive or conjunctive), if applicable.
    ///
    /// - **Disjunctive**: Marks pauses/breaks in the text
    /// - **Conjunctive**: Connects words together
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent,AccentCategory};
    ///
    /// let silluq = ProseAccent::Silluq;   // Disjunctive
    /// let munach = ProseAccent::Munach;   // Conjunctive
    ///
    /// assert_ne!(silluq.category(), munach.category());
    /// assert_eq!(silluq.category(), Some(AccentCategory::Disjunctive));
    /// assert_eq!(munach.category(), Some(AccentCategory::Conjunctive));
    /// ```
    fn category(self) -> Option<AccentCategory>;

    /// Returns whether this accent consists of multiple Unicode codepoints.
    ///
    /// Compound accents have both a primary AND secondary cantillation mark.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let shalshelet = ProseAccent::Shalshelet;  // Compound
    ///
    /// assert!(!silluq.is_compound());
    /// assert!(shalshelet.is_compound());
    fn is_compound(self) -> bool;

    /// Returns the primary cantillation mark (always present).
    ///
    /// Every accent must have exactly one primary mark.
    ///
    /// # Safety
    /// This method will **panic** if called with an invalid enum variant.
    /// Valid enum variants are guaranteed to have table entries.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, CantillationMarkPlacement, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let primary = silluq.primary_cantillation_mark();
    ///
    /// assert_eq!(primary.placement,CantillationMarkPlacement::BelowCenter);
    /// ```
    fn primary_cantillation_mark(self) -> CantillationMark;

    /// Returns the secondary cantillation mark (only for compound accents).
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let shalshelet = ProseAccent::Shalshelet;
    ///
    /// if let Some(secondary) = shalshelet.secondary_cantillation_mark() {
    ///     println!("Secondary mark: {}", secondary.symbol);  // "׀" (Paseq)
    /// } else {
    ///     println!("This accent is not compound");
    /// }
    /// ```
    fn secondary_cantillation_mark(self) -> Option<CantillationMark>;

    /// Returns scholarly notes or context about this accent, if available.
    ///
    /// # Example
    /// ```rust   
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
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
    /// - **Prose/Poetry**: Returns `Some(u8)` with strength ranking
    /// - **Pseudo**: Always returns `None` (no hierarchy)
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    ///
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
    /// - **Disjunctive accents**: Return `Some(GroupLevel)`
    /// - **Conjunctive accents**: Return `None`
    /// - **Pseudo accents**: Return `None`
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, GroupLevel};
    ///
    /// let silluq = ProseAccent::Silluq;
    ///
    /// if let Some(level) = silluq.group_level() {
    ///      assert_eq!(GroupLevel::Level1, level);
    ///   }
    /// ```
    fn group_level(self) -> Option<GroupLevel>;

    /// Returns the cantillation symbol as a Unicode string.
    ///
    /// This is the rendered representation of the accent mark(s).
    /// For compound accents, includes both primary and secondary marks.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let symbol = silluq.cantillation_symbol();
    ///
    /// println!("Display: {}", symbol);  // "֫"
    /// ```
    fn cantillation_symbol(self) -> String;

    /// Returns the maximum word span for this accent.
    ///
    /// Indicates how many words this accent can span across.
    /// Most accents span a single word; some compound or special
    /// accents may span multiple words.
    ///
    /// - **Typical**: `MaxWordSpan::OneWord`
    /// - **Special cases**: `MaxWordSpan::TwoWords` or `NotApplicable`
    /// - **Pseudo accents**: Always `None`
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, PoetryAccent, MaxWordSpan};
    ///
    /// let oleh_weyored = PoetryAccent::OlehWeYored;
    /// if let Some(span) = oleh_weyored.max_word_span() {
    ///      assert_eq!(span, MaxWordSpan::TwoWords);
    /// }
    /// ```
    fn max_word_span(self) -> Option<MaxWordSpan>;
}

// ────────────────────────────────────────────────────────────────────
// HebrewAccent Implementation (delegates to inner variant)
// ────────────────────────────────────────────────────────────────────

impl Accent for HebrewAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_name(),
            HebrewAccent::Poetry(p) => p.hebrew_name(),
            HebrewAccent::Pseudo(p) => p.hebrew_name(),
        }
    }

    #[inline]
    fn hebrew_concept(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_concept(),
            HebrewAccent::Poetry(p) => p.hebrew_concept(),
            HebrewAccent::Pseudo(p) => p.hebrew_concept(),
        }
    }

    #[inline]
    fn english_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.english_name(),
            HebrewAccent::Poetry(p) => p.english_name(),
            HebrewAccent::Pseudo(p) => p.english_name(),
        }
    }

    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.sbl_academic_name(),
            HebrewAccent::Poetry(p) => p.sbl_academic_name(),
            HebrewAccent::Pseudo(p) => p.sbl_academic_name(),
        }
    }

    #[inline]
    fn kind(self) -> Option<AccentKind> {
        match self {
            HebrewAccent::Prose(p) => p.kind(),
            HebrewAccent::Poetry(p) => p.kind(),
            HebrewAccent::Pseudo(p) => p.kind(),
        }
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        match self {
            HebrewAccent::Prose(p) => p.category(),
            HebrewAccent::Poetry(p) => p.category(),
            HebrewAccent::Pseudo(p) => p.category(),
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

    #[inline]
    fn max_word_span(self) -> Option<MaxWordSpan> {
        match self {
            HebrewAccent::Prose(p) => p.max_word_span(),
            HebrewAccent::Poetry(p) => p.max_word_span(),
            HebrewAccent::Pseudo(p) => p.max_word_span(),
        }
    }
}

// ────────────────────────────────────────────────────────────────────
// ProseAccent Implementation
// ────────────────────────────────────────────────────────────────────

impl Accent for ProseAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        PROSE_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(self) -> Option<AccentKind> {
        PROSE_ACCENT_TABLE[self.as_index()].kind.to_public()
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PROSE_ACCENT_TABLE[self.as_index()].category.to_public()
    }

    #[inline]
    fn is_compound(self) -> bool {
        PROSE_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .secondary_mark
            .is_some()
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        let info = PROSE_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .primary_mark;

        CantillationMark {
            symbol: info.symbol,
            placement: info.position.into(),
            stress_position: info.stress_position.to_public(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
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
    fn notes(self) -> Option<&'static str> {
        PROSE_ACCENT_TABLE[self.as_index()].notes
    }

    #[inline]
    fn relative_strength(self) -> Option<u8> {
        Some(BHS_PROSE_RANK_MAP[self.as_index()])
    }

    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }

    #[inline]
    fn cantillation_symbol(self) -> String {
        "TODO".to_string()
    }

    #[inline]
    fn max_word_span(self) -> Option<MaxWordSpan> {
        PROSE_ACCENT_TABLE[self.as_index()].word_span.to_public()
    }
}

// ────────────────────────────────────────────────────────────────────
// PoetryAccent Implementation
// ────────────────────────────────────────────────────────────────────

impl Accent for PoetryAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        POETRY_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(self) -> Option<AccentKind> {
        POETRY_ACCENT_TABLE[self.as_index()].kind.to_public()
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        POETRY_ACCENT_TABLE[self.as_index()].category.to_public()
    }

    #[inline]
    fn is_compound(self) -> bool {
        POETRY_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .secondary_mark
            .is_some()
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
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
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
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
    fn notes(self) -> Option<&'static str> {
        POETRY_ACCENT_TABLE[self.as_index()].notes
    }

    #[inline]
    fn relative_strength(self) -> Option<u8> {
        Some(BHS_POETRY_RANK_MAP[self.as_index()])
    }

    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }

    #[inline]
    fn cantillation_symbol(self) -> String {
        "TODO".to_string()
    }

    #[inline]
    fn max_word_span(self) -> Option<MaxWordSpan> {
        POETRY_ACCENT_TABLE[self.as_index()].word_span.to_public()
    }
}

// ────────────────────────────────────────────────────────────────────
// PseudoAccent Implementation
// ────────────────────────────────────────────────────────────────────

impl Accent for PseudoAccent {
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].hebrew_name
    }

    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].hebrew_concept
    }

    #[inline]
    fn english_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].english_name
    }

    #[inline]
    fn sbl_academic_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE[self.as_index()].sbl_academic
    }

    #[inline]
    fn kind(self) -> Option<AccentKind> {
        PSEUDO_ACCENT_TABLE[self.as_index()].kind.to_public()
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PSEUDO_ACCENT_TABLE[self.as_index()].category.to_public()
    }

    #[inline]
    fn is_compound(self) -> bool {
        PSEUDO_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .secondary_mark
            .is_some()
    }

    #[inline]
    fn primary_cantillation_mark(self) -> CantillationMark {
        let info = PSEUDO_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .primary_mark;

        CantillationMark {
            symbol: info.symbol,
            placement: info.position.into(),
            stress_position: info.stress_position.to_public(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(self) -> Option<CantillationMark> {
        PSEUDO_ACCENT_TABLE[self.as_index()]
            .cantillation_symbol
            .secondary_mark
            .map(|info| CantillationMark {
                symbol: info.symbol,
                placement: info.position.into(),
                stress_position: info.stress_position.to_public(),
            })
    }

    #[inline]
    fn notes(self) -> Option<&'static str> {
        PSEUDO_ACCENT_TABLE[self.as_index()].notes
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
        "TODO".to_string()
    }

    #[inline]
    fn max_word_span(self) -> Option<MaxWordSpan> {
        None
    }
}

#[cfg(test)]
mod tests1 {
    use crate::MaxWordSpan;
    #[test]
    fn level() {
        use crate::{Accent, GroupLevel, ProseAccent};

        let silluq = ProseAccent::Silluq;

        if let Some(level) = silluq.group_level() {
            assert_eq!(GroupLevel::Level1, level);
        } else {
            panic!("BUG::wrong grouplevel");
        }
    }
    #[test]
    fn wordspan() {
        use crate::{Accent, PoetryAccent};

        let oleh_weyored = PoetryAccent::OlehWeYored;
        if let Some(span) = oleh_weyored.max_word_span() {
            assert_eq!(span, MaxWordSpan::TwoWords);
        }
    }

    #[test]
    fn placement() {
        use crate::{Accent, CantillationMarkPlacement, ProseAccent};

        let silluq = ProseAccent::Silluq;
        let primary = silluq.primary_cantillation_mark();

        assert_eq!(primary.placement, CantillationMarkPlacement::BelowCenter);
    }
}
