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
    ///     println!("Relative strength: {:?}", poetry).relative_strength());
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
    /// There are 21 prose books and 3 poetry books in the Tanach
    /// It seems thereby logic to select the default prose accent
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

#[cfg(test)]
mod test_accent_trait {
    // ── Accent trait for HebrewAccent ──────────────────────────────────────
    //  TODO Hebrew name
    mod hebrew_name {
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        #[test]
        fn test_prose_disjunctive_hebrew_name() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).hebrew_name(),
                "סִלּוּק"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).hebrew_name(),
                "אַתְנַח"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).hebrew_name(),
                "סְגֹולְתָּא"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).hebrew_name(),
                "שַׁלְשֶׁלֶת"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).hebrew_name(),
                "זָקֵף קָטוֹן"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).hebrew_name(),
                "זָקֵף גָּדוֹל"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).hebrew_name(),
                "רְבִיעַ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).hebrew_name(),
                "טִפְחָא"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).hebrew_name(),
                "זַרְקָא"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).hebrew_name(),
                "פַּשְׁטָא"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).hebrew_name(),
                "יְתִיב"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).hebrew_name(),
                "תְּבִיר"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).hebrew_name(),
                "גֵּרֵישׁ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).hebrew_name(),
                "גֵּרְשַׁיִם"
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Pazer).hebrew_name(), "פָּזֶר");
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).hebrew_name(),
                "פָּזֶר גּדוֹל"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).hebrew_name(),
                "תְּלִישָׁא גְּדוֹלָה"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).hebrew_name(),
                "לְגַרְמֶהּ"
            );
        }

        #[test]
        fn test_prose_conjunctive_hebrew_name() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).hebrew_name(),
                "מוּנַ֣ח"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).hebrew_name(),
                "מַהְפַּךְ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).hebrew_name(),
                "מֵרְכָא"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).hebrew_name(),
                "מֵרְכָא כְּפוּלָה"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).hebrew_name(),
                "דַּרְגָּא"
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Azla).hebrew_name(), "אַזְלָא");
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).hebrew_name(),
                "תְּלִישָא קְטַנָּה"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).hebrew_name(),
                "גַּלְגַּל",
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).hebrew_name(),
                "מְאַיְלָא"
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Meteg).hebrew_name(), "מֶתֶג");
        }

        #[test]
        fn test_poetry_disjunctive_hebrew_name() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).hebrew_name(),
                "סִלּוּק"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).hebrew_name(),
                "עֹלֶה וְיֹרֵד"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).hebrew_name(),
                "אַתְנַח"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).hebrew_name(),
                "רְבִיעַ גָּדוֹל"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).hebrew_name(),
                "רְבִיעַ מֻגְרָשׁ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).hebrew_name(),
                "שַׁלְשֶׁלֶת גָּדוֹל"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).hebrew_name(),
                "צִנּוֹר"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).hebrew_name(),
                "רְבִיעַ קָטוֹן"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).hebrew_name(),
                "דֶּחִי"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).hebrew_name(),
                "פָּזֶר"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).hebrew_name(),
                "מְהֻפָּךְ לְגַרְמֵהּ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).hebrew_name(),
                "אַזְלָא לְגַרְמֶהּ"
            );
        }

        /// Test all poetry conjunctive accents
        #[test]
        fn test_poetry_conjunctive_hebrew_name() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).hebrew_name(),
                "מוּנַ֣ח"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).hebrew_name(),
                "מֵרְכָא"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).hebrew_name(),
                "עִלּוּי"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).hebrew_name(),
                "טַרְחָא"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).hebrew_name(),
                "גַּלְגַּל",
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).hebrew_name(),
                "מְהֻפָּ֤ךְ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).hebrew_name(),
                "אַזְלָא"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).hebrew_name(),
                "שַׁלְשֶׁלֶת קְטַנָּה"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).hebrew_name(),
                "צִנּוֹרִת מֵרְכָא"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).hebrew_name(),
                "צִנּוֹרִת מַהְפַּךְ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).hebrew_name(),
                "מֶתֶג"
            );
        }
        #[test]
        fn test_pseudo_accent_hebrew_name() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).hebrew_name(),
                "סוֹף פָּסוּק"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).hebrew_name(),
                "מַקָּף"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).hebrew_name(),
                "פָּסֵק"
            );
        }
    }

    mod hebrew_concept {
        //  TODO Hebrew concept
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        #[test]
        fn test_prose_disjunctive_hebrew_concept() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).hebrew_concept(),
                "close, cessation"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).hebrew_concept(),
                "a causing to rest"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).hebrew_concept(),
                "a little grape-bunch"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).hebrew_concept(),
                "chain or link"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).hebrew_concept(),
                "small upright or small standing one"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).hebrew_concept(),
                "large upright"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).hebrew_concept(),
                "fourth [in a sequence]"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).hebrew_concept(),
                "handbreadth or diagonal"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).hebrew_concept(),
                "to sprinkle, to scatter"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).hebrew_concept(),
                "extending, stretching out in length"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).hebrew_concept(),
                "resting or sitting"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).hebrew_concept(),
                "broken, downward tumble"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).hebrew_concept(),
                "expulsion, driving out, divorce"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).hebrew_concept(),
                "double of expulsion, driving out, divorce"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).hebrew_concept(),
                "lavish or scatter"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).hebrew_concept(),
                "large lavish or scatter"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).hebrew_concept(),
                "great (long) detached"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).hebrew_concept(),
                "for or by itself, independent"
            );
        }

        #[test]
        fn test_prose_conjunctive_hebrew_concept() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).hebrew_concept(),
                "resting or placed"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).hebrew_concept(),
                "turning round"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).hebrew_concept(),
                "lengthener, prolonging"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).hebrew_concept(),
                "double lengthener"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).hebrew_concept(),
                "step, stair-step"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).hebrew_concept(),
                "going on (not pausing), depart"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).hebrew_concept(),
                "small (short) detached"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).hebrew_concept(),
                "wheel, circle"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).hebrew_concept(),
                "upper, above, higher"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).hebrew_concept(),
                "to bridle, check, control"
            );
        }

        #[test]
        fn test_poetry_disjunctive_hebrew_concept() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).hebrew_concept(),
                "close, cessation"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).hebrew_concept(),
                "ascending and descending"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).hebrew_concept(),
                "a causing to rest"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).hebrew_concept(),
                "big fourth"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).hebrew_concept(),
                "exiled fourth"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).hebrew_concept(),
                "large chain or link"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).hebrew_concept(),
                "pipe or tube"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).hebrew_concept(),
                "small fourth"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).hebrew_concept(),
                "to push or drive away"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).hebrew_concept(),
                "lavish or scatter"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).hebrew_concept(),
                "reversed to its own"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).hebrew_concept(),
                "goes to its own"
            );
        }

        #[test]
        fn test_poetry_conjunctive_hebrew_concept() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).hebrew_concept(),
                "resting or placed"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).hebrew_concept(),
                "lengthener, prolonging"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).hebrew_concept(),
                "elevation or raising"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).hebrew_concept(),
                "trouble, difficulty, hardship, toil"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).hebrew_concept(),
                "wheel, circle"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).hebrew_concept(),
                "reversed"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).hebrew_concept(),
                "going on (not pausing), depart"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).hebrew_concept(),
                "small chain"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).hebrew_concept(),
                "pipe of continuation"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).hebrew_concept(),
                "pipe of reversal"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).hebrew_concept(),
                "to bridle, check, control"
            );
        }

        #[test]
        fn test_pseudo_accent_hebrew_concept() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).hebrew_concept(),
                "end of verse"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).hebrew_concept(),
                "binder"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).hebrew_concept(),
                "to pause, to stop or to interrupt"
            );
        }
    }

    mod english_name {
        // TODO English name

        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        #[test]
        fn test_prose_disjunctive_english_name() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).english_name(),
                "Silluq"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).english_name(),
                "Atnach"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).english_name(),
                "Segolta"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).english_name(),
                "Shalshelet"
            );
            // TODO assert_eq!(HebrewAccent::Prose(ProseAccent::ZaqephQatan).english_name(), "Zaqeph Qatan");
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).english_name(),
                "Zaqeph Gadol"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).english_name(),
                "Revia"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).english_name(),
                "Tiphcha"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).english_name(),
                "Zarqa"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).english_name(),
                "Pashta"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).english_name(),
                "Yetiv"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).english_name(),
                "Tevir"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).english_name(),
                "Geresh"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).english_name(),
                "Gershayim"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).english_name(),
                "Pazer"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).english_name(),
                "Pazer Gadol"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).english_name(),
                "Telisha Gedolah"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).english_name(),
                "Legarmeh"
            );
        }

        #[test]
        fn test_prose_conjunctive_english_name() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).english_name(),
                "Munach"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).english_name(),
                "Mahpakh"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).english_name(),
                "Merkha"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).english_name(),
                "Merkha Kephulah"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).english_name(),
                "Darga"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).english_name(),
                "Azla"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).english_name(),
                "Telisha Qetannah"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).english_name(),
                "Galgal"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).english_name(),
                "Meayla"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).english_name(),
                "Meteg"
            );
        }

        /// Silluq, Atnach,pazer zie prose // TODO
        #[test]
        fn test_poetry_disjunctive_english_name() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).english_name(),
                "Silluq"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).english_name(),
                "Oleh WeYored"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).english_name(),
                "Atnach"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).english_name(),
                "Revia Gadol"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).english_name(),
                "Revia Mugrash"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).english_name(),
                "Shalshelet Gadol"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).english_name(),
                "Tsinnor"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).english_name(),
                "Revia Qaton"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).english_name(),
                "Dechi"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).english_name(),
                "Pazer"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).english_name(),
                "Mehuppakh Legarmeh"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).english_name(),
                "Azla Legarmeh"
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        /// // TODO common: munach,merkha,galgal,azla,meteg
        #[test]
        fn test_poetry_conjunctive_english_name() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).english_name(),
                "Munach"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).english_name(),
                "Merkha"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).english_name(),
                "Illuy"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).english_name(),
                "Tarcha"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).english_name(),
                "Galgal"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).english_name(),
                "Mehuppakh"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).english_name(),
                "Azla"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).english_name(),
                "Shalshelet Qetannah"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).english_name(),
                "Tsinnorit Merkha"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).english_name(),
                "Tsinnorit Mahpakh"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).english_name(),
                "Meteg"
            );
        }

        #[test]
        fn test_pseudo_accent_english_name() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).english_name(),
                "Soph Pasuq"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).english_name(),
                "Maqqaph"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).english_name(),
                "Paseq"
            );
        }
    }

    mod sbl_academic_name {
        // TODO SBL academic name
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        #[test]
        fn test_prose_disjunctive_sbl_academic_name() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).sbl_academic_name(),
                "sillûq"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).sbl_academic_name(),
                "ʾatnaḥ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).sbl_academic_name(),
                "səgôlətāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).sbl_academic_name(),
                "šalšelet"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).sbl_academic_name(),
                "zāqēp qāṭôn"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).sbl_academic_name(),
                "zāqēp gādôl"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).sbl_academic_name(),
                "rəbîaʿ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).sbl_academic_name(),
                "ṭipḥāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).sbl_academic_name(),
                "zarqāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).sbl_academic_name(),
                "pašṭāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).sbl_academic_name(),
                "yətîb"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).sbl_academic_name(),
                "təbîr"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).sbl_academic_name(),
                "gērêš"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).sbl_academic_name(),
                "gērəšayīm"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).sbl_academic_name(),
                "pāzer"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).sbl_academic_name(),
                "pāzer gdôl"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).sbl_academic_name(),
                "təlîšāʾ gədôlâ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).sbl_academic_name(),
                "ləgarmeh"
            );
        }

        #[test]
        fn test_prose_conjunctive_sbl_academic_name() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).sbl_academic_name(),
                "mûnaḥ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).sbl_academic_name(),
                "mahpak"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).sbl_academic_name(),
                "mērəkāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).sbl_academic_name(),
                "mērəkāʾ kəpûlâ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).sbl_academic_name(),
                "dargāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).sbl_academic_name(),
                "ʾazlāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).sbl_academic_name(),
                "təlîšāʾ qəṭannâ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).sbl_academic_name(),
                "galgal"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).sbl_academic_name(),
                "məʾaylāʾ"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).sbl_academic_name(),
                "meteg"
            );
        }
        #[test]
        fn test_poetry_disjunctive_sbl_academic_name() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).sbl_academic_name(),
                "sillûq"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).sbl_academic_name(),
                "ʿōleh wəyōrēd"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).sbl_academic_name(),
                "ʾatnaḥ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).sbl_academic_name(),
                "rəbîaʿ gādôl"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).sbl_academic_name(),
                "rəbîaʿ mugrāš"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).sbl_academic_name(),
                "šalšelet gādôl"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).sbl_academic_name(),
                "ṣinnôr"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).sbl_academic_name(),
                "rəbîaʿ qāṭôn"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).sbl_academic_name(),
                "deḥî"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).sbl_academic_name(),
                "pāzer"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).sbl_academic_name(),
                "məhuppāk ləgarmēh"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).sbl_academic_name(),
                "ʾazlāʾ ləgarmēh"
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        /// // TODO common: munach,merkha,galgal,azla,meteg
        #[test]
        fn test_poetry_conjunctive_sbl_academic_name() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).sbl_academic_name(),
                "mûnaḥ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).sbl_academic_name(),
                "mērəkāʾ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).sbl_academic_name(),
                "ʿillûy"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).sbl_academic_name(),
                "ṭarḥāʾ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).sbl_academic_name(),
                "galgal"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).sbl_academic_name(),
                "məhuppāk"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).sbl_academic_name(),
                "ʾazlāʾ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).sbl_academic_name(),
                "šalšelet qəṭannâ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).sbl_academic_name(),
                "ṣinnôrīt mērəkāʾ"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).sbl_academic_name(),
                ""
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).sbl_academic_name(),
                "meteg"
            );
        }
        #[test]
        fn test_pseudo_accent_sbl_academic_name() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).sbl_academic_name(),
                "sôp pāsûq"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).sbl_academic_name(),
                "maqqēp"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).sbl_academic_name(),
                "pāsēq"
            );
        }
    }

    mod kind {
        // TODO kind
        use crate::{Accent, AccentKind, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        #[test]
        fn test_prose_disjunctive_kind() {
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).kind(),
                Some(AccentKind::Primary)
            );
        }

        #[test]
        fn test_prose_conjunctive_kind() {
            // Conjunctives: indices AccentKind::Primary-AccentKind::Primary, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).kind(),
                Some(AccentKind::Secondary)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).kind(),
                Some(AccentKind::Secondary)
            );
        }
        #[test]
        fn test_poetry_disjunctive_kind() {
            // Primary disjunctives: indices AccentKind::Primary-AccentKind::Primary, strengths AccentKind::Primary-AccentKind::Primary
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).kind(),
                Some(AccentKind::Primary)
            );
        }

        #[test]
        fn test_poetry_conjunctive_kind() {
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).kind(),
                Some(AccentKind::Primary)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).kind(),
                Some(AccentKind::Secondary)
            );
        }
        #[test]
        fn test_pseudo_accent_kind() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::SophPasuq).kind(), None);
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqaph).kind(), None);
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).kind(), None);
        }
    }

    mod category {
        // TODO category
        use crate::{
            Accent, AccentCategory, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent,
        };
        #[test]
        fn test_prose_disjunctive_category() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).category(),
                Some(AccentCategory::Disjunctive)
            );
        }

        #[test]
        fn test_prose_conjunctive_category() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).category(),
                Some(AccentCategory::Conjunctive)
            );
        }

        #[test]
        fn test_poetry_disjunctive_category() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).category(),
                Some(AccentCategory::Disjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).category(),
                Some(AccentCategory::Disjunctive)
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_category() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).category(),
                Some(AccentCategory::Conjunctive)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).category(),
                Some(AccentCategory::Conjunctive)
            );
        }

        #[test]
        fn test_pseudo_accent_category() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).category(),
                None
            );
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Maqqaph).category(), None);
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).category(), None);
        }
    }

    mod compound_type {
        // TODO compound type
        use crate::{Accent, CompoundType, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

        /// Test all prose disjunctive accents (should return Some(strength))
        #[test]
        fn test_prose_disjunctive_compound_type() {
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).compound_type(),
                Some(CompoundType::ContainsPaseq)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).compound_type(),
                Some(CompoundType::ContainsPaseq)
            );
        }

        #[test]
        fn test_prose_conjunctive_compound_type() {
            // Conjunctives: indices CompoundType::Primary-CompoundType::Primary, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).compound_type(),
                None
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Azla).compound_type(), None);
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).compound_type(),
                None
            );
        }
        #[test]
        fn test_poetry_disjunctive_compound_type() {
            // Primary disjunctives: indices CompoundType::Primary-CompoundType::Primary, strengths CompoundType::Primary-CompoundType::Primary
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).compound_type(),
                Some(CompoundType::CanSpanTwoWords)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).compound_type(),
                Some(CompoundType::Standard)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).compound_type(),
                Some(CompoundType::ContainsPaseq)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).compound_type(),
                Some(CompoundType::ContainsPaseq)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).compound_type(),
                Some(CompoundType::ContainsPaseq)
            );
        }

        #[test]
        fn test_poetry_conjunctive_compound_type() {
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).compound_type(),
                Some(CompoundType::CanSpanTwoWords)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).compound_type(),
                Some(CompoundType::CanSpanTwoWords)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).compound_type(),
                None
            );
        }
        #[test]
        fn test_pseudo_accent_compound_type() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).compound_type(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).compound_type(),
                None
            );
        }
    }

    mod primary_cantillation_mark {
        // TODO primary cantillation mark
        use crate::{
            Accent, CantillationMark, CantillationMarkPlacement, CantillationMarkStressPosition,
            HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent,
        };
        /// Test all prose disjunctive accents (should return Some(strength))
        #[test]
        fn test_prose_disjunctive_primary_cantillation_mark() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5bd}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{591}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{592}',
                    placement: CantillationMarkPlacement::AboveLeft,
                    stress_position: Some(CantillationMarkStressPosition::Postpositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{593}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{594}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{595}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{597}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{596}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5ae}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Postpositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{599}',
                    placement: CantillationMarkPlacement::AboveLeft,
                    stress_position: Some(CantillationMarkStressPosition::Postpositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59a}',
                    placement: CantillationMarkPlacement::BelowRight,
                    stress_position: Some(CantillationMarkStressPosition::Prepositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59b}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59c}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59e}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a1}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59f}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a0}',
                    placement: CantillationMarkPlacement::AboveRight,
                    stress_position: Some(CantillationMarkStressPosition::Prepositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a3}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_primary_cantillation_mark() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a3}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a4}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a5}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a6}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a7}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a8}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a9}',
                    placement: CantillationMarkPlacement::AboveLeft,
                    stress_position: Some(CantillationMarkStressPosition::Postpositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5aa}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{596}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5bd}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
        }

        #[test]
        fn test_poetry_disjunctive_primary_cantillation_mark() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5bd}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5ab}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{591}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{597}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{59c}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{593}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5ae}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Postpositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{597}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5ad}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a1}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a4}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a8}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_primary_cantillation_mark() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a3}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a5}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5ac}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{596}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5aa}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a4}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5a8}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{593}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{598}',
                    placement: CantillationMarkPlacement::AboveLeft,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{598}',
                    placement: CantillationMarkPlacement::AboveLeft,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '\u{5bd}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                }
            );
        }

        #[test]
        fn test_pseudo_accent_primary_cantillation_mark() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '׃',
                    placement: CantillationMarkPlacement::SofPasuq,
                    stress_position: None
                }
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '־',
                    placement: CantillationMarkPlacement::Maqqaph,
                    stress_position: None
                }
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).primary_cantillation_mark(),
                CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                }
            );
        }
    }

    mod secondary_cantillation_mark {
        // TODO secondary cantillation mark
        use crate::{
            Accent, CantillationMark, CantillationMarkPlacement, CantillationMarkStressPosition,
            HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent,
        };
        #[test]
        fn test_prose_disjunctive_secondary_cantillation_mark() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                })
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                })
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_secondary_cantillation_mark() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).secondary_cantillation_mark(),
                None
            );
        }
        /// Test all poetry disjunctive accents (should return Some(strength))
        #[test]
        fn test_poetry_disjunctive_secondary_cantillation_mark() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '\u{5a5}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '\u{597}',
                    placement: CantillationMarkPlacement::AboveCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '׀',
                    placement: CantillationMarkPlacement::Paseq,
                    stress_position: None
                })
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_secondary_cantillation_mark() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                    .secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '\u{5a5}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).secondary_cantillation_mark(),
                Some(CantillationMark {
                    symbol: '\u{5a4}',
                    placement: CantillationMarkPlacement::BelowCenter,
                    stress_position: Some(CantillationMarkStressPosition::Impositive)
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).secondary_cantillation_mark(),
                None
            );
        }
        #[test]
        fn test_pseudo_accent_secondary_cantillation_mark() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).secondary_cantillation_mark(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).secondary_cantillation_mark(),
                None
            );
        }
    }

    mod notes {
        // TODO notes
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
        #[test]
        fn test_prose_disjunctive_notes() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).notes(),
                Some(
                    r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
                )
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).notes(),
                Some(
                    r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
                )
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).notes(),
                Some(r#"Appears frequently in prose, preceding Atnach or Silluq."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).notes(),
                Some(r#"Rare accent occurring only four times in the Torah."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).notes(),
                Some(r#"Common disjunctive, often appearing before Zaqeph Gadol or Atnach."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).notes(),
                Some(r#"Less common than Zaqeph Qaton; appears after Qadma or Azla."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).notes(),
                Some(r#"probably due to its four-note tune."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).notes(),
                Some(r#"before Atnach and Silluq"#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).notes(),
                Some(r#"before Segolta"#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).notes(),
                Some(r#"appears on the last syllable of the word, extending rightward."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).notes(),
                Some(r#"occasionally substitutes for a Pashta"#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).notes(),
                Some(r#"Often followed by Revia; indicates a melodic break."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).notes(),
                Some(r#"Functions as Level 4 disjunctive; often preceded by Azla."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).notes(),
                Some(r#"Double Geresh; rarer variant appearing in specific syntactic contexts."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).notes(),
                Some(
                    r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
                )
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).notes(),
                Some(r#"Also known as Qarne Pharah; rare variant resembling horns."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).notes(),
                Some(r#"Level 4 disjunctive; distinct from Telisha Qetannah."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).notes(),
                Some(r#"Munach with Passeq; Before Revia"#)
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_notes() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).notes(),
                Some(r#"Most common conjunctive; precedes disjunctive accents."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).notes(),
                Some(r#"Conjunctive accent; in poetry can function as Mehuppakh Legarmeh."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).notes(),
                Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).notes(),
                Some(r#"Merkha duplex"#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).notes(),
                Some(r#"Conjunctive accent; synonymous with Qadma in some traditions."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).notes(),
                Some(r#"When Geresh: Qadma"#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).notes(),
                Some(r#"Short form of Telisha; conjunctive function."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).notes(),
                Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).notes(),
                Some(
                    r#"Name given to a Tiphcha, when in the same word as Atnach or Silluq.
    This accent is historically the same glyph as Tipcha (U+0596) but functions as a secondary conjunctive 
    in specific syntactic contexts.
    Different traditions use different names: 
        Tipcha (common), Tarḥa, or Me'ayla/Mayela (rare scholarly names)."#,
                )
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).notes(),
                Some(
                    r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak.
    It is possible to have both a Meteg and Silluq in one word,
    where the Meteg comes before the Silluq.!"#,
                )
            );
        }
        #[test]
        fn test_poetry_disjunctive_notes() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).notes(),
                Some(
                    r#"The terms Silluq and Sof Pasuq are indifferently used for the final accent of the verse.
            It is possible to have both a Silluq and Meteg in one word,
            where the Meteg comes before the Silluq.
            The Silluq appears once per verse.
"#
                )
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).notes(),
                Some(
                    r#"The primary_mark looks like a Mehuppakh, 
but is located above the consonant in stead of below.
It is then called OLE. When preceded by no place to attach, Yored remains alone.
The alternate BHS naming is fabricated"#,
                )
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).notes(),
                Some(
                    r#"The major disjunctive accent dividing the verse in two halves.
    It appears only once per verse"#
                )
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).notes(),
                Some(r#"Poetry Level 2 disjunctive; equivalent to Revia in prose."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).notes(),
                Some(r#"Revia with Geresh secondary mark; indicates exiled position."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).notes(),
                Some(r#"Poetic variant of Shalshelet; larger chain form."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).notes(),
                Some(r#"Poetry equivalent of Zarqa; appears in poetic books."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).notes(),
                Some(r#"After that occurs Oleh Weyored"#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).notes(),
                Some(r#"Poetry Level 2 disjunctive; related to Tiphcha."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).notes(),
                Some(
                    r#"Level 4 disjunctive; can appear in place of Zaqeph Qaton in certain conditions."#,
                )
            ); // TODO otherlevelin poetry
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).notes(),
                Some(
                    r#"While the search results suggest that Mahpach Legarmeh as a complete compound term 
        is most distinctly associated with the Ashkenazi tradition, 
        all four traditions recognize the same grammatical function (a Mahpakh functioning disjunctively with a Paseq). 
        The other traditions may use the base name Mahpakh, and determine the Legarmeh function from context, 
        rather than using the full compound name systematically. 
        This reflects a broader pattern where Ashkenazi terminology tends to be more explicit with compound names 
        (e.g., Munach Legarmeh), while Sephardi and other traditions may use simpler designations."#,
                )
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).notes(),
                Some(r#"Compound disjunctive in poetry; Qadma with Paseq."#)
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_notes() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).notes(),
                Some(r#"Most common conjunctive; precedes disjunctive accents."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).notes(),
                Some(r#"Lengthens the phrase; often precedes Revia or Telisha."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).notes(),
                Some(r#"also called Munach superior"#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).notes(),
                Some(
                    r#"it refers to the effort, strain, or inconvenience involved in doing something."#
                )
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).notes(),
                Some(r#"Conjunctive accent; also known as Yerach Ben Yomo."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).notes(),
                Some(r#"Poetic conjunctive; variant spelling of Mahpakh."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).notes(),
                Some(r#"When Geresh: Qadma"#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).notes(),
                Some(r#"Poetic conjunctive; smaller chain form."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).notes(),
                Some(r#"Compound conjunctive in poetry; Tsinnor with Merkha secondary."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).notes(),
                Some(r#"Compound conjunctive in poetry; Tsinnor with Mahpakh secondary."#)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).notes(),
                Some(
                    r#"In Masoretic Hebrew grammar, גַּעְיָה (gaʿyah) is a technical term referring to a secondary vocal stress,
    an additional emphasis placed on a syllable beyond the primary word stress. 
    It is physically marked by the מֶתֶג (meteg), a short vertical line (U+05BD) placed to the left of a vowel point.
    The metaphor is vivid: just as an ox lows or bellows to draw attention, 
    the gaʿyah mark draws the reader's attention to a secondary stressed syllable 
    that might otherwise be swallowed or de-emphasized. It "makes the syllable cry out," so to speak.
    It is possible to have both a Meteg and Silluq in one word,
    where the Meteg comes before the Silluq.!"#,
                )
            );
        }
        #[test]
        fn test_pseudo_accent_notes() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::SophPasuq).notes(), Some(
        "it doesn't carry any theological or interpretive meaning beyond marking a boundary",
    ));
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).notes(),
                Some(
                    "Can link two (or more) short words together after which 
    they function as a single compound word bearing a single Hebrew accent.",
                )
            );
            assert_eq!(HebrewAccent::Pseudo(PseudoAccent::Paseq).notes(), Some(
        "It's indicating that someone or something is stopping temporarily or creating a pause.",
    ));
        }
    }

    mod relative_strength {
        // TODO relative strength
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
        #[test]
        fn test_prose_disjunctive_strength() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).relative_strength(),
                Some(1)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).relative_strength(),
                Some(2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).relative_strength(),
                Some(3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).relative_strength(),
                Some(4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).relative_strength(),
                Some(5)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).relative_strength(),
                Some(6)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).relative_strength(),
                Some(7)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).relative_strength(),
                Some(8)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).relative_strength(),
                Some(9)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).relative_strength(),
                Some(10)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).relative_strength(),
                Some(11)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).relative_strength(),
                Some(12)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).relative_strength(),
                Some(13)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).relative_strength(),
                Some(14)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).relative_strength(),
                Some(15)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).relative_strength(),
                Some(16)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).relative_strength(),
                Some(17)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).relative_strength(),
                Some(18)
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_strength() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).relative_strength(),
                None
            );
        }
        #[test]
        fn test_poetry_disjunctive_strength() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).relative_strength(),
                Some(1)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).relative_strength(),
                Some(2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).relative_strength(),
                Some(3)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).relative_strength(),
                Some(4)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).relative_strength(),
                Some(5)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).relative_strength(),
                Some(6)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).relative_strength(),
                Some(7)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).relative_strength(),
                Some(8)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).relative_strength(),
                Some(9)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).relative_strength(),
                Some(10)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).relative_strength(),
                Some(11)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).relative_strength(),
                Some(12)
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_strength() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).relative_strength(),
                None
            );
        }
        #[test]
        fn test_pseudo_accent_strength() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).relative_strength(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).relative_strength(),
                None
            );
        }
    }

    mod group_level {
        // TODO group level
        use crate::{Accent, GroupLevel, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
        #[test]
        fn test_prose_disjunctive_group_level() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).group_level(),
                Some(GroupLevel::Tier1)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).group_level(),
                Some(GroupLevel::Tier1)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).group_level(),
                Some(GroupLevel::Tier4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).group_level(),
                Some(GroupLevel::Tier4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).group_level(),
                Some(GroupLevel::Tier4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).group_level(),
                Some(GroupLevel::Tier4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).group_level(),
                Some(GroupLevel::Tier4)
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).group_level(),
                Some(GroupLevel::Tier4)
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_group_level() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(HebrewAccent::Prose(ProseAccent::Munach).group_level(), None);
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).group_level(),
                None
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Merkha).group_level(), None);
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).group_level(),
                None
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Darga).group_level(), None);
            assert_eq!(HebrewAccent::Prose(ProseAccent::Azla).group_level(), None);
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).group_level(),
                None
            );
            assert_eq!(HebrewAccent::Prose(ProseAccent::Galgal).group_level(), None);
            assert_eq!(HebrewAccent::Prose(ProseAccent::Meayla).group_level(), None);
            assert_eq!(HebrewAccent::Prose(ProseAccent::Meteg).group_level(), None);
        }
        #[test]
        fn test_poetry_disjunctive_group_level() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).group_level(),
                Some(GroupLevel::Tier1)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).group_level(),
                Some(GroupLevel::Tier1)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).group_level(),
                Some(GroupLevel::Tier1)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).group_level(),
                Some(GroupLevel::Tier2)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).group_level(),
                Some(GroupLevel::Tier3)
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).group_level(),
                Some(GroupLevel::Tier3)
            );
        }

        /// Test all poetry conjunctive accents (should return None)
        #[test]
        fn test_poetry_conjunctive_group_level() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).group_level(),
                None
            );
            assert_eq!(HebrewAccent::Poetry(PoetryAccent::Azla).group_level(), None);
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).group_level(),
                None
            );
        }
        #[test]
        fn test_pseudo_accent_group_level() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).group_level(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).group_level(),
                None
            );
        }
    }

    mod cantillation_symbol {
        // TODO cantillation symbol
        use crate::{Accent, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
        #[test]
        fn test_prose_disjunctive_cantillation_symbol() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).cantillation_symbol(),
                "// TODO"
            );
        }

        #[test]
        fn test_prose_conjunctive_cantillation_symbol() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).cantillation_symbol(),
                "// TODO"
            );
        }
        #[test]
        fn test_poetry_disjunctive_cantillation_symbol() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).cantillation_symbol(),
                "// TODO"
            );
        }

        #[test]
        fn test_poetry_conjunctive_cantillation_symbol() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).cantillation_symbol(),
                "// TODO"
            );
        }
        #[test]
        fn test_pseudo_accent_cantillation_symbol() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).cantillation_symbol(),
                "// TODO"
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).cantillation_symbol(),
                "// TODO"
            );
        }
    }

    mod alternate_names {
        // TODO alternate names
        use crate::{
            Accent, AlternateNames, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent,
        };
        #[test]
        fn test_prose_disjunctive_alternate_names() {
            // Primary disjunctives: indices 0-17, strengths 1-18
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Silluq).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Atnach).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Segolta).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Shalshelet).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephQatan).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::ZaqephGadol).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Revia).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tiphcha).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Zarqa).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pashta).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Yetiv).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Tevir).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Geresh).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "טֶרֶס",
                    hebrew_concept: "a boundary or border",
                    english_name: "Teres",
                    sbl_academic: "ṭeres",
                })
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Gershayim).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Pazer).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::PazerGadol).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "קַרְנֵי פָרָה",
                    hebrew_concept: "horns of a cow",
                    english_name: "Qarne Pharah",
                    sbl_academic: "qarnê pārâ",
                })
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaGedolah).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Legarmeh).alternate_names(),
                None
            );
        }

        /// Test all prose conjunctive accents (should return None)
        #[test]
        fn test_prose_conjunctive_alternate_names() {
            // Conjunctives: indices 18-27, all should be None
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Munach).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Mahpakh).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Merkha).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::MerkhaKephulah).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Darga).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Azla).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::TelishaQetannah).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Galgal).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meayla).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Prose(ProseAccent::Meteg).alternate_names(),
                None
            );
        }
        #[test]
        fn test_poetry_disjunctive_alternate_names() {
            // Primary disjunctives: indices 0-11, strengths 1-12
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Silluq).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::OlehWeYored).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Atnach).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaGadol).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaMugrash).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tsinnor).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ReviaQaton).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Dechi).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Pazer).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh).alternate_names(),
                None
            );
        }

        #[test]
        fn test_poetry_conjunctive_alternate_names() {
            // Conjunctives: indices 12-22, all should be None
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Munach).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Merkha).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Illuy).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Tarcha).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Galgal).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Mehuppakh).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Azla).alternate_names(),
                Some(AlternateNames {
                    hebrew_name: "",
                    hebrew_concept: "",
                    sbl_academic: "",
                    english_name: "",
                })
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Poetry(PoetryAccent::Meteg).alternate_names(),
                None
            );
        }
        #[test]
        fn test_pseudo_accent_alternate_names() {
            // Pseudo accents never have strength hierarchy
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::SophPasuq).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Maqqaph).alternate_names(),
                None
            );
            assert_eq!(
                HebrewAccent::Pseudo(PseudoAccent::Paseq).alternate_names(),
                None
            );
        }
    }
}
