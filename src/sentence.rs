#![allow(unused)]
use crate::constants::*;
use crate::sentence;
use crate::HebrewAccent;
use crate::PoetryAccent;
use crate::ProseAccent;

use fancy_regex::Regex as FancyRegex;
use once_cell::sync::Lazy;
use regex::Regex;

static RE_COMMON_SILLUQ: Lazy<FancyRegex> = Lazy::new(||
    // Context: Prose and Poetry
    // A Meteg in the last word of a sentence is called SILLUQ (\u{05BD})
    // Most of the time a sentence ends with Sof Pasuq (\u{05C3})
    // Some times a sentence ends with "samech" (U+05E1) or an "pey" (U+05E4).
    // Some times last word are connected by a Maqqef (\u{05BE})
    FancyRegex::new(r"\u{05BD}(?!\p{Hebrew}*\u{05BE}\p{Hebrew}*)\p{Hebrew}*\s?\u{05C3}?\s?[\u{05E4}\u{05E1}]?\s?$").unwrap());

static RE_COMMON_SHALSHELET: Lazy<Regex> = Lazy::new(||
    // Context: Prose and Poetry
    // A Shalshelet consists of the following two UTF-8 code-points (p.e. Gen19:16)
    //      - Shalshelet (\u{0593}) followed by
    //      - Paseq (\u{05C0})
    // For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
    Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{0593}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());

static RE_PROSE_LEGARMEH: Lazy<Regex> = Lazy::new(||
    // Context: Prose
    // A 'Legarmeh' consists of the following two UTF-8 code-points:
    //      - Munnach (\u{05A3}) followed by
    //      - Paseq (\u{05C0})
    // For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
    Regex::new(r"[^\s\u{05BE}]\p{Hebrew}*?\u{05A3}\p{Hebrew}*?\s*?[\u{05C0}\u{007C}]").unwrap());

static RE_PROSE_MUNNACH: Lazy<FancyRegex> = Lazy::new(||
    // Context: Prose and Poetry
    // A 'Munnach' is a 'Munnach' if it is NOT FOLLOWED by a Paseq !
    // Otherwise is called a 'Legarmeh'
    //      Munnach (\u{05A3})
    //      - Paseq (\u{05C0})
    // For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
    FancyRegex::new(r"\u{05A3}(?!\p{Hebrew}*?\s*?[\u{05C0}\u{007C}])").unwrap());

static RE_PROSE_MAYLA: Lazy<Regex> = Lazy::new(||
    // Context: Prose
    // A Mayla is a Tiphcha before Silluq or Atnach in the same word
    // or words connected with a Maqqef (\u{05BE})
    // Tiphcha: U+0596
    // Atnach:  U+0591
    // Silluq:  U+05BD (Meteg in the last word)
    Regex::new(r"(\u{0596}\p{Hebrew}+\u{0591}|\u{0596}\p{Hebrew}*?\u{05BD}\p{Hebrew}*?\s?[\u{05E4}\u{05E1}]?\s?$)").unwrap());

static RE_POETRY_OLE_WE_YORED: Lazy<Regex> = Lazy::new(||
    // Context: Poetry
    // An 'Ole We Yored' consists of the following two UTF-8 code-points
    //      - Ole (\u{05AB}) followed by
    //      - Yored (\u{05A5}) aka Merkha
    // This accent can stretch over two words (a.k.a. word-unit)
    Regex::new(r"\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}").unwrap());

static RE_POETRY_REVIA_GADOL: Lazy<Regex> = Lazy::new(||
    // TODO
    // not before oleweyored and not revia mugrash ||revia
    // revia gadol - revia -revia
    // - REVIA (\u{0597})
    // - OLE (\u{05AB}) followed by
    // - YORED (\u{05A5}) aka MERKHA
    Regex::new(r"(\u{0597}[\s\p{Hebrew}]*?\u{0597}[\s\p{Hebrew}]*?\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5})|(\u{0591}[\s\p{Hebrew}]*?\u{0597})|(\u{05AB}\p{Hebrew}+\s?\p{Hebrew}*\u{05A5}[\s\p{Hebrew}]*?\u{0597})").unwrap());

