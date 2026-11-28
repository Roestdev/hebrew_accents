// TODO test_derived_traits
// TODO test subject
// TODO test_impl
use std::fmt;

/// Hebrew Accent kind — (absence is expressed via `Option<T>`)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentKind {
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

impl fmt::Display for AccentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primary => write!(f, "primary"),
            Self::Secondary => write!(f, "secondary"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accent_kind_variants() {
        assert_eq!(format!("{}", AccentKind::Primary), "primary");
        assert_eq!(format!("{}", AccentKind::Secondary), "secondary");
    }

    #[test]
    fn test_accent_kind_debug_trait() {
        let debug = format!("{:?}", AccentKind::Primary);
        assert!(debug.contains("Primary"));
    }

    #[test]
    fn test_accent_kind_copy_clone_eq_hash() {
        let k1 = AccentKind::Primary;
        let k2 = AccentKind::Primary;

        assert_eq!(k1, k2);
        assert_eq!(k1.clone(), k2);
    }

    #[test]
    fn test_accent_kind_display_exact_output() {
        assert_eq!(format!("{}", AccentKind::Primary), "primary");
        assert_eq!(format!("{}", AccentKind::Secondary), "secondary");
    }
    #[test]
    fn test_accent_kind_hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        AccentKind::Primary.hash(&mut h1);
        let hash1 = h1.finish();

        let mut h2 = DefaultHasher::new();
        AccentKind::Primary.hash(&mut h2);
        let hash2 = h2.finish();

        assert_eq!(hash1, hash2);
    }
}
