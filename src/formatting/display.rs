//! For now only used for debugging Hebrew accent information  

// Standard library

// External crates

// Crate‑internal (local modules)
// use crate::accent_data::{POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE};
// use crate::AccentInformation;

// Print every accent in the *prose* table.
// pub(crate) fn display_prose_accent_table() {
//     println!("=== PROSE ACCENT TABLE ===");
//     for (idx, info) in PROSE_ACCENT_TABLE.iter().enumerate() {
//         // `info` is a `&'static AccentInformation`
//         println!("{:02}. {:#?}", idx + 1, info);
//     }
//     println!("=== END OF PROSE TABLE ===\n");
// }

// Print every accent in the *poetry* table.
// pub(crate) fn display_poetry_accent_table() {
//     println!("=== POETRY ACCENT TABLE ===");
//     for (idx, info) in POETRY_ACCENT_TABLE.iter().enumerate() {
//         println!("{:02}.  {:#?}", idx + 1, info);
//     }
//     println!("=== END OF POETRY TABLE ===\n");
// }

// Print every accent in the *pseudo* table.
// pub(crate) fn display_pseudo_accent_table() {
//     println!("=== PSEUDO ACCENT TABLE ===");
//     for (idx, info) in PSEUDO_ACCENT_TABLE.iter().enumerate() {
//         println!("{:02}.  {:#?}", idx + 1, info);
//     }
//     println!("=== END OF PSEUDO TABLE ===\n");
// }
//
// Generic printer (wrapper)
// pub(crate) fn display_accent_table(name: &str, table: &[&AccentInformation]) {
//     println!("=== {name} ===");
//     for (idx, info) in table.iter().enumerate() {
//         println!("{:02}.  {:#?}", idx + 1, info);
//     }
//     println!("=== END OF {name} ===\n");
// }
