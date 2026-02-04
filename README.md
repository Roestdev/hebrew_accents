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
  <a href="https://github.com/Roestdev/hebrew_accents/BACKGROUND.md">
    📖 Background & References
  </a> |
  <a href="https://github.com/Roestdev/hebrew_accents/DESIGN.md">
    🛠️ Design
  </a>
</p>


### What is this crate?

`hebrew_accents` provides a **Rust library for working with the Masoretic Hebrew cantillation marks** (the “accents” that appear in the Tanach). This crate abstracts the complexities of the Masoretic Hebrew cantillation marks (see the file [BACKGROUND](BACKGROUND) for more insight in the complexities) 

### Why this crate?
 
The reason this library was created is because the Hebrew accents used in Masoretic texts **not** always exactly the same as UTF-8 code points. The system of accents employed is complex and nuanced, several factors contribute to the complexity of Hebrew accents in the Masoretic texts, including the following:

- There are **two** different accent systems
- Accents may consists of one or two UTF-8 code points
- Different Hebrew accent names can use the same symbol
- Not all accents are part of both accent systems
- Disagreement among scholars
 
If you only need raw Unicode code points, see my other companion crate [`hebrew_unicode_script`](https://crates.io/crates/hebrew_unicode_script).

### When to use

There are two main reasons for use this crate:

1. Explore the more detailed properties of each Masoretic accent.
2. Programmatically inspect Biblical texts for research or tooling.

### How to use

Add this to your `Cargo.toml`:

```toml
[dependencies]
hebrew_accents = "0.0.3"  // or a newer version
```

Then import and use the API in your `main.rs`:

``` rust
use hebrew_accents::{SentenceContext, Context, HebrewAccent, ProseAccent};
// create a new SentenceContext (String,Context)
let sc = SentenceContext::new("ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות נ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",Context::Prosaic);
assert!(sc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
// or
assert!(sc.contains_accent(ProseAccent::Tiphcha.into()));
```

### Goals

The main goal is to write a library that can be used to learn more about the Masoretic Hebrew accents as used in the Tanach

Sub-goals:

- Locate every accent token in a supplied string.
- Check whether a particular accent occurs.
- Gather simple statistics (counts, distribution, etc.).

### Non-Goals

- Detect errors in the text related to accentuation rules.
- Determine the type of text based on the identified accents.
- Achieve absolute accuracy.

### Project Status 

<span style="color: #F39C12;">⚠️ Warning: This project is currently in the design/development mode</span> 

<span style="color: #E74C3C;">❌ DO NOT USE in production!</span> 
 There will be breakages regularly.

<span style="color: #2ECC71;">✅ Success: All tests passed</span>  

### License

The `hebrew_accents` library is distributed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.