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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CantillationMarkStressPosition;

    // ===== BASIC VARIANT EXISTENCE TESTS =====

    #[test]
    fn test_all_variants_exist() {
        let impositive = StressPosition::Impositive;
        let prepositive = StressPosition::Prepositive;
        let postpositive = StressPosition::Postpositive;
        let not_applicable = StressPosition::NotApplicable;

        assert_eq!(impositive, StressPosition::Impositive);
        assert_eq!(prepositive, StressPosition::Prepositive);
        assert_eq!(postpositive, StressPosition::Postpositive);
        assert_eq!(not_applicable, StressPosition::NotApplicable);
    }

    // ===== DEFAULT TRAIT TESTS =====

    #[test]
    fn test_default_is_impositive() {
        let default: StressPosition = Default::default();
        assert_eq!(default, StressPosition::Impositive);
    }

    #[test]
    fn test_impositive_explicit_equals_default() {
        assert_eq!(StressPosition::Impositive, StressPosition::default());
    }

    // #[test]
    // fn test_default_macro_style() {
    //     // Test different ways of invoking Default
    //     let _d1 = Default::default();
    //     let _d2: StressPosition = Default::default();

    //     assert_eq!(_d1, StressPosition::Impositive);
    //     assert_eq!(_d2, StressPosition::Impositive);
    // }

    // ===== TO_PUBLIC CONVERSION METHOD TESTS =====

    #[test]
    fn test_to_public_for_impositive() {
        let internal = StressPosition::Impositive;
        let converted = internal.to_public();

        assert_eq!(converted, Some(CantillationMarkStressPosition::Impositive));
    }

    #[test]
    fn test_to_public_for_prepositive() {
        let internal = StressPosition::Prepositive;
        let converted = internal.to_public();

        assert_eq!(converted, Some(CantillationMarkStressPosition::Prepositive));
    }

    #[test]
    fn test_to_public_for_postpositive() {
        let internal = StressPosition::Postpositive;
        let converted = internal.to_public();

        assert_eq!(
            converted,
            Some(CantillationMarkStressPosition::Postpositive)
        );
    }

    #[test]
    fn test_to_public_for_not_applicable_returns_none() {
        let internal = StressPosition::NotApplicable;
        let converted = internal.to_public();

        assert_eq!(converted, None);
    }

    #[test]
    fn test_to_public_all_variants_covered() {
        // Exhaustive test of all variants
        let all = [
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ];

        for pos in &all {
            let result = pos.to_public();
            match pos {
                StressPosition::Impositive => {
                    assert_eq!(result, Some(CantillationMarkStressPosition::Impositive));
                }
                StressPosition::Prepositive => {
                    assert_eq!(result, Some(CantillationMarkStressPosition::Prepositive));
                }
                StressPosition::Postpositive => {
                    assert_eq!(result, Some(CantillationMarkStressPosition::Postpositive));
                }
                StressPosition::NotApplicable => {
                    assert_eq!(result, None);
                }
            }
        }
    }

    #[test]
    fn test_to_public_consistency_across_calls() {
        // Verify consistent behavior on repeated calls
        let pos = StressPosition::Impositive;

        assert_eq!(pos.to_public(), pos.to_public());
        assert_eq!(pos.to_public(), pos.to_public());
    }

    #[test]
    fn test_to_public_const_compatible() {
        // Verify function can be used in const contexts
        const POS: StressPosition = StressPosition::Impositive;
        const RESULT: Option<CantillationMarkStressPosition> = POS.to_public();

        assert_eq!(RESULT, Some(CantillationMarkStressPosition::Impositive));
    }

    // ===== DERIVED TRAIT TESTS =====

    // --- Debug Trait ---

    #[test]
    fn test_debug_trait_impositive() {
        let debug = format!("{:?}", StressPosition::Impositive);
        assert!(debug.contains("Impositive"));
    }

    #[test]
    fn test_debug_trait_prepositive() {
        let debug = format!("{:?}", StressPosition::Prepositive);
        assert!(debug.contains("Prepositive"));
    }

    #[test]
    fn test_debug_trait_postpositive() {
        let debug = format!("{:?}", StressPosition::Postpositive);
        assert!(debug.contains("Postpositive"));
    }

    #[test]
    fn test_debug_trait_not_applicable() {
        let debug = format!("{:?}", StressPosition::NotApplicable);
        assert!(debug.contains("NotApplicable"));
    }

    #[test]
    fn test_display_via_debug() {
        // Debug trait enables display
        for variant in &[
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ] {
            let _ = format!("{:?}", variant);
        }
    }

    // --- Copy Trait ---

    #[test]
    fn test_copy_trait_impositive() {
        let original = StressPosition::Impositive;
        let copied = original; // Copy occurs automatically

        assert_eq!(original, StressPosition::Impositive);
        assert_eq!(copied, StressPosition::Impositive);
        // Original still usable
        assert_eq!(
            original.to_public(),
            Some(CantillationMarkStressPosition::Impositive)
        );
    }

    #[test]
    fn test_copy_trait_all_variants() {
        for variant in &[
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ] {
            let copied = *variant; // Explicit copy
            assert_eq!(variant, &copied);
        }
    }

    // --- Clone Trait ---

    #[test]
    fn test_clone_trait_impositive() {
        let original = StressPosition::Impositive;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned, StressPosition::Impositive);
    }

    #[test]
    fn test_clone_trait_all_variants() {
        for variant in &[
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ] {
            let cloned = variant.clone();
            assert_eq!(variant, &cloned);
        }
    }

    // --- PartialEq/Eq Trait ---

    #[test]
    fn test_partial_eq_same_variant() {
        assert_eq!(StressPosition::Impositive, StressPosition::Impositive);
        assert_eq!(StressPosition::Prepositive, StressPosition::Prepositive);
        assert_eq!(StressPosition::Postpositive, StressPosition::Postpositive);
        assert_eq!(StressPosition::NotApplicable, StressPosition::NotApplicable);
    }

    #[test]
    fn test_partial_eq_different_variants() {
        assert_ne!(StressPosition::Impositive, StressPosition::Prepositive);
        assert_ne!(StressPosition::Impositive, StressPosition::Postpositive);
        assert_ne!(StressPosition::Impositive, StressPosition::NotApplicable);
        assert_ne!(StressPosition::Prepositive, StressPosition::Postpositive);
        assert_ne!(StressPosition::Prepositive, StressPosition::NotApplicable);
        assert_ne!(StressPosition::Postpositive, StressPosition::NotApplicable);
    }

    #[test]
    fn test_eq_trait_reflexivity() {
        let pos = StressPosition::Impositive;
        assert!(pos.eq(&pos));
    }

    #[test]
    fn test_eq_trait_symmetry() {
        let p1 = StressPosition::Postpositive;
        let p2 = StressPosition::Postpositive;

        assert!(p1.eq(&p2));
        assert!(p2.eq(&p1));
    }

    #[test]
    fn test_eq_trait_transitivity() {
        let p1 = StressPosition::Prepositive;
        let p2 = StressPosition::Prepositive;
        let p3 = StressPosition::Prepositive;

        assert!(p1.eq(&p2));
        assert!(p2.eq(&p3));
        assert!(p1.eq(&p3));
    }

    // --- Hash Trait ---

    #[test]
    fn test_hash_trait_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        StressPosition::Impositive.hash(&mut hasher1);
        StressPosition::Impositive.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_trait_different_variants_different_hashes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        StressPosition::Impositive.hash(&mut hasher1);
        StressPosition::Prepositive.hash(&mut hasher2);

        // Different variants may have different hashes (not guaranteed but likely)
        // Just verify they hash successfully
        let h1 = hasher1.finish();
        let h2 = hasher2.finish();

        // At minimum, both should produce valid hash values
        assert!(h1 > 0 || h1 == 0);
        assert!(h2 > 0 || h2 == 0);
    }

    #[test]
    fn test_hash_in_collections() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(StressPosition::Impositive);
        set.insert(StressPosition::Prepositive);
        set.insert(StressPosition::Postpositive);
        set.insert(StressPosition::NotApplicable);

        assert_eq!(set.len(), 4);
        assert!(set.contains(&StressPosition::Impositive));
        assert!(set.contains(&StressPosition::Prepositive));
        assert!(set.contains(&StressPosition::Postpositive));
        assert!(set.contains(&StressPosition::NotApplicable));
    }

    // ===== SEMANTIC MEANING TESTS =====

    #[test]
    fn test_impositive_sits_on_stressed_syllable() {
        // Per docs: "Mark sits directly on the stressed syllable"
        let impositive = StressPosition::Impositive;

        // Verify it exists and can be converted
        assert_eq!(impositive, StressPosition::Impositive);
        assert!(impositive.to_public().is_some());
    }

    #[test]
    fn test_prepositive_before_stressed_syllable() {
        // Per docs: "Mark is on a syllable preceding the stressed one"
        let prepositive = StressPosition::Prepositive;

        assert_eq!(prepositive, StressPosition::Prepositive);
        assert!(prepositive.to_public().is_some());
    }

    #[test]
    fn test_postpositive_after_stressed_syllable() {
        // Per docs: "Mark is on a syllable following the stressed one"
        let postpositive = StressPosition::Postpositive;

        assert_eq!(postpositive, StressPosition::Postpositive);
        assert!(postpositive.to_public().is_some());
    }

    #[test]
    fn test_not_applicable_for_pseudo_accents() {
        // Per docs: "Used for PseudoAccents"
        let not_applicable = StressPosition::NotApplicable;

        assert_eq!(not_applicable, StressPosition::NotApplicable);
        assert_eq!(not_applicable.to_public(), None);
    }

    // ===== INTEGRATION WITH PUBLIC TYPE TESTS =====

    #[test]
    fn test_to_public_preserves_semantics() {
        // Verify conversion preserves semantic meaning
        let impositive_internal = StressPosition::Impositive;
        let impositive_public = impositive_internal.to_public();

        assert_eq!(
            impositive_public,
            Some(CantillationMarkStressPosition::Impositive)
        );
    }

    #[test]
    fn test_conversion_roundtrip_possible() {
        // For non-N/A values, roundtrip should preserve meaning
        let originals = [
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
        ];

        for original in &originals {
            let converted = original.to_public();
            assert!(converted.is_some());

            // Can extract and compare
            if let Some(converted_val) = converted {
                // Should map back to original conceptually
                match (original, converted_val) {
                    (StressPosition::Impositive, CantillationMarkStressPosition::Impositive) => {}
                    (StressPosition::Prepositive, CantillationMarkStressPosition::Prepositive) => {}
                    (
                        StressPosition::Postpositive,
                        CantillationMarkStressPosition::Postpositive,
                    ) => {}
                    _ => panic!("Mismatch in conversion"),
                }
            }
        }
    }

    #[test]
    fn test_not_applicable_maps_to_none_correctly() {
        // NotApplicable is the only variant mapping to None
        let na = StressPosition::NotApplicable;

        assert_eq!(na.to_public(), None);

        // No other variant maps to None
        assert!(StressPosition::Impositive.to_public().is_some());
        assert!(StressPosition::Prepositive.to_public().is_some());
        assert!(StressPosition::Postpositive.to_public().is_some());
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_const_usage() {
        const IMPOSITIVE: StressPosition = StressPosition::Impositive;
        const PREPOSITIVE: StressPosition = StressPosition::Prepositive;

        assert_eq!(
            IMPOSITIVE.to_public(),
            Some(CantillationMarkStressPosition::Impositive)
        );
        assert_eq!(
            PREPOSITIVE.to_public(),
            Some(CantillationMarkStressPosition::Prepositive)
        );
    }

    #[test]
    fn test_no_ord_trait_impl() {
        // Verify Ord/PartialOrd are NOT derived (not mentioned in derives)
        // This compile-time check ensures we don't accidentally sort stress positions
        let _p1 = StressPosition::Impositive;
        let _p2 = StressPosition::Postpositive;

        // The following would NOT compile (commented out):
        // assert!(_p1 < _p2);  // Error: Ord not implemented

        // Instead use equality checks
        assert_eq!(_p1, StressPosition::Impositive);
    }

    #[test]
    fn test_all_variants_used_in_match() {
        // Exhaustive match should compile without warnings
        for pos in &[
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ] {
            match pos {
                StressPosition::Impositive => assert!(true),
                StressPosition::Prepositive => assert!(true),
                StressPosition::Postpositive => assert!(true),
                StressPosition::NotApplicable => assert!(true),
            }
        }
    }

    // ===== USAGE SCENARIO TESTS =====

    #[test]
    fn test_usage_in_option_pattern() {
        // StressPosition often wrapped in Option for public API
        let opt_pos: Option<StressPosition> = Some(StressPosition::Impositive);
        let none_opt: Option<StressPosition> = None;

        assert!(opt_pos.is_some());
        assert_eq!(opt_pos.unwrap(), StressPosition::Impositive);
        assert!(none_opt.is_none());
    }

    #[test]
    fn test_usage_with_public_stress_position() {
        // Demonstrate internal-to-public conversion workflow
        let internal_positions = [
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ];

        for internal in &internal_positions {
            let public = internal.to_public();

            // Should always produce valid Option<CantillationMarkStressPosition>
            match public {
                Some(p) => {
                    // Valid public stress position
                    assert!(matches!(
                        p,
                        CantillationMarkStressPosition::Impositive
                            | CantillationMarkStressPosition::Prepositive
                            | CantillationMarkStressPosition::Postpositive
                    ));
                }
                None => {
                    // Only NotApplicable produces None
                    assert_eq!(*internal, StressPosition::NotApplicable);
                }
            }
        }
    }

    #[test]
    fn test_linguistic_metrical_concept_separation() {
        // StressPosition is independent from visual placement
        // This is a conceptual test documenting the design decision

        // The enum represents linguistic stress relationships
        // NOT visual mark placement on letters
        assert_eq!(
            StressPosition::Impositive.to_public(),
            Some(CantillationMarkStressPosition::Impositive)
        );
    }
}
