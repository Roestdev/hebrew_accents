/// Constants below are a mix of the following:
/// - UTF8 code table (https://utf8-chartable.de/unicode-utf8-table.pl)
/// - naming of the accents according different traditions, see p.e.
///   https://en.wikipedia.org/wiki/Hebrew_cantillation
///   http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim
/// - the position of the accent relative to the related consonant
///
// Purpose: give more detailed information on the accents
/// Ordering of the list is based on the value of their Unicode code-point
// Standard library

// External crates

// Local modules / crate‑internal
use crate::CodePointPosition;
use crate::Tradition;
use crate::Utf8CodePointInfo;

/// Constructor for a single code‑point description
pub const fn utf8_cp(
    code_point: &'static str,
    hex_value: &'static str,
    name: &'static str,
    symbol: &'static str,
    position: CodePointPosition,
    traditions: &'static [Tradition],
) -> Utf8CodePointInfo {
    Utf8CodePointInfo {
        code_point,
        hex_value,
        name,
        symbol,
        position,
        traditions,
    }
}

pub const TR_ETNAHTA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "אֶתְנַחְתָּ֑א",
    name: "Etnachta",
};
pub const TR_ETNAHTA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "אַתְנָ֑ח",
    name: "Atnach",
};
pub const TR_ETNAHTA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "אַתְנָ֑ח",
    name: "Atnach",
};
pub const TR_ETNAHTA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "אֶתְנָחָ֑א",
    name: "Etnacha",
};
pub const CP_ETNAHTA: Utf8CodePointInfo = utf8_cp(
    "U+0591",
    "0xd6 0x91",
    "HEBREW ACCENT ETNAHTA",
    "֑",
    CodePointPosition::Under,
    &[
        TR_ETNAHTA_ASHKENAZI,
        TR_ETNAHTA_SEPHARDI,
        TR_ETNAHTA_ITALIAN,
        TR_ETNAHTA_YEMENITE,
    ],
);

pub const TR_SEGOL_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "סְגוֹל֒",
    name: "segol",
};
pub const TR_SEGOL_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "סְגוֹלְתָּא֒",
    name: "segolta",
};
pub const TR_SEGOL_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שְׁרֵי֒",
    name: "shere",
};
pub const CP_SEGOL: Utf8CodePointInfo = utf8_cp(
    "U+0592",
    "0xd6 0x92",
    "HEBREW ACCENT SEGOL",
    "֒",
    CodePointPosition::AbovePostPositive,
    &[TR_SEGOL_ASHKENAZI, TR_SEGOL_SEPHARDI, TR_SEGOL_ITALIAN],
);

pub const TR_SHALSHELET_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "שַׁלְשֶׁ֓לֶת",
    name: "shalshelet",
};
pub const TR_SHALSHELET_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "שַׁלְשֶׁ֓לֶת",
    name: "shalshelet",
};
pub const TR_SHALSHELET_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שַׁלְשֶׁ֓לֶת",
    name: "shalshelet",
};
pub const TR_SHALSHELET_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "שִׁישְׁלָ֓א",
    name: "shishla",
};

pub const CP_SHALSHELET: Utf8CodePointInfo = utf8_cp(
    "U+0593",
    "0xd6 0x93",
    "HEBREW ACCENT SHALSHELET",
    "֓",
    CodePointPosition::Above,
    &[
        TR_SHALSHELET_ASHKENAZI,
        TR_SHALSHELET_SEPHARDI,
        TR_SHALSHELET_ITALIAN,
        TR_SHALSHELET_YEMENITE,
    ],
);

pub const CP_ZAQEF_QATAN_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "זָקֵף קָטָ֔ן",
    name: "zaqeph qatan",
};
pub const CP_ZAQEF_QATAN_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "זָקֵף קָט֔וֹן",
    name: "zaqeph qaton",
};

pub const CP_ZAQEF_QATAN_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "זָקֵף קָט֔וֹן",
    name: "zaqeph qaton",
};
pub const CP_ZAQEF_QATAN_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "זָקֵף קָט֔וֹן",
    name: "zaqeph qaton",
};

