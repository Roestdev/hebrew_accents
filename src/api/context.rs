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
/// - **`Ord` / `PartialOrd`**: Total ordering (`Poetic < Prosaic`) for sorted collections
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
/// ## Comparison and Ordering
///
/// ```rust
/// use hebrew_accents::Context;
///
/// assert!(Context::Poetic < Context::Prosaic); // Enum discriminant order
///
/// use std::collections::BTreeSet;
/// let mut contexts = BTreeSet::new();
/// contexts.insert(Context::Prosaic);
/// contexts.insert(Context::Poetic);
/// // Iteration yields: Poetic, then Prosaic
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
/// let sentence = SentenceContext::new(
///     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
///     Context::Prosaic, // Placeholder; will be overridden by detection
/// )?;
///
/// match sentence.try_determine_context() {
///     Ok(detected) => println!("Detected: {}", detected.as_str()),
///     Err(e) => eprintln!("Cannot determine: {} (using default)", e),
/// }
/// ```
///
/// # See Also
///
/// - [`try_determine_context`](crate::SentenceContext::try_determine_context) — Detect context from accent patterns
/// - [`SentenceContext`](crate::SentenceContext) — Container for text + context
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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
    /// | Prevalence | 18+ books, ~80% of the Tanakh |
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
    // Three equivalent ways to get the default
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
    /// let genesis = SentenceContext::new(
    ///     "וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר",
    ///     Context::Prosaic,
    /// )?;
    ///
    /// assert_eq!(genesis.context(), Context::Prosaic);
    /// assert!(genesis.context().is_prosaic());
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

    /// Returns the canonical lowercase name for this context as a `&'static str`.
    ///
    /// Useful for display, serialization keys, and logging.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hebrew_accents::Context;
    ///
    /// assert_eq!(Context::Poetic.as_str(), "Poetic");
    /// assert_eq!(Context::Prosaic.as_str(), "Prosaic");
    ///
    /// // Useful in format strings
    /// let ctx = Context::default();
    /// println!("Processing as: {}", ctx.as_str());  // "Processing as: Prosaic"
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            Context::Poetic => "Poetic",
            Context::Prosaic => "Prosaic",
        }
    }
}
