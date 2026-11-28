// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use std::fmt;

/// Hebrew Accent category — (absence is expressed via `Option<T>`)
///
/// # Examples
///
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, AccentCategory};
///
/// // Disjunctive accents return Some(disjunctive)
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.category(), Some(AccentCategory::Disjunctive));
///
/// // Conjunctive accents return Some(conjunctive)
/// let munach: HebrewAccent = ProseAccent::Munach.into();
/// assert_eq!(munach.category(), Some(AccentCategory::Conjunctive));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AccentCategory {
    /// accents that separate words
    Disjunctive,
    /// accents that connect words
    Conjunctive,
}

impl fmt::Display for AccentCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disjunctive => write!(f, "disjunctive"),
            Self::Conjunctive => write!(f, "conjunctive"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // ===== ACCENT CATEGORY TESTS =====

    #[test]
    fn test_accent_category_variants() {
        assert_eq!(format!("{}", AccentCategory::Disjunctive), "disjunctive");
        assert_eq!(format!("{}", AccentCategory::Conjunctive), "conjunctive");
    }

    #[test]
    fn test_accent_category_debug_trait() {
        let debug = format!("{:?}", AccentCategory::Disjunctive);
        assert!(debug.contains("Disjunctive"));
    }

    #[test]
    fn test_accent_category_copy_clone_eq_hash() {
        let c1 = AccentCategory::Conjunctive;
        let c2 = AccentCategory::Conjunctive;

        assert_eq!(c1, c2);
        assert_eq!(c1.clone(), c2);
    }

    #[test]
    fn test_accent_category_display_exact_output() {
        assert_eq!(format!("{}", AccentCategory::Disjunctive), "disjunctive");
        assert_eq!(format!("{}", AccentCategory::Conjunctive), "conjunctive");
    }
}
