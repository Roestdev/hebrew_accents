#![allow(unused)]
use crate::HebrewAccent;
use crate::PoetryAccent;
use crate::ProseAccent;

use fancy_regex::Regex as FancyRegex;
use regex::Regex;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] //,Display
pub enum Context {
    Poetic,
    #[default]
    Prosaic,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Copy, Display
pub struct SentenceContext {
    pub sentence: String,
    pub context: Context,
}

impl SentenceContext {
    #[allow(dead_code)]
    /// Checks if the given character is a HBR accent segol.
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::Context;
    /// use hebrew_accents::SentenceContext;
    ///
    /// let sentence_context = SentenceContext::new( "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ", Context::Prosaic);
    /// assert_eq!(sentence_context.context,Context::Prosaic);
    /// assert_eq!(sentence_context.sentence,"וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ".to_string());
    /// ```
    pub fn new(sentence: &str, context: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            context,
        }
    }

    /// Checks the creation a neew instance of SentenContext.
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::ProseAccent;
    /// use hebrew_accents::Context;
    /// use hebrew_accents::HebrewAccent;
    /// use hebrew_accents::SentenceContext;
    ///
    /// let sentence_context = SentenceContext::new("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ",Context::Prosaic,);
    /// assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
    /// ```
    pub fn contains_accent(&self, accent: HebrewAccent) -> bool {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => contains_silluq(self),

            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self.sentence.contains('\u{0591}'),

            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0592}')
            }
            HebrewAccent::Prose(ProseAccent::Shalshelet)
            | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) => contains_shalshelet(self),

            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0594}')
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0595}')
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0597}')
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => {
                contains_tiphcha(self)
            }
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0598}')
            }
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0599}')
            }
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059A}')
            }
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059B}')
            }
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059C}')
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059E}')
            }
            HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A1}')
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0591}')
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A0}')
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => {
                contains_legarmeh(self)
            }

            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => {
                contains_munnach(self)
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A4}')
            }
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A5}')
            }
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A6}')
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A7}')
            }
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{05A8}')
            }
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A9}')
            }
            HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{005AA}')
            }
            HebrewAccent::Prose(ProseAccent::Meayela) if self.context == Context::Prosaic => {
                contains_meayela(self)
            }

            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            // HebrewAccent::Poetry(PoetryAccent::Silluq)  --> see PROSE accent Silluq
            HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                contains_ole_we_yored(self)
            }
            // HebrewAccent::Poetry(PoetryAccent::Atnach)  --> see PROSE accent Atnach

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
                contains_revia_qaton(self)
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
            HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                contains_mahpakh_legarmeh(self)
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
                contains_merkha(self)
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
            // TODO - but NOT Mahpakh legarmeh -> regex needed
            HebrewAccent::Poetry(PoetryAccent::Mahpakh) if self.context == Context::Poetic => {
                contains_mahpakh(self)
            }
            // TODO - but NOT azla legarmeh -> regex needed
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => {
                contains_azla(self)
            }
            // TODO  - BUT NOT a shalshelet gadol
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                contains_shalshelet_qetannah(self)
            }
            // TODO - test regex
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                contains_tsinnorit_merkha(self)
            } // TODO - test regex
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                contains_tsinnorit_mahpakh(self)
            }
            _ => false,
        }
    }

    // /// Checks if the given character is a HBR accent segol.
    // ///
    // /// # Example
    // /// ```
    // /// let prosaic_sc1 = SentenceContext::new( "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ",  Context::Prosaic, );
    // ///        assert!(prosaic_sc1.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
    // /// ```
    // pub fn find_accent(&self, accent: HebrewAccent) -> Option<usize> {
    //     match accent {
    //         // Prose Disjunctives
    //         HebrewAccent::Prose(ProseAccent::Silluq) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Atnach) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Shalshelet) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::TelishaGedolah)
    //             if self.context == Context::Prosaic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         // Prose Conjunctives
    //         HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::MerkhaKephulah)
    //             if self.context == Context::Prosaic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::TelishaQetannah)
    //             if self.context == Context::Prosaic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => Some(1),
    //         HebrewAccent::Prose(ProseAccent::Meayela) if self.context == Context::Prosaic => {
    //             Some(1)
    //         }
    //         // Poetry Disjunctives
    //         HebrewAccent::Poetry(PoetryAccent::Silluq) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Atnach) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
    //             if self.context == Context::Poetic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => Some(1),
    //         HebrewAccent::Poetry(PoetryAccent::Pazer) if self.context == Context::Poetic => Some(1),
    //         HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)
    //             if self.context == Context::Poetic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         // Poetry Conjunctives
    //         HebrewAccent::Poetry(PoetryAccent::Munnach) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => Some(1),
    //         HebrewAccent::Poetry(PoetryAccent::Tarkha) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Galgal) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Mahpakh) if self.context == Context::Poetic => {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => Some(1),
    //         HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
    //             if self.context == Context::Poetic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
    //             if self.context == Context::Poetic =>
    //         {
    //             Some(1)
    //         }
    //         HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
    //             if self.context == Context::Poetic =>
    //         {
    //             Some(1)
    //         }
    //         _ => None,
    //     }
    // }
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
        Regex::new(r"\u{05BD}{1}\p{Hebrew}*?\s*?\u{05C3}{1}?\s?[\u{05E4}\u{05E1}]{1}\s?$").unwrap();
    re_silluq.is_match(&sc.sentence)
}

