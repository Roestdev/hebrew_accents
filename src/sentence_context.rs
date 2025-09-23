use crate::accent::PoetryAccent;
use crate::accent::ProseAccent;
use crate::char::*;
use crate::regex::*;
use crate::HebrewAccent;

/// Sentence including the context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SentenceContext {
    pub sentence: String,
    pub context: Context,
}

/// Describes the context of a sentence (poetic or prosaic)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Context {
    /// The sentence follows a poetic structure (e.g., meter, rhyme).
    Poetic,
    /// The sentence follows ordinary prose conventions.
    #[default]
    Prosaic,
}

impl SentenceContext {
    /// Creates a new object: SentenceContext
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::Context;
    /// use hebrew_accents::SentenceContext;
    ///
    /// let sentence_context = SentenceContext::new( "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ", Context::Prosaic);
    /// assert_eq!(sentence_context.context,Context::Prosaic);
    /// assert_eq!(sentence_context.sentence,"וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ".to_string());
    /// ```
    pub fn new(sentence: &str, context: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            context,
        }
    }

    /// Checks the creation a neew instance of SentenContext.
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::SentenceContext;
    /// use hebrew_accents::Context;
    /// use hebrew_accents::HebrewAccent;
    /// use hebrew_accents::ProseAccent;
    ///
    /// let sentence_context = SentenceContext::new("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",Context::Prosaic,);
    /// assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
    /// ```
    pub fn contains_accent(&self, accent: HebrewAccent) -> bool {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                RE_COMMON_SILLUQ.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self.sentence.contains(ETNAHTA),
            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                self.sentence.contains(SEGOL)
            }
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.context == Context::Prosaic => {
                RE_COMMON_SHALSHELET.is_match(&self.sentence)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQaton) if self.context == Context::Prosaic => {
                self.sentence.contains(ZAQEF_QATAN)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                self.sentence.contains(ZAQEF_GADOL)
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => {
                self.sentence.contains(REVIA)
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarkha) => self.sentence.contains(TIPEHA),
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => {
                self.sentence.contains(ZARQA)
            }
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => {
                self.sentence.contains(PASHTA)
            }
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => {
                self.sentence.contains(YETIV)
            }
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => {
                self.sentence.contains(TEVIR)
            }
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => {
                self.sentence.contains(GERESH)
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                self.sentence.contains(GERSHAYIM)
            }
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence.contains(PAZER)
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                self.sentence.contains(QARNEY_PARA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains(TELISHA_GEDOLA)
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) => {
                RE_PROSE_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) if self.context == Context::Prosaic => {
                RE_PROSE_MUNACH.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
                self.sentence.contains(MAHPAKH)
            }
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => {
                self.sentence.contains(MERKHA)
            }
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains(MERKHA_KEFULA)
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => {
                self.sentence.contains(DARGA)
            }
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => {
                self.sentence.contains(QADMA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains(TELISHA_QETANA)
            }
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => self.sentence.contains(YERAH_BEN_YOMO),
            HebrewAccent::Prose(ProseAccent::Mayela) if self.context == Context::Prosaic => {
                RE_PROSE_MEAYLA.is_match(&self.sentence)
            }
            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => {
                RE_COMMON_METEG.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Maqqeph)
            | HebrewAccent::Poetry(PoetryAccent::Maqqeph) => self.sentence.contains(MAQQEPH),
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.context == Context::Poetic => {
                RE_POETRY_OLEH_WE_YORED.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                contains_poetry_revia_gadol(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                RE_POETRY_REVIA_MUGRASH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetic =>
            {
                RE_COMMON_SHALSHELET.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => {
                self.sentence.contains(ZINOR)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                contains_poetry_revia_qaton(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => {
                self.sentence.contains(DEHI)
            }
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_MEHUPPAKH_LEGARMEH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                RE_POETRY_AZLA_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.context == Context::Poetic => {
                self.sentence.contains(MUNAH)
            }
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => {
                contains_poetry_merkha(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => {
                self.sentence.contains(ILUY)
            }
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => {
                contains_poetry_mehuppakh(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => {
                RE_POETRY_AZLA.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_SHALSHELET_QETANNAH
                    .is_match(&self.sentence)
                    .unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_TSINNORIT_MERKHA.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_TSINNORIT_MAHPAKH.is_match(&self.sentence)
            }
            _ => false,
        }
    }

    pub fn find_accent(&self, accent: HebrewAccent) -> Option<usize> {
        // let haystack = "Hello, world! This is a test.";
        // let pattern = r"world"; // The regex pattern to search for
        // let re = Regex::new(pattern).unwrap();
        // if let Some(matched) = re.find(haystack) {
        //     println!("Found match at position: {:?}", matched.start());
        // } else {
        //     println!("No match found.");
        // }
        match accent {
            // Prose Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => None,
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => None,
            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQaton) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => None,
            // Prose Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Mayela) if self.context == Context::Prosaic => None,
            // Poetry Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.context == Context::Poetic => {
                None
            }
            //HebrewAccent::Poetry(PoetryAccent::Atnach) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Pazer) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                None
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Tarkha) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Galgal) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                None
            }
            _ => None,
        }
    }
}

fn contains_poetry_merkha(sentence: &str) -> bool {
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
            println!(
                "Found at least one target char, not part of another aaccent:: BREAK the loop"
            );
            return true;
        }
    }
    false
}

fn contains_poetry_mehuppakh(sentence: &str) -> bool {
    // Mehupppakh (as a poetry accent)
    //   not part of Mehuppakh Legarmeh (needs Negative Forward)
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

        println!("\nResult: Negative Looking Backward = {two_code_points_behind}");
        println!("Result: Negative Looking Forward = {is_part_of_mahpakh_legarmeh}");
        if !two_code_points_behind && !is_part_of_mahpakh_legarmeh {
            // println!(
            //     "\nResult for index {index}:\n\ttwo_code_points_behind: {two_code_points_behind}\n\tis_part_of_mahpakh_legarmeh: {is_part_of_mahpakh_legarmeh}"
            // );
            println!("Found target char, not part of another accent. Returning TRUE");
            return true;
        }
    }
    false
}

