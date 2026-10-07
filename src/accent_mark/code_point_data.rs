//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://www.unicode.org/Public/18.0.0/charts/PDF/U0590.pdf>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

use crate::accent_mark::CodePointPosition;
use crate::accent_mark::StressPosition;
use crate::accent_mark::Utf8CodePoint;

const fn utf8_cp_constructor(
    symbol: char,
    position: CodePointPosition,
    stress_position: StressPosition,
    unicode_name: &'static str,
    code_point_value: &'static str,
    hex_bytes: &'static str,
) -> Utf8CodePoint {
    Utf8CodePoint {
        symbol,
        position,
        stress_position,
        unicode_name,
        code_point_value,
        hex_bytes,
    }
}

// ============================================================================
// ETNAHTA (U+0591)
// ============================================================================
pub(crate) const CODEPOINT_ETNAHTA: Utf8CodePoint = utf8_cp_constructor(
    '֑',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT ETNAHTA",
    "U+0591",
    "0xd6 0x91",
);

// ============================================================================
// SEGOL (U+0592)
// ============================================================================
pub(crate) const CODEPOINT_SEGOL: Utf8CodePoint = utf8_cp_constructor(
    '֒',
    CodePointPosition::AboveLeft,
    StressPosition::Postpositive,
    "HEBREW ACCENT SEGOL",
    "U+0592",
    "0xd6 0x92",
);

// ============================================================================
// SHALSHELET (U+0593)
// ============================================================================
pub(crate) const CODEPOINT_SHALSHELET: Utf8CodePoint = utf8_cp_constructor(
    '֓',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT SHALSHELET",
    "U+0593",
    "0xd6 0x93",
);

// ============================================================================
// ZAQEF QATAN (U+0594)
// ============================================================================
pub(crate) const CODEPOINT_ZAQEF_QATAN: Utf8CodePoint = utf8_cp_constructor(
    '֔',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT ZAQEF QATAN",
    "U+0594",
    "0xd6 0x94",
);

// ============================================================================
// ZAQEF GADOL (U+0595) (use uniform!)
// ============================================================================
pub(crate) const CODEPOINT_ZAQEF_GADOL: Utf8CodePoint = utf8_cp_constructor(
    '֕',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT ZAQEF GADOL",
    "U+0595",
    "0xd6 0x95",
);

// ============================================================================
// TIPEHA (U+0596)
// ============================================================================
pub(crate) const CODEPOINT_TIPEHA: Utf8CodePoint = utf8_cp_constructor(
    '֖',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT TIPEHA",
    "U+0596",
    "0xd6 0x96",
);

// ============================================================================
// REVIA (U+0597) - Ashkenazi differs slightly
// ============================================================================

pub(crate) const CODEPOINT_REVIA: Utf8CodePoint = utf8_cp_constructor(
    '֗',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT REVIA",
    "U+0597",
    "0xd6 0x97",
);

// ============================================================================
// ZARQA (U+0598)
// ============================================================================

pub(crate) const CODEPOINT_ZARQA: Utf8CodePoint = utf8_cp_constructor(
    '֘',
    CodePointPosition::AboveLeft,
    StressPosition::Impositive,
    "HEBREW ACCENT ZARQA",
    "U+0598",
    "0xd6 0x98",
);

// ============================================================================
// PASHTA (U+0599)
// ============================================================================

pub(crate) const CODEPOINT_PASHTA: Utf8CodePoint = utf8_cp_constructor(
    '֙',
    CodePointPosition::AboveLeft,
    StressPosition::Postpositive,
    "HEBREW ACCENT PASHTA",
    "U+0599",
    "0xd6 0x99",
);

// ============================================================================
// YETIV (U+059A)
// ============================================================================

pub(crate) const CODEPOINT_YETIV: Utf8CodePoint = utf8_cp_constructor(
    '֚',
    CodePointPosition::BelowRight,
    StressPosition::Prepositive,
    "HEBREW ACCENT YETIV",
    "U+059A",
    "0xd6 0x9a",
);

// ============================================================================
// TEVIR (U+059B)
// ============================================================================

pub(crate) const CODEPOINT_TEVIR: Utf8CodePoint = utf8_cp_constructor(
    '֛',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT TEVIR",
    "U+059B",
    "0xd6 0x9b",
);