static RE_POETRY_REVIA_MUGRASH: Lazy<Regex> = Lazy::new(||
    // 'Geresh Muqdam' (\u{059D}) is Jiddisch?
    // A 'Revia Mugrash' consists of the following two UTF-8 code-points:
    // - Geresh (\u{059C}) followed by
    // - Revia (\u{0597})
    Regex::new(r"[\s\u{05BE}]\p{Hebrew}*[\u{059C}\u{059D}]\p{Hebrew}*\u{0597}").unwrap());

//static RE_POETRY_REVIA_QATON: Lazy<Regex> = Lazy::new(||
// TODO
// - Revia (\u{0597})
// An 'Ole We Yored' consists of the following two UTF-8 code-points
//      - Ole (\u{05AB}) followed by
//      - Yored (\u{05A5}) aka Merkha
// This accent can stretch over two words (a.k.a. word-unit)
//    Regex::new(r"\u{0597}[\s\p{Hebrew}]*?\u{05AB}\p{Hebrew}+?\s?\p{Hebrew}*?\u{05A5}").unwrap());

static RE_POETRY_MAHPAKH_LEGARMEH: Lazy<Regex> = Lazy::new(||
    // TODO add BEFORE OLEWEYORED to the regex
    // only before olweyored but not mugrash
    // oleweyored - revia qaton - revia gadol - revia gadol
    // - no GERESH (\u{059C}) (OR GERESH MUQDAM (\u{059D})?) in the same word as
    // - REVIA (\u{0597}) followed by
    // - OLE (\u{05AB}) followed by         }  can be split over two words
    // - YORED (\u{05A5}) (aka Merkha)      }
    Regex::new(r"\u{05A4}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());

static RE_POETRY_AZLA_LEGARMEH: Lazy<Regex> = Lazy::new(||
   // An 'Azla Legarmeh' consists of an 
    //      - Azla (\u{05A8}) followed by
    //      - Paseq (\u{05C0}) 
    // For readability a 'vertical line' (U+007C) is sometimes used instead of a Paseq
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{05A8}\p{Hebrew}*?\s?[\u{05C0}\u{007C}]").unwrap());

// conjunctives
//static RE_POETRY_MERKHA: Lazy<FancyRegex> = Lazy::new(||
// NOTE: FANCY-REGEX does not work for variable length in the negative look-behind
//       So currently a negative checks for "Tsinorit Merkha" and
//       "Ole_We_Yored" is NOT possible!
// - Merkha (\u{05A5})
// but NOT part of
//   Ole_We_Yored (Ole (\u{05AB}) followed by Yored (\u{05A5}) aka Merkha)
//   -- OR --
//   Tsinorit_Merkha (Tsinnorit (\u{0598}) followed by Merkha (\u{05A5})
//    FancyRegex::new(r"[(?<!\u{05AB}\p{Hebrew}*?[\s\u{05BE}]?\p{Hebrew}*?)(?<!\u{0598}\p{Hebrew}*?[\s\u{05BE}]?\p{Hebrew}*?)]\u{05A5}").unwrap());

//static RE_POETRY_MAHPAKH: Lazy<FancyRegex> = Lazy::new(||
// NOTE: FANCY-REGEX does not work for variable length in the negative look-behind
//       So currently a negative check for "TsinnoritMahpakh" is NOT possible!
// Mahpakh (\u{05A4})
//    FancyRegex::new(r"(\x{05A4}\p{Hebrew}*?\x{05BE})|(\x{05A4}(?!\p{Hebrew}*?\s*?[\x{05C0}\x{007C}]))").unwrap());

static RE_POETRY_AZLA: Lazy<FancyRegex> = Lazy::new(|| {
    FancyRegex::new(r"(\u{05A8}\p{Hebrew}*?\u{05BE})|(\u{05A8}(?!\p{Hebrew}\s*[\u{05C0}\u{007C}]))")
        .unwrap()
});

static RE_POETRY_SHALSHELET_QETANNAH: Lazy<FancyRegex> = Lazy::new(||
    // Not followed by a Sof Passuq (or a vertical line)
    FancyRegex::new(r"\u{0593}(?!\p{Hebrew}*?\s?[\u{05C0}\u{007C}])").unwrap());

static RE_POETRY_TSINNORIT_MERKHA: Lazy<Regex> = Lazy::new(||
    // A Tsinnorit Merkha consists of the following two UTF-8 code-points
    //      - Tsinnorit (\u{0598}) followed by
    //      - Merkha (\u{05A5})
    // This accent can occur in one or two words (a.k.a. word-unit)
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A5}").unwrap());

static RE_POETRY_TSINNORIT_MAHPAKH: Lazy<Regex> = Lazy::new(||
    // A Tsinnorit Mahpakh consists of the following two UTF-8 code-points
    //      - Tsinnorit (\u{0598}) followed by
    //      - Mahpakh (\u{05A4})
    // This accent can occur in one or two words (a.k.a. word-unit)
    //Regex::new(r"[\s\u{05BE}]\p{Hebrew}*\u{0598}\p{Hebrew}+[\s\u{05BE}]\p{Hebrew}*\u{05A4}").unwrap());
    Regex::new(r"[\s\u{05BE}]?\p{Hebrew}*?\u{0598}\p{Hebrew}+[\s\u{05BE}]?\p{Hebrew}*\u{05A4}").unwrap());

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
    /// use hebrew_accents::SentenceContext;
    /// use hebrew_accents::Context;
    /// use hebrew_accents::HebrewAccent;
    /// use hebrew_accents::ProseAccent;
    ///
    /// let sentence_context = SentenceContext::new("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",Context::Prosaic,);
    /// assert!(sentence_context.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
    /// ```
    pub fn contains_accent(&self, accent: HebrewAccent) -> bool {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                RE_COMMON_SILLUQ.is_match(&self.sentence).unwrap()
            }

            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self.sentence.contains('\u{0591}'),

            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0592}')
            }

            HebrewAccent::Prose(ProseAccent::Shalshelet)
            | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) => {
                RE_COMMON_SHALSHELET.is_match(&self.sentence)
            }

            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0594}')
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0595}')
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{0597}')
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarkha) => self.sentence.contains('\u{0596}'),
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
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence.contains('\u{05A1}')
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                self.sentence.contains('\u{059F}')
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                self.sentence.contains('\u{05A0}')
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) => {
                RE_PROSE_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => {
                RE_PROSE_MUNNACH.is_match(&self.sentence).unwrap()
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
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => self.sentence.contains('\u{005AA}'),
            HebrewAccent::Prose(ProseAccent::Mayla) if self.context == Context::Prosaic => {
                RE_PROSE_MAYLA.is_match(&self.sentence)
            }
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                RE_POETRY_OLE_WE_YORED.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                false
                //RE_POETRY_REVIA_GADOL.is_match(&self.sentence)
                // contains_poetry_revia_gadol(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                RE_POETRY_REVIA_MUGRASH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AE}')
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                false
                // RE_POETRY_REVIA_QATON.is_match(&self.sentence)
                // contains_poetry_revia_qaton(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AD}')
            }
            HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_MAHPAKH_LEGARMEH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                RE_POETRY_AZLA_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munnach) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05A3}')
            }
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => {
                //self.sentence.contains('\u{05A5}')
                // RE_POETRY_MERKHA.is_match(&self.sentence)
                // replacewith a function
                contains_poetry_merkha(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => {
                self.sentence.contains('\u{05AC}')
            }
            HebrewAccent::Poetry(PoetryAccent::Mahpakh) if self.context == Context::Poetic => {
                //self.sentence.contains('\u{05A4}')
                // RE_POETRY_MAHPAKH.is_match(&self.sentence).unwrap()
                contains_poetry_mahpakh(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => {
                RE_POETRY_AZLA.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_SHALSHELET_QETANNAH
                    .is_match(&self.sentence)
                    .unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_TSINNORIT_MERKHA.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                RE_POETRY_TSINNORIT_MAHPAKH.is_match(&self.sentence)
            }
            _ => false,
        }
    }

    pub fn find_accent(&self, accent: HebrewAccent) -> Option<usize> {
        // let haystack = "Hello, world! This is a test.";
        // let pattern = r"world"; // The regex pattern to search for
        // let re = Regex::new(pattern).unwrap();
        // if let Some(matched) = re.find(haystack) {
        //     println!("Found match at position: {:?}", matched.start());
        // } else {
        //     println!("No match found.");
        // }
        match accent {
            // Prose Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => None,
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => None,
            HebrewAccent::Prose(ProseAccent::Segolta) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Tiphcha) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Pashta) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Tevir) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Geresh) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Pazer) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.context == Context::Prosaic => {
                None
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) if self.context == Context::Prosaic => None,
            // Prose Conjunctives
            HebrewAccent::Prose(ProseAccent::Munnach) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Merkha) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Azla) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::TelishaQetannah)
                if self.context == Context::Prosaic =>
            {
                None
            }
            HebrewAccent::Prose(ProseAccent::Galgal) if self.context == Context::Prosaic => None,
            HebrewAccent::Prose(ProseAccent::Mayla) if self.context == Context::Prosaic => None,
            // Poetry Disjunctives
            // HebrewAccent::Poetry(PoetryAccent::Silluq) if self.context == Context::Poetic => {
            //     None
            // }
            HebrewAccent::Poetry(PoetryAccent::OleWeYored) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Atnach) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.context == Context::Poetic => {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Pazer) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetic => {
                None
            }
            // Poetry Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munnach) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Tarkha) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Galgal) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Mahpakh) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.context == Context::Poetic => None,
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetic =>
            {
                None
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetic =>
            {
                None
            }
            _ => None,
        }
    }
}

