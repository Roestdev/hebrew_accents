use crate::accent::Accent;

/// All variants of the Hebrew Prose Accents
/// 18 Disjunctives and 11 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum ProseAccent {
    #[default]
    /// Primary disjunctive prose accent Silluq
    Silluq,
    /// Primary disjunctive prose accent accent Atnach
    Atnach,
    /// Primary disjunctive prose accent Segolta
    Segolta,
    /// Primary disjunctive prose accent Shalshelet
    Shalshelet,
    /// Primary disjunctive prose accent Zaqeph Qaton
    ZaqephQatan,
    /// Primary disjunctive prose accent Zaqeph Gadol
    ZaqephGadol,
    /// Primary disjunctive prose accent Revia
    Revia,
    /// Primary disjunctive prose accent Tiphcha,
    Tiphcha,
    /// Primary disjunctive prose accent Zarqa
    Zarqa,
    /// Primary disjunctive prose accent Pashta
    Pashta,
    /// Primary disjunctive prose accent Yetiv
    Yetiv,
    /// Primary disjunctive prose accent Tevir
    Tevir,
    /// Primary disjunctive prose accent Geresh
    Geresh,
    /// Primary disjunctive prose accent Gershayim
    Gershayim,
    /// Primary disjunctive prose accent Pazer
    Pazer,
    /// Primary disjunctive prose accent Pazer Gadol
    PazerGadol,
    /// Primary disjunctive prose accent Telisha Gedolah
    TelishaGedolah,
    /// Primary disjunctive prose accent Legarmeh
    Legarmeh,
    /// Primary conjunctive prose accent Munach
    Munach,
    /// Primary conjunctive prose accent Mahpakh
    Mahpakh,
    /// Primary conjunctive prose accent Merkha
    Merkha,
    /// Primary conjunctive prose accent Merkha Kephulah
    MerkhaKephulah,
    /// Primary conjunctive prose accent Darga
    Darga,
    /// Primary conjunctive prose accent Azla
    Azla,
    /// Primary conjunctive prose accent Telisha Qetannah
    TelishaQetannah,
    /// Primary conjunctive prose accent Galgal
    Galgal,
    /// Secondary conjunctive prose accent Mayela
    Mayela,
    /// Secondary conjunctive prose accent Meteg
    Meteg,
}

impl ProseAccent {
    /// The total number of prose accents
    pub const LEN: usize = 28;
    /// Indication of how an strong an accent is (relative speaking)
    ///
    /// The stronger, the longer the pause/break when reading
    /// The strongest accent has a relative strength of 1
    ///
    /// Note: For now all Hebrew Accents have this property
    ///       However it is only valid for DISJUNCTIVE accents!
    #[inline]
    pub fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
}

impl std::fmt::Display for ProseAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}), meaning: {}",
            self.english_name(),
            self.hebrew_name(),
            self.hebrew_concept()
        )
    }
}
