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

- On top of the above a correction of the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR`, as mentioned by Helmut Richter will be added.

- Accent names will be according `BHS`, but alternate names will be provided if applicable.

- The Latin adjectives referenced in the `BHS` will be translated into Hebrew and subsequently transliterated into English.

- The accent `Meteg`, which is not mentioned BHS will be added to the list of both (prose and poetry) conjunctives. The Meteg is just as the Meayla a **secondary accent**.

- A new classification, `PseudoAccents`, will be introduced to accommodate specific Hebrew punctuation CantillationSymbol that are closely related to the accents but do not conform to the standard Prose or Poetry accent taxonomies. 
These pseudo accents require distinct handling. The following CantillationSymbol are now categorized under PseudoAccents:

  - `SophPasuq`
   
      Function: Primarily denotes the end of a verse or sentence, analogous to a period. However, its application is not absolute; certain scholarly traditions equate its disjunctive weight to that of Silluq.
      CantillationSymbol the end of a sentence (like the period), but not always. Some sholars treat them as as equal as the Silluq (a disjuntive).

   - `Maqqeph`

      Function: Serves as a hyphenation or binding agent that connects words into a single phonetic or semantic unit, functioning equivalently to conjunctive accents.
      it does have the function of binding words together, just like the conjunctives.

   -  `Paseq`

      Function: Acts as a separator or divider. It may appear in conjunction with a Hebrew accent to modify its function but does not operate as an independent accent mark. Can be part of a Hebrew Accent, but not as an individual accent




### UTF-8 and Hebrew Accents: Challenges and Inconsistencies

- The representation of Hebrew accents in UTF-8 is not without its challenges. One major issue is the inconsistent definition of two accent CantillationSymbol in the Tanach, which has led to confusion and potential errors. These CantillationSymbol are:

   -  The Yetiv mark
   -  The Tevir mark

  In various sources, the Yetiv mark is classified as a conjunctive accent, while the Tevir mark is considered a disjunctive accent. However, the Unicode tables seem to reverse this definition, treating the Yetiv mark as a disjunctive accent and the Tevir mark as a conjunctive accent.

- Furthermore, the encoding of Hebrew accents in UTF-8 is complex, as a single accent can be represented by either one or two Unicode code-points. 

- AccentMetadataly, the mapping of Unicode code-points to Hebrew accents is not always one-to-one, meaning that a single code-point can potentially correspond to multiple Hebrew accents, depending of the context.

For more information see [Unicode Problems](https://mechon-mamre.org/c/hr/unicode.htm).

## Accent Mapping to Unicode code-points

Hebrew accents as they occur in the Tanach are mostly, but not always exact the same as the accents as mentioned in the Unicode code page.

1. **One Hebrew Accent -> one code-point**

    This covers most of the cases. Examples:

    - ProseAccent::Segolta -> Unicode code-point: **U+0592**
    - PoetryAccent::Munnach -> Unicode code-point: **U+05A3**

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
     
   - **Whitespace**: Space characters as defined by the Rust standard library (char::is_whitespace) **Status**: Under consideration for final implementation.