fn contains_poetry_merkha(sentence: &str) -> bool {
    // Merkha is
    //   not part of Ole We Yored (needs Negative Lookbehind)
    //   AND
    //   not part of Tsinnorit Merkha (needs Negative Lookbehind)
    let target_char = MERKHA;
    let possible_combinations_lookbehind = [ZARQA, OLE];

    // Check for the existence of the target character in the sentence
    if !&sentence.contains(target_char) {
        return false;
    }
    // Convert the sentence into a Vec<char> for character indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // Find the indices of the target character within the sentence
    let indices = indexes_of_target_char(target_char, &char_vec);
    // loop over all character positions
    for &index in &indices {
        println!("LOOP::Index of target character = {index}\n");
        println!("LOOP::INFO:: Negative Looking Backwards");
        let is_part_of = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
            index,
            &possible_combinations_lookbehind,
            2,
        );
        if !is_part_of {
            println!(
                "Found at least one target char, not part of another aaccent:: BREAK the loop"
            );
            return true;
        }
        println!("LOOP::Target character found backwords? = {is_part_of}\n");
    }
    false
    //sentence.contains(MERKHA)
}

fn contains_poetry_mahpakh(sentence: &str) -> bool {
    // Mahpakh
    //   not part of Mahpakh Legarmeh (needs Negative Forward)
    //   not part of Tsinnorit Mahpakh (needs Negative Lookbehind)
    let target_char = MAHPAKH;
    let possible_combinations_lookbehind = [ZARQA];
    let possible_combinations_lookahead = [PASEQ, VERTICAL_LINE];

    // check if the target character is present in the senctence
    if !&sentence.contains(target_char) {
        println!("MAHPAKH not found in the senctence at all  -> return false");
        return false;
    }
    // turn sentence into a Vec of chars for indexing
    let char_vec: Vec<char> = sentence.chars().collect();
    // retrieve character positions of the target character
    let indices: Vec<usize> = indexes_of_target_char(target_char, &char_vec);
    // loop over all character positions
    for index in indices {
        println!("\n\nLOOP::Index of target character = {index}\n");
        println!("\nNegative Looking Backward");
        let two_code_points_behind = is_part_of_two_code_point_accent_look_behind(
            &char_vec,
            &target_char,
            index,
            &possible_combinations_lookbehind,
            2,
        );
        println!("\nNegative Looking Forward");
        let is_part_of_mahpakh_legarmeh = is_part_of_mapakh_legarmeh_look_ahead(index, &char_vec);

        if !two_code_points_behind && !is_part_of_mahpakh_legarmeh {
            println!("Found at least one target char, not part of another accent:: Return TRUE");
            return true;
        }
    }
    false
    //sentence.contains(MAHPAKH)
}

