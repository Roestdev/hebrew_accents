use crate::{PoetryAccent, ProseAccent, PseudoAccent};

/// Hebrew Accent, either a Prose or Poetry accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HebrewAccent {
    /// Prose variant
    Prose(ProseAccent),
    /// Poetry variant
    Poetry(PoetryAccent),
    /// Pseudo variant
    Pseudo(PseudoAccent),
}
impl HebrewAccent {
    /// Returns a reference to the inner [`ProseAccent`] if this is a Prose variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Prose variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Silluq);
    /// assert!(matches!(accent.as_prose(), Some(ProseAccent::Silluq)));
    /// ```
    ///
    /// Returns `None` for non-Prose variants:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
    /// assert_eq!(accent.as_prose(), None);
    /// ```
    ///
    /// The returned reference can be used multiple times without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{Accent, HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Segolta);
    ///
    /// // First usage
    /// let name1 = accent.as_prose().unwrap().sbl_simplified_name();
    ///
    /// // Second usage - original still available
    /// let name2 = accent.as_prose().unwrap().hebrew_concept();
    ///
    /// assert_eq!(name1, "Segolta");
    /// ```
    pub fn as_prose(self) -> Option<ProseAccent> {
        match self {
            Self::Prose(p) => Some(p),
            _ => None,
        }
    }

    /// Returns a reference to the inner [`PoetryAccent`] if this is a Poetry variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Poetry variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
    /// assert!(matches!(accent.as_poetry(), Some(PoetryAccent::OlehWeYored)));
    /// ```
    ///
    /// Failing to extract from a Prose variant (returns `None`):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::Prose(ProseAccent::Atnach);
    /// assert_eq!(accent.as_poetry(), None);
    /// ```
    ///
    /// Accessing methods on the borrowed inner value without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
    ///
    /// // Can access properties through the reference
    /// if let Some(poetry) = accent.as_poetry() {
    ///     println!("English name: {}", poetry.sbl_simplified_name());
    ///     println!("Relative strength: {}", poetry.relative_strength());
    /// }
    ///
    /// // Original accent remains usable after inspection
    /// assert!(accent.as_poetry().is_some());
    /// ```
    pub fn as_poetry(self) -> Option<PoetryAccent> {
        match self {
            Self::Poetry(p) => Some(p),
            _ => None,
        }
    }

    /// Returns a reference to the inner [`PseudoAccent`] if this is a Pseudo variant.
    ///
    /// # Examples
    ///
    /// Successfully extracting from a Pseudo variant:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, PseudoAccent, Accent};
    ///
    /// let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::SophPasuq)));
    /// ```
    ///
    /// Failing to extract from Prose or Poetry variants (returns `None`):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent, PoetryAccent};
    ///
    /// let prose = HebrewAccent::Prose(ProseAccent::Silluq);
    /// let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
    ///
    /// assert_eq!(prose.as_pseudo(), None);
    /// assert_eq!(poetry.as_pseudo(), None);
    /// ```
    ///
    /// Inspecting PseudoAccent properties without consuming the wrapper:
    ///
    /// ```
    /// use hebrew_accents::{Accent,HebrewAccent, PseudoAccent};
    ///
    /// let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);
    ///
    /// // Check the accent accenttype exists before accessing its data
    /// if let Some(pseudo) = accent.as_pseudo() {
    ///     println!("English name: {}", pseudo.sbl_simplified_name());      // "Maqqeph"
    ///     println!("Concept: {}", pseudo.hebrew_concept());         // "binder"
    /// }
    ///
    /// // The original accent remains usable after inspection
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::Maqqeph)));
    /// ```
    pub fn as_pseudo(self) -> Option<PseudoAccent> {
        match self {
            Self::Pseudo(p) => Some(p),
            _ => None,
        }
    }
}

impl std::fmt::Display for HebrewAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prose(p) => write!(f, "Prose: {}", p),
            Self::Poetry(p) => write!(f, "Poetry: {}", p),
            Self::Pseudo(p) => write!(f, "Pseudo: {}", p),
        }
    }
}

