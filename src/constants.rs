use crate::AccentPosition;
use crate::Tradition;
use crate::Utf8CodePointInfo;

// used for method: HebrewContext::contains()
pub const ETNAHTA: char = '\u{0591}';
pub const SEGOL: char = '\u{0592}';
pub const SHALSHELET: char = '\u{0593}';
pub const ZAQEF_QATAN: char = '\u{0594}';
pub const ZAQEF_GADOL: char = '\u{0595}';
pub const TIPEHA: char = '\u{0596}';
pub const REVIA: char = '\u{0597}';
pub const ZARQA: char = '\u{0598}';
pub const PASHTA: char = '\u{0599}';
pub const YETIV: char = '\u{059A}';
pub const TEVIR: char = '\u{059B}';
pub const GERESH: char = '\u{059C}';
//pub const GERESH_MUQDAM:char = '\u{059D}';
pub const GERSHAYIM: char = '\u{059E}';
pub const QARNEY_PARA: char = '\u{059F}';
pub const TELISHA_GEDOLA: char = '\u{05A0}';
pub const PAZER: char = '\u{05A1}';
//pub const ATNAH_HAFUKH:char = '\u{05A2}';
pub const MUNAH: char = '\u{05A3}';
pub const MAHPAKH: char = '\u{05A4}';
pub const MERKHA: char = '\u{05A5}';
pub const MERKHA_KEFULA: char = '\u{05A6}';
pub const DARGA: char = '\u{05A7}';
pub const QADMA: char = '\u{05A8}';
pub const TELISHA_QETANA: char = '\u{05A9}';
pub const YERAH_BEN_YOMO: char = '\u{05AA}';
pub const OLEH: char = '\u{05AB}';
pub const ILUY: char = '\u{05AC}';
pub const DEHI: char = '\u{05AD}';
pub const ZINOR: char = '\u{05AE}';
pub const METEG: char = '\u{05BD}';
pub const MAQAF: char = '\u{05BE}';
//pub const RAFE:char = '\u{05BF}';

pub const PASEQ: char = '\u{05C0}';
pub const SOF_PASUQ: char = '\u{05C3}';
pub const VERTICAL_LINE: char = '\u{007C}';

// aliases
pub const TSINNORIT: char = ZARQA;
pub const YORED: char = MERKHA;
pub const AZLA: char = QADMA;
pub const SILLUQ: char = METEG;
pub const MEAYLA: char = TIPEHA;
pub const ATNACH: char = ETNAHTA;
pub const MAQQEPH: char = MAQAF;

// Constants below are a mix of the following:
// - UTF8 code table (https://utf8-chartable.de/unicode-utf8-table.pl)
// - naming of the accents according different traditions, see p.e.
//      https://en.wikipedia.org/wiki/Hebrew_cantillation
//      http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim
// - the position of the accent relative to the related consonant
//
// Purpose: give more detailed information on the accents
//
// (ordering of the list is based on the value of their Unicode code-point)
pub const CP_ETNAHTA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0591",
    hex_value: "0xd6 0x91",
    name: "HEBREW ACCENT ETNAHTA",
    symbol: "֑",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "אֶתְנַחְתָּ֑א",
        name: "etnachta",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "אֶתְנָחָ֑א",
        name: "Etnacha",
    }),
};

pub const CP_SEGOL: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0592",
    hex_value: "0xd6 0x92",
    name: "HEBREW ACCENT SEGOL",
    symbol: "֒",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "סְגוֹל֒",
        name: "segol",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "סְגוֹלְתָּא֒",
        name: "segolta",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שְׁרֵי֒",
        name: "shere",
    }),
    yemenite: None,
};

pub const CP_SHALSHELET: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0593",
    hex_value: "0xd6 0x93",
    name: "HEBREW ACCENT SHALSHELET",
    symbol: "֓",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "שִׁישְׁלָ֓א",
        name: "shishla",
    }),
};

pub const CP_ZAQEF_QATAN: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0594",
    hex_value: "0xd6 0x94",
    name: "HEBREW ACCENT ZAQEF QATAN",
    symbol: "֔",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "זָקֵף קָטָ֔ן",
        name: "zaqeph qatan",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
};

pub const CP_ZAQEF_GADOL: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0595",
    hex_value: "0xd6 0x95",
    name: "HEBREW ACCENT ZAQEF GADOL",
    symbol: "֕",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
};