fn contains_poetry_revia_gadol(sentence: &str) -> bool {
    todo!();
    // Revia Gadol is
    //   before Ole We Yored (needs Negative Lookforward)
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    sentence.contains(REVIA);
    false
}
fn contains_poetry_revia_qaton(sentence: &str) -> bool {
    todo!();
    // Revia Qaton is
    //   before Ole We Yored (needs Negative Lookforward)
    //   not part of Revia Mugrash (needs Negative Lookbehind)
    sentence.contains(REVIA);
    false
}

fn indexes_of_target_char(target_char: char, sentence: &[char]) -> Vec<usize> {
    sentence
        .iter()
        .enumerate()
        .filter_map(|(index, &c)| if c == target_char { Some(index) } else { None })
        .collect()
}

fn is_part_of_two_code_point_accent_look_behind(
    sentence: &[char],
    target_char: &char,
    index_target_char: usize,
    possible_combinations_lookbehind: &[char],
    max_word_span: usize,
) -> bool {
    // the word containing the accent is the first word
    let mut word_count: usize = 0;
    if index_target_char == 0 {
        println!("LB::Target character is found at the first position");
        // Early exit if the position is at the start
        return false;
    }
    // backwards search
    for current_index in (0..index_target_char).rev() {
        let current_char = sentence[current_index];
        println!(
            "LB::INFO::Current character [ {current_char} ] at index {current_index} - word_count: {word_count}"
        );

        if current_char == ' ' || current_char == MAQAF {
            word_count += 1;
            println!("LB::new word_count: {word_count}");
            if word_count == max_word_span {
                println!("LB::Max word count reached");
                return false;
            }
        }
        // Check for the second directly following target character
        if current_char == *target_char {
            println!("LB::Found target character [{current_char}] at position {current_index}");
            return false;
        }
        // Check for possible combinations
        if possible_combinations_lookbehind.contains(&current_char) {
            println!("LB::Found:: Part of two code-point accent");
            return true;
        }
    }
    println!("LB::End of look behind");
    true
}

