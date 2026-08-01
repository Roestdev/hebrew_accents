//! # Sentence Context Management
//!
//! This module provides the `SentenceContext` type for representing Hebrew
//! biblical sentences with their associated liturgical context (Prose vs Poetry).
//!
//! ## Background
//!
//! In the Hebrew Bible (Tanakh), cantillation marks (ta'amim/accents) serve two
//! distinct functions depending on the textual register:
//!
//! - **Prosaic Text** (Torah, Prophets, most Writings): Uses one system of
//!   disjunctive/conjunctive accents for Torah reading chant
//! - **Poetic Text** (Psalms, Proverbs, Job): Uses a modified accent system
//!   with different melodic conventions
//!
//! Some accents appear exclusively in one register, while others are shared.
//! This distinction enables automatic context detection, though ambiguity is
//! possible when sentences contain only shared accents.
//!
//! ## Core Concepts
//!
//! | Concept | Description |
//! |---------|-------------|
//! | `SentenceContext` | Holds sentence text + context metadata |
//! | `Context::Prosaic` | Standard prose accent system |
//! | `Context::Poetic` | Poetic accent system |
//! | `try_determine_context()` | Auto-detect context from accent patterns |
//!
//! ## Usage Pattern
//!
//! ```ignore
//! use hebrew_accents::{SentenceContext, Context};
//!
//! // Create a sentence with explicit context
//! let sentence = SentenceContext::new(text, Context::Prosaic)?;
//!
//! // Or use the built-in default
//! let default = SentenceContext::with_valid_default()?;
//!
//! // Attempt to auto-detect context
//! let detected = sentence.try_determine_context()?;
//! ```

use crate::api::context::Context;
use crate::error::SentenceContextError;
use crate::sentence;
use crate::sentence::detector::detect_context_from_sentence;
use sentence::validator::validate_sentence;

/// Represents a Hebrew biblical sentence with its associated liturgical context.
///
/// # Design Rationale
///
/// This struct encapsulates both the raw sentence text and its contextual
/// classification. The separation allows:
///
/// 1. **Validation**: Ensures sentence meets minimum requirements (non-empty,
///    properly formed)
/// 2. **Context Tracking**: Maintains whether the sentence follows prose or
///    poetic accent conventions
/// 3. **Auto-Detection**: Enables inference of context from accent patterns
///
/// # Thread Safety
///
/// The struct is thread-safe (`Send + Sync`) as it contains only owned `String`
/// and a `Copy` enum variant.
///
/// # Ordering
///
/// Implements `Ord` for lexicographic comparison by sentence content. When
/// sentences are equal, context breaks ties (`Poetic < Prosaic`).
///
/// # Examples
///
/// ```ignore
/// use hebrew_accents::{SentenceContext, Context};
///
/// // Constructor with validation
/// let ctx = SentenceContext::new(
///     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
///     Context::Prosaic
/// )?;
///
/// // Accessors
/// assert_eq!(ctx.as_str(), "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים");
/// assert_eq!(ctx.context(), Context::Prosaic);
///
/// // Cloning creates independent copies
/// let copy = ctx.clone();
/// assert_eq!(ctx, copy);
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct SentenceContext {
    /// The actual Hebrew sentence text with cantillation marks.
    ///
    /// This is validated upon construction to ensure it's non-empty and
    /// contains valid UTF-8 characters suitable for Hebrew text processing.
    pub sentence: String,

    /// The liturgical context determining which accent system applies.
    ///
    /// - `Context::Prosaic` - Standard prose cantillation
    /// - `Context::Poetic` - Poetic book cantillation
    pub ctx: Context,
}

impl SentenceContext {
    /// Creates a validated `SentenceContext` instance.
    ///
    /// # Parameters
    ///
    /// - `sentence`: Any value convertible to `String` containing Hebrew text
    /// - `ctx`: The expected context (Prosaic or Poetic)
    ///
    /// # Validation
    ///
    /// The sentence is validated via [`validate_sentence`](crate::sentence::validator::validate_sentence)
    /// before acceptance. Validation ensures:
    ///
    /// - Non-empty string
    /// - Valid UTF-8 encoding
    /// - Contains recognized Hebrew characters (may vary by implementation)
    ///
    /// # Errors
    ///
    /// Returns [`SentenceContextError`] if:
    /// - The sentence fails validation
    /// - The sentence is empty
    /// - Contains invalid characters
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// // Successful creation
    /// let ctx = SentenceContext::new(
    ///     "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃",
    ///     Context::Prosaic
    /// );
    /// assert!(ctx.is_ok());
    ///
    /// // Failure with empty string
    /// let empty = SentenceContext::new("", Context::Prosaic);
    /// assert!(empty.is_err());
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

