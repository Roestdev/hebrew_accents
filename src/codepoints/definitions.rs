//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://utf8-chartable.de/unicode-utf8-table.pl>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

// Crate‑internal (local modules)
use crate::accent::Utf8CodePointInfo;
use crate::codepoints::CodePointPosition;
use crate::codepoints::Tradition;

/// Constructor for a single code‑point description
const fn utf8_cp_constructor(
    code_point_value: &'static str,
    hex_value: &'static str,
    name: &'static str,
    symbol: &'static str,
    position: CodePointPosition,
    traditions: &'static [Tradition],
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

pub(crate) const TRAD_ETNAHTA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "אֶתְנַחְתָּ֑א",
    english_name: "Etnachta",
};
pub(crate) const TRAD_ETNAHTA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "אַתְנָ֑ח",
    english_name: "Atnach",
};
pub(crate) const TRAD_ETNAHTA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "אַתְנָ֑ח",
    english_name: "Atnach",
};
pub(crate) const TRAD_ETNAHTA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "אֶתְנָחָ֑א",
    english_name: "Etnacha",
};
pub(crate) const CODEPOINT_ETNAHTA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0591",
    "0xd6 0x91",
    "HEBREW ACCENT ETNAHTA",
    "֑",
    CodePointPosition::Under,
    &[
        TRAD_ETNAHTA_ASHKENAZI,
        TRAD_ETNAHTA_SEPHARDI,
        TRAD_ETNAHTA_ITALIAN,
        TRAD_ETNAHTA_YEMENITE,
    ],
);

pub(crate) const TRAD_SEGOL_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "סְגוֹל֒",
    english_name: "segol",
};
pub(crate) const TRAD_SEGOL_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "סְגוֹלְתָּא֒",
    english_name: "segolta",
};
pub(crate) const TRAD_SEGOL_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שְׁרֵי֒",
    english_name: "shere",
};
pub(crate) const CODEPOINT_SEGOL: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0592",
    "0xd6 0x92",
    "HEBREW ACCENT SEGOL",
    "֒",
    CodePointPosition::Above,
    &[
        TRAD_SEGOL_ASHKENAZI,
        TRAD_SEGOL_SEPHARDI,
        TRAD_SEGOL_ITALIAN,
    ],
);

pub(crate) const TRAD_SHALSHELET_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "שַׁלְשֶׁ֓לֶת",
    english_name: "shalshelet",
};
pub(crate) const TRAD_SHALSHELET_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "שַׁלְשֶׁ֓לֶת",
    english_name: "shalshelet",
};
pub(crate) const TRAD_SHALSHELET_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שַׁלְשֶׁ֓לֶת",
    english_name: "shalshelet",
};
pub(crate) const TRAD_SHALSHELET_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "שִׁישְׁלָ֓א",
    english_name: "shishla",
};

pub(crate) const CODEPOINT_SHALSHELET: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0593",
    "0xd6 0x93",
    "HEBREW ACCENT SHALSHELET",
    "֓",
    CodePointPosition::Above,
    &[
        TRAD_SHALSHELET_ASHKENAZI,
        TRAD_SHALSHELET_SEPHARDI,
        TRAD_SHALSHELET_ITALIAN,
        TRAD_SHALSHELET_YEMENITE,
    ],
);

pub(crate) const TRAD_ZAQEF_QATAN_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "זָקֵף קָטָ֔ן",
    english_name: "zaqeph qatan",
};
pub(crate) const TRAD_ZAQEF_QATAN_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "זָקֵף קָט֔וֹן",
    english_name: "zaqeph qaton",
};

pub(crate) const TRAD_ZAQEF_QATAN_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "זָקֵף קָט֔וֹן",
    english_name: "zaqeph qaton",
};
pub(crate) const TRAD_ZAQEF_QATAN_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "זָקֵף קָט֔וֹן",
    english_name: "zaqeph qaton",
};

pub(crate) const CODEPOINT_ZAQEF_QATAN: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0594",
    "0xd6 0x94",
    "HEBREW ACCENT ZAQEF QATAN",
    "֔",
    CodePointPosition::Above,
    &[
        TRAD_ZAQEF_QATAN_ASHKENAZI,
        TRAD_ZAQEF_QATAN_SEPHARDI,
        TRAD_ZAQEF_QATAN_ITALIAN,
        TRAD_ZAQEF_QATAN_YEMENITE,
    ],
);
pub(crate) const TRAD_ZAQEF_GADOL_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "זָקֵף גָּד֕וֹל",
    english_name: "zaqeph gadol",
};
pub(crate) const TRAD_ZAQEF_GADOL_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "זָקֵף גָּד֕וֹל",
    english_name: "zaqeph gadol",
};
pub(crate) const TRAD_ZAQEF_GADOL_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "זָקֵף גָּד֕וֹל",
    english_name: "zaqeph gadol",
};
pub(crate) const TRAD_ZAQEF_GADOL_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "זָקֵף גָּד֕וֹל",
    english_name: "zaqeph gadol",
};
pub(crate) const CODEPOINT_ZAQEF_GADOL: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0595",
    "0xd6 0x95",
    "HEBREW ACCENT ZAQEF GADOL",
    "֕",
    CodePointPosition::Above,
    &[
        TRAD_ZAQEF_GADOL_ASHKENAZI,
        TRAD_ZAQEF_GADOL_SEPHARDI,
        TRAD_ZAQEF_GADOL_ITALIAN,
        TRAD_ZAQEF_GADOL_YEMENITE,
    ],
);

