// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use std::fmt;

/// **Disjunctive accent** hierarchy group level following Futato's classification system.
///
/// Ranges from Tier1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// Note:
///   In Prose accents, there are 4 group levels.
///   Poetry has 3 levels.
///
/// # Example
/// ```rust
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, Tier};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.tier(), Some(Tier::Tier1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.tier(), None);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Tier {
    /// Primary disjunctive tier — creates major clause/phrasal breaks
    Tier1 = 1, // value represents group number for extension in future
    /// Secondary disjunctive tier — subordinate phrase boundaries
    Tier2,
    /// Tertiary disjunctive tier — minor phrasal divisions
    Tier3,
    /// Quaternary disjunctive tier — fine-grained subdivisions
    Tier4,
}

impl Tier {
    /// Raw numeric strength value (1 = strongest disjunctive)
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Human-readable description of hierarchy tier groups
    pub const fn description(self) -> &'static str {
        match self {
            Self::Tier1 => "Primary disjunctive (major clause break)",
            Self::Tier2 => "Secondary disjunctive (phrase boundary)",
            Self::Tier3 => "Tertiary disjunctive (minor division)",
            Self::Tier4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
    /// Human-readable description of hierarchy tier groups
    ///
    /// In the literature other names are sometimes used
    pub const fn alt_description(self) -> &'static str {
        match self {
            Self::Tier1 => "Emperor",
            Self::Tier2 => "Kings",
            Self::Tier3 => "Dukes",
            Self::Tier4 => "Officers",
        }
    }
    /// Returns the maximum tier available for poetry accents.
    /// Poetry accents stop at Tier3; prose can reach Tier4.
    pub const fn max_poetry_tier() -> Self {
        Self::Tier3
    }
    /// Returns the maximum tier available for prose accents.
    pub const fn max_prose_tier() -> Self {
        Self::Tier4
    }
    /// Checks if this tier is available in poetry context.
    pub const fn is_poetry_available(self) -> bool {
        matches!(self, Self::Tier1 | Self::Tier2 | Self::Tier3)
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tier1 => write!(f, "Tier 1 (Primary disjunctive)"),
            Self::Tier2 => write!(f, "Tier 2 (Secondary disjunctive)"),
            Self::Tier3 => write!(f, "Tier 3 (Tertiary disjunctive)"),
            Self::Tier4 => write!(f, "Tier 4 (Quaternary disjunctive)"),
        }
    }
}

impl TryFrom<u8> for Tier {
    type Error = crate::TierError; // ← Use shared error type

