//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://utf8-chartable.de/unicode-utf8-table.pl>)
//! - naming of the accents according **to** different traditions:
//!   - <https://en.wikipedia.org/wiki/Hebrew_cantillation>
//!   - <http://textus-receptus.com/wiki/Cantillation#Names_and_shapes_of_the_ta.27amim>
//! - the position of the accent relative to the related consonant

use crate::accent::Utf8CodePoint;
use crate::codepoints::CodePointPosition;

const fn utf8_cp_constructor(
    code_point_value: &'static str,
    hex_bytes: &'static str,
    canonical_name: &'static str,
    symbol: &'static str,
    position: CodePointPosition,
) -> Utf8CodePoint {
    Utf8CodePoint {
        code_point_value,
        hex_bytes,
        canonical_name,
        symbol,
        position,
    }
}

// ============================================================================
// ETNAHTA (U+0591)
// ============================================================================
pub(crate) const CODEPOINT_ETNAHTA: Utf8CodePoint = utf8_cp_constructor(
    "U+0591",
    "0xd6 0x91",
    "HEBREW ACCENT ETNAHTA",
    "֑",
    CodePointPosition::Under,
);

// ============================================================================
// SEGOL (U+0592) - Only 3 traditions
// ============================================================================

pub(crate) const CODEPOINT_SEGOL: Utf8CodePoint = utf8_cp_constructor(
    "U+0592",
    "0xd6 0x92",
    "HEBREW ACCENT SEGOL",
    "֒",
    CodePointPosition::Above,
);

// ============================================================================
// SHALSHELET (U+0593)
// ============================================================================

pub(crate) const CODEPOINT_SHALSHELET: Utf8CodePoint = utf8_cp_constructor(
    "U+0593",
    "0xd6 0x93",
    "HEBREW ACCENT SHALSHELET",
    "֓",
    CodePointPosition::Above,
);

// ============================================================================
// ZAQEF QATAN (U+0594)
// ============================================================================

pub(crate) const CODEPOINT_ZAQEF_QATAN: Utf8CodePoint = utf8_cp_constructor(
    "U+0594",
    "0xd6 0x94",
    "HEBREW ACCENT ZAQEF QATAN",
    "֔",
    CodePointPosition::Above,
);

// ============================================================================
// ZAQEF GADOL (U+0595) - All identical (use uniform!)
// ============================================================================
pub(crate) const CODEPOINT_ZAQEF_GADOL: Utf8CodePoint = utf8_cp_constructor(
    "U+0595",
    "0xd6 0x95",
    "HEBREW ACCENT ZAQEF GADOL",
    "֕",
    CodePointPosition::Above,
);

// ============================================================================
// TIPEHA (U+0596)
// ============================================================================

pub(crate) const CODEPOINT_TIPEHA: Utf8CodePoint = utf8_cp_constructor(
    "U+0596",
    "0xd6 0x96",
    "HEBREW ACCENT TIPEHA",
    "֖",
    CodePointPosition::Under,
);

// ============================================================================
// REVIA (U+0597) - Ashkenazi differs slightly
// ============================================================================

pub(crate) const CODEPOINT_REVIA: Utf8CodePoint = utf8_cp_constructor(
    "U+0597",
    "0xd6 0x97",
    "HEBREW ACCENT REVIA",
    "֗",
    CodePointPosition::Above,
);

// ============================================================================
// ZARQA (U+0598)
// ============================================================================

pub(crate) const CODEPOINT_ZARQA: Utf8CodePoint = utf8_cp_constructor(
    "U+0598",
    "0xd6 0x98",
    "HEBREW ACCENT ZARQA",
    "֘",
    CodePointPosition::Above,
);

// ============================================================================
// PASHTA (U+0599)
// ============================================================================

pub(crate) const CODEPOINT_PASHTA: Utf8CodePoint = utf8_cp_constructor(
    "U+0599",
    "0xd6 0x99",
    "HEBREW ACCENT PASHTA",
    "֙",
    CodePointPosition::Above,
);

// ============================================================================
// YETIV (U+059A)
// ============================================================================

pub(crate) const CODEPOINT_YETIV: Utf8CodePoint = utf8_cp_constructor(
    "U+059A",
    "0xd6 0x9a",
    "HEBREW ACCENT YETIV",
    "֚",
    CodePointPosition::Under,
);

// ============================================================================
// TEVIR (U+059B)
// ============================================================================

