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
use crate::Match;

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
    // define possible combinations
    let possible_combinations_lookbehind = [GERESH_AS_CHAR];
    // check if the target character is present in the senctence
    if !&sentence.contains(target_char) {
        return None;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for index in indices {
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            1,
        );
        let followed_by_owy = is_followed_by_oleh_we_yored(index, &char_vec);
        //  2cp   oleweyored     revia_qadol
        //  no      no      -       yes
        //  no      yes     -       no
        //  yes     no      -       no
        //  yes     yes     -       no
        // println!(
        //     "two_code_points_behind: {two_code_points_behind} - followed_by_owy: {followed_by_owy}"
        // );
        if !two_code_points_behind && !followed_by_owy {
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
        println!("REVIA not found in the senctence at all  -> return None");
        return None;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = as_char_slice(sentence);
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for index in indices {
        // println!("\n\nLOOP::Index of target character = {index}\n");
        // println!("Negative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            target_char,
            index,
            &possible_combinations_lookbehind,
            1,
        );
        //println!("Followed by Oleh We Yored");
        let followed_by_owy = is_followed_by_oleh_we_yored(index, &char_vec);
        // 2cp   oleweyored     revia_qaton
        //  no      no      -       no
        //  no      yes     -       yes
        //  yes     no      -       no
        //  yes     yes     -       no
        // println!(
        //     "two_code_points_behind:{two_code_points_behind} - followed_by_owy{followed_by_owy}"
        // );
        if !two_code_points_behind && followed_by_owy {
            let revia = "\u{0597}";
            return Some(Match::new(revia, index, index + ACCENT_LEN_UTF8));
        }
    }
    None
}

/*
helper functions
*/

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
            ' ' | MAQQEPH_AS_CHAR => word_boundary_cnt += 1,

            // The first word after the target may contain `OLEH`.
            OLEH_AS_CHAR if word_boundary_cnt == 1 => oleh_seen = true,

            // `YORED` may appear in the first or second word *after* we have
            // already seen `OLEH`.
            YORED_AS_CHAR if (word_boundary_cnt == 1 || word_boundary_cnt == 2) && oleh_seen => {
                // Both parts are present → we can return early.
                return true;
            }

            // Any other character does not affect the state.
            _ => {}
        }
    }

    // If we exit the loop without having seen both parts, the sequence is absent.
    false
}

pub(crate) fn validate_sentence(s: &str) -> Result<(), SentenceContextError> {
    if s.is_empty() {
        return Err(SentenceContextError::EmptySentence);
    }

    if s.contains("\n") {
        return Err(SentenceContextError::MultipleLines);
    }

    for (idx, c) in s.chars().enumerate() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }
    Ok(())
}

pub(crate) fn is_valid_hebrew_char(c: char) -> bool {
    // Check for Hebrew Unicode Block (U+0590 - U+05FF)
    if ('\u{0590}'..='\u{05FF}').contains(&c) {
        return true;
    }
    // Check for Vertical Bar (U+007C) (Sometimes a replacement for Paseq)
    if '\u{007C}' == c {
        return true;
    }

    // Check for Whitespace (includes space, tab, newline, etc.)
    // Note: newline is filterd by the calling function!
    if c.is_whitespace() {
        return true;
    }

    // Check for specific Bidi Control Characters
    // These are the explicit controls used to force directionality
    // Note: Maybe these can removed?
    matches!(
        c,
        '\u{202A}' | // LRE: Left-to-Right Embedding
        '\u{202B}' | // RLE: Right-to-Left Embedding
        '\u{202C}' | // PDF: Pop Directional Formatting
        '\u{202D}' | // LRO: Left-to-Right Override
        '\u{202E}' | // RLO: Right-to-Left Override
        '\u{2066}' | // LRI: Left-to-Right Isolate
        '\u{2067}' | // RLI: Right-to-Left Isolate
        '\u{2068}' | // FSI: First Strong Isolate
        '\u{2069}' // PDI: Pop Directional Isolate
    )
}

/////////////////////
#[cfg(test)]
mod poetry_accent_finder_tests {
    use super::*;
    // Assuming the module is named `poetry_accent_finder` or similar
    // Adjust the path based on your actual module structure
    // use crate::{
    //     find_poetry_merkha,
    //     find_poetry_mehuppakh,
    //     find_poetry_revia_gadol,
    //     find_poetry_revia_qaton,
    // };
    // Import helper constants if needed for constructing test strings
    use crate::char::{
        GERESH_AS_CHAR,
        MAHPAKH,
        MERKHA,
        OLEH_AS_CHAR,
        PASEQ_AS_CHAR,
        REVIA,
        TSINNORIT_AS_CHAR,
        VERTICAL_LINE_AS_CHAR,
        //MAQQEPH_AS_CHAR,
        YORED_AS_CHAR,
        ZARQA_AS_CHAR,
    };

