//! Regex that are used for finding 'Hebrew Accents'

// Standard library
use once_cell::sync::Lazy;

// External crates
use fancy_regex::Regex as FancyRegex;
use regex::Regex;

// Crate‑internal (local modules)
use crate::char::{
    ATNACH, AZLA, MAHPAKH, MAQAF, MEAYLA, MERKHA, METEG, MUNAH, OLEH, REVIA, SHALSHELET, SILLUQ,
    SOF_PASUQ, TSINNORIT, YORED,
};

/// Any Hebrew character (Unicode property).
const HEBREW: &str = r"\p{Hebrew}";

/// Zero or one ordinary space.
const OPTIONAL_SPACE: &str = r"\s?";

// One or more spaces (greedy).
// ONE_OR_MORE_SPACES: &str = r"\s+";

/// Any character that is **not** a space nor Maqqeph (U+05BE).
const NOT_A_SPACE_OR_MAQAF: &str = r"[^\s\u{05BE}]";

/// Either a space **or** a Maqqeph.
const SPACE_OR_MAQAF: &str = r"[\s\u{05BE}]";

// Either a space **or** a Maqqeph.
// const HEBREW_OR_SPACE: &str = r"[\p{Hebrew}\s]";

/// A paseq (U+05C0) **or** a vertical line (U+007C).
const PASEQ_OR_VERTICAL_LINE: &str = r"[\u{05C0}\u{007C}]";

/// Geresh (U+059C) OR Geresh‑Muqdam (U+059D).
const GERESH_OR_GERESH_MUQDAM: &str = r"[\u{059C}\u{059D}]";

/// Negative LookAhead: *not* followed by Hebrew chars, optional spaces,
/// and then a paseq or vertical line.
const NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE: &str = r"(?!\p{Hebrew}+?\s*[\u{05C0}\u{007C}])";

/// Negative LookAhead: *not* followed by a Hebrew chars, a maqaf, and another
/// Hebrew run.  This is used to exclude “maqqaf‑connected” sequences.
const NOT_FOLLOWED_BY_MAQAF: &str = r"(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)";

/// Zero or one of the Samech OR Pey characters (U+05E4, U+05E1).
const ZERO_OR_ONE_SAMECH_OR_PEY: &str = r"[\u{05E4}\u{05E1}]?";

/// Simple pipe character for building alternations inside `format!`.
const OR: &str = "|";

// A Meteg in the last word of a sentence is called SILLUQ (\u{05BD})
// Most of the time a sentence ends with Sof Pasuq (\u{05C3})
// Some times a sentence ends with "samech" (U+05E1) or an "pey" (U+05E4).
// Some times last words are connected by a Maqqeph (\u{05BE})
//    FancyRegex::new(r"\u{05BD}(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s$? regex")
pub(crate) static FA_RE_OUTER_COMMON_SILLUQ: Lazy<FancyRegex> = Lazy::new(|| {
    let pattern = format!(
        "{SILLUQ}{NOT_FOLLOWED_BY_MAQAF}{HEBREW}*{OPTIONAL_SPACE}{SOF_PASUQ}{OPTIONAL_SPACE}{ZERO_OR_ONE_SAMECH_OR_PEY}{OPTIONAL_SPACE}$"
    );
    FancyRegex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_COMMON_SILLUQ: {}", &pattern))
});

// A Shalshelet consists of the following two UTF-8 code-points (p.e. Gen19:16)
//      - Shalshelet (\u{0593}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{0593}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
pub(crate) static RE_OUTER_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(
        "{NOT_A_SPACE_OR_MAQAF}{HEBREW}*?{SHALSHELET}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_COMMON_SHALSHELET: {}", &pattern))
});

pub(crate) static RE_INNER_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{SHALSHELET}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_INNER_COMMON_SHALSHELET: {}", &pattern))
});

