//! This file contains all static data of the 'Hebrew Accents'

// Crate‑internal (local modules)
use crate::accent::WordSpan;
use crate::accent::{AccentMetaData, AlternateNames, CantillationSymbol};
use crate::accent::{Category, Kind};

use crate::accent_mark::{
    CODEPOINT_DARGA, CODEPOINT_DEHI, CODEPOINT_ETNAHTA, CODEPOINT_GERESH, CODEPOINT_GERSHAYIM,
    CODEPOINT_ILUY, CODEPOINT_MAHAPAKH, CODEPOINT_MAQAF, CODEPOINT_MERKHA, CODEPOINT_MERKHA_KEFULA,
    CODEPOINT_METEG, CODEPOINT_MUNAH, CODEPOINT_OLE, CODEPOINT_PASEQ, CODEPOINT_PASHTA,
    CODEPOINT_PAZER, CODEPOINT_QADMA, CODEPOINT_QARNEY_PARA, CODEPOINT_REVIA, CODEPOINT_SEGOL,
    CODEPOINT_SHALSHELET, CODEPOINT_SILLUQ, CODEPOINT_SOPH_PASUQ, CODEPOINT_TELISHA_GEDOLA,
    CODEPOINT_TELISHA_QETANA, CODEPOINT_TEVIR, CODEPOINT_TIPEHA, CODEPOINT_YERAH_BEN_YOMO,
    CODEPOINT_YETIV, CODEPOINT_ZAQEF_GADOL, CODEPOINT_ZAQEF_QATAN, CODEPOINT_ZARQA,
    CODEPOINT_ZINOR, TRADITION_NAMES_AZLA_LEGARMEH, TRADITION_NAMES_DARGA, TRADITION_NAMES_DEHI,
    TRADITION_NAMES_ETNAHTA, TRADITION_NAMES_GERESH, TRADITION_NAMES_GERSHAYIM,
    TRADITION_NAMES_ILUY, TRADITION_NAMES_LEGARMEH, TRADITION_NAMES_MAHAPAKH,
    TRADITION_NAMES_MAQAF, TRADITION_NAMES_MEHUPPAKH_LEGARMEH, TRADITION_NAMES_MERKHA,
    TRADITION_NAMES_MERKHA_KEFULA, TRADITION_NAMES_METEG, TRADITION_NAMES_MUNAH,
    TRADITION_NAMES_OLEH_WEYORED, TRADITION_NAMES_PASEQ, TRADITION_NAMES_PASHTA,
    TRADITION_NAMES_PAZER, TRADITION_NAMES_QADMA, TRADITION_NAMES_QARNEY_PARA,
    TRADITION_NAMES_REVIA, TRADITION_NAMES_SEGOL, TRADITION_NAMES_SHALSHELET,
    TRADITION_NAMES_SHALSHELET_GADOL, TRADITION_NAMES_SILLUQ, TRADITION_NAMES_SOPH_PASUQ,
    TRADITION_NAMES_TELISHA_GEDOLA, TRADITION_NAMES_TELISHA_QETANA, TRADITION_NAMES_TEVIR,
    TRADITION_NAMES_TIPEHA, TRADITION_NAMES_TSINNORIT_MAHPACH, TRADITION_NAMES_TSINNORIT_MERKHA,
    TRADITION_NAMES_YERAH_BEN_YOMO, TRADITION_NAMES_YETIV, TRADITION_NAMES_ZAQEF_GADOL,
    TRADITION_NAMES_ZAQEF_QATAN, TRADITION_NAMES_ZINOR,
};

pub(crate) const SILLUQ_INFO: AccentMetaData = AccentMetaData {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    hebrew_concept: "close, cessation",
    sbl_academic: "sillûq",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SILLUQ,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::NotApplicable,
    notes: Some(
        r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse."#,
    ),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_SILLUQ,
};

pub(crate) const ATNACH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Atnach",
    hebrew_name: "אַתְנַח",
    hebrew_concept: "a causing to rest",
    sbl_academic: "ʾatnaḥ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ETNAHTA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"The major disjunctive accent dividing the verse in two halves."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_ETNAHTA,
};

pub(crate) const SEGOLTA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Segolta",
    hebrew_name: "סְגֹולְתָּא",
    hebrew_concept: "a little grape-bunch",
    sbl_academic: "səgôlətāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SEGOL,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Appears frequently in prose, preceding Atnach or Silluq."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_SEGOL,
};