pub const CP_ZAQEF_QATAN: Utf8CodePointInfo = utf8_cp(
    "U+0594",
    "0xd6 0x94",
    "HEBREW ACCENT ZAQEF QATAN",
    "֔",
    CodePointPosition::Above,
    &[
        CP_ZAQEF_QATAN_ASHKENAZI,
        CP_ZAQEF_QATAN_SEPHARDI,
        CP_ZAQEF_QATAN_ITALIAN,
        CP_ZAQEF_QATAN_YEMENITE,
    ],
);
pub const CP_ZAQEF_GADOL_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "זָקֵף גָּד֕וֹל",
    name: "zaqeph gadol",
};
pub const CP_ZAQEF_GADOL_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "זָקֵף גָּד֕וֹל",
    name: "zaqeph gadol",
};
pub const CP_ZAQEF_GADOL_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "זָקֵף גָּד֕וֹל",
    name: "zaqeph gadol",
};
pub const CP_ZAQEF_GADOL_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "זָקֵף גָּד֕וֹל",
    name: "zaqeph gadol",
};
pub const CP_ZAQEF_GADOL: Utf8CodePointInfo = utf8_cp(
    "U+0595",
    "0xd6 0x95",
    "HEBREW ACCENT ZAQEF GADOL",
    "֕",
    CodePointPosition::Above,
    &[
        CP_ZAQEF_GADOL_ASHKENAZI,
        CP_ZAQEF_GADOL_SEPHARDI,
        CP_ZAQEF_GADOL_ITALIAN,
        CP_ZAQEF_GADOL_YEMENITE,
    ],
);

pub const TR_TIPEHA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "טִפְחָ֖א",
    name: "tiphcha",
};
pub const TR_TIPEHA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "טַרְחָ֖א",
    name: "tarcha",
};
pub const TR_TIPEHA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "טַרְחָ֖א",
    name: "tarcha",
};
pub const TR_TIPEHA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "נְטוּיָ֖ה",
    name: "netuyah",
};
pub const CP_TIPEHA: Utf8CodePointInfo = utf8_cp(
    "U+0596",
    "0xd6 0x96",
    "HEBREW ACCENT TIPEHA",
    "֖",
    CodePointPosition::Under,
    &[
        TR_TIPEHA_ASHKENAZI,
        TR_TIPEHA_SEPHARDI,
        TR_TIPEHA_ITALIAN,
        TR_TIPEHA_YEMENITE,
    ],
);

pub const TR_REVIA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "רְבִ֗יע",
    name: "revia/revi’i",
};
pub const TR_REVIA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "רְבִ֗יע",
    name: "revia",
};
pub const TR_REVIA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "רְבִ֗יע",
    name: "revia",
};
pub const TR_REVIA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "רְבִ֗יע",
    name: "revia",
};
pub const CP_REVIA: Utf8CodePointInfo = utf8_cp(
    "U+0597",
    "0xd6 0x97",
    "HEBREW ACCENT REVIA",
    "֗",
    CodePointPosition::Above,
    &[
        TR_REVIA_ASHKENAZI,
        TR_REVIA_SEPHARDI,
        TR_REVIA_ITALIAN,
        TR_REVIA_YEMENITE,
    ],
);

pub const TR_ZARQA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "זַרְקָא֘",
    name: "zarqa",
};
pub const TR_ZARQA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "זַרְקָא֘",
    name: "zarqa",
};
pub const TR_ZARQA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "זַרְקָא֘",
    name: "zarqa",
};
pub const TR_ZARQA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "צִנּוֹר֘",
    name: "tsinnor",
};
pub const CP_ZARQA: Utf8CodePointInfo = utf8_cp(
    "U+0598",
    "0xd6 0x98",
    "HEBREW ACCENT ZARQA",
    "֘",
    CodePointPosition::AbovePostPositive,
    &[
        TR_ZARQA_ASHKENAZI,
        TR_ZARQA_SEPHARDI,
        TR_ZARQA_ITALIAN,
        TR_ZARQA_YEMENITE,
    ],
);