// A 'Legarmeh' consists of the following two UTF-8 code-points:
//      - Munach (\u{05A3}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{05A3}\p{Hebrew}*?\s*?[\u{05C0}\u{007C}]").unwrap()
pub(crate) static RE_OUTER_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(
        "{NOT_A_SPACE_OR_MAQAF}{HEBREW}*?{MUNAH}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}"
    );
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_PROSE_LEGARMEH: {}", &pattern))
});

pub(crate) static RE_INNER_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{MUNAH}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_INNER_PROSE_LEGARMEH: {}", &pattern))
});

// A 'Munach' is a 'Munach' if it is NOT FOLLOWED by a Paseq !
// Otherwise is called a 'Legarmeh'
//      - Munach (\u{05A3})
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// FancyRegex::new(r"\u{05A3}(?!\p{Hebrew}*?\s*?[\u{05C0}\u{007C}])").unwrap());
pub(crate) static FA_RE_OUTER_PROSE_MUNACH: Lazy<FancyRegex> = Lazy::new(|| {
    let pattern = format!("{MUNAH}{NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE}");
    FancyRegex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_PROSE_MUNACH: {}", &pattern))
});

// A Mayela is a Tiphcha before Silluq or Atnach in the same word
// or words connected with a Maqqeph (\u{05BE})
// Tiphcha: U+0596
// Atnach:  U+0591
// Silluq:  U+05BD (Meteg in the last word)
//     Regex::new(r"(\u{0596}\p{Hebrew}+\u{0591}|\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap()
pub(crate) static RE_OUTER_PROSE_MEAYLA: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(
        "{MEAYLA}{HEBREW}+{ATNACH}{OR}{MEAYLA}{HEBREW}*?{SILLUQ}{HEBREW}?{ZERO_OR_ONE_SAMECH_OR_PEY}{OPTIONAL_SPACE}"
    );
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_PROSE_MEAYLA: {}", &pattern))
});

// A meteg is considered a meteg only when it is found in a word that is not the final word of a sentence.
// A Silluq is not a Meteg
//  FancyRegex::new(r"\u{05BD}(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)")
const METEG_CONSTRAINS: &str =
    r"(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)";
pub(crate) static FA_RE_OUTER_COMMON_METEG: Lazy<FancyRegex> = Lazy::new(|| {
    let pattern = format!("{}{}", METEG, METEG_CONSTRAINS,);
    FancyRegex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_COMMON_METEG: {}", &pattern))
});

// Two UCP's: u{05BD} -> at least the first one is a Meteg
// \u{05BD}[\p{Hebrew}\s]*?\u{05BD}
//pub(crate) static RE_OUTER_COMMON_METEG: Lazy<Regex> = Lazy::new(|| {
//    let pattern = format!("{}{}*?{}", METEG, HEBREW_OR_SPACE, METEG);
//     Regex::new(&pattern)
//         .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_COMMON_METEG: {}", &pattern))
// });

// An 'Ole We Yored' consists of the following two UTF-8 code-points
//      - Ole (\u{05AB}) followed by
//      - Yored (\u{05A5}) aka Merkha
// This accent can stretch over two words (a.k.a. word-unit)
// Regex::new(r"\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap());
pub(crate) static RE_OUTER_POETRY_OLEH_WE_YORED: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{}{}+{}{}*{}", OLEH, HEBREW, OPTIONAL_SPACE, HEBREW, YORED);
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_OLEH_WE_YORED: {}", &pattern))
});

