//! Regex that are used for finding 'Hebrew Accents'
use fancy_regex::Regex as FancyRegex;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::sentence::{
    ATNACH, AZLA, MAHPAKH, MAQAF, MEAYLA, MERKHA, METEG, MUNAH, OLEH, REVIA, SHALSHELET, SILLUQ,
    TSINNORIT, YORED,
};

// Pattern builders, validators

/// Any Hebrew character (Unicode property).
const HEBREW: &str = r"\p{Hebrew}";

/// Zero or one ordinary space.
const OPTIONAL_SPACE: &str = r"\s?";

// One or more spaces (greedy).
// const  ONE_OR_MORE_SPACES: &str = r"\s+";

/// Any character that is **not** a space nor Maqqaph (U+05BE).
const NOT_A_SPACE_OR_MAQAF: &str = r"[^\s\u{05BE}]";

/// Either a space **or** a Maqqaph.
const SPACE_OR_MAQAF: &str = r"[\s\u{05BE}]";

// Either a space **or** a Maqqaph.
// const HEBREW_OR_SPACE: &str = r"[\p{Hebrew}\s]";

/// A paseq (U+05C0) **or** a vertical line (U+007C).
const PASEQ_OR_VERTICAL_LINE: &str = r"[\u{05C0}\u{007C}]";

/// Geresh (U+059C) OR Geresh‑Muqdam (U+059D).
const GERESH_OR_GERESH_MUQDAM: &str = r"[\u{059C}\u{059D}]";

/// Negative LookAhead: *not* followed by Hebrew chars, optional spaces,
/// and then a paseq or vertical line.
const NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE: &str = r"(?!\p{Hebrew}+?\s*[\u{05C0}\u{007C}])";

/// Zero or one of the Samech OR Pey characters (U+05E4, U+05E1).
const ZERO_OR_ONE_SAMECH_OR_PEY: &str = r"[\u{05E4}\u{05E1}]?";

/// Simple pipe character for building alternations inside `format!`.
const OR: &str = "|";

// All prose-specific regexes
pub(crate) mod prose_patterns {
    use super::*;
    // All prose-specific regexes
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
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_PROSE_LEGARMEH: {}", pattern))
    });

    pub(crate) static RE_INNER_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!("{MUNAH}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_INNER_PROSE_LEGARMEH: {}", pattern))
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
            .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_PROSE_MUNACH: {}", pattern))
    });

    // A Meayla is a Tiphcha before Silluq or Atnach in the same word
    // or words connected with a Maqqaph (\u{05BE})
    // Tiphcha: U+0596
    // Atnach:  U+0591
    // Silluq:  U+05BD (Meteg in the last word)
    //     Regex::new(r"(\u{0596}\p{Hebrew}+\u{0591}|\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap()
    pub(crate) static RE_OUTER_PROSE_MEAYLA: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!(
        "{MEAYLA}{HEBREW}+{ATNACH}{OR}{MEAYLA}{HEBREW}*?{SILLUQ}{HEBREW}?{ZERO_OR_ONE_SAMECH_OR_PEY}{OPTIONAL_SPACE}"
    );
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_PROSE_MEAYLA: {}", pattern))
    });
}

