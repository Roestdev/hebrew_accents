//! This file contains all static data of the 'Hebrew Accents'

// Standard library
use once_cell::sync::Lazy;

// External crates
// N/A

// Crate‑internal (local modules)
use crate::accent::{
    AccentCategory, AccentInformation, AccentType, Alternates, CodePoints, PoetryAccent,
    ProseAccent, WordStress,
};
use crate::{accent_codepoints::*, Additional, PseudoAccent};

pub static PROSE_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; ProseAccent::COUNT]> =
    Lazy::new(|| {
        &[
            &SILLUQ_INFO,
            &ATNACH_INFO,
            &SEGOLTA_INFO,
            &SHALSHELET_INFO,
            &ZAQEF_QATON_INFO,
            &ZAQEPH_GADOL_INFO,
            &REVIA_INFO,
            &TIPHCHA_INFO,
            &ZARQA_INFO,
            &PASHTA_INFO,
            &YETIV_INFO,
            &TEVIR_INFO,
            &GERESH_INFO,
            &GERSHAYIM_INFO,
            &PAZER_INFO,
            &PAZER_GADOL_INFO,
            &TELISHA_GEDOLAH_INFO,
            &LEGARMEH_INFO,
            // Conjunctives
            &MUNACH_INFO,
            &MAHPAKH_INFO,
            &MERKHA_INFO,
            &MERKHA_KEFULAH_INFO,
            &DARGA_INFO,
            &AZLA_INFO,
            &TELISHA_QETANNAH_INFO,
            &GALGAL_INFO,
            &MAYELA_INFO,
            &METEG_INFO,
        ]
    });
pub static POETRY_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; PoetryAccent::COUNT]> =
    Lazy::new(|| {
        &[
            &SILLUQ_INFO,
            &OLEH_WE_YORED_INFO,
            &ATNACH_INFO,
            &REVIA_GADOL_INFO,
            &REVIA_MUGRASH_INFO,
            &SHALSHELET_GADOL_INFO,
            &TSINNOR_INFO,
            &REVIA_QATON_INFO,
            &DECHI_INFO,
            &PAZER_INFO,
            &MEHUPPAKH_LEGARMEH_INFO,
            &AZLA_LEGARMEH_INFO,
            // Conjunctives
            &MUNACH_INFO,
            &MERKHA_INFO,
            &ILLUY_INFO,
            &TARCHA_INFO,
            &GALGAL_INFO,
            &MEHUPPAKH_INFO,
            &AZLA_INFO,
            &SHALSHELET_QETANNAH_INFO,
            &TSINNORIT_MERKHA_INFO,
            &TSINNORIT_MAHPAKH_INFO,
            &METEG_INFO,
        ]
    });

pub static PSEUDO_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; PseudoAccent::COUNT]> =
    Lazy::new(|| &[&SOPH_PASUQ_INFO, &MAQQEPH_INFO, &PASEQ_INFO]);

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variant, extend this
/// array accordingly – the `static_assertions` check below will remind you.
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::COUNT] = [
    // ---- Disjunctives ----------------------------------------------------
    /* 0 */
    1, // Silluq
    /* 1 */ 2, // OlehWeYored
    /* 2 */ 3, // Atnach
    /* 3 */ 4, // ReviaGadol
    /* 4 */ 5, // ReviaMugrash
    /* 5 */ 6, // ShalsheletGadol
    /* 6 */ 7, // Tsinnor
    /* 7 */ 8, // ReviaQaton
    /* 8 */ 9, // Dechi
    /* 9 */ 10, // Pazer
    /*10 */ 11, // MehuppakhLegarmeh
    /*11 */ 12, // AzlaLegarmeh
    // ---- Conjunctives ----------------------------------------------------
    /*12 */
    13, // Munach
    /*13 */ 14, // Merkha
    /*14 */ 15, // Illuy
    /*15 */ 16, // Tarcha
    /*16 */ 17, // Galgal
    /*17 */ 18, // Mehuppakh
    /*18 */ 19, // Azla
    /*19 */ 20, // ShalsheletQetannah
    /*20 */ 21, // TsinnoritMerkha (Same rank as TsinnoritMahpakh!)
    /*21 */ 21, // TsinnoritMahpakh (Same rank as TsinnoritMerkha!)
    /*22 */ 22, // Meteg
];

