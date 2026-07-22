
<p align="center">
  Hebrew accents
</p>

<p align="center">
  <a href="https://docs.rs/hebrew_accents">
    📚 Documentation
  </a> |
  <a href="https://github.com/Roestdev/hebrew_accents/">
    🌐 Repository
  </a> |
  <a href="https://github.com/Roestdev/hebrew_accents/blob/main/BACKGROUND.md">
    📖 Background & References
  </a> |
  <a href="https://github.com/Roestdev/hebrew_accents/blob/main/DESIGN.md">
    🛠️ Design
  </a>
</p>


# hebrew_accents

A Rust library for working with Masoretic Hebrew cantillation marks (טעמים‎ / ta'amim) — the "accents" that appear in the Tanach (Biblical Hebrew texts).

This crate abstracts the complexities of the Masoretic Hebrew accent system, providing tools to detect, identify, and analyze Hebrew accents programmatically.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
hebrew_accents = "0.0.3"   # or a newer version
```

**Basic example:**

```rust
use hebrew_accents::{SentenceContext, Context, HebrewAccent, ProseAccent};

// Create a sentence with context
let sentence_context = SentenceContext::new(
    "וַיּ֣רָא עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
    Context::Prosaic
)?;

// Check if an accent exists
assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));

// Find accent positions
if let Some(match_) = sentence_context.find_accent(ProseAccent::Atnach.into()) {
    println!("Atnach found at bytes {}: {}", match_.start(), match_.end());
    println!("Text: {}", match_.as_str());
}
```

# Why This Crate?

Hebrew accents in Masoretic texts are not always exactly the same as UTF-8 code points. The system is complex and nuanced:

  - Two different accent systems (Prose vs. Poetry) with overlapping but distinct sets
  - Compound accents consisting of one or two UTF-8 code points
  - Different accent names can share the same symbol depending on context
  - Not all accents appear in both systems
  - Scholarly disagreement on classifications

If you only need raw Unicode code points, see my other companion crate [`hebrew_unicode_script`](https://crates.io/crates/hebrew_unicode_script).

# When to Use This Crate

  - Explore detailed properties of each Masoretic accent (names, meanings, strengths, hierarchies)
  - Programmatically inspect Biblical texts for research or tooling

# Core Concepts

## Accent Types

``` rust
use hebrew_accents::{HebrewAccent, ProseAccent, PoetryAccent, PseudoAccent};

// Prose accents (used in narrative texts like Genesis, Exodus)
let prose = HebrewAccent::Prose(ProseAccent::Silluq);

// Poetry accents (used in Psalms, Job, Proverbs)
let poetry = HebrewAccent::Poetry(PoetryAccent::Atnach);

// Pseudo-accents (accent related markers)
let pseudo = HebrewAccent::Pseudo(PseudoAccent::Maqqeph);
```

## Context

Sentences have either Prosaic or Poetic context, which affects accent interpretation:
``` rust
use hebrew_accents::{SentenceContext, Context};

let prose_context = SentenceContext::new("וַיְהִי", Context::Prosaic)?;
let poetry_context = SentenceContext::new("זְמִירוֹת", Context::Poetic)?;
```

## Accent Metadata

Each accent implements the Accent trait:
``` rust
use hebrew_accents::{Accent, HebrewAccent, ProseAccent};

let accent = HebrewAccent::Prose(ProseAccent::Silluq);

println!("Hebrew name: {}", accent.hebrew_name());
println!("English name: {}", accent.english_name());
println!("Concept: {}", accent.hebrew_concept());
println!("Is compound: {}", accent.is_compound());
println!("Relative strength: {}", accent.relative_strength());
```

# API Overview

## Detection

```rust
// Check if an accent exists in a sentence
if sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)) {
    println!("Found Silluq!");
}
```
## Finding Positions

```rust
// Get byte offset of an accent
if let Some(m) = sentence_context.find_accent(HebrewAccent::Prose(ProseAccent::Atnach)) {
    println!("At {}-{}: {}", m.start(), m.end(), m.as_str());
}
```

## Context Detection

``` rust
Automatically determine whether a sentence follows prose or poetry patterns:
let result = sentence_context.try_determine_context();
match result {
    Ok(Context::Poetic) => println!("Poetry detected!"),
    Ok(Context::Prosaic) => println!("Prose detected!"),
    Err(e) => println!("Ambiguous or no distinctive accents: {}", e),
}
```
# Goals

The main goal is to write a library that can be used to learn more about the Masoretic Hebrew accents as used in the Tanach

Sub-goals:

- Locate every accent token in a supplied string
- Check whether a particular accent occurs
- Gather simple statistics (counts, distribution, etc.)
- Provide "accurate" metadata for scholarly research

# Non-Goals
  
  - Detect errors in accentuation rules
  - Determine accent type based solely on identified accents
  - Achieve absolute accuracy (scholarly disputes exist)

# Known Limitations

Because some Hebrew accents appear in both prosaic and poetic systems, accurate classification depends on finding at least one uniquely identifying accent. If the sentence contains only shared accents or a mixture from both registers, definitive determination is not possible.

# Project Status

<span style="color: #F39C12;">⚠️ Warning: This project is currently in design/development mode</span>

<span style="color: #E74C3C;">❌ DO NOT USE in production!</span>

Expect breaking changes regularly until version 1.0.

<span style="color: #2ECC71;">✅ Success: All tests passed</span>


# UPCOMING! (Examples Directory)

See the examples/ directory for complete usage scenarios:

    basic_detection.rs — Basic accent presence testing
    position_finding.rs — Locating accent positions in text
    context_analysis.rs — Prose vs. poetry detection

Run examples with:
cargo run --example basic_detection

# Contributing

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

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any accent_meta_data terms or conditions.

## Resources

    Full Documentation
    Repository
    Background & References
    Design Documents

# Acknowledgments

Special thanks to scholars of the Masoretic tradition whose work makes this library possible. Research methodology draws heavily from the Futato classification system and BHS (Biblia Hebraica Stuttgartensia) standards.

# END OF NEW README
# START OF OLD README


### What is this crate?

`hebrew_accents` provides a **Rust library for working with the Masoretic Hebrew cantillation CantillationSymbol** (the “accents” that appear in the Tanach). This crate abstracts the complexities of the Masoretic Hebrew cantillation CantillationSymbol (see the file [BACKGROUND](BACKGROUND) for more insight in the complexities) 

### Why this crate?
 
The reason this library was created is because the Hebrew accents used in Masoretic texts **not** always exactly the same as UTF-8 code points. The system of accents employed is complex and nuanced, several factors contribute to the complexity of Hebrew accents in the Masoretic texts, including the following:

- There are **two** different accent systems
- Accents may consists of one or two UTF-8 code points
- Different Hebrew accent names can use the same symbol
- Not all accents are part of both accent systems
- Disagreement among scholars
 
If you only need raw Unicode code points, see my other companion crate [`hebrew_unicode_script`](https://crates.io/crates/hebrew_unicode_script).


# END OF README