//! This file contains all static data of the 'Hebrew Accents'

// Standard library
use once_cell::sync::Lazy;

// Crate‑internal (local modules)
use crate::{
    accent_codepoints::*, AccentCategory, AccentInformation, AccentType, AlternateNames,
    CantillationSymbol, WordStress,
};
use crate::{PoetryAccent, ProseAccent, PseudoAccent};

pub static PROSE_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; ProseAccent::LEN]> =
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
pub static POETRY_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; PoetryAccent::LEN]> =
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

pub static PSEUDO_ACCENT_TABLE: Lazy<&'static [&'static AccentInformation; PseudoAccent::LEN]> =
    Lazy::new(|| &[&SOPH_PASUQ_INFO, &MAQQEPH_INFO, &PASEQ_INFO]);

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variantextend this
/// array accordingly – the `static_assertions` check below will remind you.
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::LEN] = [
    // ---- Disjunctives -----------------------------------------
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
    /*20 */ 21, // TsinnoritMerkha (Same rank as TsinnoritMahpakh!!!)
    /*21 */ 21, // TsinnoritMahpakh (Same rank as TsinnoritMerkha!!!)
    /*22 */ 22, // Meteg
];

pub(crate) const SILLUQ_INFO: AccentInformation = AccentInformation {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    hebrew_concept: "closecessation",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SILLUQ,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some(
        "The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.",
    ),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const ATNACH_INFO: AccentInformation = AccentInformation {
    english_name: "Atnach",
    hebrew_name: "אתְנָח",
    hebrew_concept: "a causing to rest",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ETNAHTA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const SEGOLTA_INFO: AccentInformation = AccentInformation {
    english_name: "Segolta",
    hebrew_name: "סְגֹולְתָּא",
    hebrew_concept: "a little grape-bunch",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SEGOL,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Post),
};

pub(crate) const SHALSHELET_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    hebrew_concept: "chain or link",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const ZAQEF_QATON_INFO: AccentInformation = AccentInformation {
    english_name: "Zaqeph Qaton",
    hebrew_name: "זָקֵף קָטוֹן",
    hebrew_concept: "small upright",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZAQEF_QATAN,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Zaqeph Qatan",
        hebrew_name: "זָקֵף קָטָן",
        hebrew_concept: "todo",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const ZAQEPH_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Zaqeph Gadol",
    hebrew_name: "זָקֵף גָּדוֹל",
    hebrew_concept: "large upright",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZAQEF_GADOL,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const REVIA_INFO: AccentInformation = AccentInformation {
    english_name: "Revia",
    hebrew_name: "רְבִיעַ",
    hebrew_concept: "fourth [in a sequence]",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("probably due to its four-note tune."),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TIPHCHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tiphcha", // ADD TARCHA
    hebrew_name: "טִפְחָא",
    hebrew_concept: "handbreadth or diagonal",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("before Atnach and Silluq"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const ZARQA_INFO: AccentInformation = AccentInformation {
    english_name: "Zarqa",
    hebrew_name: "זַרְקָא",
    hebrew_concept: "to sprinklescatter",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZINOR,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("before  Segolta"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Post),
};

pub(crate) const PASHTA_INFO: AccentInformation = AccentInformation {
    english_name: "Pashta",
    hebrew_name: "פַּשְׁטָא",
    hebrew_concept: "extendingstretching out in length",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PASHTA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("if you sound almost last (2 pasta’s in one word)"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Post),
};

pub(crate) const YETIV_INFO: AccentInformation = AccentInformation {
    english_name: "Yetiv",
    hebrew_name: "יְתִיב",
    hebrew_concept: "resting or sitting",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_YETIV,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("occasionally for a Pashta"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Pre),
};

pub(crate) const TEVIR_INFO: AccentInformation = AccentInformation {
    english_name: "Tevir",
    hebrew_name: "תְּבִיר",
    hebrew_concept: "brokendownward tumble",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TEVIR,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const GERESH_INFO: AccentInformation = AccentInformation {
    english_name: "Geresh",
    hebrew_name: "גֵּרֵישׁ",
    hebrew_concept: "expulsiondriving outdivorce",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Teres",
        hebrew_name: "טֶרֶס",
        hebrew_concept: "todo",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const GERSHAYIM_INFO: AccentInformation = AccentInformation {
    english_name: "Gershayim",
    hebrew_name: "גֵּרְשַׁיִם",
    hebrew_concept: "double of expulsiondriving outdivorce",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERSHAYIM,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const PAZER_INFO: AccentInformation = AccentInformation {
    english_name: "Pazer",
    hebrew_name: "פָּזֶר",
    hebrew_concept: "lavish or scatter",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PAZER,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Pazer Qatan",
        hebrew_name: "פָּזֵר קָטָן",
        hebrew_concept: "small lavish or scatter",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const PAZER_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Pazer Gadol",
    hebrew_name: "פָּזֶר גּדוֹל",
    hebrew_concept: "large lavish or scatter",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QARNEY_PARA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Qarne Pharah",
        hebrew_name: "קַרְנֵי פָרָה",
        hebrew_concept: "horns of a cow",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TELISHA_GEDOLAH_INFO: AccentInformation = AccentInformation {
    english_name: "Telisha Gedolah",
    hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
    hebrew_concept: "great (long) detached",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TELISHA_GEDOLA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Pre),
};

pub(crate) const LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Legarmeh",
    hebrew_name: "לְגַרְמֶהּ",
    hebrew_concept: "for or by itselfindependent",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MUNAH,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: Some(AlternateNames {
        english_name: "Munach Legarmeh",
        hebrew_name: "מוּנַח לְגַרְמֵ֣הּ",
        hebrew_concept: "todo",
    }),
    notes: Some("Munach with Passeq; Before Revia"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};
// Conjunctives
pub(crate) const MUNACH_INFO: AccentInformation = AccentInformation {
    english_name: "Munach",
    hebrew_name: "מוּנַ֣ח",
    hebrew_concept: "resting or placed",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MUNAH,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const MAHPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Mahpakh",
    hebrew_name: "מַהְפַּךְ",
    hebrew_concept: "turning round",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Mehuppakh",
        hebrew_name: "מְהֻפָּ֤ךְ",
        hebrew_concept: "reversed",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const MERKHA_INFO: AccentInformation = AccentInformation {
    english_name: "Merkha",
    hebrew_name: "מֵרְכָא",
    hebrew_concept: "lengthenerprolonging",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MERKHA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const MERKHA_KEFULAH_INFO: AccentInformation = AccentInformation {
    english_name: "Merkha Kephulah",
    hebrew_name: "מֵרְכָא כְּפוּלָה",
    hebrew_concept: "double lengthener",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MERKHA_KEFULA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("Merkha duplex"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const DARGA_INFO: AccentInformation = AccentInformation {
    english_name: "Darga",
    hebrew_name: "דַּרְגָּא",
    hebrew_concept: "stairstep",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_DARGA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const AZLA_INFO: AccentInformation = AccentInformation {
    english_name: "Azla",
    hebrew_name: "אַזְלָא",
    hebrew_concept: "going on (not pausing)depart",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QADMA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Qadma",
        hebrew_name: "קַדְמָ֨א",
        hebrew_concept: "antiquity or a former state",
    }),
    notes: Some("When Geresh: Qadma"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TELISHA_QETANNAH_INFO: AccentInformation = AccentInformation {
    english_name: "Telisha Qetannah",
    hebrew_name: "תְּלִישָא קְטַנָּה",
    hebrew_concept: "small (short) detached",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TELISHA_QETANA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Post),
};

pub(crate) const GALGAL_INFO: AccentInformation = AccentInformation {
    english_name: "Galgal",
    hebrew_name: "גַּלְגַּל",
    hebrew_concept: "wheelcircle",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_YERAH_BEN_YOMO,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Jerach Ben Jomo",
        hebrew_name: "יֵרֶח בֶּן יוֹמוֹ",
        hebrew_concept: "moon one day old",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const MAYELA_INFO: AccentInformation = AccentInformation {
    english_name: "Mayela",
    hebrew_name: "מָאיְלָא",
    hebrew_concept: "to be raised or elevated",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Meayyela",
        hebrew_name: "מְאַיְּלָא",
        hebrew_concept: "todo",
    }),
    notes: Some("Name given to a Tiphchawhen in the same word as Atnach or Silluq"),
    category: Some(AccentCategory::Conjunctive),
    accent_type: Some(AccentType::Secondary),
    word_stress: None,
};

pub(crate) const METEG_INFO: AccentInformation = AccentInformation {
    english_name: "Meteg",
    hebrew_name: "מֶתֶג",
    hebrew_concept: "accent or mark",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_METEG,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Secondary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: None,
};

/*
Shene Pashtin 	 ב֨ב   	U+0599U+05A8 	שְׁנֵ֨י פַּשְׁטִין֙
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

pub(crate) const OLEH_WE_YORED_INFO: AccentInformation = AccentInformation {
    english_name: "Oleh We Yored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    hebrew_concept: "ascending and descending",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_OLE,
        secondary_mark: Some(&CODEPOINT_MERKHA),
    },
           alternate_names: None,
 notes: Some("The primary_mark CodePoint is Mehuppakhbut located above the consonant. It is then called OLE."),
        accent_type: Some(AccentType::Primary),
        category:Some(AccentCategory::Disjunctive),
     word_stress: Some(WordStress::Im)
};

// ATNACH see PROSE section above

pub(crate) const REVIA_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Gadol",
    hebrew_name: "רְבִיעַ גּדוֹל",
    hebrew_concept: "big fourth",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const REVIA_MUGRASH_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    hebrew_concept: "exiled fourth",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: Some(&CODEPOINT_REVIA),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const SHALSHELET_GADOL_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    hebrew_concept: "large chain or link",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TSINNOR_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnor",
    hebrew_name: "צִנּוֹר",
    hebrew_concept: "pipe or tube",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZINOR,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Zarqa",
        hebrew_name: "זַרְקָא",
        hebrew_concept: "to sprinklescatter",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Post),
};

pub(crate) const REVIA_QATON_INFO: AccentInformation = AccentInformation {
    english_name: "Revia Qaton",
    hebrew_name: "רְבִיעַ קָטוֹן",
    hebrew_concept: "small fourth",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("After that occurs Oleh We Yored"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const DECHI_INFO: AccentInformation = AccentInformation {
    english_name: "Dechi",
    hebrew_name: "דֶּחִי",
    hebrew_concept: "to push or drive away",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_DEHI,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Tiphcha",
        hebrew_name: "טִפְחָא",
        hebrew_concept: "handbreadth or diagonal",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Pre),
};

// PAZER see PROSE section above

pub(crate) const MEHUPPAKH_LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    hebrew_concept: "reversed to its own",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const AZLA_LEGARMEH_INFO: AccentInformation = AccentInformation {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    hebrew_concept: "goes to its own",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QADMA,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Disjunctive),
    word_stress: Some(WordStress::Im),
};

// MUNACH see PROSE section above

// MERCHA see PROSE section above

pub(crate) const ILLUY_INFO: AccentInformation = AccentInformation {
    english_name: "Illuy",
    hebrew_name: "עִלּוּי",
    hebrew_concept: "elevation or raising",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ILUY,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("also called Munach superior"),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TARCHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tarcha",
    hebrew_name: "טַרְחָא",
    hebrew_concept: "troubledifficultyhardshiptoil",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some("it refers to the effortstrainor inconvenience involved in doing something."),
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

// GALGAL see PROSE section above

pub(crate) const MEHUPPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Mehuppakh",
    hebrew_name: "מְהֻפָּ֤ךְ",
    hebrew_concept: "reversed",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Mahpakh",
        hebrew_name: "מַהְפַּךְ",
        hebrew_concept: "turning round",
    }),
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

// AZLA see PROSE section above

pub(crate) const SHALSHELET_QETANNAH_INFO: AccentInformation = AccentInformation {
    english_name: "Shalshelet Qetannah",
    hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
    hebrew_concept: "small chain",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TSINNORIT_MERKHA_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    hebrew_concept: "pipe of continuation",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MERKHA),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

pub(crate) const TSINNORIT_MAHPAKH_INFO: AccentInformation = AccentInformation {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    hebrew_concept: "pipe of reversal",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MAHAPAKH),
    },
    alternate_names: None,
    notes: None,
    accent_type: Some(AccentType::Primary),
    category: Some(AccentCategory::Conjunctive),
    word_stress: Some(WordStress::Im),
};

/********************************************************
 *                      PSEUDO ACCENT
 *******************************************************/
pub(crate) const SOPH_PASUQ_INFO: AccentInformation = AccentInformation {
    english_name: "Soph Pasuq",
    hebrew_name: "סוֹף פָּסוּק",
    hebrew_concept: "end of verse",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SOPH_PASUQ,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some(
        "it doesn’t carry any theological or interpretive meaning beyond marking a boundary",
    ),
    accent_type: None,
    category: None,
    word_stress: None,
};

pub(crate) const MAQQEPH_INFO: AccentInformation = AccentInformation {
    english_name: "Maqqeph",
    hebrew_name: "מַקֵּף",
    hebrew_concept:"binder",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAQAF,
        secondary_mark: None,
    },
              alternate_names: None,
 notes: Some("Can link two (or more) short words togetherafter which they function as a single compound word bearing a single Hebrew accent."),
    accent_type: None,
    category: None,
    word_stress: None,
};

pub(crate) const PASEQ_INFO: AccentInformation = AccentInformation {
    english_name: "Paseq",
    hebrew_name: "פָּסֵק",
    hebrew_concept: "to pauseto stop or to interrupt",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PASEQ,
        secondary_mark: None,
    },
    alternate_names: None,
    notes: Some(
        "It’s indicating that someone or something is stopping temporarily or creating a pause.",
    ),
    accent_type: None,
    category: None,
    word_stress: None,
};

#[cfg(test)]
mod accent_data_tests {
    // Import the tables and maps
    use crate::accent::{Accent, PoetryAccent, ProseAccent, PseudoAccent};
    use crate::accent_data::{
        BHS_POETRY_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE,
    };

    // Ensures the table length matches the enum LEN constant
    #[test]
    fn test_prose_table_length_matches_enum_count() {
        assert_eq!(
            PROSE_ACCENT_TABLE.len(),
            ProseAccent::LEN,
            "PROSE_ACCENT_TABLE length ({}) does not match ProseAccent::LEN ({})",
            PROSE_ACCENT_TABLE.len(),
            ProseAccent::LEN
        );
    }
    #[test]
    fn test_poetry_table_length_matches_enum_count() {
        assert_eq!(
            POETRY_ACCENT_TABLE.len(),
            PoetryAccent::LEN,
            "POETRY_ACCENT_TABLE length ({}) does not match PoetryAccent::LEN ({})",
            POETRY_ACCENT_TABLE.len(),
            PoetryAccent::LEN
        );
    }
    #[test]
    fn test_pseudo_table_length_matches_enum_count() {
        assert_eq!(
            PSEUDO_ACCENT_TABLE.len(),
            PseudoAccent::LEN,
            "PSEUDO_ACCENT_TABLE length ({}) does not match PseudoAccent::LEN ({})",
            PSEUDO_ACCENT_TABLE.len(),
            PseudoAccent::LEN
        );
    }
    // Tests the branch logic: `if secondary_mark.is_none() { 1 } else { 2 }`
    // This ensures we cover both the "single code point" and "double code point" paths.
    #[test]
    fn test_prose_accent_code_point_branches() {
        let mut single_count = 0;
        let mut double_count = 0;

        for &info in PROSE_ACCENT_TABLE.iter() {
            if info.cantillation_symbol.secondary_mark.is_none() {
                single_count += 1;
                assert_eq!(
                    info.cantillation_symbol.primary_mark.code_point_value.len(),
                    6,
                    "Primary should be length 1"
                );
            } else {
                double_count += 1;
                assert_eq!(
                    info.cantillation_symbol.primary_mark.code_point_value.len(),
                    6,
                    "Primary should be length 1"
                );
                assert!(
                    info.cantillation_symbol.secondary_mark.is_some(),
                    "Secondary must be Some"
                );
            }
        }

        //     // Assert that we actually hit both branches (coverage requirement)
        assert_eq!(
            single_count, 26,
            "No single-code-point accents found in Prose table"
        );
        assert_eq!(
            double_count, 2,
            "No double-code-point accents found in Prose table"
        );

        //     // Example: Shalshelet has a secondary_mark Paseq
        let shalshelet_info = PROSE_ACCENT_TABLE[ProseAccent::Shalshelet as usize];
        assert!(
            shalshelet_info.cantillation_symbol.secondary_mark.is_some(),
            "Shalshelet should have secondary_mark"
        );
    }

    #[test]
    fn test_poetry_accent_code_point_branches() {
        let mut single_count = 0;
        let mut double_count = 0;

        for &info in POETRY_ACCENT_TABLE.iter() {
            if info.cantillation_symbol.secondary_mark.is_none() {
                single_count += 1;
            } else {
                double_count += 1;
            }
        }

        assert_eq!(
            single_count, 16,
            "No single-code-point accents found in Poetry table"
        );
        assert_eq!(
            double_count, 7,
            "No double-code-point accents found in Poetry table"
        );
    }
    /// Ensures the rank map covers every variant in PoetryAccent
    #[test]
    fn test_poetry_rank_map_length() {
        assert_eq!(
            BHS_POETRY_RANK_MAP.len(),
            PoetryAccent::LEN,
            "Rank map length ({}) does not match PoetryAccent::LEN ({})",
            BHS_POETRY_RANK_MAP.len(),
            PoetryAccent::LEN
        );
    }
    /// Tests specific known ranks to ensure the map is populated correctly
    #[test]
    fn test_poetry_rank_map_specific_values() {
        // Silluq (index 0) should be rank 1 (strongest)
        assert_eq!(BHS_POETRY_RANK_MAP[PoetryAccent::Silluq as usize], 1);

        // Meteg (last index) should be the highest rank (weakest)
        let last_idx = PoetryAccent::Meteg as usize;
        let last_rank = BHS_POETRY_RANK_MAP[last_idx];

        // Verify it's the maximum value in the map
        let max_rank = BHS_POETRY_RANK_MAP.iter().max().unwrap();
        assert_eq!(
            last_rank, *max_rank,
            "Meteg should have the highest rank value"
        );
    }

    /// Tests that the map handles the "same rank" case (TsinnoritMerkha vs TsinnoritMahpakh)
    #[test]
    fn test_poetry_rank_map_duplicate_ranks() {
        let tsinnor_merkha_idx = PoetryAccent::TsinnoritMerkha as usize;
        let tsinnor_mahpakh_idx = PoetryAccent::TsinnoritMahpakh as usize;

        let rank_merkha = BHS_POETRY_RANK_MAP[tsinnor_merkha_idx];
        let rank_mahpakh = BHS_POETRY_RANK_MAP[tsinnor_mahpakh_idx];

        assert_eq!(
            rank_merkha, rank_mahpakh,
            "TsinnoritMerkha and TsinnoritMahpakh should have the same rank"
        );
    }

    // ========================================================================
    // 4. Static Data Consistency
    // ========================================================================

    /// Ensures no duplicate English names exist within the same table
    #[test]
    fn test_prose_table_unique_names() {
        let names: Vec<&str> = PROSE_ACCENT_TABLE.iter().map(|i| i.english_name).collect();
        let unique_names: std::collections::HashSet<_> = names.iter().collect();

        assert_eq!(
            names.len(),
            unique_names.len(),
            "Duplicate English names found in Prose table"
        );
    }

    #[test]
    fn test_poetry_table_unique_names() {
        let names: Vec<&str> = POETRY_ACCENT_TABLE.iter().map(|i| i.english_name).collect();
        let unique_names: std::collections::HashSet<_> = names.iter().collect();

        assert_eq!(
            names.len(),
            unique_names.len(),
            "Duplicate English names found in Poetry table"
        );
    }

    /// Ensures Hebrew names are not empty
    #[test]
    fn test_all_tables_have_hebrew_names() {
        for &info in PROSE_ACCENT_TABLE.iter() {
            assert!(
                !info.hebrew_name.is_empty(),
                "Empty Hebrew name in Prose table"
            );
        }
        for &info in POETRY_ACCENT_TABLE.iter() {
            assert!(
                !info.hebrew_name.is_empty(),
                "Empty Hebrew name in Poetry table"
            );
        }
        for &info in PSEUDO_ACCENT_TABLE.iter() {
            assert!(
                !info.hebrew_name.is_empty(),
                "Empty Hebrew name in Pseudo table"
            );
        }
    }

    // ========================================================================
    // 5. Integration with Accent Trait (Indirect Coverage)
    // ========================================================================

    /// Tests that the `details()` method (which uses table indexing) works for all variants

    /// Tests that `code_points()` logic works for all variants (covering the if/else branch)
    #[test]
    fn test_code_points_calculation_all_variants() {
        // Prose
        for i in 0..ProseAccent::LEN {
            let variant: ProseAccent = unsafe { std::mem::transmute(i as u8) };
            let cp = variant.number_of_symbols();
            assert!(cp == 1 || cp == 2, "Invalid code_points count: {:?}", cp);
        }

        // Poetry
        for i in 0..PoetryAccent::LEN {
            let variant: PoetryAccent = unsafe { std::mem::transmute(i as u8) };
            let cp = variant.number_of_symbols();
            assert!(cp == 1 || cp == 2, "Invalid code_points count: {:?}", cp);
        }

        // Pseudo (should always be 1)
        for i in 0..PseudoAccent::LEN {
            let variant: PseudoAccent = unsafe { std::mem::transmute(i as u8) };
            let cp = variant.number_of_symbols();
            assert_eq!(cp, 1, "PseudoAccent code_points should always be 1");
        }
    }

    // ========================================================================
    // 6. Stress Test / Static Initialization
    // ========================================================================

    /// Ensures the Lazy statics initialize correctly and can be accessed repeatedly
    #[test]
    fn test_static_initialization_stability() {
        for _ in 0..100 {
            let _p_len = PROSE_ACCENT_TABLE.len();
            let _po_len = POETRY_ACCENT_TABLE.len();
            let _ps_len = PSEUDO_ACCENT_TABLE.len();
            let _rank_len = BHS_POETRY_RANK_MAP.len();
        }
    }
}