fn contains_poetry_revia_gadol(sentence: &str) -> bool {
    // Revia Gadol is
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    //   AND
    //   not followed by an Oleh We Yored (needs Positive LookAhead)
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
        println!(
            "two_code_points_behind: {two_code_points_behind} - followed_by_owy: {followed_by_owy}"
        );
        if !two_code_points_behind && !followed_by_owy {
            return true;
        }
    }
    false
}

fn contains_poetry_revia_qaton(sentence: &str) -> bool {
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
        println!("\n\nLOOP::Index of target character = {index}\n");
        println!("Negative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
            index,
            &possible_combinations_lookbehind,
            1,
        );
        println!("Followed by Oleh We Yored");
        let followed_by_owy = is_followed_by_oleh_we_yored(index, &char_vec);
        // 2cp   oleweyored     revia_qaton
        //  no      no      -       no
        //  no      yes     -       yes
        //  yes     no      -       no
        //  yes     yes     -       no
        println!(
            "two_code_points_behind:{two_code_points_behind} - followed_by_owy{followed_by_owy}"
        );
        if !two_code_points_behind && followed_by_owy {
            return true;
        }
    }
    false
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
    target_char: &char,
    index_target_char: usize,
    possible_combinations_lookbehind: &[char],
    max_word_span: usize,
) -> bool {
    // the word containing the accent is the first word
    println!("\n==> LOOKING BACKWARDS");
    let mut word_count: usize = 0;
    if index_target_char == 0 {
        println!("LB::Target character is found at the first position");
        // Early exit if the position is at the start
        return false;
    }
    // backwards search
    for current_index in (0..index_target_char).rev() {
        let current_char = sentence[current_index];
        println!(
            "LB::Current character [ {current_char} ] at index {current_index} - word_count: {word_count}"
        );
        if current_char == ' ' || current_char == MAQQEPH {
            word_count += 1;
            println!("LB::new word_count: {word_count}");
            if word_count == max_word_span {
                println!("LB::Max word count reached");
                return false;
            }
        }
        // Check for the second directly following target character
        if current_char == *target_char {
            println!(
                "LB::Found target character [{current_char}] at position {current_index}  ==> return false"
            );
            return false;
        }
        // Check for possible combinations
        if possible_combinations_lookbehind.contains(&current_char) {
            println!("LB::Part of two code-point accent ==> return true");
            return true;
        }
    }
    println!("LB::End of look behind ==> return false");
    false
}