pub const TR_PASHTA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "פַּשְׁטָא֙",
    name: "pashta",
};
pub const TR_PASHTA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "קַדְמָא֙",
    name: "qadma",
};
pub const TR_PASHTA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "פַּשְׁטָא֙",
    name: "pashta",
};
pub const TR_PASHTA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "אַזְלָא֙",
    name: "azla",
};
pub const CP_PASHTA: Utf8CodePointInfo = utf8_cp(
    "U+0599",
    "0xd6 0x99",
    "HEBREW ACCENT PASHTA",
    "֙",
    CodePointPosition::AbovePostPositive,
    &[
        TR_PASHTA_ASHKENAZI,
        TR_PASHTA_SEPHARDI,
        TR_PASHTA_ITALIAN,
        TR_PASHTA_YEMENITE,
    ],
);

pub const TR_YETIV_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "יְ֚תִיב",
    name: "yetiv",
};
pub const TR_YETIV_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "יְ֚תִיב",
    name: "yetiv",
};
pub const TR_YETIV_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שׁ֚וֹפָר יְתִיב",
    name: "shophar yetiv",
};
pub const TR_YETIV_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "יְ֚תִיב",
    name: "yetiv",
};
pub const CP_YETIV: Utf8CodePointInfo = utf8_cp(
    "U+059A",
    "0xd6 0x9a",
    "HEBREW ACCENT YETIV",
    "֚",
    CodePointPosition::UnderPrePositive,
    &[
        TR_YETIV_ASHKENAZI,
        TR_YETIV_SEPHARDI,
        TR_YETIV_ITALIAN,
        TR_YETIV_YEMENITE,
    ],
);

pub const TR_TEVIR_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "תְּבִ֛יר",
    name: "tevir",
};
pub const TR_TEVIR_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "תְּבִ֛יר",
    name: "tevir",
};
pub const TR_TEVIR_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "תְּבִ֛יר",
    name: "tevir",
};
pub const TR_TEVIR_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "תַּבְרָ֛א",
    name: "tavra",
};
pub const CP_TEVIR: Utf8CodePointInfo = utf8_cp(
    "U+059B",
    "0xd6 0x9b",
    "HEBREW ACCENT TEVIR",
    "֛",
    CodePointPosition::Under,
    &[
        TR_TEVIR_ASHKENAZI,
        TR_TEVIR_SEPHARDI,
        TR_TEVIR_ITALIAN,
        TR_TEVIR_YEMENITE,
    ],
);

pub const TR_GERESH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "גֵּ֜רֵשׁ",
    name: "geresh/azla",
};
pub const TR_GERESH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "גְּרִ֜ישׁ",
    name: "gerish",
};
pub const TR_GERESH_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "גֵּ֜רֵשׁ",
    name: "geresh/azla",
};
pub const TR_GERESH_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "טָרֵ֜ס",
    name: "tares",
};
pub const CP_GERESH: Utf8CodePointInfo = utf8_cp(
    "U+059C",
    "0xd6 0x9c",
    "HEBREW ACCENT GERESH",
    "֜",
    CodePointPosition::Above,
    &[
        TR_GERESH_ASHKENAZI,
        TR_GERESH_SEPHARDI,
        TR_GERESH_ITALIAN,
        TR_GERESH_YEMENITE,
    ],
);

// TODO CP_GERESH_MUQDAM
// pub const CP_GERESH_MUQDAM: Utf8CodePointInfo = utf8_cp(
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

pub const TR_GERSHAYIM_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "גֵּרְשַׁ֞יִם",
    name: "gershayim",
};
pub const TR_GERSHAYIM_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "שְׁנֵי גְרִישִׁ֞ין",
    name: "shene gerishin",
};
pub const TR_GERSHAYIM_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שְׁנֵי גְרִישִׁ֞ין",
    name: "shene gerishin",
};
pub const TR_GERSHAYIM_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "שְׁנֵי גְרִישִׁ֞ין",
    name: "shene gerishin",
};
pub const CP_GERSHAYIM: Utf8CodePointInfo = utf8_cp(
    "U+059E",
    "0xd6 0x9e",
    "HEBREW ACCENT GERSHAYIM",
    "֞",
    CodePointPosition::Above,
    &[
        TR_GERSHAYIM_ASHKENAZI,
        TR_GERSHAYIM_SEPHARDI,
        TR_GERSHAYIM_ITALIAN,
        TR_GERSHAYIM_YEMENITE,
    ],
);

