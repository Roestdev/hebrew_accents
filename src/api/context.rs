/// Represents the liturgical context for Hebrew biblical text accentuation.
///
/// # Overview
///
/// The Hebrew Bible (Tanakh) employs **two distinct cantillation systems** depending
/// on whether the text is prose or poetry. This enum classifies sentences into one
/// of these two categories, which determines:
///
/// - Which set of accent rules applies
/// - How disjunctive/conjunctive hierarchies are interpreted
/// - Which melodic chants are appropriate for reading
///
/// # Accent System Differences
///
/// | Feature | [`Prosaic`](Context::Prosaic) | [`Poetic`](Context::Poetic) |
/// |---------|-------------------------------|----------------------------|
/// | Books | Torah, Prophets, most Writings | Psalms, Proverbs, Job |
/// | Exclusive Accents | Segolta, Zaqeph Qatan/Gadol, Pashta, ShenePastin, Tevir, Yetiv, Gershayim, Pazer Gadol, Telisha Gedolah/Qetannah, Merkha Kephulah, Darga | Oleh WeYored, Dechi, Illuy, Tsinnorit Merkha/Mahpakh |
/// | Shared Accents | Silluq, Atnach, Munach, Mahpakh, etc. | Same as prose (shared accents don't determine context) |
/// | Default Status | ✅ Used when context is unknown | ❌ Must be explicitly specified |
///
/// # Design Rationale
///
/// **Why is `Prosaic` the default?**
///
/// 1. Most of the Tanakh (21 of 24 books) uses prose accentuation
/// 2. Prose is the baseline/narrative mode; poetry is exceptional
/// 3. When context detection is inconclusive, prose is the safer assumption
///
/// # Trait Derivations
///
/// - **`Copy`**: Can be passed by value without cloning overhead
/// - **`Clone`**: Allows explicit duplication (trivial for `Copy` types)
/// - **`Debug`**: Human-readable display in logging and debug output
/// - **`Default`**: `Prosaic` is the default variant (marked with `#[default]`)
/// - **`Eq` / `PartialEq`**: Structural equality for comparisons
/// - **`Hash`**: Usable as keys in `HashMap` / `HashSet`
///
/// Note: `Ord`/`PartialOrd` are intentionally not derived. Use pattern matching
/// or boolean helper methods (`is_poetic()`, `is_prosaic()`) for conditional logic.
///
/// # Usage Patterns
///
/// ## Creating with Explicit Context
///
/// ```rust
/// use hebrew_accents::Context;
///
/// // Poetry (e.g., Psalm text)
/// let poetry = Context::Poetic;
/// assert_eq!(poetry, Context::Poetic);
///
/// // Prose (e.g., Genesis narrative)
/// let prose = Context::Prosaic;
/// assert_eq!(prose, Context::Prosaic);
///
/// // Default is prose
/// let default: Context = Default::default();
/// assert_eq!(default, Context::Prosaic);
/// ```
///
/// ## Pattern Matching
///
/// ```rust
/// use hebrew_accents::Context;
///
/// fn describe(ctx: Context) {
///     match ctx {
///         Context::Poetic => println!("Applying poetic accent rules"),
///         Context::Prosaic => println!("Applying prose accent rules"),
///     }
/// }
/// ```
///
/// ## Convenience Helpers
///
/// ```rust
/// use hebrew_accents::Context;
///
/// // Boolean checks
/// assert!(Context::Poetic.is_poetic());
/// assert!(Context::Prosaic.is_prosaic());
///
/// // String representation
/// assert_eq!(Context::Poetic.as_str(), "Poetic");
/// assert_eq!(Context::Prosaic.as_str(), "Prosaic");
/// ```
///
/// ## Equality Checks
///
/// ```rust
/// use hebrew_accents::Context;
///
/// // Use PartialEq for comparisons
/// assert_ne!(Context::Poetic, Context::Prosaic);
/// assert_eq!(Context::Prosaic, Context::Prosaic);
/// ```
///
/// ## Auto-Detection Integration
///
/// When working with [`SentenceContext`](crate::SentenceContext), you can attempt
/// to detect the context automatically:
///
/// ```rust
/// use hebrew_accents::{SentenceContext, Context};
///
/// let result = SentenceContext::with_valid_default();
/// println!("result: {:?}", result);
/// if let Ok(sentence) = result {
///     println!("Context: {:?}", sentence.context());
/// }
/// ```
///
/// # See Also
///
/// - [`try_derive_context`](crate::SentenceContext::try_derive_context) — Detect context from accent patterns
/// - [`SentenceContext`](crate::SentenceContext) — Container for text + context
///
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Context {
    /// The sentence follows **prosaic** (ordinary prose) conventions.
    ///
    /// # Characteristics
    ///
    /// Prose comprises the majority of biblical narrative and discourse:
    ///
    /// | Feature | Description |
    /// |---------|-------------|
    /// | Narrative Flow | Sequential storytelling without strict parallelism |
    /// | Grammar | Standard prose syntax and morphology |
    /// | Accents | Uses prose-exclusive cantillation marks |
    /// | Prevalence | 21 books, ~80% of the Tanakh |
    ///
    /// # Prose-Exclusive Accents
    ///
    /// Presence of any of these strongly indicates prose context:
    ///
    /// - **Segolta** (סְגוֹלְתָּא) — Triple-vowel cluster accent
    /// - **Zaqeph Qatan / Zaqeph Gadol** (זָקֵף קָטֹן / גָּדוֹל) — Lesser/Greater uplift
    /// - **Pashta** (פַּשְׁטָא) — Extension accent
    /// - **Tevir** (תְּבִיר) — Break/Crack accent
    /// - **Yetiv** (יְתִיב) — Sitting/Resting accent
    /// - **Gershayim** (גֵּרְשַׁיִם) — Double-punctuation marker
    /// - **Pazer Gadol** (פָּזֵר גָּדוֹל) — Large scatter accent
    /// - **Telisha Gedolah / Qetannah** (טְלִישָׁה גְּדוֹלָה / קְטַנָּה) — Long/Short curl
    /// - **Merkha Kephulah** (מֵרכָּא כְּפוּלָּה) — Doubled connector
    /// - **Darga** (דַּרְגָּא) — Step/Grade accent
    ///
    /// # Default Behavior
    ///
    /// `Prosaic` is marked with `#[default]`, making it the automatic choice when:
    ///
    /// 1. No context is specified in constructors
    /// 2. Context detection fails (insufficient distinctive accents)
    /// 3. `Default::default()` is invoked
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// // Three equivalent ways to get the default
    /// let ctx1: Context = Default::default();
    /// let ctx2 = Context::default();
    /// let ctx3 = Context::Prosaic;
    ///
    /// assert_eq!(ctx1, ctx2);
    /// assert_eq!(ctx2, ctx3);
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context};
    ///
    /// // Genesis narrative (prose)
    /// let genesis_result = SentenceContext::new(
    ///     "וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר",
    ///     Context::Prosaic,
    /// );
    /// if let Ok(genesis) = genesis_result {
    ///     assert_eq!(genesis.context(), Context::Prosaic);
    ///     assert!(genesis.context().is_prosaic());
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Poetic`](Context::Poetic) — The alternative poetic context
    /// - [`SentenceContext::with_valid_default()`](crate::SentenceContext::with_valid_default) — Uses `Prosaic` implicitly
    #[default]
    Prosaic,

    /// The sentence follows **poetic** biblical structure.
    ///
    /// # Characteristics
    ///
    /// Poetry in the Hebrew Bible exhibits several distinguishing features:
    ///
    /// | Feature | Description |
    /// |---------|-------------|
    /// | Parallelism | Thought-based parallel lines (synonymous, antithetic, synthetic) |
    /// | Meter | Regular syllabic patterns (though debated among scholars) |
    /// | Vocabulary | Unique poetic words and rare forms |
    /// | Accents | Uses poetry-exclusive cantillation marks |
    ///
    /// # Poetry-Exclusive Accents
    ///
    /// Presence of any of these strongly indicates poetic context:
    ///
    /// - **Oleh WeYored** (עוֹלֶה וְיוֹרֵד) — "Ascending and Descending"
    /// - **Dechi** (דְּחִי) — Pushed/Accented variant
    /// - **Illuy** (עִלּוּי) — Elevated accent
    /// - **Tsinnorit** (צִנּוֹרִית) — Stream/Channel variant (Merkha or Mahpakh forms)
    ///
    /// # Biblical Books Using Poetry
    ///
    /// - **Psalms** (תְּהִלִּים) — Entirely poetic
    /// - **Proverbs** (מִשְׁלֵי) — Primarily poetic
    /// - **Job** (אִיּוֹב) — Dominantly poetic
    ///
    /// *Note*: Poetry also appears within prose books (e.g., Song of Moses in Exodus 15,
    /// Hannah's Song in 1 Samuel 2, Deborah's Song in Judges 5).
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// let psalm = Context::Poetic;
    /// assert!(psalm.is_poetic());
    /// assert_eq!(psalm.as_str(), "Poetic");
    /// ```
    Poetic,
}

