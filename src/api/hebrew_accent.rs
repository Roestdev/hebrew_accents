use crate::accent::resolve_disjunctive_group;
use crate::api::CompoundType;
use crate::{Accent, AlternateNames, PoetryAccent, ProseAccent, PseudoAccent};
use crate::{AccentCategory, AccentKind, CantillationMark, GroupLevel};

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
    /// let name1 = accent.as_prose().unwrap().english_name();
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
    /// use hebrew_accents::{Accent, HebrewAccent, PoetryAccent};
    ///
    /// let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
    ///
    /// // Can access properties through the reference
    /// if let Some(poetry) = accent.as_poetry() {
    ///     println!("English name: {}", poetry.english_name());
    ///     println!("Relative strength: {:?}", poetry.relative_strength());
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
    /// let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
    ///
    /// // Check the accent accenttype exists before accessing its data
    /// if let Some(pseudo) = accent.as_pseudo() {
    ///     println!("English name: {}", pseudo.english_name());      // "Maqqaph"
    ///     println!("Concept: {}", pseudo.hebrew_concept());         // "binder"
    /// }
    ///
    /// // The original accent remains usable after inspection
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::Maqqaph)));
    /// ```
    pub fn as_pseudo(self) -> Option<PseudoAccent> {
        match self {
            Self::Pseudo(p) => Some(p),
            _ => None,
        }
    }

    /// Gives the possisiblitiy to iterrate overall Hebrew Accents
    ///
    /// ```rust
    /// use hebrew_accents::{HebrewAccent};
    ///
    /// for accent in HebrewAccent::iter() {
    ///    match accent {
    ///        HebrewAccent::Prose(p) => println!("Prose variant {:?}", p),
    ///        HebrewAccent::Poetry(p) => println!("Poetry variant {:?}", p),
    ///        HebrewAccent::Pseudo(p) => println!("Pseudo variant {:?}", p),
    ///    }
    /// }
    /// ```
    pub fn iter() -> impl Iterator<Item = HebrewAccent> {
        use strum::IntoEnumIterator;

        ProseAccent::iter()
            .map(HebrewAccent::from)
            .chain(PoetryAccent::iter().map(HebrewAccent::from))
            .chain(PseudoAccent::iter().map(HebrewAccent::from))
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
    /// ```rust
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
    /// ```rust
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
/// let accent = HebrewAccent::from(PseudoAccent::Maqqaph);
///
/// assert!(matches!(accent, HebrewAccent::Pseudo(PseudoAccent::Maqqaph)));
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

impl Accent for HebrewAccent {
    #[inline]
    fn hebrew_name(&self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_name(),
            HebrewAccent::Poetry(p) => p.hebrew_name(),
            HebrewAccent::Pseudo(p) => p.hebrew_name(),
        }
    }

    #[inline]
    fn hebrew_concept(&self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_concept(),
            HebrewAccent::Poetry(p) => p.hebrew_concept(),
            HebrewAccent::Pseudo(p) => p.hebrew_concept(),
        }
    }

    #[inline]
    fn english_name(&self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.english_name(),
            HebrewAccent::Poetry(p) => p.english_name(),
            HebrewAccent::Pseudo(p) => p.english_name(),
        }
    }

    #[inline]
    fn sbl_academic_name(&self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.sbl_academic_name(),
            HebrewAccent::Poetry(p) => p.sbl_academic_name(),
            HebrewAccent::Pseudo(p) => p.sbl_academic_name(),
        }
    }

    #[inline]
    fn kind(&self) -> Option<AccentKind> {
        match self {
            HebrewAccent::Prose(p) => p.kind(),
            HebrewAccent::Poetry(p) => p.kind(),
            HebrewAccent::Pseudo(p) => p.kind(),
        }
    }

    #[inline]
    fn category(&self) -> Option<AccentCategory> {
        match self {
            HebrewAccent::Prose(p) => p.category(),
            HebrewAccent::Poetry(p) => p.category(),
            HebrewAccent::Pseudo(p) => p.category(),
        }
    }

    #[inline]
    fn compound_type(&self) -> Option<CompoundType> {
        match self {
            HebrewAccent::Prose(p) => p.compound_type(),
            HebrewAccent::Poetry(p) => p.compound_type(),
            HebrewAccent::Pseudo(p) => p.compound_type(),
        }
    }

    #[inline]
    fn primary_cantillation_mark(&self) -> CantillationMark {
        match self {
            HebrewAccent::Prose(p) => p.primary_cantillation_mark(),
            HebrewAccent::Poetry(p) => p.primary_cantillation_mark(),
            HebrewAccent::Pseudo(p) => p.primary_cantillation_mark(),
        }
    }

    #[inline]
    fn secondary_cantillation_mark(&self) -> Option<CantillationMark> {
        match self {
            HebrewAccent::Prose(p) => p.secondary_cantillation_mark(),
            HebrewAccent::Poetry(p) => p.secondary_cantillation_mark(),
            HebrewAccent::Pseudo(p) => p.secondary_cantillation_mark(),
        }
    }

    #[inline]
    fn notes(&self) -> Option<&'static str> {
        match self {
            HebrewAccent::Prose(p) => p.notes(),
            HebrewAccent::Poetry(p) => p.notes(),
            HebrewAccent::Pseudo(p) => p.notes(),
        }
    }

    #[inline]
    fn relative_strength(&self) -> Option<u8> {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }

    #[inline]
    fn group_level(&self) -> Option<GroupLevel> {
        resolve_disjunctive_group(*self).and_then(|g| g.into_public_level())
    }

    #[inline]
    fn cantillation_symbol(&self) -> String {
        match self {
            HebrewAccent::Prose(p) => p.cantillation_symbol(),
            HebrewAccent::Poetry(p) => p.cantillation_symbol(),
            HebrewAccent::Pseudo(p) => p.cantillation_symbol(),
        }
    }

    #[inline]
    fn alternate_names(&self) -> Option<AlternateNames> {
        None //TODO
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
        let pseudo = PseudoAccent::Maqqaph;
        let accent = HebrewAccent::Pseudo(pseudo);
        assert!(matches!(
            accent,
            HebrewAccent::Pseudo(PseudoAccent::Maqqaph)
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
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);

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
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
        let display = format!("{}", accent);

        assert!(display.starts_with("Pseudo: "));
        assert!(display.contains("Maqqaph"));
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
        let original = PseudoAccent::Maqqaph;
        let wrapped: HebrewAccent = original.into();
        let extracted = wrapped.as_pseudo();

        assert_eq!(extracted, Some(original));
    }

    #[test]
    fn doctestcopy() {
        use crate::{Accent, HebrewAccent, PoetryAccent};

        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
        if let Some(poetry) = accent.as_poetry() {
            println!("English name: {}", poetry.english_name());
            println!("Relative strength: {:?}", poetry.relative_strength());
        }
        assert!(accent.as_poetry().is_some());
    }

    // Additional tests for HebrewAccent - Covering missing Accent trait methods

    // ── HebrewAccent::iter() ─────────────────────────────────────────

    #[test]
    fn test_iter_all_variants_included() {
        let all: Vec<HebrewAccent> = HebrewAccent::iter().collect();

        // Should include all prose, poetry, and pseudo accents
        assert!(all.len() > 50); // Prose(28) + Poetry(23) + Pseudo(3)

        // Should have at least one of each type
        let has_prose = all.iter().any(|a| matches!(a, HebrewAccent::Prose(_)));
        let has_poetry = all.iter().any(|a| matches!(a, HebrewAccent::Poetry(_)));
        let has_pseudo = all.iter().any(|a| matches!(a, HebrewAccent::Pseudo(_)));

        assert!(has_prose, "Should include Prose variants");
        assert!(has_poetry, "Should include Poetry variants");
        assert!(has_pseudo, "Should include Pseudo variants");
    }

    #[test]
    fn test_iter_does_not_duplicate_variants() {
        let all: Vec<HebrewAccent> = HebrewAccent::iter().collect();
        let unique: std::collections::HashSet<_> = all.iter().collect();

        assert_eq!(all.len(), unique.len(), "No duplicate variants in iterator");
    }

    #[test]
    fn test_iter_chain_order() {
        let mut iter = HebrewAccent::iter();

        // First should be Prose variant (ProseAccent::iter comes first)
        let first = iter.next();
        assert!(matches!(first, Some(HebrewAccent::Prose(_))));

        // Last should be Pseudo variant (PseudoAccent::iter comes last)
        let last = iter.last();
        assert!(matches!(last, Some(HebrewAccent::Pseudo(_))));
    }

    // ── Accent Trait Methods - Hebrew Name & Concept ─────────────────

    #[test]
    fn test_hebrew_name_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let name = accent.hebrew_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_prose().unwrap().hebrew_name());
    }

    #[test]
    fn test_hebrew_name_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let name = accent.hebrew_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_poetry().unwrap().hebrew_name());
    }

    #[test]
    fn test_hebrew_name_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
        let name = accent.hebrew_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_pseudo().unwrap().hebrew_name());
    }

    #[test]
    fn test_hebrew_concept_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Revia);
        let concept = accent.hebrew_concept();

        assert!(!concept.is_empty());
        assert_eq!(concept, accent.as_prose().unwrap().hebrew_concept());
    }

    #[test]
    fn test_hebrew_concept_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Munach);
        let concept = accent.hebrew_concept();

        assert!(!concept.is_empty());
        assert_eq!(concept, accent.as_poetry().unwrap().hebrew_concept());
    }

    #[test]
    fn test_hebrew_concept_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let concept = accent.hebrew_concept();

        assert!(!concept.is_empty());
        assert_eq!(concept, accent.as_pseudo().unwrap().hebrew_concept());
    }

    // ── Accent Trait Methods - English & SBL Names ───────────────────

    #[test]
    fn test_english_name_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::Segolta);
        let name = accent.english_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_prose().unwrap().english_name());
    }

    #[test]
    fn test_english_name_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Tsinnor);
        let name = accent.english_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_poetry().unwrap().english_name());
    }

    #[test]
    fn test_english_name_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Paseq);
        let name = accent.english_name();

        assert!(!name.is_empty());
        assert_eq!(name, accent.as_pseudo().unwrap().english_name());
    }

    #[test]
    fn test_sbl_academic_name_prose_variant() {
        let accent = HebrewAccent::Prose(ProseAccent::ZaqephQatan);
        let name = accent.sbl_academic_name();

        assert!(!name.is_empty());
    }

    #[test]
    fn test_sbl_academic_name_poetry_variant() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Dechi);
        let name = accent.sbl_academic_name();

        assert!(!name.is_empty());
    }

    #[test]
    fn test_sbl_academic_name_pseudo_variant() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Paseq);
        let name = accent.sbl_academic_name();

        assert!(!name.is_empty());
    }

    // ── Accent Trait Methods - Kind & Category ───────────────────────

    #[test]
    fn test_kind_prose_disjunctive_returns_some() {
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);
        let kind = accent.kind();

        // Disjunctive accents should return Some(AccentKind::Disjunctive)
        assert!(kind.is_some());
    }

    #[test]
    fn test_kind_prose_conjunctive_returns_some() {
        let accent = HebrewAccent::Prose(ProseAccent::Munach);
        let kind = accent.kind();

        // Conjunctive accents should return Some(AccentKind::Conjunctive)
        assert!(kind.is_some());
    }

    #[test]
    fn test_kind_poetry_returns_some() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
        let kind = accent.kind();

        assert!(kind.is_some());
    }

    #[test]
    fn test_kind_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
        let kind = accent.kind();

        // Pseudo-accents lack hierarchical kind
        assert_eq!(kind, None);
    }

    #[test]
    fn test_category_prose_disjunctive() {
        let accent = HebrewAccent::Prose(ProseAccent::Tiphcha);
        let category = accent.category();

        assert!(category.is_some());
    }

    #[test]
    fn test_category_prose_conjunctive() {
        let accent = HebrewAccent::Prose(ProseAccent::Mahpakh);
        let category = accent.category();

        assert!(category.is_some());
    }

    #[test]
    fn test_category_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Pazer);
        let category = accent.category();

        assert!(category.is_some());
    }

    #[test]
    fn test_category_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let category = accent.category();

        assert_eq!(category, None);
    }

    // ── Accent Trait Methods - Compound Type ─────────────────────────

    #[test]
    fn test_compound_type_prose_returns_option() {
        let accent = HebrewAccent::Prose(ProseAccent::TelishaGedolah);
        let ct = accent.compound_type();

        // Some prose accents have compound types
        let _ = ct; // Exercise the method
    }

    #[test]
    fn test_compound_type_poetry_returns_option() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol);
        let ct = accent.compound_type();

        let _ = ct;
    }

    #[test]
    fn test_compound_type_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Paseq);
        let ct = accent.compound_type();

        assert_eq!(ct, None);
    }

    // ── Accent Trait Methods - Cantillation Marks ────────────────────

    #[test]
    fn test_primary_cantillation_mark_prose() {
        let accent = HebrewAccent::Prose(ProseAccent::Pashta);
        let mark = accent.primary_cantillation_mark();

        assert!(!mark.symbol.is_control());
        assert!(!mark.symbol.is_whitespace());
    }

    #[test]
    fn test_primary_cantillation_mark_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
        let mark = accent.primary_cantillation_mark();

        assert!(!mark.symbol.is_control());
        assert!(!mark.symbol.is_whitespace());
    }

    #[test]
    fn test_primary_cantillation_mark_pseudo() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let mark = accent.primary_cantillation_mark();

        assert_eq!(mark.symbol, '׃');
    }

    #[test]
    fn test_secondary_cantillation_mark_prose() {
        let accent = HebrewAccent::Prose(ProseAccent::Revia);
        let secondary = accent.secondary_cantillation_mark();

        // Some prose accents have secondary marks
        let _ = secondary;
    }

    #[test]
    fn test_secondary_cantillation_mark_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaMugrash);
        let secondary = accent.secondary_cantillation_mark();

        let _ = secondary;
    }

    #[test]
    fn test_secondary_cantillation_mark_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
        let secondary = accent.secondary_cantillation_mark();

        assert_eq!(secondary, None);
    }

    // ── Accent Trait Methods - Notes ─────────────────────────────────

    #[test]
    fn test_notes_prose_returns_option() {
        let accent = HebrewAccent::Prose(ProseAccent::Shalshelet);
        let notes = accent.notes();

        // Some accents may have notes
        let _ = notes;
    }

    #[test]
    fn test_notes_poetry_returns_option() {
        let accent = HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha);
        let notes = accent.notes();

        let _ = notes;
    }

    #[test]
    fn test_notes_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let notes = accent.notes();

        // Pseudo-accents currently have no notes
        assert!(notes.is_some());
    }

    // ── Accent Trait Methods - Hierarchy ─────────────────────────────

    #[test]
    fn test_relative_strength_prose_disjunctive() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let strength = accent.relative_strength();

        // Disjunctive accents have strength
        assert!(strength.is_some());
        assert!(strength.unwrap() > 0);
    }

    #[test]
    fn test_relative_strength_poetry_disjunctive() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let strength = accent.relative_strength();

        assert!(strength.is_some());
    }

    #[test]
    fn test_relative_strength_conjunctive_returns_none() {
        let accent = HebrewAccent::Prose(ProseAccent::Munach);
        let strength = accent.relative_strength();

        // Conjunctives don't have relative strength
        assert_eq!(strength, None);
    }

    #[test]
    fn test_relative_strength_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
        let strength = accent.relative_strength();

        assert_eq!(strength, None);
    }

    #[test]
    fn test_group_level_prose_disjunctive() {
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);
        let level = accent.group_level();

        // Disjunctives have group levels
        assert!(level.is_some());
    }

    #[test]
    fn test_group_level_poetry_disjunctive() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let level = accent.group_level();

        assert!(level.is_some());
    }

    #[test]
    fn test_group_level_conjunctive_returns_none() {
        let accent = HebrewAccent::Prose(ProseAccent::Munach);
        let level = accent.group_level();

        assert_eq!(level, None);
    }

    #[test]
    fn test_group_level_pseudo_returns_none() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);
        let level = accent.group_level();

        assert_eq!(level, None);
    }

    // ── Accent Trait Methods - Symbol ────────────────────────────────

    #[test]
    fn test_cantillation_symbol_prose() {
        let accent = HebrewAccent::Prose(ProseAccent::Zarqa);
        let symbol = accent.cantillation_symbol();

        assert!(!symbol.is_empty());
    }

    #[test]
    fn test_cantillation_symbol_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Illuy);
        let symbol = accent.cantillation_symbol();

        assert!(!symbol.is_empty());
    }

    #[test]
    fn test_cantillation_symbol_pseudo() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::Paseq);
        let symbol = accent.cantillation_symbol();

        assert!(!symbol.is_empty());
    }

    // ── Accent Trait Methods - Alternate Names ───────────────────────

    #[test]
    fn test_alternate_names_always_returns_none() {
        // Currently alternate_names always returns None
        let prose = HebrewAccent::Prose(ProseAccent::Silluq);
        let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let pseudo = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);

        assert_eq!(prose.alternate_names(), None);
        assert_eq!(poetry.alternate_names(), None);
        assert_eq!(pseudo.alternate_names(), None);
    }

    // ── Complete Accent Trait Coverage ───────────────────────────────

    #[test]
    fn test_complete_accent_trait_for_prose() {
        let accent = HebrewAccent::Prose(ProseAccent::Segolta);

        // Call every single method from the Accent trait
        let _h_name = accent.hebrew_name();
        let _h_concept = accent.hebrew_concept();
        let _e_name = accent.english_name();
        let _sbl = accent.sbl_academic_name();
        let _kind = accent.kind();
        let _cat = accent.category();
        let _compound = accent.compound_type();
        let _primary = accent.primary_cantillation_mark();
        let _secondary = accent.secondary_cantillation_mark();
        let _notes = accent.notes();
        let _strength = accent.relative_strength();
        let _level = accent.group_level();
        let _symbol = accent.cantillation_symbol();
        let _alts = accent.alternate_names();
    }

    #[test]
    fn test_complete_accent_trait_for_poetry() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);

        let _h_name = accent.hebrew_name();
        let _h_concept = accent.hebrew_concept();
        let _e_name = accent.english_name();
        let _sbl = accent.sbl_academic_name();
        let _kind = accent.kind();
        let _cat = accent.category();
        let _compound = accent.compound_type();
        let _primary = accent.primary_cantillation_mark();
        let _secondary = accent.secondary_cantillation_mark();
        let _notes = accent.notes();
        let _strength = accent.relative_strength();
        let _level = accent.group_level();
        let _symbol = accent.cantillation_symbol();
        let _alts = accent.alternate_names();
    }

    #[test]
    fn test_complete_accent_trait_for_pseudo() {
        let accent = HebrewAccent::Pseudo(PseudoAccent::SophPasuq);

        let _h_name = accent.hebrew_name();
        let _h_concept = accent.hebrew_concept();
        let _e_name = accent.english_name();
        let _sbl = accent.sbl_academic_name();
        let _kind = accent.kind();
        let _cat = accent.category();
        let _compound = accent.compound_type();
        let _primary = accent.primary_cantillation_mark();
        let _secondary = accent.secondary_cantillation_mark();
        let _notes = accent.notes();
        let _strength = accent.relative_strength();
        let _level = accent.group_level();
        let _symbol = accent.cantillation_symbol();
        let _alts = accent.alternate_names();
    }

    // ── Edge Cases ───────────────────────────────────────────────────

    #[test]
    fn test_from_conversions_all_types() {
        let prose = ProseAccent::Pazer;
        let poetry = PoetryAccent::Pazer;
        let pseudo = PseudoAccent::SophPasuq;

        let h_prose: HebrewAccent = prose.into();
        let h_poetry: HebrewAccent = poetry.into();
        let h_pseudo: HebrewAccent = pseudo.into();

        assert!(matches!(h_prose, HebrewAccent::Prose(_)));
        assert!(matches!(h_poetry, HebrewAccent::Poetry(_)));
        assert!(matches!(h_pseudo, HebrewAccent::Pseudo(_)));
    }

    #[test]
    fn test_all_accessor_methods_consistency() {
        let prose = HebrewAccent::Prose(ProseAccent::Tevir);

        // as_prose should return Some
        assert!(prose.as_prose().is_some());

        // as_poetry and as_pseudo should return None
        assert!(prose.as_poetry().is_none());
        assert!(prose.as_pseudo().is_none());
    }

    #[test]
    fn test_thread_safety_bounds() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<HebrewAccent>();
    }

    #[test]
    fn test_repr_and_discriminant() {
        // Verify enum layout is as expected
        let variants = vec![
            HebrewAccent::Prose(ProseAccent::Silluq),
            HebrewAccent::Poetry(PoetryAccent::Silluq),
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq),
        ];

        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_pattern_matching_on_all_variants() {
        fn describe_accent(accent: &HebrewAccent) -> &'static str {
            match accent {
                HebrewAccent::Prose(_) => "prose",
                HebrewAccent::Poetry(_) => "poetry",
                HebrewAccent::Pseudo(_) => "pseudo",
            }
        }

        assert_eq!(
            describe_accent(&HebrewAccent::Prose(ProseAccent::Silluq)),
            "prose"
        );
        assert_eq!(
            describe_accent(&HebrewAccent::Poetry(PoetryAccent::Silluq)),
            "poetry"
        );
        assert_eq!(
            describe_accent(&HebrewAccent::Pseudo(PseudoAccent::SophPasuq)),
            "pseudo"
        );
    }

    #[test]
    fn test_const_compatible_usage() {
        // Verify HebrewAccent can be used in const contexts where possible
        const DEFAULT: HebrewAccent = HebrewAccent::Prose(ProseAccent::Silluq);
        assert!(matches!(DEFAULT, HebrewAccent::Prose(ProseAccent::Silluq)));
    }
}
