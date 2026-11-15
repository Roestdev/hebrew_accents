use crate::api::AccentKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum InternalAccentKind {
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

impl From<InternalAccentKind> for AccentKind {
    fn from(internal: InternalAccentKind) -> Self {
        match internal {
            InternalAccentKind::Primary => AccentKind::Primary,
            InternalAccentKind::Secondary => AccentKind::Secondary,
        }
    }
}

#[cfg(test)]
mod tier_error_tests {
    use super::*;
    // Debug trait
    #[test]
    fn debug_output_contains_variant() {
        let dbg = format!("{:?}", InternalAccentKind::Primary);
        assert!(dbg.contains("Primary"), "unexpected Debug output: {dbg}");
    }
    // Clone/Copy trait
    #[test]
    fn clone_and_copy_semantics() {
        let original = InternalAccentKind::Primary;
        let copied = original; // Copy: moved AND original still valid
        let cloned = original.clone();
        assert_eq!(copied, original);
        assert_eq!(cloned, original);
    }

    // PartialEq / Eq
    #[test]
    fn partial_eq_works() {
        assert_eq!(InternalAccentKind::Primary, InternalAccentKind::Primary);
        assert_ne!(InternalAccentKind::Primary, InternalAccentKind::Secondary);
    }
    // Eq
    #[test]
    fn implements_eq() {
        fn assert_eq_trait<T: Eq>(_: &T) {}
        assert_eq_trait(&InternalAccentKind::Secondary);
    }
}