pub(crate) const TRAD_TIPEHA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "טִפְחָ֖א",
    english_name: "tiphcha",
};
pub(crate) const TRAD_TIPEHA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "טַרְחָ֖א",
    english_name: "tarcha",
};
pub(crate) const TRAD_TIPEHA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "טַרְחָ֖א",
    english_name: "tarcha",
};
pub(crate) const TRAD_TIPEHA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "נְטוּיָ֖ה",
    english_name: "netuyah",
};
pub(crate) const CODEPOINT_TIPEHA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0596",
    "0xd6 0x96",
    "HEBREW ACCENT TIPEHA",
    "֖",
    CodePointPosition::Under,
    &[
        TRAD_TIPEHA_ASHKENAZI,
        TRAD_TIPEHA_SEPHARDI,
        TRAD_TIPEHA_ITALIAN,
        TRAD_TIPEHA_YEMENITE,
    ],
);

pub(crate) const TRAD_REVIA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "רְבִ֗יע",
    english_name: "revia/revi'i",
};
pub(crate) const TRAD_REVIA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "רְבִ֗יע",
    english_name: "revia",
};
pub(crate) const TRAD_REVIA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "רְבִ֗יע",
    english_name: "revia",
};
pub(crate) const TRAD_REVIA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "רְבִ֗יע",
    english_name: "revia",
};
pub(crate) const CODEPOINT_REVIA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0597",
    "0xd6 0x97",
    "HEBREW ACCENT REVIA",
    "֗",
    CodePointPosition::Above,
    &[
        TRAD_REVIA_ASHKENAZI,
        TRAD_REVIA_SEPHARDI,
        TRAD_REVIA_ITALIAN,
        TRAD_REVIA_YEMENITE,
    ],
);

pub(crate) const TRAD_ZARQA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "זַרְקָא֘",
    english_name: "zarqa",
};
pub(crate) const TRAD_ZARQA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "זַרְקָא֘",
    english_name: "zarqa",
};
pub(crate) const TRAD_ZARQA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "זַרְקָא֘",
    english_name: "zarqa",
};
pub(crate) const TRAD_ZARQA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "צִנּוֹר֘",
    english_name: "tsinnor",
};
pub(crate) const CODEPOINT_ZARQA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0598",
    "0xd6 0x98",
    "HEBREW ACCENT ZARQA",
    "֘",
    CodePointPosition::Above,
    &[
        TRAD_ZARQA_ASHKENAZI,
        TRAD_ZARQA_SEPHARDI,
        TRAD_ZARQA_ITALIAN,
        TRAD_ZARQA_YEMENITE,
    ],
);

pub(crate) const TRAD_PASHTA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "פַּשְׁטָא֙",
    english_name: "pashta",
};
pub(crate) const TRAD_PASHTA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "קַדְמָא֙",
    english_name: "qadma",
};
pub(crate) const TRAD_PASHTA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "פַּשְׁטָא֙",
    english_name: "pashta",
};
pub(crate) const TRAD_PASHTA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "אַזְלָא֙",
    english_name: "azla",
};
pub(crate) const CODEPOINT_PASHTA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+0599",
    "0xd6 0x99",
    "HEBREW ACCENT PASHTA",
    "֙",
    CodePointPosition::Above,
    &[
        TRAD_PASHTA_ASHKENAZI,
        TRAD_PASHTA_SEPHARDI,
        TRAD_PASHTA_ITALIAN,
        TRAD_PASHTA_YEMENITE,
    ],
);

pub(crate) const TRAD_YETIV_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "יְ֚תִיב",
    english_name: "yetiv",
};
pub(crate) const TRAD_YETIV_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "יְ֚תִיב",
    english_name: "yetiv",
};
pub(crate) const TRAD_YETIV_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שׁ֚וֹפָר יְתִיב",
    english_name: "shophar yetiv",
};
pub(crate) const TRAD_YETIV_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "יְ֚תִיב",
    english_name: "yetiv",
};
pub(crate) const CODEPOINT_YETIV: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059A",
    "0xd6 0x9a",
    "HEBREW ACCENT YETIV",
    "֚",
    CodePointPosition::Under,
    &[
        TRAD_YETIV_ASHKENAZI,
        TRAD_YETIV_SEPHARDI,
        TRAD_YETIV_ITALIAN,
        TRAD_YETIV_YEMENITE,
    ],
);

pub(crate) const TRAD_TEVIR_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "תְּבִ֛יר",
    english_name: "tevir",
};
pub(crate) const TRAD_TEVIR_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "תְּבִ֛יר",
    english_name: "tevir",
};
pub(crate) const TRAD_TEVIR_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "תְּבִ֛יר",
    english_name: "tevir",
};
pub(crate) const TRAD_TEVIR_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "תַּבְרָ֛א",
    english_name: "tavra",
};
pub(crate) const CODEPOINT_TEVIR: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059B",
    "0xd6 0x9b",
    "HEBREW ACCENT TEVIR",
    "֛",
    CodePointPosition::Under,
    &[
        TRAD_TEVIR_ASHKENAZI,
        TRAD_TEVIR_SEPHARDI,
        TRAD_TEVIR_ITALIAN,
        TRAD_TEVIR_YEMENITE,
    ],
);

