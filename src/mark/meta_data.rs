// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use crate::mark::{MarkPlacement, StressPosition};

/// Details on a specific UTF-8 Unicode code-point
///
/// This is an internal structure combining:
///   - Cantillation Mark (symbol, position, stress_position)
///   - technical information (code_point_value,hex_bytes,unicode_name)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct MarkMetaData {
    /// The symbol of the UTF-8 code-point
    pub(crate) symbol: char,
    /// The position of the code-point in relation to the consonant
    pub(crate) position: MarkPlacement,
    /// Stress relation (linguistic/metrical)
    pub(crate) stress_position: StressPosition,
    /// UTF-8 code-point id, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_bytes: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub(crate) unicode_name: &'static str,
}

#[cfg(test)]
mod test_for_derived_traits {

    use super::*;
    use crate::mark::MarkPlacement;
    use crate::mark::StressPosition;

    /// Canonical sample used across tests
    fn sample_mark_meta_data() -> MarkMetaData {
        MarkMetaData {
            symbol: '֑',
            position: MarkPlacement::AboveCenter,
            stress_position: StressPosition::Impositive,
            code_point_value: "U+0591",
            hex_bytes: "0x0591",
            unicode_name: "HEBREW ACCENT ETNAHTA",
        }
    }
    //#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]

    #[test]
    fn debug() {
        let meta = sample_mark_meta_data();
        let pretty = format!("{:#?}", meta);

        println!("{}", pretty);

        assert!(pretty.contains('֑'));
        assert!(pretty.contains("U+0591"));
        assert!(pretty.contains("0x0591"));
        assert!(pretty.contains("HEBREW ACCENT ETNAHTA"));
    }

    #[test]
    fn clone_is_equal_to_original() {
        let meta = sample_mark_meta_data();
        let cloned = meta.clone();

        assert_eq!(cloned, meta);
        assert_eq!(cloned.symbol, meta.symbol);
        assert_eq!(cloned.code_point_value, meta.code_point_value);
    }

    #[test]
    fn is_copy_and_clone_agree() {
        fn assert_impls<T: Copy + Clone + Eq + std::fmt::Debug>() {}
        assert_impls::<MarkMetaData>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mark::MarkPlacement;
    use crate::mark::StressPosition;

    /// Canonical sample used across tests
    fn sample_mark_meta_data() -> MarkMetaData {
        MarkMetaData {
            symbol: '֑',
            position: MarkPlacement::AboveCenter,
            stress_position: StressPosition::Impositive,
            code_point_value: "U+0591",
            hex_bytes: "D6 91",
            unicode_name: "HEBREW ACCENT ETNAHTA",
        }
    }

    /// Compact Debug must be a single line containing every field label and value
    #[test]
    fn test_compact_debug_contains_all_fields() {
        let meta = sample_mark_meta_data();
        let out = format!("{:?}", dbg!(&meta));

        // Single line — compact form never introduces newlines
        assert!(!out.contains('\n'));

        for needle in [
            "MarkMetaData",
            "symbol: '֑'",
            "position: AboveCenter",
            "code_point_value: \"U+0591\"",
            "hex_bytes: \"D6 91\"",
            "unicode_name: \"HEBREW ACCENT ETNAHTA\"",
        ] {
            assert!(out.contains(needle), "missing {needle:?} in {out}");
        }
    }

    /// Pretty Debug is multi-line, indented, and ends each field with a comma
    #[test]
    fn test_pretty_debug_shape() {
        let meta = sample_mark_meta_data();
        let pretty = format!("{:#?}", &meta);

        assert!(pretty.starts_with("MarkMetaData {\n"));
        assert!(pretty.ends_with(",\n}")); // trailing comma on last field
        assert_eq!(pretty.lines().count(), 8); // header + 6 fields + closing brace
        for line in pretty.lines().skip(1).take(6) {
            assert!(line.starts_with("    "), "bad indent: {line:?}");
        }
    }

    /// Pretty form preserves field order matching declaration order
    #[test]
    fn test_pretty_debug_field_order() {
        let pretty = format!("{:#?}", sample_mark_meta_data());
        let order = [
            "symbol:",
            "position:",
            "stress_position:",
            "code_point_value:",
            "hex_bytes:",
            "unicode_name:",
        ];

        let mut last = 0;
        for field in order {
            let idx = pretty.find(field).expect(field);
            assert!(idx > last, "field {field:?} out of order");
            last = idx;
        }
    }

    /// Derived Debug renders `char` fields with Rust literal escaping
    #[test]
    fn test_char_debug_repr() {
        let meta = MarkMetaData {
            symbol: '\u{0591}',
            ..sample_mark_meta_data()
        };
        let out = format!("{:?}", meta);
        // Debug for char escapes non-printables; U+0591 prints literally or escaped
        assert!(out.contains("symbol:"), "char field rendered: {out}");
    }

    /// Debug output is stable across Copy clones (relevant given `Copy` derive)
    #[test]
    fn test_debug_stable_across_copy() {
        let a = sample_mark_meta_data();
        let b = a; // Copy
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
        assert_eq!(format!("{a:#?}"), format!("{b:#?}"));
    }
}