pub const CP_TIPEHA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0596",
    hex_value: "0xd6 0x96",
    name: "HEBREW ACCENT TIPEHA",
    symbol: "֖",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "טִפְחָ֖א",
        name: "tiphcha",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "נְטוּיָ֖ה",
        name: "netuyah",
    }),
};

pub const CP_REVIA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0597",
    hex_value: "0xd6 0x97",
    name: "HEBREW ACCENT REVIA",
    symbol: "֗",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "רְבִ֗יע",
        name: "revia/revi’i",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
};

pub const CP_ZARQA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0598",
    hex_value: "0xd6 0x98",
    name: "HEBREW ACCENT ZARQA",
    symbol: "֘",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "צִנּוֹר֘",
        name: "tsinnor",
    }),
};

pub const CP_PASHTA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0599",
    hex_value: "0xd6 0x99",
    name: "HEBREW ACCENT PASHTA",
    symbol: "֙",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "קַדְמָא֙",
        name: "qadma",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "אַזְלָא֙",
        name: "azla",
    }),
};

pub const CP_YETIV: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059A",
    hex_value: "0xd6 0x9a",
    name: "HEBREW ACCENT YETIV",
    symbol: "֚",
    position: AccentPosition::UnderPrePositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שׁ֚וֹפָר יְתִיב",
        name: "shophar yetiv",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
};

pub const CP_TEVIR: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059B",
    hex_value: "0xd6 0x9b",
    name: "HEBREW ACCENT TEVIR",
    symbol: "֛",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "תַּבְרָ֛א",
        name: "tavra",
    }),
};
pub const CP_GERESH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059C",
    hex_value: "0xd6 0x9c",
    name: "HEBREW ACCENT GERESH",
    symbol: "֜",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "גְּרִ֜ישׁ",
        name: "gerish",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "טָרֵ֜ס",
        name: "tares",
    }),
};

// TODO CP_GERESH_MUQDAM
// pub const CP_GERESH_MUQDAM: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+059D",
//     hex_value: "0xd6 0x9d",
//     name: "HEBREW ACCENT GERESH MUQDAM",
//     symbol: "֝",
//     position: AccentPosition::Above,
//     ashkenazi: Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     sephardi: Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     italian: Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     yemenite: Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
// };

pub const CP_GERSHAYIM: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059E",
    hex_value: "0xd6 0x9e",
    name: "HEBREW ACCENT GERSHAYIM",
    symbol: "֞",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "גֵּרְשַׁ֞יִם",
        name: "gershayim",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "טַרְסִ֞ין",
        name: "tarsin",
    }),
};

pub const CP_QARNEY_PARA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059F",
    hex_value: "0xd6 0x9f",
    name: "HEBREW ACCENT QARNEY PARA",
    symbol: "֟",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah / pazer gadol",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
};

pub const CP_TELISHA_GEDOLA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A0",
    hex_value: "0xd6 0xa0",
    name: "HEBREW ACCENT TELISHA GEDOLA",
    symbol: "֠",
    position: AccentPosition::AbovePrePositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "תִּ֠רְצָה",
        name: "tirtsah",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "תַּ֠לְשָׁא",
        name: "talsha",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    }),
};

pub const CP_PAZER: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A1",
    hex_value: "0xd6 0xa1",
    name: "HEBREW ACCENT PAZER",
    symbol: "֡",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "פָּזֵ֡ר",
        name: "pazer",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    }),
    yemenite: None,
};

// TODO CP_ATNAH_HAFUKH
// pub const CP_ATNAH_HAFUKH: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+05A2",
//     hex_value: "0xd6 0xa2",
//     name: "HEBREW ACCENT ATNAH HAFUKH",
//     symbol: "֢",
//     position: AccentPosition::Above,
//     ashkenazi: Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     sephardi: Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     italian: Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     yemenite: Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
// };

pub const CP_MUNAH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A3",
    hex_value: "0xd6 0xa3",
    name: "HEBREW ACCENT MUNAH",
    symbol: "֣",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "מוּנַ֣ח",
        name: "munach",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "שׁוֹפָר הוֹלֵ֣ךְ",
        name: "shophar holech",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שׁוֹפָר עִלּ֣וּי",
        name: "shophar illuy",
    }),
    yemenite: None,
};

