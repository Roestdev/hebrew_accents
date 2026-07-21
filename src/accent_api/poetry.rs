use crate::accent_data::BHS_POETRY_RANK_MAP;
use crate::Accent;

/// All variants of the Hebrew Poetry Accents
/// 12 Disjunctives and 12 Conjunctives.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
#[non_exhaustive]
pub enum PoetryAccent {
    #[default]
    /// Primary disjunctive poetry accent Silluq
    Silluq,
    /// Primary disjunctive poetry accent Oleh Weyored
    OlehWeYored,
    /// Primary disjunctive poetry accent Atnach
    Atnach,
    /// Primary disjunctive poetry accent Revia Gadol
    ReviaGadol,
    /// Primary disjunctive poetry accent Revia Mugrash,
    ReviaMugrash,
    /// Primary disjunctive poetry accent ShalsheletGadol
    ShalsheletGadol,
    /// Primary disjunctive poetry accent Tsinnor
    Tsinnor,
    /// Primary disjunctive poetry accent Revia Qaton
    ReviaQaton,
    /// Primary disjunctive poetry accent Dechi,
    Dechi,
    /// Primary disjunctive poetry accent Pazer
    Pazer,
    /// Primary disjunctive poetry accent MehuppakhLegarmeh
    MehuppakhLegarmeh,
    /// Primary disjunctive poetry accent AzlaLegarmeh
    AzlaLegarmeh,
    /// Primary conjunctive poetry accent Munach
    Munach,
    /// Primary conjunctive poetry accent Merkha
    Merkha,
    /// Primary conjunctive poetry accent Illuy,
    Illuy,
    /// Primary conjunctive poetry accent Tarcha
    Tarcha,
    /// Primary conjunctive poetry accent Galgal
    Galgal,
    /// Primary conjunctive poetry accent Mehuppakh
    Mehuppakh,
    /// Primary conjunctive poetry accent Azla
    Azla,
    /// Primary conjunctive poetry accent Shalshelet Qetannah
    ShalsheletQetannah,
    /// Primary conjunctive poetry accent Tsinnorit Merkha
    TsinnoritMerkha,
    /// Primary conjunctive poetry accent Tsinnorit Mahpakh
    TsinnoritMahpakh,
    /// Secondary conjunctive poetry accent Meteg
    Meteg,
}

impl PoetryAccent {
    /// Total count of all poetry accents,including some 'non-accents'
    pub const LEN: usize = 23;
    /// Indication of how an strong an accent is (relative speaking)
    ///
    /// The stronger, the longer the pause/break when reading
    /// The strongest accent has a relative strength of 1
    ///
    /// Note: For now all Hebrew Accents have this property
    ///       However it is only valid for DISJUNCTIVE accents!
    #[inline]
    pub fn relative_strength(self) -> u8 {
        // Discriminants start at 0; we want 1‑based relative_strengths.
        BHS_POETRY_RANK_MAP[self as usize]
    }
}

impl std::fmt::Display for PoetryAccent {
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
