//! # Unicode Version Information
//!
//! This module exposes the Unicode Standard version used for all cantillation marks
//! in the library, enabling users to verify compatibility with their systems.
//!
//! ## Quick Start
//!
//! ```rust
//! use hebrew_accents::unicode_version;
//!
//! let version = unicode_version();
//! println!("Unicode version: {}", version);  // "18.0"
//! ```
//!
//! ## Full Metadata Access
//!
//! ```rust
//! use hebrew_accents::unicode_version_info;
//!
//! let info = unicode_version_info();
//! assert_eq!(info.version, "18.0");
//! assert_eq!(info.block_name, "Hebrew");
//! assert_eq!(info.block_range, "U+0590–U+05FF");
//! ```
//!
//! ## Version Comparison
//!
//! ```rust
//! use hebrew_accents::UNICODE_VERSION;
//!
//! let current = UNICODE_VERSION;
//! let minimum_required = "15.1";
//!
//! // Parse versions for comparison (manual parsing or use semver crate)
//! let current_parts: Vec<&str> = current.split('.').collect();
//! let min_parts: Vec<&str> = minimum_required.split('.').collect();
//!
//! // Current version meets minimum requirement
//! assert!(current_parts[0].parse::<u8>().unwrap() > min_parts[0].parse::<u8>().unwrap());
//! ```

/// Unicode version metadata for all cantillation marks in this library.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnicodeVersionInfo {
    /// Semantic version
    pub version: &'static str,

    /// Release year-month (ISO format)
    pub release_date: &'static str,

    /// Unicode block name
    pub block_name: &'static str,

    /// Unicode block range
    pub block_range: &'static str,

    /// Script designation
    pub script: &'static str,
}

/// Unicode Standard version for all cantillation mark codepoints in this library.
///
/// All UTF-8 code points are validated against Unicode 18.0. Characters included
/// in this version ensure consistent rendering across modern systems.
///
/// # Example
///
/// ```rust
/// use hebrew_accents::UNICODE_VERSION;
///
/// // Both constant and function should return same value
///  assert_eq!(UNICODE_VERSION,"18.0");
/// ```
pub const UNICODE_VERSION: &str = "18.0";

/// Detailed Unicode reference information
///
/// # Example
///
/// ```rust
/// use hebrew_accents::UNICODE_REFERENCE;
///
/// // Both constant and function should return same value
///  assert_eq!(UNICODE_REFERENCE.block_name, "Hebrew");
///  assert_eq!(UNICODE_REFERENCE.version, "18.0");
/// ```
pub const UNICODE_REFERENCE: UnicodeVersionInfo = UnicodeVersionInfo {
    version: UNICODE_VERSION,
    release_date: "2026-09-16",
    block_name: "Hebrew",
    block_range: "U+0590–U+05FF",
    script: "Hebrew",
};

/// Returns the Unicode version for all cantillation marks in this library
///
/// # Example
///
/// ```rust
/// use hebrew_accents::{unicode_version,UNICODE_VERSION};
///
/// // Both constant and function should return same value
///  assert_eq!(UNICODE_VERSION,unicode_version());
/// ```
pub const fn unicode_version() -> &'static str {
    UNICODE_VERSION
}

/// Returns detailed Unicode metadata
/// ```rust
/// use hebrew_accents::unicode_version_info;
///
/// let info = unicode_version_info();
///
/// // Validate block range pattern
/// assert!(info.block_range.starts_with("U+"));
/// assert!(info.block_range.contains("–") || info.block_range.contains("-"));
/// ```
pub const fn unicode_version_info() -> &'static UnicodeVersionInfo {
    &UNICODE_REFERENCE
}