impl From<ProseAccent> for HebrewAccent {
    /// Converts a [`ProseAccent`] into a [`HebrewAccent`] wrapper.
    ///
    /// # Examples
    ///
    /// Basic conversion using `.into()`:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let prose = ProseAccent::Silluq;
    /// let accent: HebrewAccent = prose.into();
    ///
    /// assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Silluq)));
    /// ```
    ///
    /// Using explicit `From::from()` syntax:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let accent = HebrewAccent::from(ProseAccent::Atnach);
    ///
    /// assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Atnach)));
    /// ```
    ///
    /// Type inference works when the target accenttype is clear from context:
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// fn accept_hebrew(accent: impl Into<HebrewAccent>) {
    ///     // Function body...
    /// }
    ///
    /// // No need to call .into() explicitly here!
    /// accept_hebrew(ProseAccent::Segolta);
    /// ```
    ///
    /// Multiple conversions in sequence (e.g., collecting into vectors):
    ///
    /// ```
    /// use hebrew_accents::{HebrewAccent, ProseAccent};
    ///
    /// let prose_accents = vec![
    ///     ProseAccent::Silluq,
    ///     ProseAccent::Atnach,
    ///     ProseAccent::Revia,
    /// ];
    ///
    /// let hebrew_accents: Vec<HebrewAccent> = prose_accents
    ///     .into_iter()
    ///     .map(|p| p.into())
    ///     .collect();
    ///
    /// assert_eq!(hebrew_accents.len(), 3);
    /// ```
    fn from(a: ProseAccent) -> Self {
        HebrewAccent::Prose(a)
    }
}

/// Converts a [`PoetryAccent`] into a [`HebrewAccent`] by wrapping it in the `Poetry` variant.
///
/// # Examples
///
/// Wrapping a poetry accent:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PoetryAccent};
///
/// let poetry = PoetryAccent::Atnach;
/// let accent: HebrewAccent = poetry.into();
///
/// assert!(matches!(accent, HebrewAccent::Poetry(PoetryAccent::Atnach)));
/// ```
///
/// Using explicit `From` trait:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PoetryAccent};
/// use std::convert::From;
///
/// let poetry = PoetryAccent::Silluq;
/// let accent = HebrewAccent::from(poetry);
///
/// assert!(matches!(accent, HebrewAccent::Poetry(_)));
/// ```
impl From<PoetryAccent> for HebrewAccent {
    fn from(a: PoetryAccent) -> Self {
        HebrewAccent::Poetry(a)
    }
}

/// Converts a [`PseudoAccent`] into a [`HebrewAccent`] by wrapping it in the `Pseudo` variant.
/// # Examples
///
/// Using `.into()` for implicit conversion:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// let pseudo = PseudoAccent::SophPasuq;
/// let accent: HebrewAccent = pseudo.into();
///
/// assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::SophPasuq)));
/// ```
///
/// Using `From::from()` explicitly:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// let accent = HebrewAccent::from(PseudoAccent::Maqqeph);
///
/// assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::Maqqeph)));
/// ```
///
/// Automatic accenttype coercion in function arguments:
///
/// ```
/// use hebrew_accents::{HebrewAccent, PseudoAccent};
///
/// fn accepts_hebrew(accent: impl Into<HebrewAccent>) {}
///
/// // No explicit .into() needed at call site
/// accepts_hebrew(PseudoAccent::Paseq);
/// ```
impl From<PseudoAccent> for HebrewAccent {
    fn from(a: PseudoAccent) -> Self {
        HebrewAccent::Pseudo(a)
    }
}

