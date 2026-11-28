// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use crate::api::CompoundType;
use crate::{AccentCategory, AccentKind, AlternateNames, CantillationMark, Tier};
/// The `Accent` trait provides a unified interface for working with Hebrew
/// cantillation marks (also known as ta'amim or trope).
///
/// ## Overview
///
/// This trait abstracts over three distinct accent systems:
/// - [`ProseAccent`](crate::ProseAccent) - Used in most biblical books (prosaic texts)
/// - [`PoetryAccent`](crate::PoetryAccent) - Used in poetic books (Psalms, Proverbs, Job)
/// - [`PseudoAccent`](crate::PseudoAccent) - Non-cantillation marks (e.g., Maqqaph, Paseq)
///
/// All three accent systems are wrapped by [`HebrewAccent`](crate::HebrewAccent), which delegates to the
/// appropriate implementation.
///
/// ## Implementation Details
///
/// The trait is automatically implemented for:
/// - [`HebrewAccent`](crate::HebrewAccent) - Delegates to inner variant (Prose/Poetry/Pseudo)
/// - [`ProseAccent`](crate::ProseAccent) - Looked up via a `PROSE_ACCENT_TABLE`
/// - [`PoetryAccent`](crate::PoetryAccent) - Looked up via a `POETRY_ACCENT_TABLE`
/// - [`PseudoAccent`](crate::PseudoAccent) - Looked up via a `PSEUDO_ACCENT_TABLE`
pub trait Accent {
    /// Returns the Hebrew name of the accent.
    ///
    /// # Return Value
    ///
    /// The Hebrew name as a `&'static str`, including niqqud.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
    /// assert_eq!(silluq.hebrew_name(), "סִלּוּק");
    /// ```
    #[must_use]
    fn hebrew_name(&self) -> &'static str;

    /// Returns the semantic meaning/concept of the Hebrew name.
    ///
    /// # Return Value
    ///
    /// A literal English gloss of the Hebrew name as a `&'static str`.
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
    #[must_use]
    fn hebrew_concept(&self) -> &'static str;

    /// Returns the English transliteration of the accent name.
    ///
    /// See [Transliteration Rules](crate::docs::transliteration) for the used rules.
    ///
    /// # Return Value
    ///
    /// The English name (e.g. "Revia Gadol") as a `&'static str`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, PoetryAccent};
    ///
    /// let revia = PoetryAccent::ReviaGadol;
    /// assert_eq!(revia.english_name(), "Revia Gadol");
    /// ```
    #[must_use]
    fn english_name(&self) -> &'static str;

    /// Returns the SBL academic transliteration (Society of Biblical Literature).
    ///
    /// Note: This provides detailed distinctions between sounds and marks,
    /// accounting for dagesh and other diacritical elements.
    ///
    /// # Return Value
    ///
    /// The SBL academic transliteration as a `&'static str`; may differ from
    /// [`english_name`](Self::english_name) for phonetic precision.
    ///
    /// # Reference
    ///
    /// Source: <https://hebrewtransliteration.app/>
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// assert_eq!(silluq.sbl_academic_name(), "sillûq");
    /// ```
    #[must_use]
    fn sbl_academic_name(&self) -> &'static str;

