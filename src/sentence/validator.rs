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
// see <https://www.unicode.org/versions/Unicode15.0.0/> section 9.1 for more information
fn is_meteg_layout_char(c: char) -> bool {
    matches!(
        c,
        '\u{034F}' | // CGJ: COMBINING GRAPHEME JOINER
        '\u{200C}' | // ZWNJ: ZERO WIDTH NON-JOINER
        '\u{200D}' // ZWJ: ZERO WIDTH JOINER
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SentenceContextError;

    // ============================================================
    //  VALID INPUTS — expect Ok(())
    // ============================================================

    #[test]
    fn single_consonant_is_valid() {
        assert_eq!(validate_sentence("א"), Ok(()));
    }

    #[test]
    fn single_different_consonant_is_valid() {
        assert_eq!(validate_sentence("ת"), Ok(()));
    }

    #[test]
    fn hebrew_word_is_valid() {
        assert_eq!(validate_sentence("שלום"), Ok(()));
    }

    #[test]
    fn sentence_with_spaces_is_valid() {
        assert_eq!(validate_sentence("שלום עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_nbsp_separator_is_valid() {
        assert_eq!(validate_sentence("שלום\u{00A0}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_thin_space_is_valid() {
        assert_eq!(validate_sentence("שלום\u{2009}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_medium_math_space_is_valid() {
        assert_eq!(validate_sentence("שלום\u{205F}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_ideographic_space_is_valid() {
        assert_eq!(validate_sentence("שלום\u{3000}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_lrm_is_valid() {
        // LRM is not whitespace, so it must come after a valid consonant start
        assert_eq!(validate_sentence("שלום\u{200E}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_rlm_is_valid() {
        assert_eq!(validate_sentence("שלום\u{200F}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_paseq_bar_is_valid() {
        assert_eq!(validate_sentence("שלום|עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_zwj_is_valid() {
        assert_eq!(validate_sentence("שלו\u{200D}ם"), Ok(()));
    }

    #[test]
    fn sentence_with_zwnj_is_valid() {
        assert_eq!(validate_sentence("שלו\u{200C}ם"), Ok(()));
    }

    #[test]
    fn sentence_with_cgj_is_valid() {
        assert_eq!(validate_sentence("שלו\u{034F}ם"), Ok(()));
    }

    #[test]
    fn sentence_with_niqqud_is_valid() {
        // SHEVA (U+05B0) under the first letter — valid, in Hebrew block
        assert_eq!(validate_sentence("א\u{05B0}בני"), Ok(()));
    }

    #[test]
    fn leading_whitespace_then_consonant_is_valid() {
        assert_eq!(validate_sentence("   שלום"), Ok(()));
    }

    #[test]
    fn leading_nbsp_then_consonant_is_valid() {
        assert_eq!(validate_sentence("\u{00A0}שלום"), Ok(()));
    }

    #[test]
    fn final_form_letter_mid_sentence_is_valid() {
        // ך (KAF FINAL) at end of word — allowed everywhere except start
        assert_eq!(validate_sentence("מלך"), Ok(()));
    }

    #[test]
    fn all_final_forms_mid_sentence_are_valid() {
        assert_eq!(validate_sentence("דךדםדןדףדץ"), Ok(()));
    }

    #[test]
    fn maqaf_separator_is_valid() {
        // HEBREW PUNCTUATION MAQAF (U+05BE) is in the Hebrew block
        assert_eq!(validate_sentence("שלום\u{05BE}עולם"), Ok(()));
    }

    #[test]
    fn mixed_valid_chars_all_together_is_valid() {
        let s = "א\u{05B0}בנ\u{05B8}י \u{200E}|שלו\u{200D}ם\u{00A0}";
        assert_eq!(validate_sentence(s), Ok(()));
    }

    // ============================================================
    //  EMPTY / WHITESPACE-ONLY — expect EmptySentence
    // ============================================================

    #[test]
    fn empty_string_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence(""),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn single_space_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence(" "),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn multiple_spaces_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence("     "),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn nbsp_only_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence("\u{00A0}"),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn thin_space_only_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence("\u{2009}"),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn mixed_whitespace_only_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence(" \u{00A0}\u{2009}\u{205F}"),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn ideographic_space_only_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence("\u{3000}"),
            Err(SentenceContextError::EmptySentence)
        );
    }

    // ============================================================
    //  NEWLINE / MULTILINE — expect MultipleLines
    // ============================================================

    #[test]
    fn newline_alone_returns_multiple_lines_error() {
        assert_eq!(
            validate_sentence("\n"),
            Err(SentenceContextError::MultipleLines)
        );
    }

    #[test]
    fn two_line_sentence_returns_multiple_lines_error() {
        assert_eq!(
            validate_sentence("שלום\nעולם"),
            Err(SentenceContextError::MultipleLines)
        );
    }

    #[test]
    fn leading_newline_returns_multiple_lines_error() {
        assert_eq!(
            validate_sentence("\nשלום"),
            Err(SentenceContextError::MultipleLines)
        );
    }

    #[test]
    fn trailing_newline_returns_multiple_lines_error() {
        assert_eq!(
            validate_sentence("שלום\n"),
            Err(SentenceContextError::MultipleLines)
        );
    }

    #[test]
    fn three_line_sentence_returns_multiple_lines_error() {
        assert_eq!(
            validate_sentence("שלום\nעולם\nטוב"),
            Err(SentenceContextError::MultipleLines)
        );
    }

    // ============================================================
    //  STARTS WITH FINAL FORM — expect StartsWithFinalForm(c)
    //  Final forms: ך U+05DA, ם U+05DE, ן U+05E0, ף U+05E3, ץ U+05E5
    // ============================================================

    #[test]
    fn starts_with_kaf_final_returns_final_form_error() {
        assert_eq!(
            validate_sentence("ךדבר"),
            Err(SentenceContextError::StartsWithFinalForm('ך'))
        );
    }

    #[test]
    fn starts_with_mem_final_returns_final_form_error() {
        assert_eq!(
            validate_sentence("מישלא".replace('מ', "ם").as_str()),
            Err(SentenceContextError::StartsWithFinalForm('ם'))
        );
    }

    // #[test]
    // fn starts_with_mem_sofit_directly_returns_final_form_error() {
    //     assert_eq!(
    //         validate_sentence("\u{05DE}דבר"),
    //         Err(SentenceContextError::StartsWithFinalForm('\u{05DE}'))
    //     );
    // }

    // #[test]
    // fn starts_with_nun_final_returns_final_form_error() {
    //     assert_eq!(
    //         validate_sentence("\u{05E0}ושלר"),
    //         Err(SentenceContextError::StartsWithFinalForm('\u{05E0}'))
    //     );
    // }

    #[test]
    fn starts_with_pe_final_returns_final_form_error() {
        assert_eq!(
            validate_sentence("\u{05E3}וס"),
            Err(SentenceContextError::StartsWithFinalForm('\u{05E3}'))
        );
    }

    #[test]
    fn starts_with_tsadi_final_returns_final_form_error() {
        assert_eq!(
            validate_sentence("\u{05E5}רא"),
            Err(SentenceContextError::StartsWithFinalForm('\u{05E5}'))
        );
    }

    #[test]
    fn leading_space_then_final_form_returns_final_form_error() {
        assert_eq!(
            validate_sentence("  \u{05DA}דבר"),
            Err(SentenceContextError::StartsWithFinalForm('\u{05DA}'))
        );
    }

    #[test]
    fn single_final_form_char_returns_final_form_error() {
        assert_eq!(
            validate_sentence("ך"),
            Err(SentenceContextError::StartsWithFinalForm('ך'))
        );
    }

    // ============================================================
    //  STARTS WITH NON-CONSONANT — expect StartsWithNonConsonant(c)
    //  Triggered for: niqqud, Hebrew punctuation, ASCII, digits,
    //  bidi marks (LRM/RLM), etc. — anything that is not a final
    //  form AND not a normal consonant.
    // ============================================================

    #[test]
    fn starts_with_niqqud_returns_non_consonant_error() {
        // SHEVA U+05B0
        assert_eq!(
            validate_sentence("\u{05B0}שלום"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{05B0}'))
        );
    }

    #[test]
    fn starts_with_dagesh_returns_non_consonant_error() {
        // DAGESH / DOT IN LETTER U+05BC
        assert_eq!(
            validate_sentence("\u{05BC}אב"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{05BC}'))
        );
    }

    #[test]
    fn starts_with_maqaf_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("\u{05BE}שלום"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{05BE}'))
        );
    }

    #[test]
    fn starts_with_ascii_letter_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("Hello"),
            Err(SentenceContextError::StartsWithNonConsonant('H'))
        );
    }

    #[test]
    fn starts_with_digit_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("123אב"),
            Err(SentenceContextError::StartsWithNonConsonant('1'))
        );
    }

    #[test]
    fn starts_with_lrm_returns_non_consonant_error() {
        // LRM (U+200E) is valid as a character but not a consonant,
        // and is NOT whitespace so it's picked as first_non_ws
        assert_eq!(
            validate_sentence("\u{200E}שלום"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{200E}'))
        );
    }

    #[test]
    fn starts_with_rlm_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("\u{200F}שלום"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{200F}'))
        );
    }

    #[test]
    fn starts_with_paseq_bar_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("|שלום"),
            Err(SentenceContextError::StartsWithNonConsonant('|'))
        );
    }

    #[test]
    fn leading_space_then_niqqud_returns_non_consonant_error() {
        assert_eq!(
            validate_sentence("  \u{05B0}בני"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{05B0}'))
        );
    }

    // ============================================================
    //  INVALID CHARACTER IN BODY — expect InvalidCharacter(c, idx)
    // ============================================================

    #[test]
    fn latin_in_body_returns_invalid_character_error() {
        assert_eq!(
            validate_sentence("שלוםabc"),
            Err(SentenceContextError::InvalidCharacter('a', 4))
        );
    }

    #[test]
    fn tab_in_body_returns_invalid_character_error() {
        // Tab IS whitespace (skipped by find) but NOT a valid Hebrew char
        assert_eq!(
            validate_sentence("\tשלום"),
            Err(SentenceContextError::InvalidCharacter('\t', 0))
        );
    }

    #[test]
    fn exclamation_in_body_returns_invalid_character_error() {
        assert_eq!(
            validate_sentence("שלום!"),
            Err(SentenceContextError::InvalidCharacter('!', 4))
        );
    }

    #[test]
    fn newline_tab_char_in_body_returns_invalid_character_error() {
        // \r is not "\n" (newline check passes), and \r is whitespace
        // so first_non_ws skips it, but \r is not a valid Hebrew char
        assert_eq!(
            validate_sentence("\rשלום"),
            Err(SentenceContextError::InvalidCharacter('\r', 0))
        );
    }

    #[test]
    fn invalid_char_at_known_index_is_reported() {
        // א(0) ב(1) X(2) ג(3) → first invalid at index 2
        assert_eq!(
            validate_sentence("אבXג"),
            Err(SentenceContextError::InvalidCharacter('X', 2))
        );
    }

    #[test]
    fn invalid_char_after_spaces_at_correct_index() {
        // ' '(0) ' '(1) א(2) ב(3) Z(4)
        assert_eq!(
            validate_sentence("  אבZ"),
            Err(SentenceContextError::InvalidCharacter('Z', 4))
        );
    }

    #[test]
    fn first_invalid_char_is_reported_not_last() {
        // Multiple invalid chars; the first one encountered wins
        assert_eq!(
            validate_sentence("אבXYד"),
            Err(SentenceContextError::InvalidCharacter('X', 2))
        );
    }

    #[test]
    fn emoji_in_body_returns_invalid_character_error() {
        // Multi-byte emoji should report its char and byte/char index
        assert_eq!(
            validate_sentence("שלום😀"),
            Err(SentenceContextError::InvalidCharacter('😀', 4))
        );
    }

    // ============================================================
    //  EDGE CASES
    // ============================================================

    // #[test]
    // fn valid_start_followed_by_invalid_then_valid() {
    //     // Starts valid, hits invalid char, should report that char
    //     assert_eq!(
    //         validate_sentence("א ב ג 1 ד"),
    //         Err(SentenceContextError::InvalidCharacter('1', 7))
    //     );
    // }

    #[test]
    fn only_vertical_bar_returns_non_consonant_error() {
        // Paseq alone: '|' is valid char but not a consonant
        assert_eq!(
            validate_sentence("|"),
            Err(SentenceContextError::StartsWithNonConsonant('|'))
        );
    }

    #[test]
    fn valid_consonant_plus_only_invalid_returns_invalid_char() {
        // Single valid consonant then immediately invalid
        assert_eq!(
            validate_sentence("א#"),
            Err(SentenceContextError::InvalidCharacter('#', 1))
        );
    }
}
