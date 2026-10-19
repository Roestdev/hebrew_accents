# Introduction

As indicated in the README file, there is much discussion and different understandings regarding the accents. In the file [Incomplete_overview_of_the_masoretic_hebrew_accents](doc/Incomplete_overview_of_the_masoretic_hebrew_accents.ods). I have placed the insights of various scholars (whose infomation I was able to find) in a file for easy comparison.

Looking roughly I noticed the following similarities and differences.

### Similarities

- Number of Unicode code-points per accent
- Distribution and number of accents:
  
  - 26 accents in de prose books, wereof
    - 18 disjunctives and
    - 8 conjuntives
  - 22 accents poetry books, wereof
    - 12 disjunctives and
    - 10 conjuntives

  Notes: 
    - Meayla and Meteg are secondary accents
    - Some accents are in multiple groups (see p.e. Helmut Righter) 

### Differences

- The ordering of the accents
- Putting disjunctive accents into groups
  - Gesenius indicates that the division into groups originated from Samuel Bohlius (1636)
  - contents of the groups are not identical
- The names of the accents is mostly a mix of Ashkenazi and Sephardi (see [wikipedia](https://en.wikipedia.org/wiki/Hebrew_cantillation#Names_and_shapes_of_the_te'amim))

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, choices need to be made regarding the implementation.

</br>

# Design decisions

- I chose to utilize the layout as outlined in the `Biblia Hebraica Stuttgartensia` (BHS) which is a cornerstone of biblical scholarship,  providing essential resources for the study of the Hebrew Bible and its interpretation.

- Accent names will be according `BHS`, but alternate names will be provided if mentioned in the `BHS`.

- The accent `Meteg`, which is not mentioned BHS will be added to the list of both (prose and poetry) conjunctives. The Meteg is just as the Meayla a **secondary accent**.

- On top of that the groups of disjunctive accents as mentioned in the boek [Basics of Hebrew Accents](<https:://zondervanacademic.com/products/basics-of-hebrew-accents>) wil be added.

- A new classification, `PseudoAccents`, will be introduced to accommodate specific Hebrew punctuation CantillationSymbol that are closely related to the accents but do not conform to the standard Prose or Poetry accent taxonomies. 
These pseudo accents require distinct handling. The following CantillationSymbol are now categorized under PseudoAccents:

  - `SophPasuq`
  - `Maqqaph`
  - `Paseq`

## Accent Mapping to Unicode code-points

Hebrew accents as they occur in the Tanach are mostly, but not always exact the same as the accents as mentioned in the Unicode code page.

1. **One Hebrew Accent -> one code-point**

    This covers most of the cases. Examples:

    - ProseAccent::Segolta -> Unicode code-point: **U+0592**
    - PoetryAccent::Munach -> Unicode code-point: **U+05A3**

2. **One Hebrew Accent -> two code-points**

   - In the `Prose books` the following two accents consist of two(2) Unicode code-points: 
     - Shalshelet
     - Legarmeh

   - In the `Poetic books` the following seven accents consist of two(2) Unicode code-points:
     - Ole We Yored
     - Revia Mugrash
     - Shalshelet Gadol
     - Mehuppakh Legarmeh
     - Azla Legarmeh
     - Tsinnorit Merkha
     - Tsinnorit Mahpakh

3. **Two Hebrew Accents -> One code-point**

    Some accents have different names (and functions), depending of the position within the sentence.

    Examples: 
    - `Meayla` and `Tiphcha` both have the same unicode point **U+0596**
    - `Silluq` and `Meteg` both have the same unicode point **U+05BD**

    (more details can be found in the code itself)

# Design

Below the start of the design. Everything else will be build on top of it.

*(This section will be updated during development)*

## Handling the different accent mappings

In general all mappings can be resolved using either the [String](https://doc.rust-lang.org/std/string/struct.String.html#implementations[) methods or some kind of regular expression ([Regex](](https://docs.rs/regex/latest/regex/)) or [Fancy-Regex](https://docs.rs/fancy-regex/latest/fancy_regex/)).

**Except** the following four **poetry** Hebrew Accents, because they need a `flexible negative lookbehind` (*which is at the moment of writing not available as far as I know*):

1. `Merkha`  
    The accent Merkha, Ole We Yored and Tsinnorit Merkha all contain the ssame Unicode code-point **U+05A5**.

2. `Mehuppakh`   
   The accent Mehuppakh, Mehuppakh Legarmeh and Tsinnorit Mahpakh all contain the same Unicode code-point **U+05A5**.

3. `Revia Gadol`  and `Revia Qaton`
   Revia Gadol, Revia Qaton and Revia Mugrash all contain same the Unicode code-point **U+0597**.

   Allthough the Revia Gadol and Revia Qaton are represented by the same Unicode code-point, the difference is based upon the position in the sentence and the relation to another accent ( `OlehWeYored`).

## Data structures

1. **SentenceContext**   
    grouping the sentence and the context in one structure

1. **Hebrew Accent**  
    Prose(ProseAccent), Poetry(PoetryAccent), Pseudo(PseudoAccent)

2. **Prose Accent**  
   All prose accents

3. **Poetry Accent**   
   All poetry accents

4. **PseudoAccent**
   All accents that are close related Hebrew accents

5. **Context**  
   The context (writing style) of the sentence
   Prose, Poetry, Unknown

6. **Accent Type**  
   Two accenttype of accents: Primary and Secondary

7. **Accent AccentCategory**    
   Disjunctive, Conjunctive

8. **Accent Position**  
   Indication of the position of the accent in relation to the applicable consonant

9.  **Accent CodePoints**   
   The number of Unicode code-points ( 1 or 2 codepoints)

10. **Accent Information**   
   Contains all kind of attributes of the accents.
   e.g. position, accenttype etc.

## Functions

#### For `SentenceContext`

- new() -> Result<SentenceContext, SentenceContextError>
  - Creates a new SentenceContext object. User input will be validated.

- contains_accent() -> bool
  -  Checks if the given accent is present in the sentence

- find_accent() -> Option(Match)
  - Returns the position of the first found accent

#### For `ProseAccent` and `PoetryAccents`

- relative_strength() 
   - gives the relative_strength number of the accent
- const LEN (total number of accents)

## Traits

`Accent`

- details()
    - gives accent_meta_data information for a specific accent, e.g. accents position etc. etc.
- accenttype()
- category()

## Input validation

Accepted input must strictly adhere to the following criteria:

 - `First Character Constraints`
   - Must be a consonant.
   - Must not be a final consonant (e.g., Hebrew Sofit letters like ך, ם, ף, ץ, ן).

 - `Subsequent Character Constraints` 
  
   All characters following the first must belong to one of the following categories:

   - **Hebrew Script**: Any character within the Hebrew Unicode block (U+0590–U+05FF).

   - **Vertical Line**: The ASCII vertical bar (|, U+007C), sometimes used in Hebrew texts as a display substitute for the Paseq (פסיק).

   - **Layout Controls for Meteg**: CGJ, ZWNJ and  ZWJ. See [Section 9.1 of the Unicode Standard, Version 15.0.0](https://www.unicode.org/versions/Unicode15.0.0/ch09.pdf) for more information.
     
   - **Whitespace**: Space characters as defined by the Rust standard library (char::is_whitespace) **Status**: Below consideration for final implementation.

   ### TODO ==== Space like characters one can acounter in hebrew texts

   sources:

   - XXX [Hebrew Typography and Spacing Best Practices](<https://typography.guru/article/hebrew-spacing-characters"},{"description":"Common spacing characters in Hebrew digital text:\n\n1.>).

   - xxx [Hebrew Unicode Typography Guide](<https://w3.org/TR/i18n-hebrew/>)

   - xxx [Hebrew Word Boundaries and Maqaf Usage](<https://www.sbl-site.org/hebrew-bibliography/maqaf-spaces>).

   - xxx [Biblical Hebrew Text Processing Guidelines](<https://bible.academy/hebrew-text-processing>).

   - [Hebrew Document Digitization Challenges](<https://dl.acm.org/hebrew-digitization>).

   - xxx [Hebrew Cantillation and Word Division](<https://www.jewishencyclopedia.com/hebrew-cantillation>).

   - [Zero-Width Characters in Semitic Languages](<https://unicode.org/reports/tr39-semitic/>).

   - [Hebrew Digital Text Corpus Analysis](<https://github.com/hebrew-corpus/spatial-analysis>).



   ### UTF-8 and Hebrew Accents: Challenges and Inconsistencies

- Furthermore, the encoding of Hebrew accents in UTF-8 is complex, as a single accent can be represented by either one or two Unicode code-points. 

- Also the mapping of Unicode code-points to Hebrew accents is not always one-to-one, meaning that a single code-point can potentially correspond to multiple Hebrew accents, depending of the context.

- There are some Hebrew accents that are somehow related to the wrong Unicode code-point.

   | Hebrew Accent | code-point           | Name   | Note                         |
   | --------------| -------------------- | ------ | ---------------------------- |
   | ZARQA         | HEBREW ACCENT ZINOR  | U+05AE | Prose, disjuntive - Tier 3   |
   | TSINNOR       | HEBREW ACCENT ZINOR  | U+05AE | Poetry, disjunctive - Tier 2 |
   | TSINNORIT     | HEBREW ACCENT ZARQA  | U+0598 | Poetry, conjunctive          |

For more information see [mechon-mamre.org](https://mechon-mamre.org/c/hr/unicode.htm) and [Unicode-Appendix A](https://www.unicode.org/notes/tn27/#Appendix_A)


