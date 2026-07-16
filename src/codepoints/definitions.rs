//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://utf8-chartable.de/unicode-utf8-table.pl>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

use crate::accent::Utf8CodePointInfo;
use crate::codepoints::traditions::{AccentName, TraditionNames};
use crate::codepoints::CodePointPosition;

const fn utf8_cp_constructor(
    code_point_value: &'static str,
    hex_value: &'static str,
    name: &'static str,
    symbol: &'static str,
    position: CodePointPosition,
    traditions: TraditionNames,
) -> Utf8CodePointInfo {
    Utf8CodePointInfo {
        code_point_value,
        hex_value,
        name,
        symbol,
        position,
        traditions,
    }
}

// ============================================================================
// ETNAHTA (U+0591)
// ============================================================================
pub(crate) const TRAD_ETNAHTA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_ETNAHTA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0591",
    "0xd6 0x91",
    "HEBREW ACCENT ETNAHTA",
    "֑",
    CodePointPosition::Under,
    TRAD_ETNAHTA,
);

// ============================================================================
// SEGOL (U+0592) - Only 3 traditions
// ============================================================================
pub(crate) const TRAD_SEGOL: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_SEGOL: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0592",
    "0xd6 0x92",
    "HEBREW ACCENT SEGOL",
    "֒",
    CodePointPosition::Above,
    TRAD_SEGOL,
);

// ============================================================================
// SHALSHELET (U+0593)
// ============================================================================
pub(crate) const TRAD_SHALSHELET: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_SHALSHELET: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0593",
    "0xd6 0x93",
    "HEBREW ACCENT SHALSHELET",
    "֓",
    CodePointPosition::Above,
    TRAD_SHALSHELET,
);

// ============================================================================
// ZAQEF QATAN (U+0594)
// ============================================================================
pub(crate) const TRAD_ZAQEF_QATAN: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_ZAQEF_QATAN: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0594",
    "0xd6 0x94",
    "HEBREW ACCENT ZAQEF QATAN",
    "֔",
    CodePointPosition::Above,
    TRAD_ZAQEF_QATAN,
);

// ============================================================================
// ZAQEF GADOL (U+0595) - All identical (use uniform!)
// ============================================================================
pub(crate) const TRAD_ZAQEF_GADOL: TraditionNames =
    TraditionNames::uniform("זָקֵף גָּד֕וֹל", "zaqeph gadol");

pub(crate) const CODEPOINT_ZAQEF_GADOL: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0595",
    "0xd6 0x95",
    "HEBREW ACCENT ZAQEF GADOL",
    "֕",
    CodePointPosition::Above,
    TRAD_ZAQEF_GADOL,
);

// ============================================================================
// TIPEHA (U+0596)
// ============================================================================
pub(crate) const TRAD_TIPEHA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_TIPEHA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0596",
    "0xd6 0x96",
    "HEBREW ACCENT TIPEHA",
    "֖",
    CodePointPosition::Under,
    TRAD_TIPEHA,
);

// ============================================================================
// REVIA (U+0597) - Ashkenazi differs slightly
// ============================================================================
pub(crate) const TRAD_REVIA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "רְבִ֗יע",
        english_name: "revia/revi'i",
    }),
    ..TraditionNames::uniform("רְבִ֗יע", "revia")
};

pub(crate) const CODEPOINT_REVIA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0597",
    "0xd6 0x97",
    "HEBREW ACCENT REVIA",
    "֗",
    CodePointPosition::Above,
    TRAD_REVIA,
);

// ============================================================================
// ZARQA (U+0598)
// ============================================================================
pub(crate) const TRAD_ZARQA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_ZARQA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0598",
    "0xd6 0x98",
    "HEBREW ACCENT ZARQA",
    "֘",
    CodePointPosition::Above,
    TRAD_ZARQA,
);

// ============================================================================
// PASHTA (U+0599)
// ============================================================================
pub(crate) const TRAD_PASHTA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_PASHTA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0599",
    "0xd6 0x99",
    "HEBREW ACCENT PASHTA",
    "֙",
    CodePointPosition::Above,
    TRAD_PASHTA,
);

// ============================================================================
// YETIV (U+059A)
// ============================================================================
pub(crate) const TRAD_YETIV: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_YETIV: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059A",
    "0xd6 0x9a",
    "HEBREW ACCENT YETIV",
    "֚",
    CodePointPosition::Under,
    TRAD_YETIV,
);

// ============================================================================
// TEVIR (U+059B)
// ============================================================================
pub(crate) const TRAD_TEVIR: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_TEVIR: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059B",
    "0xd6 0x9b",
    "HEBREW ACCENT TEVIR",
    "֛",
    CodePointPosition::Under,
    TRAD_TEVIR,
);