impl Context {
    /// Returns `true` if this is the [`Poetic`](Context::Poetic) context.
    ///
    /// Convenience method for conditional logic without pattern matching.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// assert!(Context::Poetic.is_poetic());
    /// assert!(!Context::Prosaic.is_poetic());
    /// ```
    #[inline]
    pub fn is_poetic(&self) -> bool {
        matches!(self, Context::Poetic)
    }

    /// Returns `true` if this is the [`Prosaic`](Context::Prosaic) context.
    ///
    /// Convenience method for conditional logic without pattern matching.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// assert!(Context::Prosaic.is_prosaic());
    /// assert!(!Context::Poetic.is_prosaic());
    /// ```
    #[inline]
    pub fn is_prosaic(&self) -> bool {
        matches!(self, Context::Prosaic)
    }

    /// Returns the canonical name for this context as a `&'static str`.
    ///
    /// Note: Returns capitalized values ("Poetic", "Prosaic").
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// assert_eq!(Context::Poetic.as_str(), "Poetic");
    /// assert_eq!(Context::Prosaic.as_str(), "Prosaic");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Context::Poetic => "Poetic",
            Context::Prosaic => "Prosaic",
        }
    }
}

// TEMPLATE +++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod derive_traits_tests {
    // TEMPLATE
    use super::*;
    use std::{
        collections::{HashMap, HashSet},
        hash::{DefaultHasher, Hash, Hasher},
    };
    //#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]

    // DEFAULT
    #[test]
    fn default_is_active() {
        assert_eq!(Context::default(), Context::Prosaic);
    }

    #[test]
    fn default_trait_object_usage() {
        // Calls <Context as Default>::default through the trait, not just Context::default()
        fn takes_default<T: Default>() -> T {
            T::default()
        }
        assert_eq!(takes_default::<Context>(), Context::Prosaic);
    }

    // Debug trait
    #[test]
    fn debug_output_contains_variant() {
        let dbg = format!("{:?}", Context::Prosaic);
        assert!(dbg.contains("Prosaic"), "unexpected Debug output: {dbg}");

        let dbg = format!("{:?}", Context::Poetic);
        assert!(dbg.contains("Poetic"), "unexpected Debug output: {dbg}");
    }
    #[test]
    fn debug_via_formatter() {
        // Exercises Debug::fmt through the trait, including with no explicit hit on both arms
        for (ctx, expected) in [(Context::Prosaic, "Prosaic"), (Context::Poetic, "Poetic")] {
            assert_eq!(format!("{ctx:?}"), expected);
        }
    }
    // Clone/Copy trait
    #[test]
    fn clone_and_copy_semantics() {
        let original = Context::Poetic;
        let copied = original; // Copy: moved AND original still valid
        let cloned = original.clone();
        assert_eq!(copied, original);
        assert_eq!(cloned, original);
    }

    // PartialEq / Eq
    #[test]
    fn partial_eq_works() {
        assert_eq!(Context::Prosaic, Context::Prosaic);
        assert_ne!(Context::Prosaic, Context::Poetic);
    }
    // Eq
    #[test]
    fn implements_eq() {
        fn assert_eq_trait<T: Eq>(_: &T) {}
        assert_eq_trait(&Context::Poetic);
        assert_eq_trait(&Context::Prosaic);
    }
    //Hash
    #[test]
    fn hash_matches_equality() {
        let hash = |e: Context| {
            let mut h = DefaultHasher::new();
            e.hash(&mut h);
            h.finish()
        };
        assert_eq!(hash(Context::Poetic), hash(Context::Poetic));
        assert_ne!(hash(Context::Poetic), hash(Context::Prosaic));
    }
    #[test]
    fn usable_in_hashset() {
        let mut set = HashSet::new();
        set.insert(Context::Prosaic);
        set.insert(Context::Prosaic); // duplicate ignored
        assert_eq!(set.len(), 1);
        assert!(set.contains(&Context::Prosaic));
    }
    #[test]
    fn hashmap_key_roundtrip() {
        // Exercises Hash + Eq together in a real container
        let mut map = HashMap::new();
        map.insert(Context::Poetic, "poetic");
        map.insert(Context::Poetic, "poetic"); // duplicate ignored
        map.insert(Context::Prosaic, "prosaic");
        assert_eq!(map.get(&Context::Poetic), Some(&"poetic"));
        assert_eq!(map.get(&Context::Prosaic), Some(&"prosaic"));
        assert_eq!(map.len(), 2);
    }
}

#[cfg(test)]
mod tests_impl_context {
    use super::*;
    #[test]
    fn test_is_poetic() {
        let context = Context::Poetic;
        assert!(context.is_poetic());
        assert!(!context.is_prosaic());
    }
    #[test]
    fn test_is_prosaic() {
        let context = Context::Prosaic;
        assert!(!context.is_poetic());
        assert!(context.is_prosaic());
    }
    #[test]
    fn test_as_str() {
        let prosaic = Context::Prosaic;
        let poetic = Context::Poetic;
        assert_eq!(prosaic.as_str(), "Prosaic");
        assert_eq!(poetic.as_str(), "Poetic");
    }
    #[test]
    fn as_str_exhaustive_and_consistent_with_debug() {
        for ctx in [Context::Prosaic, Context::Poetic] {
            assert_eq!(ctx.as_str(), format!("{ctx:?}"));
        }
    }
}
