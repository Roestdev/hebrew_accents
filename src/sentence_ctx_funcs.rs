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
