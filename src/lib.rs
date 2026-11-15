//! # Hebrew Accents
//!
//! A Rust library for working with the *Masoretic Hebrew cantillation CantillationSymbol*
//!
//! ## Design
//!
//! ### Overview
//!
//! This crate offers two complementary perspectives:
//!
//! 1. **Text inspection**
//!
//!    Tools for parsing and analysing Hebrew texts,
//!    useful for research, tooling, or building downstream applications
//!
//! 2. **Accent metadata**
//!
//!    Access detailed information about each individual
//!    cantillation mark (name, Unicode code point(s), function, etc.)
//!
//! By separating the static accent data from the dynamic text‑processing
//! capabilities, the library remains flexible and easy to extend
//!
//! ### Crate layout
//!
//! The crate is organized into four major modules, plus some supporting
//! files such as `error.rs` and `lib.rs`:
//!
//!  1. **api** — the public API surface: the `Accent` trait, the accent
//!     enums (`ProseAccent`, `PoetryAccent`, `PseudoAccent`, `HebrewAccent`),
//!     and the metadata models they expose.
//!
//!  2. **accent** — implementation behind the API: static accent data
//!     (lookup tables, symbols, display helpers).
//!
//!  3. **mark** — the cantillation mark itself: Unicode code points,
//!     placement, and stress-position data.
//!
//!  4. **sentence** — text inspection: parsing sentences, validating
//!     accentuation, and detecting prose vs. poetry context.
//!
//! ### Main accent types
//!
//! The whole crate is built around one central type: **`HebrewAccent`**.
//! It is a thin wrapper that unifies the three accent families behind a
//! single type, so that any API accepting an accent can work with all of
//! them interchangeably:
//!
//! - *`ProseAccent`* — the accents of the prosaic system, used in most
//!   biblical books (e.g. Genesis, Kings)
//! - *`PoetryAccent`* — the accents of the poetic system, used in the
//!   poetic books (Psalms, Proverbs, Job)
//! - *`PseudoAccent`* — accent-related marks that are not cantillation
//!   accents proper (e.g. Maqqaph, Paseq)
//!
//! ```none
//! Illustrative definition of `HebrewAccent`:
//!
//! pub enum HebrewAccent {
//!     Prose(ProseAccent),
//!     Poetry(PoetryAccent),
//!     Pseudo(PseudoAccent),
//! }
//! ```
//!
//! The wrapper exists because the prosaic and poetic systems are distinct
//! sets with their own enums — several accents share a name across systems
//! (e.g. Atnach) but are treated as separate types here. See
//! [`Accent`]for the metadata available on every variant.
//!
//! ```none
//! Definition of ProseAccent (partly)
//!
//! pub enum ProseAccent {
//!     Silluq,
//!     Atnach,
//!     Segolta,
//!     Shalshelet,
//!     ...
//!     ...
//! }
//! ```
//! **Both** the *`From`* and *`Into`* traits are implemented for the `HebrewAccent`
//!
//! ### Text inspection
//!
//! It is not possible to determine the context based on the sentence alone,
//! due to the complexity of the Hebrew accent systems!
//!
//! Meaning that we need the context of the sentence in advance.
//!
//! For this reason there is a new accenttype called `SentenceContext`, which is a struct  that contains both the sentence and the corresponding context.
//!
//! ```none
//! Definition of `SentenceContext`:
//!
//! pub struct SentenceContext {
//!    pub sentence: String,
//!    pub ctx: Context,
//! }
//!
//! Definition of `Context`:
//!
//! pub enum Context {
//!     Poetic,
//!     Prose,
//! }
//! ```
//! The following functions are available for `SentenceContext`:
//!
//! - new() -> `SentenceContext`
//! - contains_accent(HebrewAccent) -> bool
//! - find_accent(HebrewAccent) -> `Option<Match>`
//!
//!
//! ### Accent metadata
//!
//! Accent metadata is exposed via the `Accent` trait. Currently,
//! the trait provides access to the following pieces of metadata:
//!
//!  - hebrew_name          (the Hebrew name)
//!  - meaning              (the meaning of the Hebrew name)
//!  - english_name         (the English name, a transliteration of the Hebrew)
//!  - Number of UTF-8 code_points    (1 or 2)
//!  - rank    (lowest number has relative more weight)
//!  - count                (the number of all accents in a specific category)
//!
//!  Optional attributes:
//!
//!  - accent type          (Primary, Secondary)
//!  - category             (Disjunctive, Conjunctive)
//!  - stress_position      (Impositive, Postpositive, Prepositive)
//!
//!  For debugging purposes:
//!
//!  - details    
//!
//! ## Examples
//!
//! ### Text inspection
//!
//! A [`SentenceContext`] couples a sentence with its liturgical context.
//! Construction validates the text, so it returns a [`SentenceContextError`]
//! for empty or malformed input:
//!
//! ```
//! use hebrew_accents::{Context, SentenceContext};
//!
//! // Genesis 1:1 (prose)
//! let sentence = SentenceContext::new(
//!     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
//!     Context::Prosaic,
//! )
//! .unwrap();
//!
//! assert_eq!(sentence.context(), Context::Prosaic);
//! assert_eq!(sentence.as_str(), "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים");
//!
//! // Construction is validated: empty input is rejected
//! assert!(SentenceContext::new("", Context::Prosaic).is_err());
//!
//! // A known-good default sentence (Genesis 1:1, complete verse):
//! let default = SentenceContext::with_valid_default().unwrap();
//! assert_eq!(default.context(), Context::Prosaic);
//! ```
//!
//! [`SentenceContext`]: crate::SentenceContext
//! [`SentenceContextError`]: crate::SentenceContextError
//!
//! ### Detecting the context
//!
//! Most of the Tanakh is prose, but Psalms, Proverbs and Job use a
//! different accent system. Detection works on the *exclusive* accents
//! found in the text:
//!
//! ```
//! use hebrew_accents::{try_derive_context, Context};
//!
//! // Psalm 1:1 — poetry-exclusive accents (Oleh WeYored, Dechi, ...)
//! let psalm = "מִזְמ֥וֹר לְדָוִ֑ד יְהוָ֥ה רֹ֝עִ֗י לֹ֣א אֶחְסָֽר׃";
//!
//! match try_derive_context(psalm) {
//!     Ok(context) => assert_eq!(context, Context::Poetic),
//!     Err(e) => panic!("Detection failed: {e}"),
//! }
//!
//! // The same detection is available as an instance method:
//! use hebrew_accents::SentenceContext;
//!
//! let sentence = SentenceContext::new(psalm, Context::Prosaic).unwrap();
//! assert_eq!(sentence.try_derive_context().unwrap(), Context::Poetic);
//! ```
//!
//! ### Finding accents in a sentence
//!
//! ```
//! use hebrew_accents::{HebrewAccent, ProseAccent, SentenceContext, Context};
//!
//! let sentence = SentenceContext::new(
//!     "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים",
//!     Context::Prosaic,
//! )
//! .unwrap();
//!
//! let atnach = HebrewAccent::Prose(ProseAccent::Atnach);
//!
//! if sentence.contains_accent(atnach) {
//!     if let Some(found) = sentence.find_accent(atnach) {
//!         println!("Found at byte range {:?}", found.range());
//!     }
//! }
//! ```
//!
//! ### Accent metadata
//!
//! The [`Accent`] trait exposes metadata for every cantillation mark.
//! `From`/`Into` conversions let you use the concrete enums directly:
//!
//! ```
//! use hebrew_accents::{Accent, AccentCategory, AccentKind, ProseAccent, Tier};
//!
//! let atnach = ProseAccent::Atnach;
//!
//! // Names
//! assert_eq!(atnach.english_name(), "Atnach");
//! assert_eq!(atnach.hebrew_concept(), "a causing to rest");
//!
//! // Structural metadata
//! assert_eq!(atnach.kind(), Some(AccentKind::Primary));
//! assert_eq!(atnach.category(), Some(AccentCategory::Disjunctive));
//! assert_eq!(atnach.tier(), Some(Tier::Tier1));
//!
//! // BHS Tabula Accentum: 1 is the strongest pause
//! assert_eq!(atnach.rank(), Some(2));
//! assert!(ProseAccent::Silluq.rank() < atnach.rank()); // Silluq is stronger
//! ```
//!
//! [`Accent`]: crate::Accent
//!
//! ### Rendering a cantillation example
//!
//! [`cantillation_example`] shows where a mark sits on a dummy word,
//! taking the accent's stress position into account:
//!
//! ```
//! use hebrew_accents::{cantillation_example, ProseAccent};
//!
//! // Into<HebrewAccent> is implemented for every accent type
//! let example = cantillation_example(ProseAccent::Silluq.into());
//! assert!(!example.is_empty());
//! ```
//!
//! ### Text inspection
//!
//! ### Accent metadata
//!
//! *Note: For more information see [DESIGN](DESIGN.md)*

