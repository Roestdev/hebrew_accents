use crate::AccentPosition;
use crate::Tradition;
use crate::Utf8CodePointInfo;

// used for method: contains()
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
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "אֶתְנַחְתָּ֑א",
        name: "etnachta",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    },
    italian: Tradition::Italian {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "אֶתְנָחָ֑א",
        name: "Etnacha",
    },
};

pub const CP_SEGOL: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0592",
    hex_value: "0xd6 0x92",
    name: "HEBREW ACCENT SEGOL",
    symbol: "֒",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "סְגוֹל֒",
        name: "segol",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "סְגוֹלְתָּא֒",
        name: "segolta",
    },
    italian: Tradition::Italian {
        hebrew: "שְׁרֵי֒",
        name: "shere",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_SHALSHELET: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0593",
    hex_value: "0xd6 0x93",
    name: "HEBREW ACCENT SHALSHELET",
    symbol: "֓",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    },
    italian: Tradition::Italian {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "שִׁישְׁלָ֓א",
        name: "shishla",
    },
};

pub const CP_ZAQEF_QATAN: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0594",
    hex_value: "0xd6 0x94",
    name: "HEBREW ACCENT ZAQEF QATAN",
    symbol: "֔",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זָקֵף קָטָ֔ן",
        name: "zaqeph qatan",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    },
    italian: Tradition::Italian {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    },
};

pub const CP_ZAQEF_GADOL: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0595",
    hex_value: "0xd6 0x95",
    name: "HEBREW ACCENT ZAQEF GADOL",
    symbol: "֕",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    },
    italian: Tradition::Italian {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    },
};

pub const CP_TIPEHA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0596",
    hex_value: "0xd6 0x96",
    name: "HEBREW ACCENT TIPEHA",
    symbol: "֖",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "טִפְחָ֖א",
        name: "tiphcha",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    },
    italian: Tradition::Italian {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "נְטוּיָ֖ה",
        name: "netuyah",
    },
};

pub const CP_REVIA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0597",
    hex_value: "0xd6 0x97",
    name: "HEBREW ACCENT REVIA",
    symbol: "֗",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "רְבִ֗יע",
        name: "revia/revi’i",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "רְבִ֗יע",
        name: "revia",
    },
    italian: Tradition::Italian {
        hebrew: "רְבִ֗יע",
        name: "revia",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "רְבִ֗יע",
        name: "revia",
    },
};

pub const CP_ZARQA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0598",
    hex_value: "0xd6 0x98",
    name: "HEBREW ACCENT ZARQA",
    symbol: "֘",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    },
    italian: Tradition::Italian {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "צִנּוֹר֘",
        name: "tsinnor",
    },
};

pub const CP_PASHTA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+0599",
    hex_value: "0xd6 0x99",
    name: "HEBREW ACCENT PASHTA",
    symbol: "֙",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "קַדְמָא֙",
        name: "qadma",
    },
    italian: Tradition::Italian {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "אַזְלָא֙",
        name: "azla",
    },
};

pub const CP_YETIV: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059A",
    hex_value: "0xd6 0x9a",
    name: "HEBREW ACCENT YETIV",
    symbol: "֚",
    position: AccentPosition::UnderPrePositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    },
    italian: Tradition::Italian {
        hebrew: "שׁ֚וֹפָר יְתִיב",
        name: "shophar yetiv",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    },
};

pub const CP_TEVIR: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059B",
    hex_value: "0xd6 0x9b",
    name: "HEBREW ACCENT TEVIR",
    symbol: "֛",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    },
    italian: Tradition::Italian {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תַּבְרָ֛א",
        name: "tavra",
    },
};
pub const CP_GERESH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059C",
    hex_value: "0xd6 0x9c",
    name: "HEBREW ACCENT GERESH",
    symbol: "֜",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "גְּרִ֜ישׁ",
        name: "gerish",
    },
    italian: Tradition::Italian {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "טָרֵ֜ס",
        name: "tares",
    },
};

// TODO
// pub const CP_GERESH_MUQDAM: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+059D",
//     hex_value: "0xd6 0x9d",
//     name: "HEBREW ACCENT GERESH MUQDAM",
//     symbol: "֝",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     },
// };

pub const CP_GERSHAYIM: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059E",
    hex_value: "0xd6 0x9e",
    name: "HEBREW ACCENT GERSHAYIM",
    symbol: "֞",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "גֵּרְשַׁ֞יִם",
        name: "gershayim",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    },
    italian: Tradition::Italian {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "טַרְסִ֞ין",
        name: "tarsin",
    },
};

pub const CP_QARNEY_PARA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+059F",
    hex_value: "0xd6 0x9f",
    name: "HEBREW ACCENT QARNEY PARA",
    symbol: "֟",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah / pazer gadol",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    },
    italian: Tradition::Italian {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    },
};

pub const CP_TELISHA_GEDOLA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A0",
    hex_value: "0xd6 0xa0",
    name: "HEBREW ACCENT TELISHA GEDOLA",
    symbol: "֠",
    position: AccentPosition::AbovePrePositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תִּ֠רְצָה",
        name: "tirtsah",
    },
    italian: Tradition::Italian {
        hebrew: "תַּ֠לְשָׁא",
        name: "talsha",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    },
};