pub const SILLUQ_INFO: AccentInformation = AccentInformation {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    meaning: "close, cessation",
    code_points: CodePoints {
        primary: &CP_SILLUQ,
        secondary: None,
    },
    comment: Some(
        "The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.",
    ),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const ATNACH_INFO: AccentInformation = AccentInformation {
    english_name: "Atnach",
    hebrew_name: "אתְנָח",
    meaning: "a causing to rest",
    code_points: CodePoints {
        primary: &CP_ETNAHTA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const SEGOLTA_INFO: AccentInformation = AccentInformation {
    english_name: "Segolta",
    hebrew_name: "סְגֹולְתָּא",
    meaning: "a little grape-bunch",
    code_points: CodePoints {
        primary: &CP_SEGOL,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PostPositive),
        alternates: None,
    }),
};

pub const SHALSHELET_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    meaning: "chain or link",
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const ZAQEF_QATON_INFO: AccentInformation = AccentInformation {
    english_name: "Zaqeph Qaton",
    hebrew_name: "זָקֵף קָטוֹן",
    meaning: "small upright",
    code_points: CodePoints {
        primary: &CP_ZAQEF_QATAN,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Zaqeph Qatan",
            hebrew_name: "זָקֵף קָטָן",
            meaning: "todo",
        }),
    }),
};

pub const ZAQEPH_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Zaqeph Gadol",
    hebrew_name: "זָקֵף גָּדוֹל",
    meaning: "large upright",
    code_points: CodePoints {
        primary: &CP_ZAQEF_GADOL,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const REVIA_INFO: AccentInformation = AccentInformation {
    english_name: "Revia",
    hebrew_name: "רְבִיעַ",
    meaning: "fourth [in a sequence]",
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: Some("probably due to its four-note tune."),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const TIPHCHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tiphcha", // ADD TARCHA
    hebrew_name: "טִפְחָא",
    meaning: "handbreadth or diagonal",
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: Some("before Atnach and Silluq"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const ZARQA_INFO: AccentInformation = AccentInformation {
    english_name: "Zarqa",
    hebrew_name: "זַרְקָא",
    meaning: "to sprinkle, scatter",
    code_points: CodePoints {
        primary: &CP_ZINOR,
        secondary: None,
    },
    comment: Some("before  Segolta"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PostPositive),
        alternates: None,
    }),
};

pub const PASHTA_INFO: AccentInformation = AccentInformation {
    english_name: "Pashta",
    hebrew_name: "פַּשְׁטָא",
    meaning: "extending, stretching out in length",
    code_points: CodePoints {
        primary: &CP_PASHTA,
        secondary: None,
    },
    comment: Some("if you sound almost last (2 pasta’s in one word)"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PostPositive),
        alternates: None,
    }),
};

pub const YETIV_INFO: AccentInformation = AccentInformation {
    english_name: "Yetiv",
    hebrew_name: "יְתִיב",
    meaning: "resting or sitting",
    code_points: CodePoints {
        primary: &CP_YETIV,
        secondary: None,
    },
    comment: Some("occasionally for a Pashta"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PrePositive),
        alternates: None,
    }),
};

pub const TEVIR_INFO: AccentInformation = AccentInformation {
    english_name: "Tevir",
    hebrew_name: "תְּבִיר",
    meaning: "broken, downward tumble",
    code_points: CodePoints {
        primary: &CP_TEVIR,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const GERESH_INFO: AccentInformation = AccentInformation {
    english_name: "Geresh",
    hebrew_name: "גֵּרֵישׁ",
    meaning: "expulsion, driving out, divorce",
    code_points: CodePoints {
        primary: &CP_GERESH,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Teres",
            hebrew_name: "טֶרֶס",
            meaning: "todo",
        }),
    }),
};

pub const GERSHAYIM_INFO: AccentInformation = AccentInformation {
    english_name: "Gershayim",
    hebrew_name: "גֵּרְשַׁיִם",
    meaning: "double of expulsion, driving out, divorce",
    code_points: CodePoints {
        primary: &CP_GERSHAYIM,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const PAZER_INFO: AccentInformation = AccentInformation {
    english_name: "Pazer",
    hebrew_name: "פָּזֶר",
    meaning: "lavish or scatter",
    code_points: CodePoints {
        primary: &CP_PAZER,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Pazer Qatan",
            hebrew_name: "פָּזֵר קָטָן",
            meaning: "small lavish or scatter",
        }),
    }),
};

pub const PAZER_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Pazer Gadol",
    hebrew_name: "פָּזֶר גּדוֹל",
    meaning: "large lavish or scatter",
    code_points: CodePoints {
        primary: &CP_QARNEY_PARA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Qarne Pharah",
            hebrew_name: "קַרְנֵי פָרָה",
            meaning: "horns of a cow",
        }),
    }),
};

pub const TELISHA_GEDOLAH_INFO: AccentInformation = AccentInformation {
    english_name: "Telisha Gedolah",
    hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
    meaning: "great (long) detached",
    code_points: CodePoints {
        primary: &CP_TELISHA_GEDOLA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PrePositive),
        alternates: None,
    }),
};

