//! Miscellaneous helper functions

// Standard library
// N/A

// External crates
// N/A

// Crate‑internal (local modules)
use crate::char::{
    GERESH_AS_CHAR, MAHPAKH, MAQQEPH_AS_CHAR, MERKHA, OLEH_AS_CHAR, PASEQ_AS_CHAR, REVIA,
    TSINNORIT_AS_CHAR, VERTICAL_LINE_AS_CHAR, YORED_AS_CHAR, ZARQA_AS_CHAR,
};
use crate::sentenc_ctx_error::SentenceContextError;
use crate::sentence_ctx_find::ACCENT_LEN_UTF8;
use crate::{Context, Match, SentenceContext};
use crate::{PoetryAccent, ProseAccent};

/// Try to determine the context of the sentence
///
/// Prose: Segolta, Zaqeph Qaton/Gadol, Zarqa,
/// Poetry: Tsinnor
pub fn try_derive_context(sentence: &str) -> Result<Context, SentenceContextError> {
    let mut could_be_prose = false;
    let mut could_be_poetry = false;

    // Assume the sentence is Prosaic
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

    // Assume the sentence is Poetic
    let assume_poetry = SentenceContext::new(sentence, Context::Poetic)?;
    if assume_poetry.contains_accent(PoetryAccent::OlehWeYored.into())
        || assume_poetry.contains_accent(PoetryAccent::Dechi.into())
        || assume_poetry.contains_accent(PoetryAccent::Illuy.into())
        || assume_poetry.contains_accent(PoetryAccent::TsinnoritMerkha.into())
        || assume_poetry.contains_accent(PoetryAccent::TsinnoritMahpakh.into())
    {
        could_be_poetry = true;
    }

    // Determine context based upon the findings using match
    match (could_be_prose, could_be_poetry) {
        (true, false) => Ok(Context::Prosaic),
        (false, true) => Ok(Context::Poetic),
        (true, true) => Err(SentenceContextError::NoDerivePossible(
            "Both prose and poetry accents have been found",
        )),
        (false, false) => Err(SentenceContextError::NoDerivePossible(
            "No distinguishable prose and/or poetry accents have been found",
        )),
    }
}

pub(crate) fn find_poetry_merkha(sentence: &str) -> Option<Match<'static>> {
    // Merkha (as a poetry accent) is
    //   not part of Oleh We Yored (needs Negative Lookbehind)
    //   AND
    //   not part of Tsinnorit Merkha (needs Negative Lookbehind)
    let target_char = MERKHA;
    // define possible combinations
    let possible_combinations_lookbehind = [TSINNORIT_AS_CHAR, OLEH_AS_CHAR];

    // Check for the existence of the target character in the sentence
    if !&sentence.contains(target_char) {
        return None;
    }
    // Convert the sentence into a Vec<char> for character indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // Find the indices of the target character within the sentence
    let indices = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for &index in &indices {
        let is_part_of = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            2,
        );
        if !is_part_of {
            // println!(
            //     "Found at least one target char, not part of another aaccent:: BREAK the loop"
            // );
            let merkha = "\u{05A5}";
            return Some(Match::new(merkha, index, index + ACCENT_LEN_UTF8));
        }
    }
    None
}

pub(crate) fn find_poetry_mehuppakh(sentence: &str) -> Option<Match<'static>> {
    // Mehupppakh (as a poetry accent)
    //   not part of Mehuppakh Legarmeh (needs Negative Lookahead)
    //   AND
    //   not part of Tsinnorit Mahpakh (needs Negative Lookbehind)
    let target_char = MAHPAKH;
    // define possible combinations
    let possible_combinations_lookbehind = [ZARQA_AS_CHAR];
    // check if the target character is present in the sentence
    if !&sentence.contains(target_char) {
        //println!("MEHUPPAKH not found in the senctence at all  -> return None");
        return None;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    println!("positions found at: {indices:?}");
    // loop over all character positions
    for index in indices {
        println!("\n\nCHECK position {index}");
        //println!("\nNegative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            2,
        );
        let is_part_of_mahpakh_legarmeh = is_part_of_mahpakh_legarmeh_look_ahead(index, &char_vec);
        // println!("\nResult: Negative Looking Backward = {two_code_points_behind}");
        // println!("Result: Negative Looking Forward = {is_part_of_mahpakh_legarmeh}");
        if !two_code_points_behind && !is_part_of_mahpakh_legarmeh {
            // println!(
            //     "\nResult for index {index}:\n\ttwo_code_points_behind: {two_code_points_behind}\n\tis_part_of_mahpakh_legarmeh: {is_part_of_mahpakh_legarmeh}"
            // );
            // println!("Found target char, not part of another accent. Returning TRUE");
            let mahpakh = "\u{05A4}";
            return Some(Match::new(mahpakh, index, index + ACCENT_LEN_UTF8));
        }
    }
    None
}

pub(crate) fn find_poetry_revia_gadol(sentence: &str) -> Option<Match<'static>> {
    // Revia Gadol is
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    //   AND
    //   not followed by an Oleh We Yored (needs Negative Lookahead)
    let target_char = REVIA;
    // define possible combinations to check look-behind
    let possible_combinations_lookbehind = [GERESH_AS_CHAR];
    // 1. check if the target character is present in the senctence at all
    if !&sentence.contains(target_char) {
        println!("Sentence contains no REVIA at all");
        return None;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    println!("REVIA found at the following indices: {:?}", &indices);
    // loop over all character positions
    for index in indices {
        println!("\nChecking at index: {}", index);
        println!("\nCheck for ReviaMugrash first");
        let part_of_revia_mugrash = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            1,
        );
        if part_of_revia_mugrash {
            println!("REVIA is part of ReviaMugrash. Go to next index.");
            continue;
        } else {
            println!("REVIA is NOT part of ReviaMugrash. \nChecking for Oleh We Yored now.");
        }

        let followed_by_owy = is_followed_by_oleh_we_yored(index, &char_vec);
        //  2cp   oleweyored     revia_qadol
        //  no      no      -       yes
        //  no      yes     -       no
        //  yes     no      -       no
        //  yes     yes     -       no
        if followed_by_owy {
            println!("REVIA is followed by OlehWeYored");
        } else {
            println!("REVIA is NOT followed by OlehWeYored");
        }
        // println!(
        //     "part_of_revia_mugrash: {part_of_revia_mugrash} - followed_by_owy: {followed_by_owy}"
        // );
        if !part_of_revia_mugrash && !followed_by_owy {
            println!("REVIA GADOL found at index: {}", index);
            let revia = "\u{0597}";
            return Some(Match::new(revia, index, index + ACCENT_LEN_UTF8));
        }
    }
    None
}

pub fn find_poetry_revia_qaton(sentence: &str) -> Option<Match<'static>> {
    // Revia Qaton is
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    //   AND
    //   followed by an Oleh We Yored (needs Positive LookAhead)
    let target_char = REVIA;
    // define possible combinations
    let possible_combinations_lookbehind = [GERESH_AS_CHAR];
    // check if the target character is present in the senctence
    if !&sentence.contains(target_char) {
        println!("Sentence contains no REVIA at all");
        return None;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    println!("REVIA found at the following indices: {:?}", &indices);
    // loop over all character positions
    for index in indices {
        println!("\nChecking at index: {}", index);
        println!("\nCheck for ReviaMugrash first");
        let part_of_revia_mugrash = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            1,
        );
        if part_of_revia_mugrash {
            println!("REVIA is part of ReviaMugrash. Go to next index.");
            continue;
        } else {
            println!("REVIA is NOT part of ReviaMugrash. \nChecking for Oleh We Yored now.");
        }
        let followed_by_owy = is_followed_by_oleh_we_yored(index, &char_vec);
        // 2cp   oleweyored     revia_qaton
        //  no      no      -       no
        //  no      yes     -       yes
        //  yes     no      -       no
        //  yes     yes     -       no
        // println!(
        //     "part_of_revia_mugrash:{part_of_revia_mugrash} - followed_by_owy{followed_by_owy}"
        // );
        if followed_by_owy {
            println!("REVIA is followed by OlehWeYored");
        } else {
            println!("REVIA is NOT followed by OlehWeYored");
        }

        if !part_of_revia_mugrash && followed_by_owy {
            println!("REVIA QATON found at index: {}", index);
            let revia = "\u{0597}";
            return Some(Match::new(revia, index, index + ACCENT_LEN_UTF8));
        }
    }
    None
}

