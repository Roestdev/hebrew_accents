/// Optional alternate naming for an accent.
/// Absence of an alternate naming is expressed via `Option<AlternateNames>`.
///
/// For some accents the *Biblia Hebraica Stuttgartensia* (BHS)
/// indicates an alternate spelling or name variant.
///
/// The following accents have alternatives documented in the BHS:
///
/// | listed       | alternative        |
/// | ------------ | ------------------ |
/// | Geresh       | Teres              |
/// | Pazer Gadol  | Qarne Pharah       |
/// | Mahpakh      | Mehuppakh          |
/// | Azla         | Teres              |
/// | Galgal       | Yerach Ben Yomo    |
/// | Meayla       | Mayla              |
/// | Meteg        | Gayah              |
/// | Oleh Weyored | Mahpakh and Merkha |
/// | Tsinnor      | Zarqa              |
/// | Dechi        | Tiphcha            |
/// | Mehuppakh    | Mahpakh            |
///
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

#[cfg(test)]
mod tests {}