impl Default for HebrewAccent {
    fn default() -> Self {
        HebrewAccent::Prose(ProseAccent::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PoetryAccent, ProseAccent, PseudoAccent};

    // ── Variant construction ────────────────────────────────────────

    #[test]
    fn prose_variant_wraps_correctly() {
        let prose = ProseAccent::Silluq;
        let accent = HebrewAccent::Prose(prose);
        assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Silluq)));
    }

    #[test]
    fn poetry_variant_wraps_correctly() {
        let poetry = PoetryAccent::Atnach;
        let accent = HebrewAccent::Poetry(poetry);
        assert!(matches!(accent, HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }

    #[test]
    fn pseudo_variant_wraps_correctly() {
        let pseudo = PseudoAccent::Maqqeph;
        let accent = HebrewAccent::Pseudo(pseudo);
        assert!(matches!(
            accent,
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph)
        ));
    }

    // ── as_prose accessor ───────────────────────────────────────────

    #[test]
    fn as_prose_returns_some_for_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        assert!(matches!(accent.as_prose(), Some(ProseAccent::Silluq)));
    }

    #[test]
    fn as_prose_returns_none_for_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        assert_eq!(accent.as_prose(), None);
    }

    #[test]
    fn as_prose_returns_none_for_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        assert_eq!(accent.as_prose(), None); // wait, this tests as_pseudo!
    }

    #[test]
    fn as_prose_does_not_consume_wrapper() {
        let accent = HebrewAccent::Prose(ProseAccent::Segolta);

        let first = accent.as_prose().unwrap();
        let second = accent.as_prose().unwrap();

        assert_eq!(first, second);
    }

    // ── as_poetry accessor ──────────────────────────────────────────

    #[test]
    fn as_poetry_returns_some_for_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
        assert!(matches!(
            accent.as_poetry(),
            Some(PoetryAccent::OlehWeYored)
        ));
    }

    #[test]
    fn as_poetry_returns_none_for_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);
        assert_eq!(accent.as_poetry(), None);
    }

    #[test]
    fn as_poetry_returns_none_for_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Paseq);
        assert_eq!(accent.as_poetry(), None);
    }

    #[test]
    fn as_poetry_does_not_consume_wrapper() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);

        let first = accent.as_poetry().unwrap();
        let second = accent.as_poetry().unwrap();

        assert_eq!(first, second);
    }

    // ── as_pseudo accessor ──────────────────────────────────────────

    #[test]
    fn as_pseudo_returns_some_for_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::SophPasuq)));
    }

    #[test]
    fn as_pseudo_returns_none_for_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        assert_eq!(accent.as_pseudo(), None);
    }

    #[test]
    fn as_pseudo_returns_none_for_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        assert_eq!(accent.as_pseudo(), None);
    }

    #[test]
    fn as_pseudo_does_not_consume_wrapper() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);

        let first = accent.as_pseudo().unwrap();
        let second = accent.as_pseudo().unwrap();

        assert_eq!(first, second);
    }

    // ── From trait implementations ──────────────────────────────────

    #[test]
    fn from_prose_accent_wraps_in_prose() {
        let prose = ProseAccent::Segolta;
        let accent: HebrewAccent = prose.into();

        assert!(matches!(accent, HebrewAccent::Prose(ProseAccent::Segolta)));
    }

    #[test]
    fn from_poetry_accent_wraps_in_poetry() {
        let poetry = PoetryAccent::Silluq;
        let accent: HebrewAccent = poetry.into();

        assert!(matches!(accent, HebrewAccent::Poetry(PoetryAccent::Silluq)));
    }

    #[test]
    fn from_pseudo_accent_wraps_in_pseudo() {
        let pseudo = PseudoAccent::Paseq;
        let accent: HebrewAccent = pseudo.into();

        assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::Paseq)));
    }

    #[test]
    fn from_trait_uses_explicit_syntax() {
        let prose = ProseAccent::Atnach;
        let accent = HebrewAccent::from(prose);

        assert!(matches!(accent, HebrewAccent::Prose(_)));
    }

    #[test]
    fn collect_into_vector_via_from() {
        let prose_accents = vec![ProseAccent::Silluq, ProseAccent::Atnach, ProseAccent::Revia];

        let hebrew_accents: Vec<HebrewAccent> =
            prose_accents.into_iter().map(|p| p.into()).collect();

        assert_eq!(hebrew_accents.len(), 3);
        assert!(matches!(
            hebrew_accents[0],
            HebrewAccent::Prose(ProseAccent::Silluq)
        ));
    }

    // ── Default implementation ──────────────────────────────────────

    #[test]
    fn default_is_prose_silluq() {
        let default = HebrewAccent::default();

        assert!(matches!(default, HebrewAccent::Prose(ProseAccent::Silluq)));
    }

    // ── Display implementation ──────────────────────────────────────

    #[test]
    fn display_formats_prose_with_prefix() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let display = format!("{}", accent);

        assert!(display.starts_with("Prose: "));
        assert!(display.contains("Silluq"));
    }

    #[test]
    fn display_formats_poetry_with_prefix() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let display = format!("{}", accent);

        assert!(display.starts_with("Poetry: "));
        assert!(display.contains("Atnach"));
    }

    #[test]
    fn display_formats_pseudo_with_prefix() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);
        let display = format!("{}", accent);

        assert!(display.starts_with("Pseudo: "));
        assert!(display.contains("Maqqeph"));
    }

    #[test]
    fn display_variants_are_distinct() {
        let prose = format!("{}", HebrewAccent::Prose(ProseAccent::Silluq));
        let poetry = format!("{}", HebrewAccent::Poetry(PoetryAccent::Silluq));
        let pseudo = format!("{}", HebrewAccent::Pseudo(PseudoAccent::SophPasuq));

        assert_ne!(prose, poetry);
        assert_ne!(poetry, pseudo);
        assert_ne!(prose, pseudo);
    }

    // ── Debug implementation ────────────────────────────────────────

    #[test]
    fn debug_output_contains_variant_label() {
        let prose = format!("{:?}", HebrewAccent::Prose(ProseAccent::Silluq));
        let poetry = format!("{:?}", HebrewAccent::Poetry(PoetryAccent::Atnach));
        let pseudo = format!("{:?}", HebrewAccent::Pseudo(PseudoAccent::Paseq));

        assert!(prose.contains("Prose"));
        assert!(poetry.contains("Poetry"));
        assert!(pseudo.contains("Pseudo"));
    }

    // ── Equality and inequality ─────────────────────────────────────

    #[test]
    fn same_variant_same_inner_value_are_equal() {
        let a = HebrewAccent::Prose(ProseAccent::Silluq);
        let b = HebrewAccent::Prose(ProseAccent::Silluq);

        assert_eq!(a, b);
    }

    #[test]
    fn same_variant_different_inner_value_are_not_equal() {
        let a = HebrewAccent::Prose(ProseAccent::Silluq);
        let b = HebrewAccent::Prose(ProseAccent::Atnach);

        assert_ne!(a, b);
    }

    #[test]
    fn different_variants_are_not_equal_even_with_similar_names() {
        // Both have "Silluq" but different types
        let prose_silluq = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry_silluq = HebrewAccent::Poetry(PoetryAccent::Silluq);

        assert_ne!(prose_silluq, poetry_silluq);
    }

    #[test]
    fn all_three_variant_types_are_mutually_unequal() {
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        assert_ne!(prose, poetry);
        assert_ne!(poetry, pseudo);
        assert_ne!(prose, pseudo);
    }

    // ── Copy and Clone ──────────────────────────────────────────────

    #[test]
    fn copy_preserves_value() {
        let original = HebrewAccent::Prose(ProseAccent::Revia);
        let copied = original; // relies on Copy

        assert_eq!(original, copied);
    }

    #[test]
    fn clone_preserves_value() {
        let original = HebrewAccent::Poetry(PoetryAccent::Munach);

        assert_eq!(original, original.clone());
    }

    // ── Hash consistency ────────────────────────────────────────────

    #[test]
    fn hash_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();
        let accent = HebrewAccent::Prose(ProseAccent::Pazer);

        accent.hash(&mut h1);
        accent.hash(&mut h2);

        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn different_variants_have_different_hashes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();

        HebrewAccent::Prose(ProseAccent::Silluq).hash(&mut h1);
        HebrewAccent::Poetry(PoetryAccent::Silluq).hash(&mut h2);

        assert_ne!(h1.finish(), h2.finish());
    }

    // ── Cross-type extraction verification ──────────────────────────

    #[test]
    fn prose_accents_fail_poetry_extraction() {
        let prose = HebrewAccent::Prose(ProseAccent::ZaqephQatan);
        let poetry_result = prose.as_poetry();
        let pseudo_result = prose.as_pseudo();

        assert_eq!(poetry_result, None);
        assert_eq!(pseudo_result, None);
    }

    #[test]
    fn poetry_accents_fail_prose_extraction() {
        let poetry = HebrewAccent::Poetry(PoetryAccent::Dechi);
        let prose_result = poetry.as_prose();
        let pseudo_result = poetry.as_pseudo();

        assert_eq!(prose_result, None);
        assert_eq!(pseudo_result, None);
    }

    #[test]
    fn pseudo_accents_fail_prose_and_poetry_extraction() {
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let prose_result = pseudo.as_prose();
        let poetry_result = pseudo.as_poetry();

        assert_eq!(prose_result, None);
        assert_eq!(poetry_result, None);
    }

    // ── Round-trip conversion ───────────────────────────────────────

    #[test]
    fn prose_roundtrip_through_wrapper() {
        let original = ProseAccent::Tevir;
        let wrapped: HebrewAccent = original.into();
        let extracted = wrapped.as_prose();

        assert_eq!(extracted, Some(original));
    }

    #[test]
    fn poetry_roundtrip_through_wrapper() {
        let original = PoetryAccent::Illuy;
        let wrapped: HebrewAccent = original.into();
        let extracted = wrapped.as_poetry();

        assert_eq!(extracted, Some(original));
    }

    #[test]
    fn pseudo_roundtrip_through_wrapper() {
        let original = PseudoAccent::Maqqeph;
        let wrapped: HebrewAccent = original.into();
        let extracted = wrapped.as_pseudo();

        assert_eq!(extracted, Some(original));
    }
}
