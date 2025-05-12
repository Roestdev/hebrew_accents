# hebrew_accents
Handling Hebrew accents

</br>

## Project Status 

**This project is currently in the design mode.**

`Current Version:`  v0.0.0.

``` txt
This README file provides a general overview of what to expect from this library.
```

</br>

## Description
 
This crate is going to be a library to find, filter, show Hebrew accents.

### Brief overview in the accents in the Tanach
 
The accents used in the Tanach have been the subject of extensive scholarly research and discussion, with numerous books and articles written on the topic over the years. The system of accents employed in the Tanach is complex and nuanced, presenting a challenging area of study for scholars and researchers.

Several factors contribute to the complexity of Hebrew accents in the Tanach, including:
-  In the Tanach, there are two main systems of accents used: one for the majority of the books, known as the "Twenty-One Books" (which includes all the books except for the three poetic books), and another for the three poetic books of Psalms, Proverbs, and Job, known as "Three Books".

- There are two major categories of Masoretic accents:
  - the disjunctive accents and
  - the conjunctive accents.

- All disjunctive accents are ordered and are divided in groups

- Accents can be disjunctive in the "Twenty-One Books" and conjunctive in the "Three Books"
  
- Accents have three role at the same time:
  - To indicate of stressed syllables, but not always.
  - To indicate the syntactic relation in a sentence.
  - Intonation of the words, used for singing the sentence.

- It is possible that one Unicode code-point can be mapped to different Hebrew accents.


### UTF-8 and Hebrew Accents: Challenges and Inconsistencies

- The representation of Hebrew accents in UTF-8 is not without its challenges. One major issue is the inconsistent definition of two accent marks in the Tanach, which has led to confusion and potential errors. The affected marks are:

   -  The Yetiv mark
   -  The Tevir mark

  In various sources, the Yetiv mark is classified as a conjunctive accent, while the Tevir mark is considered a disjunctive accent. However, the Unicode tables seem to reverse this definition, treating the Yetiv mark as a disjunctive accent and the Tevir mark as a conjunctive accent.

- Furthermore, the encoding of Hebrew accents in UTF-8 is complex, as a single accent can be represented by either one or two Unicode code-points. 

- Additionally, the mapping of Unicode code-points to Hebrew accents is not always one-to-one, meaning that a single code-point can potentially correspond to multiple Hebrew accents, depending of the context.

For more information see [Unicode Problems](https://mechon-mamre.org/c/hr/unicode.htm).

### Goals
 
- Identify Hebrew accents within the provided text.
- Offer the ability to filter specific accents.
- Provide an option to display the identified accents.
- Supply statistical information regarding the usage of the accents.

### Non-Goals

- Detect errors in the text related to accentuation rules.
- Determine the type of text based on the identified accents.
- Achieve absolute accuracy (100% correctness).

## Examples

TODO

## Releases

For an overview of released versions see [releases](https://github.com/Roestdev/hebrew_accents/releases).   

## How to install `hebrew_accents`

For installation see the [hebrew_accents](https://crates.io/crates/hebrew_accents) page at crates.io.

## Safety

TODO

## Panics

TODO

## Errors

TODO

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

## References

 - [Unicode Block Hebrew - chapter 9.1 - section: meteg](https://www.unicode.org/charts/PDF/U0590.pdf).
  
 - [Basics of Hebrew Accents](https://zondervanacademic.com/products/basics-of-hebrew-accents), written by Mark D. Futato, Sr.

 - [Hebrew Cantillation Marks And Their Encoding](https://mechon-mamre.org/c/hr/index.htm) written by Helmut Richter.
  
 - [Gesenius Hebrew Grammar - §15. The Accents.](https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar/15._The_Accents)

 - [A treatise on the accentuation of the twenty-one so-called prose books of the Old Testament](https://archive.org/details/treatiseonaccent00wickuoft) written by Wickes.

 - [A treatise on the accentuation of the three so-called poetical books of the Old Testament](https://archive.org/details/treatiseonaccent0000wick) written by Wickes.

 - [The Syntax of Masoretic Accents in the Hebrew Bible](https://jamesdprice.com/images/21_Syntax_of_Accents_rev._ed..pdf) written by James D. Price, Ph.D.

#### Notes
 - Accents are sometimes called *Hebrew Cantilationmarks*

# todo

- secondary accents (mayella, meteg)