pub const TR_QARNEY_PARA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "קַרְנֵי פָרָ֟ה",
    name: "qarne pharah / pazer gadol",
};
pub const TR_QARNEY_PARA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "קַרְנֵי פָרָ֟ה",
    name: "qarne pharah",
};
pub const TR_QARNEY_PARA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "קַרְנֵי פָרָ֟ה",
    name: "qarne pharah",
};
pub const TR_QARNEY_PARA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "קַרְנֵי פָרָ֟ה",
    name: "qarne pharah",
};
pub const CP_QARNEY_PARA: Utf8CodePointInfo = utf8_cp(
    "U+059F",
    "0xd6 0x9f",
    "HEBREW ACCENT QARNEY PARA",
    "֟",
    CodePointPosition::Above,
    &[
        TR_QARNEY_PARA_ASHKENAZI,
        TR_QARNEY_PARA_SEPHARDI,
        TR_QARNEY_PARA_ITALIAN,
        TR_QARNEY_PARA_YEMENITE,
    ],
);

pub const TR_TELISHA_GEDOLA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "תְּ֠לִישָא גְדוֹלָה",
    name: "telisha gedolah",
};
pub const TR_TELISHA_GEDOLA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "תִּ֠רְצָה",
    name: "tirtsah",
};
pub const TR_TELISHA_GEDOLA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "תַּ֠לְשָׁא",
    name: "talsha",
};
pub const TR_TELISHA_GEDOLA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "תְּ֠לִישָא גְדוֹלָה",
    name: "telisha gedolah",
};
pub const CP_TELISHA_GEDOLA: Utf8CodePointInfo = utf8_cp(
    "U+05A0",
    "0xd6 0xa0",
    "HEBREW ACCENT TELISHA GEDOLA",
    "֠",
    CodePointPosition::AbovePrePositive,
    &[
        TR_TELISHA_GEDOLA_ASHKENAZI,
        TR_TELISHA_GEDOLA_SEPHARDI,
        TR_TELISHA_GEDOLA_ITALIAN,
        TR_TELISHA_GEDOLA_YEMENITE,
    ],
);

pub const TR_PAZER_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "פָּזֵ֡ר",
    name: "pazer",
};
pub const TR_PAZER_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "פָּזֵר גָּד֡וֹל",
    name: "pazer gadol",
};
pub const TR_PAZER_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "פָּזֵר גָּד֡וֹל",
    name: "pazer gadol",
};

pub const CP_PAZER: Utf8CodePointInfo = utf8_cp(
    "U+05A1",
    "0xd6 0xa1",
    "HEBREW ACCENT PAZER",
    "֡",
    CodePointPosition::Above,
    &[TR_PAZER_ASHKENAZI, TR_PAZER_SEPHARDI, TR_PAZER_ITALIAN],
);

// TODO CP_ATNAH_HAFUKH
// pub const CP_ATNAH_HAFUKH: Utf8CodePointInfo = utf8_cp(
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

pub const TR_MUNAH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "מוּנַ֣ח",
    name: "munach",
};
pub const TR_MUNAH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "שׁוֹפָר הוֹלֵ֣ךְ",
    name: "shophar holech",
};
pub const TR_MUNAH_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שׁוֹפָר עִלּ֣וּי",
    name: "shophar illuy",
};

pub const CP_MUNAH: Utf8CodePointInfo = utf8_cp(
    "U+05A3",
    "0xd6 0xa3",
    "HEBREW ACCENT MUNAH",
    "֣",
    CodePointPosition::Under,
    &[TR_MUNAH_ASHKENAZI, TR_MUNAH_SEPHARDI, TR_MUNAH_ITALIAN],
);

pub const TR_MAHAPAKH_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "מַהְפַּ֤ך",
    name: "mahpach",
};
pub const TR_MAHAPAKH_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
    name: "(shophar) mehuppach",
};
pub const TR_MAHAPAKH_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "שׁוֹפָר הָפ֤וּךְ",
    name: "shophar haphuch",
};
pub const TR_MAHAPAKH_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "מְהֻפָּ֤ךְ",
    name: "mehuppach",
};
pub const CP_MAHAPAKH: Utf8CodePointInfo = utf8_cp(
    "U+05A4",
    "0xd6 0xa4",
    "HEBREW ACCENT MAHAPAKH",
    "֤",
    CodePointPosition::Under,
    &[
        TR_MAHAPAKH_ASHKENAZI,
        TR_MAHAPAKH_SEPHARDI,
        TR_MAHAPAKH_ITALIAN,
        TR_MAHAPAKH_YEMENITE,
    ],
);

