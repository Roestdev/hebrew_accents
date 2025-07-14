# Introduction

As indicated in the README file, there is much discussion and different understandings regarding the accents. In the file [Incomplete_overview_of_the_masoretic_hebrew_accents](doc/Incomplete_overview_of_the_masoretic_hebrew_accents.ods). I have placed the insights of various scholars (whose infomation I was able to find) in a file for easy comparison.

Looking roughly I notice the following similarities and differences.

#### Similarities

- Number of Unicode code-points per accent
- Distribution and number of accents:
  
  - 26 accents in de prose books, wereof
    - 18 disjunctives and
    - 8 conjuntives
  - 22 accents poetry books, wereof
    - 12 disjunctives and
    - 10 conjuntives

  Notes: 
    - Mayela is a secondary accent
    - Some accents are in multiple groups (Helmut Righter) 

#### Differences

- The ordering of the accents
- Putting disjunctive accents into groups
  - Gesenius indicates that the division into groups originated from Samuel Bohlius (1636)
  - somes cholars 
  - contents of the groups are not identical
- The names of the accents is a mix of Ashkenazi and Sephardi (see [wikipedia](https://en.wikipedia.org/wiki/Hebrew_cantillation#Names_and_shapes_of_the_te'amim))

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, I will make a choice regarding implementation.

</br>

# Design decision

- I chose to utilize the layout outlined in the `Biblia Hebraica Stuttgartensia` which is a cornerstone of biblical scholarship,  providing essential resources for the study of the Hebrew Bible and its interpretation.

- On top of the above a correction of the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR` as mentioned by Helmut Richter will be added.

- Accent names will be according `Biblia Hebraica Stuttgartensia`, but alternate names will be provided.

- The Latin adjectives referenced in the BHS will be translated into Hebrew and subsequently transliterated into English.

# Design

Below the start of the design. Everything else will be build on top of it.

*(This section will be updated during development)*


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


## Prose books

- Two accents consist two Unicode code-points: 
  - Shalshelet
  - Legarmeh

## Poetic books

- Seven accents consist two Unicode code-points:
  - Ole We Yored
  - Revia Mugrash
  - Shalshelet Gadol
  - Mahpakh Legarmeh
  - Azla Legarmeh
  - Tsinnorit Merkha
  - Tsinnorit Mahpakh
