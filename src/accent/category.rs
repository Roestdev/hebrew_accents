use crate::AccentCategory;

/// Hebrew Accent category (either Conjunctive or Disjunctive)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Category {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
    /// Only applicable for PseudoAccent's
    None,
}

impl Category {
    pub(crate) const fn to_public(self) -> Option<AccentCategory> {
        match self {
            Category::Conjunctive => Some(AccentCategory::Conjunctive),
            Category::Disjunctive => Some(AccentCategory::Disjunctive),
            Category::None => None,
        }
    }
}
