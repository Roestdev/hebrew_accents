use crate::AccentPosition;
use crate::Tradition;
use crate::Utf8CodePoint;

pub const ETNAHTA: char = '\u{0591}';
pub const SEGOL: char = '\u{0592}';
//pub const SHALSHELET:char = '\u{0593}';
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
//pub const METEG:char = '\u{05BD}';
pub const MAQAF: char = '\u{05BE}';
//pub const RAFE:char = '\u{05BF}';
pub const PASEQ: char = '\u{05C0}';
//pub const SOF_PASUQ:char = '\u{05C3}';

pub const VERTICAL_LINE: char = '\u{007C}';
pub const YORED: char = '\u{05A5}';
pub const TSINNORIT: char = '\u{0598}';

//pub const SILLUQ: char = METEG;
//pub const MEAYLA: char = TIPEHA;

pub const CP_SILLUQ: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05BD ",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "Sof pasuq/ silluq ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "Sof pasuq ",
    },
    italian: Tradition::Italian {
        hebrew: "סוֹף פָּסֽוּק",
        name: "Sof pasuq ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "סִלּֽוּק",
        name: "Silluq",
    },
};

pub const CP_ETNAHTA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0591 ",
    hex_value: "0xd6 0x91",
    name: "HEBREW ACCENT ETNAHTA",
    symbol: "֑",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "אֶתְנַחְתָּ֑א",
        name: "Etnachta ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "אַתְנָ֑ח",
        name: "Atnach ",
    },
    italian: Tradition::Italian {
        hebrew: "אַתְנָ֑ח",
        name: "Atnach",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "אֶתְנָחָ֑א",
        name: "Etnacha",
    },
};

pub const CP_SEGOL: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0592 ",
    hex_value: "0xd6 0x92",
    name: "HEBREW ACCENT SEGOL",
    symbol: "֒",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "סְגוֹל֒",
        name: "Segol ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "סְגוֹלְתָּא֒",
        name: "Segolta ",
    },
    italian: Tradition::Italian {
        hebrew: "שְׁרֵי֒",
        name: "Shere ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_SHALSHELET: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0593 ",
    hex_value: "0xd6 0x93",
    name: "HEBREW ACCENT SHALSHELET",
    symbol: "֓",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "Shalshelet ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "Shalshelet ",
    },
    italian: Tradition::Italian {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "Shalshelet ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "שִׁישְׁלָ֓א",
        name: "Shishla",
    },
};

pub const CP_ZAQEF_QATAN: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0594 ",
    hex_value: "0xd6 0x94",
    name: "HEBREW ACCENT ZAQEF QATAN",
    symbol: "֔",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זָקֵף קָטָ֔ן",
        name: "Zaqef qatan ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "Zaqef qaton ",
    },
    italian: Tradition::Italian {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "Zaqef qaton ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "Zaqef qaton",
    },
};

pub const CP_ZAQEF_GADOL: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0595 ",
    hex_value: "0xd6 0x95",
    name: "HEBREW ACCENT ZAQEF GADOL",
    symbol: "֕",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "Zaqef gadol ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "Zaqef gadol ",
    },
    italian: Tradition::Italian {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "Zaqef gadol ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "Zaqef gadol",
    },
};

pub const CP_TIPEHA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0596 ",
    hex_value: "0xd6 0x96",
    name: "HEBREW ACCENT TIPEHA",
    symbol: "֖",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "טִפְחָ֖א",
        name: "Tiphcha ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "טַרְחָ֖א",
        name: "Tarcha ",
    },
    italian: Tradition::Italian {
        hebrew: "טַרְחָ֖א",
        name: "Tarcha ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "נְטוּיָ֖ה",
        name: "Netuyah",
    },
};

pub const CP_REVIA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0597 ",
    hex_value: "0xd6 0x97",
    name: "HEBREW ACCENT REVIA",
    symbol: "֗",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "רְבִ֗יע",
        name: "Revia/revi’i ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "רְבִ֗יע",
        name: "Revia ",
    },
    italian: Tradition::Italian {
        hebrew: "רְבִ֗יע",
        name: "Revia ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "רְבִ֗יע",
        name: "Revia",
    },
};