pub fn is_part_of_mapakh_legarmeh_look_ahead(
    start_index_target_char: usize,
    sentence: &[char],
) -> bool {
    // TODO MULIPLESPACES NOT COVERED
    // Check if the start index is at the end of the character vector
    if start_index_target_char >= sentence.len() {
        println!("find_paseq::Target character is found at the last position");
        return false; // Early exit if the position is at the end of the sentence
    }

    let possible_combinations_lookahead = [PASEQ, VERTICAL_LINE];
    let mut word_count: usize = 0;

    // Start iterating from the next character after the target character
    for (current_index, _) in sentence
        .iter()
        .enumerate()
        .skip((start_index_target_char + 1))
    {
        let current_char = sentence[current_index];
        println!(
            "find_paseq::char current_index = {current_index} is '{current_char}', word count = {word_count}");

        // Check for possible combinations
        // Increment word count on space character
        if current_char == ' ' {
            word_count += 1;
            println!("find_paseq1::new word_count: {word_count}");
            if word_count == 2 {
                println!("find_paseq2::Max word count reached");
                return false;
            }
            continue;
        }
        if possible_combinations_lookahead.contains(&current_char) {
            println!("find_paseq3:: Part of two code-point accent!!!");
            return true;
        }
        // else {
        //     println!("find_paseq4:: Part of two code-point accent!!!");

        //     return false;
        // }

        // If one word has been counted, check for combinations again
        // if word_count == 1 {
        //     if possible_combinations_lookahead.contains(&current_char) {
        //         println!("find_paseq:: Part of two code-point accent!!!");
        //         return true;
        //     } else {
        //         println!("find_paseq:: NOT part of two code-point accent!!!");
        //         return false;
        //     }
        // }
    }

    false // Return false if no conditions are met
}

