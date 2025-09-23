use crate::accent::{AccentCategory, AccentInfo, AccentType, Alternates, CodePoints};
use crate::codepoints::*;

pub const SILLUQ_INFO: AccentInfo = AccentInfo {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    meaning: "close, cessation",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_SILLUQ,
        secondary: None,
    },

    comment: Some(
        "The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.",
    ),
};
pub const ATNACH_INFO: AccentInfo = AccentInfo {
    english_name: "Atnach",
    hebrew_name: "אתְנָח",
    meaning: "a causing to rest",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_ETNAHTA,
        secondary: None,
    },
    comment: None,
};
pub const SEGOLTA_INFO: AccentInfo = AccentInfo {
    english_name: "Segolta",
    hebrew_name: "סְגֹולְתָּא",
    meaning: "a little grape-bunch",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_SEGOL,
        secondary: None,
    },
    comment: None,
};
pub const SHALSHELET_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    meaning: "chain or link",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
};
pub const ZAQEF_QATON_INFO: AccentInfo = AccentInfo {
    english_name: "Zaqeph Qaton",
    hebrew_name: "זָקֵף קָטוֹן",
    meaning: "small upright",
    alternates: Some(Alternates {
        english_name: "Zaqeph Qatan",
        hebrew_name: "זָקֵף קָטָן",
        meaning: "todo",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_ZAQEF_QATAN,
        secondary: None,
    },
    comment: None,
};
pub const ZAQEPH_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Zaqeph Gadol",
    hebrew_name: "זָקֵף גָּדוֹל",
    meaning: "large upright",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_ZAQEF_GADOL,
        secondary: None,
    },
    comment: None,
};
pub const REVIA_INFO: AccentInfo = AccentInfo {
    english_name: "Revia",
    hebrew_name: "רְבִיעַ",
    meaning: "fourth [in a sequence]",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: Some("probably due to its four-note tune."),
};
pub const TIPHCHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tiphcha", // ADD TARCHA
    hebrew_name: "טִפְחָא",
    meaning: "handbreadth or diagonal",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: Some("before Atnach and Silluq"),
};
pub const ZARQA_INFO: AccentInfo = AccentInfo {
    english_name: "Zarqa",
    hebrew_name: "זַרְקָא",
    meaning: "to sprinkle, scatter",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_ZINOR,
        secondary: None,
    },
    comment: Some("before  Segolta"),
};
pub const PASHTA_INFO: AccentInfo = AccentInfo {
    english_name: "Pashta",
    hebrew_name: "פַּשְׁטָא",
    meaning: "extending, stretching out in length",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_PASHTA,
        secondary: None,
    },
    comment: Some("if you sound almost last (2 pasta’s in one word)"),
};
pub const YETIV_INFO: AccentInfo = AccentInfo {
    english_name: "Yetiv",
    hebrew_name: "יְתִיב",
    meaning: "resting or sitting",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_YETIV,
        secondary: None,
    },
    comment: Some("occasionally for a Pashta"),
};
pub const TEVIR_INFO: AccentInfo = AccentInfo {
    english_name: "Tevir",
    hebrew_name: "תְּבִיר",
    meaning: "broken, downward tumble",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_TEVIR,
        secondary: None,
    },
    comment: None,
};
pub const GERESH_INFO: AccentInfo = AccentInfo {
    english_name: "Geresh",
    hebrew_name: "גֵּרֵישׁ",
    meaning: "expulsion, driving out, divorce",
    alternates: Some(Alternates {
        english_name: "Teres",
        hebrew_name: "טֶרֶס",
        meaning: "todo",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_GERESH,
        secondary: None,
    },
    comment: None,
};
pub const GERSHAYIM_INFO: AccentInfo = AccentInfo {
    english_name: "Gershayim",
    hebrew_name: "גֵּרְשַׁיִם",
    meaning: "double of expulsion, driving out, divorce",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_GERSHAYIM,
        secondary: None,
    },
    comment: None,
};
pub const PAZER_INFO: AccentInfo = AccentInfo {
    english_name: "Pazer",
    hebrew_name: "פָּזֶר",
    meaning: "lavish or scatter",
    alternates: Some(Alternates {
        english_name: "Pazer Qatan",
        hebrew_name: "פָּזֵר קָטָן",
        meaning: "small lavish or scatter",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_PAZER,
        secondary: None,
    },
    comment: None,
};
pub const PAZER_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Pazer Gadol",
    hebrew_name: "פָּזֶר גּדוֹל",
    meaning: "large lavish or scatter",
    alternates: Some(Alternates {
        english_name: "Qarne Pharah",
        hebrew_name: "קַרְנֵי פָרָה",
        meaning: "horns of a cow",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_QARNEY_PARA,
        secondary: None,
    },
    comment: None,
};
pub const TELISHA_GEDOLAH_INFO: AccentInfo = AccentInfo {
    english_name: "Telisha Gedolah",
    hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
    meaning: "great (long) detached",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_TELISHA_GEDOLA,
        secondary: None,
    },
    comment: None,
};
pub const LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Legarmeh",
    hebrew_name: "לְגַרְמֶהּ",
    meaning: "for or by itself, independant",
    alternates: Some(Alternates {
        english_name: "Munach Legarmeh",
        hebrew_name: "מוּנַח לְגַרְמֵ֣הּ",
        meaning: "todo",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_MUNAH,
        secondary: Some(&CP_PASEQ),
    },
    comment: Some("Munach with Passeq; Before Revia"),
};
// Conjunctives
pub const MUNACH_INFO: AccentInfo = AccentInfo {
    english_name: "Munach",
    hebrew_name: "מוּנַ֣ח",
    meaning: "resting or placed",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MUNAH,
        secondary: None,
    },
    comment: None,
};
pub const MAHPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Mahpakh",
    hebrew_name: "מַהְפַּךְ",
    meaning: "turning round",
    alternates: Some(Alternates {
        english_name: "Mehuppakh",
        hebrew_name: "מְהֻפָּ֤ךְ",
        meaning: "reversed",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: None,
    },
    comment: None,
};
pub const MERKHA_INFO: AccentInfo = AccentInfo {
    english_name: "Merkha",
    hebrew_name: "מֵרְכָא",
    meaning: "lengthener, prolonging",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MERKHA,
        secondary: None,
    },
    comment: None,
};
pub const MERKHA_KEFULAH_INFO: AccentInfo = AccentInfo {
    english_name: "Merkha Kephulah",
    hebrew_name: "מֵרְכָא כְּפוּלָה",
    meaning: "double lengthener",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MERKHA_KEFULA,
        secondary: None,
    },
    comment: Some("Merkha duplex"),
};
pub const DARGA_INFO: AccentInfo = AccentInfo {
    english_name: "Darga",
    hebrew_name: "דַּרְגָּא",
    meaning: "stairstep",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_DARGA,
        secondary: None,
    },
    comment: None,
};
pub const AZLA_INFO: AccentInfo = AccentInfo {
    english_name: "Azla",
    hebrew_name: "אַזְלָא",
    meaning: "going on (not pausing), depart",
    alternates: Some(Alternates {
        english_name: "Qadma",
        hebrew_name: "קַדְמָ֨א",
        meaning: "antiquity or a former state",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_QADMA,
        secondary: None,
    },
    comment: Some("When Geresh: Qadma"),
};
pub const TELISHA_QETANNAH_INFO: AccentInfo = AccentInfo {
    english_name: "Telisha Qetannah",
    hebrew_name: "תְּלִישָא קְטַנָּה",
    meaning: "small (short) detached",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_TELISHA_QETANA,
        secondary: None,
    },
    comment: None,
};
pub const GALGAL_INFO: AccentInfo = AccentInfo {
    english_name: "Galgal",
    hebrew_name: "גַּלְגַּל",
    meaning: "wheel, circle",
    alternates: Some(Alternates {
        english_name: "Jerach Ben Jomo",
        hebrew_name: "יֵרֶח בֶּן יוֹמוֹ",
        meaning: "moon one day old",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_YERAH_BEN_YOMO,
        secondary: None,
    },
    comment: None,
};
pub const MAYELA_INFO: AccentInfo = AccentInfo {
    english_name: "Mayela",
    hebrew_name: "מָאיְלָא",
    meaning: "to be raised or elevated",
    alternates: Some(Alternates {
        english_name: "Meayyela",
        hebrew_name: "מְאַיְּלָא",
        meaning: "todo",
    }),
    accent_type: AccentType::Secondary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq"),
};
pub const METEG_INFO: AccentInfo = AccentInfo {
    english_name: "Meteg",
    hebrew_name: "מֶתֶג",
    meaning: "accent or mark",
    alternates: None,
    accent_type: AccentType::Secondary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_METEG,
        secondary: None,
    },
    comment: None,
};
pub const MAQQEPH_INFO: AccentInfo = AccentInfo {
    english_name: "Maqqeph",
    hebrew_name: "מַקֵּף",
    meaning:"binder",
    alternates: None,
    accent_type: AccentType::None,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MAQAF,
        secondary: None,
    },
    comment: Some("The Maqqeph (in Biblical Hebrew), can link two (or more) short words together, after which they function as a single compound word bearing a single Hebrew accent."),
};
/*
Shene pashtin 	 ב֨ב   	U+0599, U+05A8 	שְׁנֵ֨י פַּשְׁטִין֙
Shene pashtin 	תְּרֵ֨י קַדְמִין֙
Tere qadmin 	(שְׁנֵי) פַּ֨שְׁטִין֙
(Shene) pashtin
*/
// example: Genesis 28:20
//   וּשְׁמָרַ֨נִי֙
/*********************************************************
 *                          POETRY
 *******************************************************/
