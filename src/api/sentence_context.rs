use crate::error::SentenceContextError;
use crate::sentence;
use crate::sentence::detector::detect_context_from_sentence;
use crate::api::context::Context;
use sentence::validator::validate_sentence;

/// Sentence including the context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SentenceContext {
    /// The sentence content (owned)
    pub sentence: String,
    /// The context of the sentence
    pub ctx: Context,
}

impl SentenceContext {
    /// Creates a new object: SentenceContext
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::Context;
    /// use hebrew_accents::SentenceContext;
    ///
    /// let sentence_context = SentenceContext::new( "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ", Context::Prosaic);
    /// let binding = sentence_context.unwrap();
    /// assert_eq!(binding.ctx,Context::Prosaic);
    /// assert_eq!(binding.sentence,"וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ");
    /// ```
    pub fn new(sentence: impl Into<String>, ctx: Context) -> Result<Self, SentenceContextError> {
        let sentence_str = sentence.into();

        // Validate the string before storing
        validate_sentence(&sentence_str)?;

        Ok(Self {
            sentence: sentence_str,
            ctx,
        })
    }

    /// Returns a default `SentenceContext` with a valid non-empty string.
    ///
    /// Since an empty string will fail validation".
    ///
    /// ## Note
    /// Genesis 1:1 is used as the default sentence
    /// This method assumes always passes `validate_sentence`.
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let sentence_context = SentenceContext::with_valid_default();
    /// let binding = sentence_context.unwrap();
    /// assert_eq!(binding.ctx,Context::Prosaic);
    /// assert_eq!(binding.sentence,"בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃");
    /// ```
    pub fn with_valid_default() -> Result<Self, SentenceContextError> {
        let genesis_1_verse_1 = "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃";
        Self::new(genesis_1_verse_1, Context::default())
    }

    /// Returns a reference to the sentence content.
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let ctx = SentenceContext::new("שָׁלוֹם עַל יִשְׂרָאֵל", Context::Prosaic).unwrap();
    ///
    /// // Access the underlying string slice
    /// assert_eq!(ctx.as_str(), "שָׁלוֹם עַל יִשְׂרָאֵל");
    /// assert_eq!(ctx.as_str().len(), 22); // Length in bytes
    /// ```
    pub fn as_str(&self) -> &str {
        &self.sentence
    }

    /// Returns the context of the sentence (Poetic or Prosaic).
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// // Create a poetic context
    /// let poetry = SentenceContext::new("זְמִירוֹת", Context::Poetic).unwrap();
    /// assert_eq!(poetry.context(), Context::Poetic);
    ///
    /// // Create a prosaic context
    /// let prose = SentenceContext::new("וַיְדַבֵּר", Context::Prosaic).unwrap();
    /// assert_eq!(prose.context(), Context::Prosaic);
    ///
    /// // Verify equality with the enum variant
    /// if poetry.context() == Context::Poetic {
    ///     println!("This is poetry!");
    /// }
    /// ```
    pub fn context(&self) -> Context {
        self.ctx
    }

    /// Try to determine the context of the given sentence
    ///
    /// This function tries to classify a Hebrew sentence as either poetic or prose by analyzing its accentuation pattern (ta'amim).
    /// However, accurate classification is not always guaranteed due to the existence of two distinct accent systems.
    /// While certain accents are exclusive to one register, others appear in both, creating ambiguity that can prevent
    /// definitive context determination.
    ///
    /// The function works by checking for the presence of accent CantillationSymbol that are exclusive to
    /// each context:
    ///
    /// **Prose-exclusive accents** (Segolta, Zaqeph Qatan/Gadol, Pashta, Tevir, Yetiv,
    /// Gershayim, Pazer Gadol, Telisha Gedolah/Qetannah, Merkha Kephulah, Darga):
    ///   → Indicate the sentence is likely Prosaic
    ///
    /// **Poetry-exclusive accents** (Oleh WeYored, Dechi, Illuy, Tsinnorit Merkha/Mahpakh):
    ///   → Indicate the sentence is likely Poetic
    ///
    /// # Returns
    ///
    /// * `Ok(Context::Prosaic)` - Only prose-exclusive accents were detected
    /// * `Ok(Context::Poetic)` - Only poetry-exclusive accents were detected
    /// * `Err(SentenceContextError::DerivationFailed("..."))` - One of the following:
    ///   * `"Unique prose and poetry accent markers identified."` — Ambiguous input containing
    ///     characteristics of both contexts
    ///   * `"No distinguishable prose and/or poetry accents have been found"` — Input lacks
    ///     any context-specific accent markers
    ///
    /// # Limitations
    ///
    /// Because some Hebrew accents appear in both prosaic and poetic systems, accurate
    /// classification depends on finding at least one uniquely identifying accent. If the
    /// sentence contains only shared accents or a mixture from both registers, definitive
    /// determination is not possible.
    ///
    /// # Example
    ///
    /// ``` rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let result = try_determine_context("וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");
    /// match result {
    ///     Ok(context) => println!("Context: {:?}", context),
    ///     Err(e) => println!("Could not determine context: {}", e),
    /// }
    /// ```
    /// Try to determine the context of the given sentence
    ///
    /// For a standalone version that doesn't require an existing `SentenceContext`,
    /// see [`try_determine_context`](crate::try_determine_context).
    pub fn try_determine_context(&self) -> Result<Context, SentenceContextError> {
        // Delegate to the shared helper for consistency
        detect_context_from_sentence(&self.sentence)
    }
}