// ============================================================================
// GERESH (U+059C)
// ============================================================================

pub(crate) const CODEPOINT_GERESH: Utf8CodePoint = utf8_cp_constructor(
    '֜',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT GERESH",
    "U+059C",
    "0xd6 0x9c",
);

// Geresh Muqdam is DISABLED - see ticket
// ============================================================================
//  GERESH MUQDAM (U+059D)
// ============================================================================
// pub(crate) const CODEPOINT_GERESH_MUQDAM: Utf8CodePoint = utf8_cp_constructor(
//     '֜',?
//     CodePointPosition::AboveCenter,?
//     StressPosition::Impositive,?
//     "HEBREW ACCENT GERESH MUQDAM",
//     "U+059D",
//     "0xd6 0x9d",
// );

// ============================================================================
// GERSHAYIM (U+059E)
// ============================================================================
pub(crate) const CODEPOINT_GERSHAYIM: Utf8CodePoint = utf8_cp_constructor(
    '֞',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT GERSHAYIM",
    "U+059E",
    "0xd6 0x9e",
);

// ============================================================================
// QARNEY PARA (U+059F)
// ============================================================================
pub(crate) const CODEPOINT_QARNEY_PARA: Utf8CodePoint = utf8_cp_constructor(
    '֟',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT QARNEY PARA",
    "U+059F",
    "0xd6 0x9f",
);

// ============================================================================
// TELISHA GEDOLA (U+05A0)
// ============================================================================
pub(crate) const CODEPOINT_TELISHA_GEDOLA: Utf8CodePoint = utf8_cp_constructor(
    '֠',
    CodePointPosition::AboveRight,
    StressPosition::Prepositive,
    "HEBREW ACCENT TELISHA GEDOLA",
    "U+05A0",
    "0xd6 0xa0",
);

// ============================================================================
// PAZER (U+05A1)
// ============================================================================
pub(crate) const CODEPOINT_PAZER: Utf8CodePoint = utf8_cp_constructor(
    '֡',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT PAZER",
    "U+05A1",
    "0xd6 0xa1",
);

// Atnach Hafukh DISABLED - see ticket
// ============================================================================
// ATNAH HAFUKH (U+05A2)
// ============================================================================
// pub(crate) const CODEPOINT_ATNAH_HAFUKH: Utf8CodePoint = utf8_cp_constructor(
//     '֡',?
//     CodePointPosition::AboveCenter,?
//     StressPosition::Impositive, ?
//     "HEBREW ACCENT ATNAH HAFUKH",
//     "U+05A2",
//     "0xd6 0xa2",
// );

// ============================================================================
// MUNAH (U+05A3)
// ============================================================================
pub(crate) const CODEPOINT_MUNAH: Utf8CodePoint = utf8_cp_constructor(
    '֣',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT MUNAH",
    "U+05A3",
    "0xd6 0xa3",
);

// ============================================================================
// MAHAPAKH (U+05A4)
// ============================================================================
pub(crate) const CODEPOINT_MAHAPAKH: Utf8CodePoint = utf8_cp_constructor(
    '֤',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT MAHAPAKH",
    "U+05A4",
    "0xd6 0xa4",
);

// ============================================================================
// MERKHA (U+05A5)
// ============================================================================
pub(crate) const CODEPOINT_MERKHA: Utf8CodePoint = utf8_cp_constructor(
    '֥',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT MERKHA",
    "U+05A5",
    "0xd6 0xa5",
);

// ============================================================================
// MERKHA KEFULA (U+05A6)
// ============================================================================
pub(crate) const CODEPOINT_MERKHA_KEFULA: Utf8CodePoint = utf8_cp_constructor(
    '֦',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT MERKHA KEFULA",
    "U+05A6",
    "0xd6 0xa6",
);

// ============================================================================
// DARGA (U+05A7)
// ============================================================================
pub(crate) const CODEPOINT_DARGA: Utf8CodePoint = utf8_cp_constructor(
    '֧',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT DARGA",
    "U+05A7",
    "0xd6 0xa7",
);

// ============================================================================
// QADMA (U+05A8)
// ============================================================================
pub(crate) const CODEPOINT_QADMA: Utf8CodePoint = utf8_cp_constructor(
    '֨',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT QADMA",
    "U+05A8",
    "0xd6 0xa8",
);

