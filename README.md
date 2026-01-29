### hebrew_accents

Managing Hebrew Accents as used in the Masoretic text

### Why this crate
 
The reason this library was created is because the Hebrew accents used in Tanach are not the same as UTF-8 code points. 

This crate will serve as a library for finding, filtering, and displaying Hebrew accents, specifically focusing on the Tiberian accent system as documented by the Masoretes.

If you want to use the Hebrew UTF8 code points directly you could have alook at [hebrew_unicode_point](<https://crates.io/crates/hebrew_unicode_script>)

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

## License

The `hebrew_accents` library is distributed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Project Status 

**This project is currently in the design/development mode.**

#### DO NOT USE in production!
There will be breakages regularly.