pub(crate) const SHALSHELET_INFO: AccentMetaData = AccentMetaData {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    hebrew_concept: "chain or link",
    sbl_academic: "šalšelet",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Rare accent occurring only four times in the Torah."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_SHALSHELET,
};

pub(crate) const ZAQEPH_QATON_INFO: AccentMetaData = AccentMetaData {
    english_name: "Zaqeph Qaton",
    hebrew_name: "זָקֵף קָטוֹן",
    hebrew_concept: "small upright or small standing one",
    sbl_academic: "zāqēp qāṭôn",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZAQEF_QATAN,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Zaqeph Qatan",
        hebrew_name: "זָקֵף קָטָן",
        hebrew_concept: "small upright or small standing one",
        sbl_academic: "zāqēp qāṭān",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Common disjunctive, often appearing before Zaqeph Gadol or Atnach."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_ZAQEF_QATAN,
};

pub(crate) const ZAQEPH_GADOL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Zaqeph Gadol",
    hebrew_name: "זָקֵף גָּדוֹל",
    hebrew_concept: "large upright",
    sbl_academic: "zāqēp gādôl",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZAQEF_GADOL,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Less common than Zaqeph Qaton; appears after Qadma or Azla."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_ZAQEF_GADOL,
};

pub(crate) const REVIA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Revia",
    hebrew_name: "רְבִיעַ",
    hebrew_concept: "fourth [in a sequence]",
    sbl_academic: "rəbîaʿ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("probably due to its four-note tune."),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_REVIA,
};

pub(crate) const TIPHCHA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tiphcha",
    hebrew_name: "טִפְחָא",
    hebrew_concept: "handbreadth or diagonal",
    sbl_academic: "ṭipḥāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("before Atnach and Silluq"),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_TIPEHA,
};

pub(crate) const ZARQA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Zarqa",
    hebrew_name: "זַרְקָא",
    hebrew_concept: "to sprinkle, scatter",
    sbl_academic: "zarqāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZINOR,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("before Segolta"),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_ZINOR,
};

pub(crate) const PASHTA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Pashta",
    hebrew_name: "פַּשְׁטָא",
    hebrew_concept: "extending, stretching out in length",
    sbl_academic: "pašṭāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PASHTA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("appears on the last syllable of the word, extending rightward."),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_PASHTA,
};

pub(crate) const YETIV_INFO: AccentMetaData = AccentMetaData {
    english_name: "Yetiv",
    hebrew_name: "יְתִיב",
    hebrew_concept: "resting or sitting",
    sbl_academic: "yətîb",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_YETIV,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("occasionally substitutes for a Pashta"),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_YETIV,
};

pub(crate) const TEVIR_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tevir",
    hebrew_name: "תְּבִיר",
    hebrew_concept: "broken, downward tumble",
    sbl_academic: "təbîr",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TEVIR,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Often followed by Revia; indicates a melodic break."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_TEVIR,
};

pub(crate) const GERESH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Geresh",
    hebrew_name: "גֵּרֵישׁ",
    hebrew_concept: "expulsion, driving out, divorce",
    sbl_academic: "gērêš",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Teres",
        hebrew_name: "טֶרֶס",
        hebrew_concept: "a boundary or border",
        sbl_academic: "ṭeres",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Functions as Level 4 disjunctive; often preceded by Azla."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_GERESH,
};

pub(crate) const GERSHAYIM_INFO: AccentMetaData = AccentMetaData {
    english_name: "Gershayim",
    hebrew_name: "גֵּרְשַׁיִם",
    hebrew_concept: "double of expulsion, driving out, divorce",
    sbl_academic: "gērəšayīm",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERSHAYIM,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Double Geresh; rarer variant appearing in specific syntactic contexts."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_GERSHAYIM,
};

pub(crate) const PAZER_INFO: AccentMetaData = AccentMetaData {
    english_name: "Pazer",
    hebrew_name: "פָּזֶר",
    hebrew_concept: "lavish or scatter",
    sbl_academic: "pāzer",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PAZER,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(
        r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
    ),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_PAZER,
};

pub(crate) const PAZER_GADOL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Pazer Gadol",
    hebrew_name: "פָּזֶר גּדוֹל",
    hebrew_concept: "large lavish or scatter",
    sbl_academic: "pāzer gdôl",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QARNEY_PARA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Qarne Pharah",
        hebrew_name: "קַרְנֵי פָרָה",
        hebrew_concept: "horns of a cow",
        sbl_academic: "qarnê pārâ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Also known as Qarne Pharah; rare variant resembling horns."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_QARNEY_PARA,
};

