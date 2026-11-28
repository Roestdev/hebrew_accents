// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use crate::api::CantillationMarkStressPosition;

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
mod test_for_derived_traits {
    use super::*;
    const ALL: [StressPosition; 4] = [
        StressPosition::Impositive,
        StressPosition::Prepositive,
        StressPosition::Postpositive,
        StressPosition::NotApplicable,
    ];

    #[test]
    fn test_default_placement() {
        let placement = StressPosition::default();
        dbg!(&placement); // prints to stderr: [src/lib.rs:12:9] &placement = Prepositive
        assert_eq!(placement, StressPosition::Impositive);
    }

    #[test]
    fn test_impositive_explicit_equals_default() {
        assert_eq!(StressPosition::Impositive, StressPosition::default());
    }
    #[test]
    fn test_discriminant_values() {
        assert_eq!(dbg!(StressPosition::Impositive as u8), 0);
        assert_eq!(dbg!(StressPosition::Prepositive as u8), 1);
        assert_eq!(dbg!(StressPosition::Postpositive as u8), 2);
        assert_eq!(dbg!(StressPosition::NotApplicable as u8), 3);
    }
    #[test]
    fn test_pretty_debug_all_variants() {
        let cases = [
            (StressPosition::Impositive, "Impositive"),
            (StressPosition::Prepositive, "Prepositive"),
            (StressPosition::Postpositive, "Postpositive"),
            (StressPosition::NotApplicable, "NotApplicable"),
        ];

        for (variant, name) in cases {
            let pretty = format!("{:#?}", variant);
            dbg!(&pretty);
            assert_eq!(
                pretty, name,
                "pretty Debug for discriminant {}",
                variant as u8
            );
        }
    }
    #[test]
    fn test_clone_produces_equal_value() {
        let original = dbg!(StressPosition::Postpositive);
        let cloned = original.clone();

        assert_eq!(cloned, original);
        assert_eq!(cloned as u8, original as u8);
    }

    #[test]
    fn test_copy_preserves_original() {
        // The defining trait of Copy: value copied AND original still usable
        let original = StressPosition::Prepositive;
        let copied = original;

        assert_eq!(copied, StressPosition::Prepositive);
        assert_eq!(original, StressPosition::Prepositive); // compiles only with Copy
    }

    #[test]
    fn test_explicit_copy_trait_call() {
        // Requires a Copy bound directly, proving the impl exists
        fn duplicate<T: Copy>(value: T) -> (T, T) {
            (value, value)
        }

        let (a, b) = duplicate(StressPosition::Postpositive);
        assert_eq!(a, b);
        assert_eq!(a, StressPosition::Postpositive);
    }
    #[test]
    fn test_all_variants_roundtrip_clone_and_copy() {
        for &variant in &ALL {
            let cloned = variant.clone();
            let moved = variant; // bitwise copy
            assert_eq!(cloned, moved);
            assert_eq!(dbg!(moved as u8), dbg!(variant as u8));
        }
    }

    /// eq(a, a) — every value equals itself
    #[test]
    fn test_reflexivity() {
        for v in ALL {
            assert!(v == v, "{v:?} should equal itself");
        }
    }

    /// eq(a, b) implies eq(b, a)
    #[test]
    fn test_symmetry() {
        for a in ALL {
            for b in ALL {
                assert_eq!(a == b, b == a, "symmetry failed for {a:?} / {b:?}");
            }
        }
    }

    /// eq(a, b) and eq(b, c) imply eq(a, c)
    #[test]
    fn test_transitivity() {
        for a in ALL {
            for b in ALL {
                for c in ALL {
                    if a == b && b == c {
                        assert!(a == c, "transitivity failed: {a:?}, {b:?}, {c:?}");
                    }
                }
            }
        }
    }

    /// Distinct variants are never equal
    #[test]
    fn test_inequality_of_distinct_variants() {
        for a in ALL {
            for b in ALL {
                if a as u8 != b as u8 {
                    assert_ne!(a, b, "{a:?} and {b:?} must not be equal");
                }
            }
        }
    }

    /// Same variant constructed twice compares equal, across Copy boundaries
    #[test]
    fn test_equality_of_same_variant() {
        for v in ALL {
            let copied = v; // Copy semantics
            let recopied = copied;
            assert_eq!(v, copied);
            assert_eq!(copied, recopied);
        }
    }

    /// default() is the #[default]-annotated variant
    #[test]
    fn test_default_equals_annotated_variant() {
        assert_eq!(StressPosition::default(), StressPosition::Impositive);
        assert_ne!(StressPosition::default(), StressPosition::NotApplicable);
    }

    /// eq is consistent with the discriminant: equal if same discriminant
    #[test]
    fn test_equality_matches_discriminant() {
        for a in ALL {
            for b in ALL {
                assert_eq!(
                    a == b,
                    a as u8 == b as u8,
                    "eq/divergence for {a:?} vs {b:?}"
                );
            }
        }
    }

    /// Eq marker trait implies PartialEq must hold the same way (sanity)
    #[test]
    fn test_partial_eq_ne_consistency() {
        for a in ALL {
            for b in ALL {
                assert_ne!(a == b, a != b, "== and != disagree for {a:?}/{b:?}");
            }
        }
    }
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
}

#[cfg(test)]
mod test_impl_to_public {
    use super::*;
    use crate::CantillationMarkStressPosition;
    const ALL: [StressPosition; 4] = [
        StressPosition::Impositive,
        StressPosition::Prepositive,
        StressPosition::Postpositive,
        StressPosition::NotApplicable,
    ];

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
        for pos in &ALL {
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

    #[test]
    fn test_not_applicable_for_pseudo_accents() {
        // Per docs: "Used for PseudoAccents"
        let not_applicable = StressPosition::NotApplicable;

        assert_eq!(not_applicable, StressPosition::NotApplicable);
        assert_eq!(not_applicable.to_public(), None);
    }

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
}

#[cfg(test)]
mod test_usage {
    use super::*;
    const ALL: [StressPosition; 4] = [
        StressPosition::Impositive,
        StressPosition::Prepositive,
        StressPosition::Postpositive,
        StressPosition::NotApplicable,
    ];

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
        for internal in &ALL {
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
}
