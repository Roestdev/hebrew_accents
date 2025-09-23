use crate::Tradition;
use crate::CodePointPosition;
use crate::Utf8CodePointInfo;

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
//  Static lookup table (reference‑based, zero‑copy)
//  No macro – we construct the HashMap explicitly.

// ---------------------------------------------------------------------------
//  Constructor for a single code‑point description
const fn cp(
    code_point: &'static str,
    hex_value: &'static str,
    name: &'static str,
    symbol: &'static str,
    position: CodePointPosition,
    ashkenazi: Option<Tradition>,
    sephardi: Option<Tradition>,
    italian: Option<Tradition>,
    yemenite: Option<Tradition>,
) -> Utf8CodePointInfo {
    Utf8CodePointInfo {
        code_point,
        hex_value,
        name,
        symbol,
        position,
        ashkenazi,
        sephardi,
        italian,
        yemenite,
    }
}

pub const CP_ETNAHTA: Utf8CodePointInfo = cp(
    "U+0591",
    "0xd6 0x91",
    "HEBREW ACCENT ETNAHTA",
    "֑",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "אֶתְנַחְתָּ֑א",
        name: "etnachta",
    }),
    Some(Tradition::Sephardi {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    }),
    Some(Tradition::Italian {
        hebrew: "אַתְנָ֑ח",
        name: "atnach",
    }),
    Some(Tradition::Yemenite {
        hebrew: "אֶתְנָחָ֑א",
        name: "Etnacha",
    }),
);

pub const CP_SEGOL: Utf8CodePointInfo = cp(
    "U+0592",
    "0xd6 0x92",
    "HEBREW ACCENT SEGOL",
    "֒",
    CodePointPosition::AbovePostPositive,
    Some(Tradition::Ashkenazi {
        hebrew: "סְגוֹל֒",
        name: "segol",
    }),
    Some(Tradition::Sephardi {
        hebrew: "סְגוֹלְתָּא֒",
        name: "segolta",
    }),
    Some(Tradition::Italian {
        hebrew: "שְׁרֵי֒",
        name: "shere",
    }),
    None,
);