pub(crate) const CODEPOINT_TEVIR: Utf8CodePoint = utf8_cp_constructor(
    "U+059B",
    "0xd6 0x9b",
    "HEBREW ACCENT TEVIR",
    "֛",
    CodePointPosition::Under,
);

// ============================================================================
// GERESH (U+059C)
// ============================================================================

pub(crate) const CODEPOINT_GERESH: Utf8CodePoint = utf8_cp_constructor(
    "U+059C",
    "0xd6 0x9c",
    "HEBREW ACCENT GERESH",
    "֜",
    CodePointPosition::Above,
);

// Geresh Muqdam is disabled - see ticket

// ============================================================================
// GERSHAYIM (U+059E)
// ============================================================================

pub(crate) const CODEPOINT_GERSHAYIM: Utf8CodePoint = utf8_cp_constructor(
    "U+059E",
    "0xd6 0x9e",
    "HEBREW ACCENT GERSHAYIM",
    "֞",
    CodePointPosition::Above,
);

// ============================================================================
// QARNEY PARA (U+059F) - All identical
// ============================================================================
pub(crate) const CODEPOINT_QARNEY_PARA: Utf8CodePoint = utf8_cp_constructor(
    "U+059F",
    "0xd6 0x9f",
    "HEBREW ACCENT QARNEY PARA",
    "֟",
    CodePointPosition::Above,
);

// ============================================================================
// TELISHA GEDOLA (U+05A0)
// ============================================================================
pub(crate) const CODEPOINT_TELISHA_GEDOLA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A0",
    "0xd6 0xa0",
    "HEBREW ACCENT TELISHA GEDOLA",
    "֠",
    CodePointPosition::Above,
);

// ============================================================================
// PAZER (U+05A1) - Only 3 traditions
// ============================================================================
pub(crate) const CODEPOINT_PAZER: Utf8CodePoint = utf8_cp_constructor(
    "U+05A1",
    "0xd6 0xa1",
    "HEBREW ACCENT PAZER",
    "֡",
    CodePointPosition::Above,
);

// At nah Hafukh disabled - see ticket

// ============================================================================
// MUNAH (U+05A3) - Only 3 traditions
// ============================================================================
pub(crate) const CODEPOINT_MUNAH: Utf8CodePoint = utf8_cp_constructor(
    "U+05A3",
    "0xd6 0xa3",
    "HEBREW ACCENT MUNAH",
    "֣",
    CodePointPosition::Under,
);

// ============================================================================
// MAHPAKH (U+05A4)
// ============================================================================
pub(crate) const CODEPOINT_MAHAPAKH: Utf8CodePoint = utf8_cp_constructor(
    "U+05A4",
    "0xd6 0xa4",
    "HEBREW ACCENT MAHAPAKH",
    "֤",
    CodePointPosition::Under,
);

// ============================================================================
// MERKHA (U+05A5)
// ============================================================================
pub(crate) const CODEPOINT_MERKHA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A5",
    "0xd6 0xa5",
    "HEBREW ACCENT MERKHA",
    "֥",
    CodePointPosition::Under,
);

// ============================================================================
// MERKHA KEFULA (U+05A6) - Only 3 traditions
// ============================================================================

pub(crate) const CODEPOINT_MERKHA_KEFULA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A6",
    "0xd6 0xa6",
    "HEBREW ACCENT MERKHA KEFULA",
    "֦",
    CodePointPosition::Under,
);

// ============================================================================
// DARGA (U+05A7) - All identical
// ============================================================================

pub(crate) const CODEPOINT_DARGA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A7",
    "0xd6 0xa7",
    "HEBREW ACCENT DARGA",
    "֧",
    CodePointPosition::Under,
);

// ============================================================================
// QADMA (U+05A8) - Only 3 traditions
// ============================================================================

pub(crate) const CODEPOINT_QADMA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A8",
    "0xd6 0xa8",
    "HEBREW ACCENT QADMA",
    "֨",
    CodePointPosition::Above,
);

// ============================================================================
// TELISHA QETANA (U+05A9)
// ============================================================================

pub(crate) const CODEPOINT_TELISHA_QETANA: Utf8CodePoint = utf8_cp_constructor(
    "U+05A9",
    "0xd6 0xa9",
    "HEBREW ACCENT TELISHA QETANA",
    "֩",
    CodePointPosition::Above,
);

