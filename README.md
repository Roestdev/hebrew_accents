# hebrew_accents

Managing Masoretic Hebrew Accents

</br>

## Project Status 

**This project is currently in the design/development mode.**

`Current Version:`  v0.0.1.

`Current feautures:`  
- fn contains_accent(SentenceContext, HebrewAccent) -> bool

</br>

## Description
 
This crate will serve as a library for finding, filtering, and displaying Hebrew accents, specifically focusing on the Tiberian accent system as documented by the Masoretes.


### Brief overview in the accents in the Tanach
 
The accents used in the Tanach have been (are) the subject of extensive scholarly research and discussion, with numerous books and articles written on the topic over the years. The system of accents employed in the Tanach is complex and nuanced, presenting a challenging area of study for scholars and researchers.

Several factors contribute to the complexity of Hebrew accents in the Tanach, including:
- Accents have three roles at the same time:
  - To indicate of stressed syllables, but not always.
  - To indicate the syntactic relation in a sentence.
  - Intonation of the words, used for singing the sentence.

-  In the Tanach, there are two main systems of accents used: one for the majority of the books, known as the "Twenty-One Books" (which includes all the books except for the three poetic books), and another for the three poetic books of Psalms, Proverbs, and Job, known as "Three Books".

- Within the main system there are two major categories of Masoretic accents:
  - Disjunctive accents
  - Conjunctive accents

- Disjunctive accents are ordered according their relative importance.
  
- Disjunctive accents may be organized hiarchial groups. 

  - In the "Twenty-One Books," there are four groups of disjunctive accents, while the "Three Books" contain three groups.

- The same (UTF-8) accent can be disjunctive in the "Twenty-One Books" and conjunctive in the "Three Books".

- It is possible that one Unicode code-point can be mapped to different Hebrew accents.

### UTF-8 and Hebrew Accents: Challenges and Inconsistencies

- The representation of Hebrew accents in UTF-8 is not without its challenges. One major issue is the inconsistent definition of two accent marks in the Tanach, which has led to confusion and potential errors. These marks are:

   -  The Yetiv mark
   -  The Tevir mark

  In various sources, the Yetiv mark is classified as a conjunctive accent, while the Tevir mark is considered a disjunctive accent. However, the Unicode tables seem to reverse this definition, treating the Yetiv mark as a disjunctive accent and the Tevir mark as a conjunctive accent.

- Furthermore, the encoding of Hebrew accents in UTF-8 is complex, as a single accent can be represented by either one or two Unicode code-points. 

- Additionally, the mapping of Unicode code-points to Hebrew accents is not always one-to-one, meaning that a single code-point can potentially correspond to multiple Hebrew accents, depending of the context.

For more information see [Unicode Problems](https://mechon-mamre.org/c/hr/unicode.htm).

### Goal

The main goal is to write a library that can be used to learn the Masoretic Hebrew accents.

Sub-goals:
- Identify Hebrew accents within the provided text.
- Offer the ability to filter specific accents.
- Offer an option to display only specific accents including all consonants.
- Supply statistical information regarding the usage of the accents.

### Non-Goals

- Detect errors in the text related to accentuation rules.
- Determine the type of text based on the identified accents.
- Achieve absolute accuracy (100% correctness).

## Caveats

In order to locate some of the accents the crate Fancy-Regex is used.
This gives some issues, because the `negative lookbehind` feature does not function with quantifiers.
It concerns the following Poetic accents:
- Revia Gadol  
  Could also be a `Revia Mugrash` or a `Revia Qaton`

- Revia Qaton  
  Could also be a `Revia Mugrash`

- Merkha   
  Could also be an `Ole We Yored` or `Tsinnorit Merkha`

- Mahpakh  
  Could also be a `Tsinnorit Mahpkh`

## Examples

### Usage contains_accent()

``` rust
use hebrew_accents::SentenceContext;;
use hebrew_accents::Context;
use hebrew_accents::HebrewAccent;
use hebrew_accents::ProseAccent;

let newsc = SentenceContext::new("ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות נ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃", Context::Prosaic,);
assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
```

``` rust
use hebrew_accents::SentenceContext;;
use hebrew_accents::Context;
use hebrew_accents::HebrewAccent;
use hebrew_accents::PoetryAccent;

let newsc = SentenceContext::new("יצחק אל־יע֓קב ׀ ויברך", Context::Poetic);
assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
```

### Usage find_accent()

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

 - [Hebrew Cantillation Marks And Their Encoding](https://mechon-mamre.org/c/hr/index.htm) by Helmut Richter.
  
 - [Gesenius Hebrew Grammar - §15. The Accents.](https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar/15._The_Accents)

 - [A treatise on the accentuation of the twenty-one so-called prose books of the Old Testament](https://archive.org/details/treatiseonaccent00wickuoft) by Wickes

 - [A treatise on the accentuation of the three so-called poetical books of the Old Testament](https://archive.org/details/treatiseonaccent0000wick) by Wickes

 - [The Syntax of Masoretic Accents in the Hebrew Bible](https://jamesdprice.com/images/21_Syntax_of_Accents_rev._ed..pdf) by James D. Price, Ph.D.

 - [Tabula Accentum](https://www.oakleys.org.uk/files/blog_files/2023/05/tabula_accentuum.pdf) of the "Biblia Hebraica Stuttgartensia".

 - [Introduction to Tiberian Hebrew Accents](https://assets.cambridge.org/97811084/79936/excerpt/9781108479936_excerpt.pdf) an excerpt by Sung Jin Park.
 
 - [Gesenius Hebrew Grammer](https://dn790008.ca.archive.org/0/items/geseniushebrewgr00geseuoft/geseniushebrewgr00geseuoft.pdf)

 - [The Masoretes and the Punctuation of Biblical Hebrew](https://usermanual.wiki/bililite/MasoreticPunctuation.1300310592.pdf) from the British & Foreign Bible Society
  
  - [ACCENTS IN HEBREW](https://www.jewishencyclopedia.com/articles/717-accents-in-hebrew) by  Max L. Margolis

  - [The Masoretic Hebrew Accents in Translation and Interpretation](https://hebrew4christians.com/Grammar/Unit_Three/Word_Accents/HebrewAccents_Barrick.pdf) by William D. Barrick

- [Hebrew_Cantillation](https://en.wikipedia.org/wiki/Hebrew_cantillation)  (wikipedia)
  
#### Notes

 - Accents are sometimes referred to as *Hebrew Cantilationmarks*, *taʿamei ha-mikra* (טעמי המקרא) or *teʿamim* (טעמים).

 - `Disjunctives` are sometimes referred to as *pausal* or *domini*

 - `Conjunctives` are sometimes referred to as *non-pausal* or *servi*

