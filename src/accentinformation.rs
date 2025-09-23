use crate::accent::AccentInfo;
use crate::codepoints::*;



//////////////////////////////////////////////////////////////////////
pub const SILLUQ_INFO: AccentInfo = AccentInfo {
    english_name: "Silluq",
    hebrew_name: "סִלּוּק",
    meaning: "close, cessation",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_SILLUQ,
    second_code_point: None,
    comment: Some(
        "The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.",
    ),
};

pub const ATNACH_INFO: AccentInfo = AccentInfo {
    english_name: "Atnach",
    hebrew_name: "אתְנָח",
    meaning: "a causing to rest",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ETNAHTA,
    second_code_point: None,
    comment: None,
};
pub const SEGOLTA_INFO: AccentInfo = AccentInfo {
    english_name: "Segolta",
    hebrew_name: "סְגֹולְתָּא",
    meaning: "a little grape-bunch",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_SEGOL,
    second_code_point: None,
    comment: None,
};
pub const SHALSHELET_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    meaning: "chain or link",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_SHALSHELET,
    second_code_point: Some(&CP_PASEQ),
    comment: None,
};
pub const ZAQEF_QATON_INFO: AccentInfo = AccentInfo {
    english_name: "Zaqeph Qaton",
    hebrew_name: "זָקֵף קָטוֹן",
    meaning: "small upright",
    alt_english_name: Some("Zaqeph Qatan"),
    alt_hebrew_name: Some("זָקֵף קָטָן"),
    alt_meaning: None,
    first_code_point: &CP_ZAQEF_QATAN,
    second_code_point: None,
    comment: None,
};
pub const ZAQEPH_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Zaqeph Gadol",
    hebrew_name: "זָקֵף גָּדוֹל",
    meaning: "large upright",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ZAQEF_GADOL,
    second_code_point: None,
    comment: None,
};
pub const REVIA_INFO: AccentInfo = AccentInfo {
    english_name: "Revia",
    hebrew_name: "רְבִיעַ",
    meaning: "fourth [in a sequence]",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_REVIA,
    second_code_point: None,
    comment: Some("probably due to its four-note tune."),
};
pub const TIPHCHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tiphcha",
    hebrew_name: "טִפְחָא",
    meaning: "handbreadth or diagonal",
    alt_english_name: None, // ADD TARCHA
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_TIPEHA,
    second_code_point: None,
    comment: Some("before Atnach and Silluq"),
};
pub const ZARQA_INFO: AccentInfo = AccentInfo {
    english_name: "Zarqa",
    hebrew_name: "זַרְקָא",
    meaning: "to sprinkle, scatter",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ZINOR,
    second_code_point: None,
    comment: Some("before  Segolta"),
};
pub const PASHTA_INFO: AccentInfo = AccentInfo {
    english_name: "Pashta",
    hebrew_name: "פַּשְׁטָא",
    meaning: "extending, stretching out in length",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_PASHTA,
    second_code_point: None,
    comment: Some("if you sound almost last (2 pasta’s in one word)"),
};
pub const YETIV_INFO: AccentInfo = AccentInfo {
    english_name: "Yetiv",
    hebrew_name: "יְתִיב",
    meaning: "resting or sitting",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_YETIV,
    second_code_point: None,
    comment: Some("occasionally for a Pashta"),
};
pub const TEVIR_INFO: AccentInfo = AccentInfo {
    english_name: "Tevir",
    hebrew_name: "תְּבִיר",
    meaning: "broken, downward tumble",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_TEVIR,
    second_code_point: None,
    comment: None,
};
pub const GERESH_INFO: AccentInfo = AccentInfo {
    english_name: "Geresh",
    hebrew_name: "גֵּרֵישׁ",
    meaning: "expulsion, driving out, divorce",
    alt_english_name: Some("Teres"),
    alt_hebrew_name: Some("טֶרֶס"),
    alt_meaning: None,
    first_code_point: &CP_GERESH,
    second_code_point: None,
    comment: None,
};
pub const GERSHAYIM_INFO: AccentInfo = AccentInfo {
    english_name: "Gershayim",
    hebrew_name: "גֵּרְשַׁיִם",
    meaning: "double of expulsion, driving out, divorce",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_GERSHAYIM,
    second_code_point: None,
    comment: None,
};
pub const PAZER_INFO: AccentInfo = AccentInfo {
    english_name: "Pazer",
    hebrew_name: "פָּזֶר",
    meaning: "lavish or scatter",
    alt_english_name: Some("Pazer Qatan"),
    alt_hebrew_name: Some("פָּזֵר קָטָן"),
    alt_meaning: Some("small lavish or scatter"),
    first_code_point: &CP_PAZER,
    second_code_point: None,
    comment: None,
};
pub const PAZER_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Pazer Gadol",
    hebrew_name: "פָּזֶר גּדוֹל",
    meaning: "large lavish or scatter",
    alt_english_name: Some("Qarne Pharah"),
    alt_hebrew_name: Some("קַרְנֵי פָרָה"),
    alt_meaning: Some("horns of a cow"),
    first_code_point: &CP_QARNEY_PARA,
    second_code_point: None,
    comment: None,
};
pub const TELISHA_GEDOLAH_INFO: AccentInfo = AccentInfo {
    english_name: "Telisha Gedolah",
    hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
    meaning: "great (long) detached",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_TELISHA_GEDOLA,
    second_code_point: None,
    comment: None,
};
pub const LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Legarmeh",
    hebrew_name: "לְגַרְמֶהּ",
    meaning: "for or by itself, independant",
    alt_english_name: Some("Munach Legarmeh"),
    alt_hebrew_name: Some("מוּנַח לְגַרְמֵ֣הּ"),
    alt_meaning: None,
    first_code_point: &CP_MUNAH,
    second_code_point: Some(&CP_PASEQ),
    comment: Some("Munach with passeq; Before Revia"),
};
// Conjunctives
pub const MUNACH_INFO: AccentInfo = AccentInfo {
    english_name: "Munach",
    hebrew_name: "מוּנַ֣ח",
    meaning: "resting or placed",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_MUNAH,
    second_code_point: None,
    comment: None,
};
pub const MAHPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Mahpakh",
    hebrew_name: "מַהְפַּךְ",
    meaning: "turning round",
    alt_english_name: Some("Mehuppakh"),
    alt_hebrew_name: Some("מְהֻפָּ֤ךְ"),
    alt_meaning: Some("reversed"),
    first_code_point: &CP_MAHAPAKH,
    second_code_point: None,
    comment: None,
};
pub const MEHUPPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Mehuppakh",
    hebrew_name: "מְהֻפָּ֤ךְ",
    meaning: "reversed",
    alt_english_name: Some("Mahpakh"),
    alt_hebrew_name: Some("מַהְפַּךְ"),
    alt_meaning: Some("turning round"),
    first_code_point: &CP_MAHAPAKH,
    second_code_point: None,
    comment: None,
};