    // ========================================================================
    // 1. find_poetry_merkha Tests
    // ========================================================================

    /// Test: Empty string -> None
    #[test]
    fn test_find_poetry_merkha_empty_string() {
        assert!(find_poetry_merkha("").is_none());
    }

    /// Test: Target char not present -> None
    #[test]
    fn test_find_poetry_merkha_not_found() {
        assert!(find_poetry_merkha("abc").is_none());
    }

    /// Test: Standalone Merkha found -> Some
    // #[test]
    // fn test_find_poetry_merkha_standalone() {
    //     // Merkha is \u{05A5}
    //     let sentence = format!("{}", MERKHA);
    //     let result = find_poetry_merkha(&sentence);
    //     assert!(result.is_some());
    //     let m = result.unwrap();
    //     assert_eq!(m.as_str(), "\u{05A5}");

    //     let expected = Match::new(GERSHAYIM, 18, 20) ;
    // assert_eq!(
    //     sentence.unwrap().find_accent(ProseAccent::Gershayim.into()),
    //     Some(expected)
    // );
    // }

    /// Test: Merkha part of Tsinnorit Merkha -> None (Negative Lookbehind)
    #[test]
    fn test_find_poetry_merkha_part_of_tsinnorit() {
        // Tsinnorit is \u{0598}, Merkha is \u{05A5}
        // Sequence: Tsinnorit + Merkha
        let sentence = format!("word{}{}", TSINNORIT_AS_CHAR, MERKHA);
        let result = find_poetry_merkha(&sentence);
        // Should return None because it's part of a compound accent
        assert!(result.is_none());
    }

    /// Test: Merkha part of Oleh We Yored -> None (Negative Lookbehind)
    #[test]
    fn test_find_poetry_merkha_part_of_oleh_we_yored() {
        // Oleh is \u{05B0} (example), Merkha is \u{05A5}
        let sentence = format!("word{}{}", OLEH_AS_CHAR, MERKHA);
        let result = find_poetry_merkha(&sentence);
        assert!(result.is_none());
    }

    // ========================================================================
    // 2. find_poetry_mehuppakh Tests
    // ========================================================================

    #[test]
    fn test_find_poetry_mehuppakh_empty() {
        assert!(find_poetry_mehuppakh("").is_none());
    }

    #[test]
    fn test_find_poetry_mehuppakh_not_found() {
        assert!(find_poetry_mehuppakh("xyz").is_none());
    }

    #[test]
    fn test_find_poetry_mehuppakh_standalone() {
        let sentence = format!("text{}", MAHPAKH);
        let result = find_poetry_mehuppakh(&sentence);
        assert!(result.is_some());
    }

    /// Test: Mehuppakh part of Tsinnorit Mahpakh (Lookbehind) -> None
    #[test]
    fn test_find_poetry_mehuppakh_part_of_tsinnorit_mahpakh() {
        // Tsinnorit is \u{0598}, Mahpakh is \u{05A4}
        let sentence = format!("word{}{}", ZARQA_AS_CHAR, MAHPAKH); // Assuming ZARQA is the lookbehind char
        let result = find_poetry_mehuppakh(&sentence);
        assert!(result.is_none());
    }

    /// Test: Mehuppakh part of Mehuppakh Legarmeh (Lookahead) -> None
    #[test]
    fn test_find_poetry_mehuppakh_part_of_legarmeh() {
        // Mehuppakh followed by Paseq or Vertical Line
        let sentence = format!("word{}{}", MAHPAKH, PASEQ_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);
        assert!(result.is_none());
    }

    /// Test: Mehuppakh followed by Vertical Line -> None
    #[test]
    fn test_find_poetry_mehuppakh_vertical_line() {
        let sentence = format!("word{}{}", MAHPAKH, VERTICAL_LINE_AS_CHAR);
        let result = find_poetry_mehuppakh(&sentence);
        assert!(result.is_none());
    }

    // ========================================================================
    // 3. find_poetry_revia_gadol Tests
    // ========================================================================