fn is_part_of_mahpakh_legarmeh_look_ahead(index_target_char: usize, sentence: &[char]) -> bool {
    if index_target_char >= sentence.len() {
        return false;
    }
    let mut word_count: usize = 0;
    // Start iterating from the next character after the target character
    for current_char in sentence.iter().skip(index_target_char + 1) {
        match (*current_char, word_count) {
            // max word count
            (_, 2) => {
                println!("MAX wordcount");
                return false;
            }
            // space found -> count ++
            (' ', _) => {
                word_count += 1;
            }
            // mahpakh found
            (PASEQ, 0 | 1) => {
                println!("PASEQ found");
                return true;
            }
            (VERTICAL_LINE, 0 | 1) => {
                println!("VERTICAL_LINE found");
                return true;
            }
            // PASEQ or VERTICAL_LINE not found as first character in the second word
            (_, 1) => {
                println!("No PASEQ found as the first char 2nd word");
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
fn is_followed_by_oleh_we_yored(index_of_target_char: usize, sentence: &[char]) -> bool {
    println!("\n==> LOOKING FORWARD - is_followed_by_oleh_we_yored");
    if index_of_target_char >= sentence.len() {
        return false; // Early exit if the position is at the end of the sentence
    }
    let mut word_count: usize = 0;
    let mut oleh_found = false;
    let mut yored_found = false;
    // Start iterating from the next character after the target character
    for current_char in sentence.iter().skip(index_of_target_char + 1) {
        println!("\ncurr_char: {current_char} - word_count: {word_count}");
        match (*current_char, word_count) {
            // return if Oleh We Yored is found
            (_, _) if oleh_found & yored_found => {
                println!("OLEH WE YORED found!");
                return oleh_found && yored_found;
            }
            // return if maximum word count is reached
            (_, 3) => {
                println!("MAX word count reached");
                return oleh_found && yored_found;
            }
            // update word count after a space
            (' ', _) => word_count += 1,
            // update word count after a Maqqeph
            (MAQQEPH, _) => word_count += 1,
            // check for Oleh in the next word
            (OLEH, 1) => {
                println!("found Oleh");
                oleh_found = true;
            }
            // check for Yored in the next or second word
            (YORED, 1 | 2) if oleh_found => {
                println!("found YORED");
                yored_found = true;
            }
            // default
            (_, _) => {}
        }
        println!("curr_char: {current_char} - word_count:  {word_count}");
    }
    println!("==> Result: \n\toleh_found: {oleh_found}\tyored_found: {yored_found}");
    oleh_found && yored_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_prose_poetry_silluq() {
        // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
        let sentence_c = SentenceContext::new(" וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר וַֽיְהִי־אֽוֹר׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        // ProseAccent, with Soph Paseq, no Pey or Samech
        let sentence_c = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        // ProseAccent, no Soph Paseq, with Pey
        let sentence_c = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // PoetryAccent with Soph Paseq and Peh
        let sentence_c = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // Meteg not in the last word of the sentence
        let sentence_c = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // Meteg followed by Maqqeph (\u{05BE}) (meaning no Meteg in the last word)
        let sentence_c1 = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
        assert!(!sentence_c1.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
    }
    #[test]
    fn test_contains_prose_poetry_atnach() {
        // Atnach present
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
        // No Atnach present
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let sentence_c = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
        let sentence_c = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        // Shalshelet, with Paseq - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        // Shalshelet, with Paseq + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        // Shalshelet, with Vertical Bar - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        // Shalshelet, with Vertical Bar + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        // Missing Paseq or Vertical Bar
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
    }
    #[test]
    fn test_contains_prose_zaqeph_qaton() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQaton)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQaton)));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
    }
    #[test]
    fn test_contains_prose_revia() {
        let sentence_c = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
        let sentence_c = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let sentence_c = SentenceContext::new(
            "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
        let sentence_c = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let sentence_c = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
        let sentence_c = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let sentence_c =
            SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let sentence_c =
            SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
        let sentence_c =
            SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        // Legarmeh, with Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Legarmeh with a space + Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Legarmeh with two spaces + Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Legarmeh, with Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Legarmeh, with space + Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Legarmeh, with two spaces + Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        // Paseq or Vertical Bar is missing
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
    }
    // Conjunctives
    #[test]
    fn test_contains_prose_munnach() {
        // Single Munach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
        // Munach part of Legarmeh (Paseq)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
        // Munach part of Legarmeh (space + Paseq)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים ׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
        // Munach part of Legarmeh (Vertical Bar)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים|  את השּׁמים ואת הארץ׃׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
        // Munach part of Legarmeh (space +Vertical Bar)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים  |  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let sentence_c = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
        let sentence_c = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let sentence_c = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
    }
    #[test]
    fn test_contains_prose_darga() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
    }
    #[test]
    fn test_contains_prose_azla() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
    }
    #[test]
    fn test_contains_prose_galgal() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
    }
    #[test]
    fn test_contains_prose_meayla() {
        // Tiphcha followed by Atnach
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mayela)));
        // Tiphcha followed by Atnach, two words connected with a Maqqeph
        let sentence_c =
            SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mayela)));
        // Tiphcha followed by silluq
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mayela)));
        // only Tiphcha
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵ֖ת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Mayela)));
    }
    #[test]
    fn test_contains_prose_meteg() {
        // Only Silluq, No Meteg
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Meteg)));
        // Meteg and Siluq, separated by a Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Meteg)));
        // Meteg and Siluq in separate words
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Meteg)));
        // Only Meteg, no Silluq
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Meteg)));
    }
    #[test]
    fn test_contains_prose_maqqeph() {
        // No Maqqeph
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Maqqeph)));
        // One Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Maqqeph)));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_oleh_we_yored() {
        // OlehWeYored, one word
        let sentence_c = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
        // OlehWeYored, one word - context: Prosaic
        let sentence_c = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
        // OlehWeYored, two words
        let sentence_c = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
        // OlehWeYored, three words
        let sentence_c = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        // No Revia at all
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        // Two Revia's
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּרא אלהים את השּׁ֗מים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        // Revia followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        // Revia followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        // Revia followed by 'Oleh We Yored' (3 words)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        // Revia not directly followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        // Revia and Geresh
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
        // Only Revia
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
        // Only Geresh
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        // ShalsheletGadol, with Paseq - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        // ShalsheletGadol, with Paseq + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        // ShalsheletGadol, with Vertical Bar - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        // ShalsheletGadol, with Vertical Bar + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        // Missing Paseq or Vertical Bar
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        assert!(!sentence_c.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let sentence_c = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        // No revia at all
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia, not followed by OleWe Yored
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia directly followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמי֥ם ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia directly followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia directly followed by 'Oleh We Yored' (3 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים ואת האר֥ץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia NOT directly followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֗להים א֓ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // Revia is part of Revia Mugrash
        let sentence_c = SentenceContext::new(
            " שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let sentence_c = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let sentence_c = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
    }
    #[test]
    fn test_contains_poetry_mehuppakh_legarmeh() {
        // MehuppakhLegarmeh, with Paseq
        let sentence_c = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)));
        // MehuppakhLegarmeh, with Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)));
        // Mehuppakh only
        let sentence_c = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        // AzlaLegarmeh, with Paseq + no space
        let sentence_c = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        // AzlaLegarmeh, with Paseq + 1 space
        let sentence_c = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        // AzlaLegarmeh, with Vertical Bar + no space
        let sentence_c = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        // AzlaLegarmeh, with Vertical Bar + 1 space
        let sentence_c = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        // Azla only
        let sentence_c = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let sentence_c = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munach)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munach)));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        // No Merkha
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // One Merkha
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Tsinnorit + Merkha (1w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Tsinnorit + Merkha (2w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Tsinnorit + Merkha (3w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Oleh + Merkha (1w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Oleh + Merkha (2w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // Oleh + Merkha (3w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
    }

    #[test]
    fn test_contains_poetry_illuy() {
        let sentence_c = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
    }
    #[test]
    fn test_contains_poetry_tarkha() {
        let sentence_c = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarkha)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarkha)));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let sentence_c = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
    }
    #[test]
    fn test_contains_poetry_mehuppakh() {
        // No Mehuppach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Tsinnorit Mappach (one word)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Tsinnorit Mappach (two words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Tsinnorit Mappach (three words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));

        // One Mehuppach, part of Mehuppach Legarmeh (no space) TODO
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Mehuppach Legarmeh (one space) TODO
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Mehuppach Legarmeh (no space - vertical line)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Mehuppach Legarmeh (one space - vertical line)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of 'Mehuppach Legarmeh' (too many spaces)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
        let sentence_c =
            SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
        // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
        let sentence_c =
            SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh)));
    }

    #[test]
    fn test_contains_poetry_azla() {
        // contains Azla
        let sentence_c = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        // contains Azla and Azla Legarmeh
        let sentence_c = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        // Azla Legarmeh, with space + Paseq
        let sentence_c = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        // Azla Legarmeh, with Paseq
        let sentence_c = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        // Azla Legarmeh, with space + Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        // Azla Legarmeh, with Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        // Shalshelet
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        // Shalshelet Gadol, with Paseq
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        // Shalshelet Gadol, with Vertical Bar
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_merkha() {
        // accent in a single word
        let sentence_c = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in a single word, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in a single word, without Merkha
        let sentence_c = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words seperated by Maqqeph
        let sentence_c = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words seperated by Maqqeph, without Merkha
        let sentence_c = SentenceContext::new("את־א֘ברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words
        let sentence_c = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words, without Tsinnorit
        let sentence_c = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words, without Merkha
        let sentence_c = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in three words
        let sentence_c = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mahpakh() {
        // accent in a single word
        let sentence_c = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in a single word
        let sentence_c = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in a single word, without Tsinnorit
        let sentence_c = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sentence_c = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sentence_c = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words
        let sentence_c = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in two words, without Mahpakh
        let sentence_c = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in three words
        let sentence_c = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
    }

    #[test]
    fn test_contains_poetry_meteg() {
        // Only Silluq, No Meteg
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Meteg)));
        // Meteg and Siluq, separated by a Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Meteg)));
        // Meteg and Siluq in separate words
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Meteg)));
        // Only Meteg, no Silluq
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Meteg)));
    }
    #[test]
    fn test_contains_poetry_maqqeph() {
        // No Maqqeph
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Maqqeph)));
        // One Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(HebrewAccent::Poetry(PoetryAccent::Maqqeph)));
    }
}
