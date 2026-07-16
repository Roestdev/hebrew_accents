use crate::accent::AccentInformation;
use crate::data::accent_information::{
    ATNACH_INFO,
    AZLA_INFO,
    DARGA_INFO,
    GALGAL_INFO,
    GERESH_INFO,
    GERSHAYIM_INFO,
    LEGARMEH_INFO,
    MAHPAKH_INFO,
    MAYELA_INFO,
    MERKHA_INFO,
    MERKHA_KEFULAH_INFO,
    METEG_INFO,
    // Conjunctives
    MUNACH_INFO,
    PASHTA_INFO,
    PAZER_GADOL_INFO,
    PAZER_INFO,
    REVIA_INFO,
    SEGOLTA_INFO,
    SHALSHELET_INFO,
    // Disjunctives
    SILLUQ_INFO,
    TELISHA_GEDOLAH_INFO,
    TELISHA_QETANNAH_INFO,
    TEVIR_INFO,
    TIPHCHA_INFO,
    YETIV_INFO,
    ZAQEF_QATON_INFO,
    ZAQEPH_GADOL_INFO,
    ZARQA_INFO,
};
use crate::ProseAccent;

/// Static lookup table for prose cantillation metadata.
///
/// Maps `ProseAccent` variants (indices 0–27) to [`AccentInformation`] records.
/// Used by the [`Accent`](crate::accent::Accent) trait for information retrieval.
///
/// ## Structure
///
/// ### Disjunctive Accents (Indices 0–17)
/// Create clause/phrase separations in prosaic text. Strength calculated via
/// discriminant arithmetic (`self as u8 + 1`).
///
/// ### Conjunctive Accents (Indices 18–26)  
/// Connect words within phrases. Include secondary marks like Meteg/Mayela.
///
/// ## Safety Requirements
/// - Order MUST match `ProseAccent` discriminant values exactly
/// - Length equals `ProseAccent::LEN` (validated via tests)
/// - All name fields populated (no "todo" placeholders)
///
/// ## Sources
/// - Biblia Hebraica Stuttgartensia (Kittel 4th ed.)
/// - Waltke & O'Connor, *Introduction to Biblical Hebrew Syntax* (§§10.2–10.4)
pub(crate) static PROSE_ACCENT_TABLE: [AccentInformation; ProseAccent::LEN] = [
    // --------------------Disjunctives
    SILLUQ_INFO,
    ATNACH_INFO,
    SEGOLTA_INFO,
    SHALSHELET_INFO,
    ZAQEF_QATON_INFO,
    ZAQEPH_GADOL_INFO,
    REVIA_INFO,
    TIPHCHA_INFO,
    ZARQA_INFO,
    PASHTA_INFO,
    YETIV_INFO,
    TEVIR_INFO,
    GERESH_INFO,
    GERSHAYIM_INFO,
    PAZER_INFO,
    PAZER_GADOL_INFO,
    TELISHA_GEDOLAH_INFO,
    LEGARMEH_INFO,
    // ----------------------Conjunctives
    MUNACH_INFO,
    MAHPAKH_INFO,
    MERKHA_INFO,
    MERKHA_KEFULAH_INFO,
    DARGA_INFO,
    AZLA_INFO,
    TELISHA_QETANNAH_INFO,
    GALGAL_INFO,
    MAYELA_INFO,
    METEG_INFO,
];

// Compile-time guard
const _: () = assert!(
    PROSE_ACCENT_TABLE.len() == ProseAccent::LEN,
    "PROSE_ACCENT_TABLE length mismatch!"
);

#[test]
fn table_length_matches_enum_count() {
    assert_eq!(PROSE_ACCENT_TABLE.len(), ProseAccent::LEN);
}