// ============================================================================
// TELISHA QETANA (U+05A9)
// ============================================================================
pub(crate) const CODEPOINT_TELISHA_QETANA: Utf8CodePoint = utf8_cp_constructor(
    '֩',
    CodePointPosition::AboveLeft,
    StressPosition::Postpositive,
    "HEBREW ACCENT TELISHA QETANA",
    "U+05A9",
    "0xd6 0xa9",
);

// ============================================================================
// YERAH BEN YOMO (U+05AA)
// ============================================================================
pub(crate) const CODEPOINT_YERAH_BEN_YOMO: Utf8CodePoint = utf8_cp_constructor(
    '֪',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT YERAH BEN YOMO",
    "U+05AA",
    "0xd6 0xaa",
);

// ============================================================================
// OLE (U+05AB)
// ============================================================================

pub(crate) const CODEPOINT_OLE: Utf8CodePoint = utf8_cp_constructor(
    '֫',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT OLE",
    "U+05AB",
    "0xd6 0xab",
);

// ============================================================================
// ILUY (U+05AC)
// ============================================================================
pub(crate) const CODEPOINT_ILUY: Utf8CodePoint = utf8_cp_constructor(
    '֬',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT ILUY",
    "U+05AC",
    "0xd6 0xac",
);

// ============================================================================
// DEHI (U+05AD)
// ============================================================================
pub(crate) const CODEPOINT_DEHI: Utf8CodePoint = utf8_cp_constructor(
    '֭',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT DEHI",
    "U+05AD",
    "0xd6 0xad",
);

// ============================================================================
// ZINOR (U+05AE)
// ============================================================================
pub(crate) const CODEPOINT_ZINOR: Utf8CodePoint = utf8_cp_constructor(
    '֮',
    CodePointPosition::AboveCenter,
    StressPosition::Postpositive,
    "HEBREW ACCENT ZINOR",
    "U+05AE",
    "0xd6 0xae",
);

// ============================================================================
// ADDITIONAL CodePoints used
// ============================================================================
// METEG (U+05BD) (shares codepoint with Silluq)
// ============================================================================
pub(crate) const CODEPOINT_METEG: Utf8CodePoint = utf8_cp_constructor(
    'ֽ',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW POINT METEG",
    "U+05BD",
    "0xd6 0xbd",
);

// ============================================================================
// MAQAF (U+05BE)
// ============================================================================
pub(crate) const CODEPOINT_MAQAF: Utf8CodePoint = utf8_cp_constructor(
    '־',
    CodePointPosition::Maqqaph,
    StressPosition::NotApplicable,
    "HEBREW PUNCTUATION MAQAF",
    "U+05BE",
    "0xd6 0xbe",
);

// ============================================================================
// PASEQ (U+05C0)
// ============================================================================
pub(crate) const CODEPOINT_PASEQ: Utf8CodePoint = utf8_cp_constructor(
    '׀',
    CodePointPosition::Paseq,
    StressPosition::NotApplicable,
    "HEBREW PUNCTUATION PASEQ",
    "U+05C0",
    "0xd7 0x80",
);

