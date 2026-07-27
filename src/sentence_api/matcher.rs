/// Represents a single match if the accent is found
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    /// The matched HebrewAccent
    haystack: &'h str,
    /// Start byte of the match
    start: usize,
    /// End byte of the match
    end: usize,
}

impl<'h> Match<'h> {
    /// Returns the byte offset of the start of the match in the haystack.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }
    /// Returns the byte offset of the end of the match in the haystack.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }
    /// Returns true if and only if this match has a length of zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    /// Returns the length, in bytes, of this match.
    #[inline]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    /// Returns the range from start till end (byte offsets)
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }
    /// Returns the substring of the haystack that matched.
    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.haystack[self.range()]
    }
    /// Creates a new match from the given haystack and byte offsets.
    #[inline]
    pub(crate) fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {
        Match {
            haystack,
            start,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Helper macros ────────────────────────────────────────────────

    macro_rules! assert_match_invariants {
        ($match:expr) => {{
            let m = $match;
            // Invariant: end >= start
            assert!(
                m.end() >= m.start(),
                "End ({}) should be >= start ({})",
                m.end(),
                m.start()
            );
            // Invariant: len equals end - start
            assert_eq!(
                m.len(),
                m.end() - m.start(),
                "len() should equal end - start"
            );
            // Invariant: is_empty iff len == 0
            assert_eq!(
                m.is_empty(),
                m.len() == 0,
                "is_empty() should match len() == 0"
            );
            // Invariant: as_str().len() matches len()
            assert_eq!(
                m.as_str().len(),
                m.len(),
                "as_str().len() should match len()"
            );
        }};
    }

    // ── Basic construction tests ─────────────────────────────────────

    #[test]
    fn new_creates_valid_match() {
        let haystack = "וַיְהִי בְיָמֵ֗י";
        let match_result = Match::new(haystack, 0, 6);

        assert_eq!(match_result.start(), 0);
        assert_eq!(match_result.end(), 6);
        assert!(!match_result.is_empty());
        assert_eq!(match_result.len(), 6);
    }

    #[test]
    fn new_preserves_haystack_reference() {
        let haystack = "בְּרֵאשִׁ֖ית";
        let match_result = Match::new(haystack, 0, 3);

        assert_eq!(match_result.as_str(), "בְּרֵאשִׁ֖");
        // Verify the reference points to the correct slice
        assert!(haystack.contains(match_result.as_str()));
    }

    #[test]
    fn new_with_zero_length_match() {
        let haystack = "טקסט";
        let match_result = Match::new(haystack, 2, 2);

        assert!(match_result.is_empty());
        assert_eq!(match_result.len(), 0);
        assert_eq!(match_result.as_str(), "");
    }

    #[test]
    fn new_full_range_matches_entire_haystack() {
        let haystack = "Hello World";
        let match_result = Match::new(haystack, 0, haystack.len());

        assert_eq!(match_result.as_str(), haystack);
        assert_eq!(match_result.len(), haystack.len());
    }

    // ── Method getter tests ──────────────────────────────────────────

    #[test]
    fn start_returns_correct_byte_offset() {
        let haystack = "וַיְהִי"; // Hebrew text with combining marks
        let match_result = Match::new(haystack, 3, 9);

        assert_eq!(match_result.start(), 3);
    }

    #[test]
    fn end_returns_correct_byte_offset() {
        let haystack = "בְּרֵאשִׁ֖ית";
        let match_result = Match::new(haystack, 0, 12);

        assert_eq!(match_result.end(), 12);
    }

    #[test]
    fn len_calculates_correctly() {
        let haystack = "test string";
        let test_cases = vec![(0, 4, 4), (5, 11, 6), (0, 11, 11), (3, 3, 0)];

        for (start, end, expected_len) in test_cases {
            let m = Match::new(haystack, start, end);
            assert_eq!(m.len(), expected_len, "Failed for range {}..{}", start, end);
        }
    }

    #[test]
    fn range_returns_correct_range_object() {
        let haystack = "sample text";
        let match_result = Match::new(haystack, 2, 8);

        let range = match_result.range();
        assert_eq!(range.start, 2);
        assert_eq!(range.end, 8);
        assert_eq!(range, 2..8);
    }

    #[test]
    fn as_str_returns_substring_slice() {
        let haystack = "Hebrew accent detection";
        let match_result = Match::new(haystack, 7, 14);

        assert_eq!(match_result.as_str(), "accent");
        // Verify it's the same length as len()
        assert_eq!(match_result.as_str().len(), match_result.len());
    }

    // ── is_empty and edge case tests ─────────────────────────────────

    #[test]
    fn is_empty_true_when_start_equals_end() {
        let haystack = "text";
        let empty_match = Match::new(haystack, 5, 5);

        assert!(empty_match.is_empty());
    }

    #[test]
    fn is_empty_false_when_start_less_than_end() {
        let haystack = "text";
        let non_empty_match = Match::new(haystack, 0, 4);

        assert!(!non_empty_match.is_empty());
    }

    #[test]
    fn empty_match_has_zero_length() {
        let haystack = "anything";
        let empty_match = Match::new(haystack, 0, 0);

        assert!(empty_match.is_empty());
        assert_eq!(empty_match.len(), 0);
    }

    #[test]
    fn empty_match_returns_empty_string() {
        let haystack = "anything";
        let empty_match = Match::new(haystack, 3, 3);

        assert_eq!(empty_match.as_str(), "");
    }

    #[test]
    fn single_byte_match() {
        let haystack = "abc";
        let match_result = Match::new(haystack, 1, 2);

        assert_eq!(match_result.len(), 1);
        assert!(!match_result.is_empty());
        assert_eq!(match_result.as_str(), "b");
    }

    // ── Unicode/Hebrew text specific tests ───────────────────────────

    #[test]
    fn match_with_hebrew_and_cantillation_marks() {
        // Hebrew text with cantillation marks (multiple bytes per character)
        let haystack = "וַיְהִ֣י בְיָמֵ֗י";
        let match_result = Match::new(haystack, 0, 12);

        assert_eq!(match_result.as_str(), "וַיְהִ֣י");
        assert!(match_result.len() > 6); // Multi-byte characters
    }

    #[test]
    fn match_across_unicode_boundary_does_not_panic() {
        // Ensure we're working with byte offsets, not char counts
        let haystack = "αβγ δε"; // Greek letters
        let match_result = Match::new(haystack, 0, 6); // Byte offset that ends mid-char

        // Should still work (even if we split a character - the struct doesn't validate UTF-8)
        // This test ensures no panic occurs
        let _ = match_result.len();
    }

    #[test]
    fn match_middle_of_hebrew_word() {
        let haystack = "בְּרֵאשִׁ֖ית";
        let match_result = Match::new(haystack, 4, 10);

        let sliced = match_result.as_str();
        assert!(!sliced.is_empty());
        assert!(haystack.contains(sliced));
    }

    // ── Trait implementation tests ───────────────────────────────────

    #[test]
    fn debug_trait_implemented() {
        let haystack = "test";
        let match_result = Match::new(haystack, 1, 3);

        let debug_output = format!("{:?}", match_result);
        assert!(debug_output.contains("Match"));
    }

    #[test]
    fn copy_trait_works() {
        let haystack = "original";
        let original = Match::new(haystack, 0, 8);

        // Copy doesn't consume the value
        let copied = original;
        let still_original = original;

        assert_eq!(copied.start(), still_original.start());
        assert_eq!(copied.end(), still_original.end());
    }

    #[test]
    fn clone_trait_works() {
        let haystack = "cloned";
        let original = Match::new(haystack, 2, 6);
        let cloned = original.clone();

        assert_eq!(cloned.start(), original.start());
        assert_eq!(cloned.end(), original.end());
        assert_eq!(cloned.as_str(), original.as_str());
    }

    #[test]
    fn eq_trait_works() {
        let haystack = "equal_test";
        let m1 = Match::new(haystack, 0, 5);
        let m2 = Match::new(haystack, 0, 5);
        let m3 = Match::new(haystack, 1, 5);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn eq_trait_same_offsets_different_haystack() {
        let haystack1 = "first";
        let haystack2 = "second";

        let m1 = Match::new(haystack1, 0, 5);
        let m2 = Match::new(haystack2, 0, 5);

        // Eq compares start/end values, not haystack content
        assert_eq!(m1.start(), m2.start());
        assert_eq!(m1.end(), m2.end());
    }

    #[test]
    fn partial_eq_compares_fields_correctly() {
        let h1 = "text1";
        let h2 = "text2";

        let m1 = Match::new(h1, 1, 4);
        let m2 = Match::new(h1, 1, 4);
        let m3 = Match::new(h2, 1, 4);
        let m4 = Match::new(h1, 2, 4);

        assert_eq!(m1, m2); // Same haystack, same offsets
        assert_eq!(m1, m3); // Different haystacks, same offsets (Eq only checks fields)
        assert_ne!(m1, m4); // Same haystack, different offsets
    }

    // ── Range-related tests ──────────────────────────────────────────

    #[test]
    fn range_contains_start() {
        let m = Match::new("test", 5, 10);
        let range = m.range();

        assert!(range.contains(&5));
    }

    #[test]
    fn range_excludes_end() {
        let m = Match::new("test", 0, 4);
        let range = m.range();

        assert!(!range.contains(&4));
        assert!(range.contains(&3));
    }

    #[test]
    fn range_can_be_used_for_slicing() {
        let haystack = "abcdefg";
        let m = Match::new(haystack, 2, 5);
        let range = m.range();

        assert_eq!(&haystack[range], m.as_str());
    }

    // ── Invariant preservation tests ─────────────────────────────────

    #[test]
    fn invariant_end_greater_equal_start() {
        // Test with various ranges where end >= start
        let cases = vec![
            ("text", 0, 0),
            ("text", 0, 4),
            ("text", 2, 3),
            ("text", 4, 4),
        ];

        for (haystack, start, end) in cases {
            let m = Match::new(haystack, start, end);
            assert!(m.end() >= m.start());
        }
    }

    #[test]
    fn invariant_len_formula() {
        let haystack = "invariant test";
        let cases = vec![(0, 4), (2, 10), (5, 5), (12, 14)];

        for (start, end) in cases {
            let m = Match::new(haystack, start, end);
            assert_eq!(m.len(), m.end() - m.start());
        }
    }

    // ── Safety tests ─────────────────────────────────────────────────

    #[test]
    fn no_panic_on_large_offsets_within_bounds() {
        let haystack = "a".repeat(10000);
        let m = Match::new(&haystack, 5000, 9999);

        let _ = m.start();
        let _ = m.end();
        let _ = m.len();
        let _ = m.is_empty();
        let _ = m.as_str();
    }

    #[test]
    fn match_struct_is_send_sync() {
        // Verify Match can be used across threads (it should be since it's just refs and ints)
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Match>();
    }

    // ── Comprehensive integration tests ──────────────────────────────

    #[test]
    fn full_lifecycle_of_match() {
        let haystack = "complete lifecycle test";
        let m = Match::new(haystack, 9, 19);

        // All getters work
        assert_eq!(m.start(), 9);
        assert_eq!(m.end(), 19);
        assert_eq!(m.len(), 10);
        assert!(!m.is_empty());
        assert_eq!(m.as_str(), "lifecycle ");
        assert_eq!(m.range(), 9..19);

        // Can be copied
        let m2 = m;
        assert_eq!(m2.start(), 9);

        // Can be compared
        let m3 = Match::new(haystack, 9, 19);
        assert_eq!(m, m3);
    }

    #[test]
    fn multiple_matches_in_sequence() {
        let haystack = "first second third fourth";
        let matches = vec![
            Match::new(haystack, 0, 5),   // "first"
            Match::new(haystack, 6, 12),  // "second"
            Match::new(haystack, 13, 18), // "third"
            Match::new(haystack, 19, 25), // "fourth"
        ];

        let expected_words = ["first", "second", "third", "fourth"];

        for (i, m) in matches.iter().enumerate() {
            assert_eq!(m.as_str(), expected_words[i]);
            assert_match_invariants!(m);
        }
    }

    #[test]
    fn overlapping_ranges_different_matches() {
        let haystack = "overlap test";
        let m1 = Match::new(haystack, 0, 7); // "overlap"
        let m2 = Match::new(haystack, 4, 12); // "ap test"

        assert_ne!(m1, m2);
        assert_eq!(m1.start(), 0);
        assert_eq!(m2.end(), 12);
    }

    // ── Property-based style tests (manual) ──────────────────────────

    #[test]
    fn property_as_str_matches_range_slicing() {
        let haystack = "property test string";
        let start = 3;
        let end = 15;

        let m = Match::new(haystack, start, end);

        assert_eq!(m.as_str(), &haystack[m.range()]);
    }

    #[test]
    fn property_is_empty_iff_len_zero() {
        let cases = vec![(0, 0), (1, 1), (5, 5), (100, 100)];
        let haystack = "x";

        for (start, end) in cases {
            let m = Match::new(haystack, start.min(end), start.max(end).max(start));
            assert_eq!(m.is_empty(), m.len() == 0);
        }
    }
}
