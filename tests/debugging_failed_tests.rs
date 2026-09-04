
#[test]
fn doctest() {
    println!("=== Running doctest as integration test ===");

    use hebrew_accents::{Context, ProseAccent, SentenceContext};

    // Find a match in Hebrew text
    let sentence = "וְנִשְׁמַרְתֶּ֥ם מְאֹ֖ד לְנַפְשֹֽׁתֵיכֶ֑ם לְאַהֲבָ֖ה אֶת־יְהוָ֥ה אֱלֹהֵיכֶֽם׃";
    //let sent_ctx_res = SentenceContext::new(sentence,Context::Prosaic);
    if let Ok(sent_ctx) = SentenceContext::new(sentence, Context::Prosaic) {
        let matched = sent_ctx.find_accent(ProseAccent::Atnach.into());
        match matched {
            Some(match_res) => {
                // Access match properties
                assert_eq!(match_res.start(), 76);
                assert_eq!(match_res.end(), 78);
                assert_eq!(match_res.len(), 2);
                assert_eq!(match_res.as_str(), "\u{591}");
                assert_eq!(match_res.range(), 76..78);
                // Check if empty
                assert!(!match_res.is_empty());
            }
            None => {}
        }
    }
}
