# Introduction

As indicated in the README file, there is much discussion and different understandings regarding the accents. In the file [Incomplete_overview_of_the_masoretic_hebrew_accents](doc/Incomplete_overview_of_the_masoretic_hebrew_accents.ods). I have placed the insights of various scholars (whose infomation I was able to find) in a file for easy comparison.

Looking roughly I notice the following similarities and differences.

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
    - Some accents are in multiple groups (Helmut Righter) 

### Differences

- The ordering of the accents
- Putting disjunctive accents into groups
  - Gesenius indicates that the division into groups originated from Samuel Bohlius (1636)
  - contents of the groups are not identical
- The names of the accents is mostly a mix of Ashkenazi and Sephardi (see [wikipedia](https://en.wikipedia.org/wiki/Hebrew_cantillation#Names_and_shapes_of_the_te'amim))

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, choices needs to be made regarding the implementation.

</br>

# Design decision

- I chose to utilize the layout as outlined in the `Biblia Hebraica Stuttgartensia` which is a cornerstone of biblical scholarship,  providing essential resources for the study of the Hebrew Bible and its interpretation.

- On top of the above a correction of the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR` as mentioned by Helmut Richter will be added.

- Accent names will be according `Biblia Hebraica Stuttgartensia`, but alternate names will be provided.

- The Latin adjectives referenced in the BHS will be translated into Hebrew and subsequently transliterated into English.

## Accent Mapping to Unicode code-points

Hebrew accents as they occur in the Tanach are mostly, but not always exact the same as the accents as mentioned in the Unicode code page.

#### One Hebrew Accent -> one code-point

This covers most of the cases. Examples: 
 - ProseAccent::Segolta -> Unicode code-point: **U+0592**
 - PoetryAccent::Munnach -> Unicode code-point: **U05A3**

#### One Hebrew Accent -> two code-points

- In the `Prose books` the following two accents consist of two(2) Unicode code-points: 
  - Shalshelet
  - Legarmeh

- In the `Poetic books` the following seven accents consist of two(2) Unicode code-points:
  - Ole We Yored
  - Revia Mugrash
  - Shalshelet Gadol
  - Mahpakh Legarmeh
  - Azla Legarmeh
  - Tsinnorit Merkha
  - Tsinnorit Mahpakh

#### Two Hebrew Accents -> One code-point

Examples: 
- `Mayla` and `Tiphcha` both have the same unicode point **U+0596**
- `Silluq` and `Meteg` both have the same unicode point **U+05BD**

(more details can be found in the code itself)

# Design

Below the start of the design. Everything else will be build on top of it.

*(This section will be updated during development)*

## Handling the different accent mappings

In general all mappings can be resolved using either the [String](https://doc.rust-lang.org/std/string/struct.String.html#implementations[) methods or some kind of regular expression ([Regex](](https://docs.rs/regex/latest/regex/)) or [Fancy-Regex](https://docs.rs/fancy-regex/latest/fancy_regex/)).


**Except** the following four Hebrew Accents, because they need a `flexible negative lookbehind`, which is at the moment of writing not available as far as I know.

1. Merkha  
The accent `Merkha`, `OleWeYored` and `TsinnoritMerkha` all contain the unicode point **U+05A5**.
2. Mahpakh
3. Revia Gadol
4. Revia Qaton



## Data structures

1. **Enum Context**: Prose, Poetry
   
2. **Enum AccentType**: Disjunctive, Conjunctive

3. **Struct AccentAttributes**:
   - name_transl (String)
   - name_hebrew (String)
   - ashkenazi_transl (Option<String>)
   - Ashkenazi_hebrew (Option<String>)
   - Sephardi_transl (Option<String>)
   - Sephardi_hebrew (Option<String>)
   - horizontal_position (enum)
   - vertical_position (enum)
   - order (u8)
   - comment (Option<String>)
   - group_nr (Option<u8>) 

4. **Enum Vertical position**:   
   - top
   - bottom

5. **Enum Horizontal position:**
   - postpositive (left)
   - normal (middle)
   - prepositive (right)



### Merkha

The accent `Merkha`, `OleWeYored` and `TsinnoritMerkha` all contain the unicode point **U+05A5**. 

In order to find a Merkha  ('one code-point accent') we need to check if it is *not* part of a 'two code-point' accent. 

need a flexible negative lookbehind if the , which is at the moment of writing not covered by the crate fancy-regex.