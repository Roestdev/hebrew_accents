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
    - Mayela and Meteg are secondary accents
    - Some accents are in multiple groups (see p.e. Helmut Righter) 

### Differences

- The ordering of the accents
- Putting disjunctive accents into groups
  - Gesenius indicates that the division into groups originated from Samuel Bohlius (1636)
  - contents of the groups are not identical
- The names of the accents is mostly a mix of Ashkenazi and Sephardi (see [wikipedia](https://en.wikipedia.org/wiki/Hebrew_cantillation#Names_and_shapes_of_the_te'amim))

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, choices needs to be made regarding the implementation.

</br>

# Design decision

- I chose to utilize the layout as outlined in the `Biblia Hebraica Stuttgartensia` (BHS) which is a cornerstone of biblical scholarship,  providing essential resources for the study of the Hebrew Bible and its interpretation.

- On top of the above a correction of the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR`, as mentioned by Helmut Richter will be added.

- Accent names will be according `Biblia Hebraica Stuttgartensia`, but alternate names will be provided if applicable.

- The Latin adjectives referenced in the BHS will be translated into Hebrew and subsequently transliterated into English.

- The secondary accent Meteg,which is not menttioned BHS will be added to the list of both conjunctives (prose and poetry).

### UTF-8 and Hebrew Accents: Challenges and Inconsistencies

- The representation of Hebrew accents in UTF-8 is not without its challenges. One major issue is the inconsistent definition of two accent marks in the Tanach, which has led to confusion and potential errors. These marks are:

   -  The Yetiv mark
   -  The Tevir mark

  In various sources, the Yetiv mark is classified as a conjunctive accent, while the Tevir mark is considered a disjunctive accent. However, the Unicode tables seem to reverse this definition, treating the Yetiv mark as a disjunctive accent and the Tevir mark as a conjunctive accent.

- Furthermore, the encoding of Hebrew accents in UTF-8 is complex, as a single accent can be represented by either one or two Unicode code-points. 

- Additionally, the mapping of Unicode code-points to Hebrew accents is not always one-to-one, meaning that a single code-point can potentially correspond to multiple Hebrew accents, depending of the context.

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

</br>

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

   Allthough the Revia Gadol and Revia Qaton are represented by the same Unicode code-point, the difference is based upon the position in the sentence and the relation to another accent ( `OleWeYored`).

## Data structures

1. **SentenceContext**   
    grouping the sentence and the context

1. **Hebrew Accent**  
    Prose(ProseAccent), Poetry(PoetryAccent)

2. **Prose Accent** 
   All prose accents

3. **Poetry Accent**   
    All poetry accents

4. **Context** 
   The context (writing style) of the sentence
   Prose, Poetry

5. **Accent Type**  
   Two type of accents: Primairy and Secundary

6. **Accent Category**    
   Disjunctive, Conjunctive

7. **Accent Position**
   Indication of the position of the accent in relation to the applicable consonant

8. **Accent CodePoints**   
   The number of Unicode code-points

9.  **Accent Information**   
   Contains all kind of attributes of the accents.
   e.g. position, type etc. etc.

## Functions

#### For `SentenceContext`

- new() 
  - creates a new SentenceContext object

- contains_accent() 
  -  checks if the accent is present in the sentence

- find_accent() **TODO**
  - returns the position of the first found accent

#### For `ProseAccent` and `PoetryAccents`

- rank() 
   - gives the rank number of the accent

- info() **TODO**
    - gives additional information for a specific accent, e.g. accents position etc. etc.

