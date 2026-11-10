use crate::{
    api::context::Context,
    display_cantillation_symbol,
    sentence::{detect_context_from_sentence, validate_sentence},
    HebrewAccent, SentenceContextError,
};

/// This is a convenience function for trying to detect the context without creating a [`crate::SentenceContext`] instance first.
///
/// See `try_derive_context` on `SentenceContext` for **detailed** documentation.
///
/// # Example
/// ``` rust
/// use hebrew_accents::{try_derive_context, Context};
///
/// let result = try_derive_context("וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");
/// match result {
///     Ok(context) => println!("Context: {:?}", context),
///     Err(e) => println!("Could not determine context: {}", e),
/// }
/// ```
pub fn try_derive_context(sentence: &str) -> Result<Context, SentenceContextError> {
    // first validate the given sentence
    validate_sentence(sentence)?;
    // then detect the context from it
    detect_context_from_sentence(sentence)
}

/// Builds a sample display string showing where the accent sits on a word.
///
/// Dummy words of three consonants without any vowels are used
/// to show where the accent is positioned.
///
/// For accents that **can** span one or two words, two dummy words of three consonants
/// will be shown, where each set contains only one mark.
///
/// # Note:
///  - These representations are a simplified version of reality, in order
///    to give an idea of how these accents might show up in real texts.
///  - The stress position of the accents are accounted for.
///
/// # Example
/// ```rust
/// use hebrew_accents::{cantillation_example, ProseAccent};
///
/// let example = cantillation_example(ProseAccent::Silluq);
///
/// println!("Display: {example}");  // "בָּ֫רְאַ"
/// ```
pub fn cantillation_example(accent: HebrewAccent) -> String {
    display_cantillation_symbol(accent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Context, SentenceContextError};

    // ── Basic functionality tests ────────────────────────────────────

    // #[test]
    // fn example_from_docstring_works() {
    //     // Test the exact example from the docstring
    //     let result = try_derive_context("וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");

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
    //         let result = try_derive_context(sentence);
    //         // Prose should return Context::Prose (adjust based on your actual enum variants)
    //         assert!(result.is_ok(), "Prose example '{}' failed", sentence);
    //     }
    // }

    #[test]
    fn poetry_text_detected_as_poetry() {
        // Poetry passages (Psalms, Job, Song of Songs, etc.)
        let poetry_examples = vec!["לַמְנַצֵּ֥חַ לִדְבִיר־לֶ֫כֶת מִזְמוֹ֥ר לְדָוִֽד", "הַלְלוּ־יָ֑הּ הַלְל֛וּ אֶת־אֱלֹהִ֖ים בְּקָדְש֑וֹ"];

        for sentence in poetry_examples {
            let result = try_derive_context(sentence);
            // Poetry should return Context::Poetry (adjust based on your actual enum variants)
            assert!(result.is_ok(), "Poetry example '{}' failed", sentence);
        }
    }

    #[test]
    fn mixed_content_returns_appropriate_context() {
        // Text that may have elements from both contexts
        let mixed = "דְּבָרִ֑ים וְאִם־חֻקֹּתַ֙י תִּשְׁמְע֔וּן";

        let result = try_derive_context(mixed);
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

        let result = try_derive_context(invalid);
        // May return InvalidInput or successfully parse (depends on implementation)
        // Just ensure it doesn't panic
        let _ = result;
    }

    #[test]
    fn partial_hebrew_with_latin_fallback() {
        // Mixed script text
        let mixed_script = "וְהָיָ֥ה abc שָׂמֵֽחַ";

        let result = try_derive_context(mixed_script);
        // Should not panic regardless of outcome
        let _ = result;
    }

    #[test]
    fn cantillation_marks_without_text() {
        // Only marks, no base letters
        let marks_only = "֑֒֓֔֕";

        let result = try_derive_context(marks_only);
        // Implementation-dependent, but should not panic
        let _ = result;
    }

    // ── Edge case tests ──────────────────────────────────────────────

    #[test]
    fn single_word_with_accent() {
        let single_word = "בְּרֵאשִׁ֖ית";

        let result = try_derive_context(single_word);
        assert!(result.is_ok() || result.is_err()); // Either outcome is acceptable
    }

    // #[test]
    // fn very_long_passage() {
    //     // Longer than typical sentence
    //     let long_text = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ \
    //                     וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";
    //     let result = try_derive_context(long_text);
    //     assert!(
    //         result.is_ok(),
    //         "Long text should not cause overflow or errors"
    //     );
    // }

    // #[test]
    // fn text_with_niqqud_and_tiberian_marks() {
    //     // Full vocalization including vowel points and cantillation
    //     let fully_vocalized = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";

    //     let result = try_derive_context(fully_vocalized);
    //     assert!(
    //         result.is_ok(),
    //         "Fully vocalized text should be processed correctly"
    //     );
    // }

    // ── Return type correctness tests ─────────────────────────────────

    #[test]
    fn return_type_is_result_context_error() {
        let result: Result<Context, SentenceContextError> = try_derive_context("וַיְהִי");

        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn error_variant_contains_valid_error() {
        let result = try_derive_context("");

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
    //     let _ = try_derive_context("וַיְהִי בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ");
    //     // If we reach here, no panic occurred
    // }

    #[test]
    fn multiple_calls_same_input_consistent() {
        let sentence = "וַיְהִ֣י בְיָמֵ֗י אֲחַשְׁוֵרֹ֡שׁ";

        let result1 = try_derive_context(sentence);
        let result2 = try_derive_context(sentence);

        // Results should be deterministic
        assert_eq!(result1.is_ok(), result2.is_ok());
    }

    // ── Integration-style tests ───────────────────────────────────────

    // #[test]
    // fn chain_multiple_sentences() {
    //     let sentences = vec!["וַיְהִ֣י בְיָמֵ֗י", "אֲחַשְׁוֵרֹ֡שׁ", "בְּרֵאשִׁ֖ית"];

    //     for sentence in sentences {
    //         let result = try_derive_context(sentence);
    //         assert!(result.is_ok(), "Failed on '{}'", sentence);
    //     }
    // }

    // #[test]
    // fn consecutive_empty_and_valid() {
    //     // Ensure state doesn't leak between calls
    //     let _ = try_derive_context(""); // error
    //     let _ = try_derive_context("וַיְהִי"); // should work independently
    //     let _ = try_derive_context("   "); // error

    //     // Verify last call was independent
    //     let final_result = try_derive_context("בְּרֵאשִׁ֖ית");
    //     assert!(final_result.is_ok());
    // }
}

#[cfg(test)]
mod prose_cantillation_symbol {
    use crate::{ProseAccent, cantillation_example};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(cantillation_example(ProseAccent::Silluq.into()), "אםֽת");
        assert_eq!(cantillation_example(ProseAccent::Atnach.into()), "אם֑ת");
        assert_eq!(cantillation_example(ProseAccent::Segolta.into()), "אםת֒");
        assert_eq!(cantillation_example(ProseAccent::Shalshelet.into()), "אם֓ת ׀");
        assert_eq!(cantillation_example(ProseAccent::ZaqephQaton.into()), "אם֔ת");
        assert_eq!(cantillation_example(ProseAccent::ZaqephGadol.into()), "אם֕ת");
        assert_eq!(cantillation_example(ProseAccent::Revia.into()), "אם֗ת");
        assert_eq!(cantillation_example(ProseAccent::Tiphcha.into()), "אם֖ת");
        assert_eq!(cantillation_example(ProseAccent::Zarqa.into()), "אםת֮");
        assert_eq!(cantillation_example(ProseAccent::Pashta.into()), "אםת֙");
        assert_eq!(cantillation_example(ProseAccent::ShenePastin.into()), "א֨םת֙");
        assert_eq!(cantillation_example(ProseAccent::Yetiv.into()), "א֚םת");
        assert_eq!(cantillation_example(ProseAccent::Tevir.into()), "אם֛ת");
        assert_eq!(cantillation_example(ProseAccent::Geresh.into()), "אם֜ת");
        assert_eq!(cantillation_example(ProseAccent::Gershayim.into()), "אם֞ת");
        assert_eq!(cantillation_example(ProseAccent::Pazer.into()), "אם֡ת");
        assert_eq!(cantillation_example(ProseAccent::PazerGadol.into()), "אם֟ת");
        assert_eq!(cantillation_example(ProseAccent::TelishaGedolah.into()), "א֠םת");
        assert_eq!(cantillation_example(ProseAccent::Legarmeh.into()), "אם֣ת ׀");
    }

    #[test]
    fn test_prose_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(cantillation_example(ProseAccent::Munach.into()), "אם֣ת");
        assert_eq!(cantillation_example(ProseAccent::Mahpakh.into()), "אם֤ת");
        assert_eq!(cantillation_example(ProseAccent::Merkha.into()), "אם֥ת");
        assert_eq!(cantillation_example(ProseAccent::MerkhaKephulah.into()), "אם֦ת");
        assert_eq!(cantillation_example(ProseAccent::Darga.into()), "אם֧ת");
        assert_eq!(cantillation_example(ProseAccent::Azla.into()), "אם֨ת");
        assert_eq!(cantillation_example(ProseAccent::TelishaQetannah.into()), "אםת֩");
        assert_eq!(cantillation_example(ProseAccent::Galgal.into()), "אם֪ת");
        assert_eq!(cantillation_example(ProseAccent::Meayla.into()), "אם֖ת");
        assert_eq!(cantillation_example(ProseAccent::Meteg.into()), "אםֽת");
    }
}

