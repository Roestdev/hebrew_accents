//! This file contains all static data of the 'Hebrew Accents'

// Crate‑internal (local modules)
use crate::accent::AccentCategory;
use crate::accent::{
    AccentInformation, AccentType, AlternateNames, CantillationSymbol, WordStress,
};

use crate::codepoints::{
    CODEPOINT_DARGA, CODEPOINT_DEHI, CODEPOINT_ETNAHTA, CODEPOINT_GERESH, CODEPOINT_GERSHAYIM,
    CODEPOINT_ILUY, CODEPOINT_MAHAPAKH, CODEPOINT_MAQAF, CODEPOINT_MERKHA, CODEPOINT_MERKHA_KEFULA,
    CODEPOINT_METEG, CODEPOINT_MUNAH, CODEPOINT_OLE, CODEPOINT_PASEQ, CODEPOINT_PASHTA,
    CODEPOINT_PAZER, CODEPOINT_QADMA, CODEPOINT_QARNEY_PARA, CODEPOINT_REVIA, CODEPOINT_SEGOL,
    CODEPOINT_SHALSHELET, CODEPOINT_SILLUQ, CODEPOINT_SOPH_PASUQ, CODEPOINT_TELISHA_GEDOLA,
    CODEPOINT_TELISHA_QETANA, CODEPOINT_TEVIR, CODEPOINT_TIPEHA, CODEPOINT_YERAH_BEN_YOMO,
    CODEPOINT_YETIV, CODEPOINT_ZAQEF_GADOL, CODEPOINT_ZAQEF_QATAN, CODEPOINT_ZARQA,
    CODEPOINT_ZINOR,
};
pub(crate) const SILLUQ_INFO: AccentInformation = AccentInformation {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    hebrew_concept: "close, cessation",
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
        hebrew_concept: "TODO",
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
        hebrew_concept: "TODO",
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
        hebrew_concept: "TODO",
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
        hebrew_concept: "TODO",
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
    hebrew_concept: "to pause, to stop or to interrupt",
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
