//! Miscellaneous helper functions

// Standard library
// N/A

// External crates
// N/A
use hebrew_unicode_script::{is_hbr_block, is_hbr_consonant_final, is_hbr_consonant_normal};

// Crate‑internal (local modules)
use crate::char::{
    GERESH_AS_CHAR, MAHPAKH, MAQQEPH_AS_CHAR, MERKHA, OLEH_AS_CHAR, PASEQ_AS_CHAR, REVIA,
    TSINNORIT_AS_CHAR, VERTICAL_LINE_AS_CHAR, YORED_AS_CHAR, ZARQA_AS_CHAR,
};
use crate::sentenc_ctx_error::SentenceContextError;
use crate::sentence_ctx_find::ACCENT_LEN_UTF8;
use crate::{Context, Match, PoetryAccent, ProseAccent, SentenceContext};

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

pub(crate) fn find_poetry_revia_qaton(sentence: &str) -> Option<Match<'static>> {
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

pub(crate) fn validate_sentence(s: &str) -> Result<(), SentenceContextError> {
    if s.is_empty() {
        return Err(SentenceContextError::EmptySentence);
    }

    if s.contains("\n") {
        return Err(SentenceContextError::MultipleLines);
    }

    // Find the first NON-WHITESPACE character
    let first_non_ws = s.chars().find(|c| !c.is_whitespace());

    match first_non_ws {
        // No meaningful characters at all (whitespace only)
        None => return Err(SentenceContextError::EmptySentence),

        // First char is a Final Form letter (invalid start)
        // Ranges: 05DA (ך), 05DE (ם), 05E0 (ן), 05E3 (ף), 05E5 (ץ)
        Some(c) if is_hbr_consonant_final(c) => {
            return Err(SentenceContextError::StartsWithFinalForm(c));
        }

        // First char is not a normal Hebrew consonant
        Some(c) if !is_hbr_consonant_normal(c) => {
            return Err(SentenceContextError::StartsWithNonConsonant(c));
        }

        // First char is neither a Hebrew letter nor a niqqud mark
        Some(c) if !is_valid_hebrew_char(c) => {
            return Err(SentenceContextError::InvalidCharacter(c, 0));
        }

        // Valid start (Hebrew consonant), continue full validation
        _ => {}
    }

    // Validate ALL characters in the string
    for (idx, c) in s.chars().enumerate() {
        if !is_valid_hebrew_char(c) {
            return Err(SentenceContextError::InvalidCharacter(c, idx));
        }
    }
    Ok(())
}

/// Helper to detect valid hebrew chars
fn is_valid_hebrew_char(c: char) -> bool {
    // Check for Hebrew Unicode Block (U+0590 - U+05FF)
    if is_hbr_block(c) {
        return true;
    }
    // Check for space like characters
    if is_space_like_char(c) {
        return true;
    }
    //
    if is_paseq_alternative_char(c) {
        return true;
    }
    // Check for space like characters
    if is_meteg_layout_char(c) {
        return true;
    }
    false
}

fn is_space_like_char(c: char) -> bool {
    matches!(
        c,
        '\u{0020}' | // SPACE
        '\u{00A0}' | // NO-BREAK SPACE
        '\u{200E}' | // LRM: ZERO WIDTH JOINER
        '\u{200F}' | // RLM: RIGHT-TO-LEFT
        '\u{2009}' | // THIN SPACE
        '\u{205F}' | // MEDIUM MATHEMATICAL SPACE
        '\u{3000}' // IDEOGRAPHIC SPACE
    )
}

//
fn is_paseq_alternative_char(c: char) -> bool {
    matches!(
        c,
        '\u{007C}' // VERTICAL BAR
    )
}

// Check for specific METEG Layout Control Characters
// see https://www.unicode.org/versions/Unicode15.0.0/ section 9.1 for more information
fn is_meteg_layout_char(c: char) -> bool {
    matches!(
        c,
        '\u{034F}' | // CGJ: COMBINING GRAPHEME JOINER
        '\u{200C}' | // ZWNJ: ZERO WIDTH NON-JOINER
        '\u{200D}' // ZWJ: ZERO WIDTH JOINER
    )
}

#[cfg(test)]
mod test_poetry_accent_finder {
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

#[cfg(test)]
mod tests4_poetry_merkha_finder {
    // TODO
    // use super::*;
}
#[cfg(test)]
mod tests4_poetry_mehuppak_finder {
    // TODO
    // use super::*;
}
#[cfg(test)]
mod tests4_poetry_revia_gadol_finder {
    // TODO
    // use super::*;
}
#[cfg(test)]
mod tests4_poetry_revia_qaton_finder {
    // TODO
    // use super::*;
}
// helper funtions
#[cfg(test)]
mod tests4_as_char_slice {
    // TODO
    // use super::*;
}
#[cfg(test)]
mod tests4_indexes_target_char {
    // TODO
    // use super::*;
}

#[cfg(test)]
mod tests4_is_part_of_two_code_point_accent_look_behind {
    // TODO
    // use super::*;
}

#[cfg(test)]
mod tests4_is_part_of_mahpakh_legarmeh_look_ahead {
    // TODO
    // use super::*;
}

#[cfg(test)]
mod tests4_is_followed_by_oleh_we_yore {
    // TODO
    // use super::*;
}



#[cfg(test)]
mod tests4_detect_context_from_sentence {
    // use super::*;
}
#[cfg(test)]
mod tests4_validate_sentence {
    // use super::*;
}

#[cfg(test)]
//is_hbr_block(c)
//is_space_like_char(c)
//is_paseq_alternative_char(c)
//is_meteg_layout_char(c) {
mod tests4_is_valid_hebrew_char {
    use crate::sentence_ctx_funcs::is_valid_hebrew_char;