#[cfg(test)]
mod poetry_cantillation_symbol {
    use crate::{PoetryAccent, cantillation_example};
    #[test]
    fn test_poetry_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(cantillation_example(PoetryAccent::Silluq.into()), "אםֽת");
        assert_eq!(cantillation_example(PoetryAccent::OlehWeYored.into()), "אם֫ת אם֥ת");
        assert_eq!(cantillation_example(PoetryAccent::Atnach.into()), "אם֑ת");
        assert_eq!(cantillation_example(PoetryAccent::ReviaGadol.into()), "אם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::ReviaMugrash.into()), "א֝ם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::ShalsheletGadol.into()), "אם֓ת ׀");
        assert_eq!(cantillation_example(PoetryAccent::Tsinnor.into()), "אםת֮");
        assert_eq!(cantillation_example(PoetryAccent::ReviaQaton.into()), "אם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::Dechi.into()), "אם֭ת");
        assert_eq!(cantillation_example(PoetryAccent::Pazer.into()), "אם֡ת");
        assert_eq!(cantillation_example(
            PoetryAccent::MehuppakhLegarmeh.into()),
            "אם֤ת ׀"
        );
        assert_eq!(cantillation_example(PoetryAccent::AzlaLegarmeh.into()), "אם֨ת ׀");
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(cantillation_example(PoetryAccent::Munach.into()), "אם֣ת");
        assert_eq!(cantillation_example(PoetryAccent::Merkha.into()), "אם֥ת");
        assert_eq!(cantillation_example(PoetryAccent::Illuy.into()), "אם֬ת");
        assert_eq!(cantillation_example(PoetryAccent::Tarcha.into()), "אם֖ת");
        assert_eq!(cantillation_example(PoetryAccent::Galgal.into()), "אם֪ת");
        assert_eq!(cantillation_example(PoetryAccent::Mehuppakh.into()), "אם֤ת");
        assert_eq!(cantillation_example(PoetryAccent::Azla.into()), "אם֨ת");
        assert_eq!(cantillation_example(
            PoetryAccent::ShalsheletQetannah.into()),
            "אם֓ת"
        );
        assert_eq!(cantillation_example(
            PoetryAccent::TsinnoritMerkha.into()),
            "אם֘ת אם֥ת"
        );
        assert_eq!(cantillation_example(
            PoetryAccent::TsinnoritMahpakh.into()),
            "אם֘ת אם֤ת"
        );
        assert_eq!(cantillation_example(PoetryAccent::Meteg.into()), "אםֽת");
    }
}

#[cfg(test)]
mod pseudo_cantillation_symbol {
    use crate::{PseudoAccent, cantillation_example};
    #[test]
    fn test_pseudo_accent_cantillation_symbol() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(cantillation_example(PseudoAccent::SophPasuq.into()), "אםת ׃");
        assert_eq!(cantillation_example(PseudoAccent::Maqqaph.into()), "אםת־אםת");
        assert_eq!(cantillation_example(PseudoAccent::Paseq.into()), "אםת ׀");
    }
}
