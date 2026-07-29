use crate::sentence::validator::validate_sentence;
use crate::Context;
use crate::SentenceContext;
use crate::SentenceContextError;
use crate::{PoetryAccent, ProseAccent};

pub(crate) fn detect_context_from_sentence(
    sentence: &str,
) -> Result<Context, SentenceContextError> {
    validate_sentence(sentence)?;

    let mut could_be_prose = false;
    let mut could_be_poetry = false;

    // Assume Prosaic and check for prose-exclusive accents
    let assume_prose = SentenceContext::new(sentence, Context::Prosaic)?;
    if assume_prose.contains_accent(ProseAccent::Segolta.into())
        || assume_prose.contains_accent(ProseAccent::ZaqephQatan.into())
        || assume_prose.contains_accent(ProseAccent::ZaqephGadol.into())
        || assume_prose.contains_accent(ProseAccent::Pashta.into())
        || assume_prose.contains_accent(ProseAccent::Tevir.into())
        || assume_prose.contains_accent(ProseAccent::Yetiv.into())
        || assume_prose.contains_accent(ProseAccent::Gershayim.into())
        || assume_prose.contains_accent(ProseAccent::PazerGadol.into())
        || assume_prose.contains_accent(ProseAccent::TelishaGedolah.into())
        || assume_prose.contains_accent(ProseAccent::MerkhaKephulah.into())
        || assume_prose.contains_accent(ProseAccent::Darga.into())
        || assume_prose.contains_accent(ProseAccent::TelishaQetannah.into())
    {
        could_be_prose = true;
    }

    // Assume Poetic and check for poetry-exclusive accents
    let assume_poetry = SentenceContext::new(sentence, Context::Poetic)?;
    if assume_poetry.contains_accent(PoetryAccent::OlehWeYored.into())
        || assume_poetry.contains_accent(PoetryAccent::ReviaMugrash.into())
        || assume_poetry.contains_accent(PoetryAccent::Dechi.into())
        || assume_poetry.contains_accent(PoetryAccent::Illuy.into())
        || assume_poetry.contains_accent(PoetryAccent::TsinnoritMerkha.into())
        || assume_poetry.contains_accent(PoetryAccent::TsinnoritMahpakh.into())
    {
        could_be_poetry = true;
    }

    // Determine context based upon the findings
    match (could_be_prose, could_be_poetry) {
        (true, false) => Ok(Context::Prosaic),
        (false, true) => Ok(Context::Poetic),
        (true, true) => Err(SentenceContextError::DerivationFailed(
            "Unique prose and poetry accent markers identified",
        )),
        (false, false) => Err(SentenceContextError::DerivationFailed(
            "No unique prose or poetry accent markers identified",
        )),
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
mod tests {
    use super::*;
    use crate::SentenceContextError;

    // ============================================================
    //  BRANCH: (true, false) — Pure Prose Detection
    // ============================================================

    #[test]
    fn detects_pure_prose_with_segolta_returns_prosaic() {
        // Segolta (U+0592) is prose-only, no poetry accents present
        let sentence = "\u{05D0}\u{0592}\u{05D1}"; // א֒ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_zaqeph_qatan_returns_prosaic() {
        // Zaqeph Qatan (U+0594) is prose-only
        let sentence = "\u{05D0}\u{0594}\u{05D1}"; // א֔ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_zaqeph_gadol_returns_prosaic() {
        // Zaqeph Gadol (U+0595) is prose-only
        let sentence = "\u{05D0}\u{0595}\u{05D1}"; // א֕ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_pashta_returns_prosaic() {
        // Pashta (U+0599) is prose-only
        let sentence = "\u{05D0}\u{0599}\u{05D1}"; // א֙ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_tevir_returns_prosaic() {
        // Tevir (U+059B) is prose-only
        let sentence = "\u{05D0}\u{059B}\u{05D1}"; // א֛ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_yetiv_returns_prosaic() {
        // Yetiv (U+059A) is prose-only
        let sentence = "\u{05D0}\u{059A}\u{05D1}"; // א֚ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_gershayim_returns_prosaic() {
        // Gershayim (U+059E) is prose-only
        let sentence = "\u{05D0}\u{059E}\u{05D1}"; // א֞ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_pazer_gadol_returns_prosaic() {
        // Pazer Gadol/Qarney Para (U+059F) is prose-only
        let sentence = "\u{05D0}\u{059F}\u{05D1}"; // א֟ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_telisha_gedolah_returns_prosaic() {
        // Telisha Gedolah (U+05A0) is prose-only
        let sentence = "\u{05D0}\u{05A0}\u{05D1}"; // א֠ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_merkha_kephulah_returns_prosaic() {
        // Merkha Kephulah/Merkha Kefula (U+05A6) is prose-only
        let sentence = "\u{05D0}\u{05A6}\u{05D1}"; // א֦ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_darga_returns_prosaic() {
        // Darga (U+05A7) is prose-only
        let sentence = "\u{05D0}\u{05A7}\u{05D1}"; // א֧ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn detects_pure_prose_with_telisha_qetannah_returns_prosaic() {
        // Telisha Qetannah (U+05A9) is prose-only
        let sentence = "\u{05D0}\u{05A9}\u{05D1}"; // א֩ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    // ============================================================
    //  BRANCH: (false, true) — Pure Poetry Detection
    // ============================================================

    #[test]
    fn detects_pure_poetry_with_oleh_we_yored_returns_poetic() {
        // Oleh WeYored (Ole U+05AB) is poetry-only
        let sentence = "\u{05D0}\u{05AB}\u{05D1}"; // א֫ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_pure_poetry_with_revia_mugrash_returns_poetic() {
        // Revia Mugrash (Revia U+0597) is poetry-only
        let sentence = "\u{05D0}\u{0597}\u{05D1}"; // א֗ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_pure_poetry_with_dechi_returns_poetic() {
        // Dehi/Dechi (U+05AD) is poetry-only
        let sentence = "\u{05D0}\u{05AD}\u{05D1}"; // א֭ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_pure_poetry_with_illuy_returns_poetic() {
        // Illuy/Iluy (U+05AC) is poetry-only
        let sentence = "\u{05D0}\u{05AC}\u{05D1}"; // א֬ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_pure_poetry_with_tsinnorit_merkha_returns_poetic() {
        // Tsinnorit/Zarqa (U+0598) is poetry-only
        let sentence = "\u{05D0}\u{0598}\u{05D1}"; // א֘ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    #[test]
    fn detects_pure_poetry_with_tsinnorit_mahpakh_returns_poetic() {
        // Mahpakh (U+05A4) combined with Tsinnorit is poetry
        let sentence = "\u{05D0}\u{05A4}\u{05D1}"; // א֤ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Poetic));
    }

    // ============================================================
    //  BRANCH: (true, true) — Both Prose & Poetry (Derivation Failed)
    // ============================================================

    #[test]
    fn detects_both_prose_and_poetry_accents_returns_derivation_failed() {
        // Segolta (U+0592) is prose-only, Dehi (U+05AD) is poetry-only
        let sentence = "\u{05D0}\u{0592}\u{05D1}\u{05AD}\u{05D2}"; // א֒ב֭ג

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    #[test]
    fn detects_multiple_prose_and_poetry_accents_returns_derivation_failed() {
        // Zaqeph Qatan (U+0594) prose + Illuy (U+05AC) poetry
        let sentence = "\u{05D0}\u{0594}\u{05D1}\u{05AC}\u{05D2}"; // א֔ב֬ג

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "Unique prose and poetry accent markers identified"
        ));
    }

    // ============================================================
    //  BRANCH: (false, false) — No Unique Markers (Derivation Failed)
    // ============================================================

    #[test]
    fn no_unique_markers_returns_derivation_failed() {
        // Common marks (no prose/poetry exclusive ones): EtNacha (U+0591)
        // Etnahta is shared/common to both systems
        let sentence = "\u{05D0}\u{0591}\u{05D1}"; // א֑ב

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn pure_hebrew_text_no_accents_returns_derivation_failed() {
        // No cantillation marks at all
        let sentence = "שלום"; // שלום

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }

    #[test]
    fn common_marks_only_returns_derivation_failed() {
        // Sheva (U+05B0), Etnahta (U+0591), Silluq (U+05C0) — shared/common
        let sentence = "\u{05D0}\u{05B0}\u{05D1}\u{0591}"; // אְב֑

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::DerivationFailed(ref msg))
                if *msg == "No unique prose or poetry accent markers identified"
        ));
    }

    // ============================================================
    //  VALIDATION ERROR PATHS (early returns before detection logic)
    // ============================================================

    #[test]
    fn empty_sentence_returns_validation_error() {
        let sentence = "";

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Err(SentenceContextError::EmptySentence));
    }

    #[test]
    fn whitespace_only_returns_validation_error() {
        let sentence = "   ";

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Err(SentenceContextError::EmptySentence));
    }

    #[test]
    fn multiline_sentence_returns_validation_error() {
        let sentence = "שלום\nעולם";

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Err(SentenceContextError::MultipleLines));
    }

    #[test]
    fn starts_with_final_form_returns_validation_error() {
        // ך (U+05DA) is Final KAF — not allowed at start
        let sentence = "\u{05DA}\u{0592}\u{05D1}"; // ך֒ב

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::StartsWithFinalForm('\u{05DA}'))
        ));
    }

    #[test]
    fn starts_with_niqqud_returns_validation_error() {
        // Sheva (U+05B0) at start — valid char but not a consonant
        let sentence = "\u{05B0}\u{05D0}\u{05D1}"; // ְאב

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::StartsWithNonConsonant('\u{05B0}'))
        ));
    }

    #[test]
    fn starts_with_ascii_returns_validation_error() {
        let sentence = "Hello שלום";

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::StartsWithNonConsonant('H'))
        ));
    }

    #[test]
    fn invalid_char_in_body_returns_validation_error() {
        // Latin 'X' in middle of Hebrew
        let sentence = "אבXג";

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::InvalidCharacter('X', 2))
        ));
    }

    #[test]
    fn starts_with_lrm_returns_validation_error() {
        // LRM (U+200E) is valid char but not whitespace/consonant
        let sentence = "\u{200E}\u{05D0}\u{05D1}"; // ‏אב

        let result = detect_context_from_sentence(sentence);

        assert!(matches!(
            result,
            Err(SentenceContextError::StartsWithNonConsonant('\u{200E}'))
        ));
    }

    // ============================================================
    //  EDGE CASES
    // ============================================================

    #[test]
    fn sentence_with_leading_whitespace_valid_start_passes_detection() {
        // Leading spaces are skipped, then valid start
        let sentence = "  \u{05D0}\u{0592}\u{05D1}"; //   א֒ב

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }

    #[test]
    fn long_sentence_with_prose_mark_passes_detection() {
        // Longer sentence with prose accent
        let sentence = "וַיְהִי\u{0592} בֹקֶר"; // With Segolta

        let result = detect_context_from_sentence(sentence);

        assert_eq!(result, Ok(Context::Prosaic));
    }
}