pub const CP_ZARQA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+0598 ",
    hex_value: "0xd6 0x98",
    name: "HEBREW ACCENT ZARQA",
    symbol: "֘",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "זַרְקָא֘",
        name: "Zarqa ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "זַרְקָא֘",
        name: "Zarqa ",
    },
    italian: Tradition::Italian {
        hebrew: "זַרְקָא֘",
        name: "Zarqa ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "צִנּוֹר֘",
        name: "Tzinnor",
    },
};

// pub const CP_PASHTA: Utf8CodePoint = Utf8CodePoint {
//     code_point: "U+0599 ",
//     hex_value: "0xd6 0x99",
//     name: "HEBREW ACCENT PASHTA",
//     symbol: "֙",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "פַּשְׁטָא֙",
//         name: "Pashta ",
//     },
//     sephardi: Tradition::Sephardi {
//         hebrew: "קַדְמָא֙",
//         name: "Qadma ",
//     },
//     italian: Tradition::Italian {
//         hebrew: "פַּשְׁטָא֙",
//         name: "Pashta ",
//     },
//     yemenite: Tradition::Yemenite {
//         hebrew: "אַזְלָא֙",
//         name: "Azla",
//     },
// };

// pub const CP_: Utf8CodePoint = Utf8CodePoint { code_point:"U+0599 U+05A8 ", hex_value: "", name: "", symbol: "", position: AccentPosition::Above, ashkenazi: Tradition::Ashkenazi { hebrew: "שְׁנֵ֨י פַּשְׁטִין֙", name: "Shene pashtin/pashtayim ", }, sephardi: Tradition::Sephardi { hebrew: "תּרֵ֨י קַדְמִין֙", name: "Tere qadmin ",  }, italian: Tradition::Italian { hebrew: "שְׁנֵ֨י פַּשְׁטִין֙", name: "(Shene) pashtin ",  }, yemenite: Tradition::Yemenite { hebrew: "",name: "",},};

pub const CP_YETIV: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+059A ",
    hex_value: "0xd6 0x9a",
    name: "HEBREW ACCENT YETIV",
    symbol: "֚",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "יְ֚תִיב",
        name: "Yetiv ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "יְ֚תִיב",
        name: "Yetiv ",
    },
    italian: Tradition::Italian {
        hebrew: "שׁ֚וֹפָר יְתִיב",
        name: "Shophar yetiv ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "יְ֚תִיב",
        name: "Yetiv",
    },
};

pub const CP_TEVIR: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+059B ",
    hex_value: "0xd6 0x9b",
    name: "HEBREW ACCENT TEVIR",
    symbol: "֛",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּבִ֛יר",
        name: "Tevir ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תְּבִ֛יר",
        name: "Tevir ",
    },
    italian: Tradition::Italian {
        hebrew: "תְּבִ֛יר",
        name: "Tevir ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תַּבְרָ֛א",
        name: "Tavra",
    },
};

pub const CP_PAZER: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A1 ",
    hex_value: "0xd6 0xa1",
    name: "HEBREW ACCENT PAZER",
    symbol: "֡",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "פָּזֵ֡ר",
        name: "Pazer ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "Pazer gadol ",
    },
    italian: Tradition::Italian {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "Pazer gadol ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_QARNEY_PARA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+059F ",
    hex_value: "0xd6 0x9f",
    name: "HEBREW ACCENT QARNEY PARA",
    symbol: "֟",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "Qarne pharah/ pazer gadol ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "Qarne pharah ",
    },
    italian: Tradition::Italian {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "Qarne pharah ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "Qarne pharah ",
    },
};

pub const CP_TELISHA_GEDOLA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A0 ",
    hex_value: "0xd6 0xa0",
    name: "HEBREW ACCENT TELISHA GEDOLA",
    symbol: "֠",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "Telisha gedolah ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תִּ֠רְצָה",
        name: "Tirtsah ",
    },
    italian: Tradition::Italian {
        hebrew: "תַּ֠לְשָׁא",
        name: "Talsha ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "Telisha gedolah",
    },
};

pub const CP_GERESH: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+059C ",
    hex_value: "0xd6 0x9c",
    name: "HEBREW ACCENT GERESH",
    symbol: "֜",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "גֵּ֜רֵשׁ",
        name: "Geresh/azla ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "גְּרִ֜ישׁ",
        name: "Gerish ",
    },
    italian: Tradition::Italian {
        hebrew: "גֵּ֜רֵשׁ",
        name: "Geresh/azla ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "טָרֵ֜ס",
        name: "Tares",
    },
};