// ============================================================================
// YERAH BEN YOMO (U+05AA) - All identical
// ============================================================================

pub(crate) const CODEPOINT_YERAH_BEN_YOMO: Utf8CodePoint = utf8_cp_constructor(
    "U+05AA",
    "0xd6 0xaa",
    "HEBREW ACCENT YERAH BEN YOMO",
    "֪",
    CodePointPosition::Under,
);

// ============================================================================
// OLE (U+05AB) - All identical
// ============================================================================

pub(crate) const CODEPOINT_OLE: Utf8CodePoint = utf8_cp_constructor(
    "U+05AB",
    "0xd6 0xab",
    "HEBREW ACCENT OLE",
    "֫",
    CodePointPosition::Above,
);

// ============================================================================
// ILUY (U+05AC) - All identical
// ============================================================================

pub(crate) const CODEPOINT_ILUY: Utf8CodePoint = utf8_cp_constructor(
    "U+05AC",
    "0xd6 0xac",
    "HEBREW ACCENT ILUY",
    "֬",
    CodePointPosition::Above,
);

// ============================================================================
// DEHI (U+05AD) - All identical
// ============================================================================

pub(crate) const CODEPOINT_DEHI: Utf8CodePoint = utf8_cp_constructor(
    "U+05AD",
    "0xd6 0xad",
    "HEBREW ACCENT DECHI",
    "֭",
    CodePointPosition::Under,
);

// ============================================================================
// ZINOR (U+05AE) - All identical
// ============================================================================

pub(crate) const CODEPOINT_ZINOR: Utf8CodePoint = utf8_cp_constructor(
    "U+05AE",
    "0xd6 0xae",
    "HEBREW ACCENT ZINOR",
    "֮",
    CodePointPosition::Above,
);

// ============================================================================
// SILLUQ (U+05BD) - All identical (same codepoint as Meteg, different semantics)
// ============================================================================

pub(crate) const CODEPOINT_SILLUQ: Utf8CodePoint = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT SILLUQ",
    "ֽ",
    CodePointPosition::Under,
);

// ============================================================================
// METEG (U+05BD) - All identical (shares codepoint with Silluq)
// ============================================================================

pub(crate) const CODEPOINT_METEG: Utf8CodePoint = utf8_cp_constructor(
    "U+05BD",
    "0xd6 0xbd",
    "HEBREW POINT METEG",
    "ֽ",
    CodePointPosition::Under,
);

// ============================================================================
// MAQAF (U+05BE) - No traditions
// ============================================================================

pub(crate) const CODEPOINT_MAQAF: Utf8CodePoint = utf8_cp_constructor(
    "U+05BE",
    "0xd6 0xbe",
    "HEBREW PUNCTUATION MAQAF",
    "־",
    CodePointPosition::After,
);

// ============================================================================
// PASEQ (U+05C0) - All identical
// ============================================================================

pub(crate) const CODEPOINT_PASEQ: Utf8CodePoint = utf8_cp_constructor(
    "U+05C0",
    "0xd7 0x80",
    "HEBREW PUNCTUATION PASEQ",
    "׀",
    CodePointPosition::After,
);

// ============================================================================
// SOPH PASUQ (U+05C3) - No traditions
// ============================================================================

pub(crate) const CODEPOINT_SOPH_PASUQ: Utf8CodePoint = utf8_cp_constructor(
    "U+05C3",
    "0xd7 0x83",
    "HEBREW PUNCTUATION SOF PASUQ",
    "׃",
    CodePointPosition::InBetween,
);

#[cfg(test)]
mod tests {

    // ── AccentName Construction ───────────────────────────────────────

    use crate::codepoints::traditions::AccentName;

    #[test]
    fn accent_name_constructed_with_all_fields() {
        let name = AccentName {
            hebrew_name: "טַעֲמָא",
            sbl_academic: "ṭaʿămā",
            english_name: "Taama",
        };

        assert_eq!(name.hebrew_name, "טַעֲמָא");
        assert_eq!(name.sbl_academic, "ṭaʿămā");
        assert_eq!(name.english_name, "Taama");
    }

    #[test]
    fn accent_name_fields_are_static_str() {
        let name = AccentName {
            hebrew_name: "דְּגֵשָׁה",
            sbl_academic: "dᵊgēšāh",
            english_name: "Dagesh",
        };

        // Ensure &'static str fields can be used in const contexts
        assert_eq!(name.hebrew_name, "דְּגֵשָׁה");
    }