pub(crate) const TRAD_GERESH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "גֵּ֜רֵשׁ",
    english_name: "geresh/azla",
};
pub(crate) const TRAD_GERESH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "גְּרִ֜ישׁ",
    english_name: "gerish",
};
pub(crate) const TRAD_GERESH_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "גֵּ֜רֵשׁ",
    english_name: "geresh/azla",
};
pub(crate) const TRAD_GERESH_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "טָרֵ֜ס",
    english_name: "tares",
};
pub(crate) const CODEPOINT_GERESH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059C",
    "0xd6 0x9c",
    "HEBREW ACCENT GERESH",
    "֜",
    CodePointPosition::Above,
    &[
        TRAD_GERESH_ASHKENAZI,
        TRAD_GERESH_SEPHARDI,
        TRAD_GERESH_ITALIAN,
        TRAD_GERESH_YEMENITE,
    ],
);

// TICKET #1 CODEPOINT_GERESH_MUQDAM
// see https://github.com/Roestdev/hebrew_accents/issues/1
//
// pub(crate) const CODEPOINT_GERESH_MUQDAM: Utf8CodePointInfo = utf8_cp_constructor(
//     "U+059D",
//     "0xd6 0x9d",
//     "HEBREW ACCENT GERESH MUQDAM",
//     "֝",
//     CodePointPosition::Above,
//     &[
//         // Note: Need to populate tradition variants here when uncommenting
//     ],
// );

pub(crate) const TRAD_GERSHAYIM_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "גֵּרְשַׁ֞יִם",
    english_name: "gershayim",
};
pub(crate) const TRAD_GERSHAYIM_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "שְׁנֵי גְרִישִׁ֞ין",
    english_name: "shene gerishin",
};
pub(crate) const TRAD_GERSHAYIM_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שְׁנֵי גְרִישִׁ֞ין",
    english_name: "shene gerishin",
};
pub(crate) const TRAD_GERSHAYIM_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "שְׁנֵי גְרִישִׁ֞ין",
    english_name: "shene gerishin",
};
pub(crate) const CODEPOINT_GERSHAYIM: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059E",
    "0xd6 0x9e",
    "HEBREW ACCENT GERSHAYIM",
    "֞",
    CodePointPosition::Above,
    &[
        TRAD_GERSHAYIM_ASHKENAZI,
        TRAD_GERSHAYIM_SEPHARDI,
        TRAD_GERSHAYIM_ITALIAN,
        TRAD_GERSHAYIM_YEMENITE,
    ],
);

pub(crate) const TRAD_QARNEY_PARA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "קַרְנֵי פָרָ֟ה",
    english_name: "qarne pharah / pazer gadol",
};
pub(crate) const TRAD_QARNEY_PARA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "קַרְנֵי פָרָ֟ה",
    english_name: "qarne pharah",
};
pub(crate) const TRAD_QARNEY_PARA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "קַרְנֵי פָרָ֟ה",
    english_name: "qarne pharah",
};
pub(crate) const TRAD_QARNEY_PARA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "קַרְנֵי פָרָ֟ה",
    english_name: "qarne pharah",
};
pub(crate) const CODEPOINT_QARNEY_PARA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+059F",
    "0xd6 0x9f",
    "HEBREW ACCENT QARNEY PARA",
    "֟",
    CodePointPosition::Above,
    &[
        TRAD_QARNEY_PARA_ASHKENAZI,
        TRAD_QARNEY_PARA_SEPHARDI,
        TRAD_QARNEY_PARA_ITALIAN,
        TRAD_QARNEY_PARA_YEMENITE,
    ],
);

pub(crate) const TRAD_TELISHA_GEDOLA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
    english_name: "telisha gedolah",
};
pub(crate) const TRAD_TELISHA_GEDOLA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "תִּ֠רְצָה",
    english_name: "tirtsah",
};
pub(crate) const TRAD_TELISHA_GEDOLA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "תַּ֠לְשָׁא",
    english_name: "talsha",
};
pub(crate) const TRAD_TELISHA_GEDOLA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
    english_name: "telisha gedolah",
};
pub(crate) const CODEPOINT_TELISHA_GEDOLA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A0",
    "0xd6 0xa0",
    "HEBREW ACCENT TELISHA GEDOLA",
    "֠",
    CodePointPosition::Above,
    &[
        TRAD_TELISHA_GEDOLA_ASHKENAZI,
        TRAD_TELISHA_GEDOLA_SEPHARDI,
        TRAD_TELISHA_GEDOLA_ITALIAN,
        TRAD_TELISHA_GEDOLA_YEMENITE,
    ],
);

pub(crate) const TRAD_PAZER_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "פָּזֵ֡ר",
    english_name: "pazer",
};
pub(crate) const TRAD_PAZER_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "פָּזֵר גָּד֡וֹל",
    english_name: "pazer gadol",
};
pub(crate) const TRAD_PAZER_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "פָּזֵר גָּד֡וֹל",
    english_name: "pazer gadol",
};

pub(crate) const CODEPOINT_PAZER: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A1",
    "0xd6 0xa1",
    "HEBREW ACCENT PAZER",
    "֡",
    CodePointPosition::Above,
    &[
        TRAD_PAZER_ASHKENAZI,
        TRAD_PAZER_SEPHARDI,
        TRAD_PAZER_ITALIAN,
    ],
);

