use crate::constants::*;

/// Hebrew Accent, either Prose or Poetry
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
}

/// All variants of the Hebrew Prose Accents
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
    Munach,
    Mahpakh,
    Merkha,
    MerkhaKephulah,
    Darga,
    Azla,
    TelishaQetannah,
    Galgal,
    Mayela,
    Meteg,
    Maqqeph,
}

/// All variants of the Hebrew Poetry Accents
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum PoetryAccent {
    // Disjunctives
    #[default]
    Silluq,
    OlehWeYored,
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
    Munach,
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
    Maqqeph,
}

/// (non)technical details of a Hebrew Accent like category, type, UTF8 Unicode code-point(s etc.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HebrewAccentDetails {
    english_name: &'static str,
    hebrew_name: &'static str,
    meaning: &'static str,
    alt_english_name: Option<&'static str>,
    alt_hebrew_name: Option<&'static str>,
    alt_meaning: Option<&'static str>,
    first_code_point: Utf8CodePointInfo,
    second_code_point: Option<Utf8CodePointInfo>,
    comment: Option<&'static str>,
}

/// Details on a specific UTF8 Unicode code-point
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Utf8CodePointInfo {
    pub code_point: &'static str,
    pub hex_value: &'static str,
    pub name: &'static str,
    pub symbol: &'static str,
    pub position: CodePointPosition,
    pub ashkenazi: Option<Tradition>,
    pub sephardi: Option<Tradition>,
    pub italian: Option<Tradition>,
    pub yemenite: Option<Tradition>,
}
/// Names according one of four Hebrew Traditions
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

/// Hebrew Accent category (either Conjunctive or Disjunctive)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentCategory {
    Conjunctive,
    #[default]
    Disjunctive,
}

/// Hebrew Accent types
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentType {
    #[default]
    Primary,
    // used for Meayla and Meteg
    Secondary,
    // used for Maqqeph
    None,
}

/// Accent position, Indicating the location of the accent in relation to the consonant
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum CodePointPosition {
    #[default]
    Above,
    // above and after the consonant
    AbovePostPositive,
    // above and before the consonant
    AbovePrePositive,
    // at the end of the word (used for Paseq, Soph Pasuq and Maqqeph)
    End,
    Under,
    // under and after the consonant
    UnderPrePositive,
}