pub(crate) const TELISHA_GEDOLAH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Telisha Gedolah",
    hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
    hebrew_concept: "great (long) detached",
    sbl_academic: "təlîšāʾ gədôlâ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TELISHA_GEDOLA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Level 4 disjunctive; distinct from Telisha Qetannah."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_TELISHA_GEDOLA,
};

pub(crate) const LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Legarmeh",
    hebrew_name: "לְגַרְמֶהּ",
    hebrew_concept: "for or by itself, independent",
    sbl_academic: "ləgarmeh",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MUNAH,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("Munach with Passeq; Before Revia"),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_LEGARMEH,
};

// Conjunctives
pub(crate) const MUNACH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Munach",
    hebrew_name: "מוּנַ֣ח",
    hebrew_concept: "resting or placed",
    sbl_academic: "mûnaḥ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MUNAH,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Most common conjunctive; precedes disjunctive accents."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_MUNAH,
};

pub(crate) const MAHPAKH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Mahpakh",
    hebrew_name: "מַהְפַּךְ",
    hebrew_concept: "turning round",
    sbl_academic: "mahpak",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Mehuppakh",
        hebrew_name: "מְהֻפָּ֤ךְ",
        hebrew_concept: "reversed",
        sbl_academic: "məhuppāk",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Conjunctive accent; in poetry can function as Mehuppakh Legarmeh."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_MAHAPAKH,
};

pub(crate) const MERKHA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Merkha",
    hebrew_name: "מֵרְכָא",
    hebrew_concept: "lengthener, prolonging",
    sbl_academic: "mērəkāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MERKHA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_MERKHA,
};

pub(crate) const MERKHA_KEPHULAH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Merkha Kephulah",
    hebrew_name: "מֵרְכָא כְּפוּלָה",
    hebrew_concept: "double lengthener",
    sbl_academic: "mērəkāʾ kəpûlâ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MERKHA_KEFULA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("Merkha duplex"),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_MERKHA_KEFULA,
};

pub(crate) const DARGA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Darga",
    hebrew_name: "דַּרְגָּא",
    hebrew_concept: "step, stair-step",
    sbl_academic: "dargāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_DARGA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Conjunctive accent; synonymous with Qadma in some traditions."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_DARGA,
};

pub(crate) const AZLA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Azla",
    hebrew_name: "אַזְלָא",
    hebrew_concept: "going on (not pausing), depart",
    sbl_academic: "ʾazlāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QADMA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Qadma",
        hebrew_name: "קַדְמָ֨א",
        hebrew_concept: "antiquity or a former state",
        sbl_academic: "qadmāʾ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some("When Geresh: Qadma"),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_QADMA,
};

pub(crate) const TELISHA_QETANNAH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Telisha Qetannah",
    hebrew_name: "תְּלִישָא קְטַנָּה",
    hebrew_concept: "small (short) detached",
    sbl_academic: "təlîšāʾ qəṭannâ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TELISHA_QETANA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Short form of Telisha; conjunctive function."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_TELISHA_QETANA,
};

pub(crate) const GALGAL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Galgal",
    hebrew_name: "גַּלְגַּל",
    hebrew_concept: "wheel, circle",
    sbl_academic: "galgal",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_YERAH_BEN_YOMO,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Jerach Ben Jomo",
        hebrew_name: "יֵרֶח בֶּן יוֹמוֹ",
        hebrew_concept: "moon one day old",
        sbl_academic: "yēreḥ ben yômô",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_YERAH_BEN_YOMO,
};

pub(crate) const MEAYLA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Meayla",
    hebrew_name: "מְאַיְלָא",
    hebrew_concept: "upper, above, higher",
    sbl_academic: "məʾaylāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Mayla",
        hebrew_name: "מַיְלָא",
        hebrew_concept: "that which is above or elevated position",
        sbl_academic: "maylāʾ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(
        r#"Name given to a Tiphcha, when in the same word as Atnach or Silluq.
    This accent is historically the same glyph as Tipcha (U+0596) but functions as a secondary conjunctive 
    in specific syntactic contexts.
    Different traditions use different names: 
        Tipcha (common), Tarḥa, or Me'ayla/Mayela (rare scholarly names)."#,
    ),
    kind: Kind::Secondary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_TIPEHA,
};

