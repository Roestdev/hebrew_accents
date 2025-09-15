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
    bhs_name: String,
    bhs_hebrew: String,
    bhs_meaning: String,
    alt_bhs_name: Option<String>,
    alt_bhs_hebrew: Option<String>,
    alt_bhs_meaning: Option<String>,
    acc_category: AccentCategory,
    acc_type: AccentType,
    code_point_1: Utf8CodePointInfo,
    code_point_2: Option<Utf8CodePointInfo>,
    comment: Option<String>,
}

/// Details on a specific UTF8 Unicode code-point
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Utf8CodePointInfo {
    pub code_point: &'static str,
    pub hex_value: &'static str,
    pub name: &'static str,
    pub symbol: &'static str,
    pub position: AccentPosition,
    pub ashkenazi: Option<Tradition>,
    pub sephardi: Option<Tradition>,
    pub italian: Option<Tradition>,
    pub yemenite: Option<Tradition>,
}
/// Names according one of four Hebrew Traditions
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
    Primairy,
    // used for Meayla and Meteg
    Secondary,
    // used for Maqqeph
    None,
}

/// Accent position, Indicating the location of the accent in relation to the consonant
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentPosition {
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
                bhs_name: "Silluq".to_string(),
                bhs_hebrew: "סִלּוּק".to_string(),
                bhs_meaning:"close, cessation".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SILLUQ,
                code_point_2: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => HebrewAccentDetails {
                bhs_name: "Atnach".to_string(),
                bhs_hebrew: "אתְנָח".to_string(),
                bhs_meaning: "a causing to rest".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ETNAHTA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Segolta)  => HebrewAccentDetails {
                bhs_name: "Segolta".to_string(),
                bhs_hebrew: "סְגֹולְתָּא".to_string(),
                bhs_meaning: "a little grape-bunch".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SEGOL,
                code_point_2: None,
                comment: None,
                },
            HebrewAccent::Prose(ProseAccent::Shalshelet)  => HebrewAccentDetails {
                bhs_name: "Shalshelet".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת".to_string(),
                bhs_meaning: "chain or link".to_string(), 
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SHALSHELET,
                code_point_2: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::ZaqephQaton)  =>  HebrewAccentDetails {
                bhs_name: "Zaqeph Qaton".to_string(),
                bhs_hebrew: "זָקֵף קָטוֹן".to_string(),
                bhs_meaning:"small upright".to_string(), 
                alt_bhs_name: Some("Zaqeph Qatan".to_string()),
                alt_bhs_hebrew: Some("זָקֵף קָטָן".to_string()),
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZAQEF_QATAN,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) => HebrewAccentDetails {
                bhs_name: "Zaqeph Gadol".to_string(),
                bhs_hebrew: "זָקֵף גָּדוֹל".to_string(),
                bhs_meaning:"large upright".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZAQEF_GADOL,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Revia)  => HebrewAccentDetails {
                bhs_name: "Revia".to_string(),
                bhs_hebrew:"רְבִיעַ".to_string(),
                bhs_meaning:"fourth [in a sequence]".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
                comment: Some("probably due to its four-note tune.".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Tiphcha)  => HebrewAccentDetails {
                bhs_name: "Tiphcha".to_string(),
                bhs_hebrew: "טִפְחָא".to_string(),
                bhs_meaning:"handbreadth or diagonal".to_string(),
                alt_bhs_name: None, // ADD TARCHA
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
                comment: Some("before Atnach and Silluq".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Zarqa) => HebrewAccentDetails {
                bhs_name: "Zarqa".to_string(),
                bhs_hebrew: "זַרְקָא".to_string(),
                bhs_meaning: "to sprinkle, scatter".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_ZINOR,
                code_point_2: None,
                comment: Some("Ibefore  Segolta".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Pashta) => HebrewAccentDetails {
                bhs_name: "Pashta".to_string(),
                bhs_hebrew: "פַּשְׁטָא".to_string(),
                bhs_meaning:"extending, stretching out in length".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_PASHTA,
                code_point_2: None,
                comment: Some("if you sound almost last (2 pasta’s in one word)".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Yetiv) => HebrewAccentDetails {
                bhs_name: "Yetiv".to_string(),
                bhs_hebrew: "יְתִיב".to_string(),
                bhs_meaning:"resting or sitting".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_YETIV,
                code_point_2: None,
                comment: Some("occasionally for a Pashta".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Tevir) => HebrewAccentDetails {
                bhs_name: "Tevir".to_string(),
                bhs_hebrew: "תְּבִיר".to_string(),
                bhs_meaning: "broken, downward tumble".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TEVIR,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Geresh) => HebrewAccentDetails {
                bhs_name: "Geresh".to_string(),
                bhs_hebrew: "גֵּרֵישׁ".to_string(),
                bhs_meaning:"expulsion, driving out, divorce".to_string(),
                alt_bhs_name: Some("Teres".to_string()),
                alt_bhs_hebrew: Some("טֶרֶס".to_string()),
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_GERESH,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Gershayim) => HebrewAccentDetails {
                bhs_name: "Gershayim".to_string(),
                bhs_hebrew: "גֵּרְשַׁיִם".to_string(),
                bhs_meaning:"double of expulsion, driving out, divorce".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_GERSHAYIM,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => HebrewAccentDetails {
                bhs_name: "Pazer".to_string(),
                bhs_hebrew: "פָּזֶר".to_string(),
                bhs_meaning:"lavish or scatter".to_string(),
                alt_bhs_name: Some("Pazer Qatan".to_string()),
                alt_bhs_hebrew: Some("פָּזֵר קָטָן".to_string()),
                alt_bhs_meaning: Some("small lavish or scatter".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_PAZER,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::PazerGadol) => HebrewAccentDetails {
                bhs_name: "Pazer Gadol".to_string(),
                bhs_hebrew: "פָּזֶר גּדוֹל".to_string(),
                bhs_meaning:"large lavish or scatter".to_string(),
                alt_bhs_name: Some("Qarne Pharah".to_string()),
                alt_bhs_hebrew: Some("קַרְנֵי פָרָה".to_string()),
                alt_bhs_meaning: Some("horns of a cow".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_QARNEY_PARA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)=> HebrewAccentDetails {
                bhs_name: "Telisha Gedolah".to_string(),
                bhs_hebrew: "תְּלִישָׁא גְּדוֹלָה".to_string(),
                bhs_meaning:"great (long) detached".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_TELISHA_GEDOLA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Legarmeh)=> HebrewAccentDetails {
                bhs_name: "Legarmeh".to_string(),
                bhs_hebrew: "לְגַרְמֶהּ".to_string(),
                bhs_meaning:"for or by itself, independant".to_string(),
                alt_bhs_name: Some("Munach Legarmeh".to_string()),
                alt_bhs_hebrew: Some("מוּנַח לְגַרְמֵ֣הּ".to_string()),
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_MUNAH,
                code_point_2: Some(CP_PASEQ),
                comment:Some("Munach with passeq; Before Revia".to_string()),
            },
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) | HebrewAccent::Poetry(PoetryAccent::Munach) => HebrewAccentDetails {
                bhs_name: "Munach".to_string(),
                bhs_hebrew: "מוּנַ֣ח".to_string(),
                bhs_meaning:"resting or placed".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MUNAH,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Mahpakh) | HebrewAccent::Poetry(PoetryAccent::Mehuppakh)=> HebrewAccentDetails {
                bhs_name: "Mahpakh".to_string(),
                bhs_hebrew: "מַהְפַּךְ".to_string(),
                bhs_meaning:"turning round".to_string(),
                alt_bhs_name: Some("Mehuppakh".to_string()),
                alt_bhs_hebrew: Some("מְהֻפָּ֤ךְ".to_string()),
                alt_bhs_meaning: Some("reversed".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MAHAPAKH,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Merkha) | HebrewAccent::Poetry(PoetryAccent::Merkha) => HebrewAccentDetails {
                bhs_name: "Merkha".to_string(),
                bhs_hebrew: "מֵרְכָא".to_string(),
                bhs_meaning:"lengthener, prolonging".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MERKHA,
                code_point_2: None,
                comment:  None,
            },
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)=> HebrewAccentDetails {
                bhs_name: "Merkha Kephulah".to_string(),
                bhs_hebrew: "מֵרְכָא כְּפוּלָה".to_string(),
                bhs_meaning:"double lengthener".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_MERKHA_KEFULA,
                code_point_2: None,
                comment: Some("Merkha duplex".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Darga) => HebrewAccentDetails {
                bhs_name: "Darga".to_string(),
                bhs_hebrew: "דַּרְגָּא".to_string(),
                bhs_meaning:"stairstep".to_string(), 
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_DARGA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Azla) => HebrewAccentDetails {
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא".to_string(),
                bhs_meaning:"going on (not pausing), depart".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_QADMA,
                code_point_2: None,
                comment: Some("When Geresh: Qadma".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)=> HebrewAccentDetails {
                bhs_name: "Telisha Qetannah".to_string(),
                bhs_hebrew: "תְּלִישָא קְטַנָּה".to_string(),
                bhs_meaning:"small (short) detached".to_string(),
                alt_bhs_name:  None,
                alt_bhs_hebrew:  None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TELISHA_QETANA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => HebrewAccentDetails {
                bhs_name: "Galgal".to_string(),
                bhs_hebrew: "גַּלְגַּל".to_string(),
                bhs_meaning:"wheel, circle".to_string(),
                alt_bhs_name: Some("Jerach Ben Jomo".to_string()),
                alt_bhs_hebrew: Some("יֵרֶח בֶּן יוֹמוֹ".to_string()),
                alt_bhs_meaning: Some("moon one day old".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_YERAH_BEN_YOMO,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Mayela) => HebrewAccentDetails {
                bhs_name: "Mayela".to_string(),
                bhs_hebrew: "מָאיְלָא".to_string(),
                bhs_meaning:"to be raised or elevated".to_string(),
                alt_bhs_name: Some("Meayyela".to_string()),
                alt_bhs_hebrew: Some("מְאַיְּלָא".to_string()),
                alt_bhs_meaning: None,
                acc_type: AccentType::Secondary,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
                comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq".to_string()),
            },
            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => HebrewAccentDetails {
                bhs_name: "Meteg".to_string(),
                bhs_hebrew: "מֶתֶג".to_string(),
                bhs_meaning:"accent or mark".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Secondary,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_METEG,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Prose(ProseAccent::Maqqeph)
            | HebrewAccent::Poetry(PoetryAccent::Maqqeph) => HebrewAccentDetails {
                bhs_name: "Maqqeph".to_string(),
                bhs_hebrew: "מַקֵּף".to_string(),
                bhs_meaning:"binder".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::None,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_MAQAF,
                code_point_2: None,
                comment: Some("The Maqqeph (in Biblical Hebrew), can link two (or more) short words together, after which they function as a single compound word bearing a single Hebrew accent.".to_string()),
            },
            /*********************************************************
            *                          POETRY
            *******************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) => HebrewAccentDetails {
                bhs_name: "Oleh We Yored".to_string(),
                bhs_hebrew: "עוֹלֶה וְיוֹרֵד".to_string(),
                bhs_meaning:"ascending and descending".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_OLE,
                code_point_2: Some(CP_MAHAPAKH),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol)=> HebrewAccentDetails {
                bhs_name: "Revia Gadol".to_string(),
                bhs_hebrew: "רְבִיעַ גּדוֹל".to_string(),
                bhs_meaning:"big fourth".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) => HebrewAccentDetails {
                bhs_name: "Revia Mugrash".to_string(),
                bhs_hebrew: "רְבִיעַ מֻגְרָשׁ".to_string(),
                bhs_meaning:"exiled fourth".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_GERESH,
                code_point_2: Some(CP_REVIA),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)=> HebrewAccentDetails {
                bhs_name: "Shalshelet Gadol".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת גָּדוֹל".to_string(),
                bhs_meaning:"large chain or link".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_SHALSHELET,
                code_point_2: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) => HebrewAccentDetails {
                bhs_name: "Tsinnor".to_string(),
                bhs_hebrew: "צִנּוֹר".to_string(),
                bhs_meaning:"pipe or tube".to_string(),
                alt_bhs_name: Some("Zarqa".to_string()),
                alt_bhs_hebrew: Some("זַרְקָא".to_string()),
                alt_bhs_meaning: Some("to sprinkle, scatter".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_ZINOR,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) => HebrewAccentDetails {
                bhs_name: "Revia Qaton".to_string(),
                bhs_hebrew: "רְבִיעַ קָטוֹן".to_string(),
                bhs_meaning:"small fourth".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_REVIA,
                code_point_2: None,
                comment: Some("After that occurs Oleh we yored".to_string()),
            },
            HebrewAccent::Poetry(PoetryAccent::Dechi)=> HebrewAccentDetails {
                bhs_name: "Dechi".to_string(),
                bhs_hebrew: "דֶּחִי".to_string(),
                bhs_meaning:"to push or drive away".to_string(),
                alt_bhs_name: Some("Tiphcha".to_string()),
                alt_bhs_hebrew: Some("טִפְחָא".to_string()),
                alt_bhs_meaning: Some("handbreadth or diagonal".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_DEHI,
                code_point_2: None,
                comment: None,
                },
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)=> HebrewAccentDetails {
                bhs_name: "Mehuppakh Legarmeh".to_string(),
                bhs_hebrew: "מְהֻפָּךְ לְגַרְמֵהּ".to_string(),
                bhs_meaning:"reversed to its own".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_MAHAPAKH,
                code_point_2: Some(CP_PASEQ),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => HebrewAccentDetails {
                bhs_name: "Azla Legarmeh".to_string(),
                bhs_hebrew: "אַזְלָא לְגַרְמֶהּ".to_string(),
                bhs_meaning:"goes to its own".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Disjunctive,
                code_point_1: CP_QADMA,
                code_point_2: Some(CP_PASEQ),
                comment: None,
            },
            // Conjunctives
        //     HebrewAccent::Poetry(PoetryAccent::Munach) => HebrewAccentDetails {
        //         bhs_name: "Munach".to_string(),
        //         bhs_hebrew: "מֻנַּח".to_string(),
        //         bhs_meaning:"rest or placed".to_string(),
        //         alt_bhs_name: None,
        //         alt_bhs_hebrew: None,
        //         alt_bhs_meaning: None,
        //         acc_type: AccentType::Primairy,
        //         acc_category: AccentCategory::Conjunctive,
        //         code_point_1: CP_MUNAH,
        //         code_point_2: None,
        //         comment: None,
        // },
            // HebrewAccent::Poetry(PoetryAccent::Merkha)  {
            //     bhs_name: "Merkha".to_string(),
            //     bhs_hebrew: "מֵרְכָא".to_string(),
            //     bhs_meaning:"lengthener".to_string(),
            //     alt_bhs_name: None,
            //     alt_bhs_hebrew: None,
            //     alt_bhs_meaning: None,
            //     acc_type: AccentType::Primairy,
            //     acc_category: AccentCategory::Conjunctive,
            //     code_point_1: CP_MERKHA,
            //     code_point_2: None,
            //     comment: None,
            // },
            HebrewAccent::Poetry(PoetryAccent::Illuy) => HebrewAccentDetails {
                bhs_name: "Illuy".to_string(),
                bhs_hebrew: "עִלּוּי".to_string(),
                bhs_meaning:"elevation or raising".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ILUY,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::Tarkha) => HebrewAccentDetails {
                bhs_name: "Tiphcha".to_string(),
                bhs_hebrew: "טִפְחָא".to_string(),
                bhs_meaning:"handbreadth or diagonal".to_string(),
                alt_bhs_name: None, // ADD TARCHA
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                code_point_1: CP_TIPEHA,
                code_point_2: None,
                comment: None,
            },
            // HebrewAccent::Poetry(PoetryAccent::Mehuppakh) => HebrewAccentDetails {
            //     bhs_name: "Mehuppakh".to_string(),
            //     bhs_hebrew: "מְהֻפַּך".to_string(),
            //     bhs_meaning:"reversed or turned around".to_string(),
            //     alt_bhs_name: Some("Mahpakh".to_string()),
            //     alt_bhs_hebrew: Some("מַהְפַּךְ".to_string()),
            //     alt_bhs_meaning: Some("turning round".to_string()),
            //     acc_type: AccentType::Primairy,
            //     acc_category: AccentCategory::Conjunctive,
            //     code_point_1: CP_MAHAPAKH,
            //     code_point_2: None,
            //     comment: None,
            // },
            HebrewAccent::Poetry(PoetryAccent::Azla) => HebrewAccentDetails {
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא֙".to_string(),
                bhs_meaning:"going away".to_string(),
                alt_bhs_name: Some("Qadma".to_string()),
                alt_bhs_hebrew: Some("קַדְמָ֨א".to_string()),
                alt_bhs_meaning: Some("antiquity or a former state".to_string()),
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_QADMA,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)=> HebrewAccentDetails {
                bhs_name: "Shalshelet Qetannah".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת קְטַנָּה".to_string(),
                bhs_meaning:"small chain".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_SHALSHELET,
                code_point_2: None,
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)=> HebrewAccentDetails {
                bhs_name: "Tsinnorit Merkha".to_string(),
                bhs_hebrew: "צִנּוֹרִת מֵרְכָא".to_string(),
                bhs_meaning:"pipe of continuation".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ZARQA,
                code_point_2: Some(CP_MERKHA),
                comment: None,
            },
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)=> HebrewAccentDetails {
                bhs_name: "Tsinnorit Mahpakh".to_string(),
                bhs_hebrew: "צִנּוֹרִת מַהְפַּךְ".to_string(),
                bhs_meaning:"pipe of reversal".to_string(),
                alt_bhs_name: None,
                alt_bhs_hebrew: None,
                alt_bhs_meaning: None,
                acc_type: AccentType::Primairy,
                acc_category: AccentCategory::Conjunctive,
                code_point_1: CP_ZARQA,
                code_point_2: Some(CP_MAHAPAKH),
                comment: None,
            },
            //_ => false,
        }
    }
}

impl ProseAccent {
    // #[allow(unused)]
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
            ProseAccent::Munach => 19,
            ProseAccent::Mahpakh => 20,
            ProseAccent::Merkha => 21,
            ProseAccent::MerkhaKephulah => 22,
            ProseAccent::Darga => 23,
            ProseAccent::Azla => 24,
            ProseAccent::TelishaQetannah => 25,
            ProseAccent::Galgal => 26,
            ProseAccent::Mayela => 27,
            ProseAccent::Meteg => 28,
            ProseAccent::Maqqeph => 29,
        }
    }
    /// Returns detailsrmation about the accent.
    /// This can be expanded to include more details as needed.
    // #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            ProseAccent::Silluq => HebrewAccent::Poetry(PoetryAccent::Silluq).details(),
            ProseAccent::Atnach => HebrewAccent::Prose(ProseAccent::Atnach).details(),
            ProseAccent::Segolta => HebrewAccent::Prose(ProseAccent::Segolta).details(),
            ProseAccent::Shalshelet => HebrewAccent::Prose(ProseAccent::Shalshelet).details(),
            ProseAccent::ZaqephQaton => HebrewAccent::Prose(ProseAccent::ZaqephQaton).details(),
            ProseAccent::ZaqephGadol => HebrewAccent::Prose(ProseAccent::ZaqephGadol).details(),
            ProseAccent::Revia => HebrewAccent::Prose(ProseAccent::Revia).details(),
            ProseAccent::Tiphcha => HebrewAccent::Prose(ProseAccent::Tiphcha).details(),
            ProseAccent::Zarqa => HebrewAccent::Prose(ProseAccent::Zarqa).details(),
            ProseAccent::Pashta => HebrewAccent::Prose(ProseAccent::Pashta).details(),
            ProseAccent::Yetiv => HebrewAccent::Prose(ProseAccent::Yetiv).details(),
            ProseAccent::Tevir => HebrewAccent::Prose(ProseAccent::Tevir).details(),
            ProseAccent::Geresh => HebrewAccent::Prose(ProseAccent::Geresh).details(),
            ProseAccent::Gershayim => HebrewAccent::Prose(ProseAccent::Gershayim).details(),
            ProseAccent::Pazer => HebrewAccent::Prose(ProseAccent::Pazer).details(),
            ProseAccent::PazerGadol => HebrewAccent::Prose(ProseAccent::PazerGadol).details(),
            ProseAccent::TelishaGedolah  => HebrewAccent::Prose(ProseAccent::TelishaGedolah).details(),
            ProseAccent::Legarmeh => HebrewAccent::Prose(ProseAccent::Legarmeh).details(),
            // Conjunctives
            ProseAccent::Munach => HebrewAccent::Prose(ProseAccent::Munach).details(),
            ProseAccent::Mahpakh => HebrewAccent::Prose(ProseAccent::Mahpakh).details(),
            ProseAccent::Merkha => HebrewAccent::Prose(ProseAccent::Merkha).details(),
            ProseAccent::MerkhaKephulah => HebrewAccent::Prose(ProseAccent::MerkhaKephulah).details(),
            ProseAccent::Darga => HebrewAccent::Prose(ProseAccent::Darga).details(),
            ProseAccent::Azla => HebrewAccent::Prose(ProseAccent::Azla).details(),
            ProseAccent::TelishaQetannah => HebrewAccent::Prose(ProseAccent::TelishaQetannah).details(),
            ProseAccent::Galgal => HebrewAccent::Prose(ProseAccent::Galgal).details(),
            ProseAccent::Mayela => HebrewAccent::Prose(ProseAccent::Mayela).details(),
            ProseAccent::Meteg => HebrewAccent::Prose(ProseAccent::Meteg).details(),
            ProseAccent::Maqqeph => HebrewAccent::Prose(ProseAccent::Maqqeph).details(),
        }
    }
}

impl PoetryAccent {
    // #[allow(unused)]
    pub fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => 1,
            PoetryAccent::OlehWeYored => 2,
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
            PoetryAccent::Munach => 13,
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
            PoetryAccent::Maqqeph => 23,
        }
    }
    // #[allow(unused)]
    fn details(&self) -> HebrewAccentDetails {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => HebrewAccent::Poetry(PoetryAccent::Silluq).details(),
            PoetryAccent::OlehWeYored => HebrewAccent::Poetry(PoetryAccent::OlehWeYored).details(),
            PoetryAccent::Atnach => HebrewAccent::Poetry(PoetryAccent::Atnach).details(),
            PoetryAccent::ReviaGadol => HebrewAccent::Poetry(PoetryAccent::ReviaGadol).details(),
            PoetryAccent::ReviaMugrash => HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).details(),
            PoetryAccent::ShalsheletGadol => HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).details(),
            PoetryAccent::Tsinnor => HebrewAccent::Poetry(PoetryAccent::Tsinnor).details(),
            PoetryAccent::ReviaQaton => HebrewAccent::Poetry(PoetryAccent::ReviaQaton).details(),
            PoetryAccent::Dechi => HebrewAccent::Poetry(PoetryAccent::Dechi).details(),
            PoetryAccent::Pazer => HebrewAccent::Poetry(PoetryAccent::Pazer).details(),
            PoetryAccent::MehuppakhLegarmeh => HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).details(),
            PoetryAccent::AzlaLegarmeh => HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).details(),
            // Conjunctives
            PoetryAccent::Munach => HebrewAccent::Poetry(PoetryAccent::Munach).details(),
            PoetryAccent::Merkha => HebrewAccent::Poetry(PoetryAccent::Merkha).details(),
            PoetryAccent::Illuy => HebrewAccent::Poetry(PoetryAccent::Illuy).details(),
            PoetryAccent::Tarkha => HebrewAccent::Poetry(PoetryAccent::Tarkha).details(),
            PoetryAccent::Galgal => HebrewAccent::Poetry(PoetryAccent::Galgal).details(),
            PoetryAccent::Mehuppakh => HebrewAccent::Poetry(PoetryAccent::Mehuppakh).details(),
            PoetryAccent::Azla => HebrewAccent::Poetry(PoetryAccent::Azla).details(),
            PoetryAccent::ShalsheletQetannah => HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).details(),
            PoetryAccent::TsinnoritMerkha => HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).details(),
            PoetryAccent::TsinnoritMahpakh => HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).details(),
            PoetryAccent::Meteg => HebrewAccent::Poetry(PoetryAccent::Meteg).details(),
            PoetryAccent::Maqqeph => HebrewAccent::Poetry(PoetryAccent::Maqqeph).details(),
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
        assert_eq!("wheel, circle".to_string(), pa.details().bhs_meaning);
    }
    #[test]
    fn test_poetry_accent_details() {
        let pa = PoetryAccent::Galgal;
        assert_eq!("wheel, circle".to_string(), pa.details().bhs_meaning);
    }
    #[test]
    fn test_hebrew_accent_details() {
        let ha = HebrewAccent::Prose(ProseAccent::Galgal);
        assert_eq!("wheel, circle".to_string(), ha.details().bhs_meaning);
    }
}