pub const CP_GERSHAYIM: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+059E ",
    hex_value: "0xd6 0x9e",
    name: "HEBREW ACCENT GERSHAYIM",
    symbol: "֞",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "גֵּרְשַׁ֞יִם",
        name: "Gershayim ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "Shene gerishin ",
    },
    italian: Tradition::Italian {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "Shene gerishin ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "טַרְסִ֞ין",
        name: "Tarsin",
    },
};

//pub const CP_: Utf8CodePoint = Utf8CodePoint { code_point:"U+05A3", hex_value: "", name: "", symbol: "", position: AccentPosition::Above, ashkenazi: Tradition::Ashkenazi { hebrew: "מוּנַח לְגַרְמֵ֣הּ׀", name: "Munach legarmeh ", }, sephardi: Tradition::Sephardi { hebrew: "פָּסֵ֣ק׀", name: "Paseq ",  }, italian: Tradition::Italian { hebrew: "לְגַרְמֵ֣הּ׀", name: "Legarmeh ",  }, yemenite: Tradition::Yemenite { hebrew: "unknown",name: "unknown",},};

pub const CP_MERKHA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A5 ",
    hex_value: "0xd6 0xa5",
    name: "HEBREW ACCENT MERKHA",
    symbol: "֥",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מֵרְכָ֥א",
        name: "Mercha ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "מַאֲרִ֥יךְ",
        name: "Maarich ",
    },
    italian: Tradition::Italian {
        hebrew: "מַאֲרִ֥יךְ",
        name: "Maarich ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "מַאֲרְכָ֥א",
        name: "Maarcha",
    },
};

pub const CP_MUNAH: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A3 ",
    hex_value: "0xd6 0xa3",
    name: "HEBREW ACCENT MUNAH",
    symbol: "֣",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מוּנַ֣ח",
        name: "Munach ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שׁוֹפָר הוֹלֵ֣ךְ",
        name: "Shophar holech ",
    },
    italian: Tradition::Italian {
        hebrew: "שׁוֹפָר עִלּ֣וּי",
        name: "Shophar illui ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_MAHAPAKH: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A4 ",
    hex_value: "0xd6 0xa4",
    name: "HEBREW ACCENT MAHAPAKH",
    symbol: "֤",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מַהְפַּ֤ך",
        name: "Mahpach ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        name: "(Shophar) mehuppach ",
    },
    italian: Tradition::Italian {
        hebrew: "שׁוֹפָר הָפ֤וּךְ",
        name: "Shophar haphuch ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "מְהֻפָּ֤ךְ",
        name: "Mehuppach",
    },
};

pub const CP_DARGA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A7 ",
    hex_value: "0xd6 0xa7",
    name: "HEBREW ACCENT DARGA",
    symbol: "֧",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "דַּרְגָּ֧א",
        name: "Darga ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "דַּרְגָּ֧א",
        name: "Darga ",
    },
    italian: Tradition::Italian {
        hebrew: "דַּרְגָּ֧א",
        name: "Darga ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "דַּרְגָּ֧א",
        name: "Darga",
    },
};

pub const CP_QADMA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A8 ",
    hex_value: "0xd6 0xa8",
    name: "HEBREW ACCENT QADMA",
    symbol: "֨",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "קַדְמָ֨א",
        name: "Qadma ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "אַזְלָ֨א",
        name: "Azla ",
    },
    italian: Tradition::Italian {
        hebrew: "קַדְמָ֨א",
        name: "Qadma ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_TELISHA_QETANA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A9 ",
    hex_value: "0xd6 0xa9",
    name: "HEBREW ACCENT TELISHA QETANA",
    symbol: "֩",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "תְּלִישָא קְטַנָּה֩ ",
        name: "Telisha qetannah ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תַּלְשָׁא֩",
        name: "Talsha ",
    },
    italian: Tradition::Italian {
        hebrew: "תַּרְסָא֩",
        name: "Tarsa ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "Telisha qetannah",
    },
};

pub const CP_MERKHA_KEFULA: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05A6 ",
    hex_value: "0xd6 0xa6",
    name: "HEBREW ACCENT MERKHA KEFULA",
    symbol: "֦",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "מֵרְכָא כּפוּלָ֦ה",
        name: "Mercha kefulah ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "תְּרֵי טַעֲמֵ֦י",
        name: "Tere taame ",
    },
    italian: Tradition::Italian {
        hebrew: "תְּרֵין חוּטְרִ֦ין",
        name: "Teren chutrin ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "unknown",
        name: "unknown",
    },
};