pub const TR_MERKHA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "מֵרְכָ֥א",
    name: "mercha",
};
pub const TR_MERKHA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "מַאֲרִ֥יךְ",
    name: "maarich",
};
pub const TR_MERKHA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "מַאֲרִ֥יךְ",
    name: "maarich",
};
pub const TR_MERKHA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "מַאֲרְכָ֥א",
    name: "maarcha",
};
pub const CP_MERKHA: Utf8CodePointInfo = utf8_cp(
    "U+05A5",
    "0xd6 0xa5",
    "HEBREW ACCENT MERKHA",
    "֥",
    CodePointPosition::Under,
    &[
        TR_MERKHA_ASHKENAZI,
        TR_MERKHA_SEPHARDI,
        TR_MERKHA_ITALIAN,
        TR_MERKHA_YEMENITE,
    ],
);

pub const TR_MERKHA_KEFULA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "מֵרְכָא כּפוּלָ֦ה",
    name: "mercha kefulah",
};
pub const TR_MERKHA_KEFULA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "תְּרֵי טַעֲמֵ֦י",
    name: "tere taame",
};
pub const TR_MERKHA_KEFULA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "תְּרֵין חוּטְרִ֦ין",
    name: "teren chutrin",
};
pub const CP_MERKHA_KEFULA: Utf8CodePointInfo = utf8_cp(
    "U+05A6",
    "0xd6 0xa6",
    "HEBREW ACCENT MERKHA KEFULA",
    "֦",
    CodePointPosition::Under,
    &[
        TR_MERKHA_KEFULA_ASHKENAZI,
        TR_MERKHA_KEFULA_SEPHARDI,
        TR_MERKHA_KEFULA_ITALIAN,
    ],
);

pub const TR_DARGA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "דַּרְגָּ֧א",
    name: "darga",
};
pub const TR_DARGA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "דַּרְגָּ֧א",
    name: "darga",
};
pub const TR_DARGA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "דַּרְגָּ֧א",
    name: "darga",
};

pub const TR_DARGA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "דַּרְגָּ֧א",
    name: "darga",
};
pub const CP_DARGA: Utf8CodePointInfo = utf8_cp(
    "U+05A7",
    "0xd6 0xa7",
    "HEBREW ACCENT DARGA",
    "֧",
    CodePointPosition::Under,
    &[
        TR_DARGA_ASHKENAZI,
        TR_DARGA_SEPHARDI,
        TR_DARGA_ITALIAN,
        TR_DARGA_YEMENITE,
    ],
);

pub const TR_QADMA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "קַדְמָ֨א",
    name: "qadma",
};
pub const TR_QADMA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "אַזְלָ֨א",
    name: "azla",
};
pub const TR_QADMA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "קַדְמָ֨א",
    name: "qadma",
};

pub const CP_QADMA: Utf8CodePointInfo = utf8_cp(
    "U+05A8",
    "0xd6 0xa8",
    "HEBREW ACCENT QADMA",
    "֨",
    CodePointPosition::Above,
    &[TR_QADMA_ASHKENAZI, TR_QADMA_SEPHARDI, TR_QADMA_ITALIAN],
);

pub const TR_TELISHA_QETANA_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "תְּלִישָא קְטַנָּה֩",
    name: "telisha qetannah",
};
pub const TR_TELISHA_QETANA_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "תַּלְשָׁא֩",
    name: "talsha",
};
pub const TR_TELISHA_QETANA_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "תַּרְסָא֩",
    name: "tarsa",
};
pub const TR_TELISHA_QETANA_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "תְּלִישָא קְטַנָּה֩",
    name: "telisha qetannah",
};
pub const CP_TELISHA_QETANA: Utf8CodePointInfo = utf8_cp(
    "U+05A9",
    "0xd6 0xa9",
    "HEBREW ACCENT TELISHA QETANA",
    "֩",
    CodePointPosition::AbovePostPositive,
    &[
        TR_TELISHA_QETANA_ASHKENAZI,
        TR_TELISHA_QETANA_SEPHARDI,
        TR_TELISHA_QETANA_ITALIAN,
        TR_TELISHA_QETANA_YEMENITE,
    ],
);