#![deny(missing_docs, unused_imports)]
// import doc tests from README.md
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
/// #![doc = include_str!("../README.md")]
mod accent;
/// Public API for accent types, including the `Accent` trait, accent enums, and metadata models
mod api;
mod error;
mod mark;
mod sentence;
mod version;

// Re-exports
pub use crate::api::Accent;
pub use crate::api::AccentCategory;
pub use crate::api::AccentKind;
pub use crate::api::AlternateNames;
pub use crate::api::CantillationMark;
pub use crate::api::CantillationMarkPlacement;
pub use crate::api::CantillationMarkStressPosition;
pub use crate::api::CompoundType;
pub use crate::api::Context;
pub use crate::api::HebrewAccent;
pub use crate::api::Match;
pub use crate::api::PoetryAccent;
pub use crate::api::ProseAccent;
pub use crate::api::PseudoAccent;
pub use crate::api::SentenceContext;
pub use crate::api::Tier;

// public functions
pub(crate) use crate::accent::display_cantillation_symbol;
pub use crate::api::cantillation_example;
pub use crate::api::try_derive_context; // TODO remove

// errorhandling
pub use crate::error::{SentenceContextError, TierError};

// Re-export unicode info for convenient access
pub use version::{
    unicode_version, unicode_version_info, UnicodeVersionInfo, UNICODE_REFERENCE, UNICODE_VERSION,
};

/// This module contains additional documentation resources.
pub mod docs {
    /// Transliteration Rules
    ///
    /// Full documentation for how accents are transliterated.
    #[doc = include_str!("../TRANSLITERATION.md")]
    pub mod transliteration {}
}
