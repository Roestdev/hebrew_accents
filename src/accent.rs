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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct AccentInfo {
    bhs_name: String,
    bhs_hebrew: String,
    bhs_meaning: String,
    alt_name: Option<String>,
    alt_hebrew: Option<String>,
    alt_meaning: Option<String>,
    category: AccentCategory,
    acc_type: AccentType,
    nr_of_code_points: AccentCodePoints,
    position_codepoint1: AccentPosition,
    position_codepoint2: Option<AccentPosition>,
    comment: Option<String>,
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum AccentCodePoints {
    #[default]
    One,
    Two,
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
    /// Returns information about the accent.
    /// This can be expanded to include more details as needed.
    #[allow(unused)]
    fn info(&self) -> AccentInfo {
        match self {
            // Disjunctives
            ProseAccent::Silluq => AccentInfo {
                bhs_name: "Silluq".to_string(),
                bhs_hebrew: "סִלּוּק".to_string(),
                bhs_meaning:"close, cessation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Atnach => AccentInfo {
                bhs_name: "Atnach".to_string(),
                bhs_hebrew: "אתְנָח".to_string(),
                bhs_meaning: "a causing to rest".to_string(),
                alt_name: Some("Etnachta".to_string()),
                alt_hebrew: Some("אֶתְנַחְתָּא".to_string()),
                alt_meaning: Some("pauser".to_string()),
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Segolta => AccentInfo {
                bhs_name: "Segolta".to_string(),
                bhs_hebrew: "סְגֹולְתָּא".to_string(),
                bhs_meaning: "a little grape-bunch".to_string(),
                alt_name: Some("Segol".to_string()),
                alt_hebrew: Some("סְגֹול".to_string()),
                alt_meaning: Some("grape-bunch".to_string()),
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Shalshelet => AccentInfo {
                bhs_name: "Shalshelet".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת".to_string(),
                bhs_meaning: "chain or link".to_string(), 
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            ProseAccent::ZaqephQaton => AccentInfo {
                bhs_name: "Zaqeph Qaton".to_string(),
                bhs_hebrew: "זָקֵף קָטוֹן".to_string(),
                bhs_meaning:"small upright".to_string(), 
                alt_name: Some("Zaqeph Qatan".to_string()),
                alt_hebrew: Some("זָקֵף קָטָן".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::ZaqephGadol => AccentInfo {
                bhs_name: "Zaqeph Gadol".to_string(),
                bhs_hebrew: "זָקֵף גָּדוֹל".to_string(),
                bhs_meaning:"large upright".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Revia => AccentInfo {
                bhs_name: "Revia".to_string(),
                bhs_hebrew:"רְבִיעַ".to_string(),
                bhs_meaning:"fourth [in a sequence]".to_string(),
                alt_name: Some("Ravia".to_string()),
                alt_hebrew: Some("רָבִיעַ".to_string()),
                alt_meaning: None,
                comment: Some("probably due to its four-note tune.".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Tiphcha => AccentInfo {
                bhs_name: "Tiphcha".to_string(),
                bhs_hebrew: "טִפְחָא".to_string(),
                bhs_meaning:"handbreadth or diagonal".to_string(),//OK
                alt_name: Some("Tarcha".to_string()),
                alt_hebrew: Some("טַרְחָא".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Zarqa => AccentInfo {
                bhs_name: "Zarqa".to_string(),
                bhs_hebrew: "זַרְקָא".to_string(),
                bhs_meaning:"to sprinkle, scatter".to_string(),
                alt_name: Some("Tsinnor".to_string()),
                alt_hebrew: Some("צִנּוֹר֮".to_string()),
                alt_meaning: None,
                comment: Some("In Yemeni tradition, it is called Tsinnor".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Pashta => AccentInfo {
                bhs_name: "Pashta".to_string(),
                bhs_hebrew: "פַּשְׁטָא".to_string(),
                bhs_meaning:"extending, stretching out in length".to_string(),
                alt_name: Some("Qadma".to_string()),
                alt_hebrew: Some("קַדְמָא".to_string()),
                alt_meaning: Some("to progress, advance".to_string()),
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Yetiv => AccentInfo {
                bhs_name: "Yetiv".to_string(),
                bhs_hebrew: "יְתִיב".to_string(),
                bhs_meaning:"resting or sitting".to_string(),
                alt_name: Some("(Shophar) yetiv".to_string()),
                alt_hebrew: Some("(שׁוֹפָר) יְתִיב".to_string()),
                alt_meaning: None,
                comment: Some("Short for Shofar (=horn) Yetiv".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::UnderPrePositive,
                position_codepoint2: None,
            },
            ProseAccent::Tevir => AccentInfo {
                bhs_name: "Tevir".to_string(),
                bhs_hebrew: "תְּבִיר".to_string(),
                bhs_meaning:"broken, downward tumble".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Geresh => AccentInfo {
                bhs_name: "Geresh".to_string(),
                bhs_hebrew: "גֵּרֵישׁ".to_string(),
                bhs_meaning:"expulsion, driving out, divorce".to_string(),
                alt_name: Some("Gerish".to_string()),
                alt_hebrew: Some("גְּרִישׁ".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Gershayim => AccentInfo {
                bhs_name: "Gershayim".to_string(),
                bhs_hebrew: "גֵּרְשַׁיִם".to_string(),
                bhs_meaning:"double of expulsion, driving out, divorce".to_string(),
                alt_name: Some("Shene Gershayim".to_string()),
                alt_hebrew: Some("שְׁנֵי גֵּרְשַׁיִם".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Pazer => AccentInfo {
                bhs_name: "Pazer".to_string(),
                bhs_hebrew: "פָּזֶר".to_string(),
                bhs_meaning:"lavish or scatter".to_string(),
                alt_name: Some("Pazer Qatan".to_string()),
                alt_hebrew: Some("פָּזֵר קָטָן".to_string()),
                alt_meaning: Some("small lavish or scatter".to_string()),
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::PazerGadol => AccentInfo {
                bhs_name: "Pazer Gadol".to_string(),
                bhs_hebrew: "פָּזֶר גּדוֹל".to_string(),
                bhs_meaning:"large lavish or scatter".to_string(),
                alt_name: Some("Qarne Pharah".to_string()),
                alt_hebrew: Some("קַרְנֵי פָרָה".to_string()),
                alt_meaning: Some("horns of a cow".to_string()),
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::TelishaGedolah => AccentInfo {
                bhs_name: "Telisha Gedolah".to_string(),
                bhs_hebrew: "תְּלִישָׁא גְּדוֹלָה".to_string(),
                bhs_meaning:"great (long) detached".to_string(),
                alt_name: Some("Tirtsah".to_string()),
                alt_hebrew: Some("תִּרְצָה".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePrePositive,
                position_codepoint2: None,
            },
            ProseAccent::Legarmeh => AccentInfo {
                bhs_name: "Legarmeh".to_string(),
                bhs_hebrew: "לְגַרְמֶהּ".to_string(),
                bhs_meaning:"for or by itself, independant".to_string(),
                alt_name: Some("Munnach Legarmeh".to_string()),
                alt_hebrew: Some("מוּנַח לְגַרְמֵ֣הּ".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: Some(AccentPosition::End),
            },
            // Conjunctives
            ProseAccent::Munnach => AccentInfo {
                bhs_name: "Munnach".to_string(),
                bhs_hebrew: "מֻנַּח".to_string(),
                bhs_meaning:"resting or placed".to_string(),
                alt_name: Some("Shofar Holech".to_string()),
                alt_hebrew: Some("שׁוֹפָר הוֹלֵךְ".to_string()),
                alt_meaning: None,
                comment: Some("because it is shaped like a horn (Shofar) lying on its side".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Mahpakh => AccentInfo {
                bhs_name: "Mahpakh".to_string(),
                bhs_hebrew: "מַהְפַּךְ".to_string(),
                bhs_meaning:"turning round".to_string(), //OK
                alt_name: Some("(Schofar) Mehuppakh".to_string()),
                alt_hebrew: Some("שׁוֹפָר מְהֻפָּ֤ךְ".to_string()),
                alt_meaning: Some("reversed (horn)".to_string()),
                comment: Some("The form is an inverted Shofar".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Merkha => AccentInfo {
                bhs_name: "Merkha".to_string(),
                bhs_hebrew: "מֵרְכָא".to_string(),
                bhs_meaning:"lengthener, prolonging".to_string(),//OK
                alt_name: Some("Ma'arich".to_string()),
                alt_hebrew: Some("מַאֲרִ֥יךְ".to_string()),
                alt_meaning: None,
                comment:  Some("rod or stroke".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::MerkhaKephulah => AccentInfo {
                bhs_name: "Merkha Kephulah".to_string(),
                bhs_hebrew: "מֵרְכָא כְּפוּלָה".to_string(),
                bhs_meaning:"double lengthener".to_string(),
                alt_name: Some("Tere ta’ame".to_string()),
                alt_hebrew: Some("תְּרֵי טַעֲמֵ֦י".to_string()),
                alt_meaning: None,
                comment:  Some("two rods or strokes".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Darga => AccentInfo {
                bhs_name: "Darga".to_string(),
                bhs_hebrew: "דַּרְגָּא".to_string(),
                bhs_meaning:"stairstep".to_string(), // OK
                alt_name: Some("Dirjo".to_string()),
                alt_hebrew: Some("דִּרְגָּ֧א".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Azla => AccentInfo { //OK
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא".to_string(),
                bhs_meaning:"going on (not pausing), depart".to_string(),
                alt_name: Some("Qadma".to_string()),
                alt_hebrew: Some("קַדְמָא".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::TelishaQetannah => AccentInfo {
                bhs_name: "Telisha Qetannah".to_string(),
                bhs_hebrew: "תְּלִישָא קְטַנָּה".to_string(),
                bhs_meaning:"small (short) detached".to_string(),
                alt_name: Some("Talscha".to_string()),
                alt_hebrew: Some("תַּלְשָׁא".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Galgal => AccentInfo {
                bhs_name: "Galgal".to_string(),
                bhs_hebrew: "גַּלְגַּל".to_string(),
                bhs_meaning:"wheel, circle".to_string(),
                alt_name: Some("Jerach Ben Jomo".to_string()),
                alt_hebrew: Some("יֵרֶח בֶּן יוֹמוֹ".to_string()),
                alt_meaning: Some("moon one day old".to_string()),
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Mayela => AccentInfo {
                bhs_name: "Mayela".to_string(),
                bhs_hebrew: "מָאיְלָא".to_string(),
                bhs_meaning:"to be raised or elevated".to_string(),
                alt_name: Some("Meayyela".to_string()),
                alt_hebrew: Some("מְאַיְּלָא".to_string()),
                alt_meaning: None,
                comment: Some("Name given to a Tiphcha, when in the same word as Atnach or Silluq".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Secondary,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Meteg => AccentInfo {
                bhs_name: "Meteg".to_string(),
                bhs_hebrew: "מֶתֶג".to_string(),
                bhs_meaning:"accent or mark".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Secondary,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
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
    fn info(&self) -> AccentInfo {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => AccentInfo {
                bhs_name: "Silluq".to_string(),
                bhs_hebrew: "סִלּוּק".to_string(),
                bhs_meaning:"close, cessation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::OleWeYored => AccentInfo {
                bhs_name: "Ole We Yored".to_string(),
                bhs_hebrew: "עוֹלֶה וְיוֹרֵד".to_string(),
                bhs_meaning:"ascending and descending".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Under),
            },
            PoetryAccent::Atnach => AccentInfo {
                bhs_name: "Atnach".to_string(),
                bhs_hebrew: "אַתְנָח".to_string(),
                bhs_meaning:"pause, rest".to_string(), //OK
                alt_name: Some("Etnachta".to_string()),
                alt_hebrew: Some("אֶתְנַחְתָּא".to_string()),
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaGadol => AccentInfo {
                bhs_name: "Revia Gadol".to_string(),
                bhs_hebrew: "רְבִיעַ גּדוֹל".to_string(),
                bhs_meaning:"big fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaMugrash => AccentInfo {
                bhs_name: "Revia Mugrash".to_string(),
                bhs_hebrew: "רְבִיעַ מֻגְרָשׁ".to_string(),
                bhs_meaning:"exiled fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("???".to_string()),
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Above),
            },
            PoetryAccent::ShalsheletGadol => AccentInfo {
                bhs_name: "Shalshelet Gadol".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת גָּדוֹל".to_string(),
                bhs_meaning:"large chain or link".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            PoetryAccent::Tsinnor => AccentInfo {
                bhs_name: "Tsinnor".to_string(),
                bhs_hebrew: "צִנּוֹר".to_string(),
                bhs_meaning:"pipe or tube".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaQaton => AccentInfo {
                bhs_name: "Revia Qaton".to_string(),
                bhs_hebrew: "רְבִיעַ קָטוֹן".to_string(),
                bhs_meaning:"small fourth".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Dechi => AccentInfo {
                bhs_name: "Dechi".to_string(),
                bhs_hebrew: "דֶּחִי".to_string(),
                bhs_meaning:"to push or drive away".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::UnderPrePositive,
                position_codepoint2: None,
            },
            PoetryAccent::Pazer => AccentInfo {
                bhs_name: "Pazer".to_string(),
                bhs_hebrew: "פָּזֵר".to_string(),
                bhs_meaning:"lavish or scatter".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::MehuppakhLegarmeh => AccentInfo {
                bhs_name: "Mehuppakh Legarmeh".to_string(),
                bhs_hebrew: "מְהֻפָּךְ לְגַרְמֵהּ".to_string(),
                bhs_meaning:"reversed to its own".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: Some(AccentPosition::End),
            },
            PoetryAccent::AzlaLegarmeh => AccentInfo {
                bhs_name: "Azla Legarmeh".to_string(),
                bhs_hebrew: "אַזְלָא לְגַרְמֶהּ".to_string(),
                bhs_meaning:"goes to its own".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Disjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            // Conjunctives
            PoetryAccent::Munnach => AccentInfo {
                bhs_name: "Munnach".to_string(),
                bhs_hebrew: "מֻנַּח".to_string(),
                bhs_meaning:"rest or placed".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Merkha => AccentInfo {
                bhs_name: "Merkha".to_string(),
                bhs_hebrew: "מֵרְכָא".to_string(),
                bhs_meaning:"lengthener".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Illuy => AccentInfo {
                bhs_name: "Illuy".to_string(),
                bhs_hebrew: "עִלּוּי".to_string(),
                bhs_meaning:"elevation or raising".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Tarkha => AccentInfo {
                bhs_name: "Tarkha".to_string(),
                bhs_hebrew: "טַרְחָא".to_string(),
                bhs_meaning:"to be drawn out or to be extended".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Galgal => AccentInfo {
                bhs_name: "Galgal".to_string(),
                bhs_hebrew: "גַּלְגַּל".to_string(),
                bhs_meaning:"circle".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Mehuppakh => AccentInfo {
                bhs_name: "Mehuppakh".to_string(),
                bhs_hebrew: "מְהֻפַּך".to_string(),
                bhs_meaning:"reversed or turned around".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Azla => AccentInfo {
                bhs_name: "Azla".to_string(),
                bhs_hebrew: "אַזְלָא֙".to_string(),
                bhs_meaning:"going away".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::ShalsheletQetannah => AccentInfo {
                bhs_name: "Shalshelet Qetannah".to_string(),
                bhs_hebrew: "שַׁלְשֶׁלֶת קְטַנָּה".to_string(),
                bhs_meaning:"small chain".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: None,
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::TsinnoritMerkha => AccentInfo {
                bhs_name: "Tsinnorit Merkha".to_string(),
                bhs_hebrew: "צִנּוֹרִת מֵרְכָא".to_string(),
                bhs_meaning:"pipe of continuation".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Merkha Metsunneret".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Under),
            },
            PoetryAccent::TsinnoritMahpakh => AccentInfo {
                bhs_name: "Tsinnorit Mahpakh".to_string(),
                bhs_hebrew: "צִנּוֹרִת מַהְפַּךְ".to_string(),
                bhs_meaning:"pipe of reversal".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Mahpakh Metsunnar".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Primairy,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Under),
            },
            PoetryAccent::Meteg => AccentInfo {
                bhs_name: "Meteg".to_string(),
                bhs_hebrew: "מֶתֶג".to_string(),
                bhs_meaning:"accent or mark".to_string(),
                alt_name: None,
                alt_hebrew: None,
                alt_meaning: None,
                comment: Some("??? Mahpakh Metsunnar".to_string()),
                category: AccentCategory::Conjunctive,
                acc_type: AccentType::Secondary,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
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
        // let _info = a.info();
    }
    #[test]
    fn no_test_just_print_info() {
        println!("\n{:#?}", ProseAccent::Silluq.info());
        println!("\n{:#?}", ProseAccent::Atnach.info());
        println!("\n{:#?}", ProseAccent::Segolta.info());
        println!("\n{:#?}", ProseAccent::Shalshelet.info());
        println!("\n{:#?}", ProseAccent::ZaqephQaton.info());
        println!("\n{:#?}", ProseAccent::ZaqephGadol.info());
        println!("\n{:#?}", ProseAccent::Revia.info());
        println!("\n{:#?}", ProseAccent::Tiphcha.info());
        println!("\n{:#?}", ProseAccent::Zarqa.info());
        println!("\n{:#?}", ProseAccent::Pashta.info());
        println!("\n{:#?}", ProseAccent::Yetiv.info());
        println!("\n{:#?}", ProseAccent::Tevir.info());
        println!("\n{:#?}", ProseAccent::Geresh.info());
        println!("\n{:#?}", ProseAccent::Gershayim.info());
        println!("\n{:#?}", ProseAccent::Pazer.info());
        println!("\n{:#?}", ProseAccent::PazerGadol.info());
        println!("\n{:#?}", ProseAccent::TelishaGedolah.info());
        println!("\n{:#?}", ProseAccent::Legarmeh.info());
        // Conjunctives
        println!("\n{:#?}", ProseAccent::Munnach.info());
        println!("\n{:#?}", ProseAccent::Mahpakh.info());
        println!("\n{:#?}", ProseAccent::Merkha.info());
        println!("\n{:#?}", ProseAccent::MerkhaKephulah.info());
        println!("\n{:#?}", ProseAccent::Darga.info());
        println!("\n{:#?}", ProseAccent::Azla.info());
        println!("\n{:#?}", ProseAccent::TelishaQetannah.info());
        println!("\n{:#?}", ProseAccent::Galgal.info());
        println!("\n{:#?}", ProseAccent::Mayela.info());
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
    fn test_accent_info() {
        let pa = ProseAccent::Galgal;
        assert_eq!("wheel, circle".to_string(), pa.info().bhs_meaning);
    }
}
