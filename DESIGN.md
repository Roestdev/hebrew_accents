# Introduction

As indicated in the README file, there is much discussion and different understandings regarding the accents. In the file [Incomplete_overview_of_the_masoretic_hebrew_accents](doc/Incomplete_overview_of_the_masoretic_hebrew_accents), I have put a few different views side by side, in order to see where the differences are.

Despite many similarities, I also discovered some differences between the insights from scholars, including:

- ordering of accents
- alternative names (Hebrew/Aramaic)
- grouping of disjunctives
- the same accents in different groups (H. Richter)

Given that the scholars themselves do not have a unified view and that I myself am not a scholar of biblical Hebrew at all, I will make a choice regarding implementation.

</br>

# Basis of the design

The classification of accents as described in the book by Mark D. Futato, Sr. will serve as the basis for design. The reason is that this book is recently published (2020) and it is on my bookshelf :-)

The above will be supplemented with the following:

- the accent called `Mayela`, which is mentioned in the `Biblia Hebraica Stuttgartensia` which is a cornerstone of biblical scholarship.

- a correction for the errors regarding the Unicode characters `HEBREW ACCENT ZARQA` and `HEBREW ACCENT ZINOR` as mentioned by Helmut Richter.

- some minor transliteration adaptions.


#### Note

My preference is for the ranking and group classification as used in the `Biblia Hebraica Stuttgartensia`. Unfortunately, so far I have not seen a document describing the classification of disjunctives for the prosa. Should this be the case in the future it will be incorporated into the implementation.