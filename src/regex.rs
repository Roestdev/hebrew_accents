use fancy_regex::Regex as FancyRegex;
use once_cell::sync::Lazy;
use regex::Regex;

/*
    Regular expressions for SentenceContext.contains() 
*/

// A Meteg in the last word of a sentence is called SILLUQ (\u{05BD})
// Most of the time a sentence ends with Sof Pasuq (\u{05C3})
// Some times a sentence ends with "samech" (U+05E1) or an "pey" (U+05E4).
// Some times last words are connected by a Maqqef (\u{05BE})
pub static RE_COMMON_SILLUQ: Lazy<FancyRegex> = Lazy::new(|| {
    FancyRegex::new(r"\u{05BD}(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$").unwrap()
});

// A Shalshelet consists of the following two UTF-8 code-points (p.e. Gen19:16)
//      - Shalshelet (\u{0593}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
pub static RE_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{0593}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
});

// A 'Legarmeh' consists of the following two UTF-8 code-points:
//      - Munach (\u{05A3}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
pub static RE_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{05A3}\p{Hebrew}*?\s*?[\u{05C0}\u{007C}]").unwrap()
});

// Context: Prose and Poetry
// A 'Munach' is a 'Munach' if it is NOT FOLLOWED by a Paseq !
// Otherwise is called a 'Legarmeh'
//      - Munach (\u{05A3})
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
pub static RE_PROSE_MUNACH: Lazy<FancyRegex> =
    Lazy::new(|| FancyRegex::new(r"\u{05A3}(?!\p{Hebrew}*?\s*?[\u{05C0}\u{007C}])").unwrap());

// A Mayela is a Tiphcha before Silluq or Atnach in the same word
// or words connected with a Maqqef (\u{05BE})
// Tiphcha: U+0596
// Atnach:  U+0591
// Silluq:  U+05BD (Meteg in the last word)
pub static RE_PROSE_MEAYLA: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\u{0596}\p{Hebrew}+\u{0591}|\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap()
});

// A meteg is considered a meteg only when it is found in a word that is not the final word of a sentence.
// A Silluq is not a Meteg
pub static RE_COMMON_METEG: Lazy<FancyRegex> = Lazy::new(|| {
    FancyRegex::new(r"\u{05BD}(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap()
});

// An 'Ole We Yored' consists of the following two UTF-8 code-points
//      - Ole (\u{05AB}) followed by
//      - Yored (\u{05A5}) aka Merkha
// This accent can stretch over two words (a.k.a. word-unit)
pub static RE_POETRY_OLEH_WE_YORED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap());

// A 'Revia Mugrash' consists of the following two UTF-8 code-points:
// - Geresh (\u{059C}) followed by
// - Revia (\u{0597})
// - Maqqef (\u{05BE})
// - 'Geresh Muqdam' (\u{059D}) is Jiddisch?
pub static RE_POETRY_REVIA_MUGRASH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\s\u{05BE}]\p{Hebrew}*[\u{059C}\u{059D}]\p{Hebrew}*\u{0597}").unwrap()
});

// An 'Mehuppakh Legarmeh' consists of the following two UTF-8 code-points:
//      - Mehuppakh (\u{05A4}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
pub static RE_POETRY_MEHUPPAKH_LEGARMEH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\u{05A4}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());

// An 'Azla Legarmeh' consists of the following two UTF-8 code-points:
//      - Azla (\u{05A8}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
pub static RE_POETRY_AZLA_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{05A8}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
});

pub static RE_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
    FancyRegex::new(r"(\u{05A8}\p{Hebrew}*?\u{05BE})|(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))")
        .unwrap()
});

// A Shalshalet Not followed by a Sof Passuq (or a vertical line)
pub static RE_POETRY_SHALSHELET_QETANNAH: Lazy<FancyRegex> =
    Lazy::new(|| FancyRegex::new(r"\u{0593}(?!\p{Hebrew}*?\s?[\u{05C0}\u{007C}])").unwrap());

// A Tsinnorit Merkha consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Merkha (\u{05A5})
// This accent can occur in one or two words (a.k.a. word-unit)
pub static RE_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A5}")
        .unwrap()
});

// A Tsinnorit Mahpakh consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Mahpakh (\u{05A4})
// This accent can occur in one or two words (a.k.a. word-unit)
pub static RE_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A4}")
        .unwrap()
});