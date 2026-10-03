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