pub const TR_YERAH_BEN_YOMO_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
    name: "yerach ben yomo/ galgal",
};
pub const TR_YERAH_BEN_YOMO_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
    name: "yerach ben yomo",
};
pub const TR_YERAH_BEN_YOMO_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
    name: "yerach ben yomo",
};
pub const TR_YERAH_BEN_YOMO_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "יֵרֶח בֶּן יוֹמ֪וֹ",
    name: "yerach ben yomo",
};
pub const CP_YERAH_BEN_YOMO: Utf8CodePointInfo = utf8_cp(
    "U+05AA",
    "0xd6 0xaa",
    "HEBREW ACCENT YERAH BEN YOMO",
    "֪",
    CodePointPosition::Under,
    &[
        TR_YERAH_BEN_YOMO_ASHKENAZI,
        TR_YERAH_BEN_YOMO_SEPHARDI,
        TR_YERAH_BEN_YOMO_ITALIAN,
        TR_YERAH_BEN_YOMO_YEMENITE,
    ],
);

pub const TR_OLE_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "עוֹלֶה",
    name: "oleh",
};
pub const TR_OLE_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "עוֹלֶה",
    name: "oleh",
};
pub const TR_OLE_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "עוֹלֶה",
    name: "oleh",
};
pub const TR_OLE_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "עוֹלֶה",
    name: "oleh",
};
pub const CP_OLE: Utf8CodePointInfo = utf8_cp(
    "U+05AB",
    "0xd6 0xab",
    "HEBREW ACCENT OLE",
    "֫",
    CodePointPosition::Above,
    &[
        TR_OLE_ASHKENAZI,
        TR_OLE_SEPHARDI,
        TR_OLE_ITALIAN,
        TR_OLE_YEMENITE,
    ],
);

pub const TR_ILUY_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "עִלוּי",
    name: "iluy",
};
pub const TR_ILUY_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "עִלוּי",
    name: "iluy",
};
pub const TR_ILUY_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "עִלוּי",
    name: "iluy",
};
pub const TR_ILUY_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "עִלוּי",
    name: "iluy",
};
pub const CP_ILUY: Utf8CodePointInfo = utf8_cp(
    "U+05AC",
    "0xd6 0xac",
    "HEBREW ACCENT ILUY",
    "֬",
    CodePointPosition::Above,
    &[
        TR_ILUY_ASHKENAZI,
        TR_ILUY_SEPHARDI,
        TR_ILUY_ITALIAN,
        TR_ILUY_YEMENITE,
    ],
);

pub const TR_DEHI_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "דחי",
    name: "dechi",
};
pub const TR_DEHI_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "דחי",
    name: "dechi",
};
pub const TR_DEHI_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "דחי",
    name: "dechi",
};
pub const TR_DEHI_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "דחי",
    name: "dechi",
};
pub const CP_DEHI: Utf8CodePointInfo = utf8_cp(
    "U+05AD",
    "0xd6 0xad",
    "HEBREW ACCENT DEHI",
    "֭",
    CodePointPosition::UnderPrePositive,
    &[
        TR_DEHI_ASHKENAZI,
        TR_DEHI_SEPHARDI,
        TR_DEHI_ITALIAN,
        TR_DEHI_YEMENITE,
    ],
);

pub const TR_ZINOR_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "צנור",
    name: "tsinor (zarqa above left)",
};
pub const TR_ZINOR_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "צנור",
    name: "tsinor (zarqa above left)",
};
pub const TR_ZINOR_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "צנור",
    name: "tsinor (zarqa above left)",
};
pub const TR_ZINOR_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "צנור",
    name: "tsinor (zarqa above left)",
};
pub const CP_ZINOR: Utf8CodePointInfo = utf8_cp(
    "U+05AE",
    "0xd6 0xae",
    "HEBREW ACCENT ZINOR",
    "֮",
    CodePointPosition::AbovePostPositive,
    &[
        TR_ZINOR_ASHKENAZI,
        TR_ZINOR_SEPHARDI,
        TR_ZINOR_ITALIAN,
        TR_ZINOR_YEMENITE,
    ],
);