pub const LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Legarmeh",
    hebrew_name: "לְגַרְמֶהּ",
    meaning: "for or by itself, independant",
    code_points: CodePoints {
        primary: &CP_MUNAH,
        secondary: Some(&CP_PASEQ),
    },
    comment: Some("Munach with Passeq; Before Revia"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Munach Legarmeh",
            hebrew_name: "מוּנַח לְגַרְמֵ֣הּ",
            meaning: "todo",
        }),
    }),
};
// Conjunctives
pub const MUNACH_INFO: AccentInformation = AccentInformation {
    english_name: "Munach",
    hebrew_name: "מוּנַ֣ח",
    meaning: "resting or placed",
    code_points: CodePoints {
        primary: &CP_MUNAH,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const MAHPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Mahpakh",
    hebrew_name: "מַהְפַּךְ",
    meaning: "turning round",
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Mehuppakh",
            hebrew_name: "מְהֻפָּ֤ךְ",
            meaning: "reversed",
        }),
    }),
};

pub const MERKHA_INFO: AccentInformation = AccentInformation {
    english_name: "Merkha",
    hebrew_name: "מֵרְכָא",
    meaning: "lengthener, prolonging",
    code_points: CodePoints {
        primary: &CP_MERKHA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const MERKHA_KEFULAH_INFO: AccentInformation = AccentInformation {
    english_name: "Merkha Kephulah",
    hebrew_name: "מֵרְכָא כְּפוּלָה",
    meaning: "double lengthener",
    code_points: CodePoints {
        primary: &CP_MERKHA_KEFULA,
        secondary: None,
    },
    comment: Some("Merkha duplex"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const DARGA_INFO: AccentInformation = AccentInformation {
    english_name: "Darga",
    hebrew_name: "דַּרְגָּא",
    meaning: "stairstep",
    code_points: CodePoints {
        primary: &CP_DARGA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const AZLA_INFO: AccentInformation = AccentInformation {
    english_name: "Azla",
    hebrew_name: "אַזְלָא",
    meaning: "going on (not pausing), depart",
    code_points: CodePoints {
        primary: &CP_QADMA,
        secondary: None,
    },
    comment: Some("When Geresh: Qadma"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Qadma",
            hebrew_name: "קַדְמָ֨א",
            meaning: "antiquity or a former state",
        }),
    }),
};

pub const TELISHA_QETANNAH_INFO: AccentInformation = AccentInformation {
    english_name: "Telisha Qetannah",
    hebrew_name: "תְּלִישָא קְטַנָּה",
    meaning: "small (short) detached",
    code_points: CodePoints {
        primary: &CP_TELISHA_QETANA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::PostPositive),
        alternates: None,
    }),
};

pub const GALGAL_INFO: AccentInformation = AccentInformation {
    english_name: "Galgal",
    hebrew_name: "גַּלְגַּל",
    meaning: "wheel, circle",
    code_points: CodePoints {
        primary: &CP_YERAH_BEN_YOMO,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Jerach Ben Jomo",
            hebrew_name: "יֵרֶח בֶּן יוֹמוֹ",
            meaning: "moon one day old",
        }),
    }),
};

pub const MAYELA_INFO: AccentInformation = AccentInformation {
    english_name: "Mayela",
    hebrew_name: "מָאיְלָא",
    meaning: "to be raised or elevated",
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq"),
    additional: Some(Additional {
        accent_type: AccentType::Secondary,
        category: AccentCategory::Conjunctive,
        word_stress: None,
        alternates: Some(Alternates {
            english_name: "Meayyela",
            hebrew_name: "מְאַיְּלָא",
            meaning: "todo",
        }),
    }),
};

pub const METEG_INFO: AccentInformation = AccentInformation {
    english_name: "Meteg",
    hebrew_name: "מֶתֶג",
    meaning: "accent or mark",
    code_points: CodePoints {
        primary: &CP_METEG,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Secondary,
        category: AccentCategory::Conjunctive,
        word_stress: None,
        alternates: None,
    }),
};

/*
Shene Pashtin 	 ב֨ב   	U+0599, U+05A8 	שְׁנֵ֨י פַּשְׁטִין֙
Shene Pashtin 	תְּרֵ֨י קַדְמִין֙
Tere qadmin 	(שְׁנֵי) פַּ֨שְׁטִין֙
(Shene) pashtin

 example: Genesis 28:20
 וּשְׁמָרַ֨נִי֙
*/

/********************************************************
 *                          POETRY
 *******************************************************/
// Disjunctives

// SILLUQ see PROSE section above

