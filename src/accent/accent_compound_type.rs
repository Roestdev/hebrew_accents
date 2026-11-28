// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use crate::api::CompoundType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InternalCompoundType {
    /// Indicates a normal compound accent
    /// Meaning it consists of two accent marks and sits on one word only
    Standard,
    /// Indicates that the compound accent contains a Paseq
    /// Meaning the second accent mark is a Paseq
    ContainsPaseq,
    /// Indicates that the compound accent can span two words (or one word)
    CanSpanTwoWords,
}
impl From<InternalCompoundType> for CompoundType {
    fn from(internal: InternalCompoundType) -> Self {
        match internal {
            InternalCompoundType::Standard => CompoundType::Standard,
            InternalCompoundType::ContainsPaseq => CompoundType::ContainsPaseq,
            InternalCompoundType::CanSpanTwoWords => CompoundType::CanSpanTwoWords,
        }
    }
}

// TEMPLATE +++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tier_error_tests {
    // TEMPLATE
    use super::*;

    // #[test]
    // fn error_displays_correct_messages() {
    //     assert_eq!(
    //         InternalCompoundType::InvalidTier(8).to_string(),
    //         "Invalid Tier value: 8 (valid range: 1-4)"
    //     );
    // }

    //#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    //Error trait
    // #[test]
    // fn implements_std_error() {
    //     fn assert_error<E: std::error::Error>(_: &E) {}
    //     assert_error(&InternalCompoundType::InvalidTier(6));
    // }

    // Debug trait
    #[test]
    fn debug_output_contains_variant() {
        let dbg = format!("{:?}", InternalCompoundType::ContainsPaseq);
        assert!(
            dbg.contains("ContainsPaseq"),
            "unexpected Debug output: {dbg}"
        );
        let dbg = format!("{:?}", InternalCompoundType::CanSpanTwoWords);
        assert!(
            dbg.contains("CanSpanTwoWords"),
            "unexpected Debug output: {dbg}"
        );
    }
    // Clone/Copy trait

    #[test]
    fn clone_and_copy_semantics() {
        let original = InternalCompoundType::Standard;
        let copied = original; // Copy: moved AND original still valid
        let cloned = original.clone();
        assert_eq!(copied, original);
        assert_eq!(cloned, original);
    }

    // #[test]
    // fn copied_value_is_fully_usable() {
    //     fn describe(e: InternalCompoundType) -> String {
    //         e.to_string()
    //     }
    //     let err = InternalCompoundType::InvalidTier(6);
    //     let copy = err; // implicit copy
    //     assert_eq!(describe(copy), describe(err));
    // }

    // PartialEq / Eq
    #[test]
    fn partial_eq_works() {
        assert_eq!(
            InternalCompoundType::Standard,
            InternalCompoundType::Standard
        );
        assert_ne!(
            InternalCompoundType::Standard,
            InternalCompoundType::ContainsPaseq
        );
        assert_ne!(
            InternalCompoundType::ContainsPaseq,
            InternalCompoundType::CanSpanTwoWords
        );
    }
    // Eq
    #[test]
    fn implements_eq() {
        fn assert_eq_trait<T: Eq>(_: &T) {}
        assert_eq_trait(&InternalCompoundType::Standard);
        assert_eq_trait(&InternalCompoundType::ContainsPaseq);
        assert_eq_trait(&InternalCompoundType::CanSpanTwoWords);
    }
    // #[test]
    // fn equality_against_real_conversion_failure() {
    //     let err = Tier::try_from(42u8).unwrap_err();
    //     assert_eq!(err, InternalCompoundType::InvalidTier(42));
    //     assert_ne!(err, InternalCompoundType::InvalidTier(43));
    // }
}