// TICKET #3 CODEPOINT_ATNAH_HAFUKH
// (see: https://github.com/Roestdev/hebrew_accents/issues/3)
//
// pub(crate) const CODEPOINT_ATNAH_HAFUKH: Utf8CodePointInfo = utf8_cp_constructor(
//     "U+05A2",
//     "0xd6 0xa2",
//     "HEBREW ACCENT ATNAH HAFUKH",
//     "֢",
//     CodePointPosition::Above,
//     &[],
// );

pub(crate) const TRAD_MUNAH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "מוּנַ֣ח",
    english_name: "munach",
};
pub(crate) const TRAD_MUNAH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "שׁוֹפָר הוֹלֵ֣ךְ",
    english_name: "shophar holech",
};
pub(crate) const TRAD_MUNAH_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שׁוֹפָר עִלּ֣וּי",
    english_name: "shophar illuy",
};

pub(crate) const CODEPOINT_MUNAH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A3",
    "0xd6 0xa3",
    "HEBREW ACCENT MUNAH",
    "֣",
    CodePointPosition::Under,
    &[
        TRAD_MUNAH_ASHKENAZI,
        TRAD_MUNAH_SEPHARDI,
        TRAD_MUNAH_ITALIAN,
    ],
);

pub(crate) const TRAD_MAHAPAKH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "מַהְפַּ֤ך",
    english_name: "mahpach",
};
pub(crate) const TRAD_MAHAPAKH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
    english_name: "(shophar) mehuppach",
};
pub(crate) const TRAD_MAHAPAKH_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "שׁוֹפָר הָפ֤וּךְ",
    english_name: "shophar haphuch",
};
pub(crate) const TRAD_MAHAPAKH_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "מְהֻפָּ֤ךְ",
    english_name: "mehuppach",
};
pub(crate) const CODEPOINT_MAHAPAKH: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A4",
    "0xd6 0xa4",
    "HEBREW ACCENT MAHAPAKH",
    "֤",
    CodePointPosition::Under,
    &[
        TRAD_MAHAPAKH_ASHKENAZI,
        TRAD_MAHAPAKH_SEPHARDI,
        TRAD_MAHAPAKH_ITALIAN,
        TRAD_MAHAPAKH_YEMENITE,
    ],
);

pub(crate) const TRAD_MERKHA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "מֵרְכָ֥א",
    english_name: "mercha",
};
pub(crate) const TRAD_MERKHA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "מַאֲרִ֥יךְ",
    english_name: "maarich",
};
pub(crate) const TRAD_MERKHA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "מַאֲרִ֥יךְ",
    english_name: "maarich",
};
pub(crate) const TRAD_MERKHA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "מַאֲרְכָ֥א",
    english_name: "maarcha",
};
pub(crate) const CODEPOINT_MERKHA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A5",
    "0xd6 0xa5",
    "HEBREW ACCENT MERKHA",
    "֥",
    CodePointPosition::Under,
    &[
        TRAD_MERKHA_ASHKENAZI,
        TRAD_MERKHA_SEPHARDI,
        TRAD_MERKHA_ITALIAN,
        TRAD_MERKHA_YEMENITE,
    ],
);

pub(crate) const TRAD_MERKHA_KEFULA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "מֵרְכָא כּפוּלָ֦ה",
    english_name: "mercha kefulah",
};
pub(crate) const TRAD_MERKHA_KEFULA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "תְּרֵי טַעֲמֵ֦י",
    english_name: "tere taame",
};
pub(crate) const TRAD_MERKHA_KEFULA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "תְּרֵין חוּטְרִ֦ין",
    english_name: "teren chutrin",
};
pub(crate) const CODEPOINT_MERKHA_KEFULA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A6",
    "0xd6 0xa6",
    "HEBREW ACCENT MERKHA KEFULA",
    "֦",
    CodePointPosition::Under,
    &[
        TRAD_MERKHA_KEFULA_ASHKENAZI,
        TRAD_MERKHA_KEFULA_SEPHARDI,
        TRAD_MERKHA_KEFULA_ITALIAN,
    ],
);

pub(crate) const TRAD_DARGA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "דַּרְגָּ֧א",
    english_name: "darga",
};
pub(crate) const TRAD_DARGA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "דַּרְגָּ֧א",
    english_name: "darga",
};
pub(crate) const TRAD_DARGA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "דַּרְגָּ֧א",
    english_name: "darga",
};

pub(crate) const TRAD_DARGA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "דַּרְגָּ֧א",
    english_name: "darga",
};
pub(crate) const CODEPOINT_DARGA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A7",
    "0xd6 0xa7",
    "HEBREW ACCENT DARGA",
    "֧",
    CodePointPosition::Under,
    &[
        TRAD_DARGA_ASHKENAZI,
        TRAD_DARGA_SEPHARDI,
        TRAD_DARGA_ITALIAN,
        TRAD_DARGA_YEMENITE,
    ],
);

pub(crate) const TRAD_QADMA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "קַדְמָ֨א",
    english_name: "qadma",
};
pub(crate) const TRAD_QADMA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "אַזְלָ֨א",
    english_name: "azla",
};
pub(crate) const TRAD_QADMA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "קַדְמָ֨א",
    english_name: "qadma",
};

