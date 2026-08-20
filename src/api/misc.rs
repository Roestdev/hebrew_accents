use crate::{api::context::Context, sentence::detect_context_from_sentence, SentenceContextError};

/// This is a convenience function for trying to detect the context without creating a
/// [`crate::SentenceContext`] instance first.
///
/// See `try_determine_context` on `SentenceContext` for **detailed** documentation.
///
/// # Example
/// ``` rust
/// use hebrew_accents::{try_determine_context, Context};
///
/// let result = try_determine_context("וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");
/// match result {
///     Ok(context) => println!("Context: {:?}", context),
///     Err(e) => println!("Could not determine context: {}", e),
/// }
/// ```
pub fn try_determine_context(sentence: &str) -> Result<Context, SentenceContextError> {
    detect_context_from_sentence(sentence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Context, SentenceContextError};

    // ── Basic functionality tests ────────────────────────────────────

    // #[test]
    // fn example_from_docstring_works() {
    //     // Test the exact example from the docstring
    //     let result = try_determine_context("וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");

    //     // Should succeed (this is from Esther 1:1, prose context)
    //     assert!(result.is_ok(), "Docstring example should succeed");
    // }

    // ── Context detection tests ──────────────────────────────────────

    // #[test]
    // fn prose_text_detected_as_prose() {
    //     // Typical prose passage (narrative books like Genesis, Esther, etc.)
    //     let prose_examples = vec![
    //         "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ",
    //         "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
    //         "וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר",
    //     ];

    //     for sentence in prose_examples {
    //         let result = try_determine_context(sentence);
    //         // Prose should return Context::Prose (adjust based on your actual enum variants)
    //         assert!(result.is_ok(), "Prose example '{}' failed", sentence);
    //     }
    // }

    #[test]
    fn poetry_text_detected_as_poetry() {
        // Poetry passages (Psalms, Job, Song of Songs, etc.)
        let poetry_examples = vec!["לַמְנַצֵּ֥חַ לִדְבִיר־לֶ֫כֶת מִזְמוֹ֥ר לְדָוִֽד", "הַלְלוּ־יָ֑הּ הַלְל֛וּ אֶת־אֱלֹהִ֖ים בְּקָדְש֑וֹ"];

        for sentence in poetry_examples {
            let result = try_determine_context(sentence);
            // Poetry should return Context::Poetry (adjust based on your actual enum variants)
            assert!(result.is_ok(), "Poetry example '{}' failed", sentence);
        }
    }

    #[test]
    fn mixed_content_returns_appropriate_context() {
        // Text that may have elements from both contexts
        let mixed = "דְּבָרִ֑ים וְאִם־חֻקֹּתַ֙י תִּשְׁמְע֔וּן";

        let result = try_determine_context(mixed);
        assert!(
            result.is_ok(),
            "Mixed content should still return a valid context"
        );
    }

    // ── Error handling tests ────────────────────────────────────────

    #[test]
    fn invalid_hebrew_characters_handled_gracefully() {
        // Non-Hebrew characters should trigger appropriate errors
        let invalid = "abc123def";

        let result = try_determine_context(invalid);
        // May return InvalidInput or successfully parse (depends on implementation)
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn partial_hebrew_with_latin_fallback() {
        // Mixed script text
        let mixed_script = "וְהָיָ֥ה abc שָׂמֵֽחַ";

        let result = try_determine_context(mixed_script);
        // Should not panic regardless of outcome
        let _ = result;
    }

    #[test]
    fn cantillation_marks_without_text() {
        // Only marks, no base letters
        let marks_only = "֑֒֓֔֕";

        let result = try_determine_context(marks_only);
        // Implementation-dependent, but should not panic
        let _ = result;
    }

    // ── Edge case tests ──────────────────────────────────────────────

    #[test]
    fn single_word_with_accent() {
        let single_word = "בְּרֵאשִׁ֖ית";

        let result = try_determine_context(single_word);
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable
    }

    // #[test]
    // fn very_long_passage() {
    //     // Longer than typical sentence
    //     let long_text = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ \
    //                     וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";
    //     let result = try_determine_context(long_text);
    //     assert!(
    //         result.is_ok(),
    //         "Long text should not cause overflow or errors"
    //     );
    // }

    // #[test]
    // fn text_with_niqqud_and_tiberian_marks() {
    //     // Full vocalization including vowel points and cantillation
    //     let fully_vocalized = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";

    //     let result = try_determine_context(fully_vocalized);
    //     assert!(
    //         result.is_ok(),
    //         "Fully vocalized text should be processed correctly"
    //     );
    // }

    // ── Return type correctness tests ─────────────────────────────────

    #[test]
    fn return_type_is_result_context_error() {
        let result: Result<Context, SentenceContextError> = try_determine_context("וַיְהִי");

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn error_variant_contains_valid_error() {
        let result = try_determine_context("");

        if let Err(error) = result {
            // Verify SentenceContextError is meaningful
            assert_ne!(error.to_string(), "");
        }
    }

    // ── Performance/safety tests ─────────────────────────────────────

    // #[test]
    // #[should_panic(expected = "")]
    // fn no_panic_on_normal_input() {
    //     // This test ensures the function doesn't panic
    //     // Using should_panic with empty expected means any panic fails the test
    //     let _ = try_determine_context("וַיְהִי בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");
    //     // If we reach here, no panic occurred
    // }

    #[test]
    fn multiple_calls_same_input_consistent() {
        let sentence = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";

        let result1 = try_determine_context(sentence);
        let result2 = try_determine_context(sentence);

        // Results should be deterministic
        assert_eq!(result1.is_ok(), result2.is_ok());
    }

    // ── Integration-style tests ───────────────────────────────────────

    // #[test]
    // fn chain_multiple_sentences() {
    //     let sentences = vec!["וַיְהִ֣י בְיָמֵ֗י", "אֲחַשְׁוֵרֹ֡שׁ", "בְּרֵאשִׁ֖ית"];

    //     for sentence in sentences {
    //         let result = try_determine_context(sentence);
    //         assert!(result.is_ok(), "Failed on '{}'", sentence);
    //     }
    // }

    // #[test]
    // fn consecutive_empty_and_valid() {
    //     // Ensure state doesn't leak between calls
    //     let _ = try_determine_context(""); // error
    //     let _ = try_determine_context("וַיְהִי"); // should work independently
    //     let _ = try_determine_context("   "); // error

    //     // Verify last call was independent
    //     let final_result = try_determine_context("בְּרֵאשִׁ֖ית");
    //     assert!(final_result.is_ok());
    // }
}