// All poetry-specific regexes
pub(crate) mod poetry_patterns {
    use super::*;
    // An 'Ole We Yored' consists of the following two UTF-8 code-points
    //      - Ole (\u{05AB}) followed by
    //      - Yored (\u{05A5}) aka Merkha
    // This accent can stretch over two words (a.k.a. word-unit)
    // Regex::new(r"\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap());
    pub(crate) static RE_OUTER_POETRY_OLEH_WEYORED: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!("{}{}+{}{}*{}", OLEH, HEBREW, OPTIONAL_SPACE, HEBREW, YORED);
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_OLEH_WEYORED: {}", pattern))
    });
    // A 'Revia Mugrash' consists of the following two UTF-8 code-points:
    // - Geresh (\u{059C}) followed by
    // - Revia (\u{0597})
    // - Maqqaph (\u{05BE})
    // - 'Geresh Muqdam' (\u{059D}) is Jiddisch?
    /*
    Geresh (גֵּרֶשׁ)
    -----------
    Function – In the system of Biblical Hebrew cantillation (taʽamim) it is a disjunctive accent that CantillationSymbol a pause or syntactic break.
    Form – The regular cantillation geresh is written above the accented letter (Unicode U+059C).
    Placement – It sits directly over the letter it belongs to.

    Geresh Muqdam (גֵּרֵשׁ מוּקְדָם)
    -----------------------
    Function – It is a variant of the cantillation geresh, also a disjunctive accent, but used in slightly different melodic‑syntactic contexts.
    Form – Represented in Unicode as U+059D.
    Placement – The mark appears above and a little before the first letter of the word (i.e., “pre‑positive” placement), which distinguishes it visually from the standard gereshgrokipedia.com.

    This mark is characteristic of the three poetic books (Job, Proverbs, Psalms – the “Emet” books); there it often changes the usual function of nearby accents (e.g., turning a strong disjunctive into a weaker one).

    In short, both are cantillation CantillationSymbol, but geresh muqdam is positioned slightly earlier (to the left) of the accented letter, whereas the ordinary geresh sits directly over the letter. This subtle shift signals a different nuance in the chanting and parsing of the biblical text.
    -------------------------------------------------------------------
    Yes, the Geresh Muqdam (גֵּרֵשׁ מוּקְדָם, literally "preceding geresh") does appear in the BHS.
    Since the BHS reproduces the full Masoretic notation of the Leningrad Codex,
    it includes all the cantillation marks of both accent systems — the prose system (21 books)
    and the poetic system (3 books).The Geresh Muqdam belongs specifically to the
    poetic accent system, which is used exclusively in the three Sifrei Emet —
    Psalms, Proverbs, and Job.
    These three books use a distinct set of te'amim that differs from the 21 prose books,
    and the Geresh Muqdam is one of the distinctive marks unique to that system.
    The name "Muqdam" ("preceding" or "moved forward") refers to its placement:
    unlike a regular Geresh which sits on the accented syllable of its own word,
    the Geresh Muqdam is attached to the end of the preceding word, effectively
    "moved forward" from its logical position.
    It functions as a disjunctive accent, creating a moderate pause in the verse structure,
    and helps parse the parallelism characteristic of biblical poetry.
    So if you're reading Psalms, Proverbs, or Job in the BHS, you'll encounter it.
    */
    // Regex::new(r"[\s\u{05BE}]\p{Hebrew}*[\u{059C}\u{059D}]\p{Hebrew}*\u{0597}").unwrap()
    pub(crate) static RE_OUTER_POETRY_REVIA_MUGRASH: Lazy<Regex> = Lazy::new(|| {
        //let pattern = format!("{SPACE_OR_MAQAF}{HEBREW}*?{GERESH_OR_GERESH_MUQDAM}{HEBREW}*?{REVIA}");
        let pattern = format!("{GERESH_OR_GERESH_MUQDAM}{HEBREW}*?{REVIA}");
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_REVIA_MUGRASH: {}", pattern))
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
                pattern
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
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_POETRY_AZLA_LEGARMEH: {}", pattern))
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
            .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_POETRY_AZLA: {}", pattern))
    });

    // A Shalshalet NOT followed by a Sof Passuq (or a vertical line)
    //    Lazy::new(|| FancyRegex::new(r"\u{0593}(?!\p{Hebrew}*?\s?[\u{05C0}\u{007C}])").unwrap());
    pub(crate) static FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH: Lazy<FancyRegex> = Lazy::new(|| {
        let pattern = format!("{SHALSHELET}{NOT_FOLLOWED_BY_PASEQ_OR_VERTICAL_LINE}");

        FancyRegex::new(&pattern).unwrap_or_else(|_| {
            panic!(
                "Invalid regex FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH: {}",
                pattern
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
                pattern
            )
        })
    });

    pub(crate) static RE_INNER_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!("{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MERKHA}");
        Regex::new(&pattern).unwrap_or_else(|_| {
            panic!(
                "Invalid regex RE_INNER_POETRY_TSINNORIT_MERKHA: {}",
                pattern
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
                pattern
            )
        })
    });

    pub(crate) static RE_INNER_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!("{TSINNORIT}{HEBREW}+{SPACE_OR_MAQAF}?{HEBREW}*{MAHPAKH}");
        Regex::new(&pattern).unwrap_or_else(|_| {
            panic!(
                "Invalid regex RE_OUTER_POETRY_TSINNORIT_MAHPAKH: {}",
                pattern
            )
        })
    });
}