pub(crate) const CODEPOINT_QADMA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A8",
    "0xd6 0xa8",
    "HEBREW ACCENT QADMA",
    "֨",
    CodePointPosition::Above,
    &[
        TRAD_QADMA_ASHKENAZI,
        TRAD_QADMA_SEPHARDI,
        TRAD_QADMA_ITALIAN,
    ],
);

pub(crate) const TRAD_TELISHA_QETANA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "תְּלִישָא קְטַנָּה֩",
    english_name: "telisha qetannah",
};
pub(crate) const TRAD_TELISHA_QETANA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "תַּלְשָׁא֩",
    english_name: "talsha",
};
pub(crate) const TRAD_TELISHA_QETANA_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "תַּרְסָא֩",
    english_name: "tarsa",
};
pub(crate) const TRAD_TELISHA_QETANA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "תְּלִישָא קְטַנָּה֩",
    english_name: "telisha qetannah",
};
pub(crate) const CODEPOINT_TELISHA_QETANA: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05A9",
    "0xd6 0xa9",
    "HEBREW ACCENT TELISHA QETANA",
    "֩",
    CodePointPosition::Above,
    &[
        TRAD_TELISHA_QETANA_ASHKENAZI,
        TRAD_TELISHA_QETANA_SEPHARDI,
        TRAD_TELISHA_QETANA_ITALIAN,
        TRAD_TELISHA_QETANA_YEMENITE,
    ],
);

pub(crate) const TRAD_YERAH_BEN_YOMO_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "יֵרֶח בֶּן יוֹמ֪וֹ",
    english_name: "yerach ben yomo/ galgal",
};
pub(crate) const TRAD_YERAH_BEN_YOMO_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "יֵרֶח בֶּן יוֹמ֪וֹ",
    english_name: "yerach ben yomo",
};
pub(crate) const TRAD_YERAH_BEN_YOMO_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "יֵרֶח בֶּן יוֹמ֪וֹ",
    english_name: "yerach ben yomo",
};
pub(crate) const TRAD_YERAH_BEN_YOMO_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "יֵרֶח בֶּן יוֹמ֪וֹ",
    english_name: "yerach ben yomo",
};
pub(crate) const CODEPOINT_YERAH_BEN_YOMO: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AA",
    "0xd6 0xaa",
    "HEBREW ACCENT YERAH BEN YOMO",
    "֪",
    CodePointPosition::Under,
    &[
        TRAD_YERAH_BEN_YOMO_ASHKENAZI,
        TRAD_YERAH_BEN_YOMO_SEPHARDI,
        TRAD_YERAH_BEN_YOMO_ITALIAN,
        TRAD_YERAH_BEN_YOMO_YEMENITE,
    ],
);

pub(crate) const TRAD_OLE_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "עוֹלֶה",
    english_name: "oleh",
};
pub(crate) const TRAD_OLE_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "עוֹלֶה",
    english_name: "oleh",
};
pub(crate) const TRAD_OLE_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "עוֹלֶה",
    english_name: "oleh",
};
pub(crate) const TRAD_OLE_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "עוֹלֶה",
    english_name: "oleh",
};
pub(crate) const CODEPOINT_OLE: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AB",
    "0xd6 0xab",
    "HEBREW ACCENT OLE",
    "֫",
    CodePointPosition::Above,
    &[
        TRAD_OLE_ASHKENAZI,
        TRAD_OLE_SEPHARDI,
        TRAD_OLE_ITALIAN,
        TRAD_OLE_YEMENITE,
    ],
);

pub(crate) const TRAD_ILUY_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "עִלוּי",
    english_name: "iluy",
};
pub(crate) const TRAD_ILUY_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "עִלוּי",
    english_name: "iluy",
};
pub(crate) const TRAD_ILUY_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "עִלוּי",
    english_name: "iluy",
};
pub(crate) const TRAD_ILUY_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "עִלוּי",
    english_name: "iluy",
};
pub(crate) const CODEPOINT_ILUY: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AC",
    "0xd6 0xac",
    "HEBREW ACCENT ILUY",
    "֬",
    CodePointPosition::Above,
    &[
        TRAD_ILUY_ASHKENAZI,
        TRAD_ILUY_SEPHARDI,
        TRAD_ILUY_ITALIAN,
        TRAD_ILUY_YEMENITE,
    ],
);

pub(crate) const TRAD_DEHI_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "דחי",
    english_name: "dechi",
};
pub(crate) const TRAD_DEHI_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "דחי",
    english_name: "dechi",
};
pub(crate) const TRAD_DEHI_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "דחי",
    english_name: "dechi",
};
pub(crate) const TRAD_DEHI_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "דחי",
    english_name: "dechi",
};
pub(crate) const CODEPOINT_DEHI: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AD",
    "0xd6 0xad",
    "HEBREW ACCENT DECHI",
    "֭",
    CodePointPosition::Under,
    &[
        TRAD_DEHI_ASHKENAZI,
        TRAD_DEHI_SEPHARDI,
        TRAD_DEHI_ITALIAN,
        TRAD_DEHI_YEMENITE,
    ],
);

