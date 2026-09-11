use crate::{GroupLevel, HebrewAccent, PoetryAccent, ProseAccent};

/// Full Futato hierarchy classification with prose/poetry distinction.
///
/// **Internal use only**—do not rely on this accenttype publicly as it may change
/// without semver warning. Use [`super::GroupLevel`] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum DisjunctiveGroup {
    ProseTier1, // Fixed variants, no number needed
    ProseTier2,
    ProseTier3,
    ProseTier4,
    PoetryTier1,
    PoetryTier2,
    PoetryTier3, // Max tier differs between systems
}

impl DisjunctiveGroup {
    /// Convert to simplified public GroupLevel
    pub(crate) const fn into_public_level(self) -> Option<GroupLevel> {
        match self {
            Self::ProseTier1 | Self::PoetryTier1 => Some(GroupLevel::Tier1),
            Self::ProseTier2 | Self::PoetryTier2 => Some(GroupLevel::Tier2),
            Self::ProseTier3 | Self::PoetryTier3 => Some(GroupLevel::Tier3),
            Self::ProseTier4 => Some(GroupLevel::Tier4),
        }
    }
}

/// Lookup logic for accent hierarchy (private function)
pub(crate) fn resolve_disjunctive_group(accent: HebrewAccent) -> Option<DisjunctiveGroup> {
    match accent {
        HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Prose(ProseAccent::Atnach) => {
            Some(DisjunctiveGroup::ProseTier1)
        }

        HebrewAccent::Prose(ProseAccent::Segolta)
        | HebrewAccent::Prose(ProseAccent::Shalshelet)
        | HebrewAccent::Prose(ProseAccent::ZaqephQatan)
        | HebrewAccent::Prose(ProseAccent::ZaqephGadol)
        | HebrewAccent::Prose(ProseAccent::Tiphcha) => Some(DisjunctiveGroup::ProseTier2),

        HebrewAccent::Prose(ProseAccent::Revia)
        | HebrewAccent::Prose(ProseAccent::Zarqa)
        | HebrewAccent::Prose(ProseAccent::Pashta)
        | HebrewAccent::Prose(ProseAccent::Tevir)
        | HebrewAccent::Prose(ProseAccent::Yetiv) => Some(DisjunctiveGroup::ProseTier3),

        HebrewAccent::Prose(ProseAccent::Geresh)
        | HebrewAccent::Prose(ProseAccent::Gershayim)
        | HebrewAccent::Prose(ProseAccent::Pazer)
        | HebrewAccent::Prose(ProseAccent::PazerGadol)
        | HebrewAccent::Prose(ProseAccent::TelishaGedolah)
        | HebrewAccent::Prose(ProseAccent::Legarmeh) => Some(DisjunctiveGroup::ProseTier4),

        HebrewAccent::Poetry(PoetryAccent::Silluq)
        | HebrewAccent::Poetry(PoetryAccent::OlehWeYored)
        | HebrewAccent::Poetry(PoetryAccent::Atnach) => Some(DisjunctiveGroup::PoetryTier1),

        HebrewAccent::Poetry(PoetryAccent::ReviaGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)
        | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaQaton)
        | HebrewAccent::Poetry(PoetryAccent::Tsinnor)
        | HebrewAccent::Poetry(PoetryAccent::Dechi) => Some(DisjunctiveGroup::PoetryTier2),

        HebrewAccent::Poetry(PoetryAccent::Pazer)
        | HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
        | HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => Some(DisjunctiveGroup::PoetryTier3),
        _ => None, // conjunctives and pseudo-accents lack hierarchy
    }
}
