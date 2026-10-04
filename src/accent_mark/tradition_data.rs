//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://www.unicode.org/Public/18.0.0/charts/PDF/U0590.pdf>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

use crate::accent_mark::tradition::{AccentName, TraditionNames};

// ============================================================================
// COMPOUND ACCENT NOT YET IMPLEMENTED
// ============================================================================
pub(crate) const TRADITION_NAMES_SHENE_PASHTIN: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "שְׁנֵי פַּשְׁטִין",
        sbl_academic: "šənê pašṭîn",
        english_name: "Shene Pashtin / Pashtayim",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּרֵי קַדְמִין",
        sbl_academic: "Tərê Qadmîn",
        english_name: "Tere Qadmin",
    }),
    italian: Some(AccentName {
        hebrew_name: "(שְׁנֵי) פַּשְׁטִין",
        sbl_academic: "(šənê) pašṭîn",
        english_name: "(Shene) Pashtin",
    }),
    yemenite: None,
};

// ============================================================================
// COMPOUND ACCENTS
// ============================================================================
/*
SHALSHELET_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "šalšelet",
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
        sbl_academic: "munnaḥ ləgarmēh",
        english_name: "munach legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "פָּסֵק",
        sbl_academic: "pāsēq",
        english_name: "paseq",
    }),
    italian: Some(AccentName {
        hebrew_name: "לְגַרְמֵה",
        sbl_academic: "ləgarmēh",
        english_name: "legarmeh",
    }),
    yemenite: None, // "Not available from accessible sources",
};
/*
OLEH_WEYORED_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "ʿôleh wəyôrēd",
    english_name: "Oleh Weyored",
    hebrew_name: "עוֹלֶה וְיוֹרֵד",
    hebrew_concept: "ascending and descending",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_OLE,
        secondary_mark: Some(&CODEPOINT_MERKHA),
*/
pub(crate) const TRADITION_NAMES_OLEH_WEYORED: TraditionNames =
    TraditionNames::uniform("עֹלֶה וְיֹרֵד", "ʿôleh wəyôrēd", "ascending and descending");

/* REVIA_MUGRASH_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "rəbîaʿ mugrāš",
    english_name: "Revia Mugrash",
    hebrew_name: "רְבִיעַ מֻגְרָשׁ",
    hebrew_concept: "exiled fourth",
    cantillation_symbol: CantillationSymbol {
        primary_mark: &CODEPOINT_GERESH,
        secondary_mark: Some(&CODEPOINT_REVIA),
*/