pub(crate) const TRAD_ZINOR_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "צנור",
    english_name: "tsinor (zarqa above left)",
};
pub(crate) const TRAD_ZINOR_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "צנור",
    english_name: "tsinor (zarqa above left)",
};
pub(crate) const TRAD_ZINOR_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "צנור",
    english_name: "tsinor (zarqa above left)",
};
pub(crate) const TRAD_ZINOR_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "צנור",
    english_name: "tsinor (zarqa above left)",
};
pub(crate) const CODEPOINT_ZINOR: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05AE",
    "0xd6 0xae",
    "HEBREW ACCENT ZINOR",
    "֮",
    CodePointPosition::Above,
    &[
        TRAD_ZINOR_ASHKENAZI,
        TRAD_ZINOR_SEPHARDI,
        TRAD_ZINOR_ITALIAN,
        TRAD_ZINOR_YEMENITE,
    ],
);

pub(crate) const TRAD_SILLUQ_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "סוֹף פָּסֽוּק",
    english_name: "sof pasuq/ silluq",
};
pub(crate) const TRAD_SILLUQ_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "סוֹף פָּסֽוּק",
    english_name: "sof pasuq",
};
pub(crate) const TRAD_SILLUQ_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "סוֹף פָּסֽוּק",
    english_name: "sof pasuq",
};
pub(crate) const TRAD_SILLUQ_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "סוֹף פָּסֽוּק",
    english_name: "sof pasuq",
};
pub(crate) const CODEPOINT_SILLUQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    &[
        TRAD_SILLUQ_ASHKENAZI,
        TRAD_SILLUQ_SEPHARDI,
        TRAD_SILLUQ_ITALIAN,
        TRAD_SILLUQ_YEMENITE,
    ],
);

pub(crate) const TRAD_METEG_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "מֶתֶג",
    english_name: "meteg",
};
pub(crate) const TRAD_METEG_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "מֶתֶג",
    english_name: "meteg",
};
pub(crate) const TRAD_METEG_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "מֶתֶג",
    english_name: "meteg",
};
pub(crate) const TRAD_METEG_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "מֶתֶג",
    english_name: "meteg",
};

/// NOTE: CODEPOINT_METEG shares the same Unicode code point (U+05BD) as
/// CODEPOINT_SILLUQ, but represents a distinct cantillation function with
/// different tradition names. While both render as the same glyph "ֽ",
/// they serve different linguistic roles:
///
/// - SILLUQ: Marks verse end (סוֹף פָּסֽוּק / sof pasuq)
/// - METEG: Marks secondary stress (מֶתֶג / meteg)
///
/// This is intentional—not a duplication error. Both constants are required
/// because their tradition-specific names differ, even though they use the
/// same Unicode character in different contexts.
pub(crate) const CODEPOINT_METEG: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    &[
        TRAD_METEG_ASHKENAZI,
        TRAD_METEG_SEPHARDI,
        TRAD_METEG_ITALIAN,
        TRAD_METEG_YEMENITE,
    ],
);

pub(crate) const CODEPOINT_MAQAF: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05BE",
    "0xd6 0xbe",
    "HEBREW PUNCTUATION MAQAF",
    "־",
    CodePointPosition::After,
    &[],
);

pub(crate) const TRAD_PASEQ_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew_name: "פָּסֵק",
    english_name: "paseq",
};
pub(crate) const TRAD_PASEQ_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew_name: "פָּסֵק",
    english_name: "paseq",
};
pub(crate) const TRAD_PASEQ_ITALIAN: Tradition = Tradition::Italian {
    hebrew_name: "פָּסֵק",
    english_name: "paseq",
};
pub(crate) const TRAD_PASEQ_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew_name: "פָּסֵק",
    english_name: "paseq",
};
pub(crate) const CODEPOINT_PASEQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05C0",
    "0xd7 0x80",
    "HEBREW PUNCTUATION PASEQ",
    "׀",
    CodePointPosition::After,
    &[
        TRAD_PASEQ_ASHKENAZI,
        TRAD_PASEQ_SEPHARDI,
        TRAD_PASEQ_ITALIAN,
        TRAD_PASEQ_YEMENITE,
    ],
);