pub const TR_SILLUQ_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "סוֹף פָּסֽוּק",
    name: "sof pasuq/ silluq",
};
pub const TR_SILLUQ_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "סוֹף פָּסֽוּק",
    name: "sof pasuq",
};
pub const TR_SILLUQ_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "סוֹף פָּסֽוּק",
    name: "sof pasuq",
};
pub const TR_SILLUQ_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "סוֹף פָּסֽוּק",
    name: "sof pasuq",
};
pub const CP_SILLUQ: Utf8CodePointInfo = utf8_cp(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    &[
        TR_SILLUQ_ASHKENAZI,
        TR_SILLUQ_SEPHARDI,
        TR_SILLUQ_ITALIAN,
        TR_SILLUQ_YEMENITE,
    ],
);

pub const TR_METEG_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "מֶתֶג",
    name: "meteg",
};
pub const TR_METEG_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "מֶתֶג",
    name: "meteg",
};
pub const TR_METEG_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "מֶתֶג",
    name: "meteg",
};
pub const TR_METEG_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "מֶתֶג",
    name: "meteg",
};

pub const CP_METEG: Utf8CodePointInfo = utf8_cp(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
    &[
        TR_METEG_ASHKENAZI,
        TR_METEG_SEPHARDI,
        TR_METEG_ITALIAN,
        TR_METEG_YEMENITE,
    ],
);

pub const CP_MAQAF: Utf8CodePointInfo = utf8_cp(
    "U+05BE",
    "0xd6 0xbe",
    "HEBREW PUNCTUATION MAQAF",
    "־",
    CodePointPosition::End,
    &[],
);

pub const TR_PASEQ_ASHKENAZI: Tradition = Tradition::Ashkenazi {
    hebrew: "פָּסֵק",
    name: "paseq",
};
pub const TR_PASEQ_SEPHARDI: Tradition = Tradition::Sephardi {
    hebrew: "פָּסֵק",
    name: "paseq",
};
pub const TR_PASEQ_ITALIAN: Tradition = Tradition::Italian {
    hebrew: "פָּסֵק",
    name: "paseq",
};
pub const TR_PASEQ_YEMENITE: Tradition = Tradition::Yemenite {
    hebrew: "פָּסֵק",
    name: "paseq",
};
pub const CP_PASEQ: Utf8CodePointInfo = utf8_cp(
    "U+05C0",
    "0xd7 0x80",
    "HEBREW PUNCTUATION PASEQ",
    "׀",
    CodePointPosition::End,
    &[
        TR_PASEQ_ASHKENAZI,
        TR_PASEQ_SEPHARDI,
        TR_PASEQ_ITALIAN,
        TR_PASEQ_YEMENITE,
    ],
);

// pub const CP_SOF_PASUQ: Utf8CodePointInfo = utf8_cp(
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
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tradition(trad: &Tradition, exp_hebrew: &str, exp_name: &str) {
        match trad {
            Tradition::Ashkenazi { hebrew, name }
            | Tradition::Sephardi { hebrew, name }
            | Tradition::Italian { hebrew, name }
            | Tradition::Yemenite { hebrew, name } => {
                assert_eq!(hebrew, &exp_hebrew);
                assert_eq!(name, &exp_name);
            }
        }
    }

    #[test]
    fn utf8_cp_constructor_behaves_as_expected() {
        let created = utf8_cp(
            "U+1234",
            "0x12 0x34",
            "DUMMY NAME",
            "✱",
            CodePointPosition::Above,
            &[Tradition::Ashkenazi {
                hebrew: "דוגמה",
                name: "example",
            }],
        );

        assert_eq!(created.code_point, "U+1234");
        assert_eq!(created.hex_value, "0x12 0x34");
        assert_eq!(created.name, "DUMMY NAME");
        assert_eq!(created.symbol, "✱");
        assert_eq!(created.position, CodePointPosition::Above);
        assert_eq!(created.traditions.len(), 1);
        assert_tradition(&created.traditions[0], "דוגמה", "example");
    }
}
