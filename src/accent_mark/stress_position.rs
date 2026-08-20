use crate::CantillationMarkStressPosition;

/// Tonic/stress relation — linguistic/metrical concept.
///
/// Describes which syllable bears the stress relative to the marked syllable.
/// This is INDEPENDENT from visual [`MarkPlacement`].
///
/// See: https://en.wikipedia.org/wiki/Tiberian_cantillation
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum StressPosition {
    /// Mark sits directly on the stressed syllable
    #[default]
    Impositive,
    /// Mark is on a syllable preceding the stressed one
    Prepositive,
    /// Mark is on a syllable following the stressed one
    Postpositive,
    /// Used for PseudoAccents
    NotApplicable,
}

impl StressPosition {
    pub(crate) const fn to_public(self) -> Option<CantillationMarkStressPosition> {
        match self {
            StressPosition::Impositive => Some(CantillationMarkStressPosition::Impositive),
            StressPosition::Postpositive => Some(CantillationMarkStressPosition::Postpositive),
            StressPosition::Prepositive => Some(CantillationMarkStressPosition::Prepositive),
            StressPosition::NotApplicable => None,
        }
    }
}
