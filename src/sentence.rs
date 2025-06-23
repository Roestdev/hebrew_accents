#![allow(unused)]
use regex::Regex;
use crate::HebrewAccent;
use crate::ProseAccent;
use crate::PoetryAccent;

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
            HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                contains_silluq(self)
            }
            HebrewAccent::Prose(ProseAccent::Atnach) | HebrewAccent::Poetry(PoetryAccent::Atnach) => {
                self.sentence.contains('\u{0591}')
            }
            // TODO- test
            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0592}')
            }
            // TODO test
            HebrewAccent::Prose(ProseAccent::Shalshelet)
            | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) => contains_shalshelet(self),
            // TODO test
            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0594}')
            }
            // TODO test
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0595}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0597}')
            }
            // TODO - test regex, meayla
            HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => {
                contains_tiphcha(self)
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05AE}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0599}')
            } // check needed
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059A}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059B}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059C}')
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059E}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A1}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0591}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A0}')
            }
            // TODO - regex , test ,
            HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => {
                contains_legarmeh(self)
            }

            // Conjunctives
            // TODO - regex, NOT legarmeh
            HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A3}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A4}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A5}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A6}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0000}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A8}')
            } 
            // TODO - test
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05AA}')
            }
            // TODO - test
            HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{005A3}')
            }
            // TODO - test, regex + tiphcha
            HebrewAccent::Prose(ProseAccent::Meayela) if self.context == Context::Prosaic => {
                contains_meayela(self)
            }

            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            // TODO - same as for prose
            //HebrewAccent::PoetryAccent::Silluq) if self.context == Context::Poetic => true, // regex needed

            // TODO  test
            HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                contains_ole_we_yored(self)
            }
            // TODO - same as for prose
            // HebrewAccent::PoetryAccent::Atnach) if self.context == Context::Poetic => {
            //     self.sentence.contains('\u{0591}')
            // }

            // TODO - regex  (NO revia gadol/revi qaton) test
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                contains_revia_gadol(self)
            }
            // TODO - regex  (NO revia gadol/revi qaton) test
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                contains_revia_mugrash(self)
            }
            // TODO - regex test
            // HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) if self.context == Context::Poetic => {
            //     contains_shalshelet_gadol(self)
            // }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AE}')
            }
            // TODO - regex + test (NO revia gadol/revi mugrash)
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                contains_revia_gaton(self)
            }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AD}')
            }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Pazer) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A1}')
            }
            // TODO - regex + test
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh) if self.context == Context::Poetic => {
                contains_mehuppakh_legarmeh(self)
            }
            // TODO - regex + test
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                contains_azla_legarmeh(self)
            }
            ///////////////////////
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munnach) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A3}')
            }
            // TODO - but NOT TsinnoritMerkha -> regex needed
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A5}')
                // contains_mercha(self)
            }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AC}')
            }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Tarkha) if self.context == Context::Poetic => {
                self.sentence.contains('\u{0596}')
            }
            // TODO - test
            HebrewAccent::Poetry(PoetryAccent::Galgal) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AA}')
            }
            // TODO - but NOT mehuppakh legarmeh -> regex needed
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => {
                contains_mehuppakh(self)
                //self.sentence.contains('\u{05A5}')
            }
            // TODO - but NOT azla legarmeh -> regex needed
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => {
                contains_azla(self)
                //self.sentence.contains('\u{05A5}')
            }
            // TODO  - BUT NOT a shalshelet gadol
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah) if self.context == Context::Poetic => {
                contains_shalshelet_qetannah(self)
                //self.sentence.contains('\u{05A3}')
            }
            // TODO - test regex
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha) if self.context == Context::Poetic => {
                contains_tsinnorit_merkha(self)
            } // TODO - test regex
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh) if self.context == Context::Poetic => {
                contains_tsinnorit_mehuppakh(self)
            }
            _ => false,
        }
    }

    fn find_accent(&self, accent: HebrewAccent) -> Option<usize> {
        match accent {
            // Prose Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Atnach) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => Some(1),
            // Prose Conjunctives
            HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.context == Context::Prosaic => {
                Some(1)
            }
            HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => Some(1),
            HebrewAccent::Prose(ProseAccent::Meayela) if self.context == Context::Prosaic => Some(1),
            // Poetry Disjunctives
            HebrewAccent::Poetry(PoetryAccent::Silluq) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Atnach) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Pazer) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => Some(1),
            // Poetry Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munnach) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Tarkha) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Galgal) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => Some(1),
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha) if self.context == Context::Poetic => {
                Some(1)
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh) if self.context == Context::Poetic => {
                Some(1)
            }
            _ => None,
        }
    }
}

/*
    functions for 21 books
*/
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
    let re_silluq =
        Regex::new(r"\u{05BD}\p{Hebrew}*?\s*?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$").unwrap();
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
    let re_1 = Regex::new(
        r"\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s*?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$",
    )
    .unwrap();
    true
}

/*
functions for 3 book
*/
fn contains_ole_we_yored(sc: &SentenceContext) -> bool {
    // A Ole We Yored consists of the following two UTF-8 code-points
    //      - OLE (\u{05AB}) followed by
    //      - YORED (\u{05A5}) aka MERKHA
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

fn contains_mehuppakh(sc: &SentenceContext) -> bool {
    // TODO
    let re_mehuppakh_legarmeh = Regex::new(r"\u{05A4}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_mehuppakh_legarmeh.is_match(&sc.sentence)
}
fn contains_azla(sc: &SentenceContext) -> bool {
    // TODO
    let re_mehuppakh_legarmeh = Regex::new(r"\u{05A4}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_mehuppakh_legarmeh.is_match(&sc.sentence)
}
fn contains_shalshelet_qetannah(sc: &SentenceContext) -> bool {
    // TODO
    let re_mehuppakh_legarmeh = Regex::new(r"\u{05A4}T\w+\s?[\u{05C0}\u{007C}]").unwrap();
    re_mehuppakh_legarmeh.is_match(&sc.sentence)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_prose_silluq() {
        let prosaic_sc1 = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ",
            Context::Prosaic,
        );
        assert!(prosaic_sc1.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        let prosaic_sc1 = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ פ",
            Context::Prosaic,
        );
        assert!(prosaic_sc1.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        let prosaic_sc1 = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert!(prosaic_sc1.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        let prosaic_sc1 = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        assert!(prosaic_sc1.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
    }
    #[test]
    fn test_contains_prose_atnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_zaqeph_qatan() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_revia() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_munnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_darga() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_azla() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_telisha_galgal() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_meayela() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_silluq() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_ole_we_yored() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_atnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_mehuppakh_legarmeh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_illuy() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_tarkha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_mehuppach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_azla() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mercha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mehuppach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }

    #[test]
    fn test_find() {
        let newsc = SentenceContext::new("gad", Context::Prosaic);
        assert_eq!(
            newsc.find_accent(HebrewAccent::Prose(ProseAccent::Galgal)),
            Some(1)
        );
        assert_eq!(
            newsc.find_accent(HebrewAccent::Prose(ProseAccent::Atnach)),
            Some(1)
        );
        assert_eq!(
            newsc.find_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)),
            None
        );
    }
}