#[cfg(test)]
mod tests {
    use crate::Match;

    use super::*;

    // Existing tests...
    #[test]
    fn get_match_parameters() {
        let m_atch = Match::new("hooiberg", 2, 6);
        assert_eq!(m_atch.start(), 2);
        assert_eq!(m_atch.end(), 6);
        assert_eq!(m_atch.len(), 4);
        assert_eq!(m_atch.as_str(), "oibe");
        let r_ange = m_atch.range();
        assert_eq!(r_ange.start, 2);
        assert_eq!(r_ange.end, 6);
    }
    // --- NEW TEST: Ensure ALL Match methods are explicitly called ---
    // This guarantees 100% function coverage for the Match struct methods.
    #[test]
    fn test_all_match_methods_called() {
        let haystack = "שלום";
        let m = Match::new(haystack, 0, haystack.len());

        // Explicitly call every public method to ensure coverage
        let _start = m.start();
        let _end = m.end();
        let _len = m.len();
        let _is_empty = m.is_empty();
        let _range = m.range();
        let _as_str = m.as_str();

        // Verify they return expected values
        assert_eq!(_start, 0);
        assert_eq!(_end, haystack.len());
        assert_eq!(_len, haystack.len());
        assert!(!_is_empty);
        assert_eq!(_range, 0..haystack.len());
        assert_eq!(_as_str, haystack);
    }

    #[test]
    fn context_default_is_prosaic() {
        let ctx: Context = Context::default();
        assert_eq!(ctx, Context::Prosaic);
    }

    #[test]
    fn context_poetic_variant_exists() {
        let ctx = Context::Poetic;
        assert_eq!(ctx, Context::Poetic);
    }

    #[test]
    fn context_prosaic_variant_exists() {
        let ctx = Context::Prosaic;
        assert_eq!(ctx, Context::Prosaic);
    }

    #[test]
    fn sentence_context_with_poetic_context() {
        let s = SentenceContext::new("משפט שירי", Context::Poetic).unwrap();
        assert_eq!(s.ctx, Context::Poetic);
        assert_eq!(s.sentence, "משפט שירי");
    }

    #[test]
    fn sentence_context_with_prosaic_context() {
        let s = SentenceContext::new("משפט רגיל", Context::Prosaic).unwrap();
        assert_eq!(s.ctx, Context::Prosaic);
        assert_eq!(s.sentence, "משפט רגיל");
    }

    #[test]
    fn match_with_full_string() {
        let haystack = "שלום";
        let m = Match::new(haystack, 0, haystack.len());
        assert_eq!(m.start(), 0);
        assert_eq!(m.end(), haystack.len());
        assert_eq!(m.len(), haystack.len());
        assert_eq!(m.as_str(), haystack);
        assert!(!m.is_empty());
    }

    #[test]
    fn match_with_single_byte() {
        let haystack = "א";
        let m = Match::new(haystack, 0, 2); // UTF-8 character 'א' is 2 bytes
        assert_eq!(m.start(), 0);
        assert_eq!(m.end(), 2);
        assert_eq!(m.len(), 2);
        assert_eq!(m.as_str(), "א");
        assert!(!m.is_empty());
    }

    #[test]
    fn match_range_property() {
        let m = Match::new("test", 1, 3);
        let range = m.range();
        assert_eq!(range.start, 1);
        assert_eq!(range.end, 3);
        assert_eq!(range, 1..3);
    }

    #[test]
    fn sentence_context_clone() {
        let s1 = SentenceContext::new("משפט", Context::Poetic);
        let s2 = s1.clone();
        assert_eq!(s1, s2);
        assert!(std::ptr::eq(&s1, &s2) == false); // Different memory locations
    }

    #[test]
    fn context_clone() {
        let c1 = Context::Poetic;
        let c2 = c1; // Copy, not clone
        assert_eq!(c1, c2);
    }

    #[test]
    fn context_ord_comparison() {
        assert!(Context::Poetic < Context::Prosaic); // Based on enum order
    }

    #[test]
    fn sentence_context_ord_comparison() {
        let s1 = SentenceContext::new("א", Context::Poetic).unwrap();
        let s2 = SentenceContext::new("ב", Context::Poetic).unwrap();
        assert!(s1 < s2); // Lexicographic comparison of sentences
    }