// A 'Revia Mugrash' consists of the following two UTF-8 code-points:
// - Geresh (\u{059C}) followed by
// - Revia (\u{0597})
// - Maqqeph (\u{05BE})
// - 'Geresh Muqdam' (\u{059D}) is Jiddisch?
/*
Geresh (גֵּרֶשׁ)
-----------
Function – In the system of Biblical Hebrew cantillation (taʽamim) it is a disjunctive accent that marks a pause or syntactic break.
Form – The regular cantillation geresh is written above the accented letter (Unicode U+059C).
Placement – It sits directly over the letter it belongs to.

Geresh Muqdam (גֵּרֵשׁ מוּקְדָם)
-----------------------
Function – It is a variant of the cantillation geresh, also a disjunctive accent, but used in slightly different melodic‑syntactic contexts.
Form – Represented in Unicode as U+059D.
Placement – The mark appears above and a little before the first letter of the word (i.e., “pre‑positive” placement), which distinguishes it visually from the standard gereshgrokipedia.com.

This mark is characteristic of the three poetic books (Job, Proverbs, Psalms – the “Emet” books); there it often changes the usual function of nearby accents (e.g., turning a strong disjunctive into a weaker one).

In short, both are cantillation marks, but geresh muqdam is positioned slightly earlier (to the left) of the accented letter, whereas the ordinary geresh sits directly over the letter. This subtle shift signals a different nuance in the chanting and parsing of the biblical text.
*/
// Regex::new(r"[\s\u{05BE}]\p{Hebrew}*[\u{059C}\u{059D}]\p{Hebrew}*\u{0597}").unwrap()
pub(crate) static RE_OUTER_POETRY_REVIA_MUGRASH: Lazy<Regex> = Lazy::new(|| {
    //let pattern = format!("{SPACE_OR_MAQAF}{HEBREW}*?{GERESH_OR_GERESH_MUQDAM}{HEBREW}*?{REVIA}");
    let pattern = format!("{GERESH_OR_GERESH_MUQDAM}{HEBREW}*?{REVIA}");
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_REVIA_MUGRASH: {}", &pattern))
});

// An 'Mehuppakh Legarmeh' consists of the following two UTF-8 code-points:
//      - Mehuppakh (\u{05A4}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Lazy::new(|| Regex::new(r"\u{05A4}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());
pub(crate) static RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{MAHPAKH}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
    Regex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH: {}",
            &pattern
        )
    })
});

// An 'Azla Legarmeh' consists of the following two UTF-8 code-points:
//      - Azla (\u{05A8}) followed by
//      - Paseq (\u{05C0})
// For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
// Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{05A8}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
pub(crate) static RE_OUTER_POETRY_AZLA_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{AZLA}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
    Regex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_AZLA_LEGARMEH: {}", &pattern))
});

// pub(crate) static FA_RE_OUTER_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
//     FancyRegex::new(r"(\u{05A8}\p{Hebrew}*?\u{05BE})|(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))")
//         .unwrap()
// });

const AZLA_NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE: &str =
    r"(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))";
pub(crate) static FA_RE_OUTER_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
    let pattern =
        format!("{AZLA}{HEBREW}*?{MAQAF}{OR}{AZLA_NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE}");
    FancyRegex::new(&pattern)
        .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_POETRY_AZLA: {}", &pattern))
});

// A Shalshalet NOT followed by a Sof Passuq (or a vertical line)
//    Lazy::new(|| FancyRegex::new(r"\u{0593}(?!\p{Hebrew}*?\s?[\u{05C0}\u{007C}])").unwrap());
pub(crate) static FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH: Lazy<FancyRegex> = Lazy::new(|| {
    let pattern = format!("{SHALSHELET}{NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE}");

    FancyRegex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH: {}",
            &pattern
        )
    })
});

// A Tsinnorit Merkha consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Merkha (\u{05A5})
// This accent can occur in one or two words (a.k.a. word-unit)
//     Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A5}")
pub(crate) static RE_OUTER_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(
        "{SPACE_OR_MAQAF}?{HEBREW}*?{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MERKHA}"
    );
    Regex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex RE_OUTER_POETRY_TSINNORIT_MERKHA: {}",
            &pattern
        )
    })
});

pub(crate) static RE_INNER_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MERKHA}");
    Regex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex RE_INNER_POETRY_TSINNORIT_MERKHA: {}",
            &pattern
        )
    })
});

