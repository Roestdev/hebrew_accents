# hebrew_accents

Managing Masoretic Hebrew Accents
</br>

## Project Status 

**This project is currently in the design/development mode.**

#### DO NOT USE in production
There will be breakages regularly.   
For now all versions start with v0.0.x

`Current Version:`  v0.0.3

`Features:`  

##### v0.0.3    

  -  Copy trait added to the public API
  -  Trait Accent

     -  details() -> gives details of an accent
     -  category() -> return the accent category
     -  accent_type() -> return accent type
  
##### v0.0.2    

  -  Secondairy accent Meteg is added 
  
##### v0.0.1

  `SentenceContext`
  - new() ->  SentenceContext
  - contains_accent(SentenceContext, HebrewAccent) -> bool 
  - rank() -> u8 (give the ranknumber of an accent for Prose and Poetry accents)

## Description
 
This crate will serve as a library for finding, filtering, and displaying Hebrew accents, specifically focusing on the Tiberian accent system as documented by the Masoretes.

Be aware that the Hebrew accents are not exactly the same as the UTF8 Hebrew Unicode code-points!

### Brief overview in the accents in the Tanach
 
The accents used in the Tanach have been (are) the subject of extensive scholarly research and discussion, with numerous books and articles written on the topic over the years. The system of accents employed in the Tanach is complex and nuanced, presenting a challenging area of study for scholars and researchers.

Several factors contribute to the complexity of Hebrew accents in the Tanach, including the following:

- There are **two** main systems of accents: one system for the majority of the books, known as the "Twenty-One Books" (Prose) and another system for the remaining books, known as "Three Books" (Poetry)

  - `Prose` ( Hebrew: סגנון פרוזאי ): This style is used for narrative and descriptive passages, such as historical accounts, genealogies, and instructional texts. Prose is characterized by a straightforward and simple writing style, with a focus on conveying information and telling a story.

  - `Poetry` ( Hebrew: סגנון שירי ): This style is used for expressive and lyrical passages, such as psalms, songs, and wisdom literature. Poetry is characterized by a more formal and structured writing style, with a focus on rhythm, meter, and figurative language.
  
  These two styles are not mutually exclusive, and many passages in the Tanach blend elements of both prose and poetry. However, in general, the Tanach can be divided into sections that are primarily prose (such as the historical books of Genesis, Exodus, and Numbers) and sections that are primarily poetry (such as the book of Psalms and the book of Job).

  It's worth noting that the distinction between prose and poetry in the Tanach is not always clear-cut, and different scholars and traditions may have different opinions on the matter. Additionally, the writing styles in the Tanach have evolved over time, and different books and sections may reflect different literary and cultural influences.

- Accents have three roles at the same time:
  
  - To indicate of stressed syllables, but not always.
  - To indicate the syntactic relation in a sentence.
  - Intonation of the words, used for singing the sentence.

- Within each system there are two major categories of Masoretic accents:

  - Disjunctive accents
  - Conjunctive accents

- The same (UTF-8) accent can be disjunctive in the "Twenty-One Books" and conjunctive in the "Three Books".

- It is possible that one Unicode code-point can be mapped to different Hebrew accents.

#### Notes

 - Accents are sometimes referred to as *Hebrew Cantilationmarks*, *taʿamei ha-mikra* (טעמי המקרא) or *teʿamim* (טעמים).

 - `Disjunctives` are sometimes referred to as *pausal* or *domini*

 - `Conjunctives` are sometimes referred to as *non-pausal* or *servi*
  
*More details can be found in the references section at the end of this document*

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

### Usage rank()

``` rust
use hebrew_accents::ProseAccent;

let prose_accent = ProseAccent::TelishaGedolah;
assert_eq!(17, prose_accent.rank());
```

``` rust
use hebrew_accents::PoetryAccent;

let poetry_accent = PoetryAccent::ShalsheletGadol;
assert_eq!(6, poetry_accent.rank());
```


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


