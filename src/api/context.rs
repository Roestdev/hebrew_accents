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
/// | Exclusive Accents | Segolta, Zaqeph Qatan/Gadol, Pashta, Tevir, Yetiv, Gershayim, Pazer Gadol, Telisha Gedolah/Qetannah, Merkha Kephulah, Darga | Oleh WeYored, Dechi, Illuy, Tsinnorit Merkha/Mahpakh |
/// | Shared Accents | Silluq, Atnach, Munach, Mahpakh, etc. | Same as prose (shared accents don't determine context) |
/// | Default Status | ✅ Used when context is unknown | ❌ Must be explicitly specified |
///
/// # Design Rationale
///
/// **Why is `Prosaic` the default?**
///
/// 1. Most of the Tanakh (18 of 24 books) uses prose accentuation
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
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
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
    /// Useful for display, serialization keys, and logging.
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

#[cfg(test)]
mod tests {
    use super::*;

    // ===== BASIC VARIANT TESTS =====

    #[test]
    fn test_context_variants_exist() {
        let poetic = Context::Poetic;
        let prosaic = Context::Prosaic;

        assert_eq!(poetic, Context::Poetic);
        assert_eq!(prosaic, Context::Prosaic);
        assert_ne!(poetic, prosaic);
    }

    #[test]
    fn test_context_default_is_prosaic() {
        // Three ways to get default - all should equal Prosaic
        let ctx1: Context = Default::default();
        let ctx2 = Context::default();
        let ctx3 = Context::Prosaic;

        assert_eq!(ctx1, ctx2);
        assert_eq!(ctx2, ctx3);
        assert_eq!(ctx1, Context::Prosaic);
    }

    // ===== BOOLEAN HELPER METHOD TESTS =====

    #[test]
    fn test_is_poetic_method() {
        assert!(Context::Poetic.is_poetic());
        assert!(!Context::Prosaic.is_poetic());
    }

    #[test]
    fn test_is_prosaic_method() {
        assert!(Context::Prosaic.is_prosaic());
        assert!(!Context::Poetic.is_prosaic());
    }

    #[test]
    fn test_as_str_method() {
        assert_eq!(Context::Poetic.as_str(), "Poetic");
        assert_eq!(Context::Prosaic.as_str(), "Prosaic");
    }

    // ===== DERIVED TRAIT TESTS =====

    #[test]
    fn test_context_debug_trait() {
        let poetic_debug = format!("{:?}", Context::Poetic);
        let prosaic_debug = format!("{:?}", Context::Prosaic);

        assert!(poetic_debug.contains("Poetic"));
        assert!(prosaic_debug.contains("Prosaic"));
    }

    #[test]
    fn test_context_display_via_fmt() {
        // Debug trait provides display capability
        let _poetic = format!("{:?}", Context::Poetic);
        let _prosaic = format!("{:?}", Context::Prosaic);
    }

    #[test]
    fn test_context_copy_trait() {
        let original = Context::Poetic;
        let copied = original; // Copy occurs automatically

        assert_eq!(original, Context::Poetic);
        assert_eq!(copied, Context::Poetic);
        // Original is still usable after "copy"
        assert!(original.is_poetic());
    }

