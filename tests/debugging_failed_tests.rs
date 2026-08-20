use hebrew_accents::{Context, HebrewAccent, ProseAccent, SentenceContext, SentenceContextError};

#[test]
fn doctest() -> Result<(), SentenceContextError> {
    let sentence_context = SentenceContext::new(
        "וַיּ֣רָא עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
        Context::Prosaic,
    )?;

    // Check if an accent exists in a given sentence
    assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));

    // Find accent positions
    // Note: you can also use `.into()` due to the `From` trait implementation.
    if let Some(match_) = sentence_context.find_accent(ProseAccent::Atnach.into()) {
        println!("Atnach found at bytes {}: {}", match_.start(), match_.end());
        println!("Text: {}", match_.as_str());
    }

    Ok(())
}
