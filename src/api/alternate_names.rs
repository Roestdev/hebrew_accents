use crate::accent::PrivAlternateNames;

/// Optional alternate representations for an accent.
///
/// For some accents the `Biblia Hebraica Stuttgartensia` (BHS)
/// indicates an alternate spelling or name variant.
///
/// The following accents have alternatives documented:
/// | listed | alternative
/// | ----- | ------|
/// | Geresh | Teres |
/// | Pazer Gadol | Qarne Pharah |
/// | Mahpakh | Mehuppakh |
/// | Azla | Teres |
/// | Galgal | Jerach Ben Jomo |
/// | Meayla | Mayla |
/// | Meteg | Gayah |
/// | Oleh Weyored | Mahpakh and Merkha |
/// | Tsinnor | Zarqa |
/// | Dechi | Tiphcha |
/// | Mehuppakh | Mahpakh |
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlternateNames {
    /// Hebrew name of the accent
    pub hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub hebrew_concept: &'static str,
    /// Transliterated according the file `TRANSLITERATION.md`
    pub english_name: &'static str,
    /// Transliterated according `SBL Academic`
    pub sbl_academic: &'static str,
}

impl From<PrivAlternateNames> for AlternateNames {
    fn from(internal: PrivAlternateNames) -> Self {
        AlternateNames {
            hebrew_name: internal.hebrew_name,
            hebrew_concept: internal.hebrew_concept,
            english_name: internal.english_name,
            sbl_academic: internal.sbl_academic,
        }
    }
}