fn contains_shalshelet(sc: &SentenceContext) -> bool {
    // A SHALSHELET consists of the following two UTF-8 code-points (p.e. Gen19:16)
    //      - SHALSHELET (\u{0593}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_shalshelet = Regex::new(r"\u{0593}{1}\p{Hebrew}*?\s{0,1}[\u{05C0}\u{007C}]{1}").unwrap();
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
    let re_silluq =
        Regex::new(r"\s\p{Hebrew}*\u{05BD}{1}\p{Hebrew}*\s?\u{05C3}{1}\s?\p{Hebrew}?$").unwrap();
    re_silluq.is_match(&sc.sentence)
}

fn contains_legarmeh(sc: &SentenceContext) -> bool {
    // A 'Legarmeh' consists of the following two UTF-8 code-points:
    //      - MUNNACH (\u{05A3}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_legarmeh = Regex::new(r"\u{05A3}{1}\p{Hebrew}*?\s*?[\u{05C0}{1}\u{007C}{1}]").unwrap();
    re_legarmeh.is_match(&sc.sentence)
}

fn contains_munnach(sc: &SentenceContext) -> bool {
    // A 'Munnach' is a 'Munnach' if it is NOT FOLLOWED by a PASEQ !!!!!
    // Otherwise is called a 'Legarmeh'
    //      MUNNACH (\u{05A3})
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_munnach = Regex::new(r"\u{05A3}{1}\p{Hebrew}*?\s*?[^\u{05C0}\u{007C}]{1}").unwrap();
    re_munnach.is_match(&sc.sentence)
}

fn contains_meayela(sc: &SentenceContext) -> bool {
    /*
        Mayela is not a true conjunctive accent.
        Mayela (Tiphcha) only appears in the same word stressed by Athnach or Silluq.
    */
    let re_tiphcha_atnach = Regex::new(r"\u{0596}{1}\p{Hebrew}+\u{0591}{1}").unwrap();
    let re_atnach_tiphcha = Regex::new(r"\u{0591}{1}\p{Hebrew}+\u{0596}{1}").unwrap();
    let re_tiphcha_silluq = Regex::new(
        r"\u{0596}\p{Hebrew}+\u{05BD}{1}\p{Hebrew}+\s*?\u{05C3}{1}?\s?[\u{05E4}\u{05E1}]{1}\s?$",
    )
    .unwrap();
    let re_silluq_tiphcha = Regex::new(
        r"\u{05BD}\p{Hebrew}+\u{0596}{1}\p{Hebrew}+\s*?\u{05C3}{1}?\s?[\u{05E4}\u{05E1}]{1}\s?$",
    )
    .unwrap();

    re_tiphcha_atnach.is_match(&sc.sentence)
        || re_atnach_tiphcha.is_match(&sc.sentence)
        || re_tiphcha_silluq.is_match(&sc.sentence)
        || re_silluq_tiphcha.is_match(&sc.sentence)
}