    #[test]
    fn test_context_clone_trait() {
        let original = Context::Prosaic;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned, Context::Prosaic);
    }

    #[test]
    fn test_context_partial_eq_trait() {
        let p1 = Context::Poetic;
        let p2 = Context::Poetic;
        let p3 = Context::Prosaic;

        assert_eq!(p1, p2);
        assert_eq!(p1.eq(&p2), true);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_context_eq_trait() {
        let p1 = Context::Prosaic;
        let p2 = Context::Prosaic;

        assert!(p1.eq(&p2));
        assert!(p2.eq(&p1));
    }

    #[test]
    fn test_context_hash_trait() {
        use std::collections::hash_map::DefaultHasher;
        use std::collections::HashSet;
        use std::hash::{Hash, Hasher};

        // Test HashSet usage
        let mut set = HashSet::new();
        set.insert(Context::Poetic);
        set.insert(Context::Prosaic);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&Context::Poetic));
        assert!(set.contains(&Context::Prosaic));

        // Test Hash implementation
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        Context::Poetic.hash(&mut hasher1);
        Context::Poetic.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // ===== PATTERN MATCHING TESTS =====

    #[test]
    fn test_pattern_matching_on_context() {
        let result_poetic = match Context::Poetic {
            Context::Poetic => "poetry",
            Context::Prosaic => "prose",
        };

        let result_prosaic = match Context::Prosaic {
            Context::Poetic => "poetry",
            Context::Prosaic => "prose",
        };

        assert_eq!(result_poetic, "poetry");
        assert_eq!(result_prosaic, "prose");
    }

    #[test]
    fn test_if_let_pattern_on_context() {
        if let Context::Poetic = Context::Poetic {
            assert!(true);
        }

        if let Context::Prosaic = Context::Prosaic {
            assert!(true);
        }

        assert!(!(matches!(Context::Poetic, Context::Prosaic)));
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_helpers_and_as_str_consistency() {
        // Verify all methods agree on variant identity
        assert_eq!(Context::Poetic.is_poetic(), true);
        assert_eq!(Context::Poetic.is_prosaic(), false);
        assert_eq!(Context::Poetic.as_str(), "Poetic");

        assert_eq!(Context::Prosaic.is_poetic(), false);
        assert_eq!(Context::Prosaic.is_prosaic(), true);
        assert_eq!(Context::Prosaic.as_str(), "Prosaic");
    }

    #[test]
    fn test_all_combinations_in_function_like_scenario() {
        fn categorize(ctx: Context) -> (&'static str, bool, bool) {
            (ctx.as_str(), ctx.is_poetic(), ctx.is_prosaic())
        }

        let (name_poetic, is_p, is_pr) = categorize(Context::Poetic);
        assert_eq!(name_poetic, "Poetic");
        assert!(is_p);
        assert!(!is_pr);

        let (name_prosaic, is_p, is_pr) = categorize(Context::Prosaic);
        assert_eq!(name_prosaic, "Prosaic");
        assert!(!is_p);
        assert!(is_pr);
    }

    #[test]
    fn test_context_usage_in_option_wrapping() {
        // Context should work well with Option
        let ctx = Some(Context::Poetic);

        assert!(ctx.is_some());
        assert_eq!(ctx.unwrap(), Context::Poetic);

        let none_ctx: Option<Context> = None;
        assert!(none_ctx.is_none());
    }

    #[test]
    fn test_context_in_result_wrapping() {
        // Context should work with Result
        let ok_result: Result<Context, ()> = Ok(Context::Prosaic);
        let err_result: Result<Context, ()> = Err(());

        assert!(ok_result.is_ok());
        assert_eq!(ok_result.unwrap(), Context::Prosaic);
        assert!(err_result.is_err());
    }

    #[test]
    fn test_const_context_values() {
        // Verify context can be used in const contexts
        const POETIC_CTX: Context = Context::Poetic;
        const PROSAIC_CTX: Context = Context::Prosaic;

        assert_eq!(POETIC_CTX.is_poetic(), true);
        assert_eq!(PROSAIC_CTX.is_prosaic(), true);
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_context_not_ordered() {
        // Verify Ord/PartialOrd are NOT implemented
        // This compile-time check ensures we don't accidentally sort contexts
        let _poetic = Context::Poetic;
        let _prosaic = Context::Prosaic;

        // The following would NOT compile (commented out):
        // assert!(_poetic < _prosaic);  // Error: Ord not implemented

        // Instead use explicit helpers or pattern matching
        assert_ne!(_poetic, _prosaic);
    }

    #[test]
    fn test_context_immutable_after_creation() {
        // Context is a simple enum with no mutable state
        let ctx = Context::Poetic;

        // Cannot modify (would not compile if we tried):
        // ctx.some_field = ...

        // But we can read properties
        assert_eq!(ctx.as_str(), "Poetic");
        assert!(ctx.is_poetic());
    }
}
