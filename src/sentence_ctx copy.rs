// To add validation that ensures the sentence field contains only Hebrew characters, whitespace, and valid UTF-8 bidirectional (Bidi) control characters, you should implement a custom constructor (often called new or try_new) and a validation helper function.Since Rust's String is already UTF-8 compliant, we don't need to check for "UTF-8 validity" specifically, but we do need to ensure the characters fall within the allowed sets.Here is the recommended approach:1. Define the Validation Logic
// We need to check three categories of characters:

// Hebrew Unicode Block: U+0590 to U+05FF.
// Whitespace: Standard ASCII whitespace plus Unicode whitespace (using char::is_whitespace).
// Bidi Control Characters: Specifically the Unicode Bidirectional Algorithm controls (e.g., LRE, RLE, PDF, LRO, RLO, etc.). These are mostly in the U+2000–U+206F range (General Punctuation) and U+FE00–U+FE0F (Variation Selectors), but the specific Bidi controls are:

// U+202A (LRE)
// U+202B (RLE)
// U+202C (PDF)
// U+202D (LRO)
// U+202E (RLO)
// U+2066 (LRI)
// U+2067 (RLI)
// U+2068 (FSI)
// U+2069 (PDI)



// 2. Implementation
// Here is how you can modify your struct to include a safe constructor:
// use std::fmt;

// // Define the error type for validation failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SentenceContextError {
    InvalidCharacter(char, usize), // The char and its index
    EmptySentence,                 // Optional: if you want to forbid empty strings
    MultipleLines,                 // Only single lines allowed
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
        }
    }
}

impl std::error::Error for SentenceContextError {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SentenceContext {
    pub sentence: String,
    pub ctx: Context, // Assuming Context is defined elsewhere
}

impl SentenceContext {
    /// Creates a new SentenceContext with validation.
    /// Returns Err if the string contains invalid characters.
    pub fn new(sentence: String, ctx: Context) -> Result<Self, SentenceContextError> {
        validate_sentence(&sentence)?;
        Ok(Self { sentence, ctx })
    }

    /// Convenience method to create from &str
    pub fn try_from_str(s: &str, ctx: Context) -> Result<Self, SentenceContextError> {
        Self::new(s.to_string(), ctx)
    }
}

fn validate_sentence(s: &str) -> Result<(), SentenceContextError> {
    if s.is_empty() {
        // Decide if empty is allowed. If yes, remove this block.
        return Err(SentenceContextError::EmptySentence);
    }

    for (idx, c) in s.chars().enumerate() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }
    Ok(())
}

// fn is_valid_hebrew_char(c: char) -> bool {
//     // 1. Check for Hebrew Unicode Block (U+0590 - U+05FF)
//     if ('\u{0590}'..='\u{05FF}').contains(&c) {
//         return true;
//     }

//     // 2. Check for Whitespace (includes space, tab, newline, etc.)
//     if c.is_whitespace() {
//         return true;
//     }

//     // 3. Check for specific Bidi Control Characters
//     // These are the explicit controls used to force directionality
//     matches!(c, 
//         '\u{202A}' | // LRE: Left-to-Right Embedding
//         '\u{202B}' | // RLE: Right-to-Left Embedding
//         '\u{202C}' | // PDF: Pop Directional Formatting
//         '\u{202D}' | // LRO: Left-to-Right Override
//         '\u{202E}' | // RLO: Right-to-Left Override
//         '\u{2066}' | // LRI: Left-to-Right Isolate
//         '\u{2067}' | // RLI: Right-to-Left Isolate
//         '\u{2068}' | // FSI: First Strong Isolate
//         '\u{2069}'   // PDI: Pop Directional Isolate
//     )
// }

// // Example usage
fn main() {
    let dummy_ctx = Context::default(); // Replace with your actual Context construction

    // Valid: Hebrew + Space + Bidi Control
    let valid = SentenceContext::new("שלום עולם \u{202E}".to_string(), dummy_ctx.clone());
    println!("{:?}", valid);

    // Invalid: Contains Latin 'a'
    let invalid = SentenceContext::new("שלום a".to_string(), dummy_ctx);
    println!("{:?}", invalid);
}
// Key Design Decisions


// Result<T, E> Pattern:
// Instead of panicking, the new function returns a Result. This is idiomatic Rust for validation. It forces the caller to handle the case where the input is invalid.


// Character Iteration:
// We iterate using .chars().enumerate(). This is crucial because String is UTF-8 encoded, and iterating by bytes would break multi-byte characters (like Hebrew letters). Iterating by char ensures we check each logical Unicode scalar value.


// Bidi Controls:
// I explicitly listed the specific Bidi control characters. While there are many Unicode characters, only a specific subset are "Bidi controls" used to manipulate text direction. Including all of them prevents accidental inclusion of unrelated symbols that happen to be in the same Unicode block.


// Whitespace:
// Using c.is_whitespace() covers standard spaces, tabs, and non-breaking spaces, which is usually safer than checking for ' ' only.


// Alternative: Using a Crate
// If you prefer not to maintain the list of Bidi characters manually, you can use the unicode-bidi crate, which implements the Unicode Bidirectional Algorithm. However, for simple validation of allowed characters, the manual matches! macro is often faster and has fewer dependencies.
// If you need to check for any Unicode Bidi property (not just the controls), you would need to check the bidi_class from the unicode-bidi crate, but usually, strict input validation prefers allowing only the explicit control codes listed above.
// How to use this in your API
// Users will now call:
// match SentenceContext::new(user_input.to_string(), my_context) {
//     Ok(ctx) => { /* Proceed */ },
//     Err(e) => { /* Handle error, e.g., show message to user */ }
// }
// This ensures your SentenceContext is always in a valid state (an invariant) once constructed.