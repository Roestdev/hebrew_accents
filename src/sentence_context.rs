//! sdf [`asfd`] dff
//!
//! iyuFDMFOPOKPOMOIPSADSS  ssdf sdf sf weew fweflwf wel

/// Sentence including the context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SentenceContext {
    pub sentence: String,
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

// TODO add description
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    pub haystack: &'h str,
    pub start: usize,
    pub end: usize,
}

impl<'h> Match<'h> {
    /// Returns the byte offset of the start of the match in the haystack. The
    /// start of the match corresponds to the position where the match begins
    /// and includes the first byte in the match.
    ///
    /// It is guaranteed that `Match::start() <= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the byte offset of the end of the match in the haystack. The
    /// end of the match corresponds to the byte immediately following the last
    /// byte in the match. This means that `&slice[start..end]` works as one
    /// would expect.
    ///
    /// It is guaranteed that `Match::start() <= Match::end()`.
    ///
    /// This is guaranteed to fall on a valid UTF-8 codepoint boundary. That
    /// is, it will never be an offset that appears between the UTF-8 code
    /// units of a UTF-8 encoded Unicode scalar value. Consequently, it is
    /// always safe to slice the corresponding haystack using this offset.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns true if and only if this match has a length of zero.
    ///
    /// Note that an empty match can only occur when the regex itself can
    /// match the empty string. Here are some examples of regexes that can
    /// all match the empty string: `^`, `^$`, `\b`, `a?`, `a*`, `a{0}`,
    /// `(foo|\d+|quux)?`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns the range over the starting and ending byte offsets of the
    /// match in the haystack.
    ///
    /// It is always correct to slice the original haystack searched with this
    /// range. That is, because the offsets are guaranteed to fall on valid
    /// UTF-8 boundaries, the range returned is always valid.
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

// impl<'h> core::fmt::Debug for Match<'h> {
//     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
//         f.debug_struct("Match")
//             .field("start", &self.start)
//             .field("end", &self.end)
//             .field("string", &self.as_str())
//             .finish()
//     }
// }

impl<'h> From<Match<'h>> for &'h str {
    fn from(m: Match<'h>) -> &'h str {
        m.as_str()
    }
}

impl<'h> From<Match<'h>> for core::ops::Range<usize> {
    fn from(m: Match<'h>) -> core::ops::Range<usize> {
        m.range()
    }
}

// TODO add description
// fn option_index: Option<usize>) -> Option<Match<'static>> {
//     if let Some(index) = option_index {
//         // all single characters accents consists two bytes, so
//         // the next character will be located at index+2'
//         Some(Match::new(
//             "TODO: insert single character",
//             index,
//             index + 2,
//         ))
//     } else {
//         None
//     }
//}

// fn option_index: Option<usize>) -> Option<Match<'static>> {
//     option_index.map(|index| Match::new("TODO: insert single character", index, index + 2))
// }
