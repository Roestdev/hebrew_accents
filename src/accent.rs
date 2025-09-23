use crate::accentinformation::*;

/// Hebrew Accent, either Prose or Poetry
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HebrewAccent {
    Prose(ProseAccent),
    Poetry(PoetryAccent),
}

/// All variants of the Hebrew Prose Accents
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum ProseAccent {
    // 18 Disjunctives
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
    // 11 Conjunctives
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
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub enum PoetryAccent {
    // 12 Disjunctives
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
    // 12 Conjunctives
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
pub struct AccentInfo {
    pub english_name: &'static str,
    pub hebrew_name: &'static str,
    pub meaning: &'static str,
    pub alt_english_name: Option<&'static str>,
    pub alt_hebrew_name: Option<&'static str>,
    pub alt_meaning: Option<&'static str>,
    pub first_code_point: &'static Utf8CodePointInfo,
    pub second_code_point: Option<&'static Utf8CodePointInfo>,
    pub comment: Option<&'static str>,
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

pub trait Accent: Copy + Sized {
    /// Return the *static* metadata for this concrete accent.
    fn info(self) -> &'static AccentInfo;

    // Convenience wrappers that expose the fields you used most.
    // #[inline] fn rank(self)        -> u8                 { self.details().rank }
    //#[inline] fn category(self)    -> AccentCategory     { self.details().category }
    //#[inline] fn accent_type(self) -> AccentType        { self.details().accent_type }
    //#[inline] fn details(self)     -> &'static AccentInfo {
    //    self.details().details
    //}
}

impl Accent for HebrewAccent {
    #[inline]
    fn info(self) -> &'static AccentInfo {
        match self {
            HebrewAccent::Prose(p) => p.info(),
            HebrewAccent::Poetry(p) => p.info(),
        }
    }
}

impl Accent for ProseAccent {
    #[inline]
    fn info(self) -> &'static AccentInfo {
        match self {
            ProseAccent::Silluq => &SILLUQ_INFO,
            ProseAccent::Atnach => &ATNACH_INFO,
            ProseAccent::Segolta => &SEGOLTA_INFO,
            ProseAccent::Shalshelet => &SHALSHELET_INFO,
            ProseAccent::ZaqephQaton => &ZAQEF_QATON_INFO,
            ProseAccent::ZaqephGadol => &ZAQEPH_GADOL_INFO,
            ProseAccent::Revia => &REVIA_INFO,
            ProseAccent::Tiphcha => &TIPHCHA_INFO,
            ProseAccent::Zarqa => &ZARQA_INFO,
            ProseAccent::Pashta => &PASHTA_INFO,
            ProseAccent::Yetiv => &YETIV_INFO,
            ProseAccent::Tevir => &TEVIR_INFO,
            ProseAccent::Geresh => &GERESH_INFO,
            ProseAccent::Gershayim => &GERSHAYIM_INFO,
            ProseAccent::Pazer => &PAZER_INFO,
            ProseAccent::PazerGadol => &PAZER_GADOL_INFO,
            ProseAccent::TelishaGedolah => &TELISHA_GEDOLAH_INFO,
            ProseAccent::Legarmeh => &LEGARMEH_INFO,
            // Conjunctives
            ProseAccent::Munach => &MUNACH_INFO,
            ProseAccent::Mahpakh => &MAHPAKH_INFO,
            ProseAccent::Merkha => &MERKHA_INFO,
            ProseAccent::MerkhaKephulah => &MERKHA_KEFULAH_INFO,
            ProseAccent::Darga => &DARGA_INFO,
            ProseAccent::Azla => &AZLA_INFO,
            ProseAccent::TelishaQetannah => &TELISHA_QETANNAH_INFO,
            ProseAccent::Galgal => &GALGAL_INFO,
            ProseAccent::Mayela => &MAYELA_INFO,
            ProseAccent::Meteg => &METEG_INFO,
            ProseAccent::Maqqeph => &MAQQEPH_INFO,
        }
    }
}

impl Accent for PoetryAccent {
    #[inline]
    fn info(self) -> &'static AccentInfo {
        match self {
            // Disjunctives
            PoetryAccent::Silluq => &SILLUQ_INFO,
            PoetryAccent::OlehWeYored => &OLEH_WE_YORED_INFO,
            PoetryAccent::Atnach => &ATNACH_INFO,
            PoetryAccent::ReviaGadol => &REVIA_GADOL_INFO,
            PoetryAccent::ReviaMugrash => &REVIA_MUGRASH_INFO,
            PoetryAccent::ShalsheletGadol => &SHALSHELET_GADOL_INFO,
            PoetryAccent::Tsinnor => &TSINNOR_INFO,
            PoetryAccent::ReviaQaton => &REVIA_QATON_INFO,
            PoetryAccent::Dechi => &DECHI_INFO,
            PoetryAccent::Pazer => &PAZER_INFO,
            PoetryAccent::MehuppakhLegarmeh => &MEHUPPAKH_LEGARMEH_INFO,
            PoetryAccent::AzlaLegarmeh => &AZLA_LEGARMEH_INFO,
            // Conjunctives
            PoetryAccent::Munach => &MUNACH_INFO,
            PoetryAccent::Merkha => &MERKHA_INFO,
            PoetryAccent::Illuy => &ILLUY_INFO,
            PoetryAccent::Tarkha => &TARCHA_INFO,
            PoetryAccent::Galgal => &GALGAL_INFO,
            PoetryAccent::Mehuppakh => &MEHUPPAKH_INFO,
            PoetryAccent::Azla => &AZLA_INFO,
            PoetryAccent::ShalsheletQetannah => &SHALSHELET_QETANNAH_INFO,
            PoetryAccent::TsinnoritMerkha => &TSINNORIT_MERKHA_INFO,
            PoetryAccent::TsinnoritMahpakh => &TSINNORIT_MAHPAKH_INFO,
            PoetryAccent::Meteg => &METEG_INFO,
            PoetryAccent::Maqqeph => &MAQQEPH_INFO,
        }
    }
}

impl ProseAccent {
    pub const COUNT: usize = 29;
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
    // Returns detail information about the accent.
    // This can be expanded to include more details as needed.
    // #[allow(unused)]
}

impl PoetryAccent {
    pub const COUNT: usize = 24;
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
        println!("\n{:#?}", ProseAccent::Munach.info());
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
        assert_eq!("wheel, circle", pa.info().meaning);
    }
    #[test]
    fn test_poetry_accent_details() {
        let pa = PoetryAccent::Galgal;
        assert_eq!("wheel, circle", pa.info().meaning);
    }
    #[test]
    fn test_hebrew_accent_details() {
        let ha = HebrewAccent::Prose(ProseAccent::Galgal);
        assert_eq!("wheel, circle", ha.info().meaning);
    }
}