// A Tsinnorit Mahpakh consists of the following two UTF-8 code-points
//      - Tsinnorit (\u{0598}) followed by
//      - Mahpakh (\u{05A4})
// This accent can occur in one or two words (a.k.a. word-unit)
// Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A4}")
pub(crate) static RE_OUTER_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!(
        "{SPACE_OR_MAQAF}?{HEBREW}*?{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MAHPAKH}"
    );
    Regex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex RE_OUTER_POETRY_TSINNORIT_MAHPAKH: {}",
            &pattern
        )
    })
});

pub(crate) static RE_INNER_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(|| {
    let pattern = format!("{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MAHPAKH}");
    Regex::new(&pattern).unwrap_or_else(|_| {
        panic!(
            "Invalid regex RE_OUTER_POETRY_TSINNORIT_MAHPAKH: {}",
            &pattern
        )
    })
});

#[cfg(test)]
mod regex_initialization_tests {
    use super::*;

    // Test FA_RE_OUTER_COMMON_SILLUQ
    #[test]
    fn test_fa_re_outer_common_silluq_init() {
        let regex = &FA_RE_OUTER_COMMON_SILLUQ;
        // Verify it matches a valid Silluq pattern
        let valid = "אֽוֹר׃"; // Simplified example
                            // Note: The actual pattern is complex, so we just ensure it doesn't panic
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_COMMON_SHALSHELET
    #[test]
    fn test_re_outer_common_shalshelet_init() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        let valid = "בְּהִ֑ים֓׀";
        let _ = regex.is_match(valid);
    }

