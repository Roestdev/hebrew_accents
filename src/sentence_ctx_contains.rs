//! Implementation of contains_accent() for 'SentenceContext'

// Standard library

// External crates

// Local modules / crate‑internal
use crate::char::{
    DARGA, DEHI, ETNAHTA, GERESH, GERSHAYIM, ILUY, MAHPAKH, MAQQEPH, MERKHA, MERKHA_KEFULA, MUNAH,
    PASHTA, PAZER, QADMA, QARNEY_PARA, REVIA, SEGOL, TELISHA_GEDOLA, TELISHA_QETANA, TEVIR, TIPEHA,
    YERAH_BEN_YOMO, YETIV, ZAQEF_GADOL, ZAQEF_QATAN, ZARQA, ZINOR,
};
use crate::sentence_ctx_funcs::{
    find_poetry_mehuppakh, find_poetry_merkha, find_poetry_revia_gadol, find_poetry_revia_qaton,
};
use crate::sentence_ctx_regex::{
    FA_RE_OUTER_COMMON_METEG, FA_RE_OUTER_COMMON_SILLUQ, FA_RE_OUTER_POETRY_AZLA,
    FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH, FA_RE_OUTER_PROSE_MUNACH, RE_OUTER_COMMON_SHALSHELET,
    RE_OUTER_POETRY_AZLA_LEGARMEH, RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH,
    RE_OUTER_POETRY_OLEH_WE_YORED, RE_OUTER_POETRY_REVIA_MUGRASH,
    RE_OUTER_POETRY_TSINNORIT_MAHPAKH, RE_OUTER_POETRY_TSINNORIT_MERKHA, RE_OUTER_PROSE_LEGARMEH,
    RE_OUTER_PROSE_MEAYLA,
};
use crate::{Context, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent, SentenceContext};

impl SentenceContext {
    /// Returns true if the accent is present in the sentence
    /// taking into account the context
    ///  
    /// # Example
    /// ```
    /// use hebrew_accents::{SentenceContext,Context,HebrewAccent,ProseAccent,PoetryAccent};
    ///
    /// let sentence_context = SentenceContext::new("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",Context::Prosaic,);
    /// assert!(sentence_context.contains_accent(ProseAccent::Silluq.into()));
    /// assert!(!sentence_context.contains_accent(ProseAccent::Segolta.into()));
    /// assert!(!sentence_context.contains_accent(PoetryAccent::ReviaGadol.into()));
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
            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.ctx == Context::Prosaic => {
                self.sentence.contains(ZAQEF_QATAN)
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.ctx == Context::Prosaic => {
                self.sentence.contains(ZAQEF_GADOL)
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.ctx == Context::Prosaic => {
                self.sentence.contains(REVIA)
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarcha) => self.sentence.contains(TIPEHA),
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
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph) => self.sentence.contains(MAQQEPH),
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.ctx == Context::Poetic => {
                RE_OUTER_POETRY_OLEH_WE_YORED.is_match(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.ctx == Context::Poetic => {
                find_poetry_revia_gadol(&self.sentence).is_some()
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
                find_poetry_revia_qaton(&self.sentence).is_some()
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
                find_poetry_merkha(&self.sentence).is_some()
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.ctx == Context::Poetic => {
                self.sentence.contains(ILUY)
            }
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.ctx == Context::Poetic => {
                find_poetry_mehuppakh(&self.sentence).is_some()
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
}