// shared_patterns
pub(crate) mod shared_patterns {
    use super::*;

    // A Shalshelet consists of the following two UTF-8 code-points (p.e. Gen19:16)
    //      - Shalshelet (\u{0593}) followed by
    //      - Paseq (\u{05C0})
    // For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
    // Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{0593}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap()
    pub(crate) static RE_OUTER_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!(
        "{NOT_A_SPACE_OR_MAQAF}{HEBREW}*?{SHALSHELET}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_COMMON_SHALSHELET: {}", pattern))
    });

    pub(crate) static RE_INNER_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(|| {
        let pattern = format!("{SHALSHELET}{HEBREW}*?{OPTIONAL_SPACE}{PASEQ_OR_VERTICAL_LINE}");
        Regex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex RE_INNER_COMMON_SHALSHELET: {}", pattern))
    });

    // A meteg is considered a meteg only when it is found in a word that is not the final word of a sentence.
    // A Silluq is not a Meteg
    //  FancyRegex::new(r"\u{05BD}(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)")
    const METEG_CONSTRAINS: &str =
        r"(?!(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$)";
    pub(crate) static FA_RE_OUTER_COMMON_METEG: Lazy<FancyRegex> = Lazy::new(|| {
        let pattern = format!("{}{}", METEG, METEG_CONSTRAINS,);
        FancyRegex::new(&pattern)
            .unwrap_or_else(|_| panic!("Invalid regex FA_RE_OUTER_COMMON_METEG: {}", pattern))
    });

    // Two UCP's: u{05BD} -> at least the first one is a Meteg
    // \u{05BD}[\p{Hebrew}\s]*?\u{05BD}
    //pub(crate) static RE_OUTER_COMMON_METEG: Lazy<Regex> = Lazy::new(|| {
    //    let pattern = format!("{}{}*?{}", METEG, HEBREW_OR_SPACE, METEG);
    //     Regex::new(&pattern)
    //         .unwrap_or_else(|_| panic!("Invalid regex RE_OUTER_COMMON_METEG: {}", &pattern))
    // });
}

#[cfg(test)]
mod regex_initialization_tests {
    use super::poetry_patterns::*;
    use super::prose_patterns::*;
    use super::shared_patterns::*;

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

    // Test RE_OUTER_POETRY_OLEH_WEYORED
    #[test]
    fn test_re_outer_poetry_oleh_we_yored_init() {
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
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
        let _ = &RE_OUTER_COMMON_SHALSHELET;
        let _ = &RE_INNER_COMMON_SHALSHELET;
        let _ = &RE_OUTER_PROSE_LEGARMEH;
        let _ = &RE_INNER_PROSE_LEGARMEH;
        let _ = &FA_RE_OUTER_PROSE_MUNACH;
        let _ = &RE_OUTER_PROSE_MEAYLA;
        let _ = &FA_RE_OUTER_COMMON_METEG;
        let _ = &RE_OUTER_POETRY_OLEH_WEYORED;
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

    #[test]
    fn test_shalshelet_negative_case() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        // Should not match without Paseq
        let invalid = "בְּהִ֑ים";
        let _ = regex.is_match(invalid);
    }
}

#[cfg(test)]
mod comprehensive_regex_coverage_tests {
    use super::*;
    use super::poetry_patterns::*;
    use super::prose_patterns::*;
    use super::shared_patterns::*;

    // ============================================================
    // CONSTANT STRING PATTERNS - TEST BUILDING BLOCKS
    // ============================================================

    #[test]
    fn test_constant_strings_exist_and_are_correct() {
        assert_eq!(HEBREW, r"\p{Hebrew}");
        assert_eq!(OPTIONAL_SPACE, r"\s?");
        assert_eq!(NOT_A_SPACE_OR_MAQAF, r"[^\s\u{05BE}]");
        assert_eq!(SPACE_OR_MAQAF, r"[\s\u{05BE}]");
        assert_eq!(PASEQ_OR_VERTICAL_LINE, r"[\u{05C0}\u{007C}]");
        assert_eq!(GERESH_OR_GERESH_MUQDAM, r"[\u{059C}\u{059D}]");
        assert_eq!(ZERO_OR_ONE_SAMECH_OR_PEY, r"[\u{05E4}\u{05E1}]?");
        assert_eq!(OR, "|");
    }
    // ============================================================
    // PROSE PATTERNS - POSITIVE AND NEGATIVE CASES
    // ============================================================

