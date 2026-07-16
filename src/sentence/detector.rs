use crate::sentence::context::Context;
use crate::sentence::sentence_context::SentenceContext;
use crate::sentence::validator::validate_sentence;
use crate::SentenceContextError;
use crate::{PoetryAccent, ProseAccent};

/// Try to determine the context of a Hebrew sentence from its accent CantillationSymbol.
///
/// This is a convenience function for detecting context without creating a
/// [`SentenceContext`] instance first.
/// See `try_determine_context` on `SentenceContext` for detailed documentation.
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