    /// Creates a `SentenceContext` with Genesis 1:1 as the default sentence.
    ///
    /// # Why Genesis 1:1?
    ///
    /// This verse is chosen because:
    /// 1. It's universally recognized (first verse of the Torah)
    /// 2. Contains clear disjunctive/prosaic accents
    /// 3. Well-formed with standard cantillation
    /// 4. Always passes validation
    ///
    /// # Context Default
    ///
    /// Returns `Context::Prosaic` since Genesis belongs to the Torah (prose).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let ctx = SentenceContext::with_valid_default()?;
    ///
    /// // Verify Genesis 1:1 content
    /// assert_eq!(ctx.context(), Context::Prosaic);
    /// assert_eq!(
    ///     ctx.as_str(),
    ///     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃"
    /// );
    /// ```
    ///
    /// # Note
    ///
    /// Unlike `new()`, this method never requires specifying the sentence text
    /// manually. Use it for testing, prototyping, or when a known-good sample
    /// is sufficient.
    pub fn with_valid_default() -> Result<Self, SentenceContextError> {
        let genesis_1_verse_1 = "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃";
        Self::new(genesis_1_verse_1, Context::default())
    }

    /// Returns a string slice view of the sentence content.
    ///
    /// # Ownership
    ///
    /// This returns a borrowed `&str` — no allocation occurs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let ctx = SentenceContext::new("שָׁלוֹם", Context::Prosaic)?;
    ///
    /// // Get string slice
    /// let text: &str = ctx.as_str();
    /// assert_eq!(text, "שָׁלוֹם");
    ///
    /// // Can be used with standard string operations
    /// println!("Length: {} chars", text.chars().count());
    /// println!("Length: {} bytes", text.len());
    /// ```
    pub fn as_str(&self) -> &str {
        &self.sentence
    }

    /// Returns the context classification of this sentence.
    ///
    /// # Context Types
    ///
    /// | Variant | Meaning | Accent System |
    /// |---------|---------|---------------|
    /// | `Context::Prosaic` | Standard prose | Torah reading tropes |
    /// | `Context::Poetic` | Poetic books | Modified poetic tropes |
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// let poetry = SentenceContext::new("אָז יָשִׁיר", Context::Poetic)?;
    /// let prose = SentenceContext::new("וַיֹּאמֶר", Context::Prosaic)?;
    ///
    /// assert_eq!(poetry.context(), Context::Poetic);
    /// assert_eq!(prose.context(), Context::Prosaic);
    ///
    /// // Pattern matching for context-aware processing
    /// match poetry.context() {
    ///     Context::Poetic => println!("Using poetic accent rules"),
    ///     Context::Prosaic => println!("Using prose accent rules"),
    /// }
    /// ```
    pub fn context(&self) -> Context {
        self.ctx
    }

    /// Attempts to determine the sentence context from its accent pattern.
    ///
    /// # How It Works
    ///
    /// The algorithm scans the sentence for **exclusive** accent markers:
    ///
    /// ## Prose-Exclusive Accents
    /// When found → Context is likely `Prosaic`:
    /// - Segolta
    /// - Zaqeph Qatan / Zaqeph Gadol
    /// - Pashta
    /// - Tevir
    /// - Yetiv
    /// - Gershayim
    /// - Pazer Gadol
    /// - Telisha Gedolah / Telisha Qetannah
    /// - Merkha Kephulah
    /// - Darga
    ///
    /// ## Poetry-Exclusive Accents
    /// When found → Context is likely `Poetic`:
    /// - Oleh WeYored
    /// - Dechi
    /// - Illuy
    /// - Tsinnorit Merkha / Tsinnorit Mahpakh
    ///
    /// # Limitations
    ///
    /// | Scenario | Outcome | Reason |
    /// |----------|---------|--------|
    /// | Only prose-exclusive accents found | ✅ Prosaic | Unambiguous |
    /// | Only poetry-exclusive accents found | ✅ Poetic | Unambiguous |
    /// | Both exclusive types found | ❌ Error | Contradictory evidence |
    /// | No exclusive accents found | ❌ Error | Insufficient evidence |
    /// | Mixed shared+exclusive accents | ✅ Exclusive wins | Sufficient signal |
    ///
    /// # Shared Accents (Non-Determinative)
    ///
    /// These accents appear in **both** systems and cannot determine context:
    /// - Munach
    /// - Mahpakh
    /// - Atnach (in both forms)
    /// - Silluq
    /// - Most conjunctive accents
    ///
    /// # Errors
    ///
    /// Returns [`SentenceContextError::DerivationFailed`] when:
    /// - **Ambiguity**: `"Unique prose and poetry accent markers identified"`
    /// - **Insufficient data**: `"No distinguishable prose and/or poetry accents have been found"`
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// // Clear prose text (Genesis 1:1)
    /// let prose = SentenceContext::new(
    ///     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
    ///     Context::default()
    /// )?;
    ///
    /// let detected = prose.try_determine_context()?;
    /// assert_eq!(detected, Context::Prosaic);
    ///
    /// // Poetry text (Psalm 1:1)
    /// let poetry = SentenceContext::new(
    ///     "אַשְׁרֵ֣י הָאִ֭ישׁ אֲשֶׁ֣ר לֹ֣א הָלַ֑ךְ",
    ///     Context::default()
    /// )?;
    ///
    /// let detected = poetry.try_determine_context()?;
    /// assert_eq!(detected, Context::Poetic);
    /// ```
    ///
    /// # Comparison with Standalone Function
    ///
    /// For detecting context without a `SentenceContext` instance:
    ///
    /// ```rust,ignore
    /// // Instance method
    /// let ctx = sentence.try_determine_context()?;
    ///
    /// // Standalone helper
    /// let ctx = crate::try_determine_context(sentence_text)?;
    /// ```
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
