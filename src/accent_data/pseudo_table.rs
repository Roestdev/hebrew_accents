use crate::{
    accent::AccentMetaData,
    accent_data::accent_information::{MAQQEPH_INFO, PASEQ_INFO, SOPH_PASUQ_INFO},
    PseudoAccent,
};

/// Static lookup table mapping `PseudoAccent` discriminants to metadata records.
///
/// **Index Mapping:**
/// | Index | Variant   | Entry             |
/// |-------|-----------|-------------------|
/// | 0 | [`SophPasuq`](crate::PseudoAccent::SophPasuq) | `SOPH_PASUQ_INFO` |
/// | 1 | [`Maqqeph`](crate::PseudoAccent::Maqqeph)   | `MAQQEPH_INFO`    |
/// | 2 | [`Paseq`](crate::PseudoAccent::Paseq)     | `PASEQ_INFO`      |
///
/// **Safety Requirements:**
/// - `PseudoAccent::LEN` must always equal table length (validated via compile-time guard)
/// - Entries must correspond exactly to enum declaration order
/// - All name fields populated (no "todo" placeholders)
///
pub(crate) static PSEUDO_ACCENT_TABLE: [AccentMetaData; PseudoAccent::LEN] =
    [SOPH_PASUQ_INFO, MAQQEPH_INFO, PASEQ_INFO];

// ── Compile-time guards ────────────────────────────────────────────────

/// Verifies that table length matches enum count.
const _: () = assert!(
    PSEUDO_ACCENT_TABLE.len() == PseudoAccent::LEN,
    "PSEUDO_ACCENT_TABLE length mismatch"
);

#[test]
fn table_length_matches_enum_count() {
    assert_eq!(PSEUDO_ACCENT_TABLE.len(), PseudoAccent::LEN);
}
