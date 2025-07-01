#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)] //,Display,Default
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Display
pub enum ProseAccent {
    // Disjunctives
    SophPasuq,
    #[default]
    Silluq,
    Atnach,
    Segolta, //postpositive
    Shalshelet,
    ZaqephQatan,
    ZaqephGadol,
    Revia,
    Tiphcha,
    Zarqa,  //postpositive
    Pashta, //postpositive
    Yetiv,  //prepositive
    Tevir,
    Geresh,
    Gershayim,
    Pazer,
    PazerGadol,
    TelishaGedolah, //prepositive
    Legarmeh,
    // Conjunctives
    Paseq,
    Munnach,
    Mahpakh,
    Merkha,
    MerkhaKephulah,
    Darga,
    Azla,
    TelishaQetannah, //postpositive
    Galgal,
    Meayela,
    Maqqeph,
    Meteg,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Display
pub enum PoetryAccent {
    // Disjunctives
    SophPasuq,
    #[default]
    Silluq,
    OleWeYored,
    Atnach,
    ReviaGadol,
    ReviaMugrash,
    ShalsheletGadol,
    Tsinnor, // postpositive
    ReviaQaton,
    Dechi, // prepositive
    Pazer,
    MahpakhLegarmeh,
    AzlaLegarmeh,
    // Conjunctives
    Paseq,
    Munnach,
    Merkha,
    Illuy,
    Tarkha,
    Galgal,
    Mahpakh,
    Azla,
    ShalsheletQetannah,
    TsinnoritMerkha,
    TsinnoritMahpakh,
    Maqqeph,
    Meteg,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Copy, Display
pub struct AccentInfo {
    name: String,
    hebrew: String,
    meaning: Option<String>,
    seph_name: Option<String>,
    seph_hebrew_name: Option<String>,
    comment: Option<String>,
    accent_type: AccentType,
    nr_of_code_points: AccentCodePoints,
    position_codepoint1: AccentPosition,
    position_codepoint2: Option<AccentPosition>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Display
pub enum AccentType {
    Conjunctive,
    #[default]
    Disjunctive,
    None
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Display
pub enum AccentPosition {
    #[default]
    Above,
    AbovePostPositive,
    AbovePrePositive,
    End, // used to denote a Paseq or Soph Pasuq
    Under,
    UnderPostPositive,
    UnderPrePositive,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)] //Display
pub enum AccentCodePoints {
    #[default]
    One,
    Two,
}

impl ProseAccent {
    #[allow(unused)]
    fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            ProseAccent::SophPasuq => 0,
            ProseAccent::Silluq => 1,
            ProseAccent::Atnach => 2,
            ProseAccent::Segolta => 3,
            ProseAccent::Shalshelet => 4,
            ProseAccent::ZaqephQatan => 5,
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
            ProseAccent::Meayela => 27,
            ProseAccent::Paseq => 101,
            ProseAccent::Maqqeph => 102,
            ProseAccent::Meteg => 103,
        }
    }
    /// Returns information about the accent.
    /// This can be expanded to include more details as needed.
    #[allow(unused)]
    fn info(&self) -> AccentInfo {
        match self {
            // Disjunctives
            ProseAccent::SophPasuq => AccentInfo {
                name: "Soph Pasuq".to_string(),
                hebrew: "סוֹף פָּסֽוּק".to_string(),
                meaning: Some("End of verse".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("End of verse".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::End,
                position_codepoint2: None,
            },
            ProseAccent::Silluq => AccentInfo {
                name: "Silluq".to_string(),
                hebrew: "סִילֽוּק".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Atnach => AccentInfo {
                name: "Atnach".to_string(),
                hebrew: "אֶתְנַחְתָּ֑א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Segolta => AccentInfo {
                name: "Segolta".to_string(),
                hebrew: "סֶגּוֹל֒".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Shalshelet => AccentInfo {
                name: "Shalshelet".to_string(),
                hebrew: "שַׁלְשֶׁ֓לֶת".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            ProseAccent::ZaqephQatan => AccentInfo {
                name: "Zaqeph Qatan".to_string(),
                hebrew: "זָקֵף קָט֔וֹן".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::ZaqephGadol => AccentInfo {
                name: "Zaqeph Gadol".to_string(),
                hebrew: "זָקֵף גּד֕וֹל".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Revia => AccentInfo {
                name: "Revia".to_string(),
                hebrew: "רְבִ֗יע".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Tiphcha => AccentInfo {
                name: "Tiphcha".to_string(),
                hebrew: "טִפְחָ֖א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Zarqa => AccentInfo {
                name: "Zarqa".to_string(),
                hebrew: "זַרְקָא֘".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Pashta => AccentInfo {
                name: "Pashta".to_string(),
                hebrew: "פַּשְׁטָא֙".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Yetiv => AccentInfo {
                name: "Yetiv".to_string(),
                hebrew: "יְ֚תִב".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::UnderPrePositive,
                position_codepoint2: None,
            },
            ProseAccent::Tevir => AccentInfo {
                name: "Tevir".to_string(),
                hebrew: "תְּבִ֛יר".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Geresh => AccentInfo {
                name: "Geresh".to_string(),
                hebrew: "גְּרִ֜ישׁ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Gershayim => AccentInfo {
                name: "Gershayim".to_string(),
                hebrew: "גְּרִישִׁ֞יִם".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::Pazer => AccentInfo {
                name: "Pazer".to_string(),
                hebrew: "פָּזֶר".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::PazerGadol => AccentInfo {
                name: "Qarne Pharah".to_string(),
                hebrew: "קַרְנֵ פָרָ֟ה".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::TelishaGedolah => AccentInfo {
                name: "Telisha Gedolah".to_string(),
                hebrew: "תַּלִישָׁא גְּ֠דוֹלָה".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePrePositive,
                position_codepoint2: None,
            },
            ProseAccent::Legarmeh => AccentInfo {
                name: "Legarmeh".to_string(),
                hebrew: "לְגַרְמֵהּ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: Some(AccentPosition::End),
            },
            // Conjunctives
            ProseAccent::Munnach => AccentInfo {
                name: "Munnach".to_string(),
                hebrew: "מֻנַּ֣ח".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Mahpakh => AccentInfo {
                name: "Mahpakh".to_string(),
                hebrew: "מַהְפַּ֤ךְ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Merkha => AccentInfo {
                name: "Merkha".to_string(),
                hebrew: "מֵרְכָ֥א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::MerkhaKephulah => AccentInfo {
                name: "Merkha Kephulah".to_string(),
                hebrew: "מֵרְכָא כְּפוּלָ֦ה".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Darga => AccentInfo {
                name: "Darga".to_string(),
                hebrew: "דַּרְגָּ֧א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Azla => AccentInfo {
                name: "Azla".to_string(),
                hebrew: "קַדְמָ֨א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            ProseAccent::TelishaQetannah => AccentInfo {
                name: "Telisha Qetannah".to_string(),
                hebrew: "תְּ֠לִישָא קְטַנָּה".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            ProseAccent::Galgal => AccentInfo {
                name: "Galgal".to_string(),
                hebrew: "גַּלְגַּל".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Meayela => AccentInfo {
                name: "Meayela".to_string(),
                hebrew: "מְאַיְלָא".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Paseq => AccentInfo {
                name: "Paseq".to_string(),
                hebrew: "פָּסֵ֣ק".to_string(),
                meaning: Some("pause or break)".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("Follows an accent (-> multi character accent)".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Maqqeph => AccentInfo {
                name: "Maqqeph".to_string(),
                hebrew: "מַקֵּף".to_string(),
                meaning: Some("binder".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("".to_string()),
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            ProseAccent::Meteg => AccentInfo {
                name: "Meteg".to_string(),
                hebrew: "מֶתֶג".to_string(),
                meaning: Some("bridle".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("secondary stress".to_string()),
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
        }
    }
}

impl PoetryAccent {
    #[allow(unused)]
    fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            PoetryAccent::SophPasuq => 0,
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
            PoetryAccent::MahpakhLegarmeh => 11,
            PoetryAccent::AzlaLegarmeh => 12,
            // Conjunctives
            PoetryAccent::Munnach => 13,
            PoetryAccent::Merkha => 14,
            PoetryAccent::Illuy => 15,
            PoetryAccent::Tarkha => 16,
            PoetryAccent::Galgal => 17,
            PoetryAccent::Mahpakh => 18,
            PoetryAccent::Azla => 19,
            PoetryAccent::ShalsheletQetannah => 20,
            PoetryAccent::TsinnoritMerkha => 21,
            PoetryAccent::TsinnoritMahpakh => 21,
            //
            PoetryAccent::Paseq => 101,
            PoetryAccent::Maqqeph => 102,
            PoetryAccent::Meteg => 103,
        }
    }
    #[allow(unused)]
    fn info(&self) -> AccentInfo {
        match self {
            // Disjunctives
            PoetryAccent::SophPasuq => AccentInfo {
                name: "Soph Pasuq".to_string(),
                hebrew: "סוֹף פָּסֽוּק".to_string(),
                meaning: Some("End of verse".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("End of verse".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::End,
                position_codepoint2: None,
            },
            PoetryAccent::Silluq => AccentInfo {
                name: "Silluq".to_string(),
                hebrew: "סִילֽוּק".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::OleWeYored => AccentInfo {
                name: "Ole We Yored".to_string(),
                hebrew: "עוֹלֶה וְיוֹרֵד".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Under),
            },
            PoetryAccent::Atnach => AccentInfo {
                name: "Atnach".to_string(),
                hebrew: "אֶתְנַחְתָּ֑א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaGadol => AccentInfo {
                name: "Revia Gadol".to_string(),
                hebrew: "גּד֕וֹל רְבִ֗יע ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaMugrash => AccentInfo {
                name: "Revia Mugrash".to_string(),
                hebrew: "רְבִיעַ מֻגְרָשׁ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("???".to_string()),
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::Above),
            },
            PoetryAccent::ShalsheletGadol => AccentInfo {
                name: "Shalshelet Gadol".to_string(),
                hebrew: "גּד֕וֹל שַׁלְשֶׁ֓לֶת".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            PoetryAccent::Tsinnor => AccentInfo {
                name: "Tsinnor".to_string(),
                hebrew: "צִנּוֹר".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::AbovePostPositive,
                position_codepoint2: None,
            },
            PoetryAccent::ReviaQaton => AccentInfo {
                name: "Revia Qaton".to_string(),
                hebrew: "רְבִ֗יעַ קָט֔וֹן".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Dechi => AccentInfo {
                name: "Dechi".to_string(),
                hebrew: "דֶּחִי֭".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::UnderPrePositive,
                position_codepoint2: None,
            },
            PoetryAccent::Pazer => AccentInfo {
                name: "Pazer".to_string(),
                hebrew: "פָּזֵ֡ר".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::MahpakhLegarmeh => AccentInfo {
                name: "Mahpakh Legarmeh".to_string(),
                hebrew: "מַהְפָּךְ לְגַרְמֵהּ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: Some(AccentPosition::End),
            },
            PoetryAccent::AzlaLegarmeh => AccentInfo {
                name: "Azla Legarmeh".to_string(),
                hebrew: "אַזְלָא לְגַרְמֶהּ".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Disjunctive,
                nr_of_code_points: AccentCodePoints::Two,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: Some(AccentPosition::End),
            },
            // Conjunctives
            PoetryAccent::Munnach => AccentInfo {
                name: "Munnach".to_string(),
                hebrew: "מֻנַּ֣ח".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Merkha => AccentInfo {
                name: "Merkha".to_string(),
                hebrew: "מֵירְכָא".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Illuy => AccentInfo {
                name: "Illuy".to_string(),
                hebrew: "עִלּוּי".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Tarkha => AccentInfo {
                name: "Tarkha".to_string(),
                hebrew: "טַרְחָא".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Galgal => AccentInfo {
                name: "Galgal".to_string(),
                hebrew: "גַּלְגַּל".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Mahpakh => AccentInfo {
                name: "Mahpakh".to_string(),
                hebrew: "מַהְפַּ֤ך".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Under,
                position_codepoint2: None,
            },
            PoetryAccent::Azla => AccentInfo {
                name: "Azla".to_string(),
                hebrew: "קַדְמָ֨א".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::ShalsheletQetannah => AccentInfo {
                name: "Shalshelet Qetannah".to_string(),
                hebrew: "שַׁלְשֶׁלֶת קְטַנָּה".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: None,
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::TsinnoritMerkha => AccentInfo {
                name: "Tsinnorit Merkha".to_string(),
                hebrew: "מֵרְכָ֥א צִנּוֹרִת֘".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("??? Merkha Metsunneret".to_string()),
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::TsinnoritMahpakh => AccentInfo {
                name: "Tsinnorit Mahpakh".to_string(),
                hebrew: "מְהֻפָּ֤ךְ צִנּוֹרִת֘".to_string(),
                meaning: None,
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("??? Mahpakh Metsunnar".to_string()),
                accent_type: AccentType::Conjunctive,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Paseq => AccentInfo {
                name: "Paseq".to_string(),
                hebrew: "פָּסֵ֣ק".to_string(),
                meaning: Some("pause or break)".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("Follows an accent (-> multi character accent)".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Maqqeph => AccentInfo {
                name: "Maqqeph".to_string(),
                hebrew: "מַקֵּף".to_string(),
                meaning: Some("binder".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
                position_codepoint2: None,
            },
            PoetryAccent::Meteg => AccentInfo {
                name: "Meteg".to_string(),
                hebrew: "מֶתֶג".to_string(),
                meaning: Some("bridle".to_string()),
                seph_name: None,
                seph_hebrew_name: None,
                comment: Some("secondary stress".to_string()),
                accent_type: AccentType::None,
                nr_of_code_points: AccentCodePoints::One,
                position_codepoint1: AccentPosition::Above,
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
        println!("\n{:#?}", ProseAccent::ZaqephQatan.info());
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
        println!("\n{:#?}", ProseAccent::Meayela.info());
    }
    #[test]
    fn no_test_just_print_rank() {
        println!("\n{:#?}", ProseAccent::Silluq.rank());
        println!("\n{:#?}", ProseAccent::Atnach.rank());
        println!("\n{:#?}", ProseAccent::Segolta.rank());
        println!("\n{:#?}", ProseAccent::Shalshelet.rank());
        println!("\n{:#?}", ProseAccent::ZaqephQatan.rank());
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
        println!("\n{:#?}", ProseAccent::Meayela.rank());
    }
}
