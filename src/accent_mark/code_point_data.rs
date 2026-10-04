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