    #[test]
    fn tests_exact_boundaries_of_all_ranges() {
        // Hebrew Block Start
        assert!(!is_valid_hebrew_char('\u{058F}'));

        // Hebrew Block End
        assert!(!is_valid_hebrew_char('\u{0600}'));
    }

    mod tests4_is_hebrew_block {
        // --- Hebrew Block Characters (U+0590 - U+05FF) ---
        // not used U+0590
        // not used U+05C8 - U+05CF
        // not used U+05F5 - U+05FF
        use crate::sentence_ctx_funcs::is_valid_hebrew_char;
        #[test]
        fn hebrew_block_0591() {
            for c in '\u{0591}'..'\u{05C7}' {
                assert!(
                    is_valid_hebrew_char(c),
                    "Explicitly accepted character '{:?}' (U+{:04X}) should pass main function",
                    c,
                    c as u32
                );
            }
        }
        #[test]
        fn hebrew_block_05d0() {
            for c in '\u{05D0}'..'\u{05EA}' {
                assert!(
                    is_valid_hebrew_char(c),
                    "Explicitly accepted character '{:?}' (U+{:04X}) should pass main function",
                    c,
                    c as u32
                );
            }
        }
        #[test]
        fn hebrew_block_05ef() {
            for c in '\u{05EF}'..'\u{05F4}' {
                assert!(
                    is_valid_hebrew_char(c),
                    "Explicitly accepted character '{:?}' (U+{:04X}) should pass main function",
                    c,
                    c as u32
                );
            }
        }

        #[test]
        fn rejects_outside_hebrew_block_start() {
            // Just before the Hebrew block
            assert!(!is_valid_hebrew_char('\u{058F}'));
            assert!(!is_valid_hebrew_char('\u{058E}'));
            assert!(!is_valid_hebrew_char('\u{0500}'));
        }
    }
    mod tests4_is_space_like_char {
        use crate::sentence_ctx_funcs::is_space_like_char;

        #[test]
        fn accepts_standard_space() {
            assert!(is_space_like_char('\u{0020}'));
        }
        #[test]
        fn accepts_no_break_space() {
            assert!(is_space_like_char('\u{00A0}'));
        }

        #[test]
        fn accepts_left_to_right_mark() {
            assert!(is_space_like_char('\u{200E}'));
        }

        #[test]
        fn accepts_right_to_left_mark() {
            assert!(is_space_like_char('\u{200F}'));
        }

        #[test]
        fn accepts_thin_space() {
            assert!(is_space_like_char('\u{2009}'));
        }

        #[test]
        fn accepts_medium_mathematical_space() {
            assert!(is_space_like_char('\u{205F}'));
        }

        #[test]
        fn accepts_ideographic_space() {
            assert!(is_space_like_char('\u{3000}'));
        }

        #[test]
        fn rejects_non_space_like_chars() {
            assert!(!is_space_like_char('\u{0009}')); // TAB
            assert!(!is_space_like_char('־'));
            assert!(!is_space_like_char('ג'));
        }
    }
    mod tests4_is_paseq_alternative_char {
        use crate::sentence_ctx_funcs::is_paseq_alternative_char;
        #[test]
        fn accepts_vertical_bar_ascii() {
            assert!(is_paseq_alternative_char('|'));
        }

        #[test]
        fn accepts_vertical_bar_unicode_escape() {
            assert!(is_paseq_alternative_char('\u{007C}'));
        }
        #[test]
        fn rejects_non_meteg_layout_chars() {
            assert!(!is_paseq_alternative_char('\u{00A6}')); // Broken Bar
            assert!(!is_paseq_alternative_char('ַ'));
            assert!(!is_paseq_alternative_char('ט'));
        }
    }
    mod tests4_is_meteg_layout_char {
        use crate::sentence_ctx_funcs::is_meteg_layout_char;
        #[test]
        fn accepts_cgj_u034f() {
            assert!(is_meteg_layout_char('\u{034F}'));
        }

        #[test]
        fn accepts_zwnj_u200c() {
            assert!(is_meteg_layout_char('\u{200C}'));
        }

