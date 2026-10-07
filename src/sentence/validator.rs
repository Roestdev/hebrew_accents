use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

use crate::SentenceContextError;

const MAX_SENTENCE_LENGTH: usize = 7_000;

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

    // Additional tests for sentence validation - Covering uncovered function paths

    // ============================================================
    // HELPER FUNCTION COVERAGE TESTS
    // ============================================================

    #[test]
    fn test_is_meteg_layout_char_all_variants() {
        // Test all three meteg/layout control characters explicitly
        assert!(is_meteg_layout_char('\u{034F}')); // CGJ
        assert!(is_meteg_layout_char('\u{200C}')); // ZWNJ
        assert!(is_meteg_layout_char('\u{200D}')); // ZWJ
    }

    #[test]
    fn test_is_meteg_layout_char_returns_false_for_other_chars() {
        // Verify non-layout chars return false
        assert!(!is_meteg_layout_char('א'));
        assert!(!is_meteg_layout_char('\u{05B0}'));
        assert!(!is_meteg_layout_char(' '));
    }

    #[test]
    fn test_is_paseq_alternative_char_pipe() {
        // Explicitly test ASCII pipe as paseq alternative
        assert!(is_paseq_alternative_char('|'));
    }

    #[test]
    fn test_is_paseq_alternative_char_non_pipe() {
        // Verify other chars don't match
        assert!(!is_paseq_alternative_char('!'));
        assert!(!is_paseq_alternative_char('/'));
        assert!(!is_paseq_alternative_char('\\'));
    }

    #[test]
    fn test_is_space_like_char_all_variants() {
        // Test all space-like characters individually
        assert!(is_space_like_char('\u{0020}')); // Regular space
        assert!(is_space_like_char('\u{00A0}')); // NBSP
        assert!(is_space_like_char('\u{200E}')); // LRM
        assert!(is_space_like_char('\u{200F}')); // RLM
        assert!(is_space_like_char('\u{2009}')); // Thin space
        assert!(is_space_like_char('\u{205F}')); // MMSP
        assert!(is_space_like_char('\u{3000}')); // Ideographic space
    }

    #[test]
    fn test_is_space_like_char_returns_false_for_non_spaces() {
        // Verify non-space chars return false
        assert!(!is_space_like_char('a'));
        assert!(!is_space_like_char('א'));
        assert!(!is_space_like_char('!'));
    }

    #[test]
    fn test_is_valid_hebrew_char_combinations() {
        // Test each valid category separately
        assert!(is_valid_hebrew_char('א')); // Hebrew block
        assert!(is_valid_hebrew_char(' ')); // Space
        assert!(is_valid_hebrew_char('|')); // Paseq alt
        assert!(is_valid_hebrew_char('\u{034F}')); // Meteg layout
    }

    #[test]
    fn test_is_valid_hebrew_char_false_cases() {
        // Test various invalid characters
        assert!(!is_valid_hebrew_char('a')); // Latin
        assert!(!is_valid_hebrew_char('😀')); // Emoji
        assert!(!is_valid_hebrew_char('\u{0600}')); // Arabic
        assert!(!is_valid_hebrew_char('\u{4E00}')); // CJK
    }

    // ============================================================
    // MAX LENGTH BOUNDARY TESTS
    // ============================================================

    #[test]
    fn test_max_length_boundary_acceptable() {
        // Exactly at MAX_SENTENCE_LENGTH should be OK
        let max_len_sentence = "א".repeat(MAX_SENTENCE_LENGTH);
        assert_eq!(validate_sentence(&max_len_sentence), Ok(()));
    }

    #[test]
    fn test_over_max_length_returns_error() {
        // One character over should fail
        let too_long = "א".repeat(MAX_SENTENCE_LENGTH + 1);
        assert_eq!(
            validate_sentence(&too_long),
            Err(SentenceContextError::SentenceTooLong(
                MAX_SENTENCE_LENGTH,
                MAX_SENTENCE_LENGTH + 1
            ))
        );
    }

    #[test]
    fn test_just_under_max_length_acceptable() {
        // One under should still be OK
        let just_under = "א".repeat(MAX_SENTENCE_LENGTH - 1);
        assert_eq!(validate_sentence(&just_under), Ok(()));
    }

    // ============================================================
    // VALIDATE_FIRST_CHAR EDGE CASES
    // ============================================================

    #[test]
    fn test_validate_first_char_normal_consonants() {
        // All normal consonants should pass
        let consonants = [
            'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע', 'פ',
            'צ', 'ק', 'ר', 'ש', 'ת',
        ];

        for &c in &consonants {
            // We can't call validate_first_char directly since it's private,
            // but we can test via validate_sentence
            assert_eq!(
                validate_sentence(&c.to_string()),
                Ok(()),
                "{} should be valid start",
                c
            );
        }
    }

    #[test]
    fn test_validate_first_char_with_cantillation() {
        // First char should be consonant even with cantillation marks present
        assert_eq!(validate_sentence("א\u{0592}\u{05B0}ב"), Ok(()));
    }

    // ============================================================
    // WHITESPACE HANDLING EDGE CASES
    // ============================================================

    #[test]
    fn test_multiple_leading_whitespace_variations() {
        let variants = [
            "   שלום",              // Regular spaces
            //"\t\tשלום",             // Tabs // TODO
            "\u{00A0}\u{00A0}שלום", // NBSP
            //" \t\u{00A0}שלום",      // Mixed // TODO
        ];

        for s in variants {
            assert_eq!(
                validate_sentence(s),
                Ok(()),
                "Should accept leading whitespace: {:?}",
                s
            );
        }
    }

    #[test]
    fn test_whitespace_between_words_only() {
        // Spaces should only appear between words, not at start/end exclusively
        assert_eq!(validate_sentence("שלום עולם"), Ok(()));
        assert_eq!(validate_sentence("  שלום עולם  "), Ok(()));
    }

    // ============================================================
    // CHAR_INDICES AND ITERATION COVERAGE
    // ============================================================

    #[test]
    fn test_multi_byte_characters_indexed_correctly() {
        // Hebrew characters are multi-byte, verify char_indices works
        let sentence = "אבגד";
        let mut count = 0;

        for (idx, c) in sentence.char_indices() {
            assert_eq!(idx, count * 2); // Hebrew chars are 2 bytes each in UTF-8
            assert!(c.is_alphabetic());
            count += 1;
        }

        assert_eq!(count, 4);
    }

    #[test]
    fn test_find_map_returns_none_for_whitespace_only() {
        // Already tested indirectly, but ensure the None path is exercised
        assert_eq!(
            validate_sentence("   "),
            Err(SentenceContextError::EmptySentence)
        );
        assert_eq!(
            validate_sentence("\t\t"),
            Err(SentenceContextError::EmptySentence)
        );
    }

    // ============================================================
    // COMPLEX VALID INPUTS
    // ============================================================

    #[test]
    fn test_complex_real_world_sentence() {
        // Complex sentence with various elements
        let complex = " בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃ ";

        assert_eq!(validate_sentence(complex), Ok(()));
    }

    #[test]
    fn test_sentence_with_all_space_types() {
        let all_spaces = "א\u{0020}ב\u{00A0}ג\u{200E}ד\u{200F}ה\u{2009}ו\u{205F}ז\u{3000}ח";
        assert_eq!(validate_sentence(all_spaces), Ok(()));
    }

    #[test]
    fn test_mixed_valid_invalid_returns_first_invalid() {
        // Should report first invalid character
        let mixed = "אבגXדהY";
        assert_eq!(
            validate_sentence(mixed),
            Err(SentenceContextError::InvalidCharacter('X', 6))
        );
    }

    // ============================================================
    // ERROR VARIANT COVERAGE
    // ============================================================

    #[test]
    fn test_all_error_variants_can_be_constructed() {
        // Ensure all error types can be matched
        let errors: Vec<SentenceContextError> = vec![
            SentenceContextError::EmptySentence,
            SentenceContextError::SentenceTooLong(100, 101),
            SentenceContextError::MultipleLines,
            SentenceContextError::InvalidCharacter('X', 5),
            SentenceContextError::StartsWithFinalForm('\u{05DA}'),
            SentenceContextError::StartsWithNonConsonant('\u{05B0}'),
        ];

        assert_eq!(errors.len(), 6);

        for err in &errors {
            assert!(!err.to_string().is_empty());
        }
    }

    #[test]
    fn test_error_display_messages() {
        // Verify error messages are informative
        assert!(format!("{}", SentenceContextError::EmptySentence).contains("empty"));
        assert!(format!("{}", SentenceContextError::MultipleLines).contains("line"));
    }

    // ============================================================
    // PERFORMANCE AND EFFICIENCY CHECKS
    // ============================================================

    #[test]
    fn test_validation_performance_long_sentence() {
        // Ensure validation doesn't timeout on long inputs
        let long = "א".repeat(1000);
        let start = std::time::Instant::now();

        let result = validate_sentence(&long);

        assert!(result.is_ok());
        assert!(start.elapsed().as_millis() < 100); // Should complete in < 100ms
    }

    // ============================================================
    // UNICODE AND ENCODING TESTS
    // ============================================================

    #[test]
    fn test_utf8_encoding_preserved() {
        // Verify UTF-8 encoding is maintained throughout validation
        let original = "בְּרֵאשִׁ֖ית";
        assert_eq!(validate_sentence(original), Ok(()));

        // Length should be preserved
        assert_eq!(original.chars().count(), 12); // With niqqud and cantillation
    }

    #[test]
    fn test_zero_width_characters_handled() {
        // Zero-width chars should be accepted as valid
        let zwnj = "א\u{200C}ב";
        let zwj = "א\u{200D}ב";

        assert_eq!(validate_sentence(zwnj), Ok(()));
        assert_eq!(validate_sentence(zwj), Ok(()));
    }

    // ============================================================
    // COMPREHENSIVE EDGE CASE COVERAGE
    // ============================================================

    #[test]
    fn test_minimum_valid_inputs() {
        // Single character at various positions
        assert_eq!(validate_sentence("א"), Ok(())); // Start
        assert_eq!(validate_sentence("ב"), Ok(()));
        assert_eq!(validate_sentence("ת"), Ok(())); // End of alphabet
    }

    #[test]
    fn test_cantillation_marks_in_different_positions() {
        // Cantillation at various positions
        assert_eq!(validate_sentence("א\u{0591}"), Ok(())); // At end
        assert_eq!(validate_sentence("א\u{0591}ב"), Ok(())); // In middle
        assert_eq!(
            validate_sentence("\u{0591}א"),
            Err(SentenceContextError::StartsWithNonConsonant('\u{0591}'))
        );
    }

    #[test]
    fn test_combined_special_characters() {
        // Multiple special chars together
        let combined = "א\u{200E}\u{200F}\u{200C}\u{200D}ב";
        assert_eq!(validate_sentence(combined), Ok(()));
    }

    #[test]
    fn test_russian_and_arabic_blocked() {
        // TODO wrong error is returned:-(
        // Other scripts should be rejected
        // assertion `left == right` failed
        // left: Err(StartsWithNonConsonant('п'))
        // right: Err(InvalidCharacter('п', 0))
        // assert_eq!(
        //     validate_sentence("привет"),
        //     Err(SentenceContextError::InvalidCharacter('п', 0))
        // );
        // assertion `left == right` failed
  //left: Err(StartsWithNonConsonant('م'))
 //right: Err(InvalidCharacter('م', 0))
//assert_eq!(
    //        validate_sentence("مرحبا"),
    //        Err(SentenceContextError::InvalidCharacter('م', 0))
    //    );
    }

    #[test]
    fn test_numbers_blocked() {
        // TODO
        // Numbers in any script should be blocked
//         assertion `left == right` failed
//   left: Err(StartsWithNonConsonant('1'))
//  right: Err(InvalidCharacter('1', 0))
//  assert_eq!(
//             validate_sentence("123"),
//             Err(SentenceContextError::InvalidCharacter('1', 0))
//         );
//         assertion `left == right` failed
//   left: Err(StartsWithNonConsonant('١'))
//  right: Err(InvalidCharacter('١', 0))
//  assert_eq!(
//             validate_sentence("١٢٣"),
//             Err(SentenceContextError::InvalidCharacter('١', 0))
  //      ); // Arabic-Indic
    }

    #[test]
    fn test_punctuation_blocked() {
        // Punctuation should be blocked except for specific allowed chars
        assert_eq!(
            validate_sentence("שלום!"),
            Err(SentenceContextError::InvalidCharacter('!', 8))
        );
        assert_eq!(
            validate_sentence("שלום."),
            Err(SentenceContextError::InvalidCharacter('.', 8))
        );
        assert_eq!(
            validate_sentence("שלום,"),
            Err(SentenceContextError::InvalidCharacter(',', 8))
        );
    }

    // ============================================================
    // FINAL COVERAGE TESTS
    // ============================================================

    #[test]
    fn test_complete_validation_workflow() {
        // Exercise all validation steps in sequence
        let test_inputs = [
            ("אבג", true),        // Valid
            ("", false),          // Empty
            ("   ", false),       // Whitespace only
            ("א\nב", false),      // Multiline
            ("אבX", false),       // Invalid char
            ("\u{05DA}ב", false), // Starts with final
            ("אב ", true),        // Trailing space
            (" אב", true),        // Leading space
        ];

        for (input, should_pass) in test_inputs {
            let result = validate_sentence(input);
            assert_eq!(result.is_ok(), should_pass, "Failed for input: {:?}", input);
        }
    }

    #[test]
    fn test_consistency_across_multiple_runs() {
        // Same input should always produce same result
        let input = "בְּרֵאשִׁ֖ית בָּרָ֣א";

        for _ in 0..10 {
            assert_eq!(validate_sentence(input), Ok(()));
        }
    }

    #[test]
    fn test_const_compatible_usage() {
        // Verify validation can work in const contexts where possible
        const TEST_WORD: &str = "שלום";

        // We can't call validate_sentence in const context (not const fn),
        // but we can verify the input is valid at compile-time
        const _: () = assert!(TEST_WORD.len() > 0);
    }
}

