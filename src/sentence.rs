#![allow(unused)]

use regex::Regex;

use crate::HebrewAccent;
use crate::PoetryAccent;
use crate::ProseAccent;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] //,Display
pub enum Context {
    Poetic,
    #[default]
    Prosaic,
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Copy,Display
pub struct SentenceContext {
    sentence: String,
    context: Context,
}

// functions for 21 books
fn contains_silluq(sc: &SentenceContext) -> bool {
    // A METEG in the last word of a sentence is called SILLUQ (\u{05BD})
    // Technically speaking are the METEG and SILLUQ equal.
    // Some times a sentence ends with SOF PASUQ (\u{05C3})
    // Some times a sentence ends with "samech" (U+05E1) or an "pey" (U+05E4).
    //
    // The following lines will give a positive result
    // וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם ׃
    // וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם
    // וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם פ
    // וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם ׃ פ
    let re_silluq = Regex::new(r"\u{05BD}\p{Hebrew}*?\s*?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$").unwrap();
    re_silluq.is_match(&sc.sentence)
}

fn contains_shalshelet(sc: &SentenceContext) -> bool {
    // A SHALSHELET consists of the following two UTF-8 code-points (p.e. Gen19:16)
    //      - SHALSHELET (\u{0593}) followed by
    //      - PASEQ (\u{05C0})
 // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_shalshelet = Regex::new(r"\u{0593}\p{Hebrew}+\s?[\u{05C0}\u{007C}]").unwrap();
    re_shalshelet.is_match(&sc.sentence)

}

fn contains_tiphcha(sc: &SentenceContext) -> bool {
    // TODO
    /*
        Mayela is not a true conjunctive accent.
        Mayela only appears in the same word stressed by Athnach or Silluq. 
        For example, Leviticus 21:4.
    */
    // atnach = \u{0596}
    // silluq = meteg in last word
    // 
    let re_silluq = Regex::new(r"\s\w*\u{05BD}\w*\s?\u{05C3}?\s?\w?$").unwrap();
    re_silluq.is_match(&sc.sentence)
}

fn contains_legarmeh(sc: &SentenceContext) -> bool {
    // A 'Legarmeh' consists of the following two UTF-8 code-points:
    //      - MUNACH (\u{05A3}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_legarmeh = Regex::new(r"\u{05A3}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_legarmeh.is_match(&sc.sentence)
}

fn contains_meayela(sc: &SentenceContext) -> bool {
    // TODO
    /*
        Mayela is not a true conjunctive accent.
        Mayela only appears in the same word stressed by Athnach or Silluq. 
        For example, Leviticus 21:4.
    */
    // Tiphcha and (Atnach or Silluq) in one word
    let re_tiphcha_atnach_ = Regex::new(r"\u{0596}\p{Hebrew}*\u{0591}").unwrap();
    //re_tiphcha_atnach_.is_match( &sc.sentence)
    let re_atnach_tiphcha = Regex::new(r"\u{0591}\p{Hebrew}*\u{0596}").unwrap();
    //re_atnach_tiphcha.is_match( &sc.sentence)
    let re_1 = Regex::new(r"\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s*?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$").unwrap();
    true
}

// functions for 3 books
fn contains_ole_we_yored(sc: &SentenceContext) -> bool {
    // A Ole We Yored consists of the following two UTF-8 code-points 
    //      - OLE (\u{05AB}) followed by
    //      - YORED (\u{05A5})
    // This accent can stretch over two words (a.k.a. word-unit)

    let re_ole_we_yored = Regex::new(r"\u{0598}\p{Hebrew}+\s?\p{Hebrew}*\u{05AB}").unwrap();
    re_ole_we_yored.is_match(&sc.sentence)
}
fn contains_revia_gadol(sc: &SentenceContext) -> bool {
    // not before oleweyored
true
}
fn contains_revia_mugrash(sc: &SentenceContext) -> bool {
    // A 'Revia Mugrash' consists of the following two UTF-8 code-points:
    //      - GERESH (\u{059C}) followed by
    //      - REVIA (\u{0597})

    let re_revia_mugrash = Regex::new(r"\u{059C}\p{Hebrew}+\u{0597}").unwrap();
    re_revia_mugrash.is_match(&sc.sentence)

}
fn contains_shalshelet_gadol(sc: &SentenceContext) -> bool {
    // A 'Shalshelet Gadol' consists of the following two UTF-8 code-points:
    //      - SHALSHELET (\u{0593}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_mehuppakh_legarmeh = Regex::new(r"\u{05A4}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_mehuppakh_legarmeh.is_match(&sc.sentence)
}

fn contains_revia_gaton(sc: &SentenceContext) -> bool {
    // only before olweyored
true
}

fn contains_mehuppakh_legarmeh(sc: &SentenceContext) -> bool {
    // A 'Mehuppakh Legarmeh' consists of the following two UTF-8 code-points:
    //      - MEHUPPAKH (\u{05A4}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_mehuppakh_legarmeh = Regex::new(r"\u{05A4}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_mehuppakh_legarmeh.is_match(&sc.sentence)
}

fn contains_azla_legarmeh(sc: &SentenceContext) -> bool {
    // An 'Alza Legarmeh' consists of the following two UTF-8 code-points 
    //      - AZLA (\u{05A8}) followed by
    //      - PASEQ (\u{05C0})
        // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
let re_alza_legarmeh = Regex::new(r"\u{05A8}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_alza_legarmeh.is_match(&sc.sentence)
}


fn contains_tsinnorit_merkha(sc: &SentenceContext) -> bool {
    // A Tsinnorit Merkha consists of the following two UTF-8 code-points 
    //      - TSINNORIT (\u{0598}) followed by
    //      - MERKHA (\u{05A5})
    // This accent can occur in one or two words (a.k.a. word-unit)

    let re_tsin_mer = Regex::new(r"\u{0598}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap();
    re_tsin_mer.is_match(&sc.sentence)
}

fn contains_tsinnorit_mehuppakh(sc: &SentenceContext) -> bool {
    // A Tsinnorit Mehuppakh consists of the following two UTF-8 code-points 
    //      - TSINNORIT (\u{0598}) followed by
    //      - MEHUPPAKH (\u{05A4})
    // This accent can occur in one or two words (a.k.a. word-unit)

    let re_tsin_mehu = Regex::new(r"\u{0598}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap();
    re_tsin_mehu.is_match(&sc.sentence)
}

impl SentenceContext {
    #[allow(dead_code)]
    fn new(sentence: &str, context: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            context,
        }
    }

    fn contains_accent(&self, accent: HebrewAccent) -> bool {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/

            // Disjunctives
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Silluq) | HebrewAccent::Three(PoetryAccent::Silluq) => {
                contains_silluq(self)
            } 
            // TODO- test
            HebrewAccent::TwentyOne(ProseAccent::Atnach)| HebrewAccent::Three(PoetryAccent::Atnach) => {
                self.sentence.contains('\u{0591}')
            }
            HebrewAccent::TwentyOne(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0592}')
            }
            // TODO test
            HebrewAccent::TwentyOne(ProseAccent::Shalshelet) | HebrewAccent::Three(PoetryAccent::ShalsheletGadol) =>
            {
                contains_shalshelet(self)
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephQatan)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{0594}')
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephGadol)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{0595}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Revia) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0597}')
            }
            // TODO - test regex, meayla
            HebrewAccent::TwentyOne(ProseAccent::Tiphcha) if self.context == Context::Prosaic => {
                contains_tiphcha(self)
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Zarqa) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05AE}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Pashta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0599}')
            } // check needed
            HebrewAccent::TwentyOne(ProseAccent::Yetiv) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059A}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Tevir) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059B}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Geresh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059C}')
            }
            HebrewAccent::TwentyOne(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059E}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Pazer) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A1}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::PazerGadol)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{0591}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A0}')
            }
            // TODO - regex , test ,
            HebrewAccent::TwentyOne(ProseAccent::Legarmeh) if self.context == Context::Prosaic => {
                contains_legarmeh(self)
            }

            // Conjunctives
            // TODO - regex, NOT legarmeh
            HebrewAccent::TwentyOne(ProseAccent::Munnach) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A3}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A4}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Merkha) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A5}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A6}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Darga) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0000}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Azla) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A8}')
            }
           // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05AA}')
            }
            // TODO - test
            HebrewAccent::TwentyOne(ProseAccent::Galgal) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{005A3}')
            }
            // TODO - test, regex + tiphcha
            HebrewAccent::TwentyOne(ProseAccent::Meayela) if self.context == Context::Prosaic => {
                contains_meayela(self)
            }

           /* **********************************************************
             *                          POETRY
             * *********************************************************/

            // Disjunctives
            // TODO - same as for prose
            //HebrewAccent::Three(PoetryAccent::Silluq) if self.context == Context::Poetic => true, // regex needed
            // TODO  test
            HebrewAccent::Three(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                contains_ole_we_yored(self)
            }
            // TODO - same as for prose
            // HebrewAccent::Three(PoetryAccent::Atnach) if self.context == Context::Poetic => {
            //     self.sentence.contains('\u{0591}')
            // }
            // TODO - regex test
            HebrewAccent::Three(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                contains_revia_gadol(self)
            } 
            // TODO - regex test
            HebrewAccent::Three(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                contains_revia_mugrash(self)
            } 
            // TODO - regex test
            HebrewAccent::Three(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetic =>
            {
                contains_shalshelet_gadol(self)
            }
            // TODO - test
            HebrewAccent::Three(PoetryAccent::Tsinnor) if self.context == Context::Poetic =>  {
                self.sentence.contains('\u{05AE}')
            },
            // TODO - regex + test
            HebrewAccent::Three(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                contains_revia_gaton(self)
            }
            // TODO - test
            HebrewAccent::Three(PoetryAccent::Dechi) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AD}')
            },
            // TODO - test
            HebrewAccent::Three(PoetryAccent::Pazer) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A1}')
            },
            // TODO - regex
            HebrewAccent::Three(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                contains_mehuppakh_legarmeh(self)
            } 
            // TODO - regex
            HebrewAccent::Three(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                contains_azla_legarmeh(self)
            }
            ///////////////////////
            // Conjunctives
            HebrewAccent::Three(PoetryAccent::Munnach) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A3}')
            }
            // TODO - but NOT TsinnoritMerkha -> regex needed
            HebrewAccent::Three(PoetryAccent::Merkha) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
                // contains_mercha(self)
            } 
            HebrewAccent::Three(PoetryAccent::Illuy) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
            }
            HebrewAccent::Three(PoetryAccent::Tarkha) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
            }
            HebrewAccent::Three(PoetryAccent::Galgal) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
            } 
            // TODO - but NOT mehuppakh legarmeh -> regex needed
            HebrewAccent::Three(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
            } 
            // TODO - but NOT azla legarmeh -> regex needed
            HebrewAccent::Three(PoetryAccent::Azla) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
            }
            // TODO  - test
            HebrewAccent::Three(PoetryAccent::ShalsheletQetannah)if self.context == Context::Poetic =>
            {
                self.sentence.contains('\u{05A3}')
            }
            // TODO - test regex
            HebrewAccent::Three(PoetryAccent::TsinnoritMerkha) if self.context == Context::Poetic =>
            {
                contains_tsinnorit_merkha(self)

            } // TODO - test regex
            HebrewAccent::Three(PoetryAccent::TsinnoritMahpakh) if self.context == Context::Poetic =>
            {
                contains_tsinnorit_mehuppakh(self)
            }
            _ => false,
        }
    }

    fn find_accent(&self, accent: HebrewAccent) -> Option<usize> {
        match accent {
            // Prose Disjunctives
            HebrewAccent::TwentyOne(ProseAccent::Silluq) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Atnach) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Shalshelet)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephQatan)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephGadol)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Revia) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Tiphcha) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Zarqa) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Pashta) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Yetiv) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Tevir) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Geresh) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Pazer) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::PazerGadol)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Legarmeh) if self.context == Context::Prosaic => {
                Some(1)
            }
            // Prose Conjunctives
            HebrewAccent::TwentyOne(ProseAccent::Munnach) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Merkha) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Darga) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Azla) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Galgal) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Meayela) if self.context == Context::Prosaic => {
                Some(1)
            }
            // Poetry Disjunctives
            HebrewAccent::Three(PoetryAccent::Silluq) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Atnach) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetic =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Tsinnor) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Dechi) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::Pazer) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                Some(1)
            }
            // Poetry Conjunctives
            HebrewAccent::Three(PoetryAccent::Munnach) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Merkha) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::Illuy) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::Tarkha) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::Galgal) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Azla) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Three(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                Some(1)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain1() {
        let newsc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ פ",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::TwentyOne(ProseAccent::Silluq)));
        //assert!(newsc.contains_accent(HebrewAccent::TwentyOne(ProseAccent::Meayela)));
    }
    #[test]
    fn test_contain2() {
        let newsc = SentenceContext::new("וַיִּתְמַהְמָ֓הּ | וַיַּֽחֲזִ֨יקוּ", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::TwentyOne(ProseAccent::Shalshelet)));
    }
    #[test]
    fn test_find() {
        let newsc = SentenceContext::new("gad", Context::Prosaic);
        assert_eq!(
            newsc.find_accent(HebrewAccent::TwentyOne(ProseAccent::Galgal)),
            Some(1)
        );
        assert_eq!(
            newsc.find_accent(HebrewAccent::TwentyOne(ProseAccent::Atnach)),
            Some(1)
        );
        assert_eq!(
            newsc.find_accent(HebrewAccent::Three(PoetryAccent::AzlaLegarmeh)),
            None
        );
    }
}