fn is_followed_by_ole_we_yored() -> bool {
    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_prose_poetry_silluq() {
        let prosaic_sc1 = SentenceContext::new(" וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר וַֽיְהִי־אֽוֹר׃", Context::Prosaic);
        assert!(prosaic_sc1.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        // ProseAccent, with Soph Paseq, no Pey or Samech
        let prosaic_sc2 = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
            Context::Prosaic,
        );
        assert!(prosaic_sc2.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        // ProseAccent, no Soph Paseq, with Pey
        let prosaic_sc3 = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert!(prosaic_sc3.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // PoetryAccent with Soph Paseq and Peh
        let prosaic_sc4 = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        assert!(prosaic_sc4.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // Meteg not in the last word of the sentence
        let prosaic_sc10 = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
            Context::Poetic,
        );
        assert!(!prosaic_sc10.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
        // Meteg followed by Maqqef (\u{05BE}) (meaning no Meteg in the last word)
        let prosaic_sc11 = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
    }
    #[test]
    fn test_contains_prose_poetry_atnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let newsc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
        let newsc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        let newsc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
    }
    #[test]
    fn test_contains_prose_zaqeph_qatan() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQatan)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQatan)));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
    }
    #[test]
    fn test_contains_prose_revia() {
        let newsc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let newsc = SentenceContext::new(
            "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
        let newsc = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let newsc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
        let newsc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
        let newsc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let newsc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let newsc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
        let newsc = SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        let newsc =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
        let newsc =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
    }
    // Conjunctives
    #[test]
    fn test_contains_prose_munnach() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית אֱלֹהִ֣ים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        // let newsc =
        //     SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        // assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        // let newsc =
        //     SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        // assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        // let newsc =
        //     SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        // assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
        // let newsc =
        //     SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        //assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Munnach)));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let newsc = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
        let newsc = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let newsc = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
        let newsc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let newsc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
        let newsc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
    }
    #[test]
    fn test_contains_prose_darga() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
    }
    #[test]
    fn test_contains_prose_azla() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
    }
    #[test]
    fn test_contains_prose_galgal() {
        let newsc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
    }
    #[test]
    fn test_contains_prose_mayla() {
        let newsc = SentenceContext::new(
            "וְאֵ֤ל שַׁדַּי֙ יְבָרֵ֣ךְ אֹֽתְךָ֔ וְיַפְרְךָ וְיַ֖רְבֶּ֑ךָ וְהָיִיתָ לִקְהַ֥ל עַמִּֽים׃",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mayla)));
        // tiphcha followed by silluq
        let newsc = SentenceContext::new(
            "וְאֵ֤ל שַׁדַּי֙ יְבָרֵ֣ךְ אֹֽתְךָ֔ וְיַפְרְךָ֖ וְיַרְבֶּ֑ךָ וְהָיִ֖יתָ לִקְהַ֥ל עַ֖מִּֽים׃",
            Context::Prosaic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mayla)));
        // no Mayla, only tiphcha
        let newsc = SentenceContext::new(
            "וְאֵ֤ל שַׁדַּי֙ יְבָרֵ֣ךְ אֹֽתְךָ֔ וְיַפְרְ֖ךָ וְיַרְבֶּ֑ךָ וְהָיִ֖יתָ לִקְהַ֥ל עַמִּֽים׃",
            Context::Prosaic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Mayla)));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_ole_we_yored() {
        let newsc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
        let newsc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::OleWeYored)));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        // TODO
        // Add testcases: - revia before OleWe Yored
        //                  revia after OleWe Yored
        //                  revia after Atnach
        //                  revia before revia before OleWe Yored
        let newsc = SentenceContext::new(
            "שׁ֗יר שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההרים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        let newsc = SentenceContext::new(
            "שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההרים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        let newsc = SentenceContext::new(
            "שׁיר לֽמּ֫עלות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
        let newsc = SentenceContext::new(
            "שִׁיר לַמַּעֲלוֹת אֶשָּׂא עֵינַי אֶל־הֶהָרִים מֵאַיִן יָבֹא עֶזְרִי׃ ",
            Context::Poetic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
    }
    #[test]
    fn test_contains_poetry_revia_gadol_this_one_should_fail() {
        // TODO
        // Add testcases: - revia before OleWe Yored
        //                  revia after OleWe Yored
        //                  revia after Atnach
        //                  revia before revia before OleWe Yored
        //                  revia mugrash
        let newsc =
            SentenceContext::new("שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים  יב֥א עזרֽי׃", Context::Poetic);
        let newsc = SentenceContext::new(
            "מ֝א֗ין לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol)));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        //
        let newsc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
        //no geresh
        let newsc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
        // no revia
        let newsc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        let newsc = SentenceContext::new("יצחק אל־יע֓קב ׀ ויברך", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        assert!(newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
        let newsc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!newsc.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        // TODO
        // Testcases: - revia before OleWe Yored (Ps 121:1 & Geresh removed)
        //              revia after OleWe Yored
        //              revia after Atnach
        let newsc = SentenceContext::new(
            "שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        let newsc = SentenceContext::new(
            "שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההרים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        let newsc = SentenceContext::new(
            "שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
    }
    #[test]
    fn test_contains_poetry_revia_qaton_this_one_should_fail() {
        // TODO
        // Add testcases: - revia before OleWe Yored
        //                  revia after OleWe Yored
        //                  revia after Atnach
        //                  revia before revia before OleWe Yored
        //                  revia mugrash before OleWe Yored
        //                  revia mugrash after OleWe Yored
        let newsc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
        let newsc = SentenceContext::new(
            " מ֝א֗ין לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton)));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let newsc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let newsc = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let newsc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
    }
    #[test]
    fn test_contains_poetry_mahpakh_legarmeh() {
        // TODO
        let newsc = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)));
        let newsc = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)));
        let newsc = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::MahpakhLegarmeh)));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        let newsc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        let newsc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        let newsc = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        let newsc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
        let newsc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let newsc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munnach)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munnach)));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        // No merkha
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // ONE merkha
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + TSINNORIT (2w)
        let newsc = SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + TSINNORIT (1w)
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + OLE (2w)
        let newsc = SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + OLE (1w)
        let newsc = SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + TSINNORIT (3w)
        let newsc = SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));

        // MERKHA + MERKHA + TSINNORIT (2w)
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha)));
    }

    #[test]
    fn test_contains_poetry_illuy() {
        let newsc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
    }
    #[test]
    fn test_contains_poetry_tarkha() {
        let newsc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarkha)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarkha)));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let newsc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
        let newsc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
    }
    #[test]
    fn test_contains_poetry_mahpakh() {
        // No Mahpakh
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh
        let newsc = SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (one word)
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (two words)
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (three words)
        let newsc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (one word), followed with a Mahpakh
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הא֤רץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (two words), followed with a Mahpakh
        let newsc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הא֤רץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Tsinnorit Mahpakh (three words), followed with a Mahpakh
        let newsc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הא֤רץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (no space)
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (one space)
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (no space - vertical line)
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (one space - vertical line)
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (too many spaces)
        let newsc =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        //One Mahpakh, part of Mahpakh Legarmeh (no space), followed with a Mahpakh
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
        // One Mahpakh, part of Mahpakh Legarmeh (one space), followed with a Mahpakh
        let newsc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mahpakh)));
    }

    #[test]
    fn test_contains_poetry_azla() {
        let newsc = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        let newsc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        let newsc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        let newsc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        let newsc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
        let newsc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        let newsc = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        let newsc = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
        let newsc = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_merkha() {
        // accent in a single word
        let newsc = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words seperated by Maqqef
        let newsc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("את־א֘ברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in two words
        let newsc = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        let newsc = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
        // accent in three words
        let newsc = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mahpakh() {
        // accent in a single word
        let newsc = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
        // accent in two words seperated by Maqqef
        let newsc = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
        // accent in two words
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
        assert!(newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        let newsc = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
        // accent in three words
        let newsc = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
        assert!(!newsc.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
    }

    // #[test]
    // TODO
    // fn test_find() {
    //     let newsc = SentenceContext::new("gad", Context::Prosaic);
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Prose(ProseAccent::Galgal)),
    //         None
    //     );
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Prose(ProseAccent::Atnach)),
    //         None
    //     );
    //     assert_eq!(
    //         newsc.find_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)),
    //         None
    //     );
    // }
}