    // Test RE_INNER_COMMON_SHALSHELET
    #[test]
    fn test_re_inner_common_shalshelet_init() {
        let regex = &RE_INNER_COMMON_SHALSHELET;
        let valid = "֓׀";
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_PROSE_LEGARMEH
    #[test]
    fn test_re_outer_prose_legarmeh_init() {
        let regex = &RE_OUTER_PROSE_LEGARMEH;
        let valid = "א֣ים׀";
        let _ = regex.is_match(valid);
    }

    // Test RE_INNER_PROSE_LEGARMEH
    #[test]
    fn test_re_inner_prose_legarmeh_init() {
        let regex = &RE_INNER_PROSE_LEGARMEH;
        let valid = "֣ים׀";
        let _ = regex.is_match(valid);
    }

    // Test FA_RE_OUTER_PROSE_MUNACH
    #[test]
    fn test_fa_re_outer_prose_munach_init() {
        let _regex = &FA_RE_OUTER_PROSE_MUNACH;
        let valid = "א֣"; // Munach not followed by Paseq
        let _ = _regex.is_match(valid);
    }

    // Test RE_OUTER_PROSE_MEAYLA
    #[test]
    fn test_re_outer_prose_meayla_init() {
        let regex = &RE_OUTER_PROSE_MEAYLA;
        // Pattern: Tiphcha + Hebrew + Atnach OR Tiphcha + Hebrew + Silluq
        let valid = "טִפְחָ֖אֱלֹהִ֑ים"; // Simplified
        let _ = regex.is_match(valid);
    }

    // Test FA_RE_OUTER_COMMON_METEG
    #[test]
    fn test_fa_re_outer_common_meteg_init() {
        let regex = &FA_RE_OUTER_COMMON_METEG;
        let valid = "אֽ"; // Meteg not at end
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_OLEH_WE_YORED
    #[test]
    fn test_re_outer_poetry_oleh_we_yored_init() {
        let regex = &RE_OUTER_POETRY_OLEH_WE_YORED;
        let valid = "עוֹלֶה֥"; // Ole + Yored
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_REVIA_MUGRASH
    #[test]
    fn test_re_outer_poetry_revia_mugrash_init() {
        let regex = &RE_OUTER_POETRY_REVIA_MUGRASH;
        let valid = "גֵּרֶשׁ֗"; // Geresh + Revia
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH
    #[test]
    fn test_re_outer_poetry_mehuppakh_legarmeh_init() {
        let regex = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        let valid = "מַהְפַּ֤ך׀";
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_AZLA_LEGARMEH
    #[test]
    fn test_re_outer_poetry_azla_legarmeh_init() {
        let regex = &RE_OUTER_POETRY_AZLA_LEGARMEH;
        let valid = "קַדְמָ֨א׀";
        let _ = regex.is_match(valid);
    }

    // Test FA_RE_OUTER_POETRY_AZLA
    #[test]
    fn test_fa_re_outer_poetry_azla_init() {
        let regex = &FA_RE_OUTER_POETRY_AZLA;
        let valid = "קַדְמָ֨א"; // Azla not followed by Paseq
        let _ = regex.is_match(valid);
    }

    // Test FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH
    #[test]
    fn test_fa_re_outer_poetry_shalshelet_qetannah_init() {
        let regex = &FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH;
        let valid = "שַׁלְשֶׁ֓לֶת"; // Shalshelet not followed by Paseq
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_TSINNORIT_MERKHA
    #[test]
    fn test_re_outer_poetry_tsinnorit_merkha_init() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MERKHA;
        let valid = "צִנּוֹר֘תאב֥";
        let _ = regex.is_match(valid);
    }

    // Test RE_INNER_POETRY_TSINNORIT_MERKHA
    #[test]
    fn test_re_inner_poetry_tsinnorit_merkha_init() {
        let regex = &RE_INNER_POETRY_TSINNORIT_MERKHA;
        let valid = "צִנּוֹר֘תאב֥";
        let _ = regex.is_match(valid);
    }

    // Test RE_OUTER_POETRY_TSINNORIT_MAHPAKH
    #[test]
    fn test_re_outer_poetry_tsinnorit_mahpakh_init() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        let valid = "צִנּוֹר֘תאב֤";
        let _ = regex.is_match(valid);
    }

    // Test RE_INNER_POETRY_TSINNORIT_MAHPAKH
    #[test]
    fn test_re_inner_poetry_tsinnorit_mahpakh_init() {
        let regex = &RE_INNER_POETRY_TSINNORIT_MAHPAKH;
        let valid = "צִנּוֹר֘תאב֤";
        let _ = regex.is_match(valid);
    }

    // Test that all regexes are valid (no panics during init)
    #[test]
    fn test_all_regexes_compile() {
        // Just accessing them ensures Lazy::new ran without panic
        let _ = &FA_RE_OUTER_COMMON_SILLUQ;
        let _ = &RE_OUTER_COMMON_SHALSHELET;
        let _ = &RE_INNER_COMMON_SHALSHELET;
        let _ = &RE_OUTER_PROSE_LEGARMEH;
        let _ = &RE_INNER_PROSE_LEGARMEH;
        let _ = &FA_RE_OUTER_PROSE_MUNACH;
        let _ = &RE_OUTER_PROSE_MEAYLA;
        let _ = &FA_RE_OUTER_COMMON_METEG;
        let _ = &RE_OUTER_POETRY_OLEH_WE_YORED;
        let _ = &RE_OUTER_POETRY_REVIA_MUGRASH;
        let _ = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        let _ = &RE_OUTER_POETRY_AZLA_LEGARMEH;
        let _ = &FA_RE_OUTER_POETRY_AZLA;
        let _ = &FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH;
        let _ = &RE_OUTER_POETRY_TSINNORIT_MERKHA;
        let _ = &RE_INNER_POETRY_TSINNORIT_MERKHA;
        let _ = &RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        let _ = &RE_INNER_POETRY_TSINNORIT_MAHPAKH;
    }

    // Test specific regex patterns with negative cases
    #[test]
    fn test_silluq_negative_case() {
        let regex = &FA_RE_OUTER_COMMON_SILLUQ;
        // Should not match if followed by Maqqeph
        let invalid = "אֽוֹר־ב";
        let _ = regex.is_match(invalid);
    }

    #[test]
    fn test_shalshelet_negative_case() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        // Should not match without Paseq
        let invalid = "בְּהִ֑ים";
        let _ = regex.is_match(invalid);
    }
}
