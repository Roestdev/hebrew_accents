//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://utf8-chartable.de/unicode-utf8-table.pl>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

use crate::codepoints::traditions::{AccentName, TraditionNames};

// ============================================================================
// COMPOUND ACCENT NOT YET IMPLEMENTED
// ============================================================================
/*
pub(crate) const TRADITION_NAMES_SHENE_PASHTIN: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "שְׁנֵי פַּשְׁטִין",
        english_name: "shene pashtin",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּרֵי קַדְמִין",
        english_name: "tere qadmin",
    }),
    italian: Some(AccentName {
        hebrew_name: "(שְׁנֵי) פַּשְׁטִין",
        english_name: "(shene) pashtin",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "Not available from accessible sources",
        english_name: "Not available from accessible sources",
    }),
};
*/

// ============================================================================
// COMPOUND ACCENT
// ============================================================================
/*
SHALSHELET_INFO: AccentMetaData = AccentMetaData {
    english_name: "Shalshelet",
    hebrew_name: "שַׁלְשֶׁלֶת",
    hebrew_concept: "chain or link",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
*/

// ============================================================================
// TRADITION_NAMES_LEGARMEH: MUNAH + PASEQ
// ============================================================================
pub(crate) const TRADITION_NAMES_LEGARMEH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֻנַּח לְגַרְמֵה",
        english_name: "munach legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "פָּסֵק",
        english_name: "paseq",
    }),
    italian: Some(AccentName {
        hebrew_name: "לְגַרְמֵה",
        english_name: "legarmeh",
    }),
    yemenite: None, // "Not available from accessible sources",
};
/*
OLEH_WEYORED_INFO: AccentMetaData = AccentMetaData {
    english_name: "Oleh Weyored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    hebrew_concept: "ascending and descending",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_OLE,
        secondary_mark: Some(&CODEPOINT_MERKHA),
*/
pub(crate) const TRADITION_NAMES_OLEH_WEYORED: TraditionNames =
    TraditionNames::uniform("עֹלֶה וְיֹרֵד", "ascending and descending");

/* REVIA_MUGRASH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    hebrew_concept: "exiled fourth",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: Some(&CODEPOINT_REVIA),
*/

/*
SHALSHELET_GADOL_INFO: AccentMetaData = AccentMetaData {
    english_name: "Shalshelet Gadol",
    hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
    hebrew_concept: "large chain or link",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_SHALSHELET,
        secondary_mark: Some(&CODEPOINT_PASEQ),
 */
pub(crate) const TRADITION_NAMES_SHALSHELET_GADOL: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "סְגוֹל֒",
        english_name: "segol",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "סְגוֹלְתָּא֒",
        english_name: "segolta",
    }),
    italian: Some(AccentName {
        hebrew_name: "שְׁרֵי֒",
        english_name: "shere",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
MEHUPPAKH_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Mehuppakh Legarmeh",
    hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
    hebrew_concept: "reversed to its own",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_MAHAPAKH,
        secondary_mark: Some(&CODEPOINT_PASEQ),
*/
pub(crate) const TRADITION_NAMES_MEHUPPAKH_LEGARMEH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "סְמַהְפָּךְ לְגַרְמֵהּ / מֶהוּפָּךְ",
        english_name: "Mahpach Legarmeh/Mehuppakh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "מַהְפָּךְ",
        english_name: "Mahpakh (Legarmeh implied)",
    }),
    italian: Some(AccentName {
        hebrew_name: "מַהְפָּךְ",
        english_name: "Mahpakh (Legarmeh implied)",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מַהְפָּךְ / מֶהוּפָּךְ",
        english_name: "Mahpakh/Mehuppakh",
    }),
};

