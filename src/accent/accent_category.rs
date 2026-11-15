use crate::api::AccentCategory;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

#[cfg(test)]
mod internal_accent_category_derivable_traits {
    use super::*;

    //#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]

    // Debug trait
    #[test]
    fn debug_output_contains_variant() {
        let dbg = format!("{:?}", InternalAccentCategory::Disjunctive);
        assert!(
            dbg.contains("Disjunctive"),
            "unexpected Debug output: {dbg}"
        );
    }
    // Clone/Copy trait
    #[test]
    fn clone_and_copy_semantics() {
        let original = InternalAccentCategory::Disjunctive;
        let copied = original; // Copy: moved AND original still valid
        let cloned = original.clone();
        assert_eq!(copied, original);
        assert_eq!(cloned, original);
    }

    // PartialEq / Eq
    #[test]
    fn partial_eq_works() {
        assert_eq!(
            InternalAccentCategory::Disjunctive,
            InternalAccentCategory::Disjunctive
        );
        assert_ne!(
            InternalAccentCategory::Disjunctive,
            InternalAccentCategory::Conjunctive
        );
    }
    // Eq
    #[test]
    fn implements_eq() {
        fn assert_eq_trait<T: Eq>(_: &T) {}
        assert_eq_trait(&InternalAccentCategory::Disjunctive);
    }
}