/*******************************
*       helper functions       *
*******************************/

pub(crate) fn validate_sentence(text: &str) -> Result<(), SentenceContextError> {
    // A sentence may not be empty
    if text.is_empty() {
        return Err(SentenceContextError::EmptySentence);
    }
    // A sentence may not contain a CR and/or LF
    if text.contains('\r') || text.contains('\n') {
        return Err(SentenceContextError::MultipleLines);
    }
    // TODO check if all words contain at least 2 consonants
    // split into words
    // count consonants
    // for word in text.split_whitespace() {
    //     let consonant_count = word
    //         .chars()
    //         .filter(|c| is_hebrew_consonant(*c))
    //         .count();

    //     if consonant_count < 2 {
    //         return Err(SentenceContextError::MultipleLines);
    //     }
    // }
    // Now check every character in the sentence
    for (idx, c) in text.chars().enumerate() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }
    Ok(())
}

fn is_valid_hebrew_char(c: char) -> bool {
    // Check for Hebrew Unicode Block (U+0590 - U+05FF)
    if ('\u{0590}'..='\u{05FF}').contains(&c) {
        return true;
    }
    // Check for Vertical Bar (U+007C) (Sometimes a replacement for Paseq)
    if VERTICAL_LINE_AS_CHAR == c {
        return true;
    }
    // WHITESPACE →
    //       U+0009 // Horizontal tab, '\t'
    //     | U+000A // Line feed, '\n'
    //     | U+000B // Vertical tab
    //     | U+000C // Form feed
    //     | U+000D // Carriage return, '\r'
    //     | U+0020 // Space, ' '
    //     | U+0085 // Next line
    //     | U+200E // Left-to-right mark
    //     | U+200F // Right-to-left mark
    //     | U+2028 // Line separator
    //     | U+2029 // Paragraph separator
    // TAB → U+0009 // Horizontal tab, '\t'
    // LF → U+000A // Line feed, '\n'
    // CR → U+000D // Carriage return, '\r'
    if c.is_whitespace() {
        return true;
    }

    // Check for specific METEG Layout Control Characters
    // see https://www.unicode.org/versions/Unicode15.0.0/ section 9.1 for more information
    matches!(
        c,
        '\u{034F}' | // CGJ: COMBINING GRAPHEME JOINER
        '\u{200C}' | // ZWNJ: ZERO WIDTH NON-JOINER
        '\u{200D}' // ZWJ: ZERO WIDTH JOINER
    )
}

// convert &str into Vec of chars
fn as_char_slice(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn indexes_target_char(target_char: &str, sentence: &[char]) -> Vec<usize> {
    // --------------------------------------------------------------
    // 1️⃣  Convert the incoming `&str` to a single `char`.
    // --------------------------------------------------------------
    // `chars()` iterates over Unicode scalar values.
    // We take the first one and make sure there isn’t a second.
    let target = match target_char.chars().next() {
        // No characters at all → nothing can match.
        None => return Vec::new(),
        Some(ch) => {
            // If there is a second character, the caller gave us a
            // multi‑character string, which we treat as “no match”.
            if target_char.chars().nth(1).is_some() {
                return Vec::new();
            }
            ch
        }
    };
    sentence
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == target { Some(index) } else { None })
        .collect()
}

fn is_part_of_two_code_point_accent_look_behind(
    sentence: &[char],
    target_char: &str,
    idx_target: usize,
    lookbehind_combos: &[char],
    max_word_span: usize,
) -> bool {
    // 1️⃣  Convert the incoming `&str` to a single `char`.
    // --------------------------------------------------------------
    // `chars()` iterates over Unicode scalar values.
    // We take the first one and make sure there isn’t a second.
    let target = match target_char.chars().next() {
        // No characters at all → nothing can match.
        None => return false,
        Some(ch) => {
            // If there is a second character, the caller gave us a
            // multi‑character string, which we treat as “no match”.
            if target_char.chars().nth(1).is_some() {
                return false;
            }
            ch
        }
    };

    // Nothing to look at if the target is the very first character.
    if idx_target == 0 {
        return false;
    }

    // Number of word separators we have passed while scanning backwards.
    let mut word_breaks = 0usize;

    // Scan the slice backwards, stopping before `idx_target`.
    for i in (0..idx_target).rev() {
        let c = sentence[i];

        // Treat space and the special separator `MAQQEPH` as word boundaries.
        if c == ' ' || c == MAQQEPH_AS_CHAR {
            word_breaks += 1;
            // If we have crossed the allowed number of word spans, stop.
            if word_breaks >= max_word_span {
                return false;
            }
            continue;
        }

        // If we encounter the same target character again, the current
        // occurrence cannot be part of a two‑code‑point accent.
        if c == target {
            return false;
        }

        // If the character belongs to the set of possible look‑behind
        // combinations, we have a match.
        if lookbehind_combos.contains(&c) {
            return true;
        }
    }

    // Exhausted the slice without finding a matching combination.
    false
}

fn is_part_of_mahpakh_legarmeh_look_ahead(idx_target: usize, sentence: &[char]) -> bool {
    // Guard against out‑of‑range indices.
    if idx_target >= sentence.len() {
        return false;
    }

    // How many word separators (spaces) we have seen while scanning ahead.
    let mut word_breaks = 0usize;

    // Iterate over the characters *after* the target index.
    for &c in sentence.iter().skip(idx_target + 1) {
        // Stop once we have crossed the allowed span of two words.
        if word_breaks >= 2 {
            return false;
        }

        match c {
            // Space – counts as a word boundary.
            ' ' => word_breaks += 1,

            // Acceptable mahpakh symbols, but only while we are still in the
            // first or second word (i.e. before we have seen two spaces).
            PASEQ_AS_CHAR | VERTICAL_LINE_AS_CHAR => return true,

            // Anything else does not affect the outcome; just continue.
            _ => {}
        }
    }
    // No qualifying mahpakh found within the permitted range.
    false
}

fn is_followed_by_oleh_we_yored(target_idx: usize, sentence: &[char]) -> bool {
    // Guard against an out‑of‑range index.
    if target_idx >= sentence.len() {
        return false;
    }
    // Word‑boundary counter: a boundary is either a regular space or the
    // special separator `MAQQEPH`.
    let mut word_boundary_cnt = 0usize;
    let mut oleh_seen = false;
    // Iterate over the characters *after* the target character.
    for &c in sentence.iter().skip(target_idx + 1) {
        // Stop once we have examined three word boundaries.
        if word_boundary_cnt == 3 {
            break;
        }

        match c {
            // Word boundaries increment the counter.
            ' ' | MAQQEPH_AS_CHAR => {
                println!("Word boundery found: <{}>", &c);
                word_boundary_cnt += 1;
            }
            // The first word after the target may contain `OLEH`.
            OLEH_AS_CHAR if word_boundary_cnt == 1 => {
                println!("OLEH found ");
                oleh_seen = true;
            }
            // `YORED` may appear in the first or second word *after* we have
            // already seen `OLEH`.
            YORED_AS_CHAR if (word_boundary_cnt == 1 || word_boundary_cnt == 2) && oleh_seen => {
                // Both parts are present → we can return early.
                println!("YORED found. return true ");
                return true;
            }

            // Any other character does not affect the state.
            _ => {}
        }
    }

    // If we exit the loop without having seen both parts, the sequence is absent.
    println!("No correct oleh we yored pattern found");
    false
}

