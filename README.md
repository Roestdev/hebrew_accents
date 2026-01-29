### hebrew_accents

Managing Hebrew accents as used in the Masoretic texts.

### Why this crate?
 
The reason this library was created is because the Hebrew accents used in Masoretic texts always exactly the same as UTF-8 code points.

The accents used in the Masoretic texts have been (are) the subject of extensive scholarly research and discussion, with numerous books and articles written on the topic over the years. The system of accents employed is complex and nuanced, presenting a challenging area of study for scholars and researchers.

Several factors contribute to the complexity of Hebrew accents in the Masoretic texts, including the following:
- Two seperate accent systems
- Accents may consists of two UTF8 code points

If you want to use the Hebrew UTF8 code points directly you could have a look at [hebrew_unicode_script](<https://crates.io/crates/hebrew_unicode_script>)

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

**This project is currently in the design/development mode.**

#### DO NOT USE in production!
There will be breakages regularly.

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

<!-- 1️⃣ Inline HTML -->
##<span style="color:#ff4500;">orange text</span>

<!-- 2️⃣ Legacy <font> -->
##<font color="green">green text</font>