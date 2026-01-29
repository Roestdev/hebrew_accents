<p align="center">
  Hebrew accents
</p>

<p align="center">
  <a href="https://docs.rs/hebrew_accents"> 
    Documentation
  </a> | <a href="https://github.com/Roestdev/hebrew_accents/">
    Website
  </a> | <a href="https://github.com/Roestdev/hebrew_accents/BACKGROUND.md">
    Introduction & References
  </a> | <a href="https://github.com/Roestdev/hebrew_accents/DESIGN.md">
    Design
  </a>
</p>

Managing Hebrew accents as used in the Masoretic texts.

### Why this crate?
 
The reason this library was created is because the Hebrew accents used in Masoretic texts always exactly the same as UTF-8 code points.

The system of accents employed is complex and nuanced, several factors contribute to the complexity of Hebrew accents in the Masoretic texts, including the following:
- There are **two** different accent systems
- Accents may consists of one or two UTF-8 code points
- Different Hebrew accent names can use the same symbol
- Not all accents are part of both accent systems
- Disagreement among scholars
- Some accents are disjuntive in one system and conjunctive in the other system 

If you want to use the Hebrew UTF8 code points directly you could have a look at [hebrew_unicode_script](<https://crates.io/crates/hebrew_unicode_script>)

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hebrew_accents = "0.0.2"  // or a newer version
```

Then, in your `main.rs`:

``` rust
use hebrew_accents::{SentenceContext, Context, HebrewAccent, ProseAccent};

let newsc = SentenceContext::new("ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות נ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",Context::Prosaic);

assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
// or
assert!(newsc.contains_accent(ProseAccent::Tiphcha.into()));
```
### Goals

The main goal is to write a library that can be used to learn the Masoretic Hebrew accents.

Sub-goals:
- Identify Hebrew accents within the provided text.
- Locate specific Hebrew accents.
- Supply some statistical information.

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