    fn try_from(raw_value: u8) -> Result<Self, Self::Error> {
        match raw_value {
            1 => Ok(Self::Tier1),
            2 => Ok(Self::Tier2),
            3 => Ok(Self::Tier3),
            4 => Ok(Self::Tier4),
            _ => Err(crate::TierError::InvalidTier(raw_value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_error_message() {
        let err = Tier::try_from(7u8).unwrap_err();
        assert_eq!(err.to_string(), "Invalid Tier value: 7 (valid range: 1-4)");
    }

    #[test]
    fn test_tier_valid_conversions() {
        // All valid conversions should succeed
        assert_eq!(Tier::try_from(1u8), Ok(Tier::Tier1));
        assert_eq!(Tier::try_from(2u8), Ok(Tier::Tier2));
        assert_eq!(Tier::try_from(3u8), Ok(Tier::Tier3));
        assert_eq!(Tier::try_from(4u8), Ok(Tier::Tier4));
    }

    #[test]
    fn test_tier_value_method() {
        assert_eq!(Tier::Tier1.value(), 1);
        assert_eq!(Tier::Tier2.value(), 2);
        assert_eq!(Tier::Tier3.value(), 3);
        assert_eq!(Tier::Tier4.value(), 4);
    }

    #[test]
    fn test_tier_description_method() {
        assert_eq!(
            Tier::Tier1.description(),
            "Primary disjunctive (major clause break)"
        );
        assert_eq!(
            Tier::Tier2.description(),
            "Secondary disjunctive (phrase boundary)"
        );
        assert_eq!(
            Tier::Tier3.description(),
            "Tertiary disjunctive (minor division)"
        );
        assert_eq!(
            Tier::Tier4.description(),
            "Quaternary disjunctive (fine subdivision)"
        );
    }

    #[test]
    fn test_tier_alt_description_method() {
        assert_eq!(Tier::Tier1.alt_description(), "Emperor");
        assert_eq!(Tier::Tier2.alt_description(), "Kings");
        assert_eq!(Tier::Tier3.alt_description(), "Dukes");
        assert_eq!(Tier::Tier4.alt_description(), "Officers");
    }

    #[test]
    fn test_tier_max_methods() {
        assert_eq!(Tier::max_poetry_tier(), Tier::Tier3);
        assert_eq!(Tier::max_prose_tier(), Tier::Tier4);
    }

    #[test]
    fn test_tier_is_poetry_available() {
        // Poetry has tiers 1-3
        assert!(Tier::Tier1.is_poetry_available());
        assert!(Tier::Tier2.is_poetry_available());
        assert!(Tier::Tier3.is_poetry_available());
        // Tier4 is prose-only
        assert!(!Tier::Tier4.is_poetry_available());
    }

    #[test]
    fn test_tier_display() {
        assert_eq!(format!("{}", Tier::Tier1), "Tier 1 (Primary disjunctive)");
        assert_eq!(format!("{}", Tier::Tier2), "Tier 2 (Secondary disjunctive)");
        assert_eq!(format!("{}", Tier::Tier3), "Tier 3 (Tertiary disjunctive)");
        assert_eq!(
            format!("{}", Tier::Tier4),
            "Tier 4 (Quaternary disjunctive)"
        );
    }

    #[test]
    fn test_tier_debug_trait() {
        let debug_output = format!("{:?}", Tier::Tier1);
        assert!(debug_output.contains("Tier1"));
    }

    #[test]
    fn test_tier_copy_clone_trait() {
        let original = Tier::Tier1;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original, Tier::Tier1);
    }

    #[test]
    fn test_tier_eq_partial_eq_hash() {
        let l1 = Tier::Tier1;
        let l2 = Tier::Tier1;
        let l3 = Tier::Tier2;

        assert_eq!(l1, l2);
        assert_ne!(l1, l3);
        assert!(l1.eq(&l2));
    }

    #[test]
    fn test_try_from_u8_zero() {
        // Test boundary case: 0 should fail
        let result = Tier::try_from(0u8);
        assert!(matches!(result, Err(crate::TierError::InvalidTier(0))));
    }

    #[test]
    fn test_try_from_u8_boundary_min() {
        // Test minimum valid value
        let result = Tier::try_from(1u8);
        assert_eq!(result, Ok(Tier::Tier1));
    }

    #[test]
    fn test_try_from_u8_boundary_max() {
        // Test maximum valid value
        let result = Tier::try_from(4u8);
        assert_eq!(result, Ok(Tier::Tier4));
    }

    #[test]
    fn test_try_from_u8_above_max() {
        // Test above maximum
        let result = Tier::try_from(5u8);
        assert!(matches!(result, Err(crate::TierError::InvalidTier(5))));
    }

    #[test]
    fn test_try_from_u8_large_value() {
        // Test very large value
        let result = Tier::try_from(255u8);
        assert!(matches!(result, Err(crate::TierError::InvalidTier(255))));
    }

    #[test]
    fn test_tier_display_exact_output() {
        assert_eq!(format!("{}", Tier::Tier1), "Tier 1 (Primary disjunctive)");
        assert_eq!(format!("{}", Tier::Tier2), "Tier 2 (Secondary disjunctive)");
        assert_eq!(format!("{}", Tier::Tier3), "Tier 3 (Tertiary disjunctive)");
        assert_eq!(
            format!("{}", Tier::Tier4),
            "Tier 4 (Quaternary disjunctive)"
        );
    }
    // ============================================================
    // GROUP LEVEL - CONSTANT FUNCTIONS
    // ============================================================

    #[test]
    fn test_tier_value_const_fn() {
        const TIER1_VAL: u8 = Tier::Tier1.value();
        const TIER2_VAL: u8 = Tier::Tier2.value();
        const TIER3_VAL: u8 = Tier::Tier3.value();
        const TIER4_VAL: u8 = Tier::Tier4.value();

        assert_eq!(TIER1_VAL, 1);
        assert_eq!(TIER2_VAL, 2);
        assert_eq!(TIER3_VAL, 3);
        assert_eq!(TIER4_VAL, 4);
    }

    #[test]
    fn test_tier_description_const_fn() {
        const DESC1: &str = Tier::Tier1.description();
        const DESC2: &str = Tier::Tier2.description();
        const DESC3: &str = Tier::Tier3.description();
        const DESC4: &str = Tier::Tier4.description();

        assert_eq!(DESC1, "Primary disjunctive (major clause break)");
        assert_eq!(DESC2, "Secondary disjunctive (phrase boundary)");
        assert_eq!(DESC3, "Tertiary disjunctive (minor division)");
        assert_eq!(DESC4, "Quaternary disjunctive (fine subdivision)");
    }

    #[test]
    fn test_tier_alt_description_const_fn() {
        const ALT1: &str = Tier::Tier1.alt_description();
        const ALT2: &str = Tier::Tier2.alt_description();
        const ALT3: &str = Tier::Tier3.alt_description();
        const ALT4: &str = Tier::Tier4.alt_description();

        assert_eq!(ALT1, "Emperor");
        assert_eq!(ALT2, "Kings");
        assert_eq!(ALT3, "Dukes");
        assert_eq!(ALT4, "Officers");
    }

    #[test]
    fn test_tier_poetry_vs_prose_distinction() {
        // Poetry stops at Tier3
        assert_eq!(Tier::max_poetry_tier(), Tier::Tier3);
        assert!(Tier::Tier1.is_poetry_available());
        assert!(Tier::Tier2.is_poetry_available());
        assert!(Tier::Tier3.is_poetry_available());
        assert!(!Tier::Tier4.is_poetry_available());

        // Prose goes to Tier4
        assert_eq!(Tier::max_prose_tier(), Tier::Tier4);
    }

    #[test]
    fn test_tier_tier4_prose_only() {
        // Tier4 is specifically prose-only
        assert!(!Tier::Tier4.is_poetry_available());
        assert_eq!(Tier::max_poetry_tier(), Tier::Tier3);
    }

    // ============================================================
    // HASH AND PARTIAL_EQ TRAIT TESTING
    // ============================================================

    #[test]
    fn test_tier_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        Tier::Tier1.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        Tier::Tier1.hash(&mut h2);
        let hash2 = h2.finish();

        assert_eq!(hash1, hash2);
    }
}