pub(crate) const METEG_INFO: AccentMetaData = AccentMetaData {
    english_name: "Meteg",
    hebrew_name: "מֶתֶג",
    hebrew_concept: "to bridle, check, control",
    sbl_academic: "meteg",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_METEG,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Gayah",
        hebrew_name: "גַּעְיָה",
        hebrew_concept: "a lowing",
        sbl_academic: "gaʿyâ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(
        r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak."#,
    ),
    kind: Kind::Secondary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_METEG,
};

/********************************************************
 *                          POETRY
 *******************************************************/

// Disjunctives

// SILLUQ see PROSE section above

// Oleh Weyored compound accent
pub(crate) const OLEH_WEYORED_INFO: AccentMetaData = AccentMetaData {
    english_name: "Oleh Weyored",
    hebrew_name: "עֹלֶה וְיֹרֵד",
    hebrew_concept: "ascending and descending",
    sbl_academic: "ʿōleh wəyōrēd",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_OLE,
        secondary_mark: Some(&CODEPOINT_MERKHA),
    },
    alternate_names: None,
    word_span: WordSpan::TwoWords,
    notes: Some("The primary_mark codepoint is Mehuppakh but located above the consonant. It is then called OLE."),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_OLEH_WEYORED,
};

// ATNACH see PROSE section above

pub(crate) const REVIA_GADOL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Revia Gadol",
    hebrew_name: "רְבִיעַ גָּדוֹל",
    hebrew_concept: "big fourth",
    sbl_academic: "rəbîaʿ gādôl",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetry Level 2 disjunctive; equivalent to Revia in prose."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_REVIA,
};

pub(crate) const REVIA_MUGRASH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    hebrew_concept: "exiled fourth",
    sbl_academic: "rəbîaʿ mugrāš",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: Some(&CODEPOINT_REVIA),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Revia with Geresh secondary mark; indicates exiled position."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_GERESH,
};

pub(crate) const SHALSHELET_GADOL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    hebrew_concept: "large chain or link",
    sbl_academic: "šalšelet gādôl",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetic variant of Shalshelet; larger chain form."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_SHALSHELET_GADOL,
};

pub(crate) const TSINNOR_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tsinnor",
    hebrew_name: "צִנּוֹר",
    hebrew_concept: "pipe or tube",
    sbl_academic: "ṣinnôr",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZINOR,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Zarqa",
        hebrew_name: "זַרְקָא",
        hebrew_concept: "to sprinkle, scatter",
        sbl_academic: "zarqāʾ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetry equivalent of Zarqa; appears in poetic books."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_ZINOR,
};

pub(crate) const REVIA_QATON_INFO: AccentMetaData = AccentMetaData {
    english_name: "Revia Qaton",
    hebrew_name: "רְבִיעַ קָטוֹן",
    hebrew_concept: "small fourth",
    sbl_academic: "rəbîaʿ qāṭôn",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_REVIA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("After that occurs Oleh Weyored"),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_REVIA,
};

pub(crate) const DECHI_INFO: AccentMetaData = AccentMetaData {
    english_name: "Dechi",
    hebrew_name: "דֶּחִי",
    hebrew_concept: "to push or drive away",
    sbl_academic: "deḥî",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_DEHI,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Tiphcha",
        hebrew_name: "טִפְחָא",
        hebrew_concept: "handbreadth or diagonal",
        sbl_academic: "ṭipḥāʾ",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetry Level 2 disjunctive; related to Tiphcha."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_DEHI,
};

// PAZER see PROSE section above

// Mehuppakh Legarmeh compound accent
pub(crate) const MEHUPPAKH_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    hebrew_concept: "reversed to its own",
    sbl_academic: "məhuppāk ləgarmēh",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(
        r#"While the search results suggest that Mahpach Legarmeh as a complete compound term 
        is most distinctly associated with the Ashkenazi tradition, 
        all four traditions recognize the same grammatical function (a Mahpakh functioning disjunctively with a Paseq). 
        The other traditions may use the base name Mahpakh, and determine the Legarmeh function from context, 
        rather than using the full compound name systematically. 
        This reflects a broader pattern where Ashkenazi terminology tends to be more explicit with compound names 
        (e.g., Munach Legarmeh), while Sephardi and other traditions may use simpler designations."#,
    ),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_MEHUPPAKH_LEGARMEH,
};

