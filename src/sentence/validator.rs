use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

use crate::SentenceContextError;

pub(crate) fn validate_sentence(s: &str) -> Result<(), SentenceContextError> {
    if s.is_empty() {
        return Err(SentenceContextError::EmptySentence);
    }

    if s.contains("\n") {
        return Err(SentenceContextError::MultipleLines);
    }

    // Find the first NON-WHITESPACE character
    let first_non_ws = s.chars().find(|c| !c.is_whitespace());

    match first_non_ws {
        // No meaningful characters at all (whitespace only)
        None => return Err(SentenceContextError::EmptySentence),

        // First char is a Final Form letter (invalid start)
        // Ranges: 05DA (ך), 05DE (ם), 05E0 (ן), 05E3 (ף), 05E5 (ץ)
        Some(c) if is_hbr_consonant_final(c) => {
            return Err(SentenceContextError::StartsWithFinalForm(c));
        }

        // First char is not a normal Hebrew consonant
        Some(c) if !is_hbr_consonant_normal(c) => {
            return Err(SentenceContextError::StartsWithNonConsonant(c));
        }

        // First char is neither a Hebrew letter nor a niqqud mark
        Some(c) if !is_valid_hebrew_char(c) => {
            return Err(SentenceContextError::InvalidCharacter(c, 0));
        }

        // Valid start (Hebrew consonant), continue full validation
        _ => {}
    }

    // Validate ALL characters in the string
    for (idx, c) in s.chars().enumerate() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }
    Ok(())
}

/// Helper to detect valid hebrew chars
fn is_valid_hebrew_char(c: char) -> bool {
    // Check for Hebrew Unicode Block (U+0590 - U+05FF)
    if is_hbr_block(c) {
        return true;
    }
    // Check for space like characters
    if is_space_like_char(c) {
        return true;
    }
    //
    if is_paseq_alternative_char(c) {
        return true;
    }
    // Check for space like characters
    if is_meteg_layout_char(c) {
        return true;
    }
    false
}

fn is_space_like_char(c: char) -> bool {
    matches!(
        c,
        '\u{0020}' | // SPACE
        '\u{00A0}' | // NO-BREAK SPACE
        '\u{200E}' | // LRM: ZERO WIDTH JOINER
        '\u{200F}' | // RLM: RIGHT-TO-LEFT
        '\u{2009}' | // THIN SPACE
        '\u{205F}' | // MEDIUM MATHEMATICAL SPACE
        '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

//
fn is_paseq_alternative_char(c: char) -> bool {
    matches!(
        c,
        '\u{007C}' // VERTICAL BAR
    )
}

// Check for specific METEG Layout Control Characters
// see https://www.unicode.org/versions/Unicode15.0.0/ section 9.1 for more information
fn is_meteg_layout_char(c: char) -> bool {
    matches!(
        c,
        '\u{034F}' | // CGJ: COMBINING GRAPHEME JOINER
        '\u{200C}' | // ZWNJ: ZERO WIDTH NON-JOINER
        '\u{200D}' // ZWJ: ZERO WIDTH JOINER
    )
}
