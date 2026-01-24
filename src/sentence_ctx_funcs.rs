// Standard library

// External crates

// Local modules / crate‑internal
use crate::char::{
    GERESH, MAHPAKH, MAQQEPH, MERKHA, OLEH, PASEQ, REVIA, TSINNORIT, VERTICAL_LINE, YORED, ZARQA,
};
use crate::Match;

pub(crate) fn find_poetry_merkha(sentence: &str) -> Option<Match<'static>> {
    // Merkha (as a poetry accent) is
    //   not part of Oleh We Yored (needs Negative Lookbehind)
    //   AND
    //   not part of Tsinnorit Merkha (needs Negative Lookbehind)
    let target_char = MERKHA;
    // define possible combinations
    let possible_combinations_lookbehind = [TSINNORIT, OLEH];

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
            let mercha = "\u{05A5}";
            return Some(Match::new(mercha, 333, 777));
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
    let possible_combinations_lookbehind = [ZARQA];
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
            let mahpah = "\u{05A4}";
            return Some(Match::new(mahpah, 333, 777));
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
    let possible_combinations_lookbehind = [GERESH];
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
            return Some(Match::new(revia, 333, 777));
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
    let possible_combinations_lookbehind = [GERESH];
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
            return Some(Match::new(revia, 333, 777));
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

fn indexes_target_char(target_char: char, sentence: &[char]) -> Vec<usize> {
    sentence
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == target_char { Some(index) } else { None })
        .collect()
}

fn is_part_of_two_code_point_accent_look_behind(
    sentence: &[char],
    target_char: char,
    idx_target: usize,
    lookbehind_combos: &[char],
    max_word_span: usize,
) -> bool {
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
        if c == ' ' || c == MAQQEPH {
            word_breaks += 1;
            // If we have crossed the allowed number of word spans, stop.
            if word_breaks >= max_word_span {
                return false;
            }
            continue;
        }

        // If we encounter the same target character again, the current
        // occurrence cannot be part of a two‑code‑point accent.
        if c == target_char {
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
            PASEQ | VERTICAL_LINE => return true,

            // Anything else does not affect the outcome; just continue.
            _ => {}
        }
    }
    // No qualifying mahpakh found within the permitted range.
    false
}

// Check if a character is followed by an Oleh We Yored
// Note: Oleh We Yored may be distributed over two words
fn is_followed_by_oleh_we_yored(index_of_target_char: usize, sentence: &[char]) -> bool {
    //println!("\n==> LOOKING FORWARD - is_followed_by_oleh_we_yored");
    if index_of_target_char >= sentence.len() {
        return false; // Early exit if the position is at the end of the sentence
    }
    let mut word_count: usize = 0usize;
    let mut oleh_seen = false;
    let mut yored_seen = false;

    // Start iterating from the next character after the target character
    for &current_char in sentence.iter().skip(index_of_target_char + 1) {
        //println!("\ncurr_char: {current_char} - word_count: {word_count}");
        match (current_char, word_count) {
            // return if Oleh We Yored is found
            (_, _) if oleh_seen & yored_seen => {
                //println!("OLEH WE YORED found!");
                return oleh_seen && yored_seen;
            }
            // return if maximum word count is reached
            (_, 3) => {
                //println!("MAX word count reached");
                return oleh_seen && yored_seen;
            }
            // update word count after a space
            (' ', _) => word_count += 1,
            // update word count after a Maqqeph
            (MAQQEPH, _) => word_count += 1,
            // check for Oleh in the next word
            (OLEH, 1) => {
                //println!("found Oleh");
                oleh_seen = true;
            }
            // check for Yored in the next or second word
            (YORED, 1 | 2) if oleh_seen => {
                //println!("found YORED");
                yored_seen = true;
            }
            // default
            (_, _) => {}
        }
        // println!("curr_char: {current_char} - word_count:  {word_count}");
    }
    // println!("==> Result: \n\toleh_seen: {oleh_seen}\tyored_seen: {yored_seen}");
    oleh_seen && yored_seen
}
// lumo

// / Returns `true` if, after `target_idx`, the characters contain the
// / sequence **Oleh We Yored** within the next three words.
// /
// / A *word* is delimited by a normal space `' '` or by the special
// / separator `MAQQEPH`. The function stops scanning after three word
// / boundaries because the sequence is not allowed to stretch farther.
// /
// / # Arguments
// /
// / * `target_idx` – Index of the character we are checking *after*.
// / * `sentence`   – Slice of characters representing the whole sentence.
// /
// / # Returns
// /
// / * `true`  – Both `OLEH` and `YORED` were found in the correct order.
// / * `false` – The sequence was not found or the slice ended before it could be completed.

// fn _is_followed_by_oleh_we_yored(target_idx: usize, sentence: &[char]) -> bool {
//     // Guard against an out‑of‑range index.
//     if target_idx >= sentence.len() {
//         return false;
//     }

//     // Word‑boundary counter: a boundary is either a regular space or the
//     // special separator `MAQQEPH`.
//     let mut word_boundary_cnt = 0usize;
//     let mut oleh_seen = false;
//     let mut yored_seen = false;

//     // Iterate over the characters *after* the target character.
//     for &c in sentence.iter().skip(target_idx + 1) {
//         // Stop once we have examined three word boundaries.
//         if word_boundary_cnt == 3 {
//             break;
//         }

//         match c {
//             // Word boundaries increment the counter.
//             ' ' | MAQQEPH => word_boundary_cnt += 1,

//             // The first word after the target may contain `OLEH`.
//             OLEH if word_boundary_cnt == 1 => oleh_seen = true,

//             // `YORED` may appear in the first or second word *after* we have
//             // already seen `OLEH`.
//             YORED if (word_boundary_cnt == 1 || word_boundary_cnt == 2) && oleh_seen => {
//                 yored_seen = true;
//                 // Both parts are present → we can return early.
//                 return true;
//             }

//             // Any other character does not affect the state.
//             _ => {}
//         }
//     }

//     // If we exit the loop without having seen both parts, the sequence is absent.
//     false
// }