/*
AZLA_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Azla Legarmeh",
    hebrew_name: "אַזְלָא לְגַרְמֶהּ",
    hebrew_concept: "goes to its own",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_QADMA,
        secondary_mark: Some(&CODEPOINT_PASEQ),
*/
pub(crate) const TRADITION_NAMES_AZLA_LEGARMEH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
TSINNORIT_MERKHA_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tsinnorit Merkha",
    hebrew_name: "צִנּוֹרִת מֵרְכָא",
    hebrew_concept: "pipe of continuation",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MERKHA),
 */
pub(crate) const TRADITION_NAMES_TSINNORIT_MERKHA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
TSINNORIT_MAHPAKH_INFO: AccentMetaData = AccentMetaData {
    english_name: "Tsinnorit Mahpakh",
    hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
    hebrew_concept: "pipe of reversal",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_ZARQA,
        secondary_mark: Some(&CODEPOINT_MAHAPAKH),

*/
pub(crate) const TRADITION_NAMES_TSINNORIT_MAHPACH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        english_name: "Qadma Legarmeh",
    }),
    yemenite: None, // Not available from accessible sources
};

// ============================================================================
// NON-COMPOUND ACCENT
// ============================================================================

// ============================================================================
// ETNAHTA (U+0591)
// ============================================================================
pub(crate) const TRADITION_NAMES_ETNAHTA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "אֶתְנַחְתָּ֑א",
        english_name: "Etnachta",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַתְנָ֑ח",
        english_name: "Atnach",
    }),
    italian: Some(AccentName {
        hebrew_name: "אַתְנָ֑ח",
        english_name: "Atnach",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "אֶתְנָחָ֑א",
        english_name: "Etnacha",
    }),
};

// ============================================================================
// SEGOL (U+0592) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_SEGOL: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "סְגוֹל֒",
        english_name: "segol",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "סְגוֹלְתָּא֒",
        english_name: "segolta",
    }),
    italian: Some(AccentName {
        hebrew_name: "שְׁרֵי֒",
        english_name: "shere",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// SHALSHELET (U+0593)
// ============================================================================
pub(crate) const TRADITION_NAMES_SHALSHELET: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        english_name: "shalshelet",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        english_name: "shalshelet",
    }),
    italian: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        english_name: "shalshelet",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "שִׁישְׁלָ֓א",
        english_name: "shishla",
    }),
};

// ============================================================================
// ZAQEF QATAN (U+0594)
// ============================================================================
pub(crate) const TRADITION_NAMES_ZAQEF_QATAN: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "זָקֵף קָטָ֔ן",
        english_name: "zaqeph qatan",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "זָקֵף קָט֔וֹן",
        english_name: "zaqeph qaton",
    }),
    italian: Some(AccentName {
        hebrew_name: "זָקֵף קָט֔וֹן",
        english_name: "zaqeph qaton",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "זָקֵף קָט֔וֹן",
        english_name: "zaqeph qaton",
    }),
};

// ============================================================================
// ZAQEF GADOL (U+0595) - All identical (use uniform!)
// ============================================================================
pub(crate) const TRADITION_NAMES_ZAQEF_GADOL: TraditionNames =
    TraditionNames::uniform("זָקֵף גָּד֕וֹל", "zaqeph gadol");

// ============================================================================
// TIPEHA (U+0596)
// ============================================================================
pub(crate) const TRADITION_NAMES_TIPEHA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "טִפְחָ֖א",
        english_name: "tiphcha",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "טַרְחָ֖א",
        english_name: "tarcha",
    }),
    italian: Some(AccentName {
        hebrew_name: "טַרְחָ֖א",
        english_name: "tarcha",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "נְטוּיָ֖ה",
        english_name: "netuyah",
    }),
};

// ============================================================================
// REVIA (U+0597) - Ashkenazi differs slightly
// ============================================================================
pub(crate) const TRADITION_NAMES_REVIA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "רְבִ֗יע",
        english_name: "revia/revi'i",
    }),
    ..TraditionNames::uniform("רְבִ֗יע", "revia")
};