impl std::fmt::Display for UnicodeVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unicode {} ({})", self.version, self.block_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_version_constant() {
        // Core constant exists and has expected format
        assert_eq!(UNICODE_VERSION, "18.0");
        assert!(UNICODE_VERSION.matches('.').count() >= 1); // Semver-like
    }

    #[test]
    fn test_unicode_version_function() {
        assert_eq!(unicode_version(), "18.0");
        assert!(unicode_version().matches('.').count() >= 1); // Semver-like
    }

    #[test]
    fn test_unicode_reference_struct() {
        // All required fields are populated
        let info = UNICODE_REFERENCE;

        assert_eq!(info.version, "18.0");
        assert!(!info.release_date.is_empty());
        assert_eq!(info.block_name, "Hebrew");
        assert_eq!(info.script, "Hebrew");
        assert!(info.block_range.starts_with("U+"));
    }

    #[test]
    fn test_unicode_version_info_function() {
        // Function returns reference to constant
        let info = unicode_version_info();

        assert_eq!(info.version, UNICODE_VERSION);
        assert_eq!(info, &UNICODE_REFERENCE);
    }

    #[test]
    fn test_unicode_version_display() {
        // Display formatting works correctly
        let display = format!("{}", UNICODE_REFERENCE);

        assert!(display.contains("Unicode"));
        assert!(display.contains("18.0"));
        assert!(display.contains("Hebrew"));
    }

    #[test]
    fn test_version_semantic_consistency() {
        // Version major.minor format validation
        let parts: Vec<&str> = UNICODE_VERSION.split('.').collect();

        assert_eq!(parts.len(), 2, "Expected major.minor format");
        assert!(
            parts[0].parse::<u8>().is_ok(),
            "Major version should be numeric"
        );
        assert!(
            parts[1].parse::<u8>().is_ok(),
            "Minor version should be numeric"
        );
    }
    #[test]
    fn test_unicode_version_display_detail() {
        // More granular test for Display implementation
        let info = UNICODE_REFERENCE;
        let formatted = format!("{}", info);

        // Verify exact format structure
        assert_eq!(formatted, "Unicode 18.0 (Hebrew)");
        assert!(!formatted.contains('\n'));
        assert_eq!(formatted.len(), 21);
    }

    #[test]
    fn test_unicode_version_debug_trait() {
        // Test Debug trait implementation (derived)
        let info = UNICODE_REFERENCE;
        let debug_output = format!("{:?}", info);

        assert!(debug_output.contains("UnicodeVersionInfo"));
        assert!(debug_output.contains("version"));
        assert!(debug_output.contains("18.0"));
    }

    #[test]
    fn test_unicode_version_clone_trait() {
        // Test Clone trait implementation (derived)
        let original = UNICODE_REFERENCE;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert!(std::ptr::addr_of!(original.version) != std::ptr::addr_of!(cloned.version));
        // Both point to same &'static str data, not heap allocation
    }

    #[test]
    fn test_unicode_version_partial_eq_trait() {
        // Test PartialEq trait implementation (derived)
        let info1 = UNICODE_REFERENCE;
        let info2 = UNICODE_REFERENCE;

        assert_eq!(info1, info2);
    }

    #[test]
    fn test_unicode_version_eq_trait() {
        // Test Eq trait implementation (derived)
        let info1 = UNICODE_REFERENCE;
        let info2 = UNICODE_REFERENCE;

        assert_eq!(info1, info2);
        assert!(info1.eq(&info2));
    }

    #[test]
    fn test_unicode_reference_const_eval() {
        // Ensure constants are compile-time evaluable
        const VERSION: &str = unicode_version();
        const INFO: &UnicodeVersionInfo = unicode_version_info();

        assert_eq!(VERSION, "18.0");
        assert_eq!(INFO.version, "18.0");
        assert_eq!(INFO.block_name, "Hebrew");
    }

    #[test]
    fn test_release_date_iso_format() {
        // Validate ISO date format (YYYY-MM-DD)
        let info = UNICODE_REFERENCE;

        // Should match YYYY-MM-DD pattern
        let date_parts: Vec<&str> = info.release_date.split('-').collect();
        assert_eq!(date_parts.len(), 3, "Expected YYYY-MM-DD format");
        assert!(
            date_parts[0].parse::<u16>().is_ok(),
            "Year should be numeric"
        );
        assert!(
            date_parts[1].parse::<u8>().is_ok(),
            "Month should be numeric"
        );
        assert!(date_parts[2].parse::<u8>().is_ok(), "Day should be numeric");
    }

    #[test]
    fn test_block_range_valid_patterns() {
        use regex::Regex;
        let info = UNICODE_REFERENCE;
        let re = Regex::new(r"U\+(\w+)([–-])U\+(\w+)").unwrap();

        let captures = re
            .captures(&info.block_range)
            .expect("Block range should match pattern U+XXXX–U+YYYY");

        assert_eq!(captures.get(1).unwrap().as_str(), "0590");
        assert_eq!(captures.get(3).unwrap().as_str(), "05FF");
    }

    #[test]
    fn test_script_field_consistency() {
        // Verify script matches block name semantics
        let info = UNICODE_REFERENCE;

        // Hebrew block should have Hebrew script designation
        assert_eq!(info.script, "Hebrew");
        assert_eq!(info.block_name, "Hebrew");
    }

    #[test]
    fn test_version_function_const_correctness() {
        // Ensure function returns same pointer as constant (no runtime allocation)
        const C: &str = UNICODE_VERSION;
        const F: &str = unicode_version();

        // Both should resolve to same static string at compile time
        assert_eq!(C, F);
    }

    #[test]
    fn test_reference_function_returns_static_lifetime() {
        // Verify unicode_version_info returns &'static (not heap)
        let info1 = unicode_version_info();
        let info2 = unicode_version_info();

        // Both should point to same static instance
        assert_eq!(std::ptr::eq(info1, info2), true);
    }
}
