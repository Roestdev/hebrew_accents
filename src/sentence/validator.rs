use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

use crate::SentenceContextError;

const MAX_SENTENCE_LENGTH: usize = 3_000;

/// Validates a Hebrew sentence for proper character composition and structure.
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

    // Newline check (only LF triggers this specific error)
    if s.contains('\n') {
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
    validate_first_char(first_non_ws.1)?;

    // Validate remaining characters (including first)
    for (idx, c) in s.char_indices() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }

    Ok(())
}

/// Validates the first non-whitespace character of a sentence.
fn validate_first_char(c: char) -> Result<(), SentenceContextError> {
    if is_hbr_consonant_final(c) {
        return Err(SentenceContextError::StartsWithFinalForm(c));
    }

    if !is_hbr_consonant_normal(c) {
        return Err(SentenceContextError::StartsWithNonConsonant(c));
    }

    if !is_valid_hebrew_char(c) {
        // Defensive: unreachable given the consonant checks above
        return Err(SentenceContextError::InvalidCharacter(c, 0));
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
        || is_paseq_alternative_char(c)
        || is_meteg_layout_char(c)
}

/// Space-like characters (whitespace or bidirectional control marks)
fn is_space_like_char(c: char) -> bool {
    matches!(
        c,
        '\u{0020}' |  // SPACE
        '\u{00A0}' |  // NO-BREAK SPACE
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
    use super::*;

    // ============================================================
    // VALID INPUTS — expect Ok(())
    // ============================================================

    #[test]
    fn single_consonant_is_valid() {
        assert_eq!(validate_sentence("א"), Ok(()));
    }

    #[test]
    fn hebrew_word_with_niqqud_is_valid() {
        assert_eq!(validate_sentence("אֶלֶף"), Ok(()));
    }

    #[test]
    fn sentence_with_regular_spaces_is_valid() {
        assert_eq!(validate_sentence("שלום עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_various_spaces_is_valid() {
        let spaces = ['\u{00A0}', '\u{2009}', '\u{205F}', '\u{3000}'];
        for &space in &spaces {
            let sentence = format!("שלום{}עולם", space);
            assert_eq!(validate_sentence(&sentence), Ok(()));
        }
    }

    #[test]
    fn sentence_with_bidi_marks_in_body_is_valid() {
        assert_eq!(validate_sentence("שלום\u{200E}עולם"), Ok(()));
        assert_eq!(validate_sentence("שלום\u{200F}עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_cjk_spaces_is_valid() {
        assert_eq!(validate_sentence("שלו\u{200D}ם"), Ok(())); // ZWJ
        assert_eq!(validate_sentence("שלו\u{200C}ם"), Ok(())); // ZWNJ
        assert_eq!(validate_sentence("שלו\u{034F}ם"), Ok(())); // CGJ
    }

    #[test]
    fn maqaf_connects_words_is_valid() {
        assert_eq!(validate_sentence("שלום־עולם"), Ok(()));
    }

    #[test]
    fn sentence_with_cantillation_is_valid() {
        assert_eq!(validate_sentence("א\u{0592}ב\u{0591}ג"), Ok(()));
    }

    #[test]
    fn sentence_with_leading_whitespace_is_valid() {
        assert_eq!(validate_sentence("   שלום"), Ok(()));
    }

    #[test]
    fn sentence_with_trailing_whitespace_is_valid() {
        assert_eq!(validate_sentence("שלום   "), Ok(()));
    }

    #[test]
    fn sentence_with_mixed_valid_chars_is_valid() {
        let mixed = "א\u{05B0}בנ\u{05B8}י \u{200E}|שלו\u{200D}ם\u{00A0}";
        assert_eq!(validate_sentence(mixed), Ok(()));
    }

    // ============================================================
    // EMPTY / WHITESPACE-ONLY — expect EmptySentence
    // ============================================================

    #[test]
    fn empty_string_returns_empty_sentence_error() {
        assert_eq!(
            validate_sentence(""),
            Err(SentenceContextError::EmptySentence)
        );
    }

    #[test]
    fn various_whitespace_only_returns_empty_sentence_error() {
        for ws in [" ", "\u{00A0}", "\t", "\r"] {
            assert_eq!(
                validate_sentence(ws),
                Err(SentenceContextError::EmptySentence)
            );
        }
    }

    // ============================================================
    // MULTILINE — expect MultipleLines
    // ============================================================

    #[test]
    fn newline_anywhere_returns_multiple_lines_error() {
        let variants = ["\nשלום", "שלום\n", "שלום\nעולם"];
        for s in variants {
            assert_eq!(
                validate_sentence(s),
                Err(SentenceContextError::MultipleLines)
            );
        }
    }

    // ============================================================
    // START VALIDATION ERRORS
    // ============================================================

    // Final forms at start
    #[test]
    fn starts_with_final_forms_returns_final_form_error() {
        let final_form_test_cases = [
            ('\u{05DA}', "Kaf Sofit"),
            ('\u{05DD}', "Mem Sofit"),
            ('\u{05DF}', "Nun Sofit"),
            ('\u{05E3}', "Pe Sofit"),
            ('\u{05E5}', "Tsadi Sofit"),
        ];

        for &(c, name) in &final_form_test_cases {
            let sentence = format!("{}שד שב", c);
            assert_eq!(
                validate_sentence(&sentence), // Pass &str reference to String
                Err(SentenceContextError::StartsWithFinalForm(c)), // c is char
                "{} failed",
                name
            );
        }
    }

    // Non-consonants at start
    #[test]
    fn starts_with_varying_non_consonants_returns_non_consonant_error() {
        let test_cases = [
            ('\u{05B0}', "Sheva"),
            ('\u{0592}', "Segol"),
            ('\u{05BC}', "Dagesh"),
            ('\u{05BE}', "Maqaf"),
            ('H', "ASCII"),
            ('1', "Digit"),
            ('\u{200E}', "LRM"),
            ('\u{200F}', "RLM"),
            ('|', "Pipe"),
        ];

        for &(c, name) in &test_cases {
            let sentence = format!("{}ב", c);
            assert_eq!(
                validate_sentence(&sentence), // Pass &str reference to String
                Err(SentenceContextError::StartsWithNonConsonant(c)), // c is char
                "{} failed",
                name
            );
        }
    }

    // ============================================================
    // BODY VALIDATION ERRORS
    // ============================================================

    #[test]
    fn invalid_characters_reported_with_correct_index() {
        let test_cases = [
            ("\tשלום", '\t', 0, "Tab"),
            ("אבXג", 'X', 4, "Latin X"),
            ("שלום!", '!', 8, "Exclamation"),
            ("אבYדZ", 'Y', 4, "First invalid wins"),
        ];

        for (sentence, invalid_char, expected_idx, name) in test_cases {
            assert_eq!(
                validate_sentence(sentence),
                Err(SentenceContextError::InvalidCharacter(
                    invalid_char,
                    expected_idx
                )),
                "{} failed",
                name
            );
        }
    }

    #[test]
    fn carriage_return_is_invalid_character() {
        assert_eq!(
            validate_sentence("\rשלום"),
            Err(SentenceContextError::InvalidCharacter('\r', 0))
        );
    }

    #[test]
    fn emoji_are_invalid_characters() {
        assert_eq!(
            validate_sentence("שלום😀"),
            Err(SentenceContextError::InvalidCharacter('😀', 8))
        );
    }

    // ============================================================
    // REAL-WORLD EXAMPLES
    // ============================================================

    #[test]
    fn longest_tanakh_verse_esther_8_9_passes_validation() {
        // ~367 characters, ~43 Hebrew words
        let esther_8_9 = concat!(
            " וַיִּקָּרְאוּ סֹפְרֵי־הַמֶּלֶךְ בָּעֵת־הַהִיא בַּחֹדֶשׁ הַשְּׁלִישִׁי ",
            "הוּא־חֹדֶשׁ סִיוָן בִּשְׁלוֹשָׁה וְעֶשְׂרִים בּוֹ ",
            "וַיִּכָּתֵב כְּכָל־אֲשֶׁר־צִוָּה מָרְדֳּכַי אֶל־הַיְּהוּדִים ",
            "וְאֶל הָאֲחַשְׁדַּרְפְּנִים־וְהַפַּחוֹת ",
            "וְשָׂרֵי הַמְּדִינוֹת אֲשֶׁר מֵהֹדּוּ וְעַד־כּוּשׁ ",
            "שֶׁבַע וְעֶשְׂרִים וּמֵאָה מְדִינָה מְדִינָה וּמְדִינָה ",
            "כִּכְתָבָהּ וְעַם וָעָם כִּלְשֹׁנוֹ ",
            "וְאֶל־הַיְּהוּדִים כִּכְתָבָם וְכִלְשׁוֹנָם׃"
        );

        assert_eq!(validate_sentence(esther_8_9), Ok(()));
    }

    #[test]
    fn final_form_letters_within_sentences_are_valid() {
        assert_eq!(validate_sentence("מלך"), Ok(())); // ך mid-word
        assert_eq!(validate_sentence("דךדםדןדףדץ"), Ok(())); // All finals
    }
}
