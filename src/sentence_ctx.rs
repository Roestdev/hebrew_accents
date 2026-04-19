//! Main file

/// Sentence including the context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SentenceContext {
    /// The sentence
    pub sentence: String,
    /// The context of the sentence
    pub ctx: Context,
}

/// Describes the context of a sentence (poetic or prosaic)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[non_exhaustive]
pub enum Context {
    /// The sentence follows a poetic structure (e.g., meter, rhyme).
    Poetic,
    /// The sentence follows ordinary prose conventions.
    #[default]
    Prosaic,
    /// todo
    Unknown,
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
    /// assert_eq!(sentence_context.ctx,Context::Prosaic);
    /// assert_eq!(sentence_context.sentence,"וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ".to_string());
    /// ```
    pub fn new(sentence: &str, ctx: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            ctx,
        }
    }
}

/// Represents a single match if the accent is found
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    /// The matched HebrewAccent
    pub haystack: &'h str,
    /// Start byte of the match
    pub start: usize,
    /// End byte of the match
    pub end: usize,
}

impl<'h> Match<'h> {
    /// Returns the byte offset of the start of the match in the haystack.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    /// Returns the byte offset of the end of the match in the haystack.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    /// Returns true if and only if this match has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    /// Returns the range from start till end (byte offsets)
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }
    /// Returns the substring of the haystack that matched.
    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.haystack[self.range()]
    }
    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    pub(crate) fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {
        Match {
            haystack,
            start,
            end,
        }
    }
}

/// Try to determine the context of the sentence
///
/// Prose: Segolta, Zaqeph Qaton/Gadol, Zarqa,
/// Poetry: Tsinnor
pub fn try_determine_context(sentence: &str) -> Context {
    let _poetry = SentenceContext::new(sentence, Context::Poetic);
    let _prose = SentenceContext::new(sentence, Context::Prosaic);

    //Context::Poetic
    //Context::Prosaic
    Context::Unknown
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn empty_match() {
        let mut m_atch = Match::new("hooiberg", 2, 2);
        assert!(m_atch.is_empty());
        m_atch.end = 4;
        assert!(!m_atch.is_empty());
    }

    // NEW TEST: Exercise try_determine_context function
    #[test]
    fn try_determine_context_returns_unknown() {
        // Currently the function always returns Unknown
        let result = try_determine_context("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים");
        assert_eq!(result, Context::Unknown);
    }

    #[test]
    fn try_determine_context_with_empty_string() {
        let result = try_determine_context("");
        assert_eq!(result, Context::Unknown);
    }

    #[test]
    fn try_determine_context_with_poetic_text() {
        // Even with poetic-looking text, current implementation returns Unknown
        let result = try_determine_context("אֱלֹהִ֑ים צְבָא֖וֹת יְשַׁבְתִּ֣י");
        assert_eq!(result, Context::Unknown);
    }

    // NEW TEST: Exercise Context::Unknown variant
    #[test]
    fn context_unknown_variant_exists() {
        let ctx = Context::Unknown;
        assert_eq!(ctx, Context::Unknown);
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

    // NEW TEST: Test SentenceContext with all Context variants
    #[test]
    fn sentence_context_with_poetic_context() {
        let s = SentenceContext::new("משפט שירי", Context::Poetic);
        assert_eq!(s.ctx, Context::Poetic);
        assert_eq!(s.sentence, "משפט שירי");
    }

    #[test]
    fn sentence_context_with_unknown_context() {
        let s = SentenceContext::new("משפט לא ידוע", Context::Unknown);
        assert_eq!(s.ctx, Context::Unknown);
        assert_eq!(s.sentence, "משפט לא ידוע");
    }

    #[test]
    fn sentence_context_with_prosaic_context() {
        let s = SentenceContext::new("משפט רגיל", Context::Prosaic);
        assert_eq!(s.ctx, Context::Prosaic);
        assert_eq!(s.sentence, "משפט רגיל");
    }

    // NEW TEST: Test Match with edge cases
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

    // NEW TEST: Test Clone, Debug, PartialEq implementations
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
    fn match_debug_formatting() {
        let m = Match::new("test", 0, 2);
        let debug_str = format!("{:?}", m);
        assert!(debug_str.contains("Match"));
    }

    #[test]
    fn sentence_context_debug_formatting() {
        let s = SentenceContext::new("test", Context::Prosaic);
        let debug_str = format!("{:?}", s);
        assert!(debug_str.contains("SentenceContext"));
    }

    // NEW TEST: Test Ord and PartialOrd implementations
    #[test]
    fn context_ord_comparison() {
        assert!(Context::Poetic < Context::Prosaic); // Based on enum order
        assert!(Context::Prosaic < Context::Unknown);
        assert!(Context::Poetic < Context::Unknown);
    }

    #[test]
    fn sentence_context_ord_comparison() {
        let s1 = SentenceContext::new("א", Context::Poetic);
        let s2 = SentenceContext::new("ב", Context::Poetic);
        assert!(s1 < s2); // Lexicographic comparison of sentences
    }

    // NEW TEST: Test Hash implementation
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

        let s1 = SentenceContext::new("test", Context::Prosaic);
        let mut hasher1 = DefaultHasher::new();
        s1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let s2 = SentenceContext::new("test", Context::Prosaic);
        let mut hasher2 = DefaultHasher::new();
        s2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}