// Azla Legarmeh compound accent
pub(crate) const AZLA_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    hebrew_concept: "goes to its own",
    sbl_academic: "ʾazlāʾ ləgarmēh",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QADMA,
        secondary_mark: Some(&CODEPOINT_PASEQ),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Compound disjunctive in poetry; Qadma with Paseq."#),
    kind: Kind::Primary,
    category: Category::Disjunctive,
    traditions: TRADITION_NAMES_AZLA_LEGARMEH,
};

// MUNACH see PROSE section above

// MERKHA see PROSE section above

pub(crate) const ILLUY_INFO: AccentMetaData = AccentMetaData {
    english_name: "Illuy",
    hebrew_name: "עִלּוּי",
    hebrew_concept: "elevation or raising",
    sbl_academic: "ʿillûy",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ILUY,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("also called Munach superior"),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_ILUY,
};

pub(crate) const TARCHA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tarcha",
    hebrew_name: "טַרְחָא",
    hebrew_concept: "trouble, difficulty, hardship, toil",
    sbl_academic: "ṭarḥāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_TIPEHA,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some("it refers to the effort, strain, or inconvenience involved in doing something."),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_TIPEHA,
};

// GALGAL see PROSE section above

pub(crate) const MEHUPPAKH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Mehuppakh",
    hebrew_name: "מְהֻפָּ֤ךְ",
    hebrew_concept: "reversed",
    sbl_academic: "məhuppāk",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: None,
    },
    alternate_names: Some(AlternateNames {
        english_name: "Mahpakh",
        hebrew_name: "מַהְפַּךְ",
        hebrew_concept: "turning round",
        sbl_academic: "mahpak",
    }),
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetic conjunctive; variant spelling of Mahpakh."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_MAHAPAKH,
};

// AZLA see PROSE section above

pub(crate) const SHALSHELET_QETANNAH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Shalshelet Qetannah",
    hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
    hebrew_concept: "small chain",
    sbl_academic: "šalšelet qəṭannâ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Poetic conjunctive; smaller chain form."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_SHALSHELET,
};

// Tsinnorit Merkha compound accent
pub(crate) const TSINNORIT_MERKHA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    hebrew_concept: "pipe of continuation",
    sbl_academic: "ṣinnôrīt mērəkāʾ",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MERKHA),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Compound conjunctive in poetry; Tsinnor with Merkha secondary."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_TSINNORIT_MERKHA,
};

// Tsinnorit Mahpakh compound accent
pub(crate) const TSINNORIT_MAHPAKH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    hebrew_concept: "pipe of reversal",
    sbl_academic: "ṣinnôrīt mahpak",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MAHAPAKH),
    },
    alternate_names: None,
    word_span: WordSpan::OneWord,
    notes: Some(r#"Compound conjunctive in poetry; Tsinnor with Mahpakh secondary."#),
    kind: Kind::Primary,
    category: Category::Conjunctive,
    traditions: TRADITION_NAMES_TSINNORIT_MAHPACH,
};

/********************************************************
 *                      PSEUDO ACCENT
 *******************************************************/

pub(crate) const SOPH_PASUQ_INFO: AccentMetaData = AccentMetaData {
    english_name: "Soph Pasuq",
    hebrew_name: "סוֹף פָּסוּק",
    hebrew_concept: "end of verse",
    sbl_academic: "sôp pāsûq",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SOPH_PASUQ,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::NotApplicable,
    notes: Some(
        "it doesn't carry any theological or interpretive meaning beyond marking a boundary",
    ),
    kind: Kind::None,
    category: Category::None,
    traditions: TRADITION_NAMES_SOPH_PASUQ,
};

pub(crate) const MAQQEPH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Maqqeph",
    hebrew_name: "מַקֵּף",
    hebrew_concept: "binder",
    sbl_academic: "maqqēp",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAQAF,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::NotApplicable,
    notes: Some("Can link two (or more) short words together after which they function as a single compound word bearing a single Hebrew accent."),
    kind: Kind::None,
    category: Category::None,
    traditions: TRADITION_NAMES_MAQAF,
};

pub(crate) const PASEQ_INFO: AccentMetaData = AccentMetaData {
    english_name: "Paseq",
    hebrew_name: "פָּסֵק",
    hebrew_concept: "to pause, to stop or to interrupt",
    sbl_academic: "pāsēq",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_PASEQ,
        secondary_mark: None,
    },
    alternate_names: None,
    word_span: WordSpan::NotApplicable,
    notes: Some(
        "It's indicating that someone or something is stopping temporarily or creating a pause.",
    ),
    kind: Kind::None,
    category: Category::None,
    traditions: TRADITION_NAMES_PASEQ,
};