// ============================================================================
// ZARQA (U+0598)
// ============================================================================
pub(crate) const TRADITION_NAMES_ZARQA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        english_name: "zarqa",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        english_name: "zarqa",
    }),
    italian: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        english_name: "zarqa",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "צִנּוֹר֘",
        english_name: "tsinnor",
    }),
};

// ============================================================================
// PASHTA (U+0599)
// ============================================================================
pub(crate) const TRADITION_NAMES_PASHTA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "פַּשְׁטָא֙",
        english_name: "pashta",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "קַדְמָא֙",
        english_name: "qadma",
    }),
    italian: Some(AccentName {
        hebrew_name: "פַּשְׁטָא֙",
        english_name: "pashta",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "אַזְלָא֙",
        english_name: "azla",
    }),
};

// ============================================================================
// YETIV (U+059A)
// ============================================================================
pub(crate) const TRADITION_NAMES_YETIV: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        english_name: "yetiv",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        english_name: "yetiv",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁ֚וֹפָר יְתִיב",
        english_name: "shophar yetiv",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        english_name: "yetiv",
    }),
};

// ============================================================================
// TEVIR (U+059B)
// ============================================================================
pub(crate) const TRADITION_NAMES_TEVIR: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        english_name: "tevir",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        english_name: "tevir",
    }),
    italian: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        english_name: "tevir",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תַּבְרָ֛א",
        english_name: "tavra",
    }),
};

// ============================================================================
// GERESH (U+059C)
// ============================================================================
pub(crate) const TRADITION_NAMES_GERESH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "גֵּ֜רֵשׁ",
        english_name: "geresh/azla",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "גְּרִ֜ישׁ",
        english_name: "gerish",
    }),
    italian: Some(AccentName {
        hebrew_name: "גֵּ֜רֵשׁ",
        english_name: "geresh/azla",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "טָרֵ֜ס",
        english_name: "tares",
    }),
};

// Geresh Muqdam is disabled - see ticket

// ============================================================================
// GERSHAYIM (U+059E)
// ============================================================================
pub(crate) const TRADITION_NAMES_GERSHAYIM: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "גֵּרְשַׁ֞יִם",
        english_name: "gershayim",
    }),
    ..TraditionNames::uniform("שְׁנֵי גְרִישִׁ֞ין", "shene gerishin")
};

// ============================================================================
// QARNEY PARA (U+059F) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_QARNEY_PARA: TraditionNames =
    TraditionNames::uniform("קַרְנֵי פָרָ֟ה", "qarne pharah");

// ============================================================================
// TELISHA GEDOLA (U+05A0)
// ============================================================================
pub(crate) const TRADITION_NAMES_TELISHA_GEDOLA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
        english_name: "telisha gedolah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תִּ֠רְצָה",
        english_name: "tirtsah",
    }),
    italian: Some(AccentName {
        hebrew_name: "תַּ֠לְשָׁא",
        english_name: "talsha",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
        english_name: "telisha gedolah",
    }),
};

// ============================================================================
// PAZER (U+05A1) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_PAZER: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "פָּזֵ֡ר",
        english_name: "pazer",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "פָּזֵר גָּד֡וֹל",
        english_name: "pazer gadol",
    }),
    italian: Some(AccentName {
        hebrew_name: "פָּזֵר גָּד֡וֹל",
        english_name: "pazer gadol",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// At nah Hafukh disabled - see ticket

// ============================================================================
// MUNAH (U+05A3) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_MUNAH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מוּנַ֣ח",
        english_name: "munach",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שׁוֹפָר הוֹלֵ֣ךְ",
        english_name: "shophar holech",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁוֹפָר עִלּ֣וּי",
        english_name: "shophar illuy",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// MAHPAKH (U+05A4)
// ============================================================================
pub(crate) const TRADITION_NAMES_MAHAPAKH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מַהְפַּ֤ך",
        english_name: "mahpach",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        english_name: "(shophar) mehuppakh",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁוֹפָר הָפ֤וּךְ",
        english_name: "shophar haphuch",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מְהֻפָּ֤ךְ",
        english_name: "mehuppakh",
    }),
};