// Disjunctives

// SILLUQ see PROSE

pub const OLEH_WE_YORED_INFO: AccentInfo = AccentInfo {
    english_name: "Oleh We Yored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    meaning: "ascending and descending",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_OLE,
        secondary: Some(&CP_MERKHA),
    },
    comment: Some("The primary CodePoint is Mehuppakh, but located above the consonant. It is then called OLE."),
};

// ATNACH see PROSE

pub const REVIA_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Gadol",
    hebrew_name: "רְבִיעַ גּדוֹל",
    meaning: "big fourth",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: None,
};
pub const REVIA_MUGRASH_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    meaning: "exiled fourth",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_GERESH,
        secondary: Some(&CP_REVIA),
    },
    comment: None,
};
pub const SHALSHELET_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    meaning: "large chain or link",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
};
pub const TSINNOR_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnor",
    hebrew_name: "צִנּוֹר",
    meaning: "pipe or tube",
    alternates: Some(Alternates {
        english_name: "Zarqa",
        hebrew_name: "זַרְקָא",
        meaning: "to sprinkle, scatter",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_ZINOR,
        secondary: None,
    },
    comment: None,
};
pub const REVIA_QATON_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Qaton",
    hebrew_name: "רְבִיעַ קָטוֹן",
    meaning: "small fourth",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_REVIA,
        secondary: None,
    },
    comment: Some("After that occurs Oleh We Yored"),
};
pub const DECHI_INFO: AccentInfo = AccentInfo {
    english_name: "Dechi",
    hebrew_name: "דֶּחִי",
    meaning: "to push or drive away",
    alternates: Some(Alternates {
        english_name: "Tiphcha",
        hebrew_name: "טִפְחָא",
        meaning: "handbreadth or diagonal",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_DEHI,
        secondary: None,
    },
    comment: None,
};

// PAZER see PROSE section

pub const MEHUPPAKH_LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    meaning: "reversed to its own",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
};
pub const AZLA_LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    meaning: "goes to its own",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Disjunctive,
    code_points: CodePoints {
        primary: &CP_QADMA,
        secondary: Some(&CP_PASEQ),
    },
    comment: None,
};

