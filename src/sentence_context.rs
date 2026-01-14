//! sdf [`asfd`] dff
//!
//! iyuFDMFOPOKPOMOIPSADSS  ssdf sdf sf weew fweflwf wel
use crate::accent::PoetryAccent;
use crate::accent::ProseAccent;
use crate::char::*;
use crate::sentence_ctx_funcs::*;
use crate::sentence_ctx_regex::*;
use crate::HebrewAccent;

/// Sentence including the context
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SentenceContext {
    pub sentence: String,
    pub ctx: Context,
}

/// Describes the context of a sentence (poetic or prosaic)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Context {
    /// The sentence follows a poetic structure (e.g., meter, rhyme).
    Poetic,
    /// The sentence follows ordinary prose conventions.
    #[default]
    Prosaic,
}

impl SentenceContext {
    /// Creates a new object: SentenceContext
    ///
    /// # Example
    /// ```
    /// use hebrew_accents::Context;
    /// use hebrew_accents::SentenceContext;
    ///
    /// let sentence_context = SentenceContext::new( "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ", Context::Prosaic);
    /// assert_eq!(sentence_context.ctx,Context::Prosaic);
    /// assert_eq!(sentence_context.sentence,"וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃  ׃ פ".to_string());
    /// ```
    pub fn new(sentence: &str, ctx: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            ctx,
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
    /// assert!(sentence_context.contains_accent(ProseAccent::Silluq.into()));
    /// ```
    pub fn contains_accent(&self, accent: HebrewAccent) -> bool {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                FA_RE_OUTER_COMMON_SILLUQ.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self.sentence.contains(ETNAHTA),
            HebrewAccent::Prose(ProseAccent::Segolta) if self.ctx == Context::Prosaic => {
                self.sentence.contains(SEGOL)
            }
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.ctx == Context::Prosaic => {
                RE_OUTER_COMMON_SHALSHELET.is_match(&self.sentence)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQaton) if self.ctx == Context::Prosaic => {
                self.sentence.contains(ZAQEF_QATAN)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.ctx == Context::Prosaic => {
                self.sentence.contains(ZAQEF_GADOL)
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.ctx == Context::Prosaic => {
                self.sentence.contains(REVIA)
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarkha) => self.sentence.contains(TIPEHA),
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.ctx == Context::Prosaic => {
                self.sentence.contains(ZARQA)
            }
            HebrewAccent::Prose(ProseAccent::Pashta) if self.ctx == Context::Prosaic => {
                self.sentence.contains(PASHTA)
            }
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.ctx == Context::Prosaic => {
                self.sentence.contains(YETIV)
            }
            HebrewAccent::Prose(ProseAccent::Tevir) if self.ctx == Context::Prosaic => {
                self.sentence.contains(TEVIR)
            }
            HebrewAccent::Prose(ProseAccent::Geresh) if self.ctx == Context::Prosaic => {
                self.sentence.contains(GERESH)
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.ctx == Context::Prosaic => {
                self.sentence.contains(GERSHAYIM)
            }
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence.contains(PAZER)
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.ctx == Context::Prosaic => {
                self.sentence.contains(QARNEY_PARA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.ctx == Context::Prosaic => {
                self.sentence.contains(TELISHA_GEDOLA)
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) => {
                RE_OUTER_PROSE_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) if self.ctx == Context::Prosaic => {
                FA_RE_OUTER_PROSE_MUNACH.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.ctx == Context::Prosaic => {
                self.sentence.contains(MAHPAKH)
            }
            HebrewAccent::Prose(ProseAccent::Merkha) if self.ctx == Context::Prosaic => {
                self.sentence.contains(MERKHA)
            }
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.ctx == Context::Prosaic => {
                self.sentence.contains(MERKHA_KEFULA)
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.ctx == Context::Prosaic => {
                self.sentence.contains(DARGA)
            }
            HebrewAccent::Prose(ProseAccent::Azla) if self.ctx == Context::Prosaic => {
                self.sentence.contains(QADMA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.ctx == Context::Prosaic => {
                self.sentence.contains(TELISHA_QETANA)
            }
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => self.sentence.contains(YERAH_BEN_YOMO),
            HebrewAccent::Prose(ProseAccent::Mayela) if self.ctx == Context::Prosaic => {
                RE_OUTER_PROSE_MEAYLA.is_match(&self.sentence)
            }
            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => {
                FA_RE_OUTER_COMMON_METEG.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Prose(ProseAccent::Maqqeph)
            | HebrewAccent::Poetry(PoetryAccent::Maqqeph) => self.sentence.contains(MAQQEPH),
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_OLEH_WE_YORED.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.ctx == Context::Poetic => {
                contains_poetry_revia_gadol(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_REVIA_MUGRASH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) if self.ctx == Context::Poetic => {
                RE_OUTER_COMMON_SHALSHELET.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.ctx == Context::Poetic => {
                self.sentence.contains(ZINOR)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.ctx == Context::Poetic => {
                contains_poetry_revia_qaton(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.ctx == Context::Poetic => {
                self.sentence.contains(DEHI)
            }
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
                if self.ctx == Context::Poetic =>
            {
                RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_AZLA_LEGARMEH.is_match(&self.sentence)
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.ctx == Context::Poetic => {
                self.sentence.contains(MUNAH)
            }
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.ctx == Context::Poetic => {
                contains_poetry_merkha(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.ctx == Context::Poetic => {
                self.sentence.contains(ILUY)
            }
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.ctx == Context::Poetic => {
                contains_poetry_mehuppakh(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.ctx == Context::Poetic => {
                FA_RE_OUTER_POETRY_AZLA.is_match(&self.sentence).unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.ctx == Context::Poetic =>
            {
                FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH
                    .is_match(&self.sentence)
                    .unwrap()
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_TSINNORIT_MERKHA.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_TSINNORIT_MAHPAKH.is_match(&self.sentence)
            }
            _ => false,
        }
    }
    pub fn find_accent(self, accent: HebrewAccent) -> Option<usize> {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                //FA_RE_OUTER_COMMON_SILLUQ.is_match(&self.sentence).unwrap();
                if let Some(outer_match) = FA_RE_OUTER_COMMON_SILLUQ.find(&self.sentence).unwrap() {
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    Some(outer_start)
                } else {
                    println!("Outer pattern not found for SILLUQ.");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self.sentence.find(ETNAHTA),

            HebrewAccent::Prose(ProseAccent::Segolta) if self.ctx == Context::Prosaic => {
                self.sentence.find(SEGOL)
            }
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.ctx == Context::Prosaic => {
                if let Some(outer_match) = RE_OUTER_COMMON_SHALSHELET.find(&self.sentence) {
                    println!("RE_OUTER_COMMON_SHALSHELET: FOUND!");
                    println!(
                        "INNER MATCH:: start():{}  ;end():{}  ;str():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) = RE_INNER_COMMON_SHALSHELET.find(outer_match.as_str())
                    {
                        println!(
                            "INNER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                            inner_match.start(),
                            inner_match.end(),
                            inner_match.as_str()
                        );
                        let absolute_inner_start = outer_start + inner_match.start();
                        let absolute_inner_end = outer_start + inner_match.end();
                        println!("Absolute start in `hay`: {}", absolute_inner_start);
                        println!("Absolute end in `hay`: {}", absolute_inner_end);
                        Some(absolute_inner_start)
                    } else {
                        println!("Narrow pattern not found inside the first match.");
                        None
                    }
                } else {
                    println!("No shalshelet character found in the input.");
                    None
                }
            }

            HebrewAccent::Prose(ProseAccent::ZaqephQaton) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZAQEF_QATAN)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZAQEF_GADOL)
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.ctx == Context::Prosaic => {
                self.sentence.find(REVIA)
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarkha) => self.sentence.find(TIPEHA),

            HebrewAccent::Prose(ProseAccent::Zarqa) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZARQA)
            }
            HebrewAccent::Prose(ProseAccent::Pashta) if self.ctx == Context::Prosaic => {
                self.sentence.find(PASHTA)
            }
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.ctx == Context::Prosaic => {
                self.sentence.find(YETIV)
            }
            HebrewAccent::Prose(ProseAccent::Tevir) if self.ctx == Context::Prosaic => {
                self.sentence.find(TEVIR)
            }
            HebrewAccent::Prose(ProseAccent::Geresh) if self.ctx == Context::Prosaic => {
                self.sentence.find(GERESH)
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.ctx == Context::Prosaic => {
                self.sentence.find(GERSHAYIM)
            }
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence.find(PAZER)
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.ctx == Context::Prosaic => {
                self.sentence.find(QARNEY_PARA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.ctx == Context::Prosaic => {
                self.sentence.find(TELISHA_GEDOLA)
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) => {
                if let Some(outer_match) = RE_OUTER_PROSE_LEGARMEH.find(&self.sentence) {
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) = RE_INNER_PROSE_LEGARMEH.find(outer_match.as_str()) {
                        println!(
                            "INNER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                            inner_match.start(),
                            inner_match.end(),
                            inner_match.as_str()
                        );
                        let absolute_inner_start = outer_start + inner_match.start();
                        let absolute_inner_end = outer_start + inner_match.end();
                        println!("Absolute start in `hay`: {}", absolute_inner_start);
                        println!("Absolute end in `hay`: {}", absolute_inner_end);
                        Some(absolute_inner_start)
                    } else {
                        println!("Narrow pattern not found inside the first match.");
                        None
                    }
                } else {
                    println!("No shalshelet character found in the input.");
                    None
                }
            }
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) if self.ctx == Context::Prosaic => {
                //FA_RE_OUTER_PROSE_MUNACH.is_match(&self.sentence).unwrap();
                if let Some(outer_match) = FA_RE_OUTER_PROSE_MUNACH.find(&self.sentence).unwrap() {
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    Some(outer_start)
                } else {
                    println!("Outer pattern not found for MUNACH (Prose).");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.ctx == Context::Prosaic => {
                self.sentence.find(MAHPAKH)
            }
            HebrewAccent::Prose(ProseAccent::Merkha) if self.ctx == Context::Prosaic => {
                self.sentence.find(MERKHA)
            }
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.ctx == Context::Prosaic => {
                self.sentence.find(MERKHA_KEFULA)
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.ctx == Context::Prosaic => {
                self.sentence.find(DARGA)
            }
            HebrewAccent::Prose(ProseAccent::Azla) if self.ctx == Context::Prosaic => {
                self.sentence.find(QADMA)
            }
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.ctx == Context::Prosaic => {
                self.sentence.find(TELISHA_QETANA)
            }
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => self.sentence.find(YERAH_BEN_YOMO),

            HebrewAccent::Prose(ProseAccent::Mayela) if self.ctx == Context::Prosaic => {
                //RE_OUTER_PROSE_MEAYLA.is_match(&self.sentence);
                match RE_OUTER_PROSE_MEAYLA.find(&self.sentence) {
                    Some(m) => {
                        println!("Found RE_OUTER_PROSE_MEAYLA!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("No match found for RE_OUTER_PROSE_MEAYLA.");
                        None
                    }
                }
            }

            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => {
                if let Some(outer_match) = FA_RE_OUTER_COMMON_METEG.find(&self.sentence).unwrap() {
                    println!("FA_RE_OUTER_COMMON_METEG: FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    Some(outer_start)
                } else {
                    println!("FA_RE_OUTER_COMMON_METEG: NOT FOUND!");
                    None
                }
            }

            HebrewAccent::Prose(ProseAccent::Maqqeph)
            | HebrewAccent::Poetry(PoetryAccent::Maqqeph) => self.sentence.find(MAQQEPH),
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_OLEH_WE_YORED.find(&self.sentence) {
                    Some(m) => {
                        println!("RE_OUTER_POETRY_OLEH_WE_YORED: FOUND!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("RE_OUTER_POETRY_OLEH_WE_YORED: NOT FOUND!");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.ctx == Context::Poetic => {
                //contains_poetry_revia_gadol(&self.sentence);
                Some(10000) // TODO
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_REVIA_MUGRASH.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("RE_OUTER_POETRY_REVIA_MUGRASH: FOUND!");
                        println!(
                            "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                            outer_match.start(),
                            outer_match.end(),
                            outer_match.as_str()
                        );
                        Some(outer_match.start())
                    }
                    None => {
                        println!("RE_OUTER_POETRY_REVIA_MUGRASH: NOT FOUND!");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) if self.ctx == Context::Poetic => {
                if let Some(outer_match) = RE_OUTER_COMMON_SHALSHELET.find(&self.sentence) {
                    println!("RE_OUTER_COMMON_SHALSHELET: FOUND!");
                    println!(
                        "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) = RE_INNER_COMMON_SHALSHELET.find(outer_match.as_str())
                    {
                        println!("RE_INNER_COMMON_SHALSHELET: FOUND!");
                        println!(
                            "INNER MATCH:: start():{}  ;end():{}  ;str():  {}",
                            inner_match.start(),
                            inner_match.end(),
                            inner_match.as_str()
                        );
                        let absolute_inner_start = outer_start + inner_match.start();
                        let absolute_inner_end = outer_start + inner_match.end();
                        println!("Absolute start in `hay`: {}", absolute_inner_start);
                        println!("Absolute end in `hay`: {}", absolute_inner_end);
                        Some(absolute_inner_start)
                    } else {
                        println!("Narrow pattern not found inside the first match.");
                        None
                    }
                } else {
                    println!("No shalshelet character found in the input.");
                    None
                }
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.ctx == Context::Poetic => {
                self.sentence.find(ZINOR)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.ctx == Context::Poetic => {
                //contains_poetry_revia_qaton(&self.sentence);
                Some(10000) // TODO
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.ctx == Context::Poetic => {
                self.sentence.find(DEHI)
            }
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
                if self.ctx == Context::Poetic =>
            {
                //RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.is_match(&self.sentence);
                match RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.find(&self.sentence) {
                    Some(m) => {
                        println!("Found RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("No match found for RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.ctx == Context::Poetic => {
                //RE_OUTER_POETRY_AZLA_LEGARMEH.is_match(&self.sentence);
                match RE_OUTER_POETRY_AZLA_LEGARMEH.find(&self.sentence) {
                    Some(m) => {
                        println!("Found RE_OUTER_POETRY_AZLA_LEGARMEH!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("No match found for RE_OUTER_POETRY_AZLA_LEGARMEH.");
                        None
                    }
                }
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.ctx == Context::Poetic => {
                self.sentence.find(MUNAH)
            }
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.ctx == Context::Poetic => {
                //contains_poetry_merkha(&self.sentence);
                Some(10000) // TODO
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.ctx == Context::Poetic => {
                self.sentence.find(ILUY)
            }
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.ctx == Context::Poetic => {
                //contains_poetry_mehuppakh(&self.sentence);
                Some(10000) // TODO
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.ctx == Context::Poetic => {
                //FA_RE_OUTER_POETRY_AZLA.is_match(&self.sentence).unwrap();
                match FA_RE_OUTER_POETRY_AZLA.find(&self.sentence).unwrap() {
                    Some(m) => {
                        println!("Found RE_OUTER_POETRY_REVIA_MUGRASH!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("No match found for FA_RE_OUTER_POETRY_AZLA.");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)
                if self.ctx == Context::Poetic =>
            {
                // FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH
                //     .is_match(&self.sentence)
                //     .unwrap();
                Some(10000) // TODO
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_TSINNORIT_MERKHA.find(&self.sentence) {
                    Some(m) => {
                        println!("RE_OUTER_POETRY_TSINNORIT_MERKHA: FOUND!");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("RE_OUTER_POETRY_TSINNORIT_MERKHA: NOT FOUND!");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_TSINNORIT_MAHPAKH.find(&self.sentence) {
                    Some(m) => {
                        println!("RE_OUTER_POETRY_TSINNORIT_MAHPAKH: FOUND");
                        println!("Matched text: {}", m.as_str());
                        println!("Starts at byte index: {}", m.start());
                        println!("Ends at byte index: {}", m.end());
                        Some(m.start())
                    }
                    None => {
                        println!("RE_OUTER_POETRY_TSINNORIT_MAHPAKH: NOT FOUND");
                        None
                    }
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod contains {
    use super::*;

    #[test]
    fn test_contains_prose_poetry_silluq() {
        // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
        let sentence_c = SentenceContext::new(" וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר וַֽיְהִי־אֽוֹר׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Silluq.into()));
        // ProseAccent, with Soph Paseq, no Pey or Samech
        let sentence_c = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(ProseAccent::Silluq.into()));
        // ProseAccent, no Soph Paseq, with Pey
        let sentence_c = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(PoetryAccent::Silluq.into()));
        // PoetryAccent with Soph Paseq and Peh
        let sentence_c = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(PoetryAccent::Silluq.into()));
        // Meteg not in the last word of the sentence
        let sentence_c = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(PoetryAccent::Silluq.into()));
        // Meteg followed by Maqqeph (\u{05BE}) (meaning no Meteg in the last word)
        let sentence_c1 = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
        assert!(!sentence_c1.contains_accent(PoetryAccent::Silluq.into()));
    }
    #[test]
    fn test_contains_prose_poetry_atnach() {
        // Atnach present
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Atnach.into()));
        assert!(sentence_c.contains_accent(PoetryAccent::Atnach.into()));
        // No Atnach present
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(ProseAccent::Atnach.into()));
        assert!(!sentence_c.contains_accent(PoetryAccent::Atnach.into()));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let sentence_c = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(ProseAccent::Segolta.into()));
        let sentence_c = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert!(!sentence_c.contains_accent(ProseAccent::Segolta.into()));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        // Shalshelet, with Paseq - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Paseq + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Vertical Bar - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Vertical Bar + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        assert!(!sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // Missing Paseq or Vertical Bar
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        assert!(!sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
    }
    #[test]
    fn test_contains_prose_zaqeph_qaton() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::ZaqephQaton.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::ZaqephQaton.into()));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::ZaqephGadol.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::ZaqephGadol.into()));
    }
    #[test]
    fn test_contains_prose_revia() {
        let sentence_c = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Revia.into()));
        assert!(!sentence_c.contains_accent(PoetryAccent::OlehWeYored.into()));
        let sentence_c = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Revia.into()));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let sentence_c = SentenceContext::new(
            "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(ProseAccent::Tiphcha.into()));
        let sentence_c = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Tiphcha.into()));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Zarqa.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Zarqa.into()));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Pashta.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Pashta.into()));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let sentence_c = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Yetiv.into()));
        let sentence_c = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Yetiv.into()));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Tevir.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Tevir.into()));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Geresh.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Geresh.into()));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Gershayim.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Gershayim.into()));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Pazer.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Pazer.into()));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let sentence_c =
            SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::PazerGadol.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::PazerGadol.into()));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let sentence_c =
            SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::TelishaGedolah.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::TelishaGedolah.into()));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        // Legarmeh, with Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh with a space + Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh with two spaces + Paseq
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with space + Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with two spaces + Vertical Bar
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
        // Paseq or Vertical Bar is missing
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Legarmeh.into()));
    }
    // Conjunctives
    #[test]
    fn test_contains_prose_munnach() {
        // Single Munach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (Paseq)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (space + Paseq)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים ׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (Vertical Bar)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים|  את השּׁמים ואת הארץ׃׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (space +Vertical Bar)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֣להים  |  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Munach.into()));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let sentence_c = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Mahpakh.into()));
        let sentence_c = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Mahpakh.into()));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let sentence_c = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Merkha.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Merkha.into()));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::MerkhaKephulah.into()));
        let sentence_c =
            SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::MerkhaKephulah.into()));
    }
    #[test]
    fn test_contains_prose_darga() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Darga.into()));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Darga.into()));
    }
    #[test]
    fn test_contains_prose_azla() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Azla.into()));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Azla.into()));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::TelishaQetannah.into()));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::TelishaQetannah.into()));
    }
    #[test]
    fn test_contains_prose_galgal() {
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Galgal.into()));
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Galgal.into()));
    }
    #[test]
    fn test_contains_prose_meayla() {
        // Tiphcha followed by Atnach
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Mayela.into()));
        // Tiphcha followed by Atnach, two words connected with a Maqqeph
        let sentence_c =
            SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Mayela.into()));
        // Tiphcha followed by silluq
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Mayela.into()));
        // only Tiphcha
        let sentence_c =
            SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵ֖ת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Mayela.into()));
    }
    #[test]
    fn test_contains_prose_meteg() {
        // Only Silluq, No Meteg
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Prosaic);
        assert!(!sentence_c.contains_accent(ProseAccent::Meteg.into()));
        // Meteg and Siluq, separated by a Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Prosaic);
        assert!(sentence_c.contains_accent(ProseAccent::Meteg.into()));
        // Meteg and Siluq in separate words
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(ProseAccent::Meteg.into()));
        // Only Meteg, no Silluq
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Prosaic,
        );
        assert!(sentence_c.contains_accent(ProseAccent::Meteg.into()));
    }
    #[test]
    fn test_contains_prose_maqqeph() {
        // No Maqqeph
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(ProseAccent::Maqqeph.into()));
        // One Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(ProseAccent::Maqqeph.into()));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_oleh_we_yored() {
        // OlehWeYored, one word
        let sentence_c = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, one word - context: Prosaic
        let sentence_c = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
        assert!(!sentence_c.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, two words
        let sentence_c = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, three words
        let sentence_c = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::OlehWeYored.into()));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        // No Revia at all
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Two Revia's
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּרא אלהים את השּׁ֗מים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by 'Oleh We Yored' (3 words)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia not directly followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaGadol.into()));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        // Revia and Geresh
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaMugrash.into()));
        // Only Revia
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaMugrash.into()));
        // Only Geresh
        let sentence_c = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaMugrash.into()));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        // ShalsheletGadol, with Paseq - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Paseq + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Vertical Bar - no space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Vertical Bar + one space
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        assert!(!sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
        // Missing Paseq or Vertical Bar
        let sentence_c = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        assert!(!sentence_c.contains_accent(ProseAccent::Shalshelet.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let sentence_c = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Tsinnor.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Tsinnor.into()));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        // No revia at all
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia, not followed by OleWe Yored
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by Oleh We Yored (1 word)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמי֥ם ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by 'Oleh We Yored' (3 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים ואת האר֥ץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia NOT directly followed by Oleh We Yored (2 words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֗להים א֓ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia is part of Revia Mugrash
        let sentence_c = SentenceContext::new(
            " שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert!(!sentence_c.contains_accent(PoetryAccent::ReviaQaton.into()));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let sentence_c = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Dechi.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Dechi.into()));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let sentence_c = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Pazer.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Pazer.into()));
    }
    #[test]
    fn test_contains_poetry_mehuppakh_legarmeh() {
        // MehuppakhLegarmeh, with Paseq
        let sentence_c = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
        // MehuppakhLegarmeh, with Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
        // Mehuppakh only
        let sentence_c = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        // AzlaLegarmeh, with Paseq + no space
        let sentence_c = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Paseq + 1 space
        let sentence_c = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Vertical Bar + no space
        let sentence_c = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Vertical Bar + 1 space
        let sentence_c = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // Azla only
        let sentence_c = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let sentence_c = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Munach.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Munach.into()));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        // No Merkha
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // One Merkha
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (1w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (2w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (3w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (1w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (2w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (3w)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Merkha.into()));
    }

    #[test]
    fn test_contains_poetry_illuy() {
        let sentence_c = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Illuy.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Illuy.into()));
    }
    #[test]
    fn test_contains_poetry_tarkha() {
        let sentence_c = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Tarkha.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Tarkha.into()));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let sentence_c = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Galgal.into()));
        let sentence_c = SentenceContext::new("את־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Galgal.into()));
    }
    #[test]
    fn test_contains_poetry_mehuppakh() {
        // No Mehuppach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (one word)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (two words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (three words)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));

        // One Mehuppach, part of Mehuppach Legarmeh (no space) TODO
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space) TODO
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (no space - vertical line)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space - vertical line)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of 'Mehuppach Legarmeh' (too many spaces)
        let sentence_c =
            SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
        let sentence_c =
            SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
        let sentence_c =
            SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Mehuppakh.into()));
    }

    #[test]
    fn test_contains_poetry_azla() {
        // contains Azla
        let sentence_c = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Azla.into()));
        // contains Azla and Azla Legarmeh
        let sentence_c = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with space + Paseq
        let sentence_c = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with Paseq
        let sentence_c = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with space + Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with Vertical Bar
        let sentence_c = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Azla.into()));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        // Shalshelet
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
        // Shalshelet Gadol, with Paseq
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
        // Shalshelet Gadol, with Vertical Bar
        let sentence_c = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_merkha() {
        // accent in a single word
        let sentence_c = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in a single word, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in a single word, without Merkha
        let sentence_c = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph
        let sentence_c = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph, without Merkha
        let sentence_c = SentenceContext::new("את־א֘ברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words
        let sentence_c = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words, without Tsinnorit
        let sentence_c = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words, without Merkha
        let sentence_c = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in three words
        let sentence_c = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mahpakh() {
        // accent in a single word
        let sentence_c = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in a single word
        let sentence_c = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in a single word, without Tsinnorit
        let sentence_c = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sentence_c = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sentence_c = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words
        let sentence_c = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words, without Tsinnorit
        let sentence_c = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words, without Mahpakh
        let sentence_c = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in three words
        let sentence_c = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
    }

    #[test]
    fn test_contains_poetry_meteg() {
        // Only Silluq, No Meteg
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Meteg.into()));
        // Meteg and Siluq, separated by a Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Meteg.into()));
        // Meteg and Siluq in separate words
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(PoetryAccent::Meteg.into()));
        // Only Meteg, no Silluq
        let sentence_c = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Poetic,
        );
        assert!(sentence_c.contains_accent(PoetryAccent::Meteg.into()));
    }
    #[test]
    fn test_contains_poetry_maqqeph() {
        // No Maqqeph
        let sentence_c =
            SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert!(!sentence_c.contains_accent(PoetryAccent::Maqqeph.into()));
        // One Maqqeph
        let sentence_c = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert!(sentence_c.contains_accent(PoetryAccent::Maqqeph.into()));
    }
}

