use crate::api::CompoundType;
use crate::{AccentCategory, AccentKind, AlternateNames, CantillationMark, GroupLevel};
/// The `Accent` trait provides a unified interface for working with Hebrew
/// cantillation marks (also known as ta'amim or trope).
///
/// ## Overview
///
/// This trait abstracts over three distinct accent systems:
/// - [`ProseAccent`] - Used in most biblical books (prosaic texts)
/// - [`PoetryAccent`] - Used in poetic books (Psalms, Proverbs, Job)
/// - [`PseudoAccent`] - Non-cantillation marks treated similarly (e.g., Maqqaph, Paseq)
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
pub trait Accent {
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
    fn hebrew_name(&self) -> &'static str;

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
    fn hebrew_concept(&self) -> &'static str;

    /// Returns the English transliteration of the accent name.
    ///
    /// See [Transliteration Rules](crate::docs::transliteration) for the used rules.
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, PoetryAccent};
    ///
    /// let revia = PoetryAccent::ReviaGadol;
    /// assert_eq!(revia.english_name(), "Revia Gadol");
    /// ```
    fn english_name(&self) -> &'static str;

    /// Returns the SBL academic transliteration (Society of Biblical Literature).
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
    /// let silluq = ProseAccent::Silluq;
    /// // May differ from english_name for phonetic precision
    /// println!("SBL: {}", silluq.sbl_academic_name());
    /// assert_eq!(silluq.sbl_academic_name(), "sillûq");
    /// ```
    fn sbl_academic_name(&self) -> &'static str;

    /// Returns the accent kind (primary or secondary), if applicable.
    ///
    /// **Primary** accents carry the main stress on a word (e.g., silluq, munach, pashta).
    /// They determine which syllable gets the prominent beat in recitation.
    ///
    /// **Secondary** accents like meteg (also called ga'ya) mark weaker stress or vowel lengthening,
    /// often functioning as a "subordinate" stress marker alongside or in place of another accent.
    /// It can indicate where a secondary stress falls within a word or help resolve ambiguities
    /// like vocal sheva vs. silent sheva.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, PoetryAccent, PseudoAccent, AccentKind};
    ///
    /// // Primary accent: main phrasing division
    /// let silluq = ProseAccent::Silluq;
    /// assert_eq!(silluq.kind(), Some(AccentKind::Primary));
    ///
    /// // Secondary accent: stress marker
    /// let meteg = PoetryAccent::Meteg;
    /// assert_eq!(meteg.kind(), Some(AccentKind::Secondary));
    ///
    /// // Pseudo accent: no hierarchical kind
    /// let paseq = PseudoAccent::Paseq;
    /// assert_eq!(paseq.kind(), None);
    /// ```
    fn kind(&self) -> Option<AccentKind>;

    /// Returns the accent category (disjunctive or conjunctive), if applicable.
    ///
    /// - **Disjunctive**: Marks pauses/breaks in the text
    /// - **Conjunctive**: Connects words together
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, AccentCategory};
    ///
    /// let silluq = ProseAccent::Silluq;   // Disjunctive
    /// let munach = ProseAccent::Munach;   // Conjunctive
    ///
    /// assert_ne!(silluq.category(), munach.category());
    /// assert_eq!(silluq.category(), Some(AccentCategory::Disjunctive));
    /// assert_eq!(munach.category(), Some(AccentCategory::Conjunctive));
    /// ```
    fn category(&self) -> Option<AccentCategory>;

    /// Returns whether this accent consists of multiple Unicode codepoints.
    ///
    /// Compound accents combine a primary cantillation mark with a secondary
    /// diacritic, resulting in multiple Unicode codepoints when rendered.
    /// Non-compound accents use a single codepoint.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let shalshelet = ProseAccent::Shalshelet;  // Compound
    ///
    /// // assert!(!silluq.is_compound());
    /// // assert!(shalshelet.is_compound());
    fn compound_type(&self) -> Option<CompoundType>; // None = not compound

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
    fn primary_cantillation_mark(&self) -> CantillationMark;

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
    fn secondary_cantillation_mark(&self) -> Option<CantillationMark>;

    /// Returns (scholarly) notes or context about this accent, if available.
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
    fn notes(&self) -> Option<&'static str>;

    /// Indicates the relative strength for disjunctive accents only.
    ///
    /// Disjunctive accents mark where pauses occur during reading. Stronger
    /// accents correspond to longer pauses; weaker accents indicate shorter
    /// pauses. This mirrors English punctuation: a period ends a sentence with
    /// a full stop, while a comma within a sentence signals a brief pause.
    ///
    /// Strength rankings use a numeric scale where `1` represents the strongest
    /// (most dominant) accent, and higher numbers indicate progressively weaker
    /// (more subordinate) accents.
    ///
    /// # Return Value
    ///
    /// - **Disjunctive accents** (Prose or Poetry): Returns `Some(u8)` with the
    ///   strength ranking
    /// - **Conjunctive accents** (Prose or Poetry): Returns `None` (no hierarchy)
    /// - **Pseudo accents**: Always returns `None` (no hierarchy)
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, PoetryAccent,PseudoAccent};
    ///
    /// // Disjunctive accents return strength rankings (1 = strongest)
    /// let silluq = ProseAccent::Silluq;
    /// let atnach = ProseAccent::Atnach;
    /// let legarmeh = ProseAccent::Legarmeh;
    ///
    /// // Lower number = stronger accent
    /// assert_eq!(silluq.relative_strength(), Some(1));
    /// assert_eq!(atnach.relative_strength(), Some(2));
    /// assert_eq!(legarmeh.relative_strength(), Some(18));
    ///
    /// // Compare strength levels programmatically
    /// if let (Some(s1), Some(s2)) = (silluq.relative_strength(), atnach.relative_strength()) {
    ///     assert!(s1 < s2, "Lower rank means stronger pause");
    /// }
    ///
    /// // Conjunctive and pseudo accents return None (no hierarchy)
    /// let munach = ProseAccent::Munach;
    /// let paseq = PseudoAccent::Paseq;
    /// assert_eq!(munach.relative_strength(), None);
    /// assert_eq!(paseq.relative_strength(), None);
    /// ```
    fn relative_strength(&self) -> Option<u8>;

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
    ///      assert_eq!(GroupLevel::Tier1, level);
    ///   }
    /// ```
    fn group_level(&self) -> Option<GroupLevel>;

    /// Returns the cantillation symbol augmented with dotted circle(s)
    ///
    /// The U+25CC ◌ DOTTED CIRCLE serves as a neutral base glyph for displaying Hebrew combining marks,
    /// such as niqqud vowel points and cantillation symbols—in isolation,
    /// without attachment to a specific letter.
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
    fn cantillation_symbol(&self) -> String;

    /// Returns alternate/transvariant names for this accent, if documented.
    ///
    /// Some accents have variant spellings or names used in different
    /// textual traditions (e.g., BHS variants, manuscript variants).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    /// let accent = ProseAccent::PazerGadol;
    /// if let Some(alts) = accent.alternate_names() {
    ///     println!("Hebrew: {}", alts.hebrew_name);
    ///     println!("English: {}", alts.english_name);
    /// }
    /// ```
    fn alternate_names(&self) -> Option<AlternateNames>;
}

#[cfg(test)]
mod tests1 {
    #[test]
    fn level() {
        use crate::{Accent, GroupLevel, ProseAccent};

        let silluq = ProseAccent::Silluq;

        if let Some(level) = silluq.group_level() {
            assert_eq!(GroupLevel::Tier1, level);
        } else {
            panic!("BUG::wrong grouplevel");
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