pub const MERKHA_INFO: AccentInfo = AccentInfo {
    english_name: "Merkha",
    hebrew_name: "מֵרְכָא",
    meaning: "lengthener, prolonging",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_MERKHA,
    second_code_point: None,
    comment: None,
};
pub const MERKHA_KEFULAH_INFO: AccentInfo = AccentInfo {
    english_name: "Merkha Kephulah",
    hebrew_name: "מֵרְכָא כְּפוּלָה",
    meaning: "double lengthener",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_MERKHA_KEFULA,
    second_code_point: None,
    comment: Some("Merkha duplex"),
};
pub const DARGA_INFO: AccentInfo = AccentInfo {
    english_name: "Darga",
    hebrew_name: "דַּרְגָּא",
    meaning: "stairstep",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_DARGA,
    second_code_point: None,
    comment: None,
};
pub const AZLA_INFO: AccentInfo = AccentInfo {
    english_name: "Azla",
    hebrew_name: "אַזְלָא",
    meaning: "going on (not pausing), depart",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_QADMA,
    second_code_point: None,
    comment: Some("When Geresh: Qadma"),
};
pub const TELISHA_QETANNAH_INFO: AccentInfo = AccentInfo {
    english_name: "Telisha Qetannah",
    hebrew_name: "תְּלִישָא קְטַנָּה",
    meaning: "small (short) detached",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_TELISHA_QETANA,
    second_code_point: None,
    comment: None,
};
pub const GALGAL_INFO: AccentInfo = AccentInfo {
    english_name: "Galgal",
    hebrew_name: "גַּלְגַּל",
    meaning: "wheel, circle",
    alt_english_name: Some("Jerach Ben Jomo"),
    alt_hebrew_name: Some("יֵרֶח בֶּן יוֹמוֹ"),
    alt_meaning: Some("moon one day old"),
    first_code_point: &CP_YERAH_BEN_YOMO,
    second_code_point: None,
    comment: None,
};
pub const MAYELA_INFO: AccentInfo = AccentInfo {
    english_name: "Mayela",
    hebrew_name: "מָאיְלָא",
    meaning: "to be raised or elevated",
    alt_english_name: Some("Meayyela"),
    alt_hebrew_name: Some("מְאַיְּלָא"),
    alt_meaning: None,
    first_code_point: &CP_TIPEHA,
    second_code_point: None,
    comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq"),
};
pub const METEG_INFO: AccentInfo = AccentInfo {
    english_name: "Meteg",
    hebrew_name: "מֶתֶג",
    meaning: "accent or mark",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_METEG,
    second_code_point: None,
    comment: None,
};
pub const MAQQEPH_INFO: AccentInfo = AccentInfo {
                english_name: "Maqqeph",
                hebrew_name: "מַקֵּף",
                meaning:"binder",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: &CP_MAQAF,
                second_code_point: None,
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
pub const OLEH_WE_YORED_INFO: AccentInfo = AccentInfo {
    english_name: "Oleh We Yored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    meaning: "ascending and descending",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_OLE,
    second_code_point: Some(&CP_MAHAPAKH),
    comment: None,
};
pub const REVIA_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Gadol",
    hebrew_name: "רְבִיעַ גּדוֹל",
    meaning: "big fourth",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_REVIA,
    second_code_point: None,
    comment: None,
};
pub const REVIA_MUGRASH_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    meaning: "exiled fourth",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_GERESH,
    second_code_point: Some(&CP_REVIA),
    comment: None,
};
pub const SHALSHELET_GADOL_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    meaning: "large chain or link",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_SHALSHELET,
    second_code_point: Some(&CP_PASEQ),
    comment: None,
};
pub const TSINNOR_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnor",
    hebrew_name: "צִנּוֹר",
    meaning: "pipe or tube",
    alt_english_name: Some("Zarqa"),
    alt_hebrew_name: Some("זַרְקָא"),
    alt_meaning: Some("to sprinkle, scatter"),
    first_code_point: &CP_ZINOR,
    second_code_point: None,
    comment: None,
};
pub const REVIA_QATON_INFO: AccentInfo = AccentInfo {
    english_name: "Revia Qaton",
    hebrew_name: "רְבִיעַ קָטוֹן",
    meaning: "small fourth",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_REVIA,
    second_code_point: None,
    comment: Some("After that occurs Oleh we yored"),
};
pub const DECHI_INFO: AccentInfo = AccentInfo {
    english_name: "Dechi",
    hebrew_name: "דֶּחִי",
    meaning: "to push or drive away",
    alt_english_name: Some("Tiphcha"),
    alt_hebrew_name: Some("טִפְחָא"),
    alt_meaning: Some("handbreadth or diagonal"),
    first_code_point: &CP_DEHI,
    second_code_point: None,
    comment: None,
};
pub const MEHUPPAKH_LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    meaning: "reversed to its own",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_MAHAPAKH,
    second_code_point: Some(&CP_PASEQ),
    comment: None,
};
pub const AZLA_LEGARMEH_INFO: AccentInfo = AccentInfo {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    meaning: "goes to its own",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_QADMA,
    second_code_point: Some(&CP_PASEQ),
    comment: None,
};
// Conjunctives

