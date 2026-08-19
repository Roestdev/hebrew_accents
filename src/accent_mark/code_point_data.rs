//! This file contains all static data of 'UTF-8 code point'
//!
//! Constants below are a mix of the following:
//! - UTF-8 code table (<https://utf8-chartable.de/unicode-utf8-table.pl>)
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
    canonical_name: &'static str,
    code_point_value: &'static str,
    hex_bytes: &'static str,
) -> Utf8CodePoint {
    Utf8CodePoint {
        symbol,
        position,
        stress_position,
        canonical_name,
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
// SEGOL (U+0592) - Only 3 traditions
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
// ZAQEF GADOL (U+0595) - All identical (use uniform!)
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
    CodePointPosition::AboveCenter,
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

// Geresh Muqdam is disabled - see ticket

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
// QARNEY PARA (U+059F) - All identical
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
// PAZER (U+05A1) - Only 3 traditions
// ============================================================================
pub(crate) const CODEPOINT_PAZER: Utf8CodePoint = utf8_cp_constructor(
    '֡',
    CodePointPosition::AboveCenter,
    StressPosition::Impositive,
    "HEBREW ACCENT PAZER",
    "U+05A1",
    "0xd6 0xa1",
);

// At nah Hafukh disabled - see ticket

// ============================================================================
// MUNAH (U+05A3) - Only 3 traditions
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
// MAHPAKH (U+05A4)
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
// MERKHA KEFULA (U+05A6) - Only 3 traditions
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
// DARGA (U+05A7) - All identical
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
// QADMA (U+05A8) - Only 3 traditions
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
// YERAH BEN YOMO (U+05AA) - All identical
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
// OLE (U+05AB) - All identical
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
// ILUY (U+05AC) - All identical
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
// DEHI (U+05AD) - All identical
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
// ZINOR (U+05AE) - All identical
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
// SILLUQ (U+05BD) - All identical (same codepoint as Meteg, different semantics)
// ============================================================================

pub(crate) const CODEPOINT_SILLUQ: Utf8CodePoint = utf8_cp_constructor(
    'ֽ',
    CodePointPosition::BelowCenter,
    StressPosition::Impositive,
    "HEBREW POINT SILLUQ",
    "U+05BD",
    "0xd6 0xbd",
);

// ============================================================================
// METEG (U+05BD) - All identical (shares codepoint with Silluq)
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
// MAQAF (U+05BE) - No traditions
// ============================================================================

pub(crate) const CODEPOINT_MAQAF: Utf8CodePoint = utf8_cp_constructor(
    '־',
    CodePointPosition::Maqqaf,
    StressPosition::NotApplicable,
    "HEBREW PUNCTUATION MAQAF",
    "U+05BE",
    "0xd6 0xbe",
);

// ============================================================================
// PASEQ (U+05C0) - All identical
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
// SOPH PASUQ (U+05C3) - No traditions
// ============================================================================

pub(crate) const CODEPOINT_SOPH_PASUQ: Utf8CodePoint = utf8_cp_constructor(
    '׃',
    CodePointPosition::SofPasuq,
    StressPosition::NotApplicable,
    "HEBREW PUNCTUATION SOF PASUQ",
    "U+05C3",
    "0xd7 0x83",
);