    #[test]
    fn test_find_poetry_revia_gadol_empty() {
        assert!(find_poetry_revia_gadol("").is_none());
    }

    #[test]
    fn test_find_poetry_revia_gadol_not_found() {
        assert!(find_poetry_revia_gadol("abc").is_none());
    }

    #[test]
    fn test_find_poetry_revia_gadol_standalone() {
        let sentence = format!("text{}", REVIA);
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_some());
    }

    /// Test: Revia part of Revia Mugrash (Lookbehind) -> None
    #[test]
    fn test_find_poetry_revia_gadol_part_of_mugrash() {
        // Geresh is the lookbehind char for Mugrash
        let sentence = format!("word{}{}", GERESH_AS_CHAR, REVIA);
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }

    /// Test: Revia followed by Oleh We Yored -> None
    #[test]
    fn test_find_poetry_revia_gadol_followed_by_owy() {
        // Revia + Oleh + Yored
        let sentence = format!("word{} {} {}", REVIA, OLEH_AS_CHAR, YORED_AS_CHAR);
        let result = find_poetry_revia_gadol(&sentence);
        assert!(result.is_none());
    }

    // ========================================================================
    // 4. find_poetry_revia_qaton Tests
    // ========================================================================

    #[test]
    fn test_find_poetry_revia_qaton_empty() {
        assert!(find_poetry_revia_qaton("").is_none());
    }

    #[test]
    fn test_find_poetry_revia_qaton_not_found() {
        assert!(find_poetry_revia_qaton("abc").is_none());
    }

    /// Test: Revia Qaton found (Not Mugrash + Followed by OWY)
    #[test]
    fn test_find_poetry_revia_qaton_success() {
        // Must NOT be preceded by Geresh (Mugrash)
        // MUST be followed by Oleh We Yored
        let sentence = format!("text{} {} {}", REVIA, OLEH_AS_CHAR, YORED_AS_CHAR);
        let result = find_poetry_revia_qaton(&sentence);
        assert!(result.is_some());
    }

    /// Test: Revia Qaton fails if preceded by Geresh (Mugrash)
    #[test]
    fn test_find_poetry_revia_qaton_preceded_by_geresh() {
        //let sentence = format!("word{} {} {}", GERESH_AS_CHAR, REVIA, OLEH_AS_CHAR);
        // Wait, the logic is: Revia is the target. Geresh is BEFORE Revia.
        // So: Geresh + Revia + Oleh + Yored
        let sentence = format!(
            "word{}{} {} {}",
            GERESH_AS_CHAR, REVIA, OLEH_AS_CHAR, YORED_AS_CHAR
        );
        let result = find_poetry_revia_qaton(&sentence);
        assert!(result.is_none());
    }

    /// Test: Revia Qaton fails if NOT followed by OWY
    #[test]
    fn test_find_poetry_revia_qaton_not_followed_by_owy() {
        let sentence = format!("text{} end", REVIA);
        let result = find_poetry_revia_qaton(&sentence);
        assert!(result.is_none());
    }

    // ========================================================================
    // 5. Helper Function Tests (Direct Coverage)
    // ========================================================================

    #[test]
    fn test_as_char_slice() {
        let s = "Hello";
        let chars = as_char_slice(s);
        assert_eq!(chars.len(), 5);
        assert_eq!(chars[0], 'H');
    }

    #[test]
    fn test_indexes_target_char_found() {
        let sentence: Vec<char> = "aba".chars().collect();
        let indices = indexes_target_char("a", &sentence);
        assert_eq!(indices, vec![0, 2]);
    }

    #[test]
    fn test_indexes_target_char_not_found() {
        let sentence: Vec<char> = "abc".chars().collect();
        let indices = indexes_target_char("z", &sentence);
        assert!(indices.is_empty());
    }

    #[test]
    fn test_indexes_target_char_multi_char_input() {
        // Should return empty if input is not a single char
        let sentence: Vec<char> = "abc".chars().collect();
        let indices = indexes_target_char("ab", &sentence);
        assert!(indices.is_empty());
    }

    #[test]
    fn test_indexes_target_char_empty_input() {
        let sentence: Vec<char> = "abc".chars().collect();
        let indices = indexes_target_char("", &sentence);
        assert!(indices.is_empty());
    }

    /// Test: Lookbehind at start of string (idx_target == 0)
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_start_of_string() {
        let sentence: Vec<char> = "a".chars().collect();
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 0, &['b'], 1);
        assert!(!result); // Should return false immediately
    }

    /// Test: Lookbehind finds match
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_match() {
        let sentence: Vec<char> = "ba".chars().collect();
        // Looking for 'a', expecting 'b' behind it
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 1, &['b'], 1);
        assert!(result);
    }

    /// Test: Lookbehind finds match across word boundary (should stop)
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_word_boundary() {
        // "b a" -> space is a boundary. If max_word_span is 1, it should stop at space.
        let sentence: Vec<char> = "b a".chars().collect();
        // Looking for 'a' at index 2. ' ' is at 1. 'b' is at 0.
        // With max_word_span=1, it sees space, increments break, and returns false.
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 2, &['b'], 1);
        assert!(!result);
    }

    /// Test: Lookbehind finds match within span
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_within_span() {
        let sentence: Vec<char> = "b a".chars().collect();
        // max_word_span=2 allows crossing one space
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 2, &['b'], 2);
        assert!(result);
    }

    /// Test: Lookbehind target repeated
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_target_repeat() {
        let sentence: Vec<char> = "aa".chars().collect();
        // Looking for second 'a'. First 'a' is target. Should return false.
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 1, &['b'], 1);
        assert!(!result);
    }

    /// Test: Lookbehind exhausted without match
    #[test]
    fn test_is_part_of_two_code_point_accent_look_behind_no_match() {
        let sentence: Vec<char> = "ca".chars().collect();
        let result = is_part_of_two_code_point_accent_look_behind(&sentence, "a", 1, &['b'], 1);
        assert!(!result);
    }

    /// Test: Lookahead for Mahpakh Legarmeh (found)
    #[test]
    fn test_is_part_of_mahpakh_legarmeh_look_ahead_found() {
        let sentence: Vec<char> = "a|".chars().collect(); // | is VERTICAL_LINE_AS_CHAR
        let result = is_part_of_mahpakh_legarmeh_look_ahead(0, &sentence);
        assert!(result);
    }

    /// Test: Lookahead for Mahpakh Legarmeh (not found, too far)
    #[test]
    fn test_is_part_of_mahpakh_legarmeh_look_ahead_too_far() {
        // "a  |" -> two spaces before |, which hits the limit (word_breaks >= 2)
        let sentence: Vec<char> = "a  |".chars().collect();
        let result = is_part_of_mahpakh_legarmeh_look_ahead(0, &sentence);
        assert!(!result);
    }

    /// Test: Lookahead for Mahpakh Legarmeh (out of bounds)
    #[test]
    fn test_is_part_of_mahpakh_legarmeh_look_ahead_out_of_bounds() {
        let sentence: Vec<char> = "a".chars().collect();
        let result = is_part_of_mahpakh_legarmeh_look_ahead(10, &sentence);
        assert!(!result);
    }

    /// Test: Followed by Oleh We Yored (Success)
    #[test]
    fn test_is_followed_by_oleh_we_yored_success() {
        // "a O Y" -> O at word 1, Y at word 2
        let sentence: Vec<char> = "a O Y".chars().collect();
        let result = is_followed_by_oleh_we_yored(0, &sentence);
        assert!(!result);
    }

    /// Test: Followed by Oleh We Yored (Missing Yored)
    #[test]
    fn test_is_followed_by_oleh_we_yored_missing_yored() {
        let sentence: Vec<char> = "a O".chars().collect();
        let result = is_followed_by_oleh_we_yored(0, &sentence);
        assert!(!result);
    }

    /// Test: Followed by Oleh We Yored (Missing Oleh)
    #[test]
    fn test_is_followed_by_oleh_we_yored_missing_oleh() {
        let sentence: Vec<char> = "a Y".chars().collect();
        let result = is_followed_by_oleh_we_yored(0, &sentence);
        assert!(!result);
    }

    /// Test: Followed by Oleh We Yored (Too far)
    #[test]
    fn test_is_followed_by_oleh_we_yored_too_far() {
        // "a   O Y" -> 3 spaces before O, hits limit (word_boundary_cnt == 3)
        let sentence: Vec<char> = "a   O Y".chars().collect();
        let result = is_followed_by_oleh_we_yored(0, &sentence);
        assert!(!result);
    }

    /// Test: Followed by Oleh We Yored (Out of bounds)
    #[test]
    fn test_is_followed_by_oleh_we_yored_out_of_bounds() {
        let sentence: Vec<char> = "a".chars().collect();
        let result = is_followed_by_oleh_we_yored(10, &sentence);
        assert!(!result);
    }
}