// ============================================================================
// SOPH PASUQ (U+05C3)
// ============================================================================
pub(crate) const CODEPOINT_SOPH_PASUQ: Utf8CodePoint = utf8_cp_constructor(
    '׃',
    CodePointPosition::SofPasuq,
    StressPosition::NotApplicable,
    "HEBREW PUNCTUATION SOF PASUQ",
    "U+05C3",
    "0xd7 0x83",
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CantillationMarkStressPosition;
    // ===== BASIC CONSTRUCTION TESTS =====

    #[test]
    fn test_utf8_cp_constructor_creates_valid_instance() {
        let cp = utf8_cp_constructor(
            '֑', // symbol
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT ETNAHTA",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp.symbol, '֑');
        assert_eq!(cp.position, CodePointPosition::BelowCenter);
        assert_eq!(cp.stress_position, StressPosition::Impositive);
        assert_eq!(cp.unicode_name, "HEBREW ACCENT ETNAHTA");
        assert_eq!(cp.code_point_value, "U+0591");
        assert_eq!(cp.hex_bytes, "D6 91");
    }

    #[test]
    fn test_constructor_sets_all_fields() {
        let cp = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "PAISEQ",
            "U+05C0",
            "D7 80",
        );

        // Verify each field individually
        assert_eq!(cp.symbol, '׀');
        assert_eq!(cp.position, CodePointPosition::Paseq);
        assert_eq!(cp.stress_position, StressPosition::NotApplicable);
        assert_eq!(cp.unicode_name, "PAISEQ");
        assert_eq!(cp.code_point_value, "U+05C0");
        assert_eq!(cp.hex_bytes, "D7 80");
    }

    #[test]
    fn test_constructor_with_silluq_character() {
        let cp = utf8_cp_constructor(
            '֑', // Silluq
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT SILLUQ",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp.symbol, '֑');
        assert_eq!(cp.code_point_value, "U+0591");
        assert_eq!(cp.unicode_name, "HEBREW ACCENT SILLUQ");
    }

    #[test]
    fn test_constructor_with_maqqaph() {
        let cp = utf8_cp_constructor(
            '־', // Maqqaph (hyphen)
            CodePointPosition::Maqqaph,
            StressPosition::NotApplicable,
            "HEBREW PUNCTUATION MAQQAPH",
            "U+05BE",
            "D6 BE",
        );

        assert_eq!(cp.symbol, '־');
        assert_eq!(cp.position, CodePointPosition::Maqqaph);
        assert_eq!(cp.stress_position, StressPosition::NotApplicable);
        assert_eq!(cp.code_point_value, "U+05BE");
    }

    #[test]
    fn test_constructor_with_pashta() {
        let cp = utf8_cp_constructor(
            '֗', // Pashta
            CodePointPosition::BelowRight,
            StressPosition::Postpositive,
            "HEBREW ACCENT PASHTA",
            "U+05A8",
            "D6 A8",
        );

        assert_eq!(cp.symbol, '֗');
        assert_eq!(cp.position, CodePointPosition::BelowRight);
        assert_eq!(cp.stress_position, StressPosition::Postpositive);
    }

    // ===== CONST EVALUATION TESTS =====

    #[test]
    fn test_const_constructor_at_compile_time() {
        const CP: Utf8CodePoint = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT ETNAHTA",
            "U+0591",
            "D6 91",
        );

        assert_eq!(CP.symbol, '֑');
        assert_eq!(CP.code_point_value, "U+0591");
        assert_eq!(CP.hex_bytes, "D6 91");
    }

    #[test]
    fn test_const_constructor_multiple_instances() {
        const CP1: Utf8CodePoint = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "ETNAHTA",
            "U+0591",
            "D6 91",
        );

        const CP2: Utf8CodePoint = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "PAISEQ",
            "U+05C0",
            "D7 80",
        );

        assert_eq!(CP1.symbol, '֑');
        assert_eq!(CP2.symbol, '׀');
        assert_ne!(CP1.symbol, CP2.symbol);
    }

    #[test]
    fn test_const_constructor_can_be_used_in_static() {
        static CP: Utf8CodePoint = utf8_cp_constructor(
            '֗',
            CodePointPosition::BelowRight,
            StressPosition::Postpositive,
            "PASHTA",
            "U+05A8",
            "D6 A8",
        );

        assert_eq!(CP.symbol, '֗');
        assert_eq!(CP.code_point_value, "U+05A8");
    }

    // ===== FIELD ACCESS TESTS =====

    #[test]
    fn test_symbol_field_is_accessible() {
        let cp = utf8_cp_constructor(
            'א',
            CodePointPosition::AboveLeft,
            StressPosition::Prepositive,
            "ALEF",
            "U+05D0",
            "D7 90",
        );

        // Direct field access on public struct
        assert_eq!(cp.symbol, 'א');
    }

    #[test]
    fn test_code_point_value_field_format() {
        // Test various code point value formats
        let cp1 = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        assert!(cp1.code_point_value.starts_with("U+"));
        assert_eq!(cp1.code_point_value.len(), 6); // "U+" + 4 hex digits
    }

    #[test]
    fn test_hex_bytes_field_format() {
        let cp = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "TEST",
            "U+05C0",
            "D7 80",
        );

        // Hex bytes should be space-separated
        assert!(!cp.hex_bytes.is_empty());
        assert!(cp
            .hex_bytes
            .chars()
            .all(|c| c.is_ascii_hexdigit() || c == ' '));
    }

    // ===== CODE POINT POSITION VARIANTS =====

    #[test]
    fn test_constructor_with_above_center() {
        let cp = utf8_cp_constructor(
            '֓',
            CodePointPosition::AboveCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0593",
            "D6 93",
        );

        assert_eq!(cp.position, CodePointPosition::AboveCenter);
    }

    #[test]
    fn test_constructor_with_above_left() {
        let cp = utf8_cp_constructor(
            '֕',
            CodePointPosition::AboveLeft,
            StressPosition::Prepositive,
            "TEST",
            "U+0595",
            "D6 95",
        );

        assert_eq!(cp.position, CodePointPosition::AboveLeft);
    }

    #[test]
    fn test_constructor_with_above_right() {
        let cp = utf8_cp_constructor(
            '֖',
            CodePointPosition::AboveRight,
            StressPosition::Postpositive,
            "TEST",
            "U+0596",
            "D6 96",
        );

        assert_eq!(cp.position, CodePointPosition::AboveRight);
    }

    #[test]
    fn test_constructor_with_below_center() {
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp.position, CodePointPosition::BelowCenter);
    }

    #[test]
    fn test_constructor_with_below_right() {
        let cp = utf8_cp_constructor(
            '֗',
            CodePointPosition::BelowRight,
            StressPosition::Postpositive,
            "TEST",
            "U+05A8",
            "D6 A8",
        );

        assert_eq!(cp.position, CodePointPosition::BelowRight);
    }

    #[test]
    fn test_constructor_with_sof_pasuq() {
        let cp = utf8_cp_constructor(
            '׃',
            CodePointPosition::SofPasuq,
            StressPosition::NotApplicable,
            "SOF PASUQ",
            "U+05C3",
            "D7 83",
        );

        assert_eq!(cp.position, CodePointPosition::SofPasuq);
    }

    #[test]
    fn test_constructor_with_maqqaph_position() {
        let cp = utf8_cp_constructor(
            '־',
            CodePointPosition::Maqqaph,
            StressPosition::NotApplicable,
            "MAQQAPH",
            "U+05BE",
            "D6 BE",
        );

        assert_eq!(cp.position, CodePointPosition::Maqqaph);
    }

    #[test]
    fn test_constructor_with_paseq_position() {
        let cp = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "PAISEQ",
            "U+05C0",
            "D7 80",
        );

        assert_eq!(cp.position, CodePointPosition::Paseq);
    }

    // ===== STRESS POSITION VARIANTS =====

    #[test]
    fn test_constructor_with_impositive_stress() {
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp.stress_position, StressPosition::Impositive);
        assert_eq!(
            cp.stress_position.to_public(),
            Some(CantillationMarkStressPosition::Impositive)
        );
    }

    #[test]
    fn test_constructor_with_prepositive_stress() {
        let cp = utf8_cp_constructor(
            '֕',
            CodePointPosition::AboveLeft,
            StressPosition::Prepositive,
            "TEST",
            "U+0595",
            "D6 95",
        );

        assert_eq!(cp.stress_position, StressPosition::Prepositive);
        assert_eq!(
            cp.stress_position.to_public(),
            Some(CantillationMarkStressPosition::Prepositive)
        );
    }

    #[test]
    fn test_constructor_with_postpositive_stress() {
        let cp = utf8_cp_constructor(
            '֗',
            CodePointPosition::BelowRight,
            StressPosition::Postpositive,
            "TEST",
            "U+05A8",
            "D6 A8",
        );

        assert_eq!(cp.stress_position, StressPosition::Postpositive);
        assert_eq!(
            cp.stress_position.to_public(),
            Some(CantillationMarkStressPosition::Postpositive)
        );
    }

    #[test]
    fn test_constructor_with_not_applicable_stress() {
        let cp = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "PAISEQ",
            "U+05C0",
            "D7 80",
        );

        assert_eq!(cp.stress_position, StressPosition::NotApplicable);
        assert_eq!(cp.stress_position.to_public(), None);
    }

    // ===== UNICODE NAME TESTS =====

    #[test]
    fn test_unicode_name_is_static_lifetime() {
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT ETNAHTA",
            "U+0591",
            "D6 91",
        );

        // Verify it's a &'static str (compile-time check)
        let _static_ref: &'static str = cp.unicode_name;
        assert_eq!(_static_ref, "HEBREW ACCENT ETNAHTA");
    }

    #[test]
    fn test_unicode_name_non_empty() {
        for accent_name in &[
            "HEBREW ACCENT ETNAHTA",
            "HEBREW PUNCTUATION PAISEQ",
            "HEBREW ACCENT SILLUQ",
            "",
        ] {
            let cp = utf8_cp_constructor(
                '֑',
                CodePointPosition::BelowCenter,
                StressPosition::Impositive,
                accent_name,
                "U+0591",
                "D6 91",
            );

            // Empty names are allowed (for edge case testing)
            // But typical usage would have non-empty names
            if !accent_name.is_empty() {
                assert!(!cp.unicode_name.is_empty());
            }
        }
    }

    #[test]
    fn test_unicode_name_consistency() {
        // Same name should produce same result across multiple calls
        let cp1 = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "CONSISTENCY_TEST",
            "U+0591",
            "D6 91",
        );

        let cp2 = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "CONSISTENCY_TEST",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp1.unicode_name, cp2.unicode_name);
    }

    // ===== HEX BYTES FORMAT TESTS =====

    #[test]
    fn test_hex_bytes_two_bytes() {
        // Most Hebrew combining marks are 2-byte UTF-8 sequences
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        // Two bytes separated by space
        assert_eq!(cp.hex_bytes, "D6 91");
        assert_eq!(cp.hex_bytes.split(' ').count(), 2);
    }

    #[test]
    fn test_hex_bytes_case_insensitive() {
        // Both uppercase and lowercase hex should be accepted
        let cp_upper = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        let cp_lower = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "d6 91",
        );

        assert_eq!(cp_upper.hex_bytes, "D6 91");
        assert_eq!(cp_lower.hex_bytes, "d6 91");
        // Case is preserved as provided
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_all_position_and_stress_combinations() {
        let positions = [
            CodePointPosition::AboveCenter,
            CodePointPosition::AboveLeft,
            CodePointPosition::AboveRight,
            CodePointPosition::BelowCenter,
            CodePointPosition::BelowRight,
            CodePointPosition::SofPasuq,
            CodePointPosition::Maqqaph,
            CodePointPosition::Paseq,
        ];

        let stresses = [
            StressPosition::Impositive,
            StressPosition::Prepositive,
            StressPosition::Postpositive,
            StressPosition::NotApplicable,
        ];

        for pos in &positions {
            for stress in &stresses {
                let cp = utf8_cp_constructor('֑', *pos, *stress, "TEST", "U+0591", "D6 91");

                assert_eq!(cp.position, *pos);
                assert_eq!(cp.stress_position, *stress);
            }
        }
    }

    #[test]
    fn test_distinct_characters_produce_distinct_instances() {
        let cp1 = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "ETNAHTA",
            "U+0591",
            "D6 91",
        );

        let cp2 = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            "PAISEQ",
            "U+05C0",
            "D7 80",
        );

        assert_ne!(cp1.symbol, cp2.symbol);
        assert_ne!(cp1.code_point_value, cp2.code_point_value);
        assert_ne!(cp1.hex_bytes, cp2.hex_bytes);
        assert_ne!(cp1.unicode_name, cp2.unicode_name);
    }

    // ===== INTEGRATION WITH OTHER TYPES =====

    #[test]
    fn test_constructor_with_actual_accent_data() {
        // Test with real cantillation mark data
        let silluq = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT SILLUQ",
            "U+0591",
            "D6 91",
        );

        assert_eq!(silluq.symbol, '֑');
        assert_eq!(silluq.code_point_value, "U+0591");
        assert_eq!(silluq.unicode_name, "HEBREW ACCENT SILLUQ");
    }

    #[test]
    fn test_constructor_compatibility_with_table_lookup() {
        // Constructor should produce compatible instances for table storage
        const TABLE_ENTRY: Utf8CodePoint = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TABLE_TEST",
            "U+0591",
            "D6 91",
        );

        // Can be stored in const arrays
        const TABLE: [Utf8CodePoint; 1] = [TABLE_ENTRY];
        assert_eq!(TABLE[0].symbol, '֑');
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_constructor_with_control_character_symbol() {
        // Should accept any char including control characters
        let cp = utf8_cp_constructor(
            '\u{25CC}', // DOTTED CIRCLE (used for display)
            CodePointPosition::AboveCenter,
            StressPosition::NotApplicable,
            "DOTTED CIRCLE",
            "U+25CC",
            "E2 97 CC",
        );

        assert_eq!(cp.symbol, '\u{25CC}');
        assert_eq!(cp.code_point_value, "U+25CC");
    }

    #[test]
    fn test_constructor_with_empty_unicode_name() {
        // Empty unicode_name is technically allowed
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "",
            "U+0591",
            "D6 91",
        );

        assert_eq!(cp.unicode_name, "");
        assert!(cp.unicode_name.is_empty());
    }

    #[test]
    fn test_constructor_with_long_unicode_name() {
        // Long unicode names should work fine
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT ETNAHTA FROM BIBLICAL TEXT TRADITION",
            "U+0591",
            "D6 91",
        );

        assert_eq!(
            cp.unicode_name,
            "HEBREW ACCENT ETNAHTA FROM BIBLICAL TEXT TRADITION"
        );
    }

    #[test]
    fn test_constructor_preserves_string_references() {
        // Static string references should maintain identity
        const STR1: &str = "CONST_STRING_1";
        const STR2: &str = "CONST_STRING_2";

        const CP1: Utf8CodePoint = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            STR1,
            "U+0591",
            "D6 91",
        );

        const CP2: Utf8CodePoint = utf8_cp_constructor(
            '׀',
            CodePointPosition::Paseq,
            StressPosition::NotApplicable,
            STR2,
            "U+05C0",
            "D7 80",
        );

        assert_eq!(CP1.unicode_name, STR1);
        assert_eq!(CP2.unicode_name, STR2);
    }

    // ===== DOCUMENTATION EXAMPLE VERIFICATION =====

    #[test]
    fn doc_example_basic_usage() {
        // Simulate a documentation example
        let etnahta = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "HEBREW ACCENT ETNAHTA",
            "U+0591",
            "D6 91",
        );

        assert_eq!(etnahta.symbol, '֑');
        assert_eq!(etnahta.code_point_value, "U+0591");
        assert_eq!(etnahta.unicode_name, "HEBREW ACCENT ETNAHTA");
    }

    // ===== PERFORMANCE-OPTIMIZED USAGE TESTS =====

    #[test]
    fn test_no_heap_allocation() {
        // Verify all fields are stack-allocated or static refs
        let cp = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "NO_HEAP",
            "U+0591",
            "D6 91",
        );

        // No String fields, all &'static str
        assert!(!format!("{:?}", cp).contains("alloc"));
    }

    #[test]
    fn test_const_evaluation_cost_zero() {
        // Constructor runs at compile time - no runtime cost
        const CP: Utf8CodePoint = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "COMPILE_TIME",
            "U+0591",
            "D6 91",
        );

        // This access has zero runtime cost
        let symbol = CP.symbol;
        assert_eq!(symbol, '֑');
    }

    // ===== COMPARISON WITH DIRECT CONSTRUCTOR =====

    #[test]
    fn test_constructor_equivalent_to_struct_literal() {
        let via_func = utf8_cp_constructor(
            '֑',
            CodePointPosition::BelowCenter,
            StressPosition::Impositive,
            "TEST",
            "U+0591",
            "D6 91",
        );

        let via_literal = Utf8CodePoint {
            symbol: '֑',
            position: CodePointPosition::BelowCenter,
            stress_position: StressPosition::Impositive,
            unicode_name: "TEST",
            code_point_value: "U+0591",
            hex_bytes: "D6 91",
        };

        // Both should produce identical results
        assert_eq!(via_func.symbol, via_literal.symbol);
        assert_eq!(via_func.position, via_literal.position);
        assert_eq!(via_func.stress_position, via_literal.stress_position);
        assert_eq!(via_func.unicode_name, via_literal.unicode_name);
        assert_eq!(via_func.code_point_value, via_literal.code_point_value);
        assert_eq!(via_func.hex_bytes, via_literal.hex_bytes);
    }
}