pub const CP_SHALSHELET: Utf8CodePointInfo = cp(
    "U+0593",
    "0xd6 0x93",
    "HEBREW ACCENT SHALSHELET",
    "֓",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    Some(Tradition::Sephardi {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    Some(Tradition::Italian {
        hebrew: "שַׁלְשֶׁ֓לֶת",
        name: "shalshelet",
    }),
    Some(Tradition::Yemenite {
        hebrew: "שִׁישְׁלָ֓א",
        name: "shishla",
    }),
);

pub const CP_ZAQEF_QATAN: Utf8CodePointInfo = cp(
    "U+0594",
    "0xd6 0x94",
    "HEBREW ACCENT ZAQEF QATAN",
    "֔",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "זָקֵף קָטָ֔ן",
        name: "zaqeph qatan",
    }),
    Some(Tradition::Sephardi {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
    Some(Tradition::Italian {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
    Some(Tradition::Yemenite {
        hebrew: "זָקֵף קָט֔וֹן",
        name: "zaqeph qaton",
    }),
);

pub const CP_ZAQEF_GADOL: Utf8CodePointInfo = cp(
    "U+0595",
    "0xd6 0x95",
    "HEBREW ACCENT ZAQEF GADOL",
    "֕",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    Some(Tradition::Sephardi {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    Some(Tradition::Italian {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
    Some(Tradition::Yemenite {
        hebrew: "זָקֵף גָּד֕וֹל",
        name: "zaqeph gadol",
    }),
);

pub const CP_TIPEHA: Utf8CodePointInfo = cp(
    "U+0596",
    "0xd6 0x96",
    "HEBREW ACCENT TIPEHA",
    "֖",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "טִפְחָ֖א",
        name: "tiphcha",
    }),
    Some(Tradition::Sephardi {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    }),
    Some(Tradition::Italian {
        hebrew: "טַרְחָ֖א",
        name: "tarcha",
    }),
    Some(Tradition::Yemenite {
        hebrew: "נְטוּיָ֖ה",
        name: "netuyah",
    }),
);

pub const CP_REVIA: Utf8CodePointInfo = cp(
    "U+0597",
    "0xd6 0x97",
    "HEBREW ACCENT REVIA",
    "֗",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "רְבִ֗יע",
        name: "revia/revi’i",
    }),
    Some(Tradition::Sephardi {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
    Some(Tradition::Italian {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
    Some(Tradition::Yemenite {
        hebrew: "רְבִ֗יע",
        name: "revia",
    }),
);

pub const CP_ZARQA: Utf8CodePointInfo = cp(
    "U+0598",
    "0xd6 0x98",
    "HEBREW ACCENT ZARQA",
    "֘",
    CodePointPosition::AbovePostPositive,
    Some(Tradition::Ashkenazi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    Some(Tradition::Sephardi {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    Some(Tradition::Italian {
        hebrew: "זַרְקָא֘",
        name: "zarqa",
    }),
    Some(Tradition::Yemenite {
        hebrew: "צִנּוֹר֘",
        name: "tsinnor",
    }),
);

pub const CP_PASHTA: Utf8CodePointInfo = cp(
    "U+0599",
    "0xd6 0x99",
    "HEBREW ACCENT PASHTA",
    "֙",
    CodePointPosition::AbovePostPositive,
    Some(Tradition::Ashkenazi {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    }),
    Some(Tradition::Sephardi {
        hebrew: "קַדְמָא֙",
        name: "qadma",
    }),
    Some(Tradition::Italian {
        hebrew: "פַּשְׁטָא֙",
        name: "pashta",
    }),
    Some(Tradition::Yemenite {
        hebrew: "אַזְלָא֙",
        name: "azla",
    }),
);

pub const CP_YETIV: Utf8CodePointInfo = cp(
    "U+059A",
    "0xd6 0x9a",
    "HEBREW ACCENT YETIV",
    "֚",
    CodePointPosition::UnderPrePositive,
    Some(Tradition::Ashkenazi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
    Some(Tradition::Sephardi {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
    Some(Tradition::Italian {
        hebrew: "שׁ֚וֹפָר יְתִיב",
        name: "shophar yetiv",
    }),
    Some(Tradition::Yemenite {
        hebrew: "יְ֚תִיב",
        name: "yetiv",
    }),
);

pub const CP_TEVIR: Utf8CodePointInfo = cp(
    "U+059B",
    "0xd6 0x9b",
    "HEBREW ACCENT TEVIR",
    "֛",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    Some(Tradition::Sephardi {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    Some(Tradition::Italian {
        hebrew: "תְּבִ֛יר",
        name: "tevir",
    }),
    Some(Tradition::Yemenite {
        hebrew: "תַּבְרָ֛א",
        name: "tavra",
    }),
);
pub const CP_GERESH: Utf8CodePointInfo = cp(
    "U+059C",
    "0xd6 0x9c",
    "HEBREW ACCENT GERESH",
    "֜",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    }),
    Some(Tradition::Sephardi {
        hebrew: "גְּרִ֜ישׁ",
        name: "gerish",
    }),
    Some(Tradition::Italian {
        hebrew: "גֵּ֜רֵשׁ",
        name: "geresh/azla",
    }),
    Some(Tradition::Yemenite {
        hebrew: "טָרֵ֜ס",
        name: "tares",
    }),
);

// TODO CP_GERESH_MUQDAM
// pub const CP_GERESH_MUQDAM: Utf8CodePointInfo = cp(
//     "U+059D",
//     "0xd6 0x9d",
//     "HEBREW ACCENT GERESH MUQDAM",
//     "֝",
//     CodePointPosition::Above,
//     Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "גֵרֵשׁ מוּקְדָם",
//         name: "geresh muqdam",
//     }),
// );

pub const CP_GERSHAYIM: Utf8CodePointInfo = cp(
    "U+059E",
    "0xd6 0x9e",
    "HEBREW ACCENT GERSHAYIM",
    "֞",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "גֵּרְשַׁ֞יִם",
        name: "gershayim",
    }),
    Some(Tradition::Sephardi {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    }),
    Some(Tradition::Italian {
        hebrew: "שְׁנֵי גְרִישִׁ֞ין",
        name: "shene gerishin",
    }),
    Some(Tradition::Yemenite {
        hebrew: "טַרְסִ֞ין",
        name: "tarsin",
    }),
);

pub const CP_QARNEY_PARA: Utf8CodePointInfo = cp(
    "U+059F",
    "0xd6 0x9f",
    "HEBREW ACCENT QARNEY PARA",
    "֟",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah / pazer gadol",
    }),
    Some(Tradition::Sephardi {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
    Some(Tradition::Italian {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
    Some(Tradition::Yemenite {
        hebrew: "קַרְנֵי פָרָ֟ה",
        name: "qarne pharah",
    }),
);

pub const CP_TELISHA_GEDOLA: Utf8CodePointInfo = cp(
    "U+05A0",
    "0xd6 0xa0",
    "HEBREW ACCENT TELISHA GEDOLA",
    "֠",
    CodePointPosition::AbovePrePositive,
    Some(Tradition::Ashkenazi {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    }),
    Some(Tradition::Sephardi {
        hebrew: "תִּ֠רְצָה",
        name: "tirtsah",
    }),
    Some(Tradition::Italian {
        hebrew: "תַּ֠לְשָׁא",
        name: "talsha",
    }),
    Some(Tradition::Yemenite {
        hebrew: "תְּ֠לִישָא גְדוֹלָה",
        name: "telisha gedolah",
    }),
);

pub const CP_PAZER: Utf8CodePointInfo = cp(
    "U+05A1",
    "0xd6 0xa1",
    "HEBREW ACCENT PAZER",
    "֡",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "פָּזֵ֡ר",
        name: "pazer",
    }),
    Some(Tradition::Sephardi {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    }),
    Some(Tradition::Italian {
        hebrew: "פָּזֵר גָּד֡וֹל",
        name: "pazer gadol",
    }),
    None,
);

// TODO CP_ATNAH_HAFUKH
// pub const CP_ATNAH_HAFUKH: Utf8CodePointInfo = cp(
//     "U+05A2",
//     "0xd6 0xa2",
//     "HEBREW ACCENT ATNAH HAFUKH",
//     "֢",
//     CodePointPosition::Above,
//     Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "אתנח הפוך",
//         name: "atnach haphukh",
//     }),
// );

pub const CP_MUNAH: Utf8CodePointInfo = cp(
    "U+05A3",
    "0xd6 0xa3",
    "HEBREW ACCENT MUNAH",
    "֣",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "מוּנַ֣ח",
        name: "munach",
    }),
    Some(Tradition::Sephardi {
        hebrew: "שׁוֹפָר הוֹלֵ֣ךְ",
        name: "shophar holech",
    }),
    Some(Tradition::Italian {
        hebrew: "שׁוֹפָר עִלּ֣וּי",
        name: "shophar illuy",
    }),
    None,
);

pub const CP_MAHAPAKH: Utf8CodePointInfo = cp(
    "U+05A4",
    "0xd6 0xa4",
    "HEBREW ACCENT MAHAPAKH",
    "֤",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "מַהְפַּ֤ך",
        name: "mahpach",
    }),
    Some(Tradition::Sephardi {
        hebrew: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        name: "(shophar) mehuppach",
    }),
    Some(Tradition::Italian {
        hebrew: "שׁוֹפָר הָפ֤וּךְ",
        name: "shophar haphuch",
    }),
    Some(Tradition::Yemenite {
        hebrew: "מְהֻפָּ֤ךְ",
        name: "mehuppach",
    }),
);

pub const CP_MERKHA: Utf8CodePointInfo = cp(
    "U+05A5",
    "0xd6 0xa5",
    "HEBREW ACCENT MERKHA",
    "֥",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "מֵרְכָ֥א",
        name: "mercha",
    }),
    Some(Tradition::Sephardi {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    }),
    Some(Tradition::Italian {
        hebrew: "מַאֲרִ֥יךְ",
        name: "maarich",
    }),
    Some(Tradition::Yemenite {
        hebrew: "מַאֲרְכָ֥א",
        name: "maarcha",
    }),
);

pub const CP_MERKHA_KEFULA: Utf8CodePointInfo = cp(
    "U+05A6",
    "0xd6 0xa6",
    "HEBREW ACCENT MERKHA KEFULA",
    "֦",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "מֵרְכָא כּפוּלָ֦ה",
        name: "mercha kefulah",
    }),
    Some(Tradition::Sephardi {
        hebrew: "תְּרֵי טַעֲמֵ֦י",
        name: "tere taame",
    }),
    Some(Tradition::Italian {
        hebrew: "תְּרֵין חוּטְרִ֦ין",
        name: "teren chutrin",
    }),
    None,
);

pub const CP_DARGA: Utf8CodePointInfo = cp(
    "U+05A7",
    "0xd6 0xa7",
    "HEBREW ACCENT DARGA",
    "֧",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    Some(Tradition::Sephardi {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    Some(Tradition::Italian {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
    Some(Tradition::Yemenite {
        hebrew: "דַּרְגָּ֧א",
        name: "darga",
    }),
);

pub const CP_QADMA: Utf8CodePointInfo = cp(
    "U+05A8",
    "0xd6 0xa8",
    "HEBREW ACCENT QADMA",
    "֨",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    }),
    Some(Tradition::Sephardi {
        hebrew: "אַזְלָ֨א",
        name: "azla",
    }),
    Some(Tradition::Italian {
        hebrew: "קַדְמָ֨א",
        name: "qadma",
    }),
    None,
);

pub const CP_TELISHA_QETANA: Utf8CodePointInfo = cp(
    "U+05A9",
    "0xd6 0xa9",
    "HEBREW ACCENT TELISHA QETANA",
    "֩",
    CodePointPosition::AbovePostPositive,
    Some(Tradition::Ashkenazi {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    }),
    Some(Tradition::Sephardi {
        hebrew: "תַּלְשָׁא֩",
        name: "talsha",
    }),
    Some(Tradition::Italian {
        hebrew: "תַּרְסָא֩",
        name: "tarsa",
    }),
    Some(Tradition::Yemenite {
        hebrew: "תְּלִישָא קְטַנָּה֩",
        name: "telisha qetannah",
    }),
);

pub const CP_YERAH_BEN_YOMO: Utf8CodePointInfo = cp(
    "U+05AA",
    "0xd6 0xaa",
    "HEBREW ACCENT YERAH BEN YOMO",
    "֪",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo/ galgal",
    }),
    Some(Tradition::Sephardi {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
    Some(Tradition::Italian {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
    Some(Tradition::Yemenite {
        hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
        name: "yerach ben yomo",
    }),
);

pub const CP_OLE: Utf8CodePointInfo = cp(
    "U+05AB",
    "0xd6 0xab",
    "HEBREW ACCENT OLE",
    "֫",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עוֹלֶה",
        name: "oleh",
    }),
);

pub const CP_ILUY: Utf8CodePointInfo = cp(
    "U+05AC",
    "0xd6 0xac",
    "HEBREW ACCENT ILUY",
    "֬",
    CodePointPosition::Above,
    Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "עִלוּי",
        name: "iluy",
    }),
);

pub const CP_DEHI: Utf8CodePointInfo = cp(
    "U+05AD",
    "0xd6 0xad",
    "HEBREW ACCENT DEHI",
    "֭",
    CodePointPosition::UnderPrePositive,
    Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "דחי",
        name: "dechi",
    }),
);

pub const CP_ZINOR: Utf8CodePointInfo = cp(
    "U+05AE",
    "0xd6 0xae",
    "HEBREW ACCENT ZINOR",
    "֮",
    CodePointPosition::AbovePostPositive,
    Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "צנור",
        name: "tsinor (zarqa above left)",
    }),
);

