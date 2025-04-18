#![doc = include_str!("../README.md")]

//! Hebrew accents of the 21 books.
//!
//! DISJUNCTIVE ACCENTS
//!
//! | group | Hebrew    | English         | # code-points | comment
//! |---------------------------------------------------------------------------
//! |   1   | סִלּוּק        | silluq         |     1         |  
//! |   1   | אַתְנָח        | atnakh         |     1         |
//! |---------------------------------------------------------------------------
//! |   2   | סְגֽוֹלְתָּא      | segolta         |     1         |
//! |   2   | שַׁלְשֶׁלֶת      | shalshelet      |     2          |
//! |   2   | זָקֵף קָטוֹן     | zaqeph qaton    |     1        |
//! |   2   | זָקֵף גָּדוֹל     | zaqeph gadol    |     1         |
//! |   2   | טִפְחָא       | tiphkha         |     1          |  or טַרְחָא Ṭarkhā
//! |--------------------------------------------------------------------------
//! |   3   | רְבִיעַ        | revia           |     1         |
//! |   3   | זַרְקָא        | zarqa           |     1         |
//! |   3   | פַּשְׁטָ        | pashta          |     1         |
//! |   3   | תְּבִיר        | tevir           |     1          |
//! |   3   | יְתִיב        | yetiv            |     1         |
//! |-------------------------------------------------------------------------
//! |   4   | פָּזֵר         | pazer            | 1            |
//! |   4   | קַרְנֵי פָרָה    | qarne parah      | 1            | alt. פָּזֵר גָּדוֹל (pazer gadol)
//! |   4   | תְּלִישָׁא גְדוֹלָה  | telisha gedolah  | 1               |
//! |   4   | גֶּרֶשׁ        | geresh           | 1             | or  טֶרֶס Ṭères
//! |   4   | גְּרָשַׁ֫יִם       | gershayim        | 1             |
//! |   4   | לְגַרְמֶהּ       | legarmeh         | 2               |

//! CONJUNCTIVE ACCENTS
//!
//! | Hebrew       | English          | # code-points | comment
//! |------------------------------------------------------------
//! | מֵירְכָא        | merekha           | 1
//! | מֵירְכָא  כְפוּלָה  | merekha kephulah  | 1
//! | מוּנַח         | munakh            | 1
//! | מְהֻפָּךְ        | mehuppakh         | 1
//! | דַּרְגָּא        | darga              | 1
//! | אַזְלָא         | azla              | 1            | when associated with Gèreš also called Qadmā.
//! | תְּלִישָׁא קְטַנָּה   | telisha qetannah   | 1
//! | גַּלְגַּל         | galgal             | 1            | or יֶרַח Yèraḥ

//! Hebrew accents of the 3 books.
//!
//! DISJUNCTIVE ACCENTS
//!
//! | group | Hebrew      | English            | # code-points
//! ----------------------------------------------
//! |  1   |         סִלּוּק  | silluq             | 1
//! |  1   |     עוֹלֶה וְיוֹרֵד  | ole veyored          | 2
//! |  1   |         אַתְנָח  | atnakh          | 1
//! ־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־
//! |  2   |       רְבִיעַ  גָּדוֹל  | revia gadol        | 1
//! |  2   |      רְבִיעַ  מֻגְרָשׁ  | revia mugrash        | 2     | i.e. Rebhiaʿ with Gèreš on the same word.
//! |  2   |      שַׁלְשֶׁלֶת גָּדוֹל  | shalshelet gadol    | 2
//! |  2   |        רְבִיעַ קָטוֹן  | revia qaton        | 1
//! |  2   |            צִנּוֹר  | tsinnor             |      1
//! |  2   |             דְּחִי  | dechi               |       1
//! ־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־־
//! |  3   |            פָּזֵר  | pazer | 1
//! |  3   |       מְהֻפָּךְ לְגַרְמֶהּ | mehuppakh legarmeh | 2
//! |  3   |       אַזְלָא לְגַרְמֶהּ | azla legarmeh | 2

//! CONJUNCTIVE ACCENTS
//!
//! | Hebrew  | English | # code-points
//! |----------------------------------------------
//! | מוּנַח         | munakh  | 1
//! | מֵירְכָא        | merekha | 1
//! | עִלּוּי          | illuy | 1
//! | טַרְחָא        | tarkha | 1
//! | גַּלְגַּל         | galgal | 1 | or Yèraḥ
//! | מְהֻפָּךְ        | mehuppakh | 1
//! | אַזְלָא         | azla | 1
//! |     שַׁלְשֶׁלֶת קְטַנָּה | shalshelet qetannah | 1
//! |   צִנּוֹרִית  מֵירְכָא  | tsinnorit merekha | 2
//! |    צִנּוֹרִית מְהֻפָּךְ  | tsinnorit mehuppakh | 2

mod contains;
mod data_structures;
mod filter;
// re-export
pub use self::contains::*;
pub use self::data_structures::*;
pub use self::filter::*;