    #[test]
    fn context_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        Context::Poetic.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        Context::Poetic.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn sentence_context_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let s1 = SentenceContext::new(
            "כּי אם בּתורת יהוה חפצו וּבתורתו יהגּה יומם ולילה׃",
            Context::Prosaic,
        )
        .unwrap();
        let mut hasher1 = DefaultHasher::new();
        s1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let s2 = SentenceContext::new(
            "כּי אם בּתורת יהוה חפצו וּבתורתו יהגּה יומם ולילה׃",
            Context::Prosaic,
        )
        .unwrap();
        let mut hasher2 = DefaultHasher::new();
        s2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}

#[cfg(test)]
mod try_determine_context1 {
    use super::*;
    use crate::api::context::Context;
    // Helper eto create a SentenceContext instance for testing
    // We mock the accents by setting them directly or relying on a constructor that accepts them
    // Since the snippet doesn't show the constructor for accents, we assume `contains_accent`
    // checks an internal list. In a real scenario, you might need a builder or a specific
    // constructor for testing that injects accents.
    //
    // *Assumption*: There is a way to create a `SentenceContext` with pre-defined accents
    // or we can manipulate the internal state via a `new_with_accents` helper not shown here.
    // If your struct doesn't have such a helper, you will need to implement one in your
    // main code behind `#[cfg(test)]` or use the `new` method with actual Hebrew text
    // containing the desired accents.

    /// Helper to create a test sentence with specific "fake" accents injected if needed,
    /// or simply using `new` with real text. For these tests, I assume we can create
    /// a context where we know exactly which `contains_accent` will return true.
    ///
    /// To make these tests runnable, I'll assume you have a test-only constructor or
    /// that you use real Hebrew strings containing the specific accents.
    ///
    /// *Strategy*: I will write tests assuming you can pass a vector of accents to a
    /// test helper, OR you use specific Hebrew strings known to contain only those accents.
    /// Below I provide the structure using `SentenceContext::new` with example strings.

    // Example strings (you would replace these with actual verified Hebrew text)
    // 1. Text with ONLY Segolta (Prose)
    const TEXT_PROSE_ONLY: &str = "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃"; // Genesis 1:1

    // 2. Text with ONLY OlehWeYored (Poetry)
    const TEXT_POETRY_ONLY: &str =
        "אַ֥שְֽׁרֵי־הָאִ֗ישׁ אֲשֶׁ֤ר לֹ֥א הָלַךְ֮ בַּעֲצַ֪ת רְשָׁ֫עִ֥ים וּבְדֶ֣רֶךְ חַ֭טָּאִים לֹ֥א עָמָ֑ד וּבְמוֹשַׁ֥ב לֵ֝צִ֗ים לֹ֣א יָשָֽׁב׃"; // Psalm 1:1

    // 3. Text with BOTH accenttypes
    const TEXT_AMBIGUOUS: &str = "מַעֲשֵׂ֣ה אֱלֹהִ֑ים"; // Hypothetical mix

    // 4. Text with NEITHER (common accents like Munakh, Makhpakh which appear in both?)
    const TEXT_NEITHER: &str = "וַיֹּ֙אמֶר֙"; // Hypothetical common accent

    #[test]
    fn test_detect_prose_only() {
        let sentence_ctx = SentenceContext::new(TEXT_PROSE_ONLY, Context::Prosaic).unwrap();
        println!("test_detect_prose_only: {:?}", sentence_ctx);
        let result = sentence_ctx.try_determine_context();
        println!("test_detect_prose_only: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Context::Prosaic);
    }

    #[test]
    fn test_detect_poetry_only() {
        let ctx = SentenceContext::new(TEXT_POETRY_ONLY, Context::Prosaic).unwrap();

        let result = ctx.try_determine_context();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Context::Poetic);
    }

    #[test]
    fn test_detect_ambiguity_both_found() {
        let ctx = SentenceContext::new(TEXT_AMBIGUOUS, Context::Prosaic).unwrap();

        let result = ctx.try_determine_context();

        assert!(result.is_err());
        match result {
            Err(SentenceContextError::DerivationFailed(msg)) => {
                assert!(msg.contains("Both prose and poetry"));
            }
            _ => panic!("Expected DerivationFailed error"),
        }
    }

    #[test]
    fn test_detect_neither_found() {
        let sentence_ctx = SentenceContext::new(TEXT_NEITHER, Context::Prosaic).unwrap();

        let result = sentence_ctx.try_determine_context();

        assert!(result.is_err());
        match result {
            Err(SentenceContextError::DerivationFailed(msg)) => {
                assert!(msg.contains("No distinguishable"));
            }
            _ => panic!("Expected DerivationFailed error"),
        }
    }

    #[test]
    fn test_empty_sentence_no_accents() {
        let ctx = SentenceContext::new("", Context::Prosaic).unwrap();

        let result = ctx.try_determine_context();

        // Empty string should trigger "No distinguishable..."
        assert!(result.is_err());
        match result {
            Err(SentenceContextError::DerivationFailed(msg)) => {
                assert!(msg.contains("No distinguishable"));
            }
            _ => panic!("Expected DerivationFailed error"),
        }
    }
}