pub const CP_SILLUQ: Utf8CodePointInfo = cp(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq/ silluq",
    }),
    Some(Tradition::Sephardi {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    }),
    Some(Tradition::Italian {
        hebrew: "סוֹף פָּסֽוּק",
        name: "sof pasuq",
    }),
    Some(Tradition::Yemenite {
        hebrew: "סִלּֽוּק",
        name: "silluq",
    }),
);

pub const CP_METEG: Utf8CodePointInfo = cp(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "מֶתֶג",
        name: "meteg",
    }),
);

pub const CP_MAQAF: Utf8CodePointInfo = cp(
    "U+05BE",
    "0xd6 0xbe",
    "HEBREW PUNCTUATION MAQAF",
    "־",
    CodePointPosition::End,
    None,
    None,
    None,
    None,
);

pub const CP_PASEQ: Utf8CodePointInfo = cp(
    "U+05C0",
    "0xd7 0x80",
    "HEBREW PUNCTUATION PASEQ",
    "׀",
    CodePointPosition::End,
    Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
    Some(Tradition::Ashkenazi {
        hebrew: "פָּסֵק",
        name: "paseq",
    }),
);

// pub const CP_SOF_PASUQ: Utf8CodePointInfo = cp(
//     "U+05C3",
//     "0xd7 0x83",
//     "HEBREW PUNCTUATION SOF PASUQ",
//     "׃",
//     position: CodePointPosition::Above,
//     Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
//     Some(Tradition::Ashkenazi {
//         hebrew: "סוֹף פָּסֽוּק",
//         name: "soph pasuq",
//     }),
// );