impl HebrewAccent {
    pub fn details(&self) -> HebrewAccentDetails {
        match self {
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => HebrewAccentDetails {
                english_name: "Silluq",
                hebrew_name: "סִלּוּק",
                meaning:"close, cessation",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_SILLUQ,
                second_code_point: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse."),
            },
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => HebrewAccentDetails {
                english_name: "Atnach",
                hebrew_name: "אתְנָח",
                meaning: "a causing to rest",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ETNAHTA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Segolta)  => HebrewAccentDetails {
                english_name: "Segolta",
                hebrew_name: "סְגֹולְתָּא",
                meaning: "a little grape-bunch",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_SEGOL,
                second_code_point: None,
                comment: None,
                },
            HebrewAccent::Prose(ProseAccent::Shalshelet)  => HebrewAccentDetails {
                english_name: "Shalshelet",
                hebrew_name: "שַׁלְשֶׁלֶת",
                meaning: "chain or link", 
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_SHALSHELET,
                second_code_point: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::ZaqephQaton)  =>  HebrewAccentDetails {
                english_name: "Zaqeph Qaton",
                hebrew_name: "זָקֵף קָטוֹן",
                meaning:"small upright", 
                alt_english_name: Some("Zaqeph Qatan"),
                alt_hebrew_name: Some("זָקֵף קָטָן"),
                alt_meaning: None,
                first_code_point: CP_ZAQEF_QATAN,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) => HebrewAccentDetails {
                english_name: "Zaqeph Gadol",
                hebrew_name: "זָקֵף גָּדוֹל",
                meaning:"large upright",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ZAQEF_GADOL,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Revia)  => HebrewAccentDetails {
                english_name: "Revia",
                hebrew_name:"רְבִיעַ",
                meaning:"fourth [in a sequence]",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_REVIA,
                second_code_point: None,
                comment: Some("probably due to its four-note tune."),
            },
            HebrewAccent::Prose(ProseAccent::Tiphcha)  => HebrewAccentDetails {
                english_name: "Tiphcha",
                hebrew_name: "טִפְחָא",
                meaning:"handbreadth or diagonal",
                alt_english_name: None, // ADD TARCHA
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_TIPEHA,
                second_code_point: None,
                comment: Some("before Atnach and Silluq"),
            },
            HebrewAccent::Prose(ProseAccent::Zarqa) => HebrewAccentDetails {
                english_name: "Zarqa",
                hebrew_name: "זַרְקָא",
                meaning: "to sprinkle, scatter",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ZINOR,
                second_code_point: None,
                comment: Some("Ibefore  Segolta"),
            },
            HebrewAccent::Prose(ProseAccent::Pashta) => HebrewAccentDetails {
                english_name: "Pashta",
                hebrew_name: "פַּשְׁטָא",
                meaning:"extending, stretching out in length",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_PASHTA,
                second_code_point: None,
                comment: Some("if you sound almost last (2 pasta’s in one word)"),
            },
            HebrewAccent::Prose(ProseAccent::Yetiv) => HebrewAccentDetails {
                english_name: "Yetiv",
                hebrew_name: "יְתִיב",
                meaning:"resting or sitting",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_YETIV,
                second_code_point: None,
                comment: Some("occasionally for a Pashta"),
            },
            HebrewAccent::Prose(ProseAccent::Tevir) => HebrewAccentDetails {
                english_name: "Tevir",
                hebrew_name: "תְּבִיר",
                meaning: "broken, downward tumble",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_TEVIR,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Geresh) => HebrewAccentDetails {
                english_name: "Geresh",
                hebrew_name: "גֵּרֵישׁ",
                meaning:"expulsion, driving out, divorce",
                alt_english_name: Some("Teres"),
                alt_hebrew_name: Some("טֶרֶס"),
                alt_meaning: None,
                first_code_point: CP_GERESH,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Gershayim) => HebrewAccentDetails {
                english_name: "Gershayim",
                hebrew_name: "גֵּרְשַׁיִם",
                meaning:"double of expulsion, driving out, divorce",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_GERSHAYIM,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => HebrewAccentDetails {
                english_name: "Pazer",
                hebrew_name: "פָּזֶר",
                meaning:"lavish or scatter",
                alt_english_name: Some("Pazer Qatan"),
                alt_hebrew_name: Some("פָּזֵר קָטָן"),
                alt_meaning: Some("small lavish or scatter"),
                first_code_point: CP_PAZER,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::PazerGadol) => HebrewAccentDetails {
                english_name: "Pazer Gadol",
                hebrew_name: "פָּזֶר גּדוֹל",
                meaning:"large lavish or scatter",
                alt_english_name: Some("Qarne Pharah"),
                alt_hebrew_name: Some("קַרְנֵי פָרָה"),
                alt_meaning: Some("horns of a cow"),
                first_code_point: CP_QARNEY_PARA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)=> HebrewAccentDetails {
                english_name: "Telisha Gedolah",
                hebrew_name: "תְּלִישָׁא גְּדוֹלָה",
                meaning:"great (long) detached",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_TELISHA_GEDOLA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Legarmeh)=> HebrewAccentDetails {
                english_name: "Legarmeh",
                hebrew_name: "לְגַרְמֶהּ",
                meaning:"for or by itself, independant",
                alt_english_name: Some("Munach Legarmeh"),
                alt_hebrew_name: Some("מוּנַח לְגַרְמֵ֣הּ"),
                alt_meaning: None,
                first_code_point: CP_MUNAH,
                second_code_point: Some(CP_PASEQ),
                comment:Some("Munach with passeq; Before Revia"),
            },
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) | HebrewAccent::Poetry(PoetryAccent::Munach) => HebrewAccentDetails {
                english_name: "Munach",
                hebrew_name: "מוּנַ֣ח",
                meaning:"resting or placed",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_MUNAH,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Mahpakh) | HebrewAccent::Poetry(PoetryAccent::Mehuppakh)=> HebrewAccentDetails {
                english_name: "Mahpakh",
                hebrew_name: "מַהְפַּךְ",
                meaning:"turning round",
                alt_english_name: Some("Mehuppakh"),
                alt_hebrew_name: Some("מְהֻפָּ֤ךְ"),
                alt_meaning: Some("reversed"),
                first_code_point: CP_MAHAPAKH,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Merkha) | HebrewAccent::Poetry(PoetryAccent::Merkha) => HebrewAccentDetails {
                english_name: "Merkha",
                hebrew_name: "מֵרְכָא",
                meaning:"lengthener, prolonging",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_MERKHA,
                second_code_point: None,
                comment:  None,
            },
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)=> HebrewAccentDetails {
                english_name: "Merkha Kephulah",
                hebrew_name: "מֵרְכָא כְּפוּלָה",
                meaning:"double lengthener",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_MERKHA_KEFULA,
                second_code_point: None,
                comment: Some("Merkha duplex"),
            },
            HebrewAccent::Prose(ProseAccent::Darga) => HebrewAccentDetails {
                english_name: "Darga",
                hebrew_name: "דַּרְגָּא",
                meaning:"stairstep", 
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_DARGA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Azla) => HebrewAccentDetails {
                english_name: "Azla",
                hebrew_name: "אַזְלָא",
                meaning:"going on (not pausing), depart",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_QADMA,
                second_code_point: None,
                comment: Some("When Geresh: Qadma"),
            },
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)=> HebrewAccentDetails {
                english_name: "Telisha Qetannah",
                hebrew_name: "תְּלִישָא קְטַנָּה",
                meaning:"small (short) detached",
                alt_english_name:  None,
                alt_hebrew_name:  None,
                alt_meaning: None,
                first_code_point: CP_TELISHA_QETANA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => HebrewAccentDetails {
                english_name: "Galgal",
                hebrew_name: "גַּלְגַּל",
                meaning:"wheel, circle",
                alt_english_name: Some("Jerach Ben Jomo"),
                alt_hebrew_name: Some("יֵרֶח בֶּן יוֹמוֹ"),
                alt_meaning: Some("moon one day old"),
                first_code_point: CP_YERAH_BEN_YOMO,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Mayela) => HebrewAccentDetails {
                english_name: "Mayela",
                hebrew_name: "מָאיְלָא",
                meaning:"to be raised or elevated",
                alt_english_name: Some("Meayyela"),
                alt_hebrew_name: Some("מְאַיְּלָא"),
                alt_meaning: None,
                first_code_point: CP_TIPEHA,
                second_code_point: None,
                comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq"),
            },
            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => HebrewAccentDetails {
                english_name: "Meteg",
                hebrew_name: "מֶתֶג",
                meaning:"accent or mark",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_METEG,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Maqqeph)
            | HebrewAccent::Poetry(PoetryAccent::Maqqeph) => HebrewAccentDetails {
                english_name: "Maqqeph",
                hebrew_name: "מַקֵּף",
                meaning:"binder",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_MAQAF,
                second_code_point: None,
                comment: Some("The Maqqeph (in Biblical Hebrew), can link two (or more) short words together, after which they function as a single compound word bearing a single Hebrew accent."),
            },
            /*********************************************************
            *                          POETRY
            *******************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) => HebrewAccentDetails {
                english_name: "Oleh We Yored",
                hebrew_name: "עוֹלֶה וְיוֹרֵד",
                meaning:"ascending and descending",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_OLE,
                second_code_point: Some(CP_MAHAPAKH),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol)=> HebrewAccentDetails {
                english_name: "Revia Gadol",
                hebrew_name: "רְבִיעַ גּדוֹל",
                meaning:"big fourth",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_REVIA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) => HebrewAccentDetails {
                english_name: "Revia Mugrash",
                hebrew_name: "רְבִיעַ מֻגְרָשׁ",
                meaning:"exiled fourth",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_GERESH,
                second_code_point: Some(CP_REVIA),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)=> HebrewAccentDetails {
                english_name: "Shalshelet Gadol",
                hebrew_name: "שַׁלְשֶׁלֶת גָּדוֹל",
                meaning:"large chain or link",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_SHALSHELET,
                second_code_point: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) => HebrewAccentDetails {
                english_name: "Tsinnor",
                hebrew_name: "צִנּוֹר",
                meaning:"pipe or tube",
                alt_english_name: Some("Zarqa"),
                alt_hebrew_name: Some("זַרְקָא"),
                alt_meaning: Some("to sprinkle, scatter"),
                first_code_point: CP_ZINOR,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) => HebrewAccentDetails {
                english_name: "Revia Qaton",
                hebrew_name: "רְבִיעַ קָטוֹן",
                meaning:"small fourth",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_REVIA,
                second_code_point: None,
                comment: Some("After that occurs Oleh we yored"),
            },
            HebrewAccent::Poetry(PoetryAccent::Dechi)=> HebrewAccentDetails {
                english_name: "Dechi",
                hebrew_name: "דֶּחִי",
                meaning:"to push or drive away",
                alt_english_name: Some("Tiphcha"),
                alt_hebrew_name: Some("טִפְחָא"),
                alt_meaning: Some("handbreadth or diagonal"),
                first_code_point: CP_DEHI,
                second_code_point: None,
                comment: None,
                },
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)=> HebrewAccentDetails {
                english_name: "Mehuppakh Legarmeh",
                hebrew_name: "מְהֻפָּךְ לְגַרְמֵהּ",
                meaning:"reversed to its own",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_MAHAPAKH,
                second_code_point: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => HebrewAccentDetails {
                english_name: "Azla Legarmeh",
                hebrew_name: "אַזְלָא לְגַרְמֶהּ",
                meaning:"goes to its own",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_QADMA,
                second_code_point: Some(CP_PASEQ),
                comment: None,
            },
            // Conjunctives

        //     HebrewAccent::Poetry(PoetryAccent::Munach) => HebrewAccentDetails {
        //         english_name: "Munach",
        //         hebrew_name: "מֻנַּח",
        //         meaning:"rest or placed",
        //         alt_english_name: None,
        //         alt_hebrew_name: None,
        //         alt_meaning: None,
        //         acc_type: AccentType::Primary,
        //         acc_category: AccentCategory::Conjunctive,
        //         first_code_point: CP_MUNAH,
        //         second_code_point: None,
        //         comment: None,
        // },
            // HebrewAccent::Poetry(PoetryAccent::Merkha)  {
            //     english_name: "Merkha",
            //     hebrew_name: "מֵרְכָא",
            //     meaning:"lengthener",
            //     alt_english_name: None,
            //     alt_hebrew_name: None,
            //     alt_meaning: None,
            //     acc_type: AccentType::Primary,
            //     acc_category: AccentCategory::Conjunctive,
            //     first_code_point: CP_MERKHA,
            //     second_code_point: None,
            //     comment: None,
            // },
            HebrewAccent::Poetry(PoetryAccent::Illuy) => HebrewAccentDetails {
                english_name: "Illuy",
                hebrew_name: "עִלּוּי",
                meaning:"elevation or raising",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ILUY,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::Tarkha) => HebrewAccentDetails {
                english_name: "Tiphcha",
                hebrew_name: "טִפְחָא",
                meaning:"handbreadth or diagonal",
                alt_english_name: None, // ADD TARCHA
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_TIPEHA,
                second_code_point: None,
                comment: None,
            },
            // HebrewAccent::Poetry(PoetryAccent::Mehuppakh) => HebrewAccentDetails {
            //     english_name: "Mehuppakh",
            //     hebrew_name: "מְהֻפַּך",
            //     meaning:"reversed or turned around",
            //     alt_english_name: Some("Mahpakh"),
            //     alt_hebrew_name: Some("מַהְפַּךְ"),
            //     alt_meaning: Some("turning round"),
            //     acc_type: AccentType::Primary,
            //     acc_category: AccentCategory::Conjunctive,
            //     first_code_point: CP_MAHAPAKH,
            //     second_code_point: None,
            //     comment: None,
            // },
            HebrewAccent::Poetry(PoetryAccent::Azla) => HebrewAccentDetails {
                english_name: "Azla",
                hebrew_name: "אַזְלָא֙",
                meaning:"going away",
                alt_english_name: Some("Qadma"),
                alt_hebrew_name: Some("קַדְמָ֨א"),
                alt_meaning: Some("antiquity or a former state"),
                first_code_point: CP_QADMA,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)=> HebrewAccentDetails {
                english_name: "Shalshelet Qetannah",
                hebrew_name: "שַׁלְשֶׁלֶת קְטַנָּה",
                meaning:"small chain",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_SHALSHELET,
                second_code_point: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)=> HebrewAccentDetails {
                english_name: "Tsinnorit Merkha",
                hebrew_name: "צִנּוֹרִת מֵרְכָא",
                meaning:"pipe of continuation",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ZARQA,
                second_code_point: Some(CP_MERKHA),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)=> HebrewAccentDetails {
                english_name: "Tsinnorit Mahpakh",
                hebrew_name: "צִנּוֹרִת מַהְפַּךְ",
                meaning:"pipe of reversal",
                alt_english_name: None,
                alt_hebrew_name: None,
                alt_meaning: None,
                first_code_point: CP_ZARQA,
                second_code_point: Some(CP_MAHAPAKH),
                comment: None,
            },
            //_ => false,
        }
    }
}

impl ProseAccent {
    pub const fn category(self) -> AccentCategory {
        match self {
            Self::Silluq
            | Self::Atnach
            | Self::Segolta
            | Self::Shalshelet
            | Self::ZaqephQaton
            | Self::ZaqephGadol
            | Self::Revia
            | Self::Tiphcha
            | Self::Zarqa
            | Self::Pashta
            | Self::Yetiv
            | Self::Tevir
            | Self::Geresh
            | Self::Gershayim
            | Self::Pazer
            | Self::PazerGadol
            | Self::TelishaGedolah
            | Self::Legarmeh => AccentCategory::Disjunctive,
            _ => AccentCategory::Conjunctive,
        }
    }
    pub fn accent_type(self) -> AccentType {
        match self {
            Self::Silluq
            | Self::Atnach
            | Self::Segolta
            | Self::Shalshelet
            | Self::ZaqephQaton
            | Self::ZaqephGadol
            | Self::Revia
            | Self::Tiphcha
            | Self::Zarqa
            | Self::Pashta
            | Self::Yetiv
            | Self::Tevir
            | Self::Geresh
            | Self::Gershayim
            | Self::Pazer
            | Self::PazerGadol
            | Self::TelishaGedolah
            | Self::Legarmeh
            | Self::Munach
            | Self::Mahpakh
            | Self::Merkha
            | Self::MerkhaKephulah
            | Self::Darga
            | Self::Azla
            | Self::TelishaQetannah
            | Self::Galgal => AccentType::Primary,

            Self::Mayela | Self::Meteg => AccentType::Secondary,
            Self::Maqqeph => AccentType::None,
        }
    }
    pub fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            Self::Silluq => 1,
            Self::Atnach => 2,
            Self::Segolta => 3,
            Self::Shalshelet => 4,
            Self::ZaqephQaton => 5,
            Self::ZaqephGadol => 6,
            Self::Revia => 7,
            Self::Tiphcha => 8,
            Self::Zarqa => 9,
            Self::Pashta => 10,
            Self::Yetiv => 11,
            Self::Tevir => 12,
            Self::Geresh => 13,
            Self::Gershayim => 14,
            Self::Pazer => 15,
            Self::PazerGadol => 16,
            Self::TelishaGedolah => 17,
            Self::Legarmeh => 18,
            // Conjunctives
            Self::Munach => 19,
            Self::Mahpakh => 20,
            Self::Merkha => 21,
            Self::MerkhaKephulah => 22,
            Self::Darga => 23,
            Self::Azla => 24,
            Self::TelishaQetannah => 25,
            Self::Galgal => 26,
            Self::Mayela => 27,
            Self::Meteg => 28,
            Self::Maqqeph => 29,
        }
    }
    /// Returns detailsrmation about the accent.
    /// This can be expanded to include more details as needed.
    // #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            Self::Silluq => HebrewAccent::Poetry(PoetryAccent::Silluq).details(),
            Self::Atnach => HebrewAccent::Prose(ProseAccent::Atnach).details(),
            Self::Segolta => HebrewAccent::Prose(ProseAccent::Segolta).details(),
            Self::Shalshelet => HebrewAccent::Prose(ProseAccent::Shalshelet).details(),
            Self::ZaqephQaton => HebrewAccent::Prose(ProseAccent::ZaqephQaton).details(),
            Self::ZaqephGadol => HebrewAccent::Prose(ProseAccent::ZaqephGadol).details(),
            Self::Revia => HebrewAccent::Prose(ProseAccent::Revia).details(),
            Self::Tiphcha => HebrewAccent::Prose(ProseAccent::Tiphcha).details(),
            Self::Zarqa => HebrewAccent::Prose(ProseAccent::Zarqa).details(),
            Self::Pashta => HebrewAccent::Prose(ProseAccent::Pashta).details(),
            Self::Yetiv => HebrewAccent::Prose(ProseAccent::Yetiv).details(),
            Self::Tevir => HebrewAccent::Prose(ProseAccent::Tevir).details(),
            Self::Geresh => HebrewAccent::Prose(ProseAccent::Geresh).details(),
            Self::Gershayim => HebrewAccent::Prose(ProseAccent::Gershayim).details(),
            Self::Pazer => HebrewAccent::Prose(ProseAccent::Pazer).details(),
            Self::PazerGadol => HebrewAccent::Prose(ProseAccent::PazerGadol).details(),
            Self::TelishaGedolah => HebrewAccent::Prose(ProseAccent::TelishaGedolah).details(),

            Self::Legarmeh => HebrewAccent::Prose(ProseAccent::Legarmeh).details(),
            // Conjunctives
            Self::Munach => HebrewAccent::Prose(ProseAccent::Munach).details(),
            Self::Mahpakh => HebrewAccent::Prose(ProseAccent::Mahpakh).details(),
            Self::Merkha => HebrewAccent::Prose(ProseAccent::Merkha).details(),
            Self::MerkhaKephulah => HebrewAccent::Prose(ProseAccent::MerkhaKephulah).details(),
            Self::Darga => HebrewAccent::Prose(ProseAccent::Darga).details(),
            Self::Azla => HebrewAccent::Prose(ProseAccent::Azla).details(),
            Self::TelishaQetannah => HebrewAccent::Prose(ProseAccent::TelishaQetannah).details(),
            Self::Galgal => HebrewAccent::Prose(ProseAccent::Galgal).details(),
            Self::Mayela => HebrewAccent::Prose(ProseAccent::Mayela).details(),
            Self::Meteg => HebrewAccent::Prose(ProseAccent::Meteg).details(),
            Self::Maqqeph => HebrewAccent::Prose(ProseAccent::Maqqeph).details(),
        }
    }
}

impl PoetryAccent {
    pub const fn category(self) -> AccentCategory {
        match self {
            Self::Silluq
            | Self::OlehWeYored
            | Self::Atnach
            | Self::ReviaGadol
            | Self::ReviaMugrash
            | Self::ShalsheletGadol
            | Self::Tsinnor
            | Self::ReviaQaton
            | Self::Dechi
            | Self::Pazer
            | Self::MehuppakhLegarmeh
            | Self::AzlaLegarmeh => AccentCategory::Disjunctive,
            _ => AccentCategory::Conjunctive,
        }
    }
    pub const fn accent_type(self) -> AccentType {
        match self {
            Self::Silluq |
            Self::OlehWeYored |
            Self::Atnach |
            Self::ReviaGadol |
            Self::ReviaMugrash |
            Self::ShalsheletGadol |
            Self::Tsinnor |
            Self::ReviaQaton |
            Self::Dechi |
            Self::Pazer |
            Self::MehuppakhLegarmeh |
            Self::AzlaLegarmeh |
            // Conjunctives
            Self::Munach |
            Self::Merkha |
            Self::Illuy |
            Self::Tarkha |
            Self::Galgal |
            Self::Mehuppakh |
            Self::Azla |
            Self::ShalsheletQetannah |
            Self::TsinnoritMerkha |
            Self::TsinnoritMahpakh  => AccentType::Primary,
            Self::Meteg => AccentType::Secondary,
            Self::Maqqeph => AccentType::None,
        }
    }
    pub fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            Self::Silluq => 1,
            Self::OlehWeYored => 2,
            Self::Atnach => 3,
            Self::ReviaGadol => 4,
            Self::ReviaMugrash => 5,
            Self::ShalsheletGadol => 6,
            Self::Tsinnor => 7,
            Self::ReviaQaton => 8,
            Self::Dechi => 9,
            Self::Pazer => 10,
            Self::MehuppakhLegarmeh => 11,
            Self::AzlaLegarmeh => 12,
            // Conjunctives
            Self::Munach => 13,
            Self::Merkha => 14,
            Self::Illuy => 15,
            Self::Tarkha => 16,
            Self::Galgal => 17,
            Self::Mehuppakh => 18,
            Self::Azla => 19,
            Self::ShalsheletQetannah => 20,
            Self::TsinnoritMerkha => 21,
            Self::TsinnoritMahpakh => 21,
            Self::Meteg => 22,
            Self::Maqqeph => 23,
        }
    }
    // #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            Self::Silluq => HebrewAccent::Poetry(Self::Silluq).details(),
            Self::OlehWeYored => HebrewAccent::Poetry(Self::OlehWeYored).details(),
            Self::Atnach => HebrewAccent::Poetry(Self::Atnach).details(),
            Self::ReviaGadol => HebrewAccent::Poetry(Self::ReviaGadol).details(),
            Self::ReviaMugrash => HebrewAccent::Poetry(Self::ReviaMugrash).details(),
            Self::ShalsheletGadol => HebrewAccent::Poetry(Self::ShalsheletGadol).details(),
            Self::Tsinnor => HebrewAccent::Poetry(Self::Tsinnor).details(),
            Self::ReviaQaton => HebrewAccent::Poetry(Self::ReviaQaton).details(),
            Self::Dechi => HebrewAccent::Poetry(Self::Dechi).details(),
            Self::Pazer => HebrewAccent::Poetry(Self::Pazer).details(),
            Self::MehuppakhLegarmeh => HebrewAccent::Poetry(Self::MehuppakhLegarmeh).details(),
            Self::AzlaLegarmeh => HebrewAccent::Poetry(Self::AzlaLegarmeh).details(),
            // Conjunctives
            Self::Munach => HebrewAccent::Poetry(Self::Munach).details(),
            Self::Merkha => HebrewAccent::Poetry(Self::Merkha).details(),
            Self::Illuy => HebrewAccent::Poetry(Self::Illuy).details(),
            Self::Tarkha => HebrewAccent::Poetry(Self::Tarkha).details(),
            Self::Galgal => HebrewAccent::Poetry(Self::Galgal).details(),
            Self::Mehuppakh => HebrewAccent::Poetry(Self::Mehuppakh).details(),
            Self::Azla => HebrewAccent::Poetry(Self::Azla).details(),
            Self::ShalsheletQetannah => HebrewAccent::Poetry(Self::ShalsheletQetannah).details(),
            Self::TsinnoritMerkha => HebrewAccent::Poetry(Self::TsinnoritMerkha).details(),
            Self::TsinnoritMahpakh => HebrewAccent::Poetry(Self::TsinnoritMahpakh).details(),
            Self::Meteg => HebrewAccent::Poetry(Self::Meteg).details(),
            Self::Maqqeph => HebrewAccent::Poetry(PoetryAccent::Maqqeph).details(),
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
        println!("\n{:#?}", ProseAccent::Munach.details());
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
        println!("\n{:#?}", ProseAccent::Munach.rank());
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
    fn test_prose_accent_details() {
        let pa = ProseAccent::Galgal;
        assert_eq!("wheel, circle", pa.details().bhs_meaning);
    }
    #[test]
    fn test_poetry_accent_details() {
        let pa = PoetryAccent::Galgal;
        assert_eq!("wheel, circle", pa.details().bhs_meaning);
    }
    #[test]
    fn test_hebrew_accent_details() {
        let ha = HebrewAccent::Prose(ProseAccent::Galgal);
        assert_eq!("wheel, circle", ha.details().bhs_meaning);
    }
}