/*
SHALSHELET_GADOL_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "šalšelet gādôl",
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
        sbl_academic: "səgôl",
        english_name: "segol",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "סְגוֹלְתָּא֒",
        sbl_academic: "səgôlətāʾ",
        english_name: "segolta",
    }),
    italian: Some(AccentName {
        hebrew_name: "שְׁרֵי֒",
        sbl_academic: "šərê",
        english_name: "shere",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
MEHUPPAKH_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "məhuppāk ləgarmēh",
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
        sbl_academic: "səmahpāk ləgarmēh / mehûppāk",
        english_name: "Mahpach Legarmeh / Mehuppakh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "מַהְפָּךְ",
        sbl_academic: "mahpāk",
        english_name: "Mahpakh (Legarmeh implied)",
    }),
    italian: Some(AccentName {
        hebrew_name: "מַהְפָּךְ",
        sbl_academic: "mahpāk",
        english_name: "Mahpakh (Legarmeh implied)",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מַהְפָּךְ / מֶהוּפָּךְ",
        sbl_academic: "mahpāk / mehûppāk",
        english_name: "Mahpakh / Mehuppakh",
    }),
};

/*
AZLA_LEGARMEH_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "ʾazlāʾ ləgarmeh",
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
        sbl_academic: "qadmāʾ ləgarmēh",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        sbl_academic: "ʾazlāʾ ləgarmēh",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        sbl_academic: "qadmāʾ ləgarmēh",
        english_name: "Qadma Legarmeh",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
TSINNORIT_MERKHA_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "ṣinnôrīt mērəkāʾ",
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
        sbl_academic: "qadmāʾ ləgarmēh",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        sbl_academic: "ʾazlāʾ ləgarmēh",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        sbl_academic: "qadmāʾ ləgarmēh",
        english_name: "Qadma Legarmeh",
    }),
    yemenite: None, // Not available from accessible sources
};

/*
TSINNORIT_MAHPAKH_INFO: AccentMetaData = AccentMetaData {
    sbl_academic: "ṣinnôrīt mahpak",
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
        sbl_academic: "qadmāʾ ləgarmēh",
        english_name: "Qadma Legarmeh",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָא לְגַרְמֵה",
        sbl_academic: "ʾazlāʾ ləgarmēh",
        english_name: "Azla Legarmeh",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָא לְגַרְמֵה",
        sbl_academic: "qadmāʾ ləgarmēh",
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
        sbl_academic: "ʾetnaḥtāʾ",
        english_name: "Etnachta",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַתְנָ֑ח",
        sbl_academic: "ʾatnāḥ",
        english_name: "Atnach",
    }),
    italian: Some(AccentName {
        hebrew_name: "אַתְנָ֑ח",
        sbl_academic: "ʾatnāḥ",
        english_name: "Atnach",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "אֶתְנָחָ֑א",
        sbl_academic: "ʾetnaḥtāʾ",
        english_name: "Etnacha",
    }),
};

// ============================================================================
// SEGOL (U+0592) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_SEGOL: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "סְגוֹל֒",
        sbl_academic: "səgôl",
        english_name: "Segol",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "סְגוֹלְתָּא֒",
        sbl_academic: "səgôlətāʾ",
        english_name: "Segolta",
    }),
    italian: Some(AccentName {
        hebrew_name: "שְׁרֵי֒",
        sbl_academic: "šərê",
        english_name: "Shere",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// SHALSHELET (U+0593)
// ============================================================================
pub(crate) const TRADITION_NAMES_SHALSHELET: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        sbl_academic: "šalšelet",
        english_name: "Shalshelet",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        sbl_academic: "šalšelet",
        english_name: "Shalshelet",
    }),
    italian: Some(AccentName {
        hebrew_name: "שַׁלְשֶׁ֓לֶת",
        sbl_academic: "šalšelet",
        english_name: "Shalshelet",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "שִׁישְׁלָ֓א",
        sbl_academic: "šîšəlāʾ",
        english_name: "Shishla",
    }),
};

// ============================================================================
// ZAQEF QATAN (U+0594)
// ============================================================================
pub(crate) const TRADITION_NAMES_ZAQEF_QATAN: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "זָקֵף קָטָ֔ן",
        sbl_academic: "zāqēp qāṭān",
        english_name: "zaqeph qatan",
    }),
    ..TraditionNames::uniform("זָקֵף קָט֔וֹן", "zāqēp qāṭôn", "Zaqeph Qaton")
};

// ============================================================================
// ZAQEF GADOL (U+0595) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_ZAQEF_GADOL: TraditionNames =
    TraditionNames::uniform("זָקֵף גָּד֕וֹל", "zāqēp gādôl", "zaqeph gadol");

// ============================================================================
// TIPEHA (U+0596)
// ============================================================================
pub(crate) const TRADITION_NAMES_TIPEHA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "טִפְחָ֖א",
        sbl_academic: "ṭipḥāʾ",
        english_name: "Tiphcha",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "טַרְחָ֖א",
        sbl_academic: "ṭarḥāʾ",
        english_name: "Tarcha",
    }),
    italian: Some(AccentName {
        hebrew_name: "טַרְחָ֖א",
        sbl_academic: "ṭarḥāʾ",
        english_name: "Tarcha",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "נְטוּיָ֖ה",
        sbl_academic: "nəṭûyâ",
        english_name: "Netuyah",
    }),
};

// ============================================================================
// REVIA (U+0597) - Ashkenazi differs slightly
// ============================================================================
pub(crate) const TRADITION_NAMES_REVIA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "רְבִ֗יע",
        sbl_academic: "rəbîʿ",
        english_name: "revia/revi'i",
    }),
    ..TraditionNames::uniform("רְבִ֗יע", "rəbîʿ", "revia")
};

// ============================================================================
// ZARQA (U+0598)
// ============================================================================
pub(crate) const TRADITION_NAMES_ZARQA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        sbl_academic: "zarqāʾ",
        english_name: "Zarqa",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        sbl_academic: "zarqāʾ",
        english_name: "Zarqa",
    }),
    italian: Some(AccentName {
        hebrew_name: "זַרְקָא֘",
        sbl_academic: "zarqāʾ",
        english_name: "Zarqa",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "צִנּוֹר֘",
        sbl_academic: "ṣinnôr",
        english_name: "Tsinnor",
    }),
};

// ============================================================================
// PASHTA (U+0599)
// ============================================================================
pub(crate) const TRADITION_NAMES_PASHTA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "פַּשְׁטָא֙",
        sbl_academic: "pašṭāʾ",
        english_name: "Pashta",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "קַדְמָא֙",
        sbl_academic: "qadmāʾ",
        english_name: "Qadma",
    }),
    italian: Some(AccentName {
        hebrew_name: "פַּשְׁטָא֙",
        sbl_academic: "pašṭāʾ",
        english_name: "Pashta",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "אַזְלָא֙",
        sbl_academic: "ʾazlāʾ",
        english_name: "Azla",
    }),
};

// ============================================================================
// YETIV (U+059A)
// ============================================================================
pub(crate) const TRADITION_NAMES_YETIV: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        sbl_academic: "yətîb",
        english_name: "Yetiv",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        sbl_academic: "yətîb",
        english_name: "Yetiv",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁ֚וֹפָר יְתִיב",
        sbl_academic: "šôpār yətîb",
        english_name: "Shophar Yetiv",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "יְ֚תִיב",
        sbl_academic: "yətîb",
        english_name: "Yetiv",
    }),
};

// ============================================================================
// TEVIR (U+059B)
// ============================================================================
pub(crate) const TRADITION_NAMES_TEVIR: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        sbl_academic: "təbîr",
        english_name: "Tevir",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        sbl_academic: "təbîr",
        english_name: "Tevir",
    }),
    italian: Some(AccentName {
        hebrew_name: "תְּבִ֛יר",
        sbl_academic: "təbîr",
        english_name: "Tevir",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תַּבְרָ֛א",
        sbl_academic: "tabrāʾ",
        english_name: "Tavra",
    }),
};

// ============================================================================
// GERESH (U+059C)
// ============================================================================
pub(crate) const TRADITION_NAMES_GERESH: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "גֵּ֜רֵשׁ",
        sbl_academic: "gērēš",
        english_name: "geresh/azla",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "גְּרִ֜ישׁ",
        sbl_academic: "gərîš",
        english_name: "gerish",
    }),
    italian: Some(AccentName {
        hebrew_name: "גֵּ֜רֵשׁ",
        sbl_academic: "gērēš",
        english_name: "geresh/azla",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "טָרֵ֜ס",
        sbl_academic: "ṭārēs",
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
        sbl_academic: "gērəšayim",
        english_name: "gershayim",
    }),
    ..TraditionNames::uniform("שְׁנֵי גְרִישִׁ֞ין", "šənê gərîšîn", "shene gerishin")
};

// ============================================================================
// QARNEY PARA (U+059F) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_QARNEY_PARA: TraditionNames =
    TraditionNames::uniform("קַרְנֵי פָרָ֟ה", "qarnê pārâ", "qarne pharah");

// ============================================================================
// TELISHA GEDOLA (U+05A0)
// ============================================================================
pub(crate) const TRADITION_NAMES_TELISHA_GEDOLA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
        sbl_academic: "təlîšāʾ gədôlâ",
        english_name: "telisha gedolah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תִּ֠רְצָה",
        sbl_academic: "tīrṣâ",
        english_name: "tirtsah",
    }),
    italian: Some(AccentName {
        hebrew_name: "תַּ֠לְשָׁא",
        sbl_academic: "talšāʾ",
        english_name: "talsha",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תְּ֠לִישָא גְדוֹלָה",
        sbl_academic: "təlîšāʾ gədôlâ",
        english_name: "telisha gedolah",
    }),
};

// ============================================================================
// PAZER (U+05A1) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_PAZER: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "פָּזֵ֡ר",
        sbl_academic: "pāzēr",
        english_name: "pazer",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "פָּזֵר גָּד֡וֹל",
        sbl_academic: "pāzēr gādôl",
        english_name: "pazer gadol",
    }),
    italian: Some(AccentName {
        hebrew_name: "פָּזֵר גָּד֡וֹל",
        sbl_academic: "pāzēr gādôl",
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
        sbl_academic: "mûnaḥ",
        english_name: "munach",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שׁוֹפָר הוֹלֵ֣ךְ",
        sbl_academic: "šôpār hôlēk",
        english_name: "shophar holech",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁוֹפָר עִלּ֣וּי",
        sbl_academic: "šôpār ʿillûy",
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
        sbl_academic: "mahpak",
        english_name: "Mahpach",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "שׁוֹפָר) מְהֻפָּ֤ךְ)",
        sbl_academic: "(šôpār) məhupāk",
        english_name: "(Shophar) Mehuppakh",
    }),
    italian: Some(AccentName {
        hebrew_name: "שׁוֹפָר הָפ֤וּךְ",
        sbl_academic: "šôpār hāpûk",
        english_name: "Shophar Haphuch",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מְהֻפָּ֤ךְ",
        sbl_academic: "məhuppāk",
        english_name: "Mehuppakh",
    }),
};

// ============================================================================
// MERKHA (U+05A5)
// ============================================================================
pub(crate) const TRADITION_NAMES_MERKHA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֵרְכָ֥א",
        sbl_academic: "mērəkāʾ",
        english_name: "Mercha",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "מַאֲרִ֥יךְ",
        sbl_academic: "maʾărîk",
        english_name: "Maarich",
    }),
    italian: Some(AccentName {
        hebrew_name: "מַאֲרִ֥יךְ",
        sbl_academic: "maʾărîk",
        english_name: "Maarich",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "מַאֲרְכָ֥א",
        sbl_academic: "maʾărkāʾ",
        english_name: "Maarcha",
    }),
};

// ============================================================================
// MERKHA KEFULA (U+05A6) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_MERKHA_KEFULA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "מֵרְכָא כּפוּלָ֦ה",
        sbl_academic: "mērəkāʾ kpûlâ",
        english_name: "Mercha Kephulah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תְּרֵי טַעֲמֵ֦י",
        sbl_academic: "tərê ṭaʿămê",
        english_name: "Tere Taame",
    }),
    italian: Some(AccentName {
        hebrew_name: "תְּרֵין חוּטְרִ֦ין",
        sbl_academic: "tərên ḥûṭərîn",
        english_name: "Teren Chutrin",
    }),
    yemenite: None, // Not present in Yemenite tradition
};

// ============================================================================
// DARGA (U+05A7) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_DARGA: TraditionNames =
    TraditionNames::uniform("דַּרְגָּ֧א", "dargāʾ", "darga");

// ============================================================================
// QADMA (U+05A8) - Only 3 traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_QADMA: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "קַדְמָ֨א",
        sbl_academic: "qadmāʾ",
        english_name: "qadma",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "אַזְלָ֨א",
        sbl_academic: "ʾazlāʾ",
        english_name: "azla",
    }),
    italian: Some(AccentName {
        hebrew_name: "קַדְמָ֨א",
        sbl_academic: "qadmāʾ",
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
        sbl_academic: "təlîšāʾ qəṭannâ",
        english_name: "telisha qetannah",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "תַּלְשָׁא֩",
        sbl_academic: "talšāʾ",
        english_name: "talsha",
    }),
    italian: Some(AccentName {
        hebrew_name: "תַּרְסָא֩",
        sbl_academic: "tarsāʾ",
        english_name: "tarsa",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "תְּלִישָא קְטַנָּה֩",
        sbl_academic: "təlîšāʾ qəṭannâ",
        english_name: "telisha qetannah",
    }),
};

// ============================================================================
// YERAH BEN YOMO (U+05AA) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_YERAH_BEN_YOMO: TraditionNames =
    TraditionNames::uniform("יֵרֶח בֶּן יוֹמ֪וֹ", "yēreḥ ben yômô", "yerach ben yomo");

// ============================================================================
// OLE (U+05AB) - All identical
// ============================================================================
// ///pub(crate) const TRADITION_NAMES_OLE: TraditionNames = TraditionNames::uniform("עוֹלֶה","ʿôleh", "oleh");

// ============================================================================
// ILUY (U+05AC) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_ILUY: TraditionNames =
    TraditionNames::uniform("עִלוּי", "ʿilûy", "iluy");

// ============================================================================
// DEHI (U+05AD) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_DEHI: TraditionNames =
    TraditionNames::uniform("דֶּחִי֭", "deḥî", "dechi");

// ============================================================================
// ZINOR (U+05AE) - All identical
// ============================================================================
pub(crate) const TRADITION_NAMES_ZINOR: TraditionNames =
    TraditionNames::uniform("צנור", "ṣinnôr", "tsinor (zarqa above left)");

// ============================================================================
// SILLUQ (U+05BD) - All identical (same codepoint as Meteg, different semantics)
// ============================================================================
pub(crate) const TRADITION_NAMES_SILLUQ: TraditionNames =
    TraditionNames::uniform("סוֹף פָּסֽוּק", "sôp pāsûq", "sof pasuq");

// ============================================================================
// METEG (U+05BD) - All identical (shares codepoint with Silluq)
// ============================================================================
pub(crate) const TRADITION_NAMES_METEG: TraditionNames =
    TraditionNames::uniform("מֶתֶג", "meteg", "meteg");

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
pub(crate) const TRADITION_NAMES_PASEQ: TraditionNames =
    TraditionNames::uniform("פָּסֵק", "pāsēq", "paseq");

// ============================================================================
// SOPH PASUQ (U+05C3) - No traditions
// ============================================================================
pub(crate) const TRADITION_NAMES_SOPH_PASUQ: TraditionNames = TraditionNames {
    ashkenazi: Some(AccentName {
        hebrew_name: "סוֹף פָּסֽוּק / סִלּֽוּק",
        sbl_academic: "todo",
        english_name: "Sof Pasuq / Silluq",
    }),
    sephardi: Some(AccentName {
        hebrew_name: "סוֹף פָּסֽוּק",
        sbl_academic: "todo",
        english_name: "Sof pasuq",
    }),
    italian: Some(AccentName {
        hebrew_name: "סוֹף פָּסֽוּק",
        sbl_academic: "todo",
        english_name: "Sof pasuq",
    }),
    yemenite: Some(AccentName {
        hebrew_name: "סִלּֽוּק",
        sbl_academic: "todo",
        english_name: "Silluq",
    }),
};
