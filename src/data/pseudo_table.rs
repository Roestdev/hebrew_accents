use crate::{
    accent::AccentInformation,
    data::accent_information::{MAQQEPH_INFO, PASEQ_INFO, SOPH_PASUQ_INFO},
    PseudoAccent,
};

/// Static lookup table mapping `PseudoAccent` discriminants to metadata records.
///
/// **Index Mapping:**
/// | Index | Variant   | Entry             |
/// |-------|-----------|-------------------|
/// | 0 | [`SophPasuq`] | `SOPH_PASUQ_INFO` |
/// | 1 | [`Maqqeph`]   | `MAQQEPH_INFO`    |
/// | 2 | [`Paseq`]     | `PASEQ_INFO`      |
///
/// **Safety Requirements:**
/// - `PseudoAccent::LEN` must always equal table length (validated via tests)
/// - Entries must correspond exactly to enum declaration order
/// - All name fields populated (no "todo" placeholders)
///
pub(crate) static PSEUDO_ACCENT_TABLE: [AccentInformation; PseudoAccent::LEN] =
    [SOPH_PASUQ_INFO, MAQQEPH_INFO, PASEQ_INFO];

const _: () = assert!(
    PSEUDO_ACCENT_TABLE.len() == PseudoAccent::LEN,
    "Table length mismatch for PSEUDO_ACCENT_TABLE!"
);

// This panics if lengths don't match
#[test]
fn _table_length_check() {
    assert_eq!(PSEUDO_ACCENT_TABLE.len(), PseudoAccent::LEN);
}
