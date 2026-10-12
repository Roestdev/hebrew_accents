use crate::api::AlternateNames;

/// Optional alternate naming for an accent in the BHS
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct InternalAlternateNames {
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according the file `TRANSLITERATION.md`
    pub(crate) english_name: &'static str,
    /// Transliterated according `SBL Academic`
    pub(crate) sbl_academic: &'static str,
}

// Conversion from internal to public type
impl From<InternalAlternateNames> for AlternateNames {
    fn from(privnames: InternalAlternateNames) -> Self {
        AlternateNames {
            hebrew_name: privnames.hebrew_name,
            hebrew_concept: privnames.hebrew_concept,
            english_name: privnames.english_name,
            sbl_academic: privnames.sbl_academic,
        }
    }
}