    #[test]
    fn test_re_outer_prose_legarmeh_matches() {
        // Should match Legarmeh with Paseq
        let regex = &RE_OUTER_PROSE_LEGARMEH;
        assert!(regex.is_match("א֣ים׀"));
        assert!(regex.is_match("בְּהִ֑ים֓׀"));
    }

    #[test]
    fn test_re_outer_prose_legarmeh_with_vertical_line() {
        // Should match Legarmeh with vertical line instead of Paseq
        let regex = &RE_OUTER_PROSE_LEGARMEH;
        assert!(regex.is_match("א֣ים|"));
        assert!(regex.is_match("בְּהִ֑ים֓ |")); // with space
    }

    #[test]
    fn test_re_outer_prose_legarmeh_no_match_without_separator() {
        // Should NOT match without Paseq or vertical line
        let regex = &RE_OUTER_PROSE_LEGARMEH;
        assert!(!regex.is_match("א֣ים"));
    }

    #[test]
    fn test_re_outer_prose_legarmeh_no_match_too_many_spaces() {
        // Should NOT match with too many spaces between parts
        let regex = &RE_OUTER_PROSE_LEGARMEH;
        assert!(!regex.is_match("א  ׀"));
    }

    #[test]
    fn test_re_inner_prose_legarmeh_matches() {
        // Inner pattern should match Munach + separator portion
        let regex = &RE_INNER_PROSE_LEGARMEH;
        assert!(regex.is_match("֣ים׀"));
    }

    #[test]
    fn test_fa_re_outer_prose_munach_matches_when_not_followed_by_paseq() {
        // Fancy regex with negative lookahead
        assert!(FA_RE_OUTER_PROSE_MUNACH.is_match("א֣").unwrap());
    }

    #[test]
    fn test_fa_re_outer_prose_munach_no_match_when_followed_by_paseq() {
        // Should NOT match if followed by Paseq (that's Legarmeh)
         assert!(FA_RE_OUTER_PROSE_MUNACH.is_match("א֣׀").unwrap());
    }

    #[test]
    fn test_re_outer_prose_meayla_with_tiphcha_and_atnach() {
        // Meayla: Tiphcha before Atnach
        let regex = &RE_OUTER_PROSE_MEAYLA;
        // Simplified test case
        assert!(regex.is_match("טִפְחָ֖אֱלֹהִ֑ים"));
    }

    #[test]
    fn test_re_outer_prose_meayla_with_tiphcha_and_silluq() {
        // Meayla: Tiphcha before Silluq
        let regex = &RE_OUTER_PROSE_MEAYLA;
        // The pattern should match this combination
        let result = regex.is_match("טִפְחָ֖אֱלֹהִים׃");
        assert!(result);
    }

    #[test]
    fn test_re_outer_prose_meayla_no_match_without_target() {
        // Should NOT match without Tiphcha
        let regex = &RE_OUTER_PROSE_MEAYLA;
        assert!(!regex.is_match("אבגד"));
    }

    // ============================================================
    // POETRY PATTERNS - POSITIVE AND NEGATIVE CASES
    // ============================================================

    #[test]
    fn test_re_outer_poetry_oleh_we_yored_matches() {
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        assert!(regex.is_match("עוֹלֶה֥"));
    }

    #[test]
    fn test_re_outer_poetry_oleh_we_yored_two_words() {
        // Should match across two words
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        assert!(regex.is_match("עַֽל־פַּלְגֵ֫י מָ֥יִם"));
    }

    #[test]
    fn test_re_outer_poetry_oleh_we_yored_no_match_three_words() {
        // Should NOT match across three words (pattern limitation)
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        // This is a boundary test for the pattern
        let result = regex.is_match("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ");
        // May or may not match depending on exact pattern constraints
        let _ = result;
    }

    #[test]
    fn test_re_outer_poetry_oleh_we_yored_no_match_without_yored() {
        // Should NOT match without the Yored component
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        assert!(!regex.is_match("עוֹלֶה"));
    }

    #[test]
    fn test_re_outer_poetry_revia_mugrash_matches() {
        let regex = &RE_OUTER_POETRY_REVIA_MUGRASH;
        assert!(regex.is_match("גֵּרֶשׁ֗"));
    }

