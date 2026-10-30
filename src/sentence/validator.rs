use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

use crate::SentenceContextError;

const MAX_SENTENCE_LENGTH: usize = 7_000;

/// Validates a Hebrew sentence for proper character composition and structure.
///
/// # First character
///
/// All sentences **MUST** start with normal consonant, and not a final consonant.
///
/// Bidirectional control marks (LRM, RLM) and zero-width joiners (ZWJ, ZWNJ)
/// are **NOT** permitted at the beginning of a sentence. Place them
/// between words only.
///
/// # Result
///
/// Returns `Ok(())` if the sentence:
/// - Has a valid starting consonant
/// - Contains only:
///   - Hebrew Unicode block characters,
///   - valid spacing,
///   - Meteg layout marks
///   - Alternative Paseq (ASCII Vertical bar)
///   - Alternative SofPasuq (ASCII Colon)
///   - Alternative Maqqaph (ASCII Hyphen-minus)
/// - Doesn't exceed the maximum length
/// - Contains only a single line (no newlines)
///
/// Returns an Error if the conditions are not met
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

    let mut rest = s.char_indices();
    let (_, first_char) = rest.next().expect("emptiness checked above");

    // First character is final form
    if is_hbr_consonant_final(first_char) {
        return Err(SentenceContextError::StartsWithFinalForm(first_char));
    }
    // First character is not a consonant
    if !is_hbr_consonant_normal(first_char) {
        return Err(SentenceContextError::StartsWithNonConsonant(first_char));
    }
    // Validate all other characters in the sentence
    for (idx, c) in rest {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }

    Ok(())
}

/// Checks if a character is valid within Hebrew text.
///
/// Accepts:
/// - Hebrew Unicode block (U+0590–U+05FF), **except** the ones marked as 'NOT USED'
/// - A selection of whitespace like characters
/// - A selection of bidi
/// - A selection of whitespace like characters
/// - A selection of whitespace like characters
/// - Meteg layout control characters
/// - Alternative Paseq (ASCII pipe)
/// - Alternative SofPasuq (ASCII Colon)
/// - Alternative Maqqaph (ASCII ASCII Hyphen-minus)
fn is_valid_hebrew_char(c: char) -> bool {
    is_hbr_block(c)
        || is_permitted_space_char(c)
        || is_permitted_bidi_char(c)
        || is_meteg_layout_char(c)
        || is_paseq_alternative_char(c)
        || is_sof_pasuq_alternative_char(c)
        || is_maqqaph_alternative_char(c)
}

/// Space-like characters (whitespace or bidirectional control marks)
/// the list is deliberate and closed
fn is_permitted_space_char(c: char) -> bool {
    matches!(
        c,
        '\u{0020}' |  // SPACE
        '\u{00A0}' |  // NO-BREAK SPACE
        '\u{200B}' |  // ZERO WIDTH SPACE (accidental web copy)
        '\u{2009}' |  // THIN SPACE
        '\u{205F}' |  // MEDIUM MATHEMATICAL SPACE
        '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

fn is_permitted_bidi_char(c: char) -> bool {
    matches!(
        c,
        '\u{200E}' |  // LEFT-TO-RIGHT MARK
        '\u{200F}' // RIGHT-TO-LEFT MARK
    )
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

/// Alternative Paseq character (ASCII Vertical bar)
fn is_paseq_alternative_char(c: char) -> bool {
    c == '|' // U+007C
}

/// Alternative Sof Pasuq character (ASCII Colon)
fn is_sof_pasuq_alternative_char(c: char) -> bool {
    c == ':' // U+003A
}
/// Alternative maqqaph character (ASCII Hyphen-minus)
fn is_maqqaph_alternative_char(c: char) -> bool {
    c == '-' // U+002D
}

#[cfg(test)]
mod tests {}
