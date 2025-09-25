use fancy_regex::Regex as FancyRegex;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::char::*;


const HEBREW_CHAR: &str = r"\p{Hebrew}";
const OPTIONAL_SPACE: &str = r"\s?";
const ZERO_OR_ONE_SPACE: &str = r"\s?";
const ONE_OR_MORESPACES: &str = r"\s*?";
const NOT_A_SPACE_OR_MAQAF: &str = r"[^\s\u{05BE}]";
const SPACE_OR_MAQAF: &str = r"[\s\u{05BE}]";
const PASEQ_OR_VERTICAL_LINE: &str = r"[\u{05C0}\u{007C}]";
const GERESH_OR_GERESH_MUQDAM: &str = r"[\u{059C}\u{059D}]";
const NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE: &str = r"(?!\p{Hebrew}*?\s*?[\u{05C0}\u{007C}])";
const NOT_FOLLOWED_BY_MAQAF: &str = r"(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)";
const ZERO_OR_ONE_SAMECH_OR_PEY: &str = r"[\u{05E4}\u{05E1}]?";
const OR: &str = r"|";

/*
    Regular expressions for SentenceContext.contains()
*/

// A Meteg in the last word of a sentence is called SILLUQ (\u{05BD})
// Most of the time a sentence ends with Sof Pasuq (\u{05C3})
// Some times a sentence ends with "samech" (U+05E1) or an "pey" (U+05E4).
// Some times last words are connected by a Maqqeph (\u{05BE})
//    FancyRegex::new(r"\u{05BD}(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s$?")
pub static RE_COMMON_SILLUQ: Lazy<FancyRegex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}{}*{}{}{}{}{}$",
        SILLUQ,
        NOT_FOLLOWED_BY_MAQAF,
        HEBREW_CHAR,
        OPTIONAL_SPACE,
        SOF_PASUQ,
        OPTIONAL_SPACE,
        ZERO_OR_ONE_SAMECH_OR_PEY,
        OPTIONAL_SPACE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    FancyRegex::new(&pattern).expect("invalid regex for munach")
});

// A Shalshelet consists of the following two UTF-8 code-points (p.e. Gen19:16)
//      - Shalshelet (\u{0593}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{0593}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
pub static RE_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}*?{}{}",
        NOT_A_SPACE_OR_MAQAF,
        HEBREW_CHAR,
        SHALSHELET,
        HEBREW_CHAR,
        ZERO_OR_ONE_SPACE,
        PASEQ_OR_VERTICAL_LINE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Legarmeh")
});

// A 'Legarmeh' consists of the following two UTF-8 code-points:
//      - Munach (\u{05A3}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{05A3}\p{Hebrew}*?\s*?[\u{05C0}\u{007C}]").unwrap()
pub static RE_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}*?{}{}",
        NOT_A_SPACE_OR_MAQAF,
        HEBREW_CHAR,
        MUNAH,
        HEBREW_CHAR,
        ONE_OR_MORESPACES,
        PASEQ_OR_VERTICAL_LINE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Legarmeh")
});

// A 'Munach' is a 'Munach' if it is NOT FOLLOWED by a Paseq !
// Otherwise is called a 'Legarmeh'
//      - Munach (\u{05A3})
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// FancyRegex::new(r"\u{05A3}(?!\p{Hebrew}*?\s*?[\u{05C0}\u{007C}])").unwrap());
pub static RE_PROSE_MUNACH: Lazy<FancyRegex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!("{}{}", MUNAH, NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE);
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    FancyRegex::new(&pattern).expect("invalid regex for munach")
});

// A Mayela is a Tiphcha before Silluq or Atnach in the same word
// or words connected with a Maqqeph (\u{05BE})
// Tiphcha: U+0596
// Atnach:  U+0591
// Silluq:  U+05BD (Meteg in the last word)
//     Regex::new(r"(\u{0596}\p{Hebrew}+\u{0591}|\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap()
pub static RE_PROSE_MEAYLA: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}+{}{}{}{}*?{}{}?{}{}",
        MEAYLA,
        HEBREW_CHAR,
        ATNACH,
        OR,
        MEAYLA,
        HEBREW_CHAR,
        SILLUQ,
        HEBREW_CHAR,
        ZERO_OR_ONE_SAMECH_OR_PEY,
        OPTIONAL_SPACE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for oleh_we_yored")
});

