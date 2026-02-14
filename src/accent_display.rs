//! For now only used for debugging Hebrew accent information  

// Standard library

// External crates

// Crate‑internal (local modules)
use crate::accent_data::{POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE};
use crate::AccentInformation;

/// Print every accent in the *prose* table.
pub fn display_prose_accent_table() {
    println!("=== PROSE ACCENT TABLE ===");
    for (idx, info) in PROSE_ACCENT_TABLE.iter().enumerate() {
        // `info` is a `&'static AccentInformation`
        println!("{:02}. {:#?}", idx + 1, info);
    }
    println!("=== END OF PROSE TABLE ===\n");
}

/// Print every accent in the *poetry* table.
pub fn display_poetry_accent_table() {
    println!("=== POETRY ACCENT TABLE ===");
    for (idx, info) in POETRY_ACCENT_TABLE.iter().enumerate() {
        println!("{:02}.  {:#?}", idx + 1, info);
    }
    println!("=== END OF POETRY TABLE ===\n");
}

/// Print every accent in the *pseudo* table.
pub fn display_pseudo_accent_table() {
    println!("=== PSEUDO ACCENT TABLE ===");
    for (idx, info) in PSEUDO_ACCENT_TABLE.iter().enumerate() {
        println!("{:02}.  {:#?}", idx + 1, info);
    }
    println!("=== END OF PSEUDO TABLE ===\n");
}

/// Generic printer (wrapper)
pub fn display_accent_table(name: &str, table: &[&AccentInformation]) {
    println!("=== {name} ===");
    for (idx, info) in table.iter().enumerate() {
        println!("{:02}.  {:#?}", idx + 1, info);
    }
    println!("=== END OF {name} ===\n");
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_prose_table() {
        // Direct call
        display_prose_accent_table();
    }

    #[test]
    fn show_poetry_table() {
        // Direct call
        display_poetry_accent_table();
    }

    #[test]
    fn show_pseudo_table() {
        // Direct call
        display_pseudo_accent_table();
    }

    #[test]
    // Or the generic version
    fn show_tables_generic() {
        display_accent_table("PROSE (generic)", PROSE_ACCENT_TABLE.as_ref());
        display_accent_table("POETRY (generic)", POETRY_ACCENT_TABLE.as_ref());
        display_accent_table("PSEUDO (generic)", PSEUDO_ACCENT_TABLE.as_ref());
    }
}
