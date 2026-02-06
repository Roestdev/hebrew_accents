//! # Hebrew Accents
//!
//! A Rust library for working with the **Masoretic Hebrew cantillation marks**
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
//! All `text inspection` related code can be found in files starting wit *`senctence_ctx`*
//!
//! ### Main types
//!
//! The whole crate is build around the main type **`HebrewAccent`**,
//!  with its three subtypes:
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
//! For this reason there is a new type called `SentenceContext`, which is a struct  that contains both the sentence and the corresponding context.
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
//! The following funtions are avaiable for `SentenceContext`:
//! 
//! - new() -> `SentenceContext`
//! - contains_accent(HebrewAccent) -> bool 
//! - find_accent(HebrewAccent) -> Option<Match> 
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
//!  - accent_type          (Primary, Secondary - optional)
//!  - category             (Disjunctive, Conjunctive - optional)
//!  - word_stress          (ImPositive, PostPositive, PrePositive - optional)
//!  - UTF-8 code_points    (1 or 2)
//!  - relative_strength    (lowest number has relative more weight)
//!  - count                (the number of all accents in a specific category)
//!  - details              (only used for debugging at this moment)
//!
//! ## Examples
//! todo
//!
//! ### Text inspection
//!
//! ### Accent metadata
//!
//! *Note: For more information see [DESIGN]("../DESIGN.md")*

#![deny(missing_docs, unused_imports)]
// import doc tests README.md
#[cfg_attr(doctest, doc = include_str!("../README.md"))]
// common items
mod char;

// finding Hebrew Accents
mod sentence_ctx; // main entry
mod sentence_ctx_contains;
mod sentence_ctx_find;
mod sentence_ctx_funcs;
mod sentence_ctx_regex;

// static 'Hebrew Accent' data
mod accent; // main entry
mod accent_codepoints;
mod accent_data;
mod accent_display;

// exports
pub use accent::*;
pub use accent_display::*;
pub use sentence_ctx::*;
