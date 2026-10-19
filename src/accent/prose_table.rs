use crate::accent::accent_information::{
    ATNACH_META_DATA, AZLA_META_DATA, DARGA_META_DATA, GALGAL_META_DATA, GERESH_META_DATA,
    GERSHAYIM_META_DATA, LEGARMEH_META_DATA, MAHPAKH_META_DATA, MEAYLA_META_DATA,
    MERKHA_KEPHULAH_META_DATA, MERKHA_META_DATA, METEG_META_DATA, MUNACH_META_DATA,
    PASHTA_META_DATA, PAZER_GADOL_META_DATA, PAZER_META_DATA, REVIA_META_DATA, SEGOLTA_META_DATA,
    SHALSHELET_META_DATA, SILLUQ_META_DATA, TELISHA_GEDOLAH_META_DATA, TELISHA_QETANNAH_META_DATA,
    TEVIR_META_DATA, TIPHCHA_META_DATA, YETIV_META_DATA, ZAQEPH_GADOL_META_DATA,
    ZAQEPH_QATAN_META_DATA, ZARQA_META_DATA,
};
use crate::accent::AccentMetaData;
use crate::ProseAccent;

/// Static lookup table for prose cantillation metadata.
///
/// Maps `ProseAccent` variants (indices 0–27) to [`AccentMetaData`] records.
/// Used by the [`Accent`](crate::accent::Accent) trait for information retrieval.
///
/// ## Structure
///
/// ### Disjunctive Accents (Indices 0–17)
/// Create clause/phrase separations in prosaic text. Strength calculated via
/// discriminant arithmetic (`self as u8 + 1`).
///
/// ### Conjunctive Accents (Indices 18–26)  
/// Connect words within phrases. Include secondary marks like Meteg/Meayla.
///
/// ## Safety Requirements
/// - Order MUST match `ProseAccent` discriminant values exactly
/// - Length equals `ProseAccent::LEN` (validated via tests)
/// - All name fields populated
///
/// ## Sources
/// - Biblia Hebraica Stuttgartensia (Kittel 4th ed.)
/// - Waltke & O'Connor, *Introduction to Biblical Hebrew Syntax* (§§10.2–10.4)
pub(crate) static PROSE_ACCENT_TABLE: [AccentMetaData; ProseAccent::LEN] = [
    // --------------------Disjunctives
    SILLUQ_META_DATA,
    ATNACH_META_DATA,
    SEGOLTA_META_DATA,
    SHALSHELET_META_DATA,
    ZAQEPH_QATAN_META_DATA,
    ZAQEPH_GADOL_META_DATA,
    REVIA_META_DATA,
    TIPHCHA_META_DATA,
    ZARQA_META_DATA,
    PASHTA_META_DATA,
    YETIV_META_DATA,
    TEVIR_META_DATA,
    GERESH_META_DATA,
    GERSHAYIM_META_DATA,
    PAZER_META_DATA,
    PAZER_GADOL_META_DATA,
    TELISHA_GEDOLAH_META_DATA,
    LEGARMEH_META_DATA,
    // ----------------------Conjunctives
    MUNACH_META_DATA,
    MAHPAKH_META_DATA,
    MERKHA_META_DATA,
    MERKHA_KEPHULAH_META_DATA,
    DARGA_META_DATA,
    AZLA_META_DATA,
    TELISHA_QETANNAH_META_DATA,
    GALGAL_META_DATA,
    MEAYLA_META_DATA,
    METEG_META_DATA,
];

// Compile-time guard
// Verifies that table length matches enum count.
const _: () = assert!(
    PROSE_ACCENT_TABLE.len() == ProseAccent::LEN,
    "PROSE_ACCENT_TABLE length mismatch!"
);

#[test]
fn table_length_matches_enum_count() {
    assert_eq!(PROSE_ACCENT_TABLE.len(), ProseAccent::LEN);
}