// ============================================================================
// GERESH (U+059C)
// ============================================================================
pub(crate) const TRAD_GERESH: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_GERESH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059C",
    "0xd6 0x9c",
    "HEBREW ACCENT GERESH",
    "֜",
    CodePointPosition::Above,
    TRAD_GERESH,
);

// Geresh Muqdam is disabled - see ticket

// ============================================================================
// GERSHAYIM (U+059E)
// ============================================================================
pub(crate) const TRAD_GERSHAYIM: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "גֵּרְשַׁ֞יִם",
        english_name: "gershayim",
    }),
    ..TraditionNames::uniform("שְׁנֵי גְרִישִׁ֞ין", "shene gerishin")
};

pub(crate) const CODEPOINT_GERSHAYIM: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059E",
    "0xd6 0x9e",
    "HEBREW ACCENT GERSHAYIM",
    "֞",
    CodePointPosition::Above,
    TRAD_GERSHAYIM,
);

// ============================================================================
// QARNEY PARA (U+059F) - All identical
// ============================================================================
pub(crate) const TRAD_QARNEY_PARA: TraditionNames =
    TraditionNames::uniform("קַרְנֵי פָרָ֟ה", "qarne pharah");

pub(crate) const CODEPOINT_QARNEY_PARA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059F",
    "0xd6 0x9f",
    "HEBREW ACCENT QARNEY PARA",
    "֟",
    CodePointPosition::Above,
    TRAD_QARNEY_PARA,
);

// ============================================================================
// TELISHA GEDOLA (U+05A0)
// ============================================================================
pub(crate) const TRAD_TELISHA_GEDOLA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_TELISHA_GEDOLA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A0",
    "0xd6 0xa0",
    "HEBREW ACCENT TELISHA GEDOLA",
    "֠",
    CodePointPosition::Above,
    TRAD_TELISHA_GEDOLA,
);

// ============================================================================
// PAZER (U+05A1) - Only 3 traditions
// ============================================================================
pub(crate) const TRAD_PAZER: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_PAZER: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A1",
    "0xd6 0xa1",
    "HEBREW ACCENT PAZER",
    "֡",
    CodePointPosition::Above,
    TRAD_PAZER,
);

// At nah Hafukh disabled - see ticket

// ============================================================================
// MUNAH (U+05A3) - Only 3 traditions
// ============================================================================
pub(crate) const TRAD_MUNAH: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_MUNAH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A3",
    "0xd6 0xa3",
    "HEBREW ACCENT MUNAH",
    "֣",
    CodePointPosition::Under,
    TRAD_MUNAH,
);

// ============================================================================
// MAHPAKH (U+05A4)
// ============================================================================
pub(crate) const TRAD_MAHAPAKH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מַהְפַּ֤ך",
        english_name: "mahpach",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        english_name: "(shophar) mehuppach",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁוֹפָר הָפ֤וּךְ",
        english_name: "shophar haphuch",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מְהֻפָּ֤ךְ",
        english_name: "mehuppach",
    }),
};

pub(crate) const CODEPOINT_MAHAPAKH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A4",
    "0xd6 0xa4",
    "HEBREW ACCENT MAHAPAKH",
    "֤",
    CodePointPosition::Under,
    TRAD_MAHAPAKH,
);

// ============================================================================
// MERKHA (U+05A5)
// ============================================================================
pub(crate) const TRAD_MERKHA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_MERKHA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A5",
    "0xd6 0xa5",
    "HEBREW ACCENT MERKHA",
    "֥",
    CodePointPosition::Under,
    TRAD_MERKHA,
);

// ============================================================================
// MERKHA KEFULA (U+05A6) - Only 3 traditions
// ============================================================================
pub(crate) const TRAD_MERKHA_KEFULA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֵרְכָא כּפוּלָ֦ה",
        english_name: "mercha kefulah",
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

pub(crate) const CODEPOINT_MERKHA_KEFULA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A6",
    "0xd6 0xa6",
    "HEBREW ACCENT MERKHA KEFULA",
    "֦",
    CodePointPosition::Under,
    TRAD_MERKHA_KEFULA,
);

// ============================================================================
// DARGA (U+05A7) - All identical
// ============================================================================
pub(crate) const TRAD_DARGA: TraditionNames = TraditionNames::uniform("דַּרְגָּ֧א", "darga");

pub(crate) const CODEPOINT_DARGA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A7",
    "0xd6 0xa7",
    "HEBREW ACCENT DARGA",
    "֧",
    CodePointPosition::Under,
    TRAD_DARGA,
);

// ============================================================================
// QADMA (U+05A8) - Only 3 traditions
// ============================================================================
pub(crate) const TRAD_QADMA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_QADMA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A8",
    "0xd6 0xa8",
    "HEBREW ACCENT QADMA",
    "֨",
    CodePointPosition::Above,
    TRAD_QADMA,
);