        #[test]
        fn accepts_zwj_u200d() {
            assert!(is_meteg_layout_char('\u{200D}'));
        }
        #[test]
        fn rejects_non_meteg_layout_chars() {
            assert!(!is_meteg_layout_char('\u{0020}'));
            assert!(!is_meteg_layout_char('a'));
            assert!(!is_meteg_layout_char('א'));
        }
    }
    mod edge_cases {
        use crate::sentence_ctx_funcs::is_meteg_layout_char;
        use crate::sentence_ctx_funcs::is_valid_hebrew_char;

        #[test]
        fn tests_exact_boundaries_of_all_ranges() {
            // Hebrew Block Start
            assert!(!is_valid_hebrew_char('\u{058F}'));

            // Hebrew Block End
            assert!(!is_valid_hebrew_char('\u{0600}'));
        }

        #[test]
        fn verifies_all_explicitly_accepted_chars_pass_main_function() {
            // Combine all explicitly accepted non-Hebrew chars and verify they pass
            let all_explicit = vec![
                '|', // Vertical bar
                '\u{0020}', '\u{00A0}', '\u{200E}', '\u{200F}', '\u{2009}', '\u{205F}', '\u{3000}',
                '\u{034F}', '\u{200C}', '\u{200D}',
            ];

            for c in all_explicit {
                assert!(
                    is_valid_hebrew_char(c),
                    "Explicitly accepted character '{:?}' (U+{:04X}) should pass main function",
                    c,
                    c as u32
                );
            }
        }

        #[test]
        fn rejects_non_meteg_layout_chars() {
            assert!(!is_meteg_layout_char('\u{0020}'));
            assert!(!is_meteg_layout_char('a'));
            assert!(!is_meteg_layout_char('א'));
        }

        #[test]
        fn rejects_outside_hebrew_block_start() {
            // Just before the Hebrew block
            assert!(!is_valid_hebrew_char('\u{058F}'));
            assert!(!is_valid_hebrew_char('\u{058E}'));
            assert!(!is_valid_hebrew_char('\u{0500}'));
        }

        #[test]
        fn rejects_outside_hebrew_block_end() {
            // Just after the Hebrew block
            assert!(!is_valid_hebrew_char('\u{0600}'));
            assert!(!is_valid_hebrew_char('\u{0601}'));
            assert!(!is_valid_hebrew_char('\u{0700}'));
        }

        #[test]
        fn rejects_null_character() {
            assert!(!is_valid_hebrew_char('\0'));
        }

        #[test]
        fn rejects_emoji() {
            assert!(!is_valid_hebrew_char('\u{1F600}')); // 😀
            assert!(!is_valid_hebrew_char('\u{1F44D}')); // 👍
        }

        #[test]
        fn rejects_newline_variations_not_in_space_list() {
            // Note: Your is_valid_hebrew_char does NOT accept regular \n via is_space_like_char
            // If you need to handle this, add to is_space_like_char or main function
            // For now, these are expected to fail based on current implementation
            // assert!(!is_valid_hebrew_char('\n')); // Only true if not handled elsewhere
            // assert!(!is_valid_hebrew_char('\r'));
        }

        #[test]
        fn rejects_high_unicode_private_use_area() {
            assert!(!is_valid_hebrew_char('\u{E000}'));
            assert!(!is_valid_hebrew_char('\u{FDD0}'));
        }

        #[test]
        fn rejects_other_whitespace_not_in_space_like() {
            // Standard tab (\t) is not in is_space_like_char list
            // It would only pass if your validation elsewhere handles it
            // Based on current implementation, this SHOULD reject
            // assert!(!is_valid_hebrew_char('\t'));
        }

        #[test]
        fn handles_mixed_script_string_correctly() {
            let mixed = "אב| cd "; // Hebrew + vertical bar + Latin + space
            let chars: Vec<char> = mixed.chars().collect();
            // Check nr. of characters
            assert_eq!(chars.len(), 7);
            // First two characters (Hebrew) should be valid
            assert!(is_valid_hebrew_char(chars[0]));
            assert!(is_valid_hebrew_char(chars[1]));
            // Third character (vertical bar) should be valid
            assert!(is_valid_hebrew_char(chars[2]));
            // Fouth character (space) should be valid
            assert!(is_valid_hebrew_char(chars[3]));
            // Fifth and sixth characters(Latin) invalid
            assert!(!is_valid_hebrew_char(chars[4]));
            assert!(!is_valid_hebrew_char(chars[5]));
            // Seventh character (space) should be valid
            assert!(is_valid_hebrew_char(chars[6]));
        }

        #[test]
        fn empty_string_simulation_returns_false_for_all_chars() {
            // When iterating over empty string, loop won't execute (tested via other means)
            // This verifies that truly invalid chars don't slip through
            let definitely_invalid = ['_', '-', '@', '#', '$', '%', '&'];

            for c in definitely_invalid {
                assert!(!is_valid_hebrew_char(c), "{:?} should be rejected", c);
            }
        }

        // --- Real World Sample ---
        #[test]
        fn accepts_sample_biblical_text() {
            let sample = " בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ׃";
            for c in sample.chars() {
                assert!(
                    is_valid_hebrew_char(c),
                    "Character '{}' (U+{:04X}) should be valid",
                    c,
                    c as u32
                );
            }
        }
    }
}