// functions for POETRY books
fn contains_ole_we_yored(sc: &SentenceContext) -> bool {
    // A Ole We Yored consists of the following two UTF-8 code-points
    //      - OLE (\u{05AB}) followed by
    //      - YORED (\u{05A5}) aka MERKHA
    // This accent can stretch over two words (a.k.a. word-unit)

    let re_ole_we_yored = Regex::new(r"\u{05AB}{1}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}{1}").unwrap();
    re_ole_we_yored.is_match(&sc.sentence)
}
fn contains_revia_gadol(sc: &SentenceContext) -> bool {
    // not before oleweyored and not revia mugrash || revia
    // revia gadol - revia -revia
    // - REVIA (\u{0597})
    // - OLE (\u{05AB}) followed by
    // - YORED (\u{05A5}) aka MERKHA

    let re_revia_gadol = Regex::new(r"\u{0597}[\p{Hebrew}+?\s{1}]+\u{0597}{1}").unwrap();
    re_revia_gadol.is_match(&sc.sentence)
}
fn contains_revia_mugrash(sc: &SentenceContext) -> bool {
    // A 'Revia Mugrash' consists of the following two UTF-8 code-points:
    // - GERESH (\u{059C}) (and Geresh Muqdam (\u{059D}))?  followed by
    // - REVIA (\u{0597})
    let re_revia_mugrash = Regex::new(r"[\u{059C}\u{059D}]{1}\p{Hebrew}*\u{0597}{1}").unwrap();
    re_revia_mugrash.is_match(&sc.sentence)
}

fn contains_revia_qaton(sc: &SentenceContext) -> bool {
    // only before olweyored but not mugrash
    // oleweyored - revia qaton - revia gadol - revia gadol
    // - no GERESH (\u{059C}) (OR GERESH MUQDAM (\u{059D})?) in the same word as
    // - REVIA (\u{0597}) followed by
    // - OLE (\u{05AB}) followed by         }  can be split over two words
    // - YORED (\u{05A5}) (aka MERKHA)      }

    let re_revia_qaton =
        FancyRegex::new(r"(?<![\u{059C\u{059D]\p{Hebrew}+)\u{0597}{1}\u{05AB}{1}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}{1}").unwrap();
    //                         |<- NO Geresh(Muqdam)  -><   revia  -><-                     OleWeYored              ->
    re_revia_qaton.is_match(&sc.sentence).unwrap()
}

fn contains_mahpakh_legarmeh(sc: &SentenceContext) -> bool {
    // A 'Mahpakh Legarmeh' consists of the following two UTF-8 code-points:
    //      - Mahpakh (\u{05A4}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_mahpakh_legarmeh =
        Regex::new(r"\u{05A4}{1}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]{1}").unwrap();
    re_mahpakh_legarmeh.is_match(&sc.sentence)
}

fn contains_azla_legarmeh(sc: &SentenceContext) -> bool {
    // An 'Alza Legarmeh' consists of the following two UTF-8 code-points
    //      - AZLA (\u{05A8}) followed by
    //      - PASEQ (\u{05C0})
    // Some texts use a VERTICAL LINE (U+007C) in stead of a PASEQ
    let re_alza_legarmeh = Regex::new(r"\u{05A8}{1}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]{1}").unwrap();
    re_alza_legarmeh.is_match(&sc.sentence)
}

// conjunctives
fn contains_merkha(sc: &SentenceContext) -> bool {
    // TODO
    let re_merkha = FancyRegex::new(r"\u{05A5}{1}(?!\p{Hebrew}*\s+[\u{05C0}\u{007C}]{1})").unwrap();
    re_merkha.is_match(&sc.sentence).unwrap()
}
fn contains_mahpakh(sc: &SentenceContext) -> bool {
    // TODO
    let re_mahpakh =
        FancyRegex::new(r"\u{05A4}{1}(?!\p{Hebrew}*\s+[\u{05C0}\u{007C}]{1})").unwrap();
    re_mahpakh.is_match(&sc.sentence).unwrap()
}

fn contains_azla(sc: &SentenceContext) -> bool {
    // TODO
    let re_alza = Regex::new(r"\u{05A8}{1}\p{Hebrew}*?\s?[^\u{05C0}\u{007C}]{1}").unwrap();
    re_alza.is_match(&sc.sentence)
}

fn contains_shalshelet_qetannah(sc: &SentenceContext) -> bool {
    // TODO
    // let pattern = r"\u{0593}{1}(?!\u{0593}{1}\p{Hebrew}*?\s{0,1}?[^\u{05C0}{1}\u{007C}{1}])"; // Matches the specific Unicode character
    let pattern = r"\u{0593}{1}(?!\u{0593}{1}ב)"; // Matches the specific Unicode character
                                                  //let pattern = r"\u{0593}{1}"; // Matches the specific Unicode character
    let fancy_re = FancyRegex::new(pattern).unwrap();
    fancy_re.is_match(&sc.sentence).expect("REASON")
}

fn contains_tsinnorit_merkha(sc: &SentenceContext) -> bool {
    // A Tsinnorit Merkha consists of the following two UTF-8 code-points
    //      - TSINNORIT (\u{0598}) followed by
    //      - MERKHA (\u{05A5})
    // This accent can occur in one or two words (a.k.a. word-unit)

    let re_tsin_mer = Regex::new(r"\u{0598}{1}\p{Hebrew}+?\u{05A5}{1}").unwrap();
    re_tsin_mer.is_match(&sc.sentence)
}

