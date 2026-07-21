/// hjkl;
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
pub(crate) fn try_determine_context(sentence: &str) -> Result<Context, SentenceContextError> {
    detect_context_from_sentence(sentence)
}