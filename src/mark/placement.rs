// TODO test subject
// TODO test_impl
use crate::CantillationMarkPlacement;

/// Canonical position representation for Hebrew cantillation marks.
///
/// Encodes both spatial positions (above/below × left/center/right)
/// and special semantic markers (Maqqaf, Sof Pasuq, Paseq).
///
/// ## Usage
/// - **Grid positions (0-4)**: Standard placement relative to base consonant
/// - **Special positions (5-7)**: Semantic overrides that bypass normal grid
///
/// ## Notes
/// The position `BelowLeft` is not used by any accent!
///
/// All values are compile-time constants in static registries.
/// No heap allocation or runtime construction required.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum MarkPlacement {
    /// Above the baseline, left of the consonant
    AboveLeft = 0,
    /// Above the baseline, centered on the consonant
    #[default]
    AboveCenter = 1,
    /// Above the baseline, right of the consonant
    AboveRight = 2,
    // Below the baseline, left of the consonant
    // BelowLeft = 3,
    /// Below the baseline, centered on the consonant
    BelowCenter = 3,
    /// Below the baseline, right of the consonant
    BelowRight = 4,
    /// End-of-verse marker (Sof Pasuq — ׃)
    SofPasuq = 5,
    /// Inter-word connector (Maqqaf — ֿ)
    Maqqaph = 6,
    /// Visual separator (Paseq — ֊)
    Paseq = 7,
}

// Conversion from internal to public type
impl From<MarkPlacement> for CantillationMarkPlacement {
    fn from(pos: MarkPlacement) -> Self {
        use CantillationMarkPlacement as Public;
        use MarkPlacement as Internal;

        match pos {
            Internal::AboveLeft => Public::AboveLeft,
            Internal::AboveCenter => Public::AboveCenter,
            Internal::AboveRight => Public::AboveRight,
            //Internal::BelowLeft => Public::BelowLeft,
            Internal::BelowCenter => Public::BelowCenter,
            Internal::BelowRight => Public::BelowRight,
            Internal::SofPasuq => Public::SofPasuq,
            Internal::Maqqaph => Public::Maqqaph,
            Internal::Paseq => Public::Paseq,
        }
    }
}

#[cfg(test)]
mod test_for_derived_traits {
    use super::*;
    const ALL: [MarkPlacement; 8] = [
        MarkPlacement::AboveLeft,
        MarkPlacement::AboveCenter,
        MarkPlacement::AboveRight,
        MarkPlacement::BelowCenter,
        MarkPlacement::BelowRight,
        MarkPlacement::SofPasuq,
        MarkPlacement::Maqqaph,
        MarkPlacement::Paseq,
    ];

    #[test]
    fn test_default_placement() {
        let placement = MarkPlacement::default();
        dbg!(&placement); // prints to stderr: [src/lib.rs:12:9] &placement = AboveCenter
        assert_eq!(placement, MarkPlacement::AboveCenter);
    }
    #[test]
    fn test_discriminant_values() {
        assert_eq!(dbg!(MarkPlacement::AboveLeft as u8), 0);
        assert_eq!(dbg!(MarkPlacement::AboveCenter as u8), 1);
        assert_eq!(dbg!(MarkPlacement::AboveRight as u8), 2);
        assert_eq!(dbg!(MarkPlacement::BelowCenter as u8), 3);
        assert_eq!(dbg!(MarkPlacement::BelowRight as u8), 4);
        assert_eq!(dbg!(MarkPlacement::SofPasuq as u8), 5);
        assert_eq!(dbg!(MarkPlacement::Maqqaph as u8), 6);
        assert_eq!(dbg!(MarkPlacement::Paseq as u8), 7);
    }
    #[test]
    fn test_pretty_debug_all_variants() {
        let cases = [
            (MarkPlacement::AboveLeft, "AboveLeft"),
            (MarkPlacement::AboveCenter, "AboveCenter"),
            (MarkPlacement::AboveRight, "AboveRight"),
            (MarkPlacement::BelowCenter, "BelowCenter"),
            (MarkPlacement::BelowRight, "BelowRight"),
            (MarkPlacement::SofPasuq, "SofPasuq"),
            (MarkPlacement::Maqqaph, "Maqqaph"),
            (MarkPlacement::Paseq, "Paseq"),
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
        let original = dbg!(MarkPlacement::AboveRight);
        let cloned = original.clone();

        assert_eq!(cloned, original);
        assert_eq!(cloned as u8, original as u8);
    }

    #[test]
    fn test_copy_preserves_original() {
        // The defining trait of Copy: value copied AND original still usable
        let original = MarkPlacement::BelowRight;
        let copied = original;

        assert_eq!(copied, MarkPlacement::BelowRight);
        assert_eq!(original, MarkPlacement::BelowRight); // compiles only with Copy
    }

    #[test]
    fn test_explicit_copy_trait_call() {
        // Requires a Copy bound directly, proving the impl exists
        fn duplicate<T: Copy>(value: T) -> (T, T) {
            (value, value)
        }

        let (a, b) = duplicate(MarkPlacement::SofPasuq);
        assert_eq!(a, b);
        assert_eq!(a, MarkPlacement::SofPasuq);
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
        assert_eq!(MarkPlacement::default(), MarkPlacement::AboveCenter);
        assert_ne!(MarkPlacement::default(), MarkPlacement::AboveLeft);
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
}