pub const OLEH_WE_YORED_INFO: AccentInformation = AccentInformation {
    english_name: "Oleh We Yored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    meaning: "ascending and descending",
    code_points: CodePoints {
        primary: &CP_OLE,
        secondary: Some(&CP_MERKHA),
    },
    comment: Some("The primary CodePoint is Mehuppakh, but located above the consonant. It is then called OLE."),    
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

// ATNACH see PROSE section above

pub const REVIA_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Gadol",
    hebrew_name: "רְבִיעַ גּדוֹל",
    meaning: "big fourth",
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const REVIA_MUGRASH_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    meaning: "exiled fourth",
    code_points: CodePoints {
        primary: &CP_GERESH,
        secondary: Some(&CP_REVIA),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const SHALSHELET_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    meaning: "large chain or link",
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const TSINNOR_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnor",
    hebrew_name: "צִנּוֹר",
    meaning: "pipe or tube",
    code_points: CodePoints {
        primary: &CP_ZINOR,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PostPositive),
        alternates: Some(Alternates {
            english_name: "Zarqa",
            hebrew_name: "זַרְקָא",
            meaning: "to sprinkle, scatter",
        }),
    }),
};

pub const REVIA_QATON_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Qaton",
    hebrew_name: "רְבִיעַ קָטוֹן",
    meaning: "small fourth",
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: Some("After that occurs Oleh We Yored"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const DECHI_INFO: AccentInformation = AccentInformation {
    english_name: "Dechi",
    hebrew_name: "דֶּחִי",
    meaning: "to push or drive away",
    code_points: CodePoints {
        primary: &CP_DEHI,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::PrePositive),
        alternates: Some(Alternates {
            english_name: "Tiphcha",
            hebrew_name: "טִפְחָא",
            meaning: "handbreadth or diagonal",
        }),
    }),
};

// PAZER see PROSE section above

pub const MEHUPPAKH_LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    meaning: "reversed to its own",
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const AZLA_LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    meaning: "goes to its own",
    code_points: CodePoints {
        primary: &CP_QADMA,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Disjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

// MUNACH see PROSE section above

// MERCHA see PROSE section above

pub const ILLUY_INFO: AccentInformation = AccentInformation {
    english_name: "Illuy",
    hebrew_name: "עִלּוּי",
    meaning: "elevation or raising",
    code_points: CodePoints {
        primary: &CP_ILUY,
        secondary: None,
    },
    comment: Some("also called Munach superior"),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const TARCHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tarcha",
    hebrew_name: "טַרְחָא",
    meaning: "trouble, difficulty, hardship, toil",
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: Some("it refers to the effort, strain, or inconvenience involved in doing something."),
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

// GALGAL see PROSE section above

pub const MEHUPPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Mehuppakh",
    hebrew_name: "מְהֻפָּ֤ךְ",
    meaning: "reversed",
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: Some(Alternates {
            english_name: "Mahpakh",
            hebrew_name: "מַהְפַּךְ",
            meaning: "turning round",
        }),
    }),
};

// AZLA see PROSE section above

pub const SHALSHELET_QETANNAH_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet Qetannah",
    hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
    meaning: "small chain",
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: None,
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const TSINNORIT_MERKHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    meaning: "pipe of continuation",
    code_points: CodePoints {
        primary: &CP_ZARQA,
        secondary: Some(&CP_MERKHA),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

pub const TSINNORIT_MAHPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    meaning: "pipe of reversal",
    code_points: CodePoints {
        primary: &CP_ZARQA,
        secondary: Some(&CP_MAHAPAKH),
    },
    comment: None,
    additional: Some(Additional {
        accent_type: AccentType::Primary,
        category: AccentCategory::Conjunctive,
        word_stress: Some(WordStress::ImPositive),
        alternates: None,
    }),
};

/********************************************************
 *                      PSEUDO ACCENT
 *******************************************************/
pub const SOPH_PASUQ_INFO: AccentInformation = AccentInformation {
    english_name: "Soph Pasuq",
    hebrew_name: "סוֹף פָּסוּק",
    meaning: "end of verse",
    code_points: CodePoints {
        primary: &CP_SOPH_PASUQ,
        secondary: None,
    },
    comment: Some(
        "it doesn’t carry any theological or interpretive meaning beyond marking a boundary",
    ),
    additional: None,
};

pub const MAQQEPH_INFO: AccentInformation = AccentInformation {
    english_name: "Maqqeph",
    hebrew_name: "מַקֵּף",
    meaning:"binder",
    code_points: CodePoints {
        primary: &CP_MAQAF,
        secondary: None,
    },
    comment: Some("Can link two (or more) short words together, after which they function as a single compound word bearing a single Hebrew accent."),
    additional:  None,
};

pub const PASEQ_INFO: AccentInformation = AccentInformation {
    english_name: "Paseq",
    hebrew_name: "פָּסֵק",
    meaning: "to pause, to stop or to interrupt",
    code_points: CodePoints {
        primary: &CP_PASEQ,
        secondary: None,
    },
    comment: Some(
        "It’s indicating that someone or something is stopping temporarily or creating a pause.",
    ),
    additional: None,
};