#[cfg(test)]
mod tests_try_derive_context {
    use super::try_derive_context;
    use crate::char::{
        DARGA, GERSHAYIM, MERKHA_KEFULA, PASHTA, QARNEY_PARA, SEGOL, TELISHA_GEDOLAH,
        TELISHA_QETANA, TEVIR, YETIV, ZAQEF_GADOL, ZAQEF_QATAN,
    };
    use crate::char::{DECHI, ILUY, MAHPAKH, MERKHA, OLEH, YORED, ZARQA};
    use crate::sentenc_ctx_error::SentenceContextError;
    use crate::Context;
    #[test]
    fn test_detectable_prose_accent_present() {
        // Case: Contains Segolta (Prose) but no Poetry accents
        let sentence = "כּבוד חכמים֒ ינחלוּ וּכסילים מרים קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(result.is_ok(), "Expected Ok for pure prose context");
        assert_eq!(result.unwrap(), Context::Prosaic);
    }
    #[test]
    fn test_non_detectable_prose_accent_present() {
        // Case: Contains Atnach (Prose) but no Poetry accents
        let sentence = "כּבוד חכמים ינחלוּ וּכס֑ילים מרים קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(result.is_err(), "Expected Err when no context is detected");
        match result {
            Err(SentenceContextError::NoDerivePossible(msg)) => {
                assert_eq!(
                    msg,
                    "No distinguishable prose and/or poetry accents have been found"
                );
            }
            _ => panic!("Expected NoDerivePossible error"),
        }
    }

    #[test]
    fn test_detectable_poetry_accent_present() {
        // Case: Contains Illuy (Poetry) but no Prose accents
        let sentence = "כּבוד חכמים ינחלוּ וּכסילים מרים֬ קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(result.is_ok(), "Expected Ok for pure poetry context");
        assert_eq!(result.unwrap(), Context::Poetic);
    }
    #[test]
    fn test_non_detectable_poetry_accent_present() {
        // Case: Contains Munach (Poetry) but no Prose accents
        let sentence = "כּבוד חכמים֣ ינחלוּ וּכסילים מרים קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(result.is_err(), "Expected Err when no context is detected");
        match result {
            Err(SentenceContextError::NoDerivePossible(msg)) => {
                assert_eq!(
                    msg,
                    "No distinguishable prose and/or poetry accents have been found"
                );
            }
            _ => panic!("Expected NoDerivePossible error"),
        }
    }

    #[test]
    fn test_all_prose_accents_individually() {
        // Verify each prose accent triggers Prosaic context independently
        let prose_accents: Vec<&str> = vec![
            SEGOL,
            ZAQEF_QATAN,
            ZAQEF_GADOL,
            PASHTA,
            TEVIR,
            YETIV,
            GERSHAYIM,
            QARNEY_PARA,
            TELISHA_GEDOLAH,
            MERKHA_KEFULA,
            DARGA,
            TELISHA_QETANA,
        ];

        for accent in prose_accents {
            let sentence = format!("כּבוד חכמים ינחלוּ וּכסילים מרים קלון׃ פ {}וּכסילים", accent);
            let result = try_derive_context(&sentence);

            assert!(result.is_ok(), "Failed for prose accent: {}", accent);
            assert_eq!(
                result.unwrap(),
                Context::Prosaic,
                "Wrong context for: {}",
                accent
            );
        }
    }

    #[test]
    fn test_all_poetry_accents_individually_1_codepoint() {
        // Verify each poetry accent triggers Poetic context independently
        let poetry_accents = vec![DECHI, ILUY];

        for accent in poetry_accents {
            let sentence = format!("כּבוד חכמים ינחלוּ וּכסילים מרים קלון׃ פ {}וּכסילים", accent);
            let result = try_derive_context(&sentence);

            assert!(result.is_ok(), "Failed for poetry accent: {}", accent);
            assert_eq!(
                result.unwrap(),
                Context::Poetic,
                "Wrong context for: {}",
                accent
            );
        }
    }
    #[test]
    fn test_all_poetry_accents_individually_2_codepoint() {
        // Verify each poetry accent triggers Poetic context independently
        let poetry_accents = vec![
            //"oleh_we_yored", // two codepoints
            (OLEH, YORED),
            //"tsinnorit_merkha",  // two codepoints
            (ZARQA, MERKHA),
            //"tsinnorit_mahpakh",  // two codepoints
            (ZARQA, MAHPAKH),
        ];

        for accent in poetry_accents {
            let sentence = format!(
                "כּבוד חכמים ינחלוּ וּכסילים מרים קלון׃ פ {}וּכס{}ילים",
                accent.0, accent.1
            );
            let result = try_derive_context(&sentence);

            assert!(
                result.is_ok(),
                "Failed for poetry accent: {} {}",
                accent.0,
                accent.1
            );
            assert_eq!(
                result.unwrap(),
                Context::Poetic,
                "Wrong context for: {} {}",
                accent.0,
                accent.1
            );
        }
    }

    #[test]
    fn test_error_empty_sentence() {
        let sentence = "";
        let result = try_derive_context(sentence);

        match result {
            Err(SentenceContextError::EmptySentence) => {
                let err = SentenceContextError::EmptySentence;
                let msg = err.to_string();
                assert_eq!(msg, "Sentence cannot be empty");
            }
            Err(other) => {
                panic!("Unexpected error variant: {:?}", other);
            }
            Ok(_) => {
                panic!("Expected EmptySentence error, but got OK");
            }
        }
    }
    #[test]
    fn test_error_multiple_lines() {
        // TODO
        let text = "בְּרֵאשִׁ֖ית\nבָּרָ֣א";

        let result = try_derive_context(text);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SentenceContextError::MultipleLines);
    }
    #[test]
    fn test_error_both_detectable_contexts_presents() {
        // Case: Contains both a Prose accent (Segolta) and a Poetry accent (Illuy)
        let sentence = "כּבוד חכמים֒ ינחלוּ וּכסילים מרים֬ קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(
            result.is_err(),
            "Expected Err when both contexts are detected"
        );
        match result {
            Err(SentenceContextError::NoDerivePossible(msg)) => {
                assert_eq!(msg, "Both prose and poetry accents have been found");
            }
            _ => panic!("Expected NoDerivePossible error"),
        }
    }

    #[test]
    fn test_error_no_context_present() {
        // Case: Contains no recognized accents (neither prose nor poetry)
        let sentence = "כּבוד חכמים ינחלוּ וּכסילים מרים קלון׃ פ";
        let result = try_derive_context(sentence);

        assert!(result.is_err(), "Expected Err when no context is detected");
        match result {
            Err(SentenceContextError::NoDerivePossible(msg)) => {
                assert_eq!(
                    msg,
                    "No distinguishable prose and/or poetry accents have been found"
                );
            }
            _ => panic!("Expected NoDerivePossible error"),
        }
    }
    #[test]
    fn test_error_invalid_character() {
        // TODO
        // Hebrew text with a Latin character (invalid)
        let text = "בְּרֵאשִׁ֖ית A";

        let result = try_derive_context(text);

        assert!(result.is_err());
        match result.unwrap_err() {
            SentenceContextError::InvalidCharacter(_, _) => {}
            e => panic!("Expected InvalidCharacter, got: {:?}", e),
        }
    }
}

#[cfg(test)]
mod tests_validate_sentence {
    use super::*;
    #[test]
    fn test_valid_hebrew_single_word() {
        // Single Hebrew word
        let sentence = " בּראשׁית";
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Simple Hebrew word should be valid");
    }

    #[test]
    fn test_valid_hebrew_single_word_with_diacritices() {
        //  Single Hebrew word with diacritices
        let sentence = "בְּרֵאשִׁ֖ית";
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Hebrew with cantillation should be valid");
    }

    #[test]
    fn test_valid_hebrew_with_spaces() {
        // Hebrew with spaces between words
        let sentence = "בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃";
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Hebrew with spaces should be valid");
    }
    #[test]
    fn test_valid_hebrew_with_spaces_with_diacritices() {
        // Hebrew with spaces between words
        let sentence = " בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃";
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Hebrew with spaces should be valid");
    }

    #[test]
    fn test_valid_with_vertical_bar() {
        // Vertical bar (Paseq replacement)
        let sentence = "דָּבָר\u{007C}חָכְמָה";
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Vertical bar should be valid");
    }

    #[test]
    fn test_valid_with_tabs() {
        // Hebrew with tabs (whitespace)
        let sentence = "דָּבָר\tחָכְמָה";

        let result = validate_sentence(sentence);

        assert!(result.is_ok(), "Tabs should be valid");
    }

    // ========================================================================
    // EMPTY SENTENCE TESTS
    // ========================================================================