// ============================================================================
// TELISHA QETANA (U+05A9)
// ============================================================================
pub(crate) const TRAD_TELISHA_QETANA: TraditionNames = TraditionNames {
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

pub(crate) const CODEPOINT_TELISHA_QETANA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A9",
    "0xd6 0xa9",
    "HEBREW ACCENT TELISHA QETANA",
    "֩",
    CodePointPosition::Above,
    TRAD_TELISHA_QETANA,
);

// ============================================================================
// YERAH BEN YOMO (U+05AA) - All identical
// ============================================================================
pub(crate) const TRAD_YERAH_BEN_YOMO: TraditionNames =
    TraditionNames::uniform("יֵרֶח בֶּן יוֹמ֪וֹ", "yerach ben yomo");

pub(crate) const CODEPOINT_YERAH_BEN_YOMO: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AA",
    "0xd6 0xaa",
    "HEBREW ACCENT YERAH BEN YOMO",
    "֪",
    CodePointPosition::Under,
    TRAD_YERAH_BEN_YOMO,
);

// ============================================================================
// OLE (U+05AB) - All identical
// ============================================================================
pub(crate) const TRAD_OLE: TraditionNames = TraditionNames::uniform("עוֹלֶה", "oleh");

pub(crate) const CODEPOINT_OLE: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AB",
    "0xd6 0xab",
    "HEBREW ACCENT OLE",
    "֫",
    CodePointPosition::Above,
    TRAD_OLE,
);

// ============================================================================
// ILUY (U+05AC) - All identical
// ============================================================================
pub(crate) const TRAD_ILUY: TraditionNames = TraditionNames::uniform("עִלוּי", "iluy");

pub(crate) const CODEPOINT_ILUY: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AC",
    "0xd6 0xac",
    "HEBREW ACCENT ILUY",
    "֬",
    CodePointPosition::Above,
    TRAD_ILUY,
);

// ============================================================================
// DEHI (U+05AD) - All identical
// ============================================================================
pub(crate) const TRAD_DEHI: TraditionNames = TraditionNames::uniform("דחי", "dechi");

pub(crate) const CODEPOINT_DEHI: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AD",
    "0xd6 0xad",
    "HEBREW ACCENT DECHI",
    "֭",
    CodePointPosition::Under,
    TRAD_DEHI,
);

// ============================================================================
// ZINOR (U+05AE) - All identical
// ============================================================================
pub(crate) const TRAD_ZINOR: TraditionNames =
    TraditionNames::uniform("צנור", "tsinor (zarqa above left)");

pub(crate) const CODEPOINT_ZINOR: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AE",
    "0xd6 0xae",
    "HEBREW ACCENT ZINOR",
    "֮",
    CodePointPosition::Above,
    TRAD_ZINOR,
);

// ============================================================================
// SILLUQ (U+05BD) - All identical (same codepoint as Meteg, different semantics)
// ============================================================================
pub(crate) const TRAD_SILLUQ: TraditionNames = TraditionNames::uniform("סוֹף פָּסֽוּק", "sof pasuq");

pub(crate) const CODEPOINT_SILLUQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT SILLUQ",
    "ֽ",
    CodePointPosition::Under,
    TRAD_SILLUQ,
);

// ============================================================================
// METEG (U+05BD) - All identical (shares codepoint with Silluq)
// ============================================================================
pub(crate) const TRAD_METEG: TraditionNames = TraditionNames::uniform("מֶתֶג", "meteg");

pub(crate) const CODEPOINT_METEG: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    TRAD_METEG,
);

// ============================================================================
// MAQAF (U+05BE) - No traditions
// ============================================================================
pub(crate) const TRAD_MAQAF: TraditionNames = TraditionNames {
    ashkenazi: None,
    sephardi: None,
    italian: None,
    yemenite: None,
};

pub(crate) const CODEPOINT_MAQAF: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BE",
    "0xd6 0xbe",
    "HEBREW PUNCTUATION MAQAF",
    "־",
    CodePointPosition::After,
    TRAD_MAQAF,
);

// ============================================================================
// PASEQ (U+05C0) - All identical
// ============================================================================
pub(crate) const TRAD_PASEQ: TraditionNames = TraditionNames::uniform("פָּסֵק", "paseq");

pub(crate) const CODEPOINT_PASEQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05C0",
    "0xd7 0x80",
    "HEBREW PUNCTUATION PASEQ",
    "׀",
    CodePointPosition::After,
    TRAD_PASEQ,
);

// ============================================================================
// SOPH PASUQ (U+05C3) - No traditions
// ============================================================================
pub(crate) const TRAD_SOPH_PASUQ: TraditionNames = TraditionNames {
    ashkenazi: None,
    sephardi: None,
    italian: None,
    yemenite: None,
};

pub(crate) const CODEPOINT_SOPH_PASUQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05C3",
    "0xd7 0x83",
    "HEBREW PUNCTUATION SOF PASUQ",
    "׃",
    CodePointPosition::InBetween,
    TRAD_SOPH_PASUQ,
);
