# Introduction

As indicated in the README file, there is much discussion and different understandings regarding the accents. In the file [Incomplete_overview_of_the_masoretic_hebrew_accents](doc/Incomplete_overview_of_the_masoretic_hebrew_accents). In that file, I have placed the view of a few different scholars side by side for simple comparison.

Despite many similarities, I also discovered some differences between the insights from scholars, including:

- the order of the accents
- putting disjunctive accents into groups
  - Gesenius indicates that the division into groups originated from Samuel Bohlius (1636)
  - contents of the groups are not identical

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, I will make a choice regarding implementation.

</br>

# Basis of the design

I decided too use the layout as  mentioned in the `Biblia Hebraica Stuttgartensia` which is a cornerstone of biblical scholarship,  providing essential resources for the study of the Hebrew Bible and its interpretation.

On top of that a correction of the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR` as mentioned by Helmut Richter will be added.

### Note

The ordering of the accent used in the  `Biblia Hebraica Stuttgartensia` is very similar to the ones from `Gesenius`, `William Wickes` and `James D. Price`.

For now the division into groups of the disjunctives will not be implemented, because they were added not so long ago and scholar views differ too much.