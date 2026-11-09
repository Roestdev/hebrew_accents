
# Hebrew accents

A Rust library for working with Masoretic Hebrew cantillation marks (טעמים / ta'amim) — the "accents" that appear in the Tanach (Biblical Hebrew texts). This crate abstracts the complexities of the Masoretic Hebrew accent system, providing tools to detect, identify, and analyze Hebrew accents programmatically.


[📚 Documentation](https://docs.rs/hebrew_accents) |
[🌐 Repository](https://github.com/Roestdev/hebrew_accents/) |
[📖 Background & References](https://github.com/Roestdev/hebrew_accents/blob/main/BACKGROUND.md) |
[🛠️ Design](https://github.com/Roestdev/hebrew_accents/blob/main/DESIGN.md)


[![License](https://img.shields.io/crates/l/hebrew_accents)](./LICENSE)
[![Status](https://img.shields.io/badge/status-development-orange)]()
[![Crates.io](https://img.shields.io/crates/v/hebrew_accents)](https://crates.io/crates/hebrew_accents)
[![Docs.rs](https://docs.rs/hebrew_accents/badge.svg)](https://docs.rs/hebrew_accents)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hebrew_accents = "0.0.3"   # Latest: check https://crates.io/crates/hebrew_accents
```

**Basic example:**

```rust
use hebrew_accents::{SentenceContext, SentenceContextError, Context, HebrewAccent, ProseAccent};

fn main() -> Result<(), SentenceContextError> {
    let sentence_context = SentenceContext::new(
        "וַיּ֣רָא עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
        Context::Prosaic,
    )?;

    // Check if an accent exists in a given sentence
    assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));

    // Find accent positions
    // Note: you can also use `.into()` due to the `From` trait implementation.
    if let Some(match_) = sentence_context.find_accent(ProseAccent::Atnach.into()) {
        println!("Atnach found at bytes {}: {}", match_.start(), match_.end());
        println!("Text: {}", match_.as_str());
    }
    
    Ok(())
}
```

## Why This Crate?

Hebrew accents in Masoretic texts are not always exactly the same as UTF-8 code points. The system is complex and nuanced:

  - Two different accent systems (Prose vs. Poetry) with overlapping but distinct sets
  - Compound accents consisting of two UTF-8 code points
  - Different accent names can share the same symbol depending on context
  - Not all accents appear in both systems
  - Scholarly disagreement on classifications

If you only need raw Unicode code points, see my other companion crate [`hebrew_unicode_script`](https://crates.io/crates/hebrew_unicode_script).

## When to Use This Crate

  - Explore detailed properties of each Masoretic accent (names, meanings, strengths, hierarchies)
  - Programmatically inspect Biblical texts for research or tooling

## Core Concepts

### Conceptual Model

There are three distinct axes in play — and the vocabularies are easy to collide:

1. **Conjunctive vs. disjunctive** — the word's syntactic relationship to its
    neighbors (i.e., whether it belongs with the following phrase-breaker or not).

2. **Hierarchy of disjunctives** — expressed on the same axis in two ways:
    - **Tier** — as described in *Basics of Biblical Hebrew Accents* by Mark D.
      Futato, Sr. This is where earlier grammarians used "emperor/king/duke" language.
    
    - **Rank** — every disjunctive gets a unique consecutive number, where 1 is
      the strongest disjunctive and higher numbers are weaker ones.

3. **Primary vs. secondary (stress)** — phonetic only: the one main accent per
    word versus auxiliary marks like meteg/ga'ya.

### Accent Types

```rust
use hebrew_accents::{HebrewAccent, ProseAccent, PoetryAccent, PseudoAccent, Accent};

// Prose accents (used in narrative texts like Genesis, Exodus)
let prose = HebrewAccent::Prose(ProseAccent::Silluq);
// println!("Prose accent name: {}", prose.english_name());

// Poetry accents (used in Psalms, Job, Proverbs)
let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);
// println!("Poetry accent name: {}", poetry.english_name());

// Pseudo-accents (accent-related markers)
let pseudo = HebrewAccent::Pseudo(PseudoAccent::Maqqaph);
// println!("Pseudo accent name: {}", pseudo.english_name());
```

### Context

Sentences have either Prosaic or Poetic context, which affects accent interpretation:

```rust
use hebrew_accents::{SentenceContext, Context, SentenceContextError};

fn main() -> Result<(), SentenceContextError> {
    let prose_context = SentenceContext::new("וַיְהִי", Context::Prosaic)?;
    let poetry_context = SentenceContext::new("זְמִירוֹת", Context::Poetic)?;

    println!("Prose context created");
    println!("Poetry context created");

    Ok(())
}
```

## API Overview

> **Note:** All accent-taking APIs accept a `HebrewAccent` directly, or any type
> convertible into one via `From` — e.g. `ProseAccent::Atnach.into()` is equivalent
> to `HebrewAccent::Prose(ProseAccent::Atnach)`. Use whichever reads better in your code.

### Accent Detection

```rust
use hebrew_accents::{HebrewAccent, SentenceContext, ProseAccent, Context};
// Check if an accent exists in a sentence
if let Ok(sentence_context) = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃",Context::Prosaic){
    if sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)) {
        println!("Found Silluq!");
    }
}
```

### Accent Positions

```rust
// Get byte offset of an accent
use hebrew_accents::{HebrewAccent, ProseAccent, SentenceContext, Context};

let sentence_context_result = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃",Context::Prosaic);
if let Ok(sentence_context) = sentence_context_result {
    println!("Sentence: \t{}",sentence_context.as_str());
    if let Some(match_val) = sentence_context.find_accent(HebrewAccent::Prose(ProseAccent::Munach)) {
        println!("At {}-{}: {}", match_val.start(), match_val.end(), match_val.as_str());
    }
}
```

### Context Detection

> **Note:** `Known Limitations of Context Detection`
> Because some Hebrew accents appear in both prosaic and poetic systems, accurate classification depends on finding at least one uniquely identifying accent. If the sentence contains only shared accents or a mixture from both registers, definitive determination is not possible, and try_derive_context will return an error.

```rust
// Try to automatically determine whether a sentence follows prose or poetry patterns:
use hebrew_accents::{SentenceContext, Context};

fn main() {
    let result_sentence_context = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃",Context::Prosaic);
    if let Ok(sentence_context) = result_sentence_context {
        match sentence_context.try_derive_context() {
            Ok(Context::Poetic) => eprintln!("Poetry detected!"),
            Ok(Context::Prosaic) => eprintln!("Prose detected!"),
            Err(e) => eprintln!("Ambiguous or no distinctive accents: {}", e),
        }
    }
}
```

### Accent Metadata

Each accent implements the Accent trait, which opens the accent metadata.
```rust
fn main(){
    use hebrew_accents::{Accent, HebrewAccent, ProseAccent};

    let accent = HebrewAccent::Prose(ProseAccent::Silluq);

    // a few examples
    println!("Hebrew name: {}", accent.hebrew_name());
    println!("English name: {}", accent.english_name());
    println!("Concept: {}", accent.hebrew_concept());
    println!("Relative strength: {:?}", accent.rank());
}
```

## Goals

The main goal is to write a library that can be used to learn more about the Masoretic Hebrew accents as used in the Tanach.

Sub-goals:

- Locate every accent token in a supplied string
- Check whether a particular accent occurs
- Gather simple statistics (counts, distribution, etc.)
- Provide "accurate" metadata for scholarly research

## Non-Goals
  
  - Detect errors in accentuation rules
  - Determine accent type based solely on identified accents
  - Achieve absolute accuracy (scholarly disputes exist)

## Project Status

> ⚠️ **Warning:** This project is currently in design/development mode.
>
> ❌ **DO NOT USE in production!** Expect breaking changes until v1.0.
>
> ✅ **Tests passing.** CI integration coming soon.

## Contributing

Contributions welcome! Please read the Design and Background documents before submitting PRs.

## License

The `hebrew_accents` library is distributed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

## Contribution Notice

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Special thanks to scholars of the Masoretic tradition whose work makes this library possible. Research methodology draws heavily from the Futato classification system and BHS (Biblia Hebraica Stuttgartensia) standards.
