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
