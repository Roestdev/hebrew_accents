const CP_SILLUQ: Utf8CodePoint = Utf8CodePoint {
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

const CP_ETNAHTA: Utf8CodePoint = Utf8CodePoint {
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

const CP_SEGOL: Utf8CodePoint = Utf8CodePoint {
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

const CP_SHALSHELET: Utf8CodePoint = Utf8CodePoint {
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

const CP_ZAQEF_QATAN: Utf8CodePoint = Utf8CodePoint {
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

const CP_ZAQEF_GADOL: Utf8CodePoint = Utf8CodePoint {
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

const CP_TIPEHA: Utf8CodePoint = Utf8CodePoint {
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

const CP_REVIA: Utf8CodePoint = Utf8CodePoint {
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

const CP_ZARQA: Utf8CodePoint = Utf8CodePoint {
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

// const CP_PASHTA: Utf8CodePoint = Utf8CodePoint {
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

// const CP_: Utf8CodePoint = Utf8CodePoint { code_point:"U+0599 U+05A8 ", hex_value: "", name: "", symbol: "", position: AccentPosition::Above, ashkenazi: Tradition::Ashkenazi { hebrew: "שְׁנֵ֨י פַּשְׁטִין֙", name: "Shene pashtin/pashtayim ", }, sephardi: Tradition::Sephardi { hebrew: "תּרֵ֨י קַדְמִין֙", name: "Tere qadmin ",  }, italian: Tradition::Italian { hebrew: "שְׁנֵ֨י פַּשְׁטִין֙", name: "(Shene) pashtin ",  }, yemenite: Tradition::Yemenite { hebrew: "",name: "",},};

const CP_YETIV: Utf8CodePoint = Utf8CodePoint {
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

const CP_TEVIR: Utf8CodePoint = Utf8CodePoint {
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

const CP_PAZER: Utf8CodePoint = Utf8CodePoint {
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

const CP_QARNEY_PARA: Utf8CodePoint = Utf8CodePoint {
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

const CP_TELISHA_GEDOLA: Utf8CodePoint = Utf8CodePoint {
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

const CP_GERESH: Utf8CodePoint = Utf8CodePoint {
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

const CP_GERSHAYIM: Utf8CodePoint = Utf8CodePoint {
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

//const CP_: Utf8CodePoint = Utf8CodePoint { code_point:"U+05A3", hex_value: "", name: "", symbol: "", position: AccentPosition::Above, ashkenazi: Tradition::Ashkenazi { hebrew: "מוּנַח לְגַרְמֵ֣הּ׀", name: "Munach legarmeh ", }, sephardi: Tradition::Sephardi { hebrew: "פָּסֵ֣ק׀", name: "Paseq ",  }, italian: Tradition::Italian { hebrew: "לְגַרְמֵ֣הּ׀", name: "Legarmeh ",  }, yemenite: Tradition::Yemenite { hebrew: "unknown",name: "unknown",},};

const CP_MERKHA: Utf8CodePoint = Utf8CodePoint {
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

const CP_MUNAH: Utf8CodePoint = Utf8CodePoint {
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

const CP_MAHAPAKH: Utf8CodePoint = Utf8CodePoint {
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

const CP_DARGA: Utf8CodePoint = Utf8CodePoint {
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

const CP_QADMA: Utf8CodePoint = Utf8CodePoint {
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

const CP_TELISHA_QETANA: Utf8CodePoint = Utf8CodePoint {
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

const CP_MERKHA_KEFULA: Utf8CodePoint = Utf8CodePoint {
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

const CP_YERAH_BEN_YOMO: Utf8CodePoint = Utf8CodePoint {
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

// const CP_GERESH_MUQDAM: Utf8CodePoint = Utf8CodePoint {
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

// const CP_ATNAH_HAFUKH: Utf8CodePoint = Utf8CodePoint {
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

const CP_OLE: Utf8CodePoint = Utf8CodePoint {
    code_point: "U+05AB ",
    hex_value: "0xd6 0xab",
    name: "HEBREW ACCENT OLE",
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

const CP_ILUY: Utf8CodePoint = Utf8CodePoint {
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

const CP_DEHI: Utf8CodePoint = Utf8CodePoint {
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

const CP_ZINOR: Utf8CodePoint = Utf8CodePoint {
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

const CP_METEG: Utf8CodePoint = Utf8CodePoint {
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

// const CP_MAQAF: Utf8CodePoint = Utf8CodePoint {
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

const CP_PASEQ: Utf8CodePoint = Utf8CodePoint {
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

// const CP_SOF_PASUQ: Utf8CodePoint = Utf8CodePoint {
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

//////////////////////////

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum ProseAccent {
    // Disjunctives
    #[default]
    Silluq,
    Atnach,
    Segolta,
    Shalshelet,
    ZaqephQaton,
    ZaqephGadol,
    Revia,
    Tiphcha,
    Zarqa,
    Pashta,
    Yetiv,
    Tevir,
    Geresh,
    Gershayim,
    Pazer,
    PazerGadol,
    TelishaGedolah,
    Legarmeh,
    // Conjunctives
    Munnach,
    Mahpakh,
    Merkha,
    MerkhaKephulah,
    Darga,
    Azla,
    TelishaQetannah,
    Galgal,
    Mayela,
    Meteg,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum PoetryAccent {
    // Disjunctives
    #[default]
    Silluq,
    OleWeYored,
    Atnach,
    ReviaGadol,
    ReviaMugrash,
    ShalsheletGadol,
    Tsinnor,
    ReviaQaton,
    Dechi,
    Pazer,
    MehuppakhLegarmeh,
    AzlaLegarmeh,
    // Conjunctives
    Munnach,
    Merkha,
    Illuy,
    Tarkha,
    Galgal,
    Mehuppakh,
    Azla,
    ShalsheletQetannah,
    TsinnoritMerkha,
    TsinnoritMahpakh,
    Meteg,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HebrewAccentDetails {
    bhs_name: String,
    bhs_hebrew: String,
    bhs_meaning: String,
    alt_name: Option<String>,
    alt_hebrew: Option<String>,
    alt_meaning: Option<String>,
    acc_category: AccentCategory,
    acc_type: AccentType,
    code_point_1: Utf8CodePoint,
    code_point_2: Option<Utf8CodePoint>,
    comment: Option<String>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct Utf8CodePoint {
    code_point: &'static str,
    hex_value: &'static str,
    name: &'static str,
    symbol: &'static str,
    position: AccentPosition,
    ashkenazi: Tradition,
    sephardi: Tradition,
    italian: Tradition,
    yemenite: Tradition,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Tradition {
    Ashkenazi {
        hebrew: &'static str,
        name: &'static str,
    },
    Sephardi {
        hebrew: &'static str,
        name: &'static str,
    },
    Italian {
        hebrew: &'static str,
        name: &'static str,
    },
    Yemenite {
        hebrew: &'static str,
        name: &'static str,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentCategory {
    Conjunctive,
    #[default]
    Disjunctive,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentType {
    #[default]
    Primairy,
    Secondary,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentPosition {
    #[default]
    Above,
    AbovePostPositive,
    AbovePrePositive,
    End, // used to denote a Paseq, Soph Pasuq
    Under,
    UnderPrePositive,
}

impl ProseAccent {
    #[allow(unused)]
    pub fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            ProseAccent::Silluq => 1,
            ProseAccent::Atnach => 2,
            ProseAccent::Segolta => 3,
            ProseAccent::Shalshelet => 4,
            ProseAccent::ZaqephQaton => 5,
            ProseAccent::ZaqephGadol => 6,
            ProseAccent::Revia => 7,
            ProseAccent::Tiphcha => 8,
            ProseAccent::Zarqa => 9,
            ProseAccent::Pashta => 10,
            ProseAccent::Yetiv => 11,
            ProseAccent::Tevir => 12,
            ProseAccent::Geresh => 13,
            ProseAccent::Gershayim => 14,
            ProseAccent::Pazer => 15,
            ProseAccent::PazerGadol => 16,
            ProseAccent::TelishaGedolah => 17,
            ProseAccent::Legarmeh => 18,
            // Conjunctives
            ProseAccent::Munnach => 19,
            ProseAccent::Mahpakh => 20,
            ProseAccent::Merkha => 21,
            ProseAccent::MerkhaKephulah => 22,
            ProseAccent::Darga => 23,
            ProseAccent::Azla => 24,
            ProseAccent::TelishaQetannah => 25,
            ProseAccent::Galgal => 26,
            ProseAccent::Mayela => 27,
            ProseAccent::Meteg => 28,
        }
    }
    /// Returns detailsrmation about the accent.
    /// This can be expanded to include more details as needed.
    #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            ProseAccent::Silluq => HebrewAccentDetails {
                bhs_name: "Silluq".to_string(),
                bhs_hebrew: "סִלּוּק".to_string(),
                bhs_meaning:"close, cessation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SILLUQ,
                code_point_2: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.".to_string()),
                            },
            ProseAccent::Atnach => HebrewAccentDetails {
                bhs_name: "Atnach".to_string(),
                bhs_hebrew: "אתְנָח".to_string(),
                bhs_meaning: "a causing to rest".to_string(),
                alt_name: Some("Etnachta".to_string()),
                alt_hebrew: Some("אֶתְנַחְתָּא".to_string()),
                alt_meaning: Some("pauser".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ETNAHTA,
                code_point_2: None,
                comment: None,            },
            ProseAccent::Segolta => HebrewAccentDetails {
                bhs_name: "Segolta".to_string(),
                bhs_hebrew: "סְגֹולְתָּא".to_string(),
                bhs_meaning: "a little grape-bunch".to_string(),
                alt_name: Some("Segol".to_string()),
                alt_hebrew: Some("סְגֹול".to_string()),
                alt_meaning: Some("grape-bunch".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SEGOL,
                code_point_2: None,
                comment: None,            },
            ProseAccent::Shalshelet => HebrewAccentDetails {
                bhs_name: "Shalshelet".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת".to_string(),
                bhs_meaning: "chain or link".to_string(), 
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SHALSHELET,
                code_point_2: Some(CP_PASEQ),
                comment: None,            },
            ProseAccent::ZaqephQaton => HebrewAccentDetails {
                bhs_name: "Zaqeph Qaton".to_string(),
                bhs_hebrew: "זָקֵף קָטוֹן".to_string(),
                bhs_meaning:"small upright".to_string(), 
                alt_name: Some("Zaqeph Qatan".to_string()),
                alt_hebrew: Some("זָקֵף קָטָן".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZAQEF_QATAN,
                code_point_2: None,
                comment: None,            },
            ProseAccent::ZaqephGadol => HebrewAccentDetails {
                bhs_name: "Zaqeph Gadol".to_string(),
                bhs_hebrew: "זָקֵף גָּדוֹל".to_string(),
                bhs_meaning:"large upright".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZAQEF_GADOL,
                code_point_2: None,
                comment: None,            },
            ProseAccent::Revia => HebrewAccentDetails {
                bhs_name: "Revia".to_string(),
                bhs_hebrew:"רְבִיעַ".to_string(),
                bhs_meaning:"fourth [in a sequence]".to_string(),
                alt_name: Some("Ravia".to_string()),
                alt_hebrew: Some("רָבִיעַ".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
                comment: Some("probably due to its four-note tune.".to_string()),            },
            ProseAccent::Tiphcha => HebrewAccentDetails {
                bhs_name: "Tiphcha".to_string(),
                bhs_hebrew: "טִפְחָא".to_string(),
                bhs_meaning:"handbreadth or diagonal".to_string(),//OK
                alt_name: Some("Tarcha".to_string()),
                alt_hebrew: Some("טַרְחָא".to_string()),
                alt_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
                comment: None,            },
            ProseAccent::Zarqa => HebrewAccentDetails {
                bhs_name: "Zarqa".to_string(),
                bhs_hebrew: "זַרְקָא".to_string(),
                bhs_meaning:"to sprinkle, scatter".to_string(),
                alt_name: Some("Tsinnor".to_string()),
                alt_hebrew: Some("צִנּוֹר֮".to_string()),
                alt_meaning: None,
                comment: Some("In Yemeni tradition, it is called Tsinnor".to_string()),
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_ZINOR,
                code_point_2: None,
   },
            ProseAccent::Pashta => HebrewAccentDetails {
                bhs_name: "Pashta".to_string(),
                bhs_hebrew: "פַּשְׁטָא".to_string(),
                bhs_meaning:"extending, stretching out in length".to_string(),
                alt_name: Some("Qadma".to_string()),
                alt_hebrew: Some("קַדְמָא".to_string()),
                alt_meaning: Some("to progress, advance".to_string()),
                comment: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_QADMA,
                code_point_2: None,
},
            ProseAccent::Yetiv => HebrewAccentDetails {
                bhs_name: "Yetiv".to_string(),
                bhs_hebrew: "יְתִיב".to_string(),
                bhs_meaning:"resting or sitting".to_string(),
                alt_name: Some("(Shophar) yetiv".to_string()),
                alt_hebrew: Some("(שׁוֹפָר) יְתִיב".to_string()),
                alt_meaning: None,
                comment: Some("Short for Shofar (=horn) Yetiv".to_string()),
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_YETIV,
                code_point_2: None,
        },
            ProseAccent::Tevir => HebrewAccentDetails {
                bhs_name: "Tevir".to_string(),
                bhs_hebrew: "תְּבִיר".to_string(),
                bhs_meaning:"broken, downward tumble".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TEVIR,
                code_point_2: None,
            },
            ProseAccent::Geresh => HebrewAccentDetails {
                bhs_name: "Geresh".to_string(),
                bhs_hebrew: "גֵּרֵישׁ".to_string(),
                bhs_meaning:"expulsion, driving out, divorce".to_string(),
                alt_name: Some("Gerish".to_string()),
                alt_hebrew: Some("גְּרִישׁ".to_string()),
                alt_meaning: None,
                comment: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_GERESH,
                code_point_2: None,
            },
            ProseAccent::Gershayim => HebrewAccentDetails {
                bhs_name: "Gershayim".to_string(),
                bhs_hebrew: "גֵּרְשַׁיִם".to_string(),
                bhs_meaning:"double of expulsion, driving out, divorce".to_string(),
                alt_name: Some("Shene Gershayim".to_string()),
                alt_hebrew: Some("שְׁנֵי גֵּרְשַׁיִם".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_GERSHAYIM,
                code_point_2: None,
                comment: None,
            },
            ProseAccent::Pazer => HebrewAccentDetails {
                bhs_name: "Pazer".to_string(),
                bhs_hebrew: "פָּזֶר".to_string(),
                bhs_meaning:"lavish or scatter".to_string(),
                alt_name: Some("Pazer Qatan".to_string()),
                alt_hebrew: Some("פָּזֵר קָטָן".to_string()),
                alt_meaning: Some("small lavish or scatter".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_PAZER,
                code_point_2: None,
                comment: None,
            },
            ProseAccent::PazerGadol => HebrewAccentDetails {
                bhs_name: "Pazer Gadol".to_string(),
                bhs_hebrew: "פָּזֶר גּדוֹל".to_string(),
                bhs_meaning:"large lavish or scatter".to_string(),
                alt_name: Some("Qarne Pharah".to_string()),
                alt_hebrew: Some("קַרְנֵי פָרָה".to_string()),
                alt_meaning: Some("horns of a cow".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_QARNEY_PARA,
                code_point_2: None,
                comment: None,
            },
            ProseAccent::TelishaGedolah => HebrewAccentDetails {
                bhs_name: "Telisha Gedolah".to_string(),
                bhs_hebrew: "תְּלִישָׁא גְּדוֹלָה".to_string(),
                bhs_meaning:"great (long) detached".to_string(),
                alt_name: Some("Tirtsah".to_string()),
                alt_hebrew: Some("תִּרְצָה".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_TELISHA_GEDOLA,
                code_point_2: None,
                comment: None,
            },
            ProseAccent::Legarmeh => HebrewAccentDetails {
                bhs_name: "Legarmeh".to_string(),
                bhs_hebrew: "לְגַרְמֶהּ".to_string(),
                bhs_meaning:"for or by itself, independant".to_string(),
                alt_name: Some("Munnach Legarmeh".to_string()),
                alt_hebrew: Some("מוּנַח לְגַרְמֵ֣הּ".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_MUNAH,
                code_point_2: Some(CP_PASEQ),
                comment: None,
            },
            // Conjunctives
            ProseAccent::Munnach => HebrewAccentDetails {
                bhs_name: "Munnach".to_string(),
                bhs_hebrew: "מֻנַּח".to_string(),
                bhs_meaning:"resting or placed".to_string(),
                alt_name: Some("Shofar Holech".to_string()),
                alt_hebrew: Some("שׁוֹפָר הוֹלֵךְ".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MUNAH,
                code_point_2: None,
           comment: Some("because it is shaped like a horn (Shofar) lying on its side".to_string()),
                 },
            ProseAccent::Mahpakh => HebrewAccentDetails {
                bhs_name: "Mahpakh".to_string(),
                bhs_hebrew: "מַהְפַּךְ".to_string(),
                bhs_meaning:"turning round".to_string(), //OK
                alt_name: Some("(Schofar) Mehuppakh".to_string()),
                alt_hebrew: Some("שׁוֹפָר מְהֻפָּ֤ךְ".to_string()),
                alt_meaning: Some("reversed (horn)".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MAHAPAKH,
                code_point_2: None,
                comment: Some("The form is an inverted Shofar".to_string()),
            },
            ProseAccent::Merkha => HebrewAccentDetails {
                bhs_name: "Merkha".to_string(),
                bhs_hebrew: "מֵרְכָא".to_string(),
                bhs_meaning:"lengthener, prolonging".to_string(),//OK
                alt_name: Some("Ma'arich".to_string()),
                alt_hebrew: Some("מַאֲרִ֥יךְ".to_string()),
                alt_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MERKHA,
                code_point_2: None,
            comment:  Some("rod or stroke".to_string()),
                },
            ProseAccent::MerkhaKephulah => HebrewAccentDetails {
                bhs_name: "Merkha Kephulah".to_string(),
                bhs_hebrew: "מֵרְכָא כְּפוּלָה".to_string(),
                bhs_meaning:"double lengthener".to_string(),
                alt_name: Some("Tere ta’ame".to_string()),
                alt_hebrew: Some("תְּרֵי טַעֲמֵ֦י".to_string()),
                alt_meaning: None,
                comment:  Some("two rods or strokes".to_string()),
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_MERKHA_KEFULA,
                code_point_2: None,
            },
            ProseAccent::Darga => HebrewAccentDetails {
                bhs_name: "Darga".to_string(),
                bhs_hebrew: "דַּרְגָּא".to_string(),
                bhs_meaning:"stairstep".to_string(), // OK
                alt_name: Some("Dirjo".to_string()),
                alt_hebrew: Some("דִּרְגָּ֧א".to_string()),
                alt_meaning: None,
                comment: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_DARGA,
                code_point_2: None,
            },
            ProseAccent::Azla => HebrewAccentDetails { //OK
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא".to_string(),
                bhs_meaning:"going on (not pausing), depart".to_string(),
                alt_name: Some("Qadma".to_string()),
                alt_hebrew: Some("קַדְמָא".to_string()),
                alt_meaning: None,
                comment: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_QADMA,
                code_point_2: None,},
            ProseAccent::TelishaQetannah => HebrewAccentDetails {
                bhs_name: "Telisha Qetannah".to_string(),
                bhs_hebrew: "תְּלִישָא קְטַנָּה".to_string(),
                bhs_meaning:"small (short) detached".to_string(),
                alt_name: Some("Talscha".to_string()),
                alt_hebrew: Some("תַּלְשָׁא".to_string()),
                alt_meaning: None,
                comment: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TELISHA_QETANA,
                code_point_2: None,

             },
            ProseAccent::Galgal => HebrewAccentDetails {
                bhs_name: "Galgal".to_string(),
                bhs_hebrew: "גַּלְגַּל".to_string(),
                bhs_meaning:"wheel, circle".to_string(),
                alt_name: Some("Jerach Ben Jomo".to_string()),
                alt_hebrew: Some("יֵרֶח בֶּן יוֹמוֹ".to_string()),
                alt_meaning: Some("moon one day old".to_string()),
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_YERAH_BEN_YOMO,
                code_point_2: None,
            },
            ProseAccent::Mayela => HebrewAccentDetails {
                bhs_name: "Mayela".to_string(),
                bhs_hebrew: "מָאיְלָא".to_string(),
                bhs_meaning:"to be raised or elevated".to_string(),
                alt_name: Some("Meayyela".to_string()),
                alt_hebrew: Some("מְאַיְּלָא".to_string()),
                alt_meaning: None,
                comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq".to_string()),
                acc_type: AccentType::Secondary,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
            },
            ProseAccent::Meteg => HebrewAccentDetails {
                bhs_name: "Meteg".to_string(),
                bhs_hebrew: "מֶתֶג".to_string(),
                bhs_meaning:"accent or mark".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Secondary,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_METEG,
                code_point_2: None,
            },
        }
    }
}

impl PoetryAccent {
    #[allow(unused)]
    pub fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => 1,
            PoetryAccent::OleWeYored => 2,
            PoetryAccent::Atnach => 3,
            PoetryAccent::ReviaGadol => 4,
            PoetryAccent::ReviaMugrash => 5,
            PoetryAccent::ShalsheletGadol => 6,
            PoetryAccent::Tsinnor => 7,
            PoetryAccent::ReviaQaton => 8,
            PoetryAccent::Dechi => 9,
            PoetryAccent::Pazer => 10,
            PoetryAccent::MehuppakhLegarmeh => 11,
            PoetryAccent::AzlaLegarmeh => 12,
            // Conjunctives
            PoetryAccent::Munnach => 13,
            PoetryAccent::Merkha => 14,
            PoetryAccent::Illuy => 15,
            PoetryAccent::Tarkha => 16,
            PoetryAccent::Galgal => 17,
            PoetryAccent::Mehuppakh => 18,
            PoetryAccent::Azla => 19,
            PoetryAccent::ShalsheletQetannah => 20,
            PoetryAccent::TsinnoritMerkha => 21,
            PoetryAccent::TsinnoritMahpakh => 21,
            PoetryAccent::Meteg => 22,
        }
    }
    #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => HebrewAccentDetails {
                bhs_name: "Silluq".to_string(),
                bhs_hebrew: "סִלּוּק".to_string(),
                bhs_meaning:"close, cessation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SILLUQ,
                code_point_2: None,
            },
            PoetryAccent::OleWeYored => HebrewAccentDetails {
                bhs_name: "Ole We Yored".to_string(),
                bhs_hebrew: "עוֹלֶה וְיוֹרֵד".to_string(),
                bhs_meaning:"ascending and descending".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_OLE,
code_point_2: Some(CP_MAHAPAKH),
             },
            PoetryAccent::Atnach => HebrewAccentDetails {
                bhs_name: "Atnach".to_string(),
                bhs_hebrew: "אַתְנָח".to_string(),
                bhs_meaning:"pause, rest".to_string(), //OK
                alt_name: Some("Etnachta".to_string()),
                alt_hebrew: Some("אֶתְנַחְתָּא".to_string()),
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ETNAHTA,
                code_point_2: None,
            },
            PoetryAccent::ReviaGadol => HebrewAccentDetails {
                bhs_name: "Revia Gadol".to_string(),
                bhs_hebrew: "רְבִיעַ גּדוֹל".to_string(),
                bhs_meaning:"big fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
            },
            PoetryAccent::ReviaMugrash => HebrewAccentDetails {
                bhs_name: "Revia Mugrash".to_string(),
                bhs_hebrew: "רְבִיעַ מֻגְרָשׁ".to_string(),
                bhs_meaning:"exiled fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("???".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_GERESH,
code_point_2: Some(CP_REVIA),
                        },
            PoetryAccent::ShalsheletGadol => HebrewAccentDetails {
                bhs_name: "Shalshelet Gadol".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת גָּדוֹל".to_string(),
                bhs_meaning:"large chain or link".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SHALSHELET,
code_point_2: Some(CP_PASEQ),
                    },
            PoetryAccent::Tsinnor => HebrewAccentDetails {
                bhs_name: "Tsinnor".to_string(),
                bhs_hebrew: "צִנּוֹר".to_string(),
                bhs_meaning:"pipe or tube".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZINOR,
                code_point_2: None,
 },
            PoetryAccent::ReviaQaton => HebrewAccentDetails {
                bhs_name: "Revia Qaton".to_string(),
                bhs_hebrew: "רְבִיעַ קָטוֹן".to_string(),
                bhs_meaning:"small fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
            },
            PoetryAccent::Dechi => HebrewAccentDetails {
                bhs_name: "Dechi".to_string(),
                bhs_hebrew: "דֶּחִי".to_string(),
                bhs_meaning:"to push or drive away".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_DEHI,
                code_point_2: None,
 },
            PoetryAccent::Pazer => HebrewAccentDetails {
                bhs_name: "Pazer".to_string(),
                bhs_hebrew: "פָּזֵר".to_string(),
                bhs_meaning:"lavish or scatter".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_PAZER,
                code_point_2: None,
            },
            PoetryAccent::MehuppakhLegarmeh => HebrewAccentDetails {
                bhs_name: "Mehuppakh Legarmeh".to_string(),
                bhs_hebrew: "מְהֻפָּךְ לְגַרְמֵהּ".to_string(),
                bhs_meaning:"reversed to its own".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_MAHAPAKH,
code_point_2: Some(CP_PASEQ),
               },
            PoetryAccent::AzlaLegarmeh => HebrewAccentDetails {
                bhs_name: "Azla Legarmeh".to_string(),
                bhs_hebrew: "אַזְלָא לְגַרְמֶהּ".to_string(),
                bhs_meaning:"goes to its own".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_QADMA,
code_point_2: Some(CP_PASEQ),
          },
            // Conjunctives
            PoetryAccent::Munnach => HebrewAccentDetails {
                bhs_name: "Munnach".to_string(),
                bhs_hebrew: "מֻנַּח".to_string(),
                bhs_meaning:"rest or placed".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MUNAH,
                code_point_2: None,
            },
            PoetryAccent::Merkha => HebrewAccentDetails {
                bhs_name: "Merkha".to_string(),
                bhs_hebrew: "מֵרְכָא".to_string(),
                bhs_meaning:"lengthener".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MERKHA,
                code_point_2: None,
            },
            PoetryAccent::Illuy => HebrewAccentDetails {
                bhs_name: "Illuy".to_string(),
                bhs_hebrew: "עִלּוּי".to_string(),
                bhs_meaning:"elevation or raising".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ILUY,
                code_point_2: None,
            },
            PoetryAccent::Tarkha => HebrewAccentDetails {
                bhs_name: "Tarkha".to_string(),
                bhs_hebrew: "טַרְחָא".to_string(),
                bhs_meaning:"to be drawn out or to be extended".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
            },
            PoetryAccent::Galgal => HebrewAccentDetails {
                bhs_name: "Galgal".to_string(),
                bhs_hebrew: "גַּלְגַּל".to_string(),
                bhs_meaning:"circle".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_YERAH_BEN_YOMO,
                code_point_2: None,
            },
            PoetryAccent::Mehuppakh => HebrewAccentDetails {
                bhs_name: "Mehuppakh".to_string(),
                bhs_hebrew: "מְהֻפַּך".to_string(),
                bhs_meaning:"reversed or turned around".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MAHAPAKH,
                code_point_2: None,
            },
            PoetryAccent::Azla => HebrewAccentDetails {
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא֙".to_string(),
                bhs_meaning:"going away".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_QADMA,
                code_point_2: None,
            },
            PoetryAccent::ShalsheletQetannah => HebrewAccentDetails {
                bhs_name: "Shalshelet Qetannah".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת קְטַנָּה".to_string(),
                bhs_meaning:"small chain".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_SHALSHELET,
                code_point_2: None,
            },
            PoetryAccent::TsinnoritMerkha => HebrewAccentDetails {
                bhs_name: "Tsinnorit Merkha".to_string(),
                bhs_hebrew: "צִנּוֹרִת מֵרְכָא".to_string(),
                bhs_meaning:"pipe of continuation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Merkha Metsunneret".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ZARQA,
code_point_2: Some(CP_MERKHA),
          },
            PoetryAccent::TsinnoritMahpakh => HebrewAccentDetails {
                bhs_name: "Tsinnorit Mahpakh".to_string(),
                bhs_hebrew: "צִנּוֹרִת מַהְפַּךְ".to_string(),
                bhs_meaning:"pipe of reversal".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Mahpakh Metsunnar".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ZARQA,
code_point_2: Some(CP_MAHAPAKH),
         },
            PoetryAccent::Meteg => HebrewAccentDetails {
                bhs_name: "Meteg".to_string(),
                bhs_hebrew: "מֶתֶג".to_string(),
                bhs_meaning:"accent or mark".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Mahpakh Metsunnar".to_string()),
                acc_type: AccentType::Secondary,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_METEG,
                code_point_2: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silluq() {
        let pa = ProseAccent::Silluq;
        let pa_silluq_ord = pa.rank();
        assert_eq!(1, pa_silluq_ord);
        // let _details = a.details();
    }
    #[test]
    fn no_test_just_print_details() {
        println!("\n{:#?}", ProseAccent::Silluq.details());
        println!("\n{:#?}", ProseAccent::Atnach.details());
        println!("\n{:#?}", ProseAccent::Segolta.details());
        println!("\n{:#?}", ProseAccent::Shalshelet.details());
        println!("\n{:#?}", ProseAccent::ZaqephQaton.details());
        println!("\n{:#?}", ProseAccent::ZaqephGadol.details());
        println!("\n{:#?}", ProseAccent::Revia.details());
        println!("\n{:#?}", ProseAccent::Tiphcha.details());
        println!("\n{:#?}", ProseAccent::Zarqa.details());
        println!("\n{:#?}", ProseAccent::Pashta.details());
        println!("\n{:#?}", ProseAccent::Yetiv.details());
        println!("\n{:#?}", ProseAccent::Tevir.details());
        println!("\n{:#?}", ProseAccent::Geresh.details());
        println!("\n{:#?}", ProseAccent::Gershayim.details());
        println!("\n{:#?}", ProseAccent::Pazer.details());
        println!("\n{:#?}", ProseAccent::PazerGadol.details());
        println!("\n{:#?}", ProseAccent::TelishaGedolah.details());
        println!("\n{:#?}", ProseAccent::Legarmeh.details());
        // Conjunctives
        println!("\n{:#?}", ProseAccent::Munnach.details());
        println!("\n{:#?}", ProseAccent::Mahpakh.details());
        println!("\n{:#?}", ProseAccent::Merkha.details());
        println!("\n{:#?}", ProseAccent::MerkhaKephulah.details());
        println!("\n{:#?}", ProseAccent::Darga.details());
        println!("\n{:#?}", ProseAccent::Azla.details());
        println!("\n{:#?}", ProseAccent::TelishaQetannah.details());
        println!("\n{:#?}", ProseAccent::Galgal.details());
        println!("\n{:#?}", ProseAccent::Mayela.details());
    }
    #[test]
    fn no_test_just_print_rank() {
        println!("\n{:#?}", ProseAccent::Silluq.rank());
        println!("\n{:#?}", ProseAccent::Atnach.rank());
        println!("\n{:#?}", ProseAccent::Segolta.rank());
        println!("\n{:#?}", ProseAccent::Shalshelet.rank());
        println!("\n{:#?}", ProseAccent::ZaqephQaton.rank());
        println!("\n{:#?}", ProseAccent::ZaqephGadol.rank());
        println!("\n{:#?}", ProseAccent::Revia.rank());
        println!("\n{:#?}", ProseAccent::Tiphcha.rank());
        println!("\n{:#?}", ProseAccent::Zarqa.rank());
        println!("\n{:#?}", ProseAccent::Pashta.rank());
        println!("\n{:#?}", ProseAccent::Yetiv.rank());
        println!("\n{:#?}", ProseAccent::Tevir.rank());
        println!("\n{:#?}", ProseAccent::Geresh.rank());
        println!("\n{:#?}", ProseAccent::Gershayim.rank());
        println!("\n{:#?}", ProseAccent::Pazer.rank());
        println!("\n{:#?}", ProseAccent::PazerGadol.rank());
        println!("\n{:#?}", ProseAccent::TelishaGedolah.rank());
        println!("\n{:#?}", ProseAccent::Legarmeh.rank());
        // Conjunctives
        println!("\n{:#?}", ProseAccent::Munnach.rank());
        println!("\n{:#?}", ProseAccent::Mahpakh.rank());
        println!("\n{:#?}", ProseAccent::Merkha.rank());
        println!("\n{:#?}", ProseAccent::MerkhaKephulah.rank());
        println!("\n{:#?}", ProseAccent::Darga.rank());
        println!("\n{:#?}", ProseAccent::Azla.rank());
        println!("\n{:#?}", ProseAccent::TelishaQetannah.rank());
        println!("\n{:#?}", ProseAccent::Galgal.rank());
        println!("\n{:#?}", ProseAccent::Mayela.rank());
    }

    #[test]
    fn test_accent_details() {
        let pa = ProseAccent::Galgal;
        assert_eq!("wheel, circle".to_string(), pa.details().bhs_meaning);
    }
}
