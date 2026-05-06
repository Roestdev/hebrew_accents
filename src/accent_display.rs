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

#[cfg(test)]
mod lumo_tests {
    use super::*;

    #[test]
    fn test_display_prose_accent_table_executes() {
        // This ensures the function runs and iterates over the table
        // If the table is empty, it should still print headers.
        display_prose_accent_table();
        // If we got here without panic, the function executed successfully.
    }

    #[test]
    fn test_display_poetry_accent_table_executes() {
        display_poetry_accent_table();
    }

    #[test]
    fn test_display_pseudo_accent_table_executes() {
        display_pseudo_accent_table();
    }

    #[test]
    fn test_display_accent_table_generic() {
        // Test the generic function with the prose table
        display_accent_table("CUSTOM PROSE TEST", PROSE_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_display_accent_table_empty() {
        // Test edge case: Empty table
        let empty_table: Vec<&AccentInformation> = vec![];
        display_accent_table("EMPTY TABLE", &empty_table);
        // Should print headers and "END OF EMPTY TABLE" without looping
    }

    #[test]
    fn test_all_wrappers_call_generic_logic() {
        // Verify that the specific wrappers effectively behave like the generic one
        // by checking that they don't panic and produce output (conceptually).
        // Since we can't easily capture stdout in a standard unit test without dependencies,
        // we rely on the fact that they iterate.

        // Run all to ensure 100% function execution
        display_prose_accent_table();
        display_poetry_accent_table();
        display_pseudo_accent_table();

        // Run generic
        display_accent_table("Generic Test", PROSE_ACCENT_TABLE.as_ref());
    }
}
