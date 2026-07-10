use crate::accent::{PoetryAccent, ProseAccent, PseudoAccent};

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
    pub fn as_prose(&self) -> Option<&ProseAccent> {
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
    ///     println!("English name: {}", poetry.english_name());
    ///     println!("Relative strength: {}", poetry.relative_strength());
    /// }
    ///
    /// // Original accent remains usable after inspection
    /// assert!(accent.as_poetry().is_some());
    /// ```
    pub fn as_poetry(&self) -> Option<&PoetryAccent> {
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
    /// // Check the accent type exists before accessing its data
    /// if let Some(pseudo) = accent.as_pseudo() {
    ///     println!("English name: {}", pseudo.english_name());      // "Maqqeph"
    ///     println!("Concept: {}", pseudo.hebrew_concept());         // "binder"
    /// }
    ///
    /// // The original accent remains usable after inspection
    /// assert!(matches!(accent.as_pseudo(), Some(PseudoAccent::Maqqeph)));
    /// ```
    pub fn as_pseudo(&self) -> Option<&PseudoAccent> {
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
    /// Type inference works when the target type is clear from context:
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
/// Automatic type coercion in function arguments:
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
