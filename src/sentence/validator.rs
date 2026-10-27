use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

use crate::SentenceContextError;

const MAX_SENTENCE_LENGTH: usize = 7_000;

/// Validates a Hebrew sentence for proper character composition and structure.
///
/// # Leading Whitespace
///
/// The following leading spaces are permitted:
/// - Regular space (U+0020)
/// - No-break space (U+00A0)
/// - Thin space (U+2009)
///
/// Bidirectional control marks (LRM, RLM) and zero-width joiners (ZWJ, ZWNJ)
/// are **NOT** permitted at the beginning of a sentence. Place them
/// between words only.
///
/// Returns `Ok(())` if the sentence:
/// - Has a valid starting consonant (after skipping leading whitespace)
/// - Contains only Hebrew Unicode block characters, valid spacing, and layout marks
/// - Doesn't exceed the maximum length
/// - Contains only a single line (no newlines)
pub(crate) fn validate_sentence(s: &str) -> Result<(), SentenceContextError> {
    // Check for empty sentence
    if s.is_empty() {
        return Err(SentenceContextError::EmptySentence);
    }

    // Max length check
    let char_count = s.chars().count();
    if char_count > MAX_SENTENCE_LENGTH {
        return Err(SentenceContextError::SentenceTooLong(
            MAX_SENTENCE_LENGTH,
            char_count,
        ));
    }

    // Newline check (handles all Unicode line separators)
    if s.contains(|c: char| {
        matches!(
            c,
            '\n' |      // LF - Line Feed (Unix/Linux/macOS)
            '\r' |      // CR - Carriage Return (Classic Mac)
            '\u{000C}' |   // FF (Form Feed) - Page breaks, printers
            '\u{000B}' |   // VT (Vertical Tab)	- Legacy terminals
            '\u{0085}' |   // NEL - Next Line (EBCDIC/IBM)
            '\u{2028}' |   // LS - Line Separator (Unicode)
            '\u{2029}' // PS - Paragraph Separator (Unicode)
        )
    }) {
        return Err(SentenceContextError::MultipleLines);
    }

    // Find first non-whitespace character
    let Some(first_non_ws) = s
        .char_indices()
        .find_map(|(idx, c)| (!c.is_whitespace()).then_some((idx, c)))
    else {
        return Err(SentenceContextError::EmptySentence);
    };

    // Validate first non-whitespace character
    validate_first_non_white_char(first_non_ws.1)?;

    // Validate remaining characters (including first)
    for (idx, c) in s.char_indices().skip(first_non_ws.0) {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }

    Ok(())
}

/// Validates the first non-whitespace character of a sentence.
fn validate_first_non_white_char(c: char) -> Result<(), SentenceContextError> {
    if !is_valid_hebrew_char(c) {
        // Defensive: unreachable given the consonant checks above
        return Err(SentenceContextError::InvalidCharacter(c, 0));
    }

    if is_hbr_consonant_final(c) {
        return Err(SentenceContextError::StartsWithFinalForm(c));
    }

    if !is_hbr_consonant_normal(c) {
        // TODO check if prepositive can be first char
        return Err(SentenceContextError::StartsWithNonConsonant(c));
    }

    Ok(())
}

/// Checks if a character is valid within Hebrew text.
///
/// Accepts:
/// - Hebrew Unicode block (U+0590–U+05FF)
/// - Whitespace characters (various space types)
/// - Paseq alternatives (ASCII pipe)
/// - Meteg layout control characters
fn is_valid_hebrew_char(c: char) -> bool {
    is_hbr_block(c)
        || is_space_like_char(c)
        || is_meteg_layout_char(c)
        || is_paseq_alternative_char(c)
        || is_sof_pasuq_alternative_char(c)
        || is_maqqaph_alternative_char(c)
}

/// Space-like characters (whitespace or bidirectional control marks)
fn is_space_like_char(c: char) -> bool {
    matches!(
        c,
        '\u{0020}' |  // SPACE
        '\u{00A0}' |  // NO-BREAK SPACE
        '\u{200B}' |  // ZERO WIDTH SPACE (accidental web copy)
        '\u{200E}' |  // LEFT-TO-RIGHT MARK
        '\u{200F}' |  // RIGHT-TO-LEFT MARK
        '\u{2009}' |  // THIN SPACE
        '\u{205F}' |  // MEDIUM MATHEMATICAL SPACE
        '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

/// Alternative paseq character (ASCII vertical bar)
fn is_paseq_alternative_char(c: char) -> bool {
    c == '|'
}

/// Alternative sof pasuq character (ASCII semicolon)
fn is_sof_pasuq_alternative_char(c: char) -> bool {
    c == ':'
}
/// Alternative sof pasuq character (ASCII semicolon)
fn is_maqqaph_alternative_char(c: char) -> bool {
    c == '-'
}

/// Layout control characters for meteg positioning
fn is_meteg_layout_char(c: char) -> bool {
    matches!(
        c,
        '\u{034F}' |  // COMBINING GRAPHEME JOINER
        '\u{200C}' |  // ZERO WIDTH NON-JOINER
        '\u{200D}' // ZERO WIDTH JOINER
    )
}

#[cfg(test)]
mod tests {
}