fn contains_tsinnorit_mahpakh(sc: &SentenceContext) -> bool {
    // A Tsinnorit Mahpakh consists of the following two UTF-8 code-points
    //      - TSINNORIT (\u{0598}) followed by
    //      - Mahpakh (\u{05A4})
    // This accent can occur in one or two words (a.k.a. word-unit)

    let re_tsinnorit_mahpakh = Regex::new(r"\u{0598}\p{Hebrew}+\u{05A4}").unwrap();
    re_tsinnorit_mahpakh.is_match(&sc.sentence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_prose_poetry_silluq() {
        let prosaic_sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ",
            Context::Prosaic,
        );
        assert!(prosaic_sc.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        let prosaic_sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ פ",
            Context::Prosaic,
        );
        assert!(prosaic_sc.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        let prosaic_sc = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert!(prosaic_sc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
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
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let newsc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        let newsc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_zaqeph_qatan() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQatan)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_revia() {
        let newsc = SentenceContext::new("בְּרֵ֗אשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣֚א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let newsc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let newsc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        let newsc =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    // Conjunctives
    #[test]
    fn test_contains_prose_munnach() {
        let newsc =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let newsc = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_darga() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_azla() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_galgal() {
        let newsc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_prose_meayela() {
        let newsc = SentenceContext::new("בּראשׁית בּרא א֖לה֑ים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Meayela)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אלה֑ים֖ את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Meayela)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אל֑ה֖ים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Meayela)));
        let newsc = SentenceContext::new("בּראשׁית בּרא א֖ל֑הים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Meayela)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_ole_we_yored() {
        let newsc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new(
            "וְֽהָיָ֗ה כְּעֵץ֮ שָׁת֪וּל עַֽל־פַּלְגֵ֫י מָ֥יִם אֲשֶׁ֤ר פִּרְי֨וֹ יִתֵּ֬ן בְּעִתּ֗וֹ וְעָלֵ֥הוּ לֹֽא־יִבּ֑וֹל וְכֹ֖ל אֲשֶׁר־יַעֲשֶׂ֣ה יַצְלִֽיחַ׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_atnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        // Ps 29:3
        let newsc = SentenceContext::new(
            "מזמ֗ור לד֫ו֥ד הב֣וּ לֽ֭יהוה בּנ֣י אל֑ים הב֥וּ ל֝יהו֗ה כּב֥וד ועֽז׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        let newsc = SentenceContext::new("י֣אבד י֭ום אוּ֣לד בּ֑ו והלּ֥ילה א֝מ֗ר ה֣רה גֽבר׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        let newsc = SentenceContext::new("יצחק אל־יע֓קב ׀ ויברך", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        // Ps 29:3
        let newsc = SentenceContext::new(
            "מזמ֗ור לד֫ו֥ד הב֣וּ לֽ֭יהוה בּנ֣י אל֑ים הב֥וּ ל֝יהו֗ה כּב֥וד ועֽז׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        // assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let newsc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let newsc = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let newsc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_mahpakh_legarmeh() {
        let newsc = SentenceContext::new(" ׀ את־אברהם֤ ׀ א־א", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        let newsc = SentenceContext::new(" ׀ את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let newsc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munnach)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        let newsc = SentenceContext::new("את־אברה֥ם  אברהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        // let newsc = SentenceContext::new("את־אבר֥הם ׀ אברהם", Context::Poetic);
        // assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
        //assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_illuy() {
        let newsc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_tarkha() {
        let newsc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarkha)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let newsc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_mahpakh() {
        let newsc = SentenceContext::new("את־אבר֤הם  אברהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        let newsc = SentenceContext::new("את־אבר֤הם ׀ אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_azla() {
        //let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        let newsc = SentenceContext::new(" ׀ את־אברה֨ם ׀ א־אם", Context::Poetic);

        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        // let newsc = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        // assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        let newsc = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        // assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_merkha() {
        let newsc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        //assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mahpakh() {
        let newsc = SentenceContext::new("את־א֘ב֤רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }

    // #[test]
    // // TODO
    // fn test_find() {
    //     let newsc = SentenceContext::new("gad", Context::Prosaic);
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Prose(ProseAccent::Galgal)),
    //         Some(1)
    //     );
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Prose(ProseAccent::Atnach)),
    //         Some(1)
    //     );
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)),
    //         None
    //     );
    // }
}
