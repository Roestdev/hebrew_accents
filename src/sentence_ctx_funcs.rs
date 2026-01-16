// Standard library

// External crates

// Local modules / crate‑internal
use crate::char::{
    GERESH, MAHPAKH, MAQQEPH, MERKHA, OLEH, PASEQ, REVIA, TSINNORIT, VERTICAL_LINE, YORED, ZARQA,
};

pub(crate) fn contains_poetry_merkha(sentence: &str) -> bool {
    // Merkha (as a poetry accent) is
    //   not part of Oleh We Yored (needs Negative Lookbehind)
    //   AND
    //   not part of Tsinnorit Merkha (needs Negative Lookbehind)
    let target_char = MERKHA;
    let possible_combinations_lookbehind = [TSINNORIT, OLEH];

    // Check for the existence of the target character in the sentence
    if !&sentence.contains(target_char) {
        return false;
    }
    // Convert the sentence into a Vec<char> for character indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // Find the indices of the target character within the sentence
    let indices = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for &index in &indices {
        let is_part_of = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
            index,
            &possible_combinations_lookbehind,
            2,
        );
        if !is_part_of {
            // println!(
            //     "Found at least one target char, not part of another aaccent:: BREAK the loop"
            // );
            return true;
        }
    }
    false
}

pub(crate) fn contains_poetry_mehuppakh(sentence: &str) -> bool {
    // Mehupppakh (as a poetry accent)
    //   not part of Mehuppakh Legarmeh (needs Negative Lookahead)
    //   AND
    //   not part of Tsinnorit Mahpakh (needs Negative Lookbehind)
    let target_char = MAHPAKH;
    let possible_combinations_lookbehind = [ZARQA];

    // check if the target character is present in the sentence
    if !&sentence.contains(target_char) {
        //println!("MEHUPPAKH not found in the senctence at all  -> return false");
        return false;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    println!("positions found at: {indices:?}");
    // loop over all character positions
    for index in indices {
        println!("\n\nCHECK position {index}");
        //println!("\nNegative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
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
            return true;
        }
    }
    false
}

pub(crate) fn contains_poetry_revia_gadol(sentence: &str) -> bool {
    // Revia Gadol is
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    //   AND
    //   not followed by an Oleh We Yored (needs Negative Lookahead)
    let target_char = REVIA;
    let possible_combinations_lookbehind = [GERESH];

    // check if the target character is present in the senctence
    if !&sentence.contains(target_char) {
        return false;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for index in indices {
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
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
            return true;
        }
    }
    false
}

pub(crate) fn contains_poetry_revia_qaton(sentence: &str) -> bool {
    // Revia Qaton is
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    //   AND
    //   followed by an Oleh We Yored (needs Positive LookAhead)
    let target_char = REVIA;
    let possible_combinations_lookbehind = [GERESH];

    // check if the target character is present in the senctence
    if !&sentence.contains(target_char) {
        println!("REVIA not found in the senctence at all  -> return false");
        return false;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_target_char(target_char, &char_vec);
    // loop over all character positions
    for index in indices {
        // println!("\n\nLOOP::Index of target character = {index}\n");
        // println!("Negative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
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
            return true;
        }
    }
    false
}

pub(crate) fn indexes_target_char(target_char: char, sentence: &[char]) -> Vec<usize> {
    sentence
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == target_char { Some(index) } else { None })
        .collect()
}

pub(crate) fn is_part_of_two_code_point_accent_look_behind(
    sentence: &[char],
    target_char: &char,
    index_target_char: usize,
    possible_combinations_lookbehind: &[char],
    max_word_span: usize,
) -> bool {
    // the word containing the accent is the first word
    //println!("\n==> LOOKING BACKWARDS");
    let mut word_count: usize = 0;
    if index_target_char == 0 {
        //println!("LB::Target character is found at the first position");
        // Early exit if the position is at the start
        return false;
    }
    // backwards search
    for current_index in (0..index_target_char).rev() {
        let current_char = sentence[current_index];
        // println!(
        //     "LB::Current character [ {current_char} ] at index {current_index} - word_count: {word_count}"
        // );
        if current_char == ' ' || current_char == MAQQEPH {
            word_count += 1;
            //println!("LB::new word_count: {word_count}");
            if word_count == max_word_span {
                //println!("LB::Max word count reached");
                return false;
            }
        }
        // Check for the second directly following target character
        if current_char == *target_char {
            // println!(
            //     "LB::Found target character [{current_char}] at position {current_index}  ==> return false"
            // );
            return false;
        }
        // Check for possible combinations
        if possible_combinations_lookbehind.contains(&current_char) {
            //println!("LB::Part of two code-point accent ==> return true");
            return true;
        }
    }
    //println!("LB::End of look behind ==> return false");
    false
}

pub(crate) fn is_part_of_mahpakh_legarmeh_look_ahead(
    index_target_char: usize,
    sentence: &[char],
) -> bool {
    if index_target_char >= sentence.len() {
        return false;
    }
    let mut word_count: usize = 0;
    // Start iterating from the next character after the target character
    for current_char in sentence.iter().skip(index_target_char + 1) {
        match (*current_char, word_count) {
            // max word count
            (_, 2) => {
                //println!("MAX wordcount");
                return false;
            }
            // space found -> count ++
            (' ', _) => {
                word_count += 1;
            }
            // mahpakh found
            (PASEQ, 0 | 1) => {
                //println!("PASEQ found");
                return true;
            }
            (VERTICAL_LINE, 0 | 1) => {
                //println!("VERTICAL_LINE found");
                return true;
            }
            // PASEQ or VERTICAL_LINE not found as first character in the second word
            (_, 1) => {
                //println!("No PASEQ found as the first char 2nd word");
                return false;
            }
            // default
            (_, _) => {}
        }
    }
    false
}

// Check if a character is followed by an Oleh We Yored
// Note: Oleh We Yored may be distributed over two words
pub(crate) fn is_followed_by_oleh_we_yored(index_of_target_char: usize, sentence: &[char]) -> bool {
    //println!("\n==> LOOKING FORWARD - is_followed_by_oleh_we_yored");
    if index_of_target_char >= sentence.len() {
        return false; // Early exit if the position is at the end of the sentence
    }
    let mut word_count: usize = 0;
    let mut oleh_found = false;
    let mut yored_found = false;
    // Start iterating from the next character after the target character
    for current_char in sentence.iter().skip(index_of_target_char + 1) {
        //println!("\ncurr_char: {current_char} - word_count: {word_count}");
        match (*current_char, word_count) {
            // return if Oleh We Yored is found
            (_, _) if oleh_found & yored_found => {
                //println!("OLEH WE YORED found!");
                return oleh_found && yored_found;
            }
            // return if maximum word count is reached
            (_, 3) => {
                //println!("MAX word count reached");
                return oleh_found && yored_found;
            }
            // update word count after a space
            (' ', _) => word_count += 1,
            // update word count after a Maqqeph
            (MAQQEPH, _) => word_count += 1,
            // check for Oleh in the next word
            (OLEH, 1) => {
                //println!("found Oleh");
                oleh_found = true;
            }
            // check for Yored in the next or second word
            (YORED, 1 | 2) if oleh_found => {
                //println!("found YORED");
                yored_found = true;
            }
            // default
            (_, _) => {}
        }
        // println!("curr_char: {current_char} - word_count:  {word_count}");
    }
    // println!("==> Result: \n\toleh_found: {oleh_found}\tyored_found: {yored_found}");
    oleh_found && yored_found
}
