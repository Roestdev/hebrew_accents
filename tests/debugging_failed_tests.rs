use hebrew_accents::{Context, HebrewAccent, ProseAccent, SentenceContext};

#[test]
fn doctest() {
    println!("=== Running doctest as integration test ===");

    let sentence_context_result = SentenceContext::new(
        "וַיּ֣רָא עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
        Context::Prosaic,
    );
    if let Ok(sentence_context) = sentence_context_result {
        // Check if an accent exists in a given sentence
        if sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)) {
            println!("Tiphcha found");
        }
        if let Some(match_) = sentence_context.find_accent(ProseAccent::Atnach.into()) {
            println!(
                "Atnach found at bytes {} untill {}",
                match_.start(),
                match_.end()
            );
            println!("Text: {}", match_.as_str());
        }
    }

    // Find accent positions
    // Note: you can also use `.into()` due to the `From` trait implementation.
}
