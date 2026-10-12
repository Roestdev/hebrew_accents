use crate::api::AccentCategory;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub(crate) enum InternalAccentCategory {
    /// accents that separate words
    Disjunctive,
    /// accents that connect words
    Conjunctive,
}

impl From<InternalAccentCategory> for AccentCategory {
    fn from(internal: InternalAccentCategory) -> Self {
        match internal {
            InternalAccentCategory::Disjunctive => AccentCategory::Disjunctive,
            InternalAccentCategory::Conjunctive => AccentCategory::Conjunctive,
        }
    }
}