// TODO
// A meteg is considered a meteg only when it is found in a word that is not the final word of a sentence.
// A Silluq is not a Meteg
//  FancyRegex::new(r"\u{05BD}(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)")
const NAAMVERZINNEN: &str =
    r"(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)";
pub static RE_COMMON_METEG: Lazy<FancyRegex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!("{}{}", METEG, NAAMVERZINNEN,);
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    FancyRegex::new(&pattern).expect("invalid regex for Shalshelet Qetannah")
});

// An 'Ole We Yored' consists of the following two UTF-8 code-points
//      - Ole (\u{05AB}) followed by
//      - Yored (\u{05A5}) aka Merkha
// This accent can stretch over two words (a.k.a. word-unit)
// Regex::new(r"\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap());
pub static RE_POETRY_OLEH_WE_YORED: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}+{}{}*{}",
        OLEH, HEBREW_CHAR, OPTIONAL_SPACE, HEBREW_CHAR, YORED
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for oleh_we_yored")
});

// A 'Revia Mugrash' consists of the following two UTF-8 code-points:
// - Geresh (\u{059C}) followed by
// - Revia (\u{0597})
// - Maqqeph (\u{05BE})
// - 'Geresh Muqdam' (\u{059D}) is Jiddisch?
// Regex::new(r"[\s\u{05BE}]\p{Hebrew}*[\u{059C}\u{059D}]\p{Hebrew}*\u{0597}").unwrap()
pub static RE_POETRY_REVIA_MUGRASH: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}*?{}",
        SPACE_OR_MAQAF, HEBREW_CHAR, GERESH_OR_GERESH_MUQDAM, HEBREW_CHAR, REVIA
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Revia Mugrash")
});

// An 'Mehuppakh Legarmeh' consists of the following two UTF-8 code-points:
//      - Mehuppakh (\u{05A4}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Lazy::new(|| Regex::new(r"\u{05A4}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());
pub static RE_POETRY_MEHUPPAKH_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}",
        MAHPAKH, HEBREW_CHAR, ZERO_OR_ONE_SPACE, PASEQ_OR_VERTICAL_LINE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Revia Mugrash")
});

// An 'Azla Legarmeh' consists of the following two UTF-8 code-points:
//      - Azla (\u{05A8}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{05A8}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
pub static RE_POETRY_AZLA_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}",
        AZLA, HEBREW_CHAR, ZERO_OR_ONE_SPACE, PASEQ_OR_VERTICAL_LINE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Revia Mugrash")
});

// pub static RE_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
//     FancyRegex::new(r"(\u{05A8}\p{Hebrew}*?\u{05BE})|(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))")
//         .unwrap()
// });
const AZLA_NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE: &str =
    r"(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))";
pub static RE_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}{}*?{}{}{}",
        AZLA, HEBREW_CHAR, MAQAF, OR, AZLA_NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    FancyRegex::new(&pattern).expect("invalid regex for Shalshelet Qetannah")
});

// A Shalshalet NOT followed by a Sof Passuq (or a vertical line)
//    Lazy::new(|| FancyRegex::new(r"\u{0593}(?!\p{Hebrew}*?\s?[\u{05C0}\u{007C}])").unwrap());
pub static RE_POETRY_SHALSHELET_QETANNAH: Lazy<FancyRegex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!("{}{}", SHALSHELET, NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE);
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    FancyRegex::new(&pattern).expect("invalid regex for Shalshelet Qetannah")
});

// A Tsinnorit Merkha consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Merkha (\u{05A5})
// This accent can occur in one or two words (a.k.a. word-unit)
//     Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A5}")
pub static RE_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}?{}*?{}{}+{}?{}*{}",
        SPACE_OR_MAQAF, HEBREW_CHAR, TSINNORIT, HEBREW_CHAR, SPACE_OR_MAQAF, HEBREW_CHAR, MERKHA
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Revia Mugrash")
});

// A Tsinnorit Mahpakh consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Mahpakh (\u{05A4})
// This accent can occur in one or two words (a.k.a. word-unit)
// Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A4}")
pub static RE_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(|| {
    // `format!` creates a `String` from the constant fragments.
    let pattern = format!(
        "{}?{}*?{}{}+{}?{}*{}",
        SPACE_OR_MAQAF, HEBREW_CHAR, TSINNORIT, HEBREW_CHAR, SPACE_OR_MAQAF, HEBREW_CHAR, MAHPAKH
    );
    // `Regex::new` takes a `&str`, so we borrow the `String` here.
    Regex::new(&pattern).expect("invalid regex for Revia Mugrash")
});
