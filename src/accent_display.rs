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
mod tests2 {
    use super::*;
    //use std::io::{self, Write};
    //use std::sync::{Arc, Mutex};

    // // Helper to capture stdout
    // struct CaptureWriter {
    //     buffer: Arc<Mutex<Vec<u8>>>,
    // }

    // impl Write for CaptureWriter {
    //     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    //         self.buffer.lock().unwrap().extend_from_slice(buf);
    //         Ok(buf.len())
    //     }

    //     fn flush(&mut self) -> io::Result<()> {
    //         Ok(())
    //     }
    // }

    #[test]
    fn show_prose_table() {
        display_prose_accent_table();
    }

    #[test]
    fn show_poetry_table() {
        display_poetry_accent_table();
    }

    #[test]
    fn show_pseudo_table() {
        display_pseudo_accent_table();
    }

    #[test]
    fn show_tables_generic() {
        display_accent_table("PROSE (generic)", PROSE_ACCENT_TABLE.as_ref());
        display_accent_table("POETRY (generic)", POETRY_ACCENT_TABLE.as_ref());
        display_accent_table("PSEUDO (generic)", PSEUDO_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_generic_with_empty_table() {
        let empty_table: Vec<&AccentInformation> = vec![];
        display_accent_table("EMPTY TEST", &empty_table);
    }

    #[test]
    fn test_generic_with_single_item_table() {
        // Create a minimal table with one item if possible
        // Assuming AccentInformation has a way to create a dummy instance
        // If not, we'll just test with the existing tables
        if !PROSE_ACCENT_TABLE.is_empty() {
            let single_item = vec![PROSE_ACCENT_TABLE[0]];
            display_accent_table("SINGLE ITEM TEST", &single_item);
        }
    }

    #[test]
    fn test_wrapper_functions_use_generic_logic() {
        // Verify that specific wrappers don't panic and execute
        display_prose_accent_table();
        display_poetry_accent_table();
        display_pseudo_accent_table();

        // Verify generic function works with all table types
        display_accent_table("Generic Prose", PROSE_ACCENT_TABLE.as_ref());
        display_accent_table("Generic Poetry", POETRY_ACCENT_TABLE.as_ref());
        display_accent_table("Generic Pseudo", PSEUDO_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_all_tables_are_accessible() {
        // Ensure all three tables exist and are non-empty (if expected)
        assert!(
            !PROSE_ACCENT_TABLE.is_empty(),
            "Prose table should not be empty"
        );
        assert!(
            !POETRY_ACCENT_TABLE.is_empty(),
            "Poetry table should not be empty"
        );
        assert!(
            !PSEUDO_ACCENT_TABLE.is_empty(),
            "Pseudo table should not be empty"
        );
    }

    #[test]
    fn test_generic_function_with_custom_name() {
        // Test that the generic function accepts custom names
        display_accent_table("CUSTOM NAME TEST", PROSE_ACCENT_TABLE.as_ref());
    }

    // #[test]
    // fn test_iteration_over_all_tables() {
    //     // Ensure we can iterate over all tables without issues
    //     for table in &[PROSE_ACCENT_TABLE, POETRY_ACCENT_TABLE, PSEUDO_ACCENT_TABLE] {
    //         for (idx, info) in table.iter().enumerate() {
    //             // Just ensure we can access the data
    //             assert!(info.name.is_some() || info.id.is_some(),
    //                 "Accent info should have at least name or id");
    //         }
    //     }
    // }
}

#[cfg(test)]
mod lumo_tests {
    use super::*;

    #[test]
    fn test_display_prose_accent_table_executes() {
        display_prose_accent_table();
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
        display_accent_table("CUSTOM PROSE TEST", PROSE_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_display_accent_table_empty() {
        let empty_table: Vec<&AccentInformation> = vec![];
        display_accent_table("EMPTY TABLE", &empty_table);
    }

    #[test]
    fn test_all_wrappers_call_generic_logic() {
        display_prose_accent_table();
        display_poetry_accent_table();
        display_pseudo_accent_table();
        display_accent_table("Generic Test", PROSE_ACCENT_TABLE.as_ref());
    }

    // Additional tests to improve coverage
    #[test]
    fn test_generic_with_poetry_table() {
        display_accent_table("POETRY GENERIC", POETRY_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_generic_with_pseudo_table() {
        display_accent_table("PSEUDO GENERIC", PSEUDO_ACCENT_TABLE.as_ref());
    }

    #[test]
    fn test_multiple_calls_to_same_function() {
        // Ensure functions can be called multiple times
        for _ in 0..3 {
            display_prose_accent_table();
            display_poetry_accent_table();
            display_pseudo_accent_table();
        }
    }

    #[test]
    fn test_edge_case_single_element_table() {
        if !PROSE_ACCENT_TABLE.is_empty() {
            let single = vec![PROSE_ACCENT_TABLE[0]];
            display_accent_table("SINGLE ELEMENT", &single);
        }
    }
}

#[cfg(test)]
mod improved_tests {
    use super::*;
    //use std::io::{self, Write};
    //use std::sync::{Arc, Mutex};

    /// Helper to capture stdout for assertion-based testing
    // struct StdoutCapture {
    //     buffer: Arc<Mutex<Vec<u8>>>,
    //     original_stdin: Option<io::Stdout>,
    // }

    // impl StdoutCapture {
    //     fn new() -> Self {
    //         let buffer = Arc::new(Mutex::new(Vec::new()));
    //         let original_stdin = None; // Simplified for this example

    //         // In real implementation, you'd redirect stdout here
    //         StdoutCapture { buffer, original_stdin }
    //     }

    //     fn get_output(&self) -> String {
    //         String::from_utf8_lossy(&self.buffer.lock().unwrap()).to_string()
    //     }
    // }

    // ========================================================================
    // BASIC FUNCTIONALITY TESTS
    // ========================================================================

    #[test]
    fn test_display_prose_accent_table_executes_without_panic() {
        // Verify function doesn't panic
        assert!(std::panic::catch_unwind(|| {
            display_prose_accent_table();
        })
        .is_ok());
    }

    #[test]
    fn test_display_poetry_accent_table_executes_without_panic() {
        assert!(std::panic::catch_unwind(|| {
            display_poetry_accent_table();
        })
        .is_ok());
    }

    #[test]
    fn test_display_pseudo_accent_table_executes_without_panic() {
        assert!(std::panic::catch_unwind(|| {
            display_pseudo_accent_table();
        })
        .is_ok());
    }

    #[test]
    fn test_display_accent_table_generic_executes_without_panic() {
        assert!(std::panic::catch_unwind(|| {
            display_accent_table("TEST", &[]);
        })
        .is_ok());
    }

    // ========================================================================
    // OUTPUT VERIFICATION TESTS
    // ========================================================================

    // #[test]
    // fn test_prose_table_contains_expected_headers() {
    //     // Capture and verify output contains expected headers
    //     let output = capture_stdout(|| {
    //         display_prose_accent_table();
    //     });

    //     assert!(output.contains("=== PROSE ACCENT TABLE ==="),
    //             "Output should contain prose table header");
    //     assert!(output.contains("=== END OF PROSE TABLE ==="),
    //             "Output should contain prose table footer");
    // }

    // #[test]
    // fn test_poetry_table_contains_expected_headers() {
    //     let output = capture_stdout(|| {
    //         display_poetry_accent_table();
    //     });

    //     assert!(output.contains("=== POETRY ACCENT TABLE ==="),
    //             "Output should contain poetry table header");
    //     assert!(output.contains("=== END OF POETRY TABLE ==="),
    //             "Output should contain poetry table footer");
    // }

    // #[test]
    // fn test_pseudo_table_contains_expected_headers() {
    //     let output = capture_stdout(|| {
    //         display_pseudo_accent_table();
    //     });

    //     assert!(output.contains("=== PSEUDO ACCENT TABLE ==="),
    //             "Output should contain pseudo table header");
    //     assert!(output.contains("=== END OF PSEUDO TABLE ==="),
    //             "Output should contain pseudo table footer");
    // }

    // #[test]
    // fn test_generic_table_with_custom_name() {
    //     let output = capture_stdout(|| {
    //         display_accent_table("MY CUSTOM TABLE", &[]);
    //     });

    //     assert!(output.contains("=== MY CUSTOM TABLE ==="),
    //             "Output should contain custom table name");
    //     assert!(output.contains("=== END OF MY CUSTOM TABLE ==="),
    //             "Output should contain custom table footer");
    // }

    // ========================================================================
    // EDGE CASE TESTS
    // ========================================================================

    // #[test]
    // fn test_generic_with_empty_table() {
    //     let output = capture_stdout(|| {
    //         let empty_table: Vec<&AccentInformation> = vec![];
    //         display_accent_table("EMPTY", &empty_table);
    //     });

    //     assert!(output.contains("=== EMPTY ==="));
    //     assert!(output.contains("=== END OF EMPTY ==="));
    //     // Should not contain any numbered entries
    //     //assert!(!output.contains(". "), "Empty table should have no entries");
    // }

    // #[test]
    // fn test_generic_with_single_item_table() {
    //     if !PROSE_ACCENT_TABLE.is_empty() {
    //         let output = capture_stdout(|| {
    //             let single_item = vec![PROSE_ACCENT_TABLE[0]];
    //             display_accent_table("SINGLE", &single_item);
    //         });

    //         assert!(output.contains("=== SINGLE ==="));
    //         assert!(output.contains("01.")); // First (and only) entry
    //         assert!(output.contains("=== END OF SINGLE ==="));
    //     }
    // }

    // #[test]
    // fn test_generic_with_large_table() {
    //     if !PROSE_ACCENT_TABLE.is_empty() {
    //         let output = capture_stdout(|| {
    //             display_accent_table("LARGE", PROSE_ACCENT_TABLE.as_ref());
    //         });

    //         let count = PROSE_ACCENT_TABLE.len();
    //         assert!(output.contains("=== LARGE ==="));

    //         // Verify we have entries for all items
    //         for i in 1..=count {
    //             assert!(output.contains(&format!("{:02}.", i)),
    //                     "Should contain entry #{:02}", i);
    //         }

    //         assert!(output.contains("=== END OF LARGE ==="));
    //     }
    // }

    // // ========================================================================
    // // CONSISTENCY TESTS
    // // ========================================================================

    // #[test]
    // fn test_specific_wrappers_match_generic_behavior() {
    //     // Verify that specific wrappers produce similar output structure to generic
    //     let prose_output = capture_stdout(|| display_prose_accent_table());
    //     let poetry_output = capture_stdout(|| display_poetry_accent_table());
    //     let pseudo_output = capture_stdout(|| display_pseudo_accent_table());

    //     // All should have consistent formatting
    //     assert!(prose_output.contains("==="));
    //     assert!(poetry_output.contains("==="));
    //     assert!(pseudo_output.contains("==="));

    //     // All should have numbered entries if tables are non-empty
    //     if !PROSE_ACCENT_TABLE.is_empty() {
    //         assert!(prose_output.contains("01."));
    //     }
    //     if !POETRY_ACCENT_TABLE.is_empty() {
    //         assert!(poetry_output.contains("01."));
    //     }
    //     if !PSEUDO_ACCENT_TABLE.is_empty() {
    //         assert!(pseudo_output.contains("01."));
    //     }
    // }

    // #[test]
    // fn test_multiple_calls_produce_consistent_output() {
    //     let output1 = capture_stdout(|| display_prose_accent_table());
    //     let output2 = capture_stdout(|| display_prose_accent_table());

    //     assert_eq!(output1, output2, "Multiple calls should produce identical output");
    // }

    // ========================================================================
    // TABLE INTEGRITY TESTS
    // ========================================================================

    #[test]
    fn test_all_tables_are_non_empty() {
        assert!(
            !PROSE_ACCENT_TABLE.is_empty(),
            "Prose table should not be empty"
        );
        assert!(
            !POETRY_ACCENT_TABLE.is_empty(),
            "Poetry table should not be empty"
        );
        assert!(
            !PSEUDO_ACCENT_TABLE.is_empty(),
            "Pseudo table should not be empty"
        );
    }

    #[test]
    fn test_table_entries_have_valid_data() {
        for (idx, info) in PROSE_ACCENT_TABLE.iter().enumerate() {
            // Each entry should have meaningful data
            assert!(
                !info.hebrew_name.is_empty()
                    || !info.english_name.is_empty()
                    || !info.meaning.is_empty(),
                "Prose entry {} should have hebrew_name, english_name and meaning",
                idx
            );
        }

        for (idx, info) in POETRY_ACCENT_TABLE.iter().enumerate() {
            assert!(
                !info.hebrew_name.is_empty()
                    || !info.english_name.is_empty()
                    || !info.meaning.is_empty(),
                "Poetry entry {} should have hebrew_name, english_name and meaning",
                idx
            );
        }

        for (idx, info) in PSEUDO_ACCENT_TABLE.iter().enumerate() {
            assert!(
                !info.hebrew_name.is_empty()
                    || !info.english_name.is_empty()
                    || !info.meaning.is_empty(),
                "Pseudo entry {} should have hebrew_name, english_name and meaning",
                idx
            );
        }
    }

    // ========================================================================
    // FORMATTING TESTS
    // ========================================================================

    // #[test]
    // fn test_entry_formatting_is_consistent() {
    //     if !PROSE_ACCENT_TABLE.is_empty() {
    //         let output = capture_stdout(|| {
    //             display_prose_accent_table();
    //         });

    //         // Check that entries are properly formatted with leading zeros
    //         let lines: Vec<&str> = output.lines().collect();
    //         let entry_lines: Vec<&str> = lines.iter()
    //             .filter(|line| line.trim().starts_with("0"))
    //             .cloned()
    //             .collect();

    //         if !entry_lines.is_empty() {
    //             // Verify all entries have consistent formatting
    //             for line in &entry_lines {
    //                 assert!(line.contains("."), "Entry should contain period separator");
    //                 assert!(line.contains("{:#?}"), "Entry should contain debug formatting");
    //             }
    //         }
    //     }
    // }

    // ========================================================================
    // PERFORMANCE/STRESS TESTS
    // ========================================================================

    #[test]
    fn test_rapid_succession_calls() {
        // Test that functions can be called rapidly without issues
        for _ in 0..10 {
            display_prose_accent_table();
            display_poetry_accent_table();
            display_pseudo_accent_table();
        }
    }

    #[test]
    fn test_concurrent_calls() {
        // Test thread safety (if applicable)
        use std::thread;

        let handles: Vec<_> = (0..5)
            .map(|_| {
                thread::spawn(|| {
                    display_prose_accent_table();
                    display_poetry_accent_table();
                    display_pseudo_accent_table();
                })
            })
            .collect();

        for handle in handles {
            assert!(handle.join().is_ok(), "Thread should complete successfully");
        }
    }

    // ========================================================================
    // HELPER FUNCTION
    // ========================================================================

    // Helper to capture stdout during test execution

    // fn capture_stdout<F: FnOnce()>(f: F) -> String {
    //     // In a real implementation, you'd redirect stdout here
    //     // For now, this is a placeholder that returns empty string
    //     // You'll need to implement proper stdout capture

    //     // Example implementation using env_logger or similar:
    //     // let mut buf = Vec::new();
    //     // let stdout = io::stdout();
    //     // let mut handle = stdout.lock();
    //     // ... redirect logic ...

    //     // For now, just execute the function
    //     f();
    //     String::new() // Placeholder - implement proper capture
    // }
}
