use thiserror::Error;

/// Error types for Hebrew sentence validation and context detection.
///
/// This module defines all errors that can be returned by:
/// - [`crate::validate_sentence()`](crate::validate_sentence) - Input validation
/// - [`crate::SentenceContext::try_determine_context()`](crate::SentenceContext::try_determine_context) - Context detection
///
/// # Example
///
/// ```rust
/// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
///
/// // Empty sentence error
/// let result = SentenceContext::new("", Context::Prosaic);
/// assert!(matches!(result, Err(SentenceContextError::EmptySentence)));
///
/// // Final form at start error
/// let result = SentenceContext::new("ךדבר", Context::Prosaic);
/// assert!(matches!(result, Err(SentenceContextError::StartsWithFinalForm(c)) if c == 'ך'));
/// ```
///
/// # Error Categories
///
/// ## Validation Errors
/// | Variant | Triggered When |
/// |---------|----------------|
/// | `EmptySentence` | Input is empty or contains only whitespace |
/// | `MultipleLines` | Input contains newline characters |
/// | `SentenceTooLong` | Input exceeds maximum character limit |
/// | `InvalidCharacter` | Contains characters outside allowed set |
/// | `StartsWithNonConsonant` | First non-whitespace char is not a Hebrew consonant |
/// | `StartsWithFinalForm` | First char is a final-form letter (ך, ם, ן, ף, ץ) |
///
/// ## Detection Errors
/// | Variant | Triggered When |
/// |---------|----------------|
/// | `DerivationFailed` | Insufficient or contradictory accent evidence for context detection |
///
/// # Allowed Characters
///
/// The following character types are permitted in valid Hebrew sentences:
///
/// - **Hebrew Unicode Block** *(U+0590 .. U+05FF)*: Consonants, niqqud, cantillation marks
/// - **Meteg Layout Characters** *(Unicode 15.0, Section 9.1)*:
///   - CGJ: Combining Grapheme Joiner *(U+034F)*
///   - ZWNJ: Zero Width Non-Joiner *(U+200C)*
///   - ZWJ: Zero Width Joiner *(U+200D)*
/// - **Space-like Characters**:
///   - SPACE *(U+0020)*
///   - NO-BREAK SPACE *(U+00A0)*
///   - THIN SPACE *(U+2009)*
///   - MEDIUM MATHEMATICAL SPACE *(U+205F)*
///   - IDEOGRAPHIC SPACE *(U+3000)*
/// - **Bidi Control Characters**:
///   - LRM: Left-to-Right Mark *(U+200E)*
///   - RLM: Right-to-Left Mark *(U+200F)*
/// - **Punctuation**:
///   - Vertical Bar *(U+007C)* - Alternative Paseq representation
///
/// # Derivation Errors
///
/// The `DerivationFailed` error indicates that automatic context detection could not
/// determine whether a sentence follows prosaic or poetic accent conventions:
///
/// - **Ambiguous**: Both prose-exclusive and poetry-exclusive accents found
/// - **Insufficient**: Only shared/common accents present (e.g., Atnach, Silluq)
///
/// See [`crate::SentenceContext::try_determine_context()`] for detailed detection logic.

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SentenceContextError {
    /// Character outside the allowed set was encountered.
    ///
    /// The invalid character and its position in the sentence are reported.
    ///
    /// # Valid Characters
    ///
    /// See [module-level documentation](self#allowed-characters) for the complete list.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// let result = SentenceContext::new("שלוםabc", Context::Prosaic);
    /// assert!(matches!(result, Err(SentenceContextError::InvalidCharacter(c, idx))
    ///                  if c == 'a' && idx == 4));
    /// ```
    #[error("Invalid character '{}' at index {}: only Hebrew, METEG Layout Control Characters, Vertical Bar and whitespace allowed", .0, .1)]
    InvalidCharacter(char, usize),

    /// The sentence was empty or contained only whitespace(s).
    ///
    /// # Resolution
    ///
    /// Provide a non-empty sentence with at least one Hebrew consonant character.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// let result = SentenceContext::new("   ", Context::Prosaic);
    /// assert_eq!(result, Err(SentenceContextError::EmptySentence));
    ///
    /// let valid = SentenceContext::new("שלום", Context::Prosaic);
    /// assert!(valid.is_ok());
    /// ```
    #[error("Sentence cannot be empty or contain only whitespace characters")]
    EmptySentence,

    /// Sentence contains too many characters.
    ///
    /// The first field is the maximum allowed length; the second is the actual character count.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// let long_text: String = "א".repeat(10_001);
    /// let result = SentenceContext::new(long_text, Context::Prosaic);
    /// assert!(matches!(result, Err(SentenceContextError::SentenceTooLong(max, actual))
    ///                  if max < actual));
    /// ```
    #[error("Sentence contains too many characters; allowed: '{}' , actual number: '{}'", .0, .1)]
    SentenceTooLong(usize, usize),

    /// Input contained newline characters.
    ///
    /// Sentences must be single-line only. Multiple lines require separate [`SentenceContext`] instances.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// let multi_line = "שלום\nעולם";
    /// let result = SentenceContext::new(multi_line, Context::Prosaic);
    /// assert_eq!(result, Err(SentenceContextError::MultipleLines));
    /// ```
    #[error("Sentence must be a single line")]
    MultipleLines,

    /// The first non-whitespace character is not a valid Hebrew consonant.
    ///
    /// # Resolution
    ///
    /// Ensure the sentence begins with a normal Hebrew consonant (not niqqud, punctuation,
    /// or control characters). Leading whitespace is automatically skipped.
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// // Invalid: starts with niqqud (Sheva)
    /// let result = SentenceContext::new("\u{05B0}שלום", Context::Prosaic);
    /// assert!(matches!(result, Err(SentenceContextError::StartsWithNonConsonant(c))
    ///                  if c == '\u{05B0}'));
    ///
    /// // Valid: consonant at start
    /// let valid = SentenceContext::new("שׁלוֹם", Context::Prosaic);
    /// assert!(valid.is_ok());
    /// ```
    #[error("Sentence must start with a Hebrew consonant, found '{}'", .0)]
    StartsWithNonConsonant(char),

    /// The first character is a final-form letter (ך, ם, ן, ף, ץ).
    ///
    /// Final-form letters (also called "sofit" letters) are valid only within a sentence,
    /// never at the beginning.
    ///
    /// # Final Form Letters
    ///
    /// | Normal | Final | Unicode |
    /// |--------|-------|---------|
    /// | כ | ך | U+05DA |
    /// | מ | ם | U+05DE |
    /// | נ | ן | U+05E0 |
    /// | פ | ף | U+05E3 |
    /// | צ | ץ | U+05E5 |
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// let result = SentenceContext::new("ךדבר", Context::Prosaic);
    /// assert!(matches!(result, Err(SentenceContextError::StartsWithFinalForm('ך'))));
    ///
    /// // Valid: final form in the middle
    /// let valid = SentenceContext::new("מלך", Context::Prosaic);
    /// assert!(valid.is_ok());
    /// ```
    #[error("Final-form letter '{}' cannot appear at sentence start", .0)]
    StartsWithFinalForm(char),

    /// Ambiguous or insufficient Cantillation Symbol(s) found for context determination.
    ///
    /// This error occurs when [`SentenceContext::try_determine_context()`](crate::SentenceContext::try_determine_context)
    /// cannot distinguish between prosaic and poetic text due to:
    ///
    /// - **Insufficient Data**: Only shared/common accents found (e.g., Munach, Mahpakh)
    /// - **Contradictory Data**: Both prose-exclusive and poetry-exclusive accents present
    ///
    /// # Resolution
    ///
    /// Provide a sentence containing at least one exclusive accent marker:
    ///
    /// | Context | Exclusive Accents |
    /// |---------|-------------------|
    /// | Prosaic | Segolta, Zaqeph Qatan/Gadol, Pashta, Tevir, Yetiv, Gershayim, Pazer Gadol, Telisha Gedolah/Qetannah, Merkha Kephulah, Darga |
    /// | Poetic | Oleh WeYored, Dechi, Illuy, Tsinnorit Merkha/Mahpakh |
    ///
    /// # Example
    ///
    /// ```
    /// use hebrew_accents::{SentenceContext, Context, SentenceContextError};
    ///
    /// // Genesis 1:1 contains only shared accents - detection fails
    /// let genesis = SentenceContext::new("בְּרֵאשִׁית בָּרָא אֱלֹהִים", Context::Prosaic).unwrap();
    /// let result = genesis.try_determine_context();
    /// assert!(matches!(result, Err(SentenceContextError::DerivationFailed(_))));
    ///
    /// // Psalm 1:1 contains poetry-exclusive accents - detection succeeds
    /// let psalm = SentenceContext::new("אַשְׁרֵי־הָאִישׁ...", Context::Prosaic).unwrap();
    /// let context = psalm.try_determine_context().unwrap();
    /// assert_eq!(context, Context::Poetic);
    /// ```
    #[error("Derivation failed: {0}")]
    DerivationFailed(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    /// Helper to compute hash for testing Hash trait
    fn get_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_invalid_character_formatting() {
        let err = SentenceContextError::InvalidCharacter('@', 5);

        assert_eq!(
            err.to_string(),
            "Invalid character '@' at index 5: only Hebrew, METEG Layout Control Characters, Vertical Bar and whitespace allowed"
        );
        assert_eq!(err, SentenceContextError::InvalidCharacter('@', 5));

        // Different index should not be equal
        assert_ne!(err, SentenceContextError::InvalidCharacter('@', 6));

        // Different char should not be equal
        assert_ne!(err, SentenceContextError::InvalidCharacter('$', 5));
    }

    #[test]
    fn test_empty_sentence_formatting() {
        let err = SentenceContextError::EmptySentence;
        assert_eq!(
            err.to_string(),
            "Sentence cannot be empty or contain only whitespace characters"
        );

        // Unit variants should be equal to themselves
        assert_eq!(err, SentenceContextError::EmptySentence);
    }

    #[test]
    fn test_multiple_lines_formatting() {
        let err = SentenceContextError::MultipleLines;
        assert_eq!(err.to_string(), "Sentence must be a single line");
        assert_eq!(err, SentenceContextError::MultipleLines);
    }

    #[test]
    fn test_starts_with_non_consonant_formatting() {
        let sheva = '\u{05B0}'; // Sheva (U+05B0)
        let err = SentenceContextError::StartsWithNonConsonant(sheva);

        assert_eq!(
            err.to_string(),
            "Sentence must start with a Hebrew consonant, found '\u{05B0}'"
        );
        assert_eq!(
            err,
            SentenceContextError::StartsWithNonConsonant('\u{05B0}')
        );

        // Test with space (valid space char but not a consonant)
        let space_err = SentenceContextError::StartsWithNonConsonant(' ');
        assert_eq!(
            space_err.to_string(),
            "Sentence must start with a Hebrew consonant, found ' '"
        );
    }

    #[test]
    fn test_startswith_final_form_formatting() {
        // Test all five final forms
        let finals = ['ך', 'ם', 'ן', 'ף', 'ץ'];

        for &final_char in &finals {
            let err = SentenceContextError::StartsWithFinalForm(final_char);
            let expected = format!(
                "Final-form letter '{}' cannot appear at sentence start",
                final_char
            );

            assert_eq!(err.to_string(), expected);
            assert_eq!(err, SentenceContextError::StartsWithFinalForm(final_char));
        }
    }

    #[test]
    fn test_derivation_failed_formatting() {
        let reason = "Ambiguous accents";
        let err = SentenceContextError::DerivationFailed(reason);

        assert_eq!(err.to_string(), "Derivation failed: Ambiguous accents");
        assert_eq!(
            err,
            SentenceContextError::DerivationFailed("Ambiguous accents")
        );

        // Different messages should not be equal
        assert_ne!(
            err,
            SentenceContextError::DerivationFailed("Different reason")
        );

        // Same message (same &'static str pointer) should be equal
        assert_eq!(
            err,
            SentenceContextError::DerivationFailed("Ambiguous accents")
        );
    }

    #[test]
    fn test_clone_and_copy() {
        // Verify Clone works
        let err = SentenceContextError::InvalidCharacter('א', 0);
        let cloned = err.clone();
        assert_eq!(err, cloned);

        // Verify Copy works (by using it twice without move error)
        let _first_use = SentenceContextError::EmptySentence;
        let _second_use = SentenceContextError::EmptySentence; // This would fail if not Copy
    }

    #[test]
    fn test_hash_consistency() {
        // Two identical errors must produce the same hash
        let err1 = SentenceContextError::InvalidCharacter('!', 0);
        let err2 = SentenceContextError::InvalidCharacter('!', 0);

        assert_eq!(get_hash(&err1), get_hash(&err2));

        // Different errors should produce different hashes (note: collisions are theoretically possible)
        let err3 = SentenceContextError::InvalidCharacter('@', 0);
        let h1 = get_hash(&SentenceContextError::EmptySentence);
        let h2 = get_hash(&SentenceContextError::MultipleLines);
        let h3 = get_hash(&err3);

        // Verify distinct error types have different hashes
        // Note: Hash collisions are extremely unlikely for this small error space
        assert_ne!(h1, h2);
        assert_ne!(h1, h3);
        assert_ne!(h2, h3);

        // Same character at different indices should have different hashes
        assert_ne!(
            get_hash(&err1),
            get_hash(&SentenceContextError::InvalidCharacter('@', 1))
        );
    }

    #[test]
    fn test_debug_trait() {
        let err = SentenceContextError::InvalidCharacter('\u{200F}', 10); // RLM mark
        let debug_str = format!("{:?}", err);
        println!("test_debug_trait: {}", debug_str);

        // Debug output should contain the variant name and data
        assert!(debug_str.contains("InvalidCharacter"));

        // U+200F may be displayed as escaped unicode - accept both upper/lowercase
        assert!(
            debug_str.contains("\\u{200F}") || debug_str.contains("\\u{200f}"),
            "Debug output should contain escaped unicode: {}",
            debug_str
        );

        assert!(debug_str.contains("10"));
    }

    #[test]
    fn test_error_conversion_to_string() {
        // Ensure .to_string() works on all variants
        let errors = vec![
            SentenceContextError::InvalidCharacter('x', 0),
            SentenceContextError::EmptySentence,
            SentenceContextError::MultipleLines,
            SentenceContextError::StartsWithNonConsonant(' '),
            SentenceContextError::StartsWithFinalForm('ץ'),
            SentenceContextError::DerivationFailed("Test"),
        ];

        for e in errors {
            let s = e.to_string();
            assert!(!s.is_empty());
            assert!(s.len() > 5); // Sanity check
        }
    }

    #[test]
    fn test_error_message_contains_meaningful_info() {
        // InvalidCharacter should include the character and index
        let err = SentenceContextError::InvalidCharacter('!', 42);
        let msg = err.to_string();
        assert!(msg.contains('!'));
        assert!(msg.contains("42"));

        // StartsWithFinalForm should include the character
        let err = SentenceContextError::StartsWithFinalForm('ם');
        let msg = err.to_string();
        assert!(msg.contains('ם'));

        // DerivationFailed should include the reason
        let err = SentenceContextError::DerivationFailed("No unique markers");
        let msg = err.to_string();
        assert!(msg.contains("No unique markers"));
    }

    #[test]
    fn test_sentence_too_long_formatting() {
        let err = SentenceContextError::SentenceTooLong(1000, 1500);
        let expected =
            "Sentence contains too many characters; allowed: '1000' , actual number: '1500'";

        assert_eq!(err.to_string(), expected);
        assert_eq!(err, SentenceContextError::SentenceTooLong(1000, 1500));

        // Different limits should not be equal
        assert_ne!(err, SentenceContextError::SentenceTooLong(1000, 2000));
        assert_ne!(err, SentenceContextError::SentenceTooLong(500, 1500));
    }
}