#[cfg(test)]
mod find {
    use super::*;
    #[test]
    fn test_find_prose_poetry_silluq() {
        // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
        let sc = SentenceContext::new("הִי אֽוֹר׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(9));
        // ProseAccent, with Soph Pasuq, no Pey or Samech
        let sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
            Context::Prosaic,
        );
        assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(159));
        // ProseAccent, no Soph Paseq, with Pey
        let sc = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(165));
        // PoetryAccent with Soph Paseq and Peh
        let sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(159));
        // Meteg not in the last word of the sentence
        let sc = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), None);
        // Meteg followed by Maqqeph (\u{05BE}) (meaning no Meteg in the last word)
        let sc1 = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
        assert_eq!(sc1.find_accent(PoetryAccent::Silluq.into()), None);
    }
    #[test]
    fn test_find_prose_poetry_atnach() {
        // Atnach present
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), Some(52));
        // TODO:assert_eq!(sc.find_accent(PoetryAccent::Atnach.into()),Some(1));
        // No Atnach present
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), None);
        // TODO:assert_eq!(sc.find_accent(PoetryAccent::Atnach.into()),None);
    }
    #[test]
    fn test_find_prose_segolta() {
        let sc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert_eq!(sc.find_accent(ProseAccent::Segolta.into()), Some(67));
        let sc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        assert_eq!(sc.find_accent(ProseAccent::Segolta.into()), None);
    }
    #[test]
    fn test_find_prose_shalshelet() {
        // Shalshelet, with Paseq - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), Some(16));
        // Shalshelet, with Paseq + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), Some(16));
        // Shalshelet, with Vertical Bar - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), Some(16));
        // Shalshelet, with Vertical Bar + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), Some(16));
        // Missing Paseq or Vertical Bar
        let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), None);
    }
    #[test]
    fn test_find_prose_zaqeph_qaton() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::ZaqephQaton.into()), Some(63));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::ZaqephQaton.into()), None);
    }
    #[test]
    fn test_find_prose_zaqeph_gadol() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::ZaqephGadol.into()), Some(48));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::ZaqephGadol.into()), None);
    }
    #[test]
    fn test_find_prose_revia() {
        let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Revia.into()), Some(44));
        // TODO:assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()),None);
        let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Revia.into()), None);
    }
    #[test]
    fn test_find_prose_tiphcha() {
        let sc = SentenceContext::new(
            "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
            Context::Prosaic,
        );
        assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(10));
        let sc = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(9));
    }
    #[test]
    fn test_find_prose_zarqa() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), Some(120));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), None);
    }
    #[test]
    fn test_find_prose_pashta() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), Some(44));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), None);
    }
    #[test]
    fn test_find_prose_yetiv() {
        let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), Some(36));
        let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), None);
    }
    #[test]
    fn test_find_prose_tevir() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), Some(84));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), None);
    }
    #[test]
    fn test_find_prose_geresh() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), Some(78));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), None);
    }
    #[test]
    fn test_find_prose_gershayim() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Gershayim.into()), Some(18));
        let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Gershayim.into()), None);
    }
    #[test]
    fn test_find_prose_pazer() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), Some(99));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), None);
    }
    #[test]
    fn test_find_prose_pazer_gadol() {
        let sc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::PazerGadol.into()), Some(12));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::PazerGadol.into()), None);
    }
    #[test]
    fn test_find_prose_telisha_gadolah() {
        let sc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::TelishaGedolah.into()), Some(12));
        let sc = SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::TelishaGedolah.into()), None);
    }
    #[test]
    fn test_find_prose_legarmeh() {
        // Legarmeh, with Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(52)); //  - 60
                                                                            // Legarmeh with a space + Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(52)); // 40 - 61
                                                                            // Legarmeh with two spaces + Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
        // Legarmeh, with Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(52)); // 40 - 59
                                                                            // Legarmeh, with space + Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(52)); // 40 - 60
                                                                            // Legarmeh, with two spaces + Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
        // Paseq or Vertical Bar is missing
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
    }
    // Conjunctives
    #[test]
    fn test_find_prose_munnach() {
        // Single Munach
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Munach.into()), Some(28));
        // Munach part of Legarmeh (Paseq)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
        // Munach part of Legarmeh (space + Paseq)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים ׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
        // Munach part of Legarmeh (Vertical Bar)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים|  את השּׁמים ואת הארץ׃׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
        // Munach part of Legarmeh (space + Vertical Bar)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים  |  את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Munach.into()), None);
    }
    #[test]
    fn test_find_prose_mahpakh() {
        let sc = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), Some(10));
        let sc = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), None);
    }
    #[test]
    fn test_find_prose_merkha() {
        let sc = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), Some(6));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), None);
    }
    #[test]
    fn test_find_prose_merkha_kephulah() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::MerkhaKephulah.into()), Some(18));
        let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::MerkhaKephulah.into()), None);
    }
    #[test]
    fn test_find_prose_darga() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Darga.into()), Some(56));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Darga.into()), None);
    }
    #[test]
    fn test_find_prose_azla() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Azla.into()), Some(39));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Azla.into()), None);
    }
    #[test]
    fn test_find_prose_telisha_qetannah() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        assert_eq!(
            sc.find_accent(ProseAccent::TelishaQetannah.into()),
            Some(61)
        );
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::TelishaQetannah.into()), None);
    }
    #[test]
    fn test_find_prose_galgal() {
        let sc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), Some(23));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), None);
    }
    #[test]
    fn test_find_prose_meayla() {
        // Tiphcha followed by Atnach
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(48));
        // Tiphcha followed by Atnach, two words connected with a Maqqeph
        let sc = SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(6));
        // Tiphcha followed by silluq
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(104));
        // only Tiphcha
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵ֖ת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), None);
    }
    #[test]
    fn test_find_prose_poetry_meteg() {
        // Only Silluq, No Meteg
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), None);
        // Meteg and Siluq, separated by a Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(48));
        // Meteg and Siluq in separate words
        let sc = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(30));
        // Only Meteg, no Silluq
        let sc = SentenceContext::new("וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Prosaic);
        assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(2));
        // Two Meteg's, no Silluq
        let sc = SentenceContext::new("ום וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(7));
    }
    #[test]
    fn test_find_prose_poetry_maqqeph() {
        // No Maqqeph
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(ProseAccent::Maqqeph.into()), None);
        // One Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert_eq!(sc.find_accent(ProseAccent::Maqqeph.into()), Some(56));
        // No Maqqeph
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Maqqeph.into()), None);
        // One Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Maqqeph.into()), Some(56));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_find_poetry_oleh_we_yored() {
        // OlehWeYored, one word
        let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), Some(34));
        // OlehWeYored, one word - context: Prosaic
        let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
        assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), None);
        // OlehWeYored, two words
        let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), Some(26));
        // OlehWeYored, three words
        let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), None);
    }
    #[test]
    fn test_find_poetry_revia_gadol() {
        // No Revia at all
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
        // Two Revia's
        let sc = SentenceContext::new("בּר֗אשׁית בּרא אלהים את השּׁ֗מים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
        // Revia followed by Oleh We Yored (1 word)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
        // Revia followed by Oleh We Yored (2 words)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
        // Revia followed by 'Oleh We Yored' (3 words)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
        // Revia not directly followed by Oleh We Yored (1 word)
        let sc = SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    }
    #[test]
    fn test_find_poetry_revia_mugrash() {
        // Revia and Geresh (Ps 32:3)
        let sc = SentenceContext::new("בְּ֝שַׁאֲגָתִ֗י", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), Some(6));
        // Revia and Geresh (Ps 110:6) - accent on a single character
        let sc = SentenceContext::new("יָדִ֣ין בַּ֭גּוֹיִם מָלֵ֣א גְוִיּ֑וֹת מָ֥חַץ רֹ֝֗אשׁ עַל־אֶ֥רֶץ רַבָּֽה׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), Some(89));
        // Only Revia
        let sc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), None);
        // Only Geresh
        let sc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::ReviaMugrash.into()), None);
    }
    #[test]
    fn test_find_poetry_shalshelet_gadol() {
        // Shalshelet Gadol, with Paseq - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
            Some(16)
        );
        // Shalshelet Gadol, with Paseq + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
            Some(16)
        );
        // Shalshelet Gadol, with Vertical Bar - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
            Some(16)
        );
        // Shalshelet Gadol, with Vertical Bar + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
            Some(16)
        );
        // Missing Paseq or Vertical Bar
        let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ShalsheletGadol.into()), None);
    }
    #[test]
    fn test_find_poetry_tsinnor() {
        let sc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Tsinnor.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Tsinnor.into()), None);
    }
    #[test]
    fn test_find_poetry_revia_qaton() {
        // No revia at all
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia, not followed by OleWe Yored
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia directly followed by Oleh We Yored (1 word)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמי֥ם ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia directly followed by Oleh We Yored (2 words)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia directly followed by 'Oleh We Yored' (3 words)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים ואת האר֥ץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia NOT directly followed by Oleh We Yored (2 words)
        let sc = SentenceContext::new("בּראשׁית בּרא א֗להים א֓ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
        // Revia is part of Revia Mugrash
        let sc = SentenceContext::new(
            " שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        assert_eq!(sc.find_accent(PoetryAccent::ReviaQaton.into()), None);
    }
    #[test]
    fn test_find_poetry_dechi() {
        let sc = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), None);
    }
    #[test]
    fn test_find_poetry_pazer() {
        let sc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), None);
    }
    #[test]
    fn test_find_poetry_mehuppakh_legarmeh() {
        // MehuppakhLegarmeh, with Paseq
        let sc = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
            Some(17)
        );
        // MehuppakhLegarmeh, with Vertical Bar
        let sc = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
            Some(17)
        );
        // Mehuppakh only
        let sc = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()), None);
    }
    #[test]
    fn test_find_poetry_azla_legarmeh() {
        // AzlaLegarmeh, with Paseq + no space
        let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), Some(15));
        // AzlaLegarmeh, with Paseq + 1 space
        let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), Some(15));
        // AzlaLegarmeh, with Vertical Bar + no space
        let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), Some(15));
        // AzlaLegarmeh, with Vertical Bar + 1 space
        let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), Some(15));
        // Azla only
        let sc = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), None);
    }
    #[test]
    fn test_find_poetry_munnach() {
        let sc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Munach.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Munach.into()), None);
    }
    #[test]
    fn test_find_poetry_merkha() {
        // No Merkha
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // One Merkha
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Tsinnorit + Merkha (1w)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Tsinnorit + Merkha (2w)
        let sc = SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Tsinnorit + Merkha (3w)
        let sc = SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Oleh + Merkha (1w)
        let sc = SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Oleh + Merkha (2w)
        let sc = SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
        // Oleh + Merkha (3w)
        let sc = SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    }

    #[test]
    fn test_find_poetry_illuy() {
        let sc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), Some(10));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), None);
    }
    #[test]
    fn test_find_poetry_tarkha() {
        let sc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Tarkha.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Tarkha.into()), None);
    }
    #[test]
    fn test_find_poetry_galgal() {
        let sc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Galgal.into()), Some(12));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Galgal.into()), None);
    }
    #[test]
    fn test_find_poetry_mehuppakh() {
        // No Mehuppach
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Tsinnorit Mappach (one word)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Tsinnorit Mappach (two words)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Tsinnorit Mappach (three words)
        let sc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);

        // One Mehuppach, part of Mehuppach Legarmeh (no space) TODO
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Mehuppach Legarmeh (one space) TODO
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Mehuppach Legarmeh (no space - vertical line)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Mehuppach Legarmeh (one space - vertical line)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of 'Mehuppach Legarmeh' (too many spaces)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
        let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
        // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
        let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    }

    #[test]
    fn test_find_poetry_azla() {
        // contains Azla
        let sc = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
        // contains Azla and Azla Legarmeh
        let sc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
        // Azla Legarmeh, with space + Paseq
        let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
        // Azla Legarmeh, with Paseq
        let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
        // Azla Legarmeh, with space + Vertical Bar
        let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
        // Azla Legarmeh, with Vertical Bar
        let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), None);
    }
    #[test]
    fn test_find_poetry_shalshelet_qetannah() {
        // Shalshelet
        let sc = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
            None
        );
        // Shalshelet Gadol, with Paseq
        let sc = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
            None
        );
        // Shalshelet Gadol, with Vertical Bar
        let sc = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
            None
        );
    }
    #[test]
    fn test_find_poetry_tsinnorit_merkha() {
        // accent in a single word
        let sc = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
            Some(0)
        );
        // accent in a single word, without Tsinnorit
        let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in a single word, without Merkha
        let sc = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in two words seperated by Maqqeph
        let sc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
            Some(0)
        );
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in two words seperated by Maqqeph, without Merkha
        let sc = SentenceContext::new("את־א֘ברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in two words
        let sc = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
            Some(0)
        );
        // accent in two words, without Tsinnorit
        let sc = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in two words, without Merkha
        let sc = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
        // accent in three words
        let sc = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    }
    #[test]
    fn test_find_poetry_tsinnorit_mahpakh() {
        // accent in a single word
        let sc = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
            Some(0)
        );
        // accent in a single word
        let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in a single word, without Tsinnorit
        let sc = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sc = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
            Some(0)
        );
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sc = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in two words
        let sc = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
        assert_eq!(
            sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
            Some(0)
        );
        // accent in two words, without Tsinnorit
        let sc = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in two words, without Mahpakh
        let sc = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
        // accent in three words
        let sc = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    }

}
