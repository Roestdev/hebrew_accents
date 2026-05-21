use std::fmt;

/// Define the error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceContextError {
    /// The char and its index
    InvalidCharacter(char, usize),

    /// Forbid empty strings
    EmptySentence,

    /// Only single lines allowed
    MultipleLines,

    /// Context derivation failed
    NoDerivePossible(&'static str),
}

impl fmt::Display for SentenceContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SentenceContextError::InvalidCharacter(c, index) => {
                write!(
                    f,
                    "Invalid character '{}' at index {} (only Hebrew, meteg display, white space and vertical bar is allowed)",
                    c, index
                )
            }
            SentenceContextError::EmptySentence => {
                write!(f, "Sentence cannot be empty")
            }
            SentenceContextError::MultipleLines => {
                write!(f, "Sentence must be a single line")
            }
            SentenceContextError::NoDerivePossible(reason) => {
                write!(f, "Could not derive the context. Reason: {}", reason)
            }
        }
    }
}

impl std::error::Error for SentenceContextError {}

#[cfg(test)]
mod test_display_formatting {
    use super::*;

    // 1. Test Display formatting for InvalidCharacter
    #[test]
    fn test_invalid_character_display() {
        let err = SentenceContextError::InvalidCharacter('@', 5);
        let msg = err.to_string();
        assert_eq!(
             msg,
             "Invalid character '@' at index 5 (only Hebrew, meteg display, white space and vertical bar is allowed)"
         );
    }

    #[test]
    fn test_empty_sentence_display() {
        let err = SentenceContextError::EmptySentence;
        assert_eq!(err.to_string(), "Sentence cannot be empty");
    }

    #[test]
    fn test_multiple_lines_display() {
        let err = SentenceContextError::MultipleLines;
        assert_eq!(err.to_string(), "Sentence must be a single line");
    }

    #[test]
    fn test_no_derive_possible_display() {
        let err = SentenceContextError::NoDerivePossible("missing context data");
        assert_eq!(
            err.to_string(),
            "Could not derive the context. Reason: missing context data"
        );
    }
}

#[cfg(test)]
mod test_debug_formatting {
    use super::*;

    #[test]
    fn test_debug_formatting() {
        let err = SentenceContextError::InvalidCharacter('!', 0);
        let debug_str = format!("{:?}", err.to_string());
        // Debug output includes the variant name and the inner data
        assert!(debug_str.contains("Invalid character"));
        assert!(debug_str.contains("'!'"));
        assert!(debug_str.contains("0"));
    }
}
#[cfg(test)]
mod test_eq_and_partialeq {
    use super::*;

    #[test]
    fn test_equality() {
        // Same values should be equal
        let err1 = SentenceContextError::InvalidCharacter('a', 10);
        let err2 = SentenceContextError::InvalidCharacter('a', 10);
        assert_eq!(err1, err2);

        // Different char should not be equal
        let err3 = SentenceContextError::InvalidCharacter('b', 10);
        assert_ne!(err1, err3);

        // Different index should not be equal
        let err4 = SentenceContextError::InvalidCharacter('a', 11);
        assert_ne!(err1, err4);

        // Different variants should not be equal
        let err5 = SentenceContextError::EmptySentence;
        assert_ne!(err1, err5);
    }
}
#[cfg(test)]
mod test_clone {
    use super::*;

    #[test]
    fn test_clone() {
        let original = SentenceContextError::InvalidCharacter('z', 99);
        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Verify they are distinct instances (though for simple enums this is mostly structural)
        drop(original);
        // Cloned should still be usable
        assert_eq!(cloned.to_string(), "Invalid character 'z' at index 99 (only Hebrew, meteg display, white space and vertical bar is allowed)");
    }
}
#[cfg(test)]
mod test_with_error_simulation {
    use super::*;

    // 8. Integration-style test: Simulating a validation function returning these errors
    #[test]
    fn test_error_construction_in_context() {
        // Simulate a function that might return these errors
        fn validate_char_simulation(c: char, idx: usize) -> Result<(), SentenceContextError> {
            if c == '@' {
                return Err(SentenceContextError::InvalidCharacter(c, idx));
            }
            Ok(())
        }

        // Test failure case
        let result = validate_char_simulation('@', 3);
        assert!(result.is_err());

        if let Err(SentenceContextError::InvalidCharacter(char, idx)) = result {
            assert_eq!(char, '@');
            assert_eq!(idx, 3);
        } else {
            panic!("Expected InvalidCharacter error");
        }

        // Test success case (hypothetically)
        let result_ok = validate_char_simulation('א', 0); // Hebrew letter
        assert!(result_ok.is_ok());
    }
}

#[cfg(test)]
mod test_derive_variants {
    use super::*;

    // 9. Test NoDerivePossible with different static strings
    #[test]
    fn test_no_derive_variants() {
        let err1 = SentenceContextError::NoDerivePossible("reason one");
        let err2 = SentenceContextError::NoDerivePossible("reason two");

        assert_ne!(err1, err2);
        assert!(err1.to_string().contains("reason one"));
        assert!(err2.to_string().contains("reason two"));
    }
}