// ============================================================================
// MERKHA (U+05A5)
// ============================================================================
pub(crate) const TRADITION_NAMES_MERKHA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֵרְכָ֥א",
        english_name: "mercha",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "מַאֲרִ֥יךְ",
        english_name: "maarich",
    }),
    italian: Some(AccentName {
        hebrew_name: "מַאֲרִ֥יךְ",
        english_name: "maarich",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מַאֲרְכָ֥א",
        english_name: "maarcha",
    }),
};

// ============================================================================
// MERKHA KEFULA (U+05A6) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_MERKHA_KEFULA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֵרְכָא כּפוּלָ֦ה",
        english_name: "mercha Kephulah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּרֵי טַעֲמֵ֦י",
        english_name: "tere taame",
    }),
    italian: Some(AccentName {
        hebrew_name: "תְּרֵין חוּטְרִ֦ין",
        english_name: "teren chutrin",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// DARGA (U+05A7) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_DARGA: TraditionNames = TraditionNames::uniform("דַּרְגָּ֧א", "darga");

// ============================================================================
// QADMA (U+05A8) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_QADMA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "קַדְמָ֨א",
        english_name: "qadma",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָ֨א",
        english_name: "azla",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָ֨א",
        english_name: "qadma",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// TELISHA QETANA (U+05A9)
// ============================================================================
pub(crate) const TRADITION_NAMES_TELISHA_QETANA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "תְּלִישָא קְטַנָּה֩",
        english_name: "telisha qetannah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תַּלְשָׁא֩",
        english_name: "talsha",
    }),
    italian: Some(AccentName {
        hebrew_name: "תַּרְסָא֩",
        english_name: "tarsa",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תְּלִישָא קְטַנָּה֩",
        english_name: "telisha qetannah",
    }),
};

// ============================================================================
// YERAH BEN YOMO (U+05AA) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_YERAH_BEN_YOMO: TraditionNames =
    TraditionNames::uniform("יֵרֶח בֶּן יוֹמ֪וֹ", "yerach ben yomo");

// ============================================================================
// OLE (U+05AB) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_OLE: TraditionNames = TraditionNames::uniform("עוֹלֶה", "oleh");

// ============================================================================
// ILUY (U+05AC) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_ILUY: TraditionNames = TraditionNames::uniform("עִלוּי", "iluy");

// ============================================================================
// DEHI (U+05AD) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_DEHI: TraditionNames = TraditionNames::uniform("דחי", "dechi");

// ============================================================================
// ZINOR (U+05AE) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_ZINOR: TraditionNames =
    TraditionNames::uniform("צנור", "tsinor (zarqa above left)");

// ============================================================================
// SILLUQ (U+05BD) - All identical (same codepoint as Meteg, different semantics)
// ============================================================================
pub(crate) const TRADITION_NAMES_SILLUQ: TraditionNames =
    TraditionNames::uniform("סוֹף פָּסֽוּק", "sof pasuq");

// ============================================================================
// METEG (U+05BD) - All identical (shares codepoint with Silluq)
// ============================================================================
pub(crate) const TRADITION_NAMES_METEG: TraditionNames = TraditionNames::uniform("מֶתֶג", "meteg");

// ============================================================================
// MAQAF (U+05BE) - No traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_MAQAF: TraditionNames = TraditionNames {
    ashkenazi: None,
    sephardi: None,
    italian: None,
    yemenite: None,
};

// ============================================================================
// PASEQ (U+05C0) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_PASEQ: TraditionNames = TraditionNames::uniform("פָּסֵק", "paseq");

// ============================================================================
// SOPH PASUQ (U+05C3) - No traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_SOPH_PASUQ: TraditionNames = TraditionNames {
    ashkenazi: None,
    sephardi: None,
    italian: None,
    yemenite: None,
};
