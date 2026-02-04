// Standard library

// External crates

// Crate‑internal (local modules)
use crate::accent_data::{POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE};
use crate::AccentInfo;

/// Re‑export the tables so the helper functions can see them without pulling in the whole
/// `crate::accent` hierarchy again.  Adjust the path if the tables live in a different module.
///
/// Print every accent in the *prose* table.
///
/// The function simply iterates over the static slice and writes each `AccentInfo` to stdout.
/// It uses the `Display` implementation you already added for `AccentInfo`; if you prefer a
/// raw `Debug` dump you can replace `println!("{}", info);` with `println!("{:#?}", info);`.
pub fn display_prose_accent_table() {
    println!("=== PROSE ACCENT TABLE ===");
    for (idx, info) in PROSE_ACCENT_TABLE.iter().enumerate() {
        // `info` is a `&'static AccentInfo`
        println!("{:02}. {:#?}", idx + 1, info);
    }
    println!("=== END OF PROSE TABLE ===\n");
}

/// Print every accent in the *poetry* table.
///
/// Works exactly like `display_prose_accent_table` but walks `POETRY_ACCENT_TABLE`.
pub fn display_poetry_accent_table() {
    println!("=== POETRY ACCENT TABLE ===");
    for (idx, info) in POETRY_ACCENT_TABLE.iter().enumerate() {
        println!("{:02}.  {:#?}", idx + 1, info);
    }
    println!("=== END OF POETRY TABLE ===\n");
}

/// Print every accent in the *pseudo* table.
///
/// Works exactly like `display_pseudo_accent_table` but walks `PSEUDO_ACCENT_TABLE
///
///
///
/// `.
pub fn display_pseudo_accent_table() {
    println!("=== PSEUDO ACCENT TABLE ===");
    for (idx, info) in PSEUDO_ACCENT_TABLE.iter().enumerate() {
        println!("{:02}.  {:#?}", idx + 1, info);
    }
    println!("=== END OF PSEUDO TABLE ===\n");
}

/// Generic printer – handy if you ever add more tables.
pub fn display_accent_table(name: &str, table: &[&AccentInfo]) {
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