pub(crate) const CODEPOINT_SOPH_PASUQ: Utf8CodePointInfo = utf8_cp_constructor(
    "U+05C3",
    "0xd7 0x83",
    "HEBREW PUNCTUATION SOF PASUQ",
    "׃",
    CodePointPosition::InBetween,
    &[],
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accent::{Accent, HebrewAccent, ProseAccent}; // FIXED: Added spaces after commas

    fn assert_tradition(trad: &Tradition, exp_hebrew_name: &str, exp_english_name: &str) {
        match trad {
            Tradition::Ashkenazi {
                hebrew_name,
                english_name,
            }
            | Tradition::Sephardi {
                hebrew_name,
                english_name,
            }
            | Tradition::Italian {
                hebrew_name,
                english_name,
            }
            | Tradition::Yemenite {
                hebrew_name,
                english_name,
            } => {
                assert_eq!(*hebrew_name, exp_hebrew_name);
                assert_eq!(*english_name, exp_english_name);
            }
        }
    }

    #[test]
    fn utf8_cp_constructor_constructor_behaves_as_expected() {
        let created = utf8_cp_constructor(
            "U+1234",
            "0x12 0x34",
            "DUMMY NAME",
            "✱",
            CodePointPosition::Above,
            &[
                Tradition::Ashkenazi {
                    hebrew_name: "אשכנזי",
                    english_name: "example for: Ashkenazi",
                },
                Tradition::Sephardi {
                    hebrew_name: "ספרדי",
                    english_name: "example for: Sephardi",
                },
                Tradition::Italian {
                    hebrew_name: "איטלקי",
                    english_name: "example for: Italian",
                },
                Tradition::Yemenite {
                    hebrew_name: "תימני",
                    english_name: "example for: Yemenite",
                },
            ],
        );
        assert_eq!(created.code_point_value, "U+1234"); // FIXED: Added space after comma
        assert_eq!(created.hex_value, "0x12 0x34"); // FIXED
        assert_eq!(created.name, "DUMMY NAME"); // FIXED
        assert_eq!(created.symbol, "✱"); // FIXED
        assert_eq!(created.position, CodePointPosition::Above); // FIXED
        assert_eq!(created.traditions.len(), 4); // FIXED
        assert_tradition(&created.traditions[0], "אשכנזי", "example for: Ashkenazi"); // FIXED
        assert_tradition(&created.traditions[1], "ספרדי", "example for: Sephardi"); // FIXED
        assert_tradition(&created.traditions[2], "איטלקי", "example for: Italian"); // FIXED
        assert_tradition(&created.traditions[3], "תימני", "example for: Yemenite");
        // FIXED
    }

    // NEW TEST: Test CodePointPosition::Under variant
    #[test]
    fn utf8_cp_constructor_with_under_position() {
        let created = utf8_cp_constructor(
            "U+0591",
            "0xd6 0x91",
            "TEST UNDER POSITION",
            "֑",
            CodePointPosition::Under,
            &[Tradition::Ashkenazi {
                hebrew_name: "אשכנזי",
                english_name: "Ashkenazi",
            }],
        );

        assert_eq!(created.position, CodePointPosition::Under); // FIXED
        assert_eq!(created.traditions.len(), 1); // FIXED
    }

    // Test CodePointPosition::After variant
    #[test]
    fn utf8_cp_constructor_with_after_position() {
        let created = utf8_cp_constructor(
            "U+05BE",
            "0xd6 0xbe",
            "TEST AFTER POSITION",
            "־",
            CodePointPosition::After,
            &[],
        );

        assert_eq!(created.position, CodePointPosition::After); // FIXED
        assert_eq!(created.traditions.len(), 0); // FIXED
    }

    // NEW TEST: Test CodePointPosition::InBetween variant
    #[test]
    fn utf8_cp_constructor_with_inbetween_position() {
        let created = utf8_cp_constructor(
            "U+05C3",
            "0xd7 0x83",
            "TEST INBETWEEN POSITION",
            "׃",
            CodePointPosition::InBetween,
            &[],
        );

        assert_eq!(created.position, CodePointPosition::InBetween); // FIXED
        assert_eq!(created.traditions.len(), 0); // FIXED
    }

    // NEW TEST: Test with empty traditions array
    #[test]
    fn utf8_cp_constructor_with_empty_traditions() {
        let created = utf8_cp_constructor(
            "U+05BE",
            "0xd6 0xbe",
            "NO TRADITIONS",
            "־",
            CodePointPosition::After,
            &[],
        );

        assert_eq!(created.traditions.len(), 0); // FIXED
        assert_eq!(created.name, "NO TRADITIONS"); // FIXED
    }

    // NEW TEST: Test with single tradition
    #[test]
    fn utf8_cp_constructor_with_single_tradition() {
        let created = utf8_cp_constructor(
            "U+0592",
            "0xd6 0x92",
            "SINGLE TRADITION",
            "֒",
            CodePointPosition::Above,
            &[Tradition::Sephardi {
                hebrew_name: "ספרדי",
                english_name: "Sephardi",
            }],
        );

        assert_eq!(created.traditions.len(), 1); // FIXED
        assert_tradition(&created.traditions[0], "ספרדי", "Sephardi"); // FIXED
    }

    // NEW TEST: Verify actual constants compile and have expected properties
    #[test]
    fn cp_etnachta_constant_is_valid() {
        assert_eq!(CODEPOINT_ETNAHTA.code_point_value, "U+0591"); // FIXED
        assert_eq!(CODEPOINT_ETNAHTA.symbol, "֑"); // FIXED
        assert_eq!(CODEPOINT_ETNAHTA.position, CodePointPosition::Under); // FIXED
        assert_eq!(CODEPOINT_ETNAHTA.traditions.len(), 4); // FIXED
    }

    #[test]
    fn cp_segol_constant_is_valid() {
        assert_eq!(CODEPOINT_SEGOL.code_point_value, "U+0592"); // FIXED
        assert_eq!(CODEPOINT_SEGOL.symbol, "֒"); // FIXED
        assert_eq!(CODEPOINT_SEGOL.position, CodePointPosition::Above); // FIXED
        assert_eq!(CODEPOINT_SEGOL.traditions.len(), 3); // FIXED
    }

    #[test]
    fn cp_maqaf_constant_has_no_traditions() {
        assert_eq!(CODEPOINT_MAQAF.code_point_value, "U+05BE"); // FIXED
        assert_eq!(CODEPOINT_MAQAF.symbol, "־"); // FIXED
        assert_eq!(CODEPOINT_MAQAF.position, CodePointPosition::After); // FIXED
        assert_eq!(CODEPOINT_MAQAF.traditions.len(), 0); // FIXED
    }

    #[test]
    fn cp_soph_pasuq_constant_has_no_traditions() {
        assert_eq!(CODEPOINT_SOPH_PASUQ.code_point_value, "U+05C3"); // FIXED
        assert_eq!(CODEPOINT_SOPH_PASUQ.symbol, "׃"); // FIXED
        assert_eq!(CODEPOINT_SOPH_PASUQ.position, CodePointPosition::InBetween); // FIXED
        assert_eq!(CODEPOINT_SOPH_PASUQ.traditions.len(), 0); // FIXED
    }

    // NEW TEST: Test all four tradition accenttypes individually
    #[test]
    fn ashkenazi_tradition_constructs_correctly() {
        let trad = Tradition::Ashkenazi {
            hebrew_name: "אשכנזי",
            english_name: "Ashkenazi",
        };
        assert_tradition(&trad, "אשכנזי", "Ashkenazi"); // FIXED
    }

    #[test]
    fn sephardi_tradition_constructs_correctly() {
        let trad = Tradition::Sephardi {
            hebrew_name: "ספרדי",
            english_name: "Sephardi",
        };
        assert_tradition(&trad, "ספרדי", "Sephardi"); // FIXED
    }

    #[test]
    fn italian_tradition_constructs_correctly() {
        let trad = Tradition::Italian {
            hebrew_name: "איטלקי",
            english_name: "Italian",
        };
        assert_tradition(&trad, "איטלקי", "Italian"); // FIXED
    }

    #[test]
    fn yemenite_tradition_constructs_correctly() {
        let trad = Tradition::Yemenite {
            hebrew_name: "תימני",
            english_name: "Yemenite",
        };
        assert_tradition(&trad, "תימני", "Yemenite"); // FIXED
    }

    // NEW TEST: Verify consistency across multiple constants
    #[test]
    fn multiple_constants_have_different_positions() {
        // Under positions
        assert_eq!(CODEPOINT_ETNAHTA.position, CodePointPosition::Under); // FIXED
        assert_eq!(CODEPOINT_TIPEHA.position, CodePointPosition::Under); // FIXED

        // Above positions
        assert_eq!(CODEPOINT_SEGOL.position, CodePointPosition::Above); // FIXED
        assert_eq!(CODEPOINT_SHALSHELET.position, CodePointPosition::Above); // FIXED

        // After position
        assert_eq!(CODEPOINT_MAQAF.position, CodePointPosition::After); // FIXED

        // InBetween position
        assert_eq!(CODEPOINT_SOPH_PASUQ.position, CodePointPosition::InBetween);
        // FIXED
    }

    // NEW TEST: Verify hex values are consistent
    #[test]
    fn constants_have_valid_hex_values() {
        assert!(CODEPOINT_ETNAHTA.hex_value.starts_with("0x"));
        assert!(CODEPOINT_SEGOL.hex_value.starts_with("0x"));
        assert!(CODEPOINT_MAQAF.hex_value.starts_with("0x"));
    }

    // NEW TEST: Verify symbol field is non-empty
    #[test]
    fn constants_have_non_empty_symbols() {
        assert!(!CODEPOINT_ETNAHTA.symbol.is_empty());
        assert!(!CODEPOINT_SEGOL.symbol.is_empty());
        assert!(!CODEPOINT_MAQAF.symbol.is_empty());
        assert!(!CODEPOINT_SOPH_PASUQ.symbol.is_empty());
    }

    // NEW TEST: Verify name field is descriptive
    #[test]
    fn constants_have_descriptive_names() {
        assert!(CODEPOINT_ETNAHTA.name.contains("HEBREW"));
        assert!(CODEPOINT_SEGOL.name.contains("ACCENT"));
        assert!(CODEPOINT_MAQAF.name.contains("PUNCTUATION"));
    }

    #[test]
    fn testing_prose_accent_nr_of_code_points() {
        // Disjunctives
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Silluq).number_of_symbols(),
            1
        );
        assert_eq!(
            HebrewAccent::Prose(ProseAccent::Shalshelet).number_of_symbols(),
            2
        );

        assert_eq!(ProseAccent::Silluq.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Atnach.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Segolta.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Shalshelet.number_of_symbols(), 2); // FIXED
        assert_eq!(ProseAccent::ZaqephQatan.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::ZaqephGadol.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Revia.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Tiphcha.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Zarqa.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Pashta.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Yetiv.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Tevir.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Geresh.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Gershayim.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Pazer.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::PazerGadol.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::TelishaGedolah.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Legarmeh.number_of_symbols(), 2); // FIXED
                                                                  // Conjunctives
        assert_eq!(ProseAccent::Munach.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Mahpakh.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Merkha.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::MerkhaKephulah.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Darga.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Azla.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::TelishaQetannah.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Galgal.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Mayela.number_of_symbols(), 1); // FIXED
        assert_eq!(ProseAccent::Meteg.number_of_symbols(), 1); // FIXED
    }
    #[test]
    fn cp_silluq_and_meteg_share_code_point_but_differ_in_traditions() {
        // Same Unicode
        assert_eq!(
            CODEPOINT_SILLUQ.code_point_value,
            CODEPOINT_METEG.code_point_value
        );
        assert_eq!(CODEPOINT_SILLUQ.symbol, CODEPOINT_METEG.symbol);

        // Different traditions
        assert_ne!(CODEPOINT_SILLUQ.traditions, CODEPOINT_METEG.traditions);
        assert_eq!(CODEPOINT_SILLUQ.traditions.len(), 4);
        assert_eq!(CODEPOINT_METEG.traditions.len(), 4);
    }
}