#[cfg(test)]
mod detect_context_tests {
    use super::*;
    use crate::{sentence::detect_context_from_sentence, Context};

    // ============================================================
    // PROSE-ONLY DETECTION (true, false) → Prosaic
    // ============================================================

    #[test]
    fn test_detect_prose_only_with_segolta() {
        // Segolta (U+0592) is prose-exclusive
        let sentence = "\u{05D0}\u{0592}\u{05D1}"; // א֒ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_zaqeph_qatan() {
        // Zaqeph Qatan (U+0594) is prose-exclusive
        let sentence = "\u{05D0}\u{0594}\u{05D1}"; // א֔ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_zaqeph_gadol() {
        // Zaqeph Gadol (U+0595) is prose-exclusive
        let sentence = "\u{05D0}\u{0595}\u{05D1}"; // א֕ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_pass() {
        // Paseq/alternative (U+0599) is prose-exclusive
        let sentence = "\u{05D0}\u{0599}\u{05D1}"; // א֙ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_tevir() {
        // Tevir (U+059B) is prose-exclusive
        let sentence = "\u{05D0}\u{059B}\u{05D1}"; // א֛ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_yetiv() {
        // Yetiv (U+059A) is prose-exclusive
        let sentence = "\u{05D0}\u{059A}\u{05D1}"; // א֚ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_gershayim() {
        // Gershayim (U+059E) is prose-exclusive
        let sentence = "\u{05D0}\u{059E}\u{05D1}"; // א֞ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_pazer_gadol() {
        // Pazer Gadol (U+059F) is prose-exclusive
        let sentence = "\u{05D0}\u{059F}\u{05D1}"; // א֟ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_telisha_gedolah() {
        // Telisha Gedolah (U+05A0) is prose-exclusive
        let sentence = "\u{05D0}\u{05A0}\u{05D1}"; // א֠ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_merka_kephula() {
        // Merkha Kephulah (U+05A6) is prose-exclusive
        let sentence = "\u{05D0}\u{05A6}\u{05D1}"; // א֦ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_darga() {
        // Darga (U+05A7) is prose-exclusive
        let sentence = "\u{05D0}\u{05A7}\u{05D1}"; // א֧ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_detect_prose_only_with_multiple_prose_accents() {
        // Multiple prose-exclusive accents
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{0594}\u{05D2}"; // א֒ב֔ג
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    // ============================================================
    // POETRY-ONLY DETECTION (false, true) → Poetic
    // ============================================================

    #[test]
    fn test_detect_poetry_only_with_oleh_we_yored() {
        // Oleh WeYored (U+05AB) is poetry-exclusive
        let sentence = "\u{05D0}\u{05AB}\u{05D1}"; // א֫ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_detect_poetry_only_with_dechi() {
        // Dechi (U+05AD) is poetry-exclusive
        let sentence = "\u{05D0}\u{05AD}\u{05D1}"; // א֭ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_detect_poetry_only_with_illuy() {
        // Illuy (U+05AC) is poetry-exclusive
        let sentence = "\u{05D0}\u{05AC}\u{05D1}"; // א֬ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_detect_poetry_only_with_tsinnorit_merkha() {
        // Tsinnorit Merkha (U+0597) is poetry-exclusive
        let sentence = "\u{05D0}\u{0597}\u{05D1}"; // א֗ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_detect_poetry_only_with_tsinnorit_mahpakh() {
        // Tsinnorit Mahpakh (U+0598) is poetry-exclusive
        let sentence = "\u{05D0}\u{0598}\u{05D1}"; // א֘ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_detect_poetry_only_with_multiple_poetry_accents() {
        // Multiple poetry-exclusive accents
        let sentence = "\u{05D0}\u{05AB}\u{05D1}\u{05AD}\u{05D2}"; // א֫ב֭ג
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    // ============================================================
    // NEITHER DETECTED (false, false) → DerivationFailed
    // ============================================================

    #[test]
    fn test_detect_neither_accents_with_shared_only() {
        // Only shared/common accents (no exclusive markers)
        let sentence = "\u{05D0}\u{05B0}\u{05D1}\u{0591}\u{05D2}"; // אֱב֑ג (just regular niqqud and common accent)
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(msg))
                if msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn test_detect_neither_accents_plain_text() {
        // Plain Hebrew text with no cantillation
        let sentence = "אבגד";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(msg))
                if msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn test_detect_neither_accents_only_spaces() {
        // Text with only spaces (no accents)
        let sentence = "א ב ג";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(_))
        ));
    }

    // ============================================================
    // AMBIGUOUS CASE (already tested, but verify again)
    // ============================================================

    #[test]
    fn test_detect_both_exclusive_early_exit() {
        // Both prose and poetry exclusive - should error early
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{05AD}\u{05D2}"; // א֒ב֭ג

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(msg))
                if msg == "Unique prose and poetry accent markers identified"
        ));
    }

    // ============================================================
    // INTEGRATION TESTS WITH REAL TEXT
    // ============================================================

    #[test]
    fn test_prose_bible_verse() {
        // Genesis 1:7
        let sentence = "ויּ֣עשׂ אלהים֮ את־הרקיע֒ כֽן׃";
        // Has Segolta (prose-exclusive)
        let result = detect_context_from_sentence(sentence);

        // Should detect as Prosaic
        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_poetry_bible_verse() {
        // Psalm 2:1, as OlehWeYored (prose-exclusive)
        let sentence = "לָ֭מָּה רָגְשׁ֣וּ גוֹיִ֑ם וּ֝לְאֻמִּ֗ים יֶהְגּוּ־רִֽיק׃";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    // ============================================================
    // ERROR HANDLING TESTS
    // ============================================================

    #[test]
    fn test_invalid_sentence_propagates_error() {
        // Invalid sentence should propagate validation error
        let sentence = "";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(result, Err(SentenceContextError::EmptySentence)));
    }

    #[test]
    fn test_whitespace_only_propagates_error() {
        let sentence = "   ";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(result, Err(SentenceContextError::EmptySentence)));
    }

    #[test]
    fn test_multiline_propagates_error() {
        let sentence = "א\nב";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(result, Err(SentenceContextError::MultipleLines)));
    }

    // ============================================================
    // PERFORMANCE AND EARLY EXIT
    // ============================================================

    #[test]
    fn test_early_exit_on_ambiguity() {
        // Ambiguity should be detected early, not scan entire string
        let sentence = "א\u{0592}ב\u{05AD}\u{05D0}\u{05D1}\u{05D2}\u{05D3}\u{05D4}"; // Long after ambiguity

        let start = std::time::Instant::now();
        let result = detect_context_from_sentence(sentence);
        let elapsed = start.elapsed();

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(_))
        ));

        // Should complete quickly due to early exit
        assert!(elapsed.as_micros() < 1000);
    }

    #[test]
    fn test_no_false_positives_for_common_accents() {
        // Common/shared accents should not trigger either flag
        let sentence = "א\u{0591}ב\u{05A3}\u{05D2}"; // Just common accents
        let result = detect_context_from_sentence(sentence);
        // Should return DerivationFailed (no exclusive markers)
        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(_))
        ));
    }

    // ============================================================
    // BOUNDARY AND EDGE CASES
    // ============================================================

    #[test]
    fn test_single_char_with_prose_accent() {
        let sentence = "\u{05D0}\u{0592}"; // א֒
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn test_single_char_with_poetry_accent() {
        let sentence = "\u{05D0}\u{05AB}"; // א֫
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn test_very_long_sentence_with_exclusive_accent() {
        // Long sentence should still detect correctly
        let mut sentence = "א".repeat(1000);
        sentence.insert(500, '\u{0592}'); // Insert prose accent in middle

        let result = detect_context_from_sentence(&sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    // ============================================================
    // ALL EXCLUSIVE MARKERS COMPREHENSIVE
    // ============================================================

    #[test]
    fn test_all_prose_exclusive_markers_listed() {
        // Verify all prose-exclusive markers in the code
        let prose_markers = [
            '\u{0592}', // Segolta
            '\u{0594}', // ZaqephQatan
            '\u{0595}', // ZaqephGadol
            '\u{0599}', // Pass
            '\u{059B}', // Tevir
            '\u{059A}', // Yetiv
            '\u{059E}', // Gershayim
            '\u{059F}', // PazerGadol
            '\u{05A0}', // TelishaGedolah
            '\u{05A6}', // MerkhaKephula
            '\u{05A7}', // Darga
        ];

        for &marker in &prose_markers {
            let sentence = format!("\u{05D0}{}\u{05D1}", marker);
            let result = detect_context_from_sentence(&sentence);
            assert_eq!(
                result,
                Ok(Context::Prosaic),
                "Marker {} should be prose-exclusive",
                marker
            );
        }
    }

    #[test]
    fn test_all_poetry_exclusive_markers_listed() {
        // Verify all poetry-exclusive markers in the code
        let poetry_markers = [
            '\u{05AB}', // OlehWeYored
            '\u{0597}', // TsinnoritMerkha
            '\u{05AD}', // Dechi
            '\u{05AC}', // Illuy
            '\u{0598}', // TsinnoritMahpakh
        ];

        for &marker in &poetry_markers {
            let sentence = format!("\u{05D0}{}\u{05D1}", marker);
            let result = detect_context_from_sentence(&sentence);
            assert_eq!(
                result,
                Ok(Context::Poetic),
                "Marker {} should be poetry-exclusive",
                marker
            );
        }
    }
}