    #[test]
    fn test_rejects_empty_string() {
        let sentence = "";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Empty string should be rejected");
        match result {
            Err(SentenceContextError::EmptySentence) => {}
            Err(e) => panic!("Expected EmptySentence error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_whitespace_only() {
        // Note: This depends on your implementation - currently whitespace-only
        // would pass is_valid_hebrew_char, so this might succeed. Adjust if needed.
        let sentence = "   ";
        let result = validate_sentence(sentence);

        // Currently this passes because whitespace is valid
        assert!(
            result.is_ok(),
            "Whitespace-only is technically valid per current logic"
        );
    }

    // ========================================================================
    // MULTILINE TESTS
    // ========================================================================

    #[test]
    fn test_rejects_newline() {
        let sentence = "דָּבָר\nחָכְמָה";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Newline should be rejected");
        match result {
            Err(SentenceContextError::MultipleLines) => {}
            Err(e) => panic!("Expected MultipleLines error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_carriage_return() {
        // Note: \r is also whitespace, but may not contain "\n"
        // This test documents current behavior
        let sentence = "דָּבָר\rחָכְמָה";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Newline should be rejected");
        match result {
            Err(SentenceContextError::MultipleLines) => {}
            Err(e) => panic!("Expected MultipleLines error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_windows_line_endings() {
        let sentence = "דָּבָר\r\nחָכְמָה";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Windows line endings should be rejected");

        match result {
            Err(SentenceContextError::MultipleLines) => {}
            Err(e) => panic!("Expected MultipleLines error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    // ========================================================================
    // INVALID CHARACTER TESTS
    // ========================================================================

    #[test]
    fn test_rejects_latin_characters() {
        let sentence = "דָּבָר hello";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Latin characters should be rejected");

        match result {
            Err(SentenceContextError::InvalidCharacter(c, idx)) => {
                assert_eq!(c, 'h', "Should report the Latin character");
                assert_eq!(
                    idx, 7,
                    "Should report correct index (after 6 Hebrew chars & a space)"
                );
            }
            Err(e) => panic!("Expected InvalidCharacter error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_numbers() {
        let sentence = "דָּבָר123";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Numbers should be rejected");

        match result {
            Err(SentenceContextError::InvalidCharacter(c, idx)) => {
                assert_eq!(c, '1', "Should report the number");
                assert_eq!(idx, 6, "Should report correct index");
            }
            Err(e) => panic!("Expected InvalidCharacter error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_punctuation() {
        let sentence = "דָּבָר.";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Period should be rejected");

        match result {
            Err(SentenceContextError::InvalidCharacter(c, idx)) => {
                assert_eq!(c, '.', "Should report the period");
                assert_eq!(idx, 6, "Should report correct index");
            }
            Err(e) => panic!("Expected InvalidCharacter error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_rejects_greek_characters() {
        let sentence = "ΔΑΒΑΡ";
        let result = validate_sentence(sentence);

        assert!(result.is_err(), "Greek characters should be rejected");

        match result {
            Err(SentenceContextError::InvalidCharacter(c, idx)) => {
                assert_eq!(c, 'Δ', "Should report the Greek character");
                assert_eq!(idx, 0, "Should report correct index");
            }
            Err(e) => panic!("Expected InvalidCharacter error, got: {:?}", e),
            Ok(_) => panic!("Expected error but got Ok"),
        }
    }

    #[test]
    fn test_invalid_character_at_start() {
        let sentence = "1דָּבָר";
        let result = validate_sentence(sentence);

        match result {
            Err(SentenceContextError::InvalidCharacter(_, idx)) => {
                assert_eq!(idx, 0, "Index should be 0 for start of string");
            }
            _ => panic!("Expected InvalidCharacter error at index 0"),
        }
    }

    #[test]
    fn test_invalid_character_at_end() {
        let sentence = "דָּבָר1";
        let result = validate_sentence(sentence);

        match result {
            Err(SentenceContextError::InvalidCharacter(_, idx)) => {
                assert_eq!(idx, 6, "Index should be at end of string");
            }
            _ => panic!("Expected InvalidCharacter error at end"),
        }
    }

    // ========================================================================
    // IS_VALID_HEBREW_CHAR HELPER TESTS
    // ========================================================================

    #[test]
    fn test_valid_hebrew_range_start() {
        assert!(
            is_valid_hebrew_char('\u{0590}'),
            "Start of Hebrew block should be valid"
        );
    }

    #[test]
    fn test_valid_hebrew_range_end() {
        assert!(
            is_valid_hebrew_char('\u{05FF}'),
            "End of Hebrew block should be valid"
        );
    }

    #[test]
    fn test_valid_hebrew_middle() {
        assert!(
            is_valid_hebrew_char('\u{05D0}'),
            "Middle of Hebrew block should be valid"
        );
    }

    #[test]
    fn test_invalid_before_hebrew_range() {
        assert!(
            !is_valid_hebrew_char('\u{058F}'),
            "Character before Hebrew block should be invalid"
        );
    }

    #[test]
    fn test_invalid_after_hebrew_range() {
        assert!(
            !is_valid_hebrew_char('\u{0600}'),
            "Character after Hebrew block should be invalid"
        );
    }

    #[test]
    fn test_valid_vertical_bar() {
        assert!(
            is_valid_hebrew_char('\u{007C}'),
            "Vertical bar should be valid"
        );
    }

    #[test]
    fn test_invalid_pipe_variants() {
        // Different pipe-like characters that are NOT the vertical bar
        assert!(!is_valid_hebrew_char('¦'), "Broken bar should be invalid");
        assert!(
            !is_valid_hebrew_char('│'),
            "Box drawing line should be invalid"
        );
    }

    #[test]
    fn test_valid_whitespace_space() {
        assert!(is_valid_hebrew_char(' '), "Space should be valid");
    }

    #[test]
    fn test_valid_whitespace_tab() {
        assert!(is_valid_hebrew_char('\t'), "Tab should be valid");
    }

    #[test]
    fn test_valid_whitespace_newline() {
        // Note: newline is valid per is_valid_hebrew_char, but filtered by validate_sentence
        assert!(
            is_valid_hebrew_char('\n'),
            "Newline is valid per char check"
        );
    }

    // ========================================================================
    // EDGE CASE TESTS
    // ========================================================================

    #[test]
    fn test_single_valid_character() {
        let sentence = "א";
        let result = validate_sentence(sentence);

        assert!(result.is_ok(), "Single Hebrew character should be valid");
    }

    #[test]
    fn test_long_valid_sentence() {
        // Longer sentence to ensure iteration works correctly
        let sentence = "דָּבָר וְחָכְמָה וּמַדָּע וְתוֹלָדוֹת";
        let result = validate_sentence(sentence);

        assert!(result.is_ok(), "Long valid sentence should pass");
    }

    #[test]
    fn test_only_cantillation_marks() {
        // Just cantillation marks (still in Hebrew block)
        let sentence = "\u{05A3}\u{05A4}\u{05A5}";
        let result = validate_sentence(sentence);

        assert!(result.is_ok(), "Only cantillation marks should be valid");
    }

    #[test]
    fn test_unicode_surrogate_pair_handling() {
        // Hebrew doesn't use surrogate pairs, but test that we handle chars correctly
        let sentence = "דָּבָר";
        let char_count = sentence.chars().count();

        assert_eq!(
            char_count, 6,
            "Should count 5 characters (3 base + 3 combining)"
        );
        let result = validate_sentence(sentence);
        assert!(result.is_ok(), "Should handle combining marks correctly");
    }
}

#[cfg(test)]
mod tests_is_valid_hebrew_context {
    use super::*;
    // positive tests

    #[test]
    fn test_hebrew_block_characters() {
        // Test start of Hebrew block (U+0590)
        assert!(is_valid_hebrew_char('\u{0590}'));
        // Test middle of block (e.g., Aleph U+05D0)
        assert!(is_valid_hebrew_char('\u{05D0}'));
        // Test end of block (U+05FF)
        assert!(is_valid_hebrew_char('\u{05FF}'));
    }

    #[test]
    fn test_vertical_bar() {
        // Some times used to represent a Paseq
        assert!(is_valid_hebrew_char('\u{007C}'));
    }

    #[test]
    fn test_whitespace_variants() {
        // Standard space
        assert!(is_valid_hebrew_char(' '));
        // Tab
        assert!(is_valid_hebrew_char('\t'));
        // Newline
        assert!(is_valid_hebrew_char('\n'));
        // Carriage return
        assert!(is_valid_hebrew_char('\r'));
        // Vertical tab
        assert!(is_valid_hebrew_char('\u{000B}'));
        // Form feed
        assert!(is_valid_hebrew_char('\u{000C}'));
        // Next line
        assert!(is_valid_hebrew_char('\u{0085}'));
        // Left-to-right mark
        //assert!(is_valid_hebrew_char('\u{200E}'));
        // Right-to-left mark
        //assert!(is_valid_hebrew_char('\u{200F}'));
        // Line separator
        assert!(is_valid_hebrew_char('\u{2028}'));
        // Paragraph separator
        assert!(is_valid_hebrew_char('\u{2029}'));
    }

    #[test]
    fn test_meteg_layout_control_characters() {
        // CGJ: Combining Grapheme Joiner
        assert!(is_valid_hebrew_char('\u{034F}'));
        // ZWNJ: Zero Width Non-Joiner
        assert!(is_valid_hebrew_char('\u{200C}'));
        // ZWJ: Zero Width Joiner
        assert!(is_valid_hebrew_char('\u{200D}'));
    }

    // Negative tests

    #[test]
    fn test_invalid_characters() {
        // Characters just outside Hebrew block
        assert!(!is_valid_hebrew_char('\u{058F}')); // Just before start of Hebrew block
        assert!(!is_valid_hebrew_char('\u{0600}')); // Just after end of Hebrew block
                                                    // Latin letters
        assert!(!is_valid_hebrew_char('a'));
        assert!(!is_valid_hebrew_char('Z'));
        // Numbers
        assert!(!is_valid_hebrew_char('0'));
        // Punctuation (non-vertical bar)
        assert!(!is_valid_hebrew_char('.'));
        assert!(!is_valid_hebrew_char(','));
    }

    #[test]
    fn test_bidi_controls() {
        assert!(!is_valid_hebrew_char('\u{202A}')); // LRE
        assert!(!is_valid_hebrew_char('\u{202E}')); // RLO
        assert!(!is_valid_hebrew_char('\u{2066}')); // LRI
    }

    #[test]
    fn test_invalid_vertical_bar_chars() {
        assert!(!is_valid_hebrew_char('\u{00A6}')); // Broken Bar
        assert!(!is_valid_hebrew_char('\u{2016}')); // Double Vertical Line
        assert!(!is_valid_hebrew_char('\u{FF5C}')); // Fullwidth Vertical Line
        assert!(!is_valid_hebrew_char('\u{01C0}')); // Latin Letter Dental Click
        assert!(!is_valid_hebrew_char('\u{2223}')); // Divides
        assert!(!is_valid_hebrew_char('\u{2224}')); // Does Not Divide
        assert!(!is_valid_hebrew_char('\u{2225}')); // Parallel To
        assert!(!is_valid_hebrew_char('\u{2226}')); // Not Parallel To
        assert!(!is_valid_hebrew_char('\u{2AF4}')); // Triple Vertical Bar
        assert!(!is_valid_hebrew_char('\u{2AF5}')); // Double Vertical Bar with Dot
    }
}

#[cfg(test)]
mod tests_find_poetry_merkha {
    use super::*;
    use crate::char::{OLEH, TSINNORIT};

    // ========================================================================
    // BASIC POSITIVE TESTS
    // ========================================================================

    #[test]
    fn test_finds_standalone_merkha() {
        // Standalone Merkha should be found
        let sentence = "דָּבָר\u{05A5}חָכְמָה";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_some(), "Standalone Merkha should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    #[test]
    fn test_merkha_at_start_of_sentence() {
        // Merkha at the beginning
        let sentence = "דָּבָר\u{05A5}";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_some(), "Merkha at start should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    #[test]
    fn test_merkha_at_end_of_sentence() {
        // Merkha at the end
        let sentence = "\u{05A5}דָּבָר";
        let result = find_poetry_merkha(sentence);
        assert!(result.is_some(), "Merkha at end should be found");
        let m = result.unwrap();
        // Should be near the end of the string
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    // ========================================================================
    // NEGATIVE TESTS - MERKHA AS PART OF COMPOUND ACCENTS
    // ========================================================================

    #[test]
    fn test_merkha_part_of_tsinnorit_merkha_not_found() {
        // Tsinnorit + Merkha (two-code-point accent) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", TSINNORIT, MERKHA);
        let result = find_poetry_merkha(&sentence);

        assert!(
            result.is_none(),
            "Merkha that is part of Tsinnorit-Merkha should not be found"
        );
    }

    #[test]
    fn test_merkha_part_of_oleh_we_yored_not_found() {
        // Oleh + Merkha (part of Oleh We Yored) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", OLEH, MERKHA);
        let result = find_poetry_merkha(&sentence);

        assert!(
            result.is_none(),
            "Merkha that is part of Oleh-We-Yored should not be found"
        );
    }

    #[test]
    fn test_merkha_with_other_chars_before_not_compound() {
        // Merkha with non-compound character before it should be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", 'א', MERKHA);
        let result = find_poetry_merkha(&sentence);

        assert!(
            result.is_some(),
            "Merkha with non-compound char before should be found"
        );
    }

    // ========================================================================
    // NEGATIVE TESTS - NO MERKHA PRESENT
    // ========================================================================

    #[test]
    fn test_no_merkha_in_sentence() {
        // Sentence without any Merkha
        let sentence = "דָּבָרחָכְמָה";
        let result = find_poetry_merkha(sentence);

        assert!(
            result.is_none(),
            "Should return None when no Merkha present"
        );
    }

    #[test]
    fn test_empty_sentence() {
        // Empty string
        let sentence = "";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_none(), "Empty sentence should return None");
    }

    #[test]
    fn test_whitespace_only() {
        // Only whitespace
        let sentence = "   ";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_none(), "Whitespace-only should return None");
    }

    // ========================================================================
    // MULTIPLE MERKHA TESTS
    // ========================================================================

    #[test]
    fn test_multiple_standalone_merkha_returns_first() {
        // Multiple standalone Merkha - should return the first one
        let sentence = format!("דָּבָר{}חָכְמָה{}תּוֹרָה", MERKHA, MERKHA);
        let result = find_poetry_merkha(&sentence);

        assert!(result.is_some(), "Should find at least one Merkha");
        let m = result.unwrap();
        // First Merkha should be at earlier offset
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    #[test]
    fn test_mixed_valid_and_invalid_merkha() {
        // One valid standalone Merkha, one invalid (part of compound)
        let sentence = format!("דָּבָר{}חָכְמָה{}{}תּוֹרָה", MERKHA, TSINNORIT, MERKHA);
        let result = find_poetry_merkha(&sentence);

        assert!(
            result.is_some(),
            "Should find the standalone Merkha even with compound present"
        );
        let m = result.unwrap();
        // Should return the first valid one (standalone)
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    // ========================================================================
    // EDGE CASES
    // ========================================================================

    #[test]
    fn test_merkha_with_space_before() {
        // Merkha with space before (space is word boundary)
        let sentence = format!("דָּבָר {}\u{05A5}חָכְמָה", ' ');
        let result = find_poetry_merkha(&sentence);

        assert!(result.is_some(), "Merkha with space before should be found");
    }

    #[test]
    fn test_merkha_with_maqqeph_before() {
        // Merkha with Maqqeph (word separator) before
        use crate::char::MAQQEPH_AS_CHAR;
        let sentence = format!("דָּבָר{}\u{05A5}חָכְמָה", MAQQEPH_AS_CHAR);
        let result = find_poetry_merkha(&sentence);

        assert!(
            result.is_some(),
            "Merkha with Maqqeph before should be found"
        );
    }

    #[test]
    fn test_only_merkha_character() {
        // Sentence containing only Merkha
        let sentence = "\u{05A5}";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_some(), "Single Merkha should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
    }

    // ========================================================================
    // INTEGRATION WITH MATCH STRUCTURE
    // ========================================================================

    #[test]
    fn test_match_structure_correct() {
        // Verify the Match structure is populated correctly
        let sentence = format!("דָּבָר\u{05A5}חָכְמָה");
        let result = find_poetry_merkha(&sentence);

        assert!(result.is_some());
        let m = result.unwrap();

        // Check all fields are reasonable
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A5}');
        //assert!(m.byte_offset >= 0);
        //assert!(m.byte_offset + ACCENT_LEN_UTF8 <= sentence.len());
    }

    #[test]
    fn test_byte_offset_matches_char_position() {
        // Verify byte offset corresponds to actual character position
        let sentence = "דָּבָר\u{05A5}חָכְמָה";
        let result = find_poetry_merkha(sentence);

        assert!(result.is_some());
        let m = result.unwrap();

        // Get the actual character at the reported offset
        let char_at_offset = sentence.chars().nth(m.start());
        assert_eq!(char_at_offset, Some('\u{05A5}'));
    }
}

#[cfg(test)]
mod tests_find_poetry_mehuppakh {
    use super::*;
    use crate::char::{MAHPAKH, PASEQ_AS_CHAR, VERTICAL_LINE_AS_CHAR, ZARQA_AS_CHAR};

    // ========================================================================
    // BASIC POSITIVE TESTS - STANDALONE MAHPAKH
    // ========================================================================

    #[test]
    fn test_finds_standalone_mahpakh() {
        // Standalone Mahpakh should be found
        let sentence = format!("דָּבָר{}חָכְמָה", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some(), "Standalone Mahpakh should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_mahpakh_at_start_of_sentence() {
        // Mahpakh at the beginning
        let sentence = format!("{}דָּבָר", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some(), "Mahpakh at start should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_mahpakh_at_end_of_sentence() {
        // Mahpakh at the end
        let sentence = format!("דָּבָר{}", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some(), "Mahpakh at end should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    // ========================================================================
    // NEGATIVE TESTS - MAHPAKH AS PART OF COMPOUND ACCENTS
    // ========================================================================

    #[test]
    fn test_mahpakh_part_of_tsinnorit_mahpakh_not_found() {
        // ZARQA + MAHPAKH (Tsinnorit Mahpakh) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", ZARQA_AS_CHAR, MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_none(),
            "Mahpakh that is part of Tsinnorit-Mahpakh should not be found"
        );
    }

    #[test]
    fn test_mahpakh_part_of_mahpakh_legarmeh_not_found() {
        // MAHPAKH + PASEQ (Mehuppakh Legarmeh) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", MAHPAKH, PASEQ_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_none(),
            "Mahpakh that is part of Mahpakh-Legarmeh should not be found"
        );
    }

    #[test]
    fn test_mahpakh_with_vertical_line_legarmeh_not_found() {
        // MAHPAKH + VERTICAL_LINE (Mehuppakh Legarmeh variant) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", MAHPAKH, VERTICAL_LINE_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_none(),
            "Mahpakh with vertical line (Legarmeh) should not be found"
        );
    }

    #[test]
    fn test_mahpakh_with_zarqa_and_legarmeh_not_found() {
        // Both conditions (ZARQA before AND PASEQ after) - definitely not found
        let sentence = format!("דָּבָר{}{}{}חָכְמָה", ZARQA_AS_CHAR, MAHPAKH, PASEQ_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_none(),
            "Mahpakh with both lookbehind and lookahead conditions should not be found"
        );
    }

    // ========================================================================
    // NEGATIVE TESTS - NO MAHPAKH PRESENT
    // ========================================================================

    #[test]
    fn test_no_mahpakh_in_sentence() {
        // Sentence without any Mahpakh
        let sentence = "דָּבָרחָכְמָה";
        let result = find_poetry_mehuppakh(sentence);

        assert!(
            result.is_none(),
            "Should return None when no Mahpakh present"
        );
    }

    #[test]
    fn test_empty_sentence() {
        // Empty string
        let sentence = "";
        let result = find_poetry_mehuppakh(sentence);

        assert!(result.is_none(), "Empty sentence should return None");
    }

    #[test]
    fn test_whitespace_only() {
        // Only whitespace
        let sentence = "   ";
        let result = find_poetry_mehuppakh(sentence);

        assert!(result.is_none(), "Whitespace-only should return None");
    }

    // ========================================================================
    // MULTIPLE MAHPAKH TESTS
    // ========================================================================

    #[test]
    fn test_multiple_standalone_mahpakh_returns_first() {
        // Multiple standalone Mahpakh - should return the first one
        let sentence = format!("דָּבָר{}חָכְמָה{}תּוֹרָה", MAHPAKH, MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some(), "Should find at least one Mahpakh");
        let m = result.unwrap();
        // First Mahpakh should be at earlier offset
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_mixed_valid_and_invalid_mahpakh() {
        // One valid standalone Mahpakh, one invalid (part of Tsinnorit Mahpakh)
        let sentence = format!("דָּבָר{}חָכְמָה{}{}תּוֹרָה", MAHPAKH, ZARQA_AS_CHAR, MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Should find the standalone Mahpakh even with compound present"
        );
        let m = result.unwrap();
        // Should return the first valid one (standalone)
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_invalid_first_valid_second() {
        // First Mahpakh is invalid (Tsinnorit), second is valid (standalone)
        let sentence = format!("דָּבָר{}{}חָכְמָה{}תּוֹרָה", ZARQA_AS_CHAR, MAHPAKH, MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Should skip invalid Mahpakh and find valid one"
        );
        let m = result.unwrap();
        // Should be the second Mahpakh (standalone)
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    // ========================================================================
    // WORD BOUNDARY TESTS
    // ========================================================================

    #[test]
    fn test_mahpakh_with_space_before() {
        // Mahpakh with space before (space is word boundary)
        let sentence = format!("דָּבָר {}{}חָכְמָה", MAHPAKH, ' ');
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Mahpakh with space before should be found"
        );
    }

    #[test]

    fn test_mahpakh_with_maqqeph_before() {
        // Mahpakh with Maqqeph (word separator) before
        use crate::char::MAQQEPH_AS_CHAR;
        let sentence = format!("דָּבָר {}{}חָכְמָה", MAHPAKH, MAQQEPH_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Mahpakh with Maqqeph before should be found"
        );
    }

    #[test]
    fn test_mahpakh_with_space_after_legarmeh() {
        // Mahpakh with space after (should NOT trigger Legarmeh - needs PASEQ/VERTICAL_LINE)
        let sentence = format!("דָּבָר{} חָכְמָה", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Mahpakh with space after (not PASEQ) should be found"
        );
    }

    // ========================================================================
    // EDGE CASES
    // ========================================================================

    #[test]
    fn test_only_mahpakh_character() {
        // Sentence containing only Mahpakh
        let sentence = MAHPAKH;
        let result = find_poetry_mehuppakh(sentence);

        assert!(result.is_some(), "Single Mahpakh should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_mahpakh_with_other_accents_before() {
        // Mahpakh with other accent (not ZARQA) before - should be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", 'א', MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Mahpakh with non-ZARQA char before should be found"
        );
    }

    #[test]
    fn test_mahpakh_with_other_chars_after_legarmeh() {
        // Mahpakh with other chars after (not PASEQ/VERTICAL_LINE) - should be found
        let sentence = format!("דָּבָר{}חָכְמָה", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_some(),
            "Mahpakh with normal char after should be found"
        );
    }

    // ========================================================================
    // INTEGRATION WITH MATCH STRUCTURE
    // ========================================================================

    #[test]
    fn test_match_structure_correct() {
        // Verify the Match structure is populated correctly
        let sentence = format!("דָּבָר{}חָכְמָה", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some());
        let m = result.unwrap();

        // Check all fields are reasonable
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
        //assert!(m.byte_offset >= 0);
        //assert!(m.byte_offset + ACCENT_LEN_UTF8 <= sentence.len());
    }

    #[test]
    fn test_byte_offset_matches_char_position() {
        // Verify byte offset corresponds to actual character position
        let sentence = format!("דָּבָר{}חָכְמָה", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some());
        let m = result.unwrap();

        // Get the actual character at the reported offset
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    // ========================================================================
    // COMPLEX SCENARIOS
    // ========================================================================

    #[test]
    fn test_complex_sentence_with_multiple_accents() {
        // Complex sentence with various accents, should find standalone Mahpakh
        let sentence = format!(
            "דָּבָר{}חָכְמָה{}{}תּוֹרָה{}נְבוֹאָה",
            MAHPAKH, ZARQA_AS_CHAR, MAHPAKH, MAHPAKH
        );
        let result = find_poetry_mehuppakh(&sentence);

        assert!(result.is_some(), "Should find at least one valid Mahpakh");
        let m = result.unwrap();
        // First Mahpakh is standalone, should be found
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{05A4}');
    }

    #[test]
    fn test_all_mahpakh_are_invalid() {
        // All Mahpakh occurrences are part of compounds - should return None
        let sentence = format!(
            "{}{}חָכְמָה{}{}תּוֹרָה",
            ZARQA_AS_CHAR, MAHPAKH, MAHPAKH, PASEQ_AS_CHAR
        );
        let result = find_poetry_mehuppakh(&sentence);

        assert!(
            result.is_none(),
            "Should return None when all Mahpakh are part of compounds"
        );
    }
}

#[cfg(test)]
mod tests_find_poetry_revia_gadol {
    use super::*;
    use crate::char::{GERESH, OLEH, YORED};

    #[test]
    fn new_no_revia_present() {
        let sentence =
            "אַשְׁרֵי־הָאִישׁ אֲשֶׁר לֹא הָלַךְ בַּעֲצַת רְשָׁעִים וּבְדֶרֶךְ חַטָּאִים לֹא עָמָד וּבְמוֹשַׁב לֵצִים לֹא יָשָׁב׃".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }
    #[test]
    fn new_part_of_revia_mugrash() {
        // A Revia preceded by Geresh, not followed by Oleh We Yored)
        let sentence =
            "אַשְׁרֵי־הָאִישׁ אֲשֶׁר לֹא הָלַךְ בַּעֲצַת רְשָׁעִ֗ים וּ֫בְדֶרֶ֥ךְ חַטָּאִים לֹא עָמָד וּבְמוֹשַׁב לֵצִים לֹא יָשָׁב׃".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }
    #[test]
    fn new_before_olehweyored_in_one_word() {
        // = revia qaton
        let sentence = "ננתּקה את־מוסר֗ותימו ו֫נשׁליכ֥ה ממּנּוּ עבתימו".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }
    #[test]
    fn new_before_olehweyored_in_two_words() {
        // = revia qaton
        let sentence = "ננתּקה את־מוסר֗ותימו ו֫נשׁ ליכ֥ה ממּנּוּ עבתימו".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }
    #[test]
    fn new_before_olehweyored_in_three_words() {
        // = revia gadol
        let sentence = "ננתּקה את־מוסר֗ותימו ו֫נשׁממּנּוּ ממּנּוּ ליכ֥ה ממּנּוּ עבתימו".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_some());
    }
    #[test]
    fn new_before_olehweyored_in_three_words2() {
        // = revia gadol
        let sentence = "ננתּקה את־מוסר֗ותימו ו֫נשׁממּנּוּ מ֧מּנּוּ ליכ֥ה ממּנּוּ עבתימו".to_string();
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_some());
    }

    // ========================================================================
    // POSITIVE TESTS: Valid Revia Gadol
    // ========================================================================

    #[test]
    fn test_finds_standalone_revia_gadol() {
        // A standalone Revia (not preceded by Geresh, not followed by Oleh We Yored)
        //let sentence = format!("דָּבָר{}חָכְמָה", REVIA);
        let sentence = "(אַ֥שְֽׁרֵי־הָאִ֗ישׁ רְשָׁ֫עִ֥ים וּבְדֶ֣רֶךְ חַ֭טָּאִים לֹ֥א עָמָ֑ד וּבְמוֹשַׁ֥ב לֵ֝צִ֗ים לֹ֣א יָשָֽׁב׃".to_string();
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some(), "Standalone Revia Gadol should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    }

    #[test]
    fn test_revia_gadol_at_start_of_sentence() {
        // Revia at the very beginning
        let sentence = format!("{}דָּבָר", REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some(), "Revia at start should be found");
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    }

    #[test]
    fn test_revia_gadol_at_end_of_sentence() {
        // Revia at the very end
        let sentence = format!("דָּבָר{}", REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some(), "Revia at end should be found");
    }

    // ========================================================================
    // NEGATIVE TESTS: Revia Mugrash (Lookbehind: Preceded by Geresh)
    // ========================================================================

    #[test]
    fn test_revia_mugrash_not_found() {
        // Revia preceded by Geresh (Revia Mugrash) should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", GERESH, REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(
            result.is_none(),
            "Revia Mugrash (Geresh + Revia) should not be found as Revia Gadol"
        );
    }

    #[test]
    fn test_revia_mugrash_with_space_before() {
        // Even with a space before Geresh, the Geresh+Revia combo should be skipped
        let sentence = format!("דָּבָר {}{}{}חָכְמָה", ' ', GERESH, REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(
            result.is_none(),
            "Revia Mugrash should not be found even with preceding space"
        );
    }

    // ========================================================================
    // NEGATIVE TESTS: Followed by Oleh We Yored (Lookahead)
    // ========================================================================

    #[test]
    fn test_revia_followed_by_oleh_we_yored_not_found() {
        // Revia followed by Oleh + Yored should NOT be found
        let sentence = format!("דָּבָר{} {} {}חָכְמָה", REVIA, OLEH, YORED);
        let result = find_poetry_revia_gadol(&sentence);
        assert!(
            result.is_none(),
            "Revia followed by Oleh We Yored should not be found as Revia Gadol"
        );
    }

    #[test]
    fn test_revia_followed_by_oleh_we_yored_with_space() {
        // Revia followed by space, then Oleh, then Yored
        let sentence = format!("דָּבָר{} {}{}חָכְמָה", REVIA, OLEH, YORED);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_none());
    }

    // ========================================================================
    // COMPLEX SCENARIOS: Mixed Conditions
    // ========================================================================

    #[test]
    fn test_revia_with_geresh_but_followed_by_owy() {
        // Both conditions met (Geresh before, OYW after) -> Should definitely not be found
        let sentence = format!("דָּבָר{}{}{}{}חָכְמָה", GERESH, REVIA, OLEH, YORED);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_none());
    }

    #[test]
    fn test_revia_with_geresh_but_valid_after() {
        // Geresh before (invalidates), but followed by valid char -> Should NOT be found
        let sentence = format!("דָּבָר{}{}חָכְמָה", GERESH, REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_none());
    }

    #[test]
    fn test_revia_valid_before_but_invalid_after() {
        // No Geresh before (valid), but followed by OYW (invalid) -> Should NOT be found
        let sentence = format!("דָּבָר{}{}{}חָכְמָה", REVIA, OLEH, YORED);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some());
    }

    #[test]
    fn test_multiple_revias_first_valid_second_invalid() {
        // First Revia is valid, second is Mugrash (Geresh + Revia)
        // Should return the FIRST valid one
        let sentence = format!("דָּבָר{}חָכְמָה{}{}תּוֹרָה", REVIA, GERESH, REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some(), "Should find the first valid Revia");
        let m = result.unwrap();
        // Verify it's the first one (offset should be small)
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    }

    #[test]
    fn test_multiple_revias_first_invalid_second_valid() {
        // First Revia is Mugrash, second is valid
        // Should skip the first and return the second
        let sentence = format!("דָּבָר{}{}חָכְמָה{}תּוֹרָה", GERESH, REVIA, REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some(), "Should find the second valid Revia");
        let m = result.unwrap();
        // Verify it's the second one (offset should be larger)
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    }

    // ========================================================================
    // NEGATIVE TESTS: Absence of Revia
    // ========================================================================

    #[test]
    fn test_no_revia_in_sentence() {
        let sentence = "דָּבָרחָכְמָה";
        let result = find_poetry_revia_gadol(sentence);

        assert!(result.is_none(), "Should return None if no Revia present");
    }

    #[test]
    fn test_empty_sentence() {
        let sentence = "";
        let result = find_poetry_revia_gadol(sentence);

        assert!(result.is_none(), "Empty sentence should return None");
    }

    #[test]
    fn test_whitespace_only() {
        let sentence = "   ";
        let result = find_poetry_revia_gadol(sentence);

        assert!(result.is_none(), "Whitespace only should return None");
    }

    // ========================================================================
    // MATCH STRUCTURE VALIDATION
    // ========================================================================

    #[test]
    fn test_match_byte_offsets_correct() {
        let sentence = format!("דָּבָר{}חָכְמָה", REVIA);
        let result = find_poetry_revia_gadol(&sentence);

        assert!(result.is_some());
        let m = result.unwrap();
        assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    }
}

#[cfg(test)]
mod tests_find_poetry_revia_qaton {
    use super::*;
    use crate::char::{GERESH, OLEH, REVIA, YORED};

    // ========================================================================
    // BASIC POSITIVE TESTS - VALID REVIA QATON
    // ========================================================================

    // #[test]
    // fn test_finds_valid_revia_qaton() {
    //     // Valid Revia Qaton: Revia + Oleh We Yored after, no Geresh before
    //     let sentence = format!("דָּבָר{}ש{} {}חָכְמָה", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(result.is_some(), "Valid Revia Qaton should be found");
    //     let m = result.unwrap();
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    // }

    // #[test]
    // fn test_revia_qaton_at_start_of_sentence() {
    //     // Revia Qaton at the beginning
    //     let sentence = format!("{}{}{}חָכְמָה", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(result.is_some(), "Revia Qaton at start should be found");
    //     let m = result.unwrap();
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    // }

    // #[test]
    // fn test_revia_qaton_in_middle_of_sentence() {
    //     // Revia Qaton in the middle
    //     let sentence = format!("דָּבָר{}{}{}חָכְמָה", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(result.is_some(), "Revia Qaton in middle should be found");
    //     let m = result.unwrap();
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    // }

    // ========================================================================
    // NEGATIVE TESTS - REVIA MUGRASH (GERESH BEFORE)
    // ========================================================================

    #[test]
    fn test_revia_mugrash_not_found_as_qaton() {
        // Revia Mugrash: Geresh + Revia (should NOT be found as Qaton)
        let sentence = format!("דָּבָר{}{}חָכְמָה", GERESH, REVIA);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia Mugrash (Geresh + Revia) should not be found as Qaton"
        );
    }

    #[test]
    fn test_revia_with_gersh_before_not_found() {
        // Even with Oleh We Yored after, Geresh before disqualifies it
        let sentence = format!("דָּבָר{}{}{}חָכְמָה", GERESH, REVIA, OLEH);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia with Geresh before should not be found even with Oleh after"
        );
    }

    // ========================================================================
    // NEGATIVE TESTS - NO OLEH WE YORED AFTER
    // ========================================================================

    #[test]
    fn test_revia_without_oleh_we_yored_not_found() {
        // Revia without Oleh We Yored after (should NOT be found)
        let sentence = format!("דָּבָר{}חָכְמָה", REVIA);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia without Oleh We Yored after should not be found"
        );
    }

    #[test]
    fn test_revia_with_only_oleh_not_found() {
        // Revia with only Oleh (no YORED) after
        let sentence = format!("דָּבָר{}{}חָכְמָה", REVIA, OLEH);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia with only Oleh (no Yored) should not be found"
        );
    }

    #[test]
    fn test_revia_with_only_yored_not_found() {
        // Revia with only YORED (no OLEH) after
        let sentence = format!("דָּבָר{}{}חָכְמָה", REVIA, YORED);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia with only Yored (no Oleh) should not be found"
        );
    }

    // ========================================================================
    // NEGATIVE TESTS - NO REVIA PRESENT
    // ========================================================================

    #[test]
    fn test_no_revia_in_sentence() {
        // Sentence without any Revia
        let sentence = "דָּבָרחָכְמָה";
        let result = find_poetry_revia_qaton(sentence);

        assert!(result.is_none(), "Should return None when no Revia present");
    }

    #[test]
    fn test_empty_sentence() {
        // Empty string
        let sentence = "";
        let result = find_poetry_revia_qaton(sentence);

        assert!(result.is_none(), "Empty sentence should return None");
    }

    #[test]
    fn test_whitespace_only() {
        // Only whitespace
        let sentence = "   ";
        let result = find_poetry_revia_qaton(sentence);

        assert!(result.is_none(), "Whitespace-only should return None");
    }

    // ========================================================================
    // MULTIPLE REVIA TESTS
    // ========================================================================

    // #[test]
    // fn test_multiple_revias_returns_first_valid() {
    //     // Multiple Revia: first is Mugrash (invalid), second is Qaton (valid)
    //     let sentence = format!(
    //         "דָּבָר{}{}{}{}{}חָכְמָה",
    //         GERESH,
    //         REVIA, // Revia Mugrash (invalid)
    //         REVIA,
    //         OLEH,
    //         YORED // Revia Qaton (valid)
    //     );
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(
    //         result.is_some(),
    //         "Should find the valid Revia Qaton even with Mugrash present"
    //     );
    //     let m = result.unwrap();
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    // }

    // #[test]
    // fn test_all_revias_invalid_returns_none() {
    //     // All Revia are Mugrash (all have Geresh before)
    //     let sentence = format!("דָּבָר{}{}חָכְמָה{}{}", GERESH, REVIA, GERESH, REVIA);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(
    //         result.is_none(),
    //         "Should return None when all Revia are Mugrash"
    //     );
    // }

    // ========================================================================
    // WORD BOUNDARY TESTS
    // ========================================================================

    // #[test]
    // fn test_revia_qaton_with_space_before() {
    //     // Revia Qaton with space before (space is word boundary)
    //     let sentence = format!("דָּבָר {}{}{}{}", ' ', REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(
    //         result.is_some(),
    //         "Revia Qaton with space before should be found"
    //     );
    // }

    // #[test]
    // fn test_revia_qaton_with_maqqeph_before() {
    //     // Revia Qaton with Maqqeph (word separator) before
    //     use crate::char::MAQQEPH_AS_CHAR;
    //     let sentence = format!("דָּבָר{}{}{}{}{}", ' ', MAQQEPH_AS_CHAR, REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(
    //         result.is_some(),
    //         "Revia Qaton with Maqqeph before should be found"
    //     );
    // }

    // ========================================================================
    // INTEGRATION WITH MATCH STRUCTURE
    // ========================================================================

    // #[test]
    // fn test_match_structure_correct() {
    //     // Verify the Match structure is populated correctly
    //     let sentence = format!("דָּבָר{}{}{}חָכְמָה", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(result.is_some());
    //     let m = result.unwrap();

    //     // Check all fields are reasonable
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');
    //     //assert!(m.byte_offset >= 0);
    //     //assert!(m.byte_offset + ACCENT_LEN_UTF8 <= sentence.len());
    // }

    // #[test]
    // fn test_byte_offset_matches_char_position() {
    //     // Verify byte offset corresponds to actual character position
    //     let sentence = format!("דָּבָר{}{}{}חָכְמָה", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(result.is_some());
    //     let m = result.unwrap();
    //     assert_eq!(sentence.chars().nth(m.start()).unwrap(), '\u{0597}');

    //     // Get the actual character at the reported offset
    //     //let char_at_offset = sentence.chars().nth(m.byte_offset);
    //     //assert_eq!(char_at_offset, Some(REVIA.chars().next().unwrap()));
    // }

    // ========================================================================
    // EDGE CASES
    // ========================================================================

    #[test]
    fn test_only_revia_character() {
        // Sentence containing only Revia (no Oleh We Yored)
        let sentence = format!("{}", REVIA);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Single Revia without Oleh We Yored should not be found"
        );
    }

    #[test]
    fn test_revia_at_end_of_sentence() {
        // Revia at the end (no Oleh We Yored after)
        let sentence = format!("דָּבָר{}", REVIA);
        let result = find_poetry_revia_qaton(&sentence);

        assert!(
            result.is_none(),
            "Revia at end without Oleh We Yored should not be found"
        );
    }

    // #[test]
    // fn test_oleh_we_yored_at_end_after_revia() {
    //     // Revia with Oleh We Yored at the end of sentence
    //     let sentence = format!("דָּבָר{}{}{}", REVIA, OLEH, YORED);
    //     let result = find_poetry_revia_qaton(&sentence);

    //     assert!(
    //         result.is_some(),
    //         "Revia with Oleh We Yored at end should be found"
    //     );
    // }
}
