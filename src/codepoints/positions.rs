/// Accent position, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum CodePointPosition {
    // Changed from `pub` to `pub(crate)`
    /// UTF-8 code point is located above the consonant
    Above,
    /// UTF-8 code point is located after the consonant
    /// Used for punctuation marks that follow the word boundary
    /// Used for Paseq, Soph Pasuq and Maqqeph
    After,
    /// UTF-8 code point is located in between two words
    /// Used for separators between two words/clauses
    InBetween,
    /// UTF-8 code point is located under the consonant
    #[default]
    Under,
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Variant existence tests ──────────────────────────────────────

    #[test]
    fn above_variant_exists() {
        let pos = CodePointPosition::Above;
        assert_eq!(pos, CodePointPosition::Above);
    }

    #[test]
    fn after_variant_exists() {
        let pos = CodePointPosition::After;
        assert_eq!(pos, CodePointPosition::After);
    }

    #[test]
    fn in_between_variant_exists() {
        let pos = CodePointPosition::InBetween;
        assert_eq!(pos, CodePointPosition::InBetween);
    }

    #[test]
    fn under_variant_exists() {
        let pos = CodePointPosition::Under;
        assert_eq!(pos, CodePointPosition::Under);
    }

    // ── Distinctness tests ───────────────────────────────────────────

    #[test]
    fn all_variants_are_distinct() {
        let above = CodePointPosition::Above;
        let after = CodePointPosition::After;
        let in_between = CodePointPosition::InBetween;
        let under = CodePointPosition::Under;

        assert_ne!(above, after);
        assert_ne!(above, in_between);
        assert_ne!(above, under);
        assert_ne!(after, in_between);
        assert_ne!(after, under);
        assert_ne!(in_between, under);
    }

    #[test]
    fn variants_hash_to_different_values() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn hash_value(v: &CodePointPosition) -> u64 {
            let mut hasher = DefaultHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        }

        let positions = [
            CodePointPosition::Above,
            CodePointPosition::After,
            CodePointPosition::InBetween,
            CodePointPosition::Under,
        ];

        // All should have unique hash values
        let hashes: Vec<u64> = positions.iter().map(hash_value).collect();
        let unique_hashes: std::collections::HashSet<_> = hashes.iter().cloned().collect();

        // Note: Hash collisions are theoretically possible but unlikely with DefaultHasher
        // This test documents the expectation that they're typically different
        assert_eq!(
            unique_hashes.len(),
            hashes.len(),
            "All positions should typically hash to unique values"
        );
    }

    // ── Default trait tests ──────────────────────────────────────────

    #[test]
    fn default_is_under() {
        let default_pos = CodePointPosition::default();
        assert_eq!(default_pos, CodePointPosition::Under);
    }

    #[test]
    fn explicit_default_constructor() {
        let pos: CodePointPosition = Default::default();
        assert_eq!(pos, CodePointPosition::Under);
    }

    #[test]
    fn under_variant_is_default() {
        // Verify the #[default] attribute is correctly applied to Under
        assert_eq!(CodePointPosition::Under, CodePointPosition::default());
    }

    // ── Copy and Clone trait tests ───────────────────────────────────

    #[test]
    fn copy_trait_preserves_value() {
        let original = CodePointPosition::Above;
        let copied = original; // Copy happens implicitly

        // Original is still usable (wasn't moved)
        assert_eq!(original, CodePointPosition::Above);
        assert_eq!(copied, CodePointPosition::Above);
    }

    #[test]
    fn clone_trait_produces_equal_value() {
        let original = CodePointPosition::After;
        let cloned = original.clone();

        assert_eq!(cloned, original);
        assert_eq!(cloned, CodePointPosition::After);
    }

    #[test]
    fn copy_clone_yield_identical_variants() {
        let pos = CodePointPosition::InBetween;
        let copied = pos;
        let cloned = pos.clone();

        assert_eq!(pos, copied);
        assert_eq!(pos, cloned);
        assert_eq!(copied, cloned);
    }

    // ── Eq and PartialEq trait tests ─────────────────────────────────

    #[test]
    fn eq_reflexive() {
        let pos = CodePointPosition::Under;
        assert_eq!(pos, pos);
    }

    #[test]
    fn eq_symmetric() {
        let pos1 = CodePointPosition::Above;
        let pos2 = CodePointPosition::Above;

        assert_eq!(pos1, pos2);
        assert_eq!(pos2, pos1);
    }

    #[test]
    fn eq_transitive() {
        let pos1 = CodePointPosition::InBetween;
        let pos2 = CodePointPosition::InBetween;
        let pos3 = CodePointPosition::InBetween;

        assert_eq!(pos1, pos2);
        assert_eq!(pos2, pos3);
        assert_eq!(pos1, pos3);
    }

    #[test]
    fn ne_opposite_variants() {
        assert_ne!(CodePointPosition::Above, CodePointPosition::Under);
    }

    // Wait, there's no Below variant - let me fix that test
    #[test]
    fn ne_different_variants() {
        assert_ne!(CodePointPosition::Above, CodePointPosition::Under);
        assert_ne!(CodePointPosition::After, CodePointPosition::InBetween);
    }

    // ── Hash trait tests ─────────────────────────────────────────────

    #[test]
    fn hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn hash_value(v: &CodePointPosition) -> u64 {
            let mut hasher = DefaultHasher::new();
            v.hash(&mut hasher);
            hasher.finish()
        }

        let pos = CodePointPosition::Above;
        let hash1 = hash_value(&pos);
        let hash2 = hash_value(&pos);

        assert_eq!(hash1, hash2, "Same value should produce same hash");
    }

    #[test]
    fn hash_works_in_hashmap() {
        use std::collections::HashMap;

        let mut map = HashMap::new();

        map.insert(CodePointPosition::Above, 1);
        map.insert(CodePointPosition::Under, 2);
        map.insert(CodePointPosition::After, 3);

        assert_eq!(map.get(&CodePointPosition::Above), Some(&1));
        assert_eq!(map.get(&CodePointPosition::Under), Some(&2));
        assert_eq!(map.get(&CodePointPosition::After), Some(&3));
    }

    #[test]
    fn hash_works_in_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::new();

        set.insert(CodePointPosition::Above);
        set.insert(CodePointPosition::Under);
        set.insert(CodePointPosition::After);

        assert!(set.contains(&CodePointPosition::Above));
        assert!(set.contains(&CodePointPosition::Under));
        assert!(set.contains(&CodePointPosition::After));
        assert_eq!(set.len(), 3); // No duplicates
    }

    // ── Debug trait tests ────────────────────────────────────────────

    #[test]
    fn debug_formats_as_expected() {
        let above = CodePointPosition::Above;
        let debug_str = format!("{:?}", above);

        assert_eq!(debug_str, "Above");
    }

    #[test]
    fn debug_all_variants_readable() {
        let positions = [
            (CodePointPosition::Above, "Above"),
            (CodePointPosition::After, "After"),
            (CodePointPosition::InBetween, "InBetween"),
            (CodePointPosition::Under, "Under"),
        ];

        for (pos, expected_name) in &positions {
            let debug_str = format!("{:?}", pos);
            assert_eq!(
                &debug_str, expected_name,
                "Debug output for {:?} should be '{}'",
                pos, expected_name
            );
        }
    }

    // ── Pattern matching tests ───────────────────────────────────────

    #[test]
    fn match_statement_works() {
        let pos = CodePointPosition::InBetween;

        let result = match pos {
            CodePointPosition::Above => "above",
            CodePointPosition::After => "after",
            CodePointPosition::InBetween => "in-between",
            CodePointPosition::Under => "under",
        };

        assert_eq!(result, "in-between");
    }

    #[test]
    fn match_with_wildcard_still_comprehensive() {
        let test_cases = vec![
            CodePointPosition::Above,
            CodePointPosition::After,
            CodePointPosition::InBetween,
            CodePointPosition::Under,
        ];

        for pos in test_cases {
            let result = match pos {
                CodePointPosition::Above | CodePointPosition::Under => "vertical",
                CodePointPosition::After | CodePointPosition::InBetween => "horizontal",
            };

            assert!(!result.is_empty());
        }
    }

    // ── Non-exhaustive attribute tests ───────────────────────────────

    #[test]
    fn variants_can_be_constructed_explicitly() {
        // Even though #[non_exhaustive] is set, existing variants are accessible
        let _above = CodePointPosition::Above;
        let _after = CodePointPosition::After;
        let _in_between = CodePointPosition::InBetween;
        let _under = CodePointPosition::Under;
    }

    #[test]
    fn default_works_with_non_exhaustive() {
        // #[non_exhaustive] requires Default to be implemented for matching
        let pos = CodePointPosition::default();

        // Should be able to match with wildcard
        let category = match pos {
            CodePointPosition::Above => "vertical-above",
            CodePointPosition::Under => "vertical-under",
            CodePointPosition::After | CodePointPosition::InBetween => "horizontal",
            #[allow(unreachable_patterns)]
            _ => "future-variant",
        };

        assert_eq!(category, "vertical-under");
    }

    // ── Usage scenario tests ─────────────────────────────────────────

    #[test]
    fn position_used_in_struct() {
        #[derive(Debug, PartialEq)]
        struct AccentInfo {
            name: &'static str,
            position: CodePointPosition,
        }

        let accent = AccentInfo {
            name: "Meteg",
            position: CodePointPosition::Under,
        };

        assert_eq!(accent.position, CodePointPosition::Under);
    }

    #[test]
    fn position_in_collection() {
        let positions = vec![
            CodePointPosition::Above,
            CodePointPosition::Under,
            CodePointPosition::After,
            CodePointPosition::InBetween,
        ];

        assert_eq!(positions.len(), 4);
        assert!(positions.contains(&CodePointPosition::Above));
    }

    #[test]
    fn position_filtering() {
        let all_positions = vec![
            CodePointPosition::Above,
            CodePointPosition::Under,
            CodePointPosition::After,
            CodePointPosition::InBetween,
            CodePointPosition::Under, // Duplicate
        ];

        let under_count: usize = all_positions
            .iter()
            .filter(|&&p| p == CodePointPosition::Under)
            .count();

        assert_eq!(under_count, 2);
    }

    // ── Semantic meaning tests ───────────────────────────────────────

    #[test]
    fn under_semantic_meaning() {
        // Under is the default (most common position for Hebrew vowels)
        let default_pos = CodePointPosition::default();
        assert_eq!(default_pos, CodePointPosition::Under);
    }

    #[test]
    fn after_semantic_meaning() {
        // After is used for punctuation at word boundary
        let pos = CodePointPosition::After;
        assert!(matches!(pos, CodePointPosition::After));
    }

    #[test]
    fn in_between_semantic_meaning() {
        // InBetween is used for separators between words/clauses
        let pos = CodePointPosition::InBetween;
        assert!(matches!(pos, CodePointPosition::InBetween));
    }

    // ── Byte size and layout tests ───────────────────────────────────

    #[test]
    fn enum_has_minimal_size() {
        // As a unit enum with Copy, should have minimal memory footprint
        assert!(std::mem::size_of::<CodePointPosition>() <= std::mem::size_of::<u8>());
    }

    #[test]
    fn enum_has_no_padding() {
        // Unit variants should not add padding
        assert_eq!(
            std::mem::size_of::<CodePointPosition>(),
            std::mem::size_of::<u8>()
        );
    }

    #[test]
    fn alignement_is_reasonable() {
        // Alignment should be at least 1 byte
        assert!(std::mem::align_of::<CodePointPosition>() >= 1);
    }

    // ── Thread safety tests ──────────────────────────────────────────

    #[test]
    fn send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<CodePointPosition>();
    }

    // ── Const evaluation tests ───────────────────────────────────────

    #[test]
    fn const_position() {
        const UNDER_POS: CodePointPosition = CodePointPosition::Under;
        assert_eq!(UNDER_POS, CodePointPosition::Under);
    }

    // ── Comprehensive integration tests ──────────────────────────────

    #[test]
    fn full_test_coverage_matrix() {
        // Test all combinations of traits and behaviors
        let positions = [
            CodePointPosition::Above,
            CodePointPosition::After,
            CodePointPosition::InBetween,
            CodePointPosition::Under,
        ];

        for &pos in &positions {
            // Debug formatting works
            let _debug = format!("{:?}", pos);

            // PartialEq works
            assert_eq!(pos, pos);

            // Clone works
            let cloned = pos.clone();
            assert_eq!(cloned, pos);

            // Hash works
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            pos.hash(&mut hasher);
            let _hash = hasher.finish();

            // Copy works
            let copied = pos;
            assert_eq!(copied, pos);
        }
    }

    #[test]
    fn default_among_all_variants() {
        let all_positions = vec![
            CodePointPosition::Above,
            CodePointPosition::After,
            CodePointPosition::InBetween,
            CodePointPosition::Under,
        ];

        let default_count: usize = all_positions
            .iter()
            .filter(|&&p| p == CodePointPosition::default())
            .count();

        assert_eq!(default_count, 1, "Only one variant should be default");
        assert_eq!(CodePointPosition::default(), CodePointPosition::Under);
    }
}
