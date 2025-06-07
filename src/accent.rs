#[allow(unused)]
pub enum AccentType {
    Disjunctive,
    Conjunctive,
}

#[allow(unused)]
pub enum NrOfCodePoints {
    One,
    Two,
}

#[allow(unused)]
pub enum HorPosition {
    Prepositive,  //located before the consonant
    Normal,       // located in the middle of the consonant
    Postpositive, // located after the consonant
}

#[allow(unused)]
pub enum VertPosition {
    Above, // located above the consonant
    Under, // located under the consonant
}

#[allow(unused)]
pub struct AccentInfo {
    name: String, // "pronounciation"
    accent_type: AccentType,
    // meaning: String,
    // seph_name: String, //Sephardi
    // seph_hebrew_name: String, //Sephardi
    // ashk_name: String, //Ashkenazi
    // ashk_hebrew_name: String, //Ashkenazi
    // comment:Option<String>,
    // nr_of_code_points: NrOfCodePoints,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProseAccent {  // 21 books
    // Disjunctives
    Silluq,
    Atnakh,  // Sephardi  of Atnach???
    Segolta, // Sephardi
    Shalshelet, // all
    ZaqephQaton, //Sephardi
    ZaqephGadol, // all
    Revia, // Ashkenazi
    Tiphkha, // Ashkenazi
    Zarqa, // all
    Pashta, // Ashkenazi
    Yetiv, // Ashkenazi
    Tevir, // all
    Geresh, // Teres
    Gershayim, // ????
    Pazer,  // Ashkenazi 
    PazerGadol, //Qarne Pharah (Fara); Pazer Gadol
    TelishaGedolah, // Ashkenazi
    Legarmeh,
    // Conjunctives
    Munakh,
    Mahpakh, // Ashkenazi
    Merekha,
    MerekhaKhephulah,
    Darga, // all
    Azla,
    TelishaQetanna,
    Galgal, //Yareach Ben Yomo,Jera
    Mayela,
}

impl ProseAccent {
    fn rank(&self) -> u8 {
        match self {
            ProseAccent::Silluq => 1,
            ProseAccent::Atnakh => 2,
            ProseAccent::Segolta => 3,
            ProseAccent::Shalshelet => 4,
            ProseAccent::ZaqephQaton => 5, // parvum
            ProseAccent::ZaqephGadol => 6, // magnum
            ProseAccent::Revia => 7,
            ProseAccent::Tiphkha => 8,
            ProseAccent::Zarqa => 9, // tsinnorit ??
            ProseAccent::Pashta => 10,
            ProseAccent::Yetiv => 11,
            ProseAccent::Tevir => 12,
            ProseAccent::Geresh => 13,
            ProseAccent::Gershayim => 14,
            ProseAccent::Pazer => 15,
            ProseAccent::PazerGadol => 16, // magnum
            ProseAccent::TelishaGedolah => 17, // magnum
            ProseAccent::Legarmeh => 18, // with Munakh 
            // Conjunctives
            ProseAccent::Munakh => 19,
            ProseAccent::Mahpakh => 20,
            ProseAccent::Merekha => 21,
            ProseAccent::MerekhaKhephulah => 22,
            ProseAccent::Darga => 23,
            ProseAccent::Azla => 24,
            ProseAccent::TelishaQetanna => 25, // parvum = small
            ProseAccent::Galgal => 26,
            ProseAccent::Mayela => 27,
        }
    }
    /// Returns information about the accent.
    /// This can be expanded to include more details as needed.
    fn info(&self) -> AccentInfo {
        match self {
            ProseAccent::Silluq => AccentInfo {
                name: "Silluq".to_string(),
                accent_type: AccentType::Disjunctive,
            },
            // Add information for other accents as needed
            // ProseAccent::Atnach => AccentInfo { name: "Atnach".to_string(), description: "Description of Atnach.".to_string() },
            // ...
            _ => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PoetryAccent {  // 3 books
    // Disjunctives
    Silluq,
    OleWeYored,
    Atnakh, // Sephardi
    ReviaGadol, // magnum
    ReviaMugrash,
    ShalsheletGadol, //magnum
    Tsinnor, // Zarqa
    ReviaQaton,//parvum
    Dechi, // all
    Pazer,
    MehuppakhLegarmeh,
    AzlaLegarmeh,
    // Conjunctives
    Munnakh, // Ashkenazi
    Merekha, // Ashkenazi
    Illuy, // all
    Tarkha, // Sephardi
    Galgal, // Jerah
    Mehuppakh,
    Azla, // Sephardi
    ShalsheletQetanna,// parvum
    TsinnoritMerekha,
    TsinnoritMehuppakh,
}

impl PoetryAccent {
    fn rank(&self) -> u8 {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => 1,
            PoetryAccent::OleWeYored => 2,
            PoetryAccent::Atnakh => 3,
            PoetryAccent::ReviaGadol => 4,
            PoetryAccent::ReviaMugrash => 5,
            PoetryAccent::ShalsheletGadol => 6,
            PoetryAccent::Tsinnor => 7, // Zarqa
            PoetryAccent::ReviaQaton => 8,
            PoetryAccent::Dechi => 9,
            PoetryAccent::Pazer => 10,
            PoetryAccent::MehuppakhLegarmeh => 11,
            PoetryAccent::AzlaLegarmeh => 12,
            // Conjunctives
            PoetryAccent::Munnakh => 13,
            PoetryAccent::Merekha => 14,
            PoetryAccent::Illuy => 15,
            PoetryAccent::Tarkha => 16,
            PoetryAccent::Galgal => 17,
            PoetryAccent::Mehuppakh => 18,
            PoetryAccent::Azla => 19,
            PoetryAccent::ShalsheletQetanna => 20,
            PoetryAccent::TsinnoritMerekha => 21,
            PoetryAccent::TsinnoritMehuppakh => 21,
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
}
