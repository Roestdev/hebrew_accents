use std::fmt;
/// Define the error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceContextError {
    /// The char and its index
    InvalidCharacter(char, usize),
    /// Optional: if you want to forbid empty strings
    EmptySentence,
    /// Only single lines allowed
    MultipleLines,
    /// todo
    ContextCanNotBeDetermined,
}

impl fmt::Display for SentenceContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SentenceContextError::InvalidCharacter(c, idx) => {
                write!(f, "Invalid character '{}' at index {}: only Hebrew, whitespace, and Bidi controls allowed.", c, idx)
            }
            SentenceContextError::EmptySentence => {
                write!(f, "Sentence cannot be empty")
            }
            SentenceContextError::MultipleLines => {
                write!(f, "Sentence must be a single line")
            }
            SentenceContextError::ContextCanNotBeDetermined => {
                write!(f, "Could not determine the context (Prosaic vs Poetic)")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a formatter for Display tests
    fn format_error(err: SentenceContextError) -> String {
        err.to_string()
    }

    #[test]
    fn test_invalid_character_display() {
        let err = SentenceContextError::InvalidCharacter('@', 5);
        let msg = format_error(err);

        assert_eq!(
            msg,
            "Invalid character '@' at index 5: only Hebrew, whitespace, and Bidi controls allowed."
        );
    }

    #[test]
    fn test_invalid_character_debug() {
        let err = SentenceContextError::InvalidCharacter('א', 10);
        let debug_str = format!("{:?}", err);

        // Verify Debug output includes the variant name and data
        assert!(debug_str.contains("InvalidCharacter"));
        assert!(debug_str.contains("'א'"));
        assert!(debug_str.contains("10"));
    }

    #[test]
    fn test_empty_sentence_display() {
        let err = SentenceContextError::EmptySentence;
        let msg = format_error(err);

        assert_eq!(msg, "Sentence cannot be empty");
    }

    #[test]
    fn test_multiple_lines_display() {
        let err = SentenceContextError::MultipleLines;
        let msg = format_error(err);

        assert_eq!(msg, "Sentence must be a single line");
    }

    #[test]
    fn test_context_undetermined_display() {
        let err = SentenceContextError::ContextCanNotBeDetermined;
        let msg = format_error(err);

        // Note: Your current impl prints "Sentence must be a single line" for this too.
        // You might want to update the Display impl to be more specific later.
        assert_eq!(msg, "Could not determine the context (Prosaic vs Poetic)");
    }

    #[test]
    fn test_partial_eq() {
        let err1 = SentenceContextError::InvalidCharacter('!', 0);
        let err2 = SentenceContextError::InvalidCharacter('!', 0);
        let err3 = SentenceContextError::InvalidCharacter('?', 0);
        let err4 = SentenceContextError::EmptySentence;

        // Same variant and data
        assert_eq!(err1, err2);

        // Same variant and different char position
        assert_ne!(err1, err3);

        // Different data (index)
        assert_ne!(err1, SentenceContextError::InvalidCharacter('!', 1));

        // Different variant
        assert_ne!(err1, err4);
    }

    #[test]
    fn test_clone() {
        let err = SentenceContextError::InvalidCharacter('ז', 42);
        let cloned = err.clone();

        assert_eq!(err, cloned);
    }

    #[test]
    fn test_special_characters_in_error() {
        // Test with a Hebrew character to ensure formatting handles non-ASCII correctly
        let err = SentenceContextError::InvalidCharacter('ב', 1);
        let msg = format_error(err);

        assert!(msg.contains("ב"));
        assert!(msg.contains("1"));
    }
}