//     HebrewAccent::Poetry(PoetryAccent::Munach) => AccentInfo {
//         english_name: "Munach",
//         hebrew_name: "מֻנַּח",
//         meaning:"rest or placed",
//         alt_english_name: None,
//         alt_hebrew_name: None,
//         alt_meaning: None,
//         acc_type: AccentType::Primary,
//         acc_category: AccentCategory::Conjunctive,
//         first_code_point: &CP_MUNAH,
//         second_code_point: None,
//         comment: None,
// },
// HebrewAccent::Poetry(PoetryAccent::Merkha)  {
//     english_name: "Merkha",
//     hebrew_name: "מֵרְכָא",
//     meaning:"lengthener",
//     alt_english_name: None,
//     alt_hebrew_name: None,
//     alt_meaning: None,
//     acc_type: AccentType::Primary,
//     acc_category: AccentCategory::Conjunctive,
//     first_code_point: &CP_MERKHA,
//     second_code_point: None,
//     comment: None,
// },
pub const ILLUY_INFO: AccentInfo = AccentInfo {
    english_name: "Illuy",
    hebrew_name: "עִלּוּי",
    meaning: "elevation or raising",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ILUY,
    second_code_point: None,
    comment: None,
};
pub const TARCHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tiphcha",
    hebrew_name: "טִפְחָא",
    meaning: "handbreadth or diagonal",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_TIPEHA,
    second_code_point: None,
    comment: None,
};
// pub const SILLUQ_INFO: AccentInfo = AccentInfo {
//     english_name: "Mehuppakh",
//     hebrew_name: "מְהֻפַּך",
//     meaning:"reversed or turned around",
//     alt_english_name: Some("Mahpakh"),
//     alt_hebrew_name: Some("מַהְפַּךְ"),
//     alt_meaning: Some("turning round"),
//     acc_type: AccentType::Primary,
//     acc_category: AccentCategory::Conjunctive,
//     first_code_point: &CP_MAHAPAKH,
//     second_code_point: None,
//     comment: None,
// },
// pub const AZLA_INFO: AccentInfo = AccentInfo {
//                 english_name: "Azla",
//                 hebrew_name: "אַזְלָא֙",
//                 meaning:"going away",
//                 alt_english_name: Some("Qadma"),
//                 alt_hebrew_name: Some("קַדְמָ֨א"),
//                 alt_meaning: Some("antiquity or a former state"),
//                 first_code_point: &CP_QADMA,
//                 second_code_point: None,
//     comment: None,
// };
pub const SHALSHELET_QETANNAH_INFO: AccentInfo = AccentInfo {
    english_name: "Shalshelet Qetannah",
    hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
    meaning: "small chain",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_SHALSHELET,
    second_code_point: None,
    comment: None,
};
pub const TSINNORIT_MERKHA_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    meaning: "pipe of continuation",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ZARQA,
    second_code_point: Some(&CP_MERKHA),
    comment: None,
};
pub const TSINNORIT_MAHPAKH_INFO: AccentInfo = AccentInfo {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    meaning: "pipe of reversal",
    alt_english_name: None,
    alt_hebrew_name: None,
    alt_meaning: None,
    first_code_point: &CP_ZARQA,
    second_code_point: Some(&CP_MAHAPAKH),
    comment: None,
};