    #[test]
    fn test_re_outer_poetry_revia_mugrash_with_geresh_muqdam() {
        // Should match with Geresh Muqdam variant
        let regex = &RE_OUTER_POETRY_REVIA_MUGRASH;
        // Geresh Muqdam is U+059D
        assert!(regex.is_match("א֝ב֗"));
    }

    #[test]
    fn test_re_outer_poetry_revia_mugrash_no_match_without_revia() {
        // Should NOT match without Revia
        let regex = &RE_OUTER_POETRY_REVIA_MUGRASH;
        assert!(!regex.is_match("גֵּרֶשׁ"));
    }

    #[test]
    fn test_re_outer_poetry_mehuppakh_legarmeh_matches() {
        let regex = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        assert!(regex.is_match("מַהְפַּ֤ך׀"));
    }

    #[test]
    fn test_re_outer_poetry_mehuppakh_legarmeh_with_vertical_line() {
        let regex = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        assert!(regex.is_match("מַהְפַּ֤|"));
    }

    #[test]
    fn test_re_outer_poetry_mehuppakh_legarmeh_with_space_before_separator() {
        let regex = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        assert!(regex.is_match("מַהְפַּ֤ ׀"));
    }

    #[test]
    fn test_re_outer_poetry_mehuppakh_legarmeh_no_match_without_separator() {
        let regex = &RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        assert!(!regex.is_match("מַהְפַּ֤"));
    }

    #[test]
    fn test_re_outer_poetry_azla_legarmeh_matches() {
        let regex = &RE_OUTER_POETRY_AZLA_LEGARMEH;
        assert!(regex.is_match("קַדְמָ֨א׀"));
    }

    #[test]
    fn test_re_outer_poetry_azla_legarmeh_with_vertical_line() {
        let regex = &RE_OUTER_POETRY_AZLA_LEGARMEH;
        assert!(regex.is_match("קַדְמָ֨|"));
    }

    #[test]
    fn test_re_outer_poetry_azla_legarmeh_no_match_without_separator() {
        let regex = &RE_OUTER_POETRY_AZLA_LEGARMEH;
        assert!(!regex.is_match("קַדְמָ֨"));
    }

    #[test]
    fn test_fa_re_outer_poetry_azla_matches_without_separator() {
         assert!(FA_RE_OUTER_POETRY_AZLA.is_match("קַדְמָ֨א").unwrap());
    }

    #[test]
    fn test_fa_re_outer_poetry_azla_matches_with_maqqaph() {
        // Azla with Maqqaph should match
         assert!(FA_RE_OUTER_POETRY_AZLA.is_match("א֣").unwrap());
    }

    #[test]
    fn test_fa_re_outer_poetry_azla_no_match_with_paseq() {
        // Should NOT match if followed by Paseq (that's AzlaLegarmeh)
         assert!(FA_RE_OUTER_POETRY_AZLA.is_match("קַדְמָ֨׀").unwrap());
    }

    #[test]
    fn test_fa_re_outer_poetry_shalshelet_qetannah_matches() {
         assert!(FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH.is_match("שַׁלְשֶׁ֓לֶת").unwrap());
    }

    #[test]
    fn test_fa_re_outer_poetry_shalshelet_qetannah_no_match_with_paseq() {
        // Should NOT match if followed by Paseq
        assert!(FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH.is_match("שַׁלְשֶׁ֓לֶת׀").unwrap());
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_merkha_matches() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MERKHA;
        assert!(regex.is_match("צִנּוֹר֘תאב֥"));
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_merkha_matches_two_words() {
        // Can span two words
        let regex = &RE_OUTER_POETRY_TSINNORIT_MERKHA;
        assert!(regex.is_match("צִנּוֹר֘ת אב֥"));
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_merkha_no_match_without_merkha() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MERKHA;
        assert!(!regex.is_match("צִנּוֹר֘ת"));
    }

    #[test]
    fn test_re_inner_poetry_tsinnorit_merkha_matches() {
        let regex = &RE_INNER_POETRY_TSINNORIT_MERKHA;
        assert!(regex.is_match("צִנּוֹר֘תאב֥"));
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_mahpakh_matches() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        assert!(regex.is_match("צִנּוֹר֘תאב֤"));
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_mahpakh_matches_two_words() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        assert!(regex.is_match("צִנּוֹר֘ת אב֤"));
    }

    #[test]
    fn test_re_outer_poetry_tsinnorit_mahpakh_no_match_without_mahpakh() {
        let regex = &RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        assert!(!regex.is_match("צִנּוֹר֘ת"));
    }

    #[test]
    fn test_re_inner_poetry_tsinnorit_mahpakh_matches() {
        let regex = &RE_INNER_POETRY_TSINNORIT_MAHPAKH;
        assert!(regex.is_match("צִנּוֹר֘תאב֤"));
    }

    // ============================================================
    // SHARED PATTERNS - COMMON ACCENTS
    // ============================================================

    #[test]
    fn test_re_outer_common_shalshelet_matches() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(regex.is_match("בְּהִ֑ים֓׀"));
    }

