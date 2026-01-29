//!Managing Masoretic Hebrew Accents
//!
//!## Project Status 
//!
//!**This project is currently in the design/development mode.**
//!
//!#### DO NOT USE in production
//!There will be breakages regularly.   
//!For now all versions start with v0.0.x
//!
//!`Current Version:`  v0.0.3
//!
//!`Features:`  
//!
//!##### v0.0.3    
//!
//!  -  Copy trait added to the public API
//!  -  Trait Accent
//!
//!     -  details() -> gives details of an accent
//!     -  category() -> return the accent category
//!     -  accent_type() -> return accent type
//!  
//!##### v0.0.2    
//!
//!  -  Secondairy accent Meteg is added 
//!  
//!##### v0.0.1
//!
//!  `SentenceContext`
//!  - new() ->  SentenceContext
//!  - contains_accent(SentenceContext, HebrewAccent) -> bool 
//!  - relative_strength() -> u8 (give the relative_strengthnumber of an accent for Prose and Poetry accents)
//!
//!## Description
//!
//!This crate will serve as a library for finding, filtering, and displaying Hebrew accents, specifically focusing on the Tiberian accent system as documented by the Masoretes.
//!
//!Be aware that the Hebrew accents are not exactly the same as the UTF8 Hebrew Unicode code-points!
//!
//!### Brief overview in the accents in the Tanach
 //!
//!The accents used in the Tanach have been (are) the subject of extensive scholarly research and discussion, with numerous books and articles written on the topic over the years. The system of accents employed in the Tanach is complex and nuanced, presenting a challenging area of study for scholars and researchers.
//!
//!Several factors contribute to the complexity of Hebrew accents in the Tanach, including the following:
//!
//!- There are **two** main systems of accents: one system for the majority of the books, known as the "Twenty-One Books" (Prose) and another system for the remaining books, known as "Three Books" (Poetry)
//!
 //! - `Prose` ( Hebrew: סגנון פרוזאי ): This style is used for narrative and descriptive passages, such as historical accounts, genealogies, and instructional texts. Prose is characterized by a straightforward and simple writing style, with a focus on conveying information and telling a story.
//!
//!  - `Poetry` ( Hebrew: סגנון שירי ): This style is used for expressive and lyrical passages, such as psalms, songs, and wisdom literature. Poetry is characterized by a more formal and structured writing style, with a focus on rhythm, meter, and figurative language.
//!  
//!  These two styles are not mutually exclusive, and many passages in the Tanach blend elements of both prose and poetry. However, in general, the Tanach can be divided into sections that are primarily prose (such as the historical books of Genesis, Exodus, and Numbers) and sections that are primarily poetry (such as the book of Psalms and the book of Job).
//!
//!  It's worth noting that the distinction between prose and poetry in the Tanach is not always clear-cut, and different scholars and traditions may have different opinions on the matter. Additionally, the writing styles in the Tanach have evolved over time, and different books and sections may reflect different literary and cultural influences.
//!
//!- Accents have three roles at the same time:
//!  
//!  - To indicate of stressed syllables, but not always.
//!  - To indicate the syntactic relation in a sentence.
//!  - Intonation of the words, used for singing the sentence.
//! 
//!- Within each system there are two major categories of Masoretic accents:
//!
//!  - Disjunctive accents
//!  - Conjunctive accents
//!
//!- The same (UTF-8) accent can be disjunctive in the "Twenty-One Books" and conjunctive in the "Three Books".
//!
//!- It is possible that one Unicode code-point can be mapped to different Hebrew accents.
//!
//!#### Notes
//!
//! - Accents are sometimes referred to as *Hebrew Cantilationmarks*, *taʿamei ha-mikra* (טעמי המקרא) or *teʿamim* (טעמים).
//!
//! - `Disjunctives` are sometimes referred to as *pausal* or *domini*
//!
//! - `Conjunctives` are sometimes referred to as *non-pausal* or *servi*
//!  
//!*More details can be found in the references section at the end of this document*
//!
//!### Goal
//!
//!The main goal is to write a library that can be used to learn the Masoretic Hebrew accents.
//!
//!Sub-goals:
//!- Identify Hebrew accents within the provided text.
//!- Offer the ability to filter specific accents.
//!- Offer an option to display only specific accents including all consonants.
//!- Supply statistical information regarding the usage of the accents.
//!
//!### Non-Goals
//!
//!- Detect errors in the text related to accentuation rules.
//!- Determine the type of text based on the identified accents.
//!- Achieve absolute accuracy (100% correctness).
//!
//!## Examples
//!
//!### Usage contains_accent()
//!
//!//!``` rust
//!use hebrew_accents::SentenceContext;;
//!use hebrew_accents::Context;
//!use hebrew_accents::HebrewAccent;
//!use hebrew_accents::ProseAccent;

//!let newsc = SentenceContext::new("ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות נ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃", Context::Prosaic,);
//!assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
//!// or
//!assert!(newsc.contains_accent(ProseAccent::Tiphcha.into()));
//!//!```
//!
//!``` rust
//!use hebrew_accents::SentenceContext;;
//!use hebrew_accents::Context;
//!use hebrew_accents::HebrewAccent;
//!use hebrew_accents::PoetryAccent;
//!
//!let newsc = SentenceContext::new("יצחק אל־יע֓קב ׀ ויברך", Context::Poetic);
//!assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
//!//!```
//!
//!### Usage find_accent()
//!
//!TODO
//!
//!### Usage relative_strength()
//!
//!//!``` rust
//!use hebrew_accents::ProseAccent;
//!
//!let prose_accent = ProseAccent::TelishaGedolah;
//!assert_eq!(17, prose_accent.relative_strength());
//!```
//!
//!``` rust
//!use hebrew_accents::PoetryAccent;
//!
//!let poetry_accent = PoetryAccent::ShalsheletGadol;
//!assert_eq!(6, poetry_accent.relative_strength());
//!```
//!
//!
//!## Releases
//!
//!For an overview of released versions see [releases](<https:://github.com/Roestdev/hebrew_accents/releases>).   
//!
//!## How to install `hebrew_accents`
//!
//!For installation see the [hebrew_accents](<https:://crates.io/crates/hebrew_accents>) page at crates.io.
//!
//!## Safety
//!
//!TODO
//!
//!## Panics
//!
//!TODO
//!
//!## Errors
//!
//!TODO
//!

#![warn(missing_docs)]

// common
mod char;

// finding accents
mod sentence_context; // main entry
mod sentence_ctx_contains;
mod sentence_ctx_find;
mod sentence_ctx_funcs;
mod sentence_ctx_regex;

// static accent data
mod accent; // main entry
mod accent_codepoints;
mod accent_data;
mod accent_display;

// exports
pub use accent::*;
pub use accent_display::*;
pub use sentence_context::*;
