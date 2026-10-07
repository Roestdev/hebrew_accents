use crate::sentence::validator::validate_sentence;
use crate::Context;
use crate::SentenceContextError;

pub(crate) fn detect_context_from_sentence(
    sentence: &str,
) -> Result<Context, SentenceContextError> {
    validate_sentence(sentence)?;

    let mut could_be_prose = false;
    let mut could_be_poetry = false;

    // Single pass through characters
    for c in sentence.chars() {
        // Check if this char is a prose-exclusive accent
        if matches!(
            c,
            '\u{0592}'
                | '\u{0594}'
                | '\u{0595}'
                | '\u{0599}'
                | '\u{059B}'
                | '\u{059A}'
                | '\u{059E}'
                | '\u{059F}'
                | '\u{05A0}'
                | '\u{05A6}'
                | '\u{05A7}'
                | '\u{05A9}'
        ) {
            could_be_prose = true;
        }

        // Check if this char is a poetry-exclusive accent
        if matches!(
            c,
            '\u{05AB}' | '\u{0597}' | '\u{05AD}' | '\u{05AC}' | '\u{0598}'
        ) {
            could_be_poetry = true;
        }

        // ⭐ EARLY EXIT HERE ⭐
        // If both flags are true, we can STOP immediately
        if could_be_prose && could_be_poetry {
            return Err(SentenceContextError::DerivationFailed(
                "Unique prose and poetry accent markers identified",
            ));
        }
    }

    match (could_be_prose, could_be_poetry) {
        (true, false) => Ok(Context::Prosaic),
        (false, true) => Ok(Context::Poetic),
        (false, false) => Err(SentenceContextError::DerivationFailed(
            "No unique prose or poetry accent markers identified",
        )),
        // Unreachable due to early exit above
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use crate::SentenceContextError;

    /// The sentence contains both:
    ///   - Segolta (U+0592), a **prose-exclusive** cantillation mark, and
    ///   - Dehi   (U+05AD), a **poetry-exclusive** cantillation mark.
    ///
    /// `detect_context_from_sentence` should therefore detect that the input
    /// could belong to *both* systems simultaneously and return a
    /// `DerivationFailed` error with the message
    /// "Unique prose and poetry accent markers identified".
    #[test]
    fn detects_both_prose_and_poetry_accents_returns_derivation_failed() {
        // Construct a valid Hebrew sentence that includes both marks.
        // Characters:  א U+05D0   (valid start consonant)
        //              ֒ U+0592   (SEGOL / Segolta — prose-only)
        //              ב U+05D1   (consonant)
        //              ֭ U+05AD   (DEHI — poetry-only)
        //              ג U+05D2   (consonant)
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{05AD}\u{05D2}";

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }
}

#[cfg(test)]
mod comprehensive_coverage_tests {
    use super::*;
    use crate::SentenceContextError;

    // ============================================================
    // PROSE-ONLY DETECTION PATHS
    // ============================================================

    #[test]
    fn detects_prose_only_with_segolta() {
        // Segolta (U+0592) is prose-exclusive only
        let sentence = "\u{05D0}\u{0592}\u{05D1}"; // א֒ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_zaqeph_qatan() {
        // Zaqeph Qatan (U+0594) is prose-exclusive only
        let sentence = "\u{05D0}\u{0594}\u{05D1}"; // א֔ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_zaqeph_gadol() {
        // Zaqeph Gadol (U+0595) is prose-exclusive only
        let sentence = "\u{05D0}\u{0595}\u{05D1}"; // א֕ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_paseq() {
        // Paseq (U+0599) is prose-exclusive only
        let sentence = "\u{05D0}\u{0599}\u{05D1}"; // א֙ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_tevir() {
        // Tevir (U+059B) is prose-exclusive only
        let sentence = "\u{05D0}\u{059B}\u{05D1}"; // א֛ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_yetiv() {
        // Yetiv (U+059A) is prose-exclusive only
        let sentence = "\u{05D0}\u{059A}\u{05D1}"; // א֚ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_gershayim() {
        // Gershayim (U+059E) is prose-exclusive only
        let sentence = "\u{05D0}\u{059E}\u{05D1}"; // א֞ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_pazer_gadol() {
        // Pazer Gadol (U+059F) is prose-exclusive only
        let sentence = "\u{05D0}\u{059F}\u{05D1}"; // א֟ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_telisha_gedolah() {
        // Telisha Gedolah (U+05A0) is prose-exclusive only
        let sentence = "\u{05D0}\u{05A0}\u{05D1}"; // א֠ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_merka_kephula() {
        // Merkha Kephulah (U+05A6) is prose-exclusive only
        let sentence = "\u{05D0}\u{05A6}\u{05D1}"; // א֦ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_darga() {
        // Darga (U+05A7) is prose-exclusive only
        let sentence = "\u{05D0}\u{05A7}\u{05D1}"; // א֧ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_with_multiple_prose_markers() {
        // Multiple prose-exclusive markers
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{0594}\u{05D2}\u{059B}\u{05D3}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_prose_only_at_end_of_sentence() {
        // Prose marker appearing late in the sentence
        let sentence = "\u{05D0}\u{05D1}\u{05D2}\u{05D3}\u{05D4}\u{05D5}\u{05D6}\u{05D7}\u{05D8}\u{05D9}\u{05D0}\u{05D1}\u{05D2}\u{05D3}\u{0592}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    // ============================================================
    // POETRY-ONLY DETECTION PATHS
    // ============================================================

    #[test]
    fn detects_poetry_only_with_oleh_we_yored() {
        // Oleh WeYored (U+05AB) is poetry-exclusive only
        let sentence = "\u{05D0}\u{05AB}\u{05D1}"; // א֫ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_with_dechi() {
        // Dechi (U+05AD) is poetry-exclusive only
        let sentence = "\u{05D0}\u{05AD}\u{05D1}"; // א֭ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_with_illuy() {
        // Illuy (U+05AC) is poetry-exclusive only
        let sentence = "\u{05D0}\u{05AC}\u{05D1}"; // א֬ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_with_tsinnorit_merkha() {
        // Tsinnorit Merkha (U+0597) is poetry-exclusive only
        let sentence = "\u{05D0}\u{0597}\u{05D1}"; // א֗ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_with_tsinnorit_mahpakh() {
        // Tsinnorit Mahpakh (U+0598) is poetry-exclusive only
        let sentence = "\u{05D0}\u{0598}\u{05D1}"; // א֘ב
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_with_multiple_poetry_markers() {
        // Multiple poetry-exclusive markers
        let sentence = "\u{05D0}\u{05AB}\u{05D1}\u{05AD}\u{05D2}\u{0597}\u{05D3}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_poetry_only_at_end_of_sentence() {
        // Poetry marker appearing late in the sentence
        let sentence = "\u{05D0}\u{05D1}\u{05D2}\u{05D3}\u{05D4}\u{05D5}\u{05D6}\u{05D7}\u{05D8}\u{05D9}\u{05DA}\u{05DB}\u{05D5}\u{05D7}\u{05D8}\u{05D9}\u{05AB}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    // ============================================================
    // NEITHER DETECTED PATHS (DERIVATION_FAILED)
    // ============================================================

    #[test]
    fn detects_neither_no_exclusive_markers() {
        // Hebrew text with no cantillation marks
        let sentence = "אבגד";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_neither_only_shared_accents() {
        // Text with only common/shared accents (not exclusive to prose or poetry)
        let sentence = "\u{05D0}\u{05B0}\u{05D1}\u{0591}\u{05D2}"; // Just niqqud and common accent
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_neither_whitespace_only() {
        let sentence = "   ";
        let result = detect_context_from_sentence(sentence);

        // Whitespace-only should fail validation first
        assert!(matches!(result, Err(SentenceContextError::EmptySentence)));
    }

    // ============================================================
    // BOTH DETECTED (EARLY EXIT) - EXPAND TESTING
    // ============================================================

    #[test]
    fn detects_both_markers_at_beginning() {
        // Both markers appear early - should trigger early exit
        let sentence = "\u{05D0}\u{0592}\u{05AD}\u{05D2}\u{05D3}\u{05D4}";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_both_markers_poetry_first_then_prose() {
        // Poetry marker first, then prose marker
        let sentence = "\u{05D0}\u{05AD}\u{05D1}\u{0592}\u{05D2}";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_both_markers_prose_first_then_poetry() {
        // Prose marker first, then poetry marker
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{05AD}\u{05D2}";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_both_markers_scattered_throughout() {
        // Markers scattered throughout long text
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{05D2}\u{05D3}\u{05D4}\u{05D5}\u{05D6}\u{05D7}\u{05D8}\u{05D9}\u{05AD}\u{05DA}";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    // ============================================================
    // VALIDATION ERROR PROPAGATION
    // ============================================================

    #[test]
    fn propagates_empty_sentence_error() {
        let sentence = "";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(result, Err(SentenceContextError::EmptySentence)));
    }

    #[test]
    fn propagates_multiline_error() {
        let sentence = "אב\nגד";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(result, Err(SentenceContextError::MultipleLines)));
    }

    #[test]
    fn propagates_invalid_character_error() {
        // Latin character should fail validation
        let sentence = "אבcד";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::InvalidCharacter(_, _))
        ));
    }

    #[test]
    fn propagates_final_form_start_error() {
        // Starting with a final form character may trigger validation error
        let sentence = "\u{05DF}abc"; // Final Nun followed by invalid chars
        let result = detect_context_from_sentence(sentence);

        // May be InvalidCharacter or StartsWithFinalForm depending on implementation
        assert!(matches!(
            result,
            Err(SentenceContextError::InvalidCharacter(..)
                | SentenceContextError::StartsWithFinalForm(_))
        ));
    }

    // ============================================================
    // EDGE CASES AND BOUNDARIES
    // ============================================================

    #[test]
    fn handles_single_char_with_prose_marker() {
        let sentence = "\u{05D0}\u{0592}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn handles_single_char_with_poetry_marker() {
        let sentence = "\u{05D0}\u{05AD}";
        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn handles_very_long_sentence_with_marker_at_end() {
        // Performance test - early exit should work efficiently
        let mut sentence = "א".repeat(1000);
        sentence.push('\u{0592}'); // Add prose marker at the end

        let start = std::time::Instant::now();
        let result = detect_context_from_sentence(&sentence);
        let elapsed = start.elapsed();

        assert_eq!(result, Ok(Context::Prosaic));
        // Should complete quickly (< 1ms)
        assert!(elapsed.as_micros() < 10000);
    }

    #[test]
    fn handles_all_prose_markers_in_sequence() {
        let prose_markers: Vec<char> = vec![
            '\u{0592}', // Segolta
            '\u{0594}', // ZaqephQatan
            '\u{0595}', // ZaqephGadol
            '\u{0599}', // Paseq
            '\u{059B}', // Tevir
            '\u{059A}', // Yetiv
            '\u{059E}', // Gershayim
            '\u{059F}', // PazerGadol
            '\u{05A0}', // TelishaGedolah
            '\u{05A6}', // MerkaKephula
            '\u{05A7}', // Darga
        ];

        for marker in prose_markers {
            let sentence = format!("\u{05D0}{}\u{05D1}", marker);
            let result = detect_context_from_sentence(&sentence);
            assert_eq!(result, Ok(Context::Prosaic), "Marker {} failed", marker);
        }
    }

    #[test]
    fn handles_all_poetry_markers_in_sequence() {
        let poetry_markers: Vec<char> = vec![
            '\u{05AB}', // OlehWeYored
            '\u{0597}', // TsinnoritMerkha
            '\u{05AD}', // Dechi
            '\u{05AC}', // Illuy
            '\u{0598}', // TsinnoritMahpakh
        ];

        for marker in poetry_markers {
            let sentence = format!("\u{05D0}{}\u{05D1}", marker);
            let result = detect_context_from_sentence(&sentence);
            assert_eq!(result, Ok(Context::Poetic), "Marker {} failed", marker);
        }
    }

    #[test]
    fn handles_marker_adjacent_to_another_marker() {
        // Two consecutive markers (edge case)
        let sentence = "\u{05D0}\u{0592}\u{05AD}\u{05D1}";
        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(_))
        ));
    }

    #[test]
    fn handles_non_hebrew_but_valid_chars() {
        // ZWJ, ZWNJ, CGJ are allowed in the validator
        let sentence = "\u{05D0}\u{200D}\u{05D1}"; // With ZWJ
        let result = detect_context_from_sentence(sentence);

        // Should fail derivation (no exclusive markers) but pass validation
        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }
}