// MUNACH see PROSE

// MERCHA see PROSE

pub const ILLUY_INFO: AccentInfo = AccentInfo {
    english_name: "Illuy",
    hebrew_name: "עִלּוּי",
    meaning: "elevation or raising",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_ILUY,
        secondary: None,
    },
    comment: Some("also called Munach superior"),
};
pub const TARCHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tarcha",
    hebrew_name: "טַרְחָא",
    meaning: "handbreadth or diagonal", //todo
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_TIPEHA,
        secondary: None,
    },
    comment: None,
};

// GALGAL see PROSE

pub const MEHUPPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Mehuppakh",
    hebrew_name: "מְהֻפָּ֤ךְ",
    meaning: "reversed",
    alternates: Some(Alternates {
        english_name: "Mahpakh",
        hebrew_name: "מַהְפַּךְ",
        meaning: "turning round",
    }),
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_MAHAPAKH,
        secondary: None,
    },
    comment: None,
};

// AZLA see PROSE

pub const SHALSHELET_QETANNAH_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet Qetannah",
    hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
    meaning: "small chain",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_SHALSHELET,
        secondary: None,
    },
    comment: None,
};
pub const TSINNORIT_MERKHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    meaning: "pipe of continuation",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_ZARQA,
        secondary: Some(&CP_MERKHA),
    },
    comment: None,
};
pub const TSINNORIT_MAHPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    meaning: "pipe of reversal",
    alternates: None,
    accent_type: AccentType::Primary,
    category: AccentCategory::Conjunctive,
    code_points: CodePoints {
        primary: &CP_ZARQA,
        secondary: Some(&CP_MAHAPAKH),
    },
    comment: None,
};

// METEG see PROSE

//MAQQEPH see PROSE