    /// Returns the accent kind (primary or secondary), if applicable.
    ///
    /// This indicates the (primary versus secondary) stress — phonetic only:
    /// the one main ta'am per word vs. auxiliary marks like meteg/ga'ya.
    ///
    /// **Primary accents**
    /// - carry the main stress on a word (e.g., silluq, munach, pashta).
    /// - also perform syntactic work (tipcha marks a break; munach marks a join)
    /// - determine which syllable gets the prominent beat in recitation
    ///
    /// Every accentuated word has exactly one primary accent, whether disjunctive
    /// or conjunctive. This is the main stress of the word, and it simultaneously
    /// does the syntactic work. So in Beckman's framework, "primary" doesn't rank
    /// disjunctives against each other at all — a word with a lowly conjunctive
    /// munach still has that as its primary accent. All accents are equal
    /// candidates for the primary slot.
    ///
    /// **Secondary accents**
    /// - appear on long words with unstressed syllables at some distance from the
    ///   main stress (as a rule of thumb, words with three or more syllables might
    ///   contain a secondary accent)
    /// - have a core job that is phonological, not the musical-syntactic layer
    /// - can also clarify parsing — a classic example is marking a word as a
    ///   construct noun or resolving vocal shewa vs. silent shewa ambiguities
    ///   (two adjacent shewas: which one is vocal? Meteg answers).
    /// - are not part of the cantillation hierarchy
    /// - are sometimes indicated as auxiliary marks
    ///
    /// Secondary accents such as meteg (also called ga'ya) mark weaker stress or
    /// vowel lengthening, often functioning as a "subordinate" stress marker
    /// alongside or in place of another accent. They can indicate where a
    /// secondary stress falls within a word or help resolve ambiguities like
    /// vocal sheva vs. silent sheva.
    ///
    /// ## Note
    ///
    /// In the Masoretic Text of the Tanakh, each word carries a cantillation mark
    /// (ta'am) that fulfills two roles: most marks identify the word's stressed
    /// syllable (typically the ultima), while simultaneously marking the word's
    /// syntactic function as either conjunctive (connecting) or disjunctive
    /// (pausal).
    ///
    /// These are `distinct dimensions` — one phonological, one grammatical — and
    /// should not be conflated.
    ///
    /// # Return Value
    ///
    /// - `Some(AccentKind::Primary)` for primary accents
    /// - `Some(AccentKind::Secondary)` for secondary accents
    /// - `None` for pseudo accents (no stress role)
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, PoetryAccent, PseudoAccent, AccentKind};
    ///
    /// // Primary accent: main stress
    /// let silluq = ProseAccent::Silluq;
    /// assert_eq!(silluq.kind(), Some(AccentKind::Primary));
    ///
    /// // Secondary accent: stress marker
    /// let meteg = PoetryAccent::Meteg;
    /// assert_eq!(meteg.kind(), Some(AccentKind::Secondary));
    ///
    /// // Pseudo accent: no stress marker
    /// let paseq = PseudoAccent::Paseq;
    /// assert_eq!(paseq.kind(), None);
    /// ```
    #[must_use]
    fn kind(&self) -> Option<AccentKind>;

    /// Returns the accent category (disjunctive or conjunctive), if applicable.
    ///
    /// Indicates the word's syntactic relationship to its neighbours.
    ///
    /// - **Disjunctive**: Marks pauses/breaks in the text
    /// - **Conjunctive**: Connects words together
    ///
    /// # Return Value
    ///
    /// - `Some(AccentCategory::Disjunctive)` for disjunctive accents
    /// - `Some(AccentCategory::Conjunctive)` for conjunctive accents
    /// - `None` for pseudo accents (no syntactic function)
    ///
    /// # Example
    ///
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
    #[must_use]
    fn category(&self) -> Option<AccentCategory>;

    /// Returns how this accent composes multiple Unicode codepoints, if it is compound.
    ///
    /// Compound accents combine a primary cantillation mark with a secondary
    /// diacritic, resulting in multiple Unicode codepoints when rendered.
    /// Non-compound accents use a single Unicode codepoint.
    ///
    /// # Return Value
    ///
    /// - `Some(CompoundType)` for compound accents
    /// - `None` for non-compound accents
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, CompoundType};
    ///
    /// let silluq = ProseAccent::Silluq; // Non-compound accent
    /// let shalshelet = ProseAccent::Shalshelet; // Compound accent
    ///
    /// assert_eq!(silluq.compound_type(), None);
    /// assert_eq!(shalshelet.compound_type(), Some(CompoundType::ContainsPaseq));
    /// ```
    #[must_use]
    fn compound_type(&self) -> Option<CompoundType>;

    /// Returns the primary cantillation mark (always present).
    ///
    /// Every accent must have exactly one primary mark.
    ///
    /// # Return Value
    ///
    /// The [`CantillationMark`] carrying the main stress for this accent.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, CantillationMarkPlacement, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let primary = silluq.primary_cantillation_mark();
    ///
    /// assert_eq!(primary.placement, CantillationMarkPlacement::BelowCenter);
    /// ```
    #[must_use]
    fn primary_cantillation_mark(&self) -> CantillationMark;

    /// Returns the secondary cantillation mark, if applicable.
    ///
    /// # Return Value
    ///
    /// - `Some(`[`CantillationMark`]`)` for all compound accents
    /// - `None` for all non-compound accents
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    /// let shalshelet = ProseAccent::Shalshelet;
    ///
    /// if let Some(secondary) = shalshelet.secondary_cantillation_mark() {
    ///     println!("Secondary mark: {}", secondary.symbol); // "׀" (Paseq)
    /// } else {
    ///     println!("This accent is not compound");
    /// }
    /// ```
    #[must_use]
    fn secondary_cantillation_mark(&self) -> Option<CantillationMark>;

    /// Returns (scholarly) notes or context about this accent, if available.
    ///
    /// # Return Value
    ///
    /// - `Some(&'static str)` with scholarly notes, when available
    /// - `None` if no additional notes are documented for this accent
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent};
    ///
    /// let silluq = ProseAccent::Silluq;
    ///
    /// if let Some(notes) = silluq.notes() {
    ///     println!("Notes: {}", notes);
    /// } else {
    ///     println!("No additional notes available");
    /// }
    /// ```
    #[must_use]
    fn notes(&self) -> Option<&'static str>;

    /// Returns the fine-grained disjunctive strength of this accent
    /// (BHS Tabula Accentum ordering), where 1 is the strongest pause and
    /// higher numbers are progressively weaker.
    ///
    /// Ranking uses a numeric scale where `1` represents the strongest
    /// (most dominant) accent, and higher numbers indicate progressively weaker
    /// (more subordinate) accents.
    ///
    /// **Note:** The order given in the BHS (*Biblia Hebraica Stuttgartensia*) is
    /// used; see the [Tabula Accentum](https://www.oakleys.org.uk/files/blog_files/2023/05/tabula_accentuum.pdf)
    /// of the *Biblia Hebraica Stuttgartensia*.
    ///
    /// # Return Value
    ///
    /// - **Disjunctive accents** (Prose and Poetry): `Some(u8)` with the strength ranking
    /// - **Conjunctive accents** (Prose and Poetry): `None` (no ranking)
    /// - **Pseudo accents**: `None` (no ranking)
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, PseudoAccent};
    ///
    /// // Disjunctive accents return strength rankings (1 = strongest)
    /// let silluq = ProseAccent::Silluq;
    /// let atnach = ProseAccent::Atnach;
    /// let legarmeh = ProseAccent::Legarmeh;
    ///
    /// // Lower number means stronger accent
    /// assert_eq!(silluq.rank(), Some(1)); // strongest prose accent
    /// assert_eq!(atnach.rank(), Some(2));
    /// assert_eq!(legarmeh.rank(), Some(18)); // weakest prose accent
    ///
    /// // Compare strength levels programmatically
    /// if let (Some(s1), Some(s2)) = (silluq.rank(), atnach.rank()) {
    ///     assert!(s1 < s2, "Lower rank means stronger pause");
    /// }
    ///
    /// // Conjunctive and pseudo accents return None (no hierarchy)
    /// let munach = ProseAccent::Munach;
    /// let paseq = PseudoAccent::Paseq;
    /// assert_eq!(munach.rank(), None);
    /// assert_eq!(paseq.rank(), None);
    /// ```
    #[must_use]
    fn rank(&self) -> Option<u8>;

    /// Returns the coarse tier level of this accent (Futato classification),
    /// grouping the fine-grained ranks into four tiers for prose (three for poetry).
    ///
    /// - **Disjunctive accents**: Return `Some(Tier)`
    /// - **Conjunctive accents**: Return `None`
    /// - **Pseudo accents**: Return `None`
    ///
    /// Note: poetry accents (Psalms, Proverbs, Job) occupy tiers 1–3 only;
    /// tier 4 exists for prose disjunctives. See [`Tier::max_poetry_tier`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, Tier};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// assert_eq!(silluq.tier(), Some(Tier::Tier1));
    /// ```
    ///
    /// ```rust
    /// use hebrew_accents::{Accent, ProseAccent, Tier};
    ///
    /// let silluq = ProseAccent::Silluq;
    /// let revia = ProseAccent::Revia;
    ///
    /// // Coarse tiers group many accents together...
    /// assert_ne!(silluq.tier(), revia.tier()); // Tier1 vs Tier3
    ///
    /// // ...while ranks separate them more finely.
    /// assert!(silluq.rank().unwrap() < revia.rank().unwrap());
    ///
    /// // Conjunctive accents have neither.
    /// assert_eq!(ProseAccent::Munach.tier(), None);
    /// assert_eq!(ProseAccent::Munach.rank(), None);
    /// ```
    #[must_use]
    fn tier(&self) -> Option<Tier>;

    /// Returns alternate/variant names for this accent, if documented.
    ///
    /// Some accents have variant spellings or names used in different
    /// textual traditions (e.g., BHS variants, manuscript variants).
    ///
    /// # Return Value
    ///
    /// - `Some(AlternateNames)` with the documented variants, when available
    /// - `None` if no alternate names are documented for this accent
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
    #[must_use]
    fn alternate_names(&self) -> Option<AlternateNames>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn level() {
        use crate::{Accent, ProseAccent, Tier};

        let silluq = ProseAccent::Silluq;

        assert_eq!(silluq.tier(), Some(Tier::Tier1));
    }

    #[test]
    fn placement() {
        use crate::{Accent, CantillationMarkPlacement, ProseAccent};

        let silluq = ProseAccent::Silluq;
        let primary = silluq.primary_cantillation_mark();

        assert_eq!(primary.placement, CantillationMarkPlacement::BelowCenter);
    }
}