pub const CP_PAZER: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A1",
    hex_value: "0xd6 0xa1",
    name: "HEBREW ACCENT PAZER",
    symbol: "֡",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "פָּזֵ֡ר",
        name: "pazer",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    },
    italian: Tradition::Italian {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

// TODO
// pub const CP_ATNAH_HAFUKH: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+05A2",
//     hex_value: "0xd6 0xa2",
//     name: "HEBREW ACCENT ATNAH HAFUKH",
//     symbol: "֢",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     },
// };

// TODO
//pub const CP_: Utf8CodePointInfo = Utf8CodePointInfo { code_point:"U+05A3", hex_value: "", name: "", symbol: "", position: AccentPosition::Above, ashkenazi: Tradition::Ashkenazi { hebrew: "מוּנַח לְגַרְמֵ֣הּ׀", name: "Munach legarmeh", }, sephardi: Tradition::Sephardi { hebrew: "פָּסֵ֣ק׀", name: "Paseq",  }, italian: Tradition::Italian { hebrew: "לְגַרְמֵ֣הּ׀", name: "Legarmeh",  }, yemenite: Tradition::Yemenite { hebrew: "unknown",name: "unknown",},};

pub const CP_MUNAH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A3",
    hex_value: "0xd6 0xa3",
    name: "HEBREW ACCENT MUNAH",
    symbol: "֣",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מוּנַ֣ח",
        name: "munach",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שׁוֹפָר הוֹלֵ֣ךְ",
        name: "shophar holech",
    },
    italian: Tradition::Italian {
        hebrew: "שׁוֹפָר עִלּ֣וּי",
        name: "shophar illuy",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_MAHAPAKH: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A4",
    hex_value: "0xd6 0xa4",
    name: "HEBREW ACCENT MAHAPAKH",
    symbol: "֤",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מַהְפַּ֤ך",
        name: "mahpach",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        name: "(shophar) mehuppach",
    },
    italian: Tradition::Italian {
        hebrew: "שׁוֹפָר הָפ֤וּךְ",
        name: "shophar haphuch",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "מְהֻפָּ֤ךְ",
        name: "mehuppach",
    },
};

pub const CP_MERKHA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A5",
    hex_value: "0xd6 0xa5",
    name: "HEBREW ACCENT MERKHA",
    symbol: "֥",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מֵרְכָ֥א",
        name: "mercha",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    },
    italian: Tradition::Italian {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "מַאֲרְכָ֥א",
        name: "maarcha",
    },
};

pub const CP_MERKHA_KEFULA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A6",
    hex_value: "0xd6 0xa6",
    name: "HEBREW ACCENT MERKHA KEFULA",
    symbol: "֦",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מֵרְכָא כּפוּלָ֦ה",
        name: "mercha kefulah",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תְּרֵי טַעֲמֵ֦י",
        name: "tere taame",
    },
    italian: Tradition::Italian {
        hebrew: "תְּרֵין חוּטְרִ֦ין",
        name: "teren chutrin",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_DARGA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A7",
    hex_value: "0xd6 0xa7",
    name: "HEBREW ACCENT DARGA",
    symbol: "֧",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    },
    italian: Tradition::Italian {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    },
};

pub const CP_QADMA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A8",
    hex_value: "0xd6 0xa8",
    name: "HEBREW ACCENT QADMA",
    symbol: "֨",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "אַזְלָ֨א",
        name: "azla",
    },
    italian: Tradition::Italian {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    },
    yemenite: Tradition::Yemenite {
        // unknown, meaning could not find any info yet
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_TELISHA_QETANA: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05A9",
    hex_value: "0xd6 0xa9",
    name: "HEBREW ACCENT TELISHA QETANA",
    symbol: "֩",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תַּלְשָׁא֩",
        name: "talsha",
    },
    italian: Tradition::Italian {
        hebrew: "תַּרְסָא֩",
        name: "tarsa",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    },
};

pub const CP_YERAH_BEN_YOMO: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AA",
    hex_value: "0xd6 0xaa",
    name: "HEBREW ACCENT YERAH BEN YOMO",
    symbol: "֪",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo/ galgal",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    },
    italian: Tradition::Italian {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    },
};

pub const CP_OLE: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AB",
    hex_value: "0xd6 0xab",
    name: "HEBREW ACCENT OLE",
    symbol: "֫",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    },
};

pub const CP_ILUY: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AC",
    hex_value: "0xd6 0xac",
    name: "HEBREW ACCENT ILUY",
    symbol: "֬",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    },
};

pub const CP_DEHI: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AD",
    hex_value: "0xd6 0xad",
    name: "HEBREW ACCENT DEHI",
    symbol: "֭",
    position: AccentPosition::UnderPrePositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    },
};

pub const CP_ZINOR: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05AE",
    hex_value: "0xd6 0xae",
    name: "HEBREW ACCENT ZINOR",
    symbol: "֮",
    position: AccentPosition::AbovePostPositive,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    },
};

pub const CP_SILLUQ: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05BD",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq/ silluq",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    },
    italian: Tradition::Italian {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "סִלּֽוּק",
        name: "silluq",
    },
};

pub const CP_METEG: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05BD",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Under,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    },
};

// pub const CP_MAQAF: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+05BE",
//     hex_value: "0xd6 0xbe",
//     name: "HEBREW PUNCTUATION MAQAF",
//     symbol: "־",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "מַקָּף",
//         name: "maqqaph",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "מַקָּף",
//         name: "maqqaph",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "מַקָּף",
//         name: "maqqaph",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "מַקָּף",
//         name: "maqqaph",
//     },
// };

pub const CP_PASEQ: Utf8CodePointInfo = Utf8CodePointInfo {
    code_point: "U+05C0",
    hex_value: "0xd7 0x80",
    name: "HEBREW PUNCTUATION PASEQ",
    symbol: "׀",
    position: AccentPosition::End,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    },
};

// pub const CP_SOF_PASUQ: Utf8CodePointInfo = Utf8CodePointInfo {
//     code_point: "U+05C3",
//     hex_value: "0xd7 0x83",
//     name: "HEBREW PUNCTUATION SOF PASUQ",
//     symbol: "׃",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     },
// };