    // ── AccentName Trait Tests ───────────────────────────────────────

    #[test]
    fn accent_name_eq_reflexive() {
        let name = AccentName {
            hebrew_name: "פִּסִּיק",
            sbl_academic: "pissiq",
            english_name: "Paseq",
        };
        assert_eq!(name, name);
    }

    #[test]
    fn accent_name_eq_symmetric() {
        let a = AccentName {
            hebrew_name: "מַקֵּף",
            sbl_academic: "maqqep̄",
            english_name: "Maqqeph",
        };
        let b = a.clone();
        assert_eq!(a, b);
        assert_eq!(b, a);
    }

    #[test]
    fn accent_name_eq_when_all_fields_match() {
        let a = AccentName {
            hebrew_name: "סוֹף פָּסוּק",
            sbl_academic: "sop̄ pāsûq",
            english_name: "Soph Pasuq",
        };
        let b = AccentName {
            hebrew_name: "סוֹף פָּסוּק",
            sbl_academic: "sop̄ pāsûq",
            english_name: "Soph Pasuq",
        };
        assert_eq!(a, b);
    }

    #[test]
    fn accent_name_ne_when_hebrew_differs() {
        let a = AccentName {
            hebrew_name: "אֶתְנַחְתָּא",
            sbl_academic: "ʾetnaḥtā",
            english_name: "Atnach",
        };
        let b = AccentName {
            hebrew_name: "זָקֵף",
            sbl_academic: "ʾetnaḥtā",
            english_name: "Atnach",
        };
        assert_ne!(a, b);
    }

    #[test]
    fn accent_name_ne_when_sbl_differs() {
        let a = AccentName {
            hebrew_name: "טֶרֶשׁ",
            sbl_academic: "ṭereš",
            english_name: "Teres",
        };
        let b = AccentName {
            hebrew_name: "טֶרֶשׁ",
            sbl_academic: "ṭeres",
            english_name: "Teres",
        };
        assert_ne!(a, b);
    }

    #[test]
    fn accent_name_ne_when_english_differs() {
        let a = AccentName {
            hebrew_name: "טֶרֶשׁ",
            sbl_academic: "ṭereš",
            english_name: "Teres",
        };
        let b = AccentName {
            hebrew_name: "טֶרֶשׁ",
            sbl_academic: "ṭereš",
            english_name: "Teres Variant",
        };
        assert_ne!(a, b);
    }

    #[test]
    fn accent_name_copy_preserves_original() {
        let original = AccentName {
            hebrew_name: "שַׁלְשֶׁלֶת",
            sbl_academic: "šalšelet",
            english_name: "Shalshelet",
        };
        let copied = original;

        assert_eq!(original.hebrew_name, "שַׁלְשֶׁלֶת");
        assert_eq!(copied.hebrew_name, "שַׁלְשֶׁלֶת");
        assert_eq!(original, copied);
    }

    #[test]
    fn accent_name_clone_produces_equal() {
        let original = AccentName {
            hebrew_name: "מֻנַּח",
            sbl_academic: "munnaḥ",
            english_name: "Munach",
        };
        let cloned = original.clone();

        assert_eq!(cloned, original);
    }

    #[test]
    fn accent_name_debug_output_contains_fields() {
        let name = AccentName {
            hebrew_name: "זִקְרָא",
            sbl_academic: "ziqrā",
            english_name: "Zikra",
        };

        let debug = format!("{:?}", name);
        assert!(debug.contains("זִקְרָא"));
        assert!(debug.contains("ziqrā"));
        assert!(debug.contains("Zikra"));
        assert!(debug.contains("AccentName"));
    }

    #[test]
    fn accent_name_hashable() {
        use std::collections::HashSet;

        let name1 = AccentName {
            hebrew_name: "רְבִיעַ",
            sbl_academic: "rᵊbîaʿ",
            english_name: "Revia",
        };
        let name2 = AccentName {
            hebrew_name: "רְבִיעַ",
            sbl_academic: "rᵊbîaʿ",
            english_name: "Revia",
        };
        let name3 = AccentName {
            hebrew_name: "פָּסְתָּא",
            sbl_academic: "pāstā",
            english_name: "Pashta",
        };

        let mut set = HashSet::new();
        set.insert(name1);
        set.insert(name2);
        set.insert(name3);

        assert_eq!(set.len(), 2, "Equal names should deduplicate")
    }
}