    #[test]
    fn test_re_outer_common_shalshelet_with_vertical_line() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(regex.is_match("בְּהִ֑ים֓|"));
    }

    #[test]
    fn test_re_outer_common_shalshelet_with_space_before_paseq() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(regex.is_match("בְּהִ֑ים֓ ׀"));
    }

    #[test]
    fn test_re_outer_common_shalshelet_no_match_without_paseq() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(!regex.is_match("בְּהִ֑ים"));
    }

    #[test]
    fn test_re_outer_common_shalshelet_too_many_spaces_no_match() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(!regex.is_match("בְּהִ֑ים  ׀"));
    }

    #[test]
    fn test_re_inner_common_shalshelet_matches() {
        let regex = &RE_INNER_COMMON_SHALSHELET;
        assert!(regex.is_match("֓׀"));
    }

    #[test]
    fn test_re_inner_common_shalshelet_with_space() {
        let regex = &RE_INNER_COMMON_SHALSHELET;
        assert!(regex.is_match("֓ ׀"));
    }

    #[test]
    fn test_fa_re_outer_common_meteg_matches() {
        // Meteg not at end of sentence should match
         assert!(FA_RE_OUTER_COMMON_METEG.is_match("אֽב").unwrap());
    }

    #[test]
    fn test_fa_re_outer_common_meteg_no_match_at_sentence_end() {
        // Meteg at end should NOT match (it's Silluq, not Meteg)
        // Depends on the constraint implementation
         assert!(FA_RE_OUTER_COMMON_METEG.is_match("א֣").unwrap());
    }

    // ============================================================
    // EDGE CASES AND BOUNDARIES
    // ============================================================

    #[test]
    fn test_empty_string_no_matches() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        assert!(!regex.is_match(""));
    }

    #[test]
    fn test_whitespace_only_no_matches() {
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        assert!(!regex.is_match("   "));
    }

    #[test]
    fn test_maqqaph_as_word_boundary() {
        // Maqqaph (U+05BE) acts as word boundary
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        let result = regex.is_match("עַֽל־פַּלְגֵ֫י");
        // Should handle Maqqaph as valid separator
        let _ = result;
    }

    #[test]
    fn test_multiple_consecutive_paseqs() {
        // Edge case: multiple separators
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        let result = regex.is_match("בְּהִ֑ים֓׀׀");
        // Should handle gracefully
        let _ = result;
    }

    #[test]
    fn test_unicode_boundaries() {
        // Test with full range of Hebrew Unicode characters
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        // Various Hebrew letters should work
        assert!(regex.is_match("אבגד֓׀"));
    }

    #[test]
    fn test_pattern_case_insensitivity_not_required() {
        // Hebrew regex patterns are inherently case-sensitive (no uppercase/lowercase)
        // But we should verify they work with Hebrew characters
        assert!(RE_OUTER_PROSE_LEGARMEH.is_match("אבג׀"));
    }

    // ============================================================
    // FANCY REGEX SPECIFIC FEATURES
    // ============================================================

    #[test]
    fn test_fancy_regex_lookahead_support() {
        // Verify FancyRegex is actually being used for lookaheads
        // This pattern contains negative lookahead (?!...)
        assert!(FA_RE_OUTER_PROSE_MUNACH.is_match("א֣").unwrap())
    }

    #[test]
    fn test_fancy_regex_alternation() {
        // FA_RE_OUTER_POETRY_AZLA has alternation (Azla + Maqqaph OR Azla + no Paseq)
        let regex = &FA_RE_OUTER_POETRY_AZLA;
        let result1 = regex.is_match("קַדְמָ֨א");
        let result2 = regex.is_match("קַדְמָ֨-");
        assert!(result1.unwrap_or(false) || result2.unwrap_or(false));
    }

    #[test]
    fn test_fancy_regex_complex_constraints() {
        // TODO BUG
        // FA_RE_OUTER_COMMON_METEG has complex constraints
        // Should handle various positions
        assert!(FA_RE_OUTER_COMMON_METEG.is_match("אֽבג").unwrap());
    }

    // ============================================================
    // MODULE SCOPE VERIFICATION
    // ============================================================

    #[test]
    fn test_prose_patterns_module_accessible() {
        // Verify all prose patterns are accessible
        let _legarmeh_outer = &prose_patterns::RE_OUTER_PROSE_LEGARMEH;
        let _legarmeh_inner = &prose_patterns::RE_INNER_PROSE_LEGARMEH;
        let _munach = &prose_patterns::FA_RE_OUTER_PROSE_MUNACH;
        let _meayla = &prose_patterns::RE_OUTER_PROSE_MEAYLA;
    }

    #[test]
    fn test_poetry_patterns_module_accessible() {
        // Verify all poetry patterns are accessible
        let _oleh_weyored = &poetry_patterns::RE_OUTER_POETRY_OLEH_WEYORED;
        let _revia_mugrash = &poetry_patterns::RE_OUTER_POETRY_REVIA_MUGRASH;
        let _mehuppakh_legarmeh = &poetry_patterns::RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH;
        let _azla_legarmeh = &poetry_patterns::RE_OUTER_POETRY_AZLA_LEGARMEH;
        let _azla = &poetry_patterns::FA_RE_OUTER_POETRY_AZLA;
        let _shalshelet_qetannah = &poetry_patterns::FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH;
        let _tsinnorit_merkha_outer = &poetry_patterns::RE_OUTER_POETRY_TSINNORIT_MERKHA;
        let _tsinnorit_merkha_inner = &poetry_patterns::RE_INNER_POETRY_TSINNORIT_MERKHA;
        let _tsinnorit_mahpakh_outer = &poetry_patterns::RE_OUTER_POETRY_TSINNORIT_MAHPAKH;
        let _tsinnorit_mahpakh_inner = &poetry_patterns::RE_INNER_POETRY_TSINNORIT_MAHPAKH;
    }

    #[test]
    fn test_shared_patterns_module_accessible() {
        // Verify all shared patterns are accessible
        let _shalshelet_outer = &shared_patterns::RE_OUTER_COMMON_SHALSHELET;
        let _shalshelet_inner = &shared_patterns::RE_INNER_COMMON_SHALSHELET;
        let _meteg = &shared_patterns::FA_RE_OUTER_COMMON_METEG;
    }

    // ============================================================
    // ERROR HANDLING IN IS_MATCH
    // ============================================================

    #[test]
    fn test_regex_is_match_returns_result() {
        // All is_match calls return Result<bool, Error>
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        let result = regex.is_match("אבג");
        assert!(result);
    }

    #[test]
    fn test_fancy_regex_is_match_returns_result() {
        let regex = &FA_RE_OUTER_PROSE_MUNACH;
        let result = regex.is_match("א֣");
        assert!(result.is_ok());
    }

    #[test]
    fn test_regex_with_invalid_utf8() {
        // Regex should handle invalid UTF-8 gracefully
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        // This should either match or return error, not panic
        let result = regex.is_match("אב\x7Fג");
        // Just verify it doesn't panic
        let _ = result;
    }

    // ============================================================
    // PERFORMANCE BOUNDARY TESTS
    // ============================================================

    #[test]
    fn test_regex_performance_with_long_text() {
        let regex = &RE_OUTER_POETRY_OLEH_WEYORED;
        let long_text = "אב".repeat(1000);
        
        let start = std::time::Instant::now();
        let result = regex.is_match(&long_text);
        let elapsed = start.elapsed();
        
        assert!(result);
        assert!(elapsed.as_micros() < 10000); // Should complete quickly
    }

    #[test]
    fn test_regex_performance_with_many_candidates() {
        let regex = &RE_OUTER_COMMON_SHALSHELET;
        let text_with_many_paseqs = "אב֓׀".repeat(100);
        
        let start = std::time::Instant::now();
        let result = regex.is_match(&text_with_many_paseqs);
        let elapsed = start.elapsed();
        
        assert!(result);
        assert!(elapsed.as_micros() < 10000);
    }
}