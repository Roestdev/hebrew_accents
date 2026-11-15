/// Indicates the compound type of a [`HebrewAccent`](crate::HebrewAccent)
/// Absence of a compound is expressed via `Option<CompoundType>`.
///
/// When an accent consists of two cantillation marks it is called a
/// *compound accent*. Three types are distinguished:
///
/// - [`Standard`](CompoundType::Standard) — two marks on one word
/// - [`ContainsPaseq`](CompoundType::ContainsPaseq) — the second mark is a Paseq
/// - [`CanSpanTwoWords`](CompoundType::CanSpanTwoWords) — may span two words,
///   e.g. Oleh WeYored (עוֹלֶה וְיוֹרֵד), Tsinnorit Merkha (צִנּוֹרִית מֵרכָּא),
///   Tsinnorit Mahpakh (צִנּוֹרִית מַהְפָּךְ)
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CompoundType {
    /// Indicates a normal compound accent
    /// Meaning it consists of two accent marks and sits on one word only
    Standard,
    /// Indicates that the compound accent contains a Paseq
    /// Meaning the second accent mark is a Paseq
    ContainsPaseq,
    /// Indicates that the compound accent can span two words (or one word)
    CanSpanTwoWords,
}

// TEMPLATE +++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tier_error_tests {
    // TEMPLATE
    use super::*;
    use std::{
        collections::HashSet,
        hash::{DefaultHasher, Hash, Hasher},
    };

    // #[test]
    // fn error_displays_correct_messages() {
    //     assert_eq!(
    //         CompoundType::InvalidTier(8).to_string(),
    //         "Invalid Tier value: 8 (valid range: 1-4)"
    //     );
    // }

    //#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    //Error trait
    // #[test]
    // fn implements_std_error() {
    //     fn assert_error<E: std::error::Error>(_: &E) {}
    //     assert_error(&CompoundType::InvalidTier(6));
    // }

    // Debug trait
    #[test]
    fn debug_output_contains_variant() {
        let dbg = format!("{:?}", CompoundType::ContainsPaseq);
        assert!(
            dbg.contains("ContainsPaseq"),
            "unexpected Debug output: {dbg}"
        );
        let dbg = format!("{:?}", CompoundType::CanSpanTwoWords);
        assert!(
            dbg.contains("CanSpanTwoWords"),
            "unexpected Debug output: {dbg}"
        );
    }
    // Clone/Copy trait

    #[test]
    fn clone_and_copy_semantics() {
        let original = CompoundType::Standard;
        let copied = original; // Copy: moved AND original still valid
        let cloned = original.clone();
        assert_eq!(copied, original);
        assert_eq!(cloned, original);
    }

    // #[test]
    // fn copied_value_is_fully_usable() {
    //     fn describe(e: CompoundType) -> String {
    //         e.to_string()
    //     }
    //     let err = CompoundType::InvalidTier(6);
    //     let copy = err; // implicit copy
    //     assert_eq!(describe(copy), describe(err));
    // }

    // PartialEq / Eq
    #[test]
    fn partial_eq_works() {
        assert_eq!(CompoundType::Standard, CompoundType::Standard);
        assert_ne!(CompoundType::Standard, CompoundType::ContainsPaseq);
        assert_ne!(CompoundType::ContainsPaseq, CompoundType::CanSpanTwoWords);
    }
    // Eq
    #[test]
    fn implements_eq() {
        fn assert_eq_trait<T: Eq>(_: &T) {}
        assert_eq_trait(&CompoundType::Standard);
        assert_eq_trait(&CompoundType::ContainsPaseq);
        assert_eq_trait(&CompoundType::CanSpanTwoWords);
    }
    // #[test]
    // fn equality_against_real_conversion_failure() {
    //     let err = Tier::try_from(42u8).unwrap_err();
    //     assert_eq!(err, CompoundType::InvalidTier(42));
    //     assert_ne!(err, CompoundType::InvalidTier(43));
    // }

    //Hash

    #[test]
    fn hash_matches_equality() {
        let hash = |e: CompoundType| {
            let mut h = DefaultHasher::new();
            e.hash(&mut h);
            h.finish()
        };
        assert_eq!(hash(CompoundType::Standard), hash(CompoundType::Standard));
        assert_ne!(
            hash(CompoundType::Standard),
            hash(CompoundType::ContainsPaseq)
        );
        assert_ne!(
            hash(CompoundType::CanSpanTwoWords),
            hash(CompoundType::ContainsPaseq)
        );
    }

    // Hash       //     Standard,ContainsPaseq,CanSpanTwoWords,
    #[test]
    fn usable_in_hashset() {
        let mut set = HashSet::new();
        set.insert(CompoundType::ContainsPaseq);
        set.insert(CompoundType::ContainsPaseq); // duplicate ignored
        assert_eq!(set.len(), 1);
        assert!(set.contains(&CompoundType::ContainsPaseq));
    }
}