pub const CP_MAHAPAKH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A4",
    hex_value: "0xd6 0xa4",
    name: "HEBREW ACCENT MAHAPAKH",
    symbol: "֤",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "מַהְפַּ֤ך",
        name: "mahpach",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        name: "(shophar) mehuppach",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "שׁוֹפָר הָפ֤וּךְ",
        name: "shophar haphuch",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "מְהֻפָּ֤ךְ",
        name: "mehuppach",
    }),
};

pub const CP_MERKHA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A5",
    hex_value: "0xd6 0xa5",
    name: "HEBREW ACCENT MERKHA",
    symbol: "֥",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "מֵרְכָ֥א",
        name: "mercha",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "מַאֲרְכָ֥א",
        name: "maarcha",
    }),
};

pub const CP_MERKHA_KEFULA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A6",
    hex_value: "0xd6 0xa6",
    name: "HEBREW ACCENT MERKHA KEFULA",
    symbol: "֦",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "מֵרְכָא כּפוּלָ֦ה",
        name: "mercha kefulah",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "תְּרֵי טַעֲמֵ֦י",
        name: "tere taame",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "תְּרֵין חוּטְרִ֦ין",
        name: "teren chutrin",
    }),
    yemenite: None,
};

pub const CP_DARGA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A7",
    hex_value: "0xd6 0xa7",
    name: "HEBREW ACCENT DARGA",
    symbol: "֧",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
};

pub const CP_QADMA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A8",
    hex_value: "0xd6 0xa8",
    name: "HEBREW ACCENT QADMA",
    symbol: "֨",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "אַזְלָ֨א",
        name: "azla",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    }),
    yemenite: None,
};

pub const CP_TELISHA_QETANA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A9",
    hex_value: "0xd6 0xa9",
    name: "HEBREW ACCENT TELISHA QETANA",
    symbol: "֩",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "תַּלְשָׁא֩",
        name: "talsha",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "תַּרְסָא֩",
        name: "tarsa",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    }),
};

pub const CP_YERAH_BEN_YOMO: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AA",
    hex_value: "0xd6 0xaa",
    name: "HEBREW ACCENT YERAH BEN YOMO",
    symbol: "֪",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo/ galgal",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
};

pub const CP_OLE: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AB",
    hex_value: "0xd6 0xab",
    name: "HEBREW ACCENT OLE",
    symbol: "֫",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
};

pub const CP_ILUY: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AC",
    hex_value: "0xd6 0xac",
    name: "HEBREW ACCENT ILUY",
    symbol: "֬",
    position: AccentPosition::Above,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
};

pub const CP_DEHI: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AD",
    hex_value: "0xd6 0xad",
    name: "HEBREW ACCENT DEHI",
    symbol: "֭",
    position: AccentPosition::UnderPrePositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
};

pub const CP_ZINOR: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AE",
    hex_value: "0xd6 0xae",
    name: "HEBREW ACCENT ZINOR",
    symbol: "֮",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
};

pub const CP_SILLUQ: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05BD",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq/ silluq",
    }),
    sephardi: Some(Tradition::Sephardi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    }),
    italian: Some(Tradition::Italian {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    }),
    yemenite: Some(Tradition::Yemenite {
        hebrew: "סִלּֽוּק",
        name: "silluq",
    }),
};

pub const CP_METEG: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05BD",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Under,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
};

pub const CP_MAQAF: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05BE",
    hex_value: "0xd6 0xbe",
    name: "HEBREW PUNCTUATION MAQAF",
    symbol: "־",
    position: AccentPosition::End,
    ashkenazi: None,
    sephardi: None,
    italian: None,
    yemenite: None,
};

pub const CP_PASEQ: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05C0",
    hex_value: "0xd7 0x80",
    name: "HEBREW PUNCTUATION PASEQ",
    symbol: "׀",
    position: AccentPosition::End,
    ashkenazi: Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    sephardi: Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    italian: Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    yemenite: Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
};

// pub const CP_SOF_PASUQ: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+05C3",
//     hex_value: "0xd7 0x83",
//     name: "HEBREW PUNCTUATION SOF PASUQ",
//     symbol: "׃",
//     position: AccentPosition::Above,
//     ashkenazi: Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     sephardi: Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     italian: Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     yemenite: Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
// };
