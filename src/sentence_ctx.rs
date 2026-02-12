//! Main file
//!

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
