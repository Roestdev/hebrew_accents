use thiserror::Error;

/// Defines the error type for validation if returned
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SentenceContextError {
    /// Character outside the allowed set was encountered
    ///
    /// The invalid character and its position in the senctence will be showed
    ///
    ///
    /// ## Valid characters are:
    /// - Hebrew Unicode Codeblock *(U+0590 .. U+05FF)*, **except** the ones labelled as 'NOT USED'
    /// - Meteg layout characters (CGJ, ZWNJ and ZWJ):
    ///   - CGJ: COMBINING GRAPHEME JOINER *(U+034F)*
    ///   - ZWNJ: ZERO WIDTH NON-JOINER *(U+200C)*
    ///   - ZWJ: ZERO WIDTH JOINER *(U+200D)*
    /// 
    ///   (see <https://www.unicode.org/versions/Unicode15.0.0/> section 9.1 for more information)
    /// - Vertical bar *(U+007C)*, some times used as an alternative Paseq in computer text.
    /// - The follwoing "space" characters:
    ///   - SPACE *(U+0020)*
    ///   - NO-BREAK SPACE *(U+00A0)* 
    ///   - LRM: ZERO WIDTH JOINER *(U+200E)* 
    ///   - RLM: RIGHT-TO-LEFT *(U+200F)* 
    ///   - THIN SPACE *(U+2009)* 
    ///   - MEDIUM MATHEMATICAL SPACE *(U+205F)* 
    ///   - IDEOGRAPHIC SPACE *(U+3000)* 
    #[error("Invalid character '{}' at index {}: only Hebrew, METEG Layout Control Characters, Vertical Bar and whitespace allowed", .0, .1)]
    InvalidCharacter(char, usize),

    /// The sentence was empty or contained only whitespace(s)
    #[error("Sentence cannot be empty or contain only whitespace characters")]
    EmptySentence,

    /// Input contained newline characters
    #[error("Sentence must be a single line")]
    MultipleLines,

    /// The first character is not a valid Hebrew consonant
    #[error("Sentence must start with a Hebrew consonant, found '{}'", .0)]
    StartsWithNonConsonant(char),

    /// The first character is a final-form letter (ך, ם, ן, ף, ץ)
    #[error("Final-form letter '{}' cannot appear at sentence start", .0)]
    StartsWithFinalForm(char),

    /// Ambiguous or insufficient Cantillation Symbol(s) found for context determination
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

        assert_eq!(err.to_string(), "Invalid character '@' at index 5: only Hebrew, METEG Layout Control Characters, Vertical Bar and whitespace allowed");
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
        let vowel = 'ְ'; // Sheva (U+05B0)
        let err = SentenceContextError::StartsWithNonConsonant(vowel);

        assert_eq!(
            err.to_string(),
            "Sentence must start with a Hebrew consonant, found 'U+05B0}'"
        );
        assert_eq!(err, SentenceContextError::StartsWithNonConsonant('ְ'));

        // Test with a space (often treated as non-consonant start in loose parsing, though usually whitespace skipped)
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

        // String slice comparison
        assert_ne!(
            err,
            SentenceContextError::DerivationFailed("Different reason")
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

        // Different errors must likely produce different hashes
        let err3 = SentenceContextError::InvalidCharacter('@', 0);
        let h1 = get_hash(&SentenceContextError::EmptySentence);
        let h2 = get_hash(&SentenceContextError::MultipleLines);
        let h3 = get_hash(&err3);

        // Verify distinct error accenttypes have different hashes
        assert_ne!(h1, h2);
        assert_ne!(h1, h3); // EmptySentence vs InvalidCharacter('!', 0)
        assert_ne!(h2, h3); // MultipleLines vs InvalidCharacter('@', 0)

        // Same character at different indices should have different hashes
        assert_ne!(
            get_hash(&err1),
            get_hash(&SentenceContextError::InvalidCharacter('@', 1))
        );
    }

    #[test]
    fn test_debug_trait() {
        let err = SentenceContextError::InvalidCharacter('\u{200F}', 10); // RTL mark
        let debug_str = format!("{:?}", err);
        println!("test_debug_trait: {}", debug_str);

        // Debug output should contain the variant name and data
        assert!(debug_str.contains("InvalidCharacter"));
        assert!(debug_str.contains("\\u{200f}")); // escape the '/'
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
}
