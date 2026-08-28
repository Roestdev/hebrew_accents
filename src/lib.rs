//! # Hebrew Accents
//!
//! A Rust library for working with the *Masoretic Hebrew cantillation CantillationSymbol*
//!
//! ## Design
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
//! All `accent` related code can be found in the files starting with *`accent`*
//! All `text inspection` related code can be found in files starting with *`sentence_ctx`*
//!
//! ### Main accenttypes
//!
//! The whole crate is build around the main accenttype **`HebrewAccent`**,
//!  with its three subaccenttypes:
//!  - *`ProseAccent`* (all prose accents)
//!  - *`PoetryAccent`* (all poetry accents)
//!  - *`PseudoAccent`* (actually non-accents, but accent related)
//!
//! ```none
//! Definition of HebrewAccent:
//!
//! pub enum HebrewAccent {
//!     Prose(ProseAccent),
//!     Poetry(PoetryAccent),
//!     Pseudo(PseudoAccent),
//! }
//! ```
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
//!  - relative_strength    (lowest number has relative more weight)
//!  - count                (the number of all accents in a specific category)
//!
//!  Optional attributes:
//!
//!  - accent type          (Primary, Secondary)
//!  - category             (Disjunctive, Conjunctive)
//!  - stress_position          (Impositive, Postpositive, Prepositive)
//!
//!  For debugging purposes:
//!
//!  - details    
//!
//! ## Examples
//! todo
//!
//! ### Text inspection
//!
//! ### Accent metadata
//!
//! *Note: For more information see [DESIGN](DESIGN.md)*

#![deny(missing_docs, unused_imports)]
// import doc tests from README.md
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
#![doc = include_str!("../README.md")]

mod accent;
mod accent_data;
mod accent_mark;
/// Public API for accent types, including the `Accent` trait, accent enums, and metadata models
mod api;
mod error;
mod formatting;
mod sentence;

// Re-exports
pub use crate::api::Accent;
pub use crate::api::AccentCategory;
pub use crate::api::AccentKind;
pub use crate::api::CantillationMark;
pub use crate::api::CantillationMarkPlacement;
pub use crate::api::CantillationMarkStressPosition;
pub use crate::api::Context;
pub use crate::api::GroupLevel;
pub use crate::api::HebrewAccent;
pub use crate::api::Match;
pub use crate::api::MaxWordSpan;
pub use crate::api::PoetryAccent;
pub use crate::api::ProseAccent;
pub use crate::api::PseudoAccent;
pub use crate::api::SentenceContext;

// public functions
pub use crate::api::try_derive_context;

// used for errorhandling
pub use crate::error::SentenceContextError;