pub const CP_YERAH_BEN_YOMO: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AA ",
    hex_value: "0xd6 0xaa",
    name: "HEBREW ACCENT YERAH BEN YOMO",
    symbol: "֪",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "Yerach ben yomo/ galgal ",
    },
    sephardi: Tradition::Sephardi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "Yerach ben yomo ",
    },
    italian: Tradition::Italian {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "Yerach ben yomo ",
    },
    yemenite: Tradition::Yemenite {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "Yerach ben yomo",
    },
};

// pub const CP_GERESH_MUQDAM: Utf8CodePoint = Utf8CodePoint {
//     code_point: "U+059D ",
//     hex_value: "0xd6 0x9d",
//     name: "HEBREW ACCENT GERESH MUQDAM",
//     symbol: "֝",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם ",
//         name: "geresh muqdam ",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם ",
//         name: "geresh muqdam ",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם ",
//         name: "geresh muqdam ",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם ",
//         name: "geresh muqdam ",
//     },
// };

// pub const CP_ATNAH_HAFUKH: Utf8CodePoint = Utf8CodePoint {
//     code_point: "U+05A2 ",
//     hex_value: "0xd6 0xa2",
//     name: "HEBREW ACCENT ATNAH HAFUKH",
//     symbol: "֢",
//     position: AccentPosition::Above,
//     ashkenazi: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך ",
//         name: "atnach haphukh ",
//     },
//     sephardi: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך ",
//         name: "atnach haphukh ",
//     },
//     italian: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך ",
//         name: "atnach haphukh ",
//     },
//     yemenite: Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך ",
//         name: "atnach haphukh ",
//     },
// };

pub const CP_OLEH: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AB ",
    hex_value: "0xd6 0xab",
    name: "HEBREW ACCENT OLEH",
    symbol: "֫",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה ",
        name: "oleh ",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה ",
        name: "oleh ",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה ",
        name: "oleh ",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "עוֹלֶה ",
        name: "oleh ",
    },
};

pub const CP_ILUY: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AC ",
    hex_value: "0xd6 0xac",
    name: "HEBREW ACCENT ILUY",
    symbol: "֬",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "עִלוּי ",
        name: "iluy ",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "עִלוּי ",
        name: "iluy ",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "עִלוּי ",
        name: "iluy ",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "עִלוּי ",
        name: "iluy ",
    },
};

pub const CP_DEHI: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AD ",
    hex_value: "0xd6 0xad",
    name: "HEBREW ACCENT DEHI",
    symbol: "֭",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "דחי ",
        name: "dechi ",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "דחי ",
        name: "dechi ",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "דחי ",
        name: "dechi ",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "דחי ",
        name: "dechi ",
    },
};

pub const CP_ZINOR: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AE ",
    hex_value: "0xd6 0xae",
    name: "HEBREW ACCENT ZINOR",
    symbol: "֮",
    position: AccentPosition::Above,
    ashkenazi: Tradition::Ashkenazi {
        hebrew: "צנור ",
        name: "tsinor (zarqa above left)",
    },
    sephardi: Tradition::Ashkenazi {
        hebrew: "צנור ",
        name: "tsinor (zarqa above left)",
    },
    italian: Tradition::Ashkenazi {
        hebrew: "צנור ",
        name: "tsinor (zarqa above left)",
    },
    yemenite: Tradition::Ashkenazi {
        hebrew: "צנור ",
        name: "tsinor (zarqa above left)",
    },
};

pub const CP_METEG: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05BD",
    hex_value: "0xd6 0xbd",
    name: "HEBREW POINT METEG",
    symbol: "ֽ",
    position: AccentPosition::Above,
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

// pub const CP_MAQAF: Utf8CodePoint = Utf8CodePoint {
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

pub const CP_PASEQ: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05C0",
    hex_value: "0xd7 0x80",
    name: "HEBREW PUNCTUATION PASEQ",
    symbol: "׀",
    position: AccentPosition::Above,
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

// pub const CP_SOF_PASUQ: Utf8CodePoint = Utf8CodePoint {
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
