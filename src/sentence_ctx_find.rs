//! Implementation of find_accent() for 'SentenceContext'

// Standard library

// External crates

// Local modules / crate‑internal
use crate::char::{
    ATNACH, AZLA, DARGA, DEHI, GALGAL, GERESH, GERSHAYIM, ILUY, MAHPAKH, MAQQEPH, MEAYLA, MERKHA,
    MERKHA_KEFULA, METEG, MUNACH, MUNAH, PASEQ, PASHTA, PAZER, PAZER_GADOL, QADMA, REVIA, SEGOLTA,
    SILLUQ, SOF_PASUQ, TARCHA, TELISHA_GEDOLA, TELISHA_QETANA, TEVIR, TIPHCHA, YETIV, ZAQEF_GADOL,
    ZAQEF_QATAN, ZARQA, ZINOR,
};
use crate::sentence_ctx_funcs::{
    find_poetry_mehuppakh, find_poetry_merkha, find_poetry_revia_gadol, find_poetry_revia_qaton,
};
use crate::sentence_ctx_regex::{
    FA_RE_OUTER_COMMON_METEG, FA_RE_OUTER_COMMON_SILLUQ, FA_RE_OUTER_POETRY_AZLA,
    FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH, FA_RE_OUTER_PROSE_MUNACH, RE_INNER_COMMON_SHALSHELET,
    RE_INNER_POETRY_TSINNORIT_MAHPAKH, RE_INNER_POETRY_TSINNORIT_MERKHA, RE_INNER_PROSE_LEGARMEH,
    RE_OUTER_COMMON_SHALSHELET, RE_OUTER_POETRY_AZLA_LEGARMEH, RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH,
    RE_OUTER_POETRY_OLEH_WE_YORED, RE_OUTER_POETRY_REVIA_MUGRASH,
    RE_OUTER_POETRY_TSINNORIT_MAHPAKH, RE_OUTER_POETRY_TSINNORIT_MERKHA, RE_OUTER_PROSE_LEGARMEH,
    RE_OUTER_PROSE_MEAYLA,
};
use crate::{
    Context, HebrewAccent, Match, PoetryAccent, ProseAccent, PseudoAccent, SentenceContext,
};

pub(crate) const ACCENT_LEN_UTF8: usize = 2;

impl SentenceContext {
    /// This routine searches for the first match of a HebrewAccent in the sentence
    /// taking into account the context.
    /// If found, it returns a [`Match`]. The `Match` provides access to both
    /// the byte offsets of the match and the actual substring that matched.
    ///
    /// Note if you just want to test the existence of a HebrewAccent,
    /// it's potentially faster to use `hebrew_accent::find_accent(HebrewAccent)`
    /// instead of `hebrew_accent::find_accent(HebrewAccent).is_some()`.
    ///
    /// # Example
    /// ```rust
    /// use hebrew_accents::{SentenceContext, Context,Match};
    ///
    /// let sc = SentenceContext::new("הִי אֽוֹר׃", Context::Prosaic);
    /// let expected = Match {
    ///    haystack: "TODO::Outermatch",
    ///    start: 9,
    ///    end: 19,
    /// };
    /// ```
    pub fn find_accent(self, accent: HebrewAccent) -> Option<Match<'static>> {
        match accent {
            /* **********************************************************
             *                          PROSE
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Prose(ProseAccent::Silluq)
            | HebrewAccent::Poetry(PoetryAccent::Silluq) => {
                if let Some(outer_match) = FA_RE_OUTER_COMMON_SILLUQ.find(&self.sentence).unwrap() {
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    Some(Match::new(SILLUQ, outer_match.start(), outer_match.end()))
                } else {
                    println!("ProseAccent::Silluq not found.");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => self
                .sentence
                .find(ATNACH)
                .map(|index| Match::new(ATNACH, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Segolta) if self.ctx == Context::Prosaic => self
                .sentence
                .find(SEGOLTA)
                .map(|index| Match::new(SEGOLTA, index, index + ACCENT_LEN_UTF8)),

            // TODOnnew code
            HebrewAccent::Prose(ProseAccent::Shalshelet) => {
                if self.ctx != Context::Prosaic {
                    // wrong context, early return
                    return None;
                }
                let outer = match RE_OUTER_COMMON_SHALSHELET.find(&self.sentence) {
                    Some(m) => {
                        println!(
                            "\n==> RE_OUTER_COMMON_SHALSHELET: FOUND!\n\
                                OUTER MATCH :: start:{} ; end:{} ; str:{}",
                            m.start(),
                            m.end(),
                            m.as_str()
                        );
                        m
                    }
                    None => {
                        println!("ProseAccent::Shalshelet is not found (outer match).");
                        return None;
                    }
                };
                let inner = match RE_INNER_COMMON_SHALSHELET.find(outer.as_str()) {
                    Some(m) => {
                        println!(
                            "\n==> RE_INNER_COMMON_SHALSHELET: found!\n\
                                INNER MATCH :: start:{} ; end:{} ; str:{}",
                            m.start(),
                            m.end(),
                            m.as_str()
                        );
                        m
                    }
                    None => {
                        println!("ProseAccent::Shalshelet is not found (inner match).");
                        return None;
                    }
                };
                let outer_start = outer.start();
                let abs_start = outer_start + inner.start();
                let abs_end = outer_start + inner.end();

                println!("Absolute start in `hay`: {}", abs_start);
                println!("Absolute end   in `hay`: {}", abs_end);

                Some(Match::new(
                    "2CodePoints", // TODO: replace with the real identifier
                    abs_start,
                    abs_end,
                ))
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQatan) if self.ctx == Context::Prosaic => self
                .sentence
                .find(ZAQEF_QATAN)
                .map(|index| Match::new(ZAQEF_QATAN, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.ctx == Context::Prosaic => self
                .sentence
                .find(ZAQEF_GADOL)
                .map(|index| Match::new(ZAQEF_GADOL, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Revia) if self.ctx == Context::Prosaic => self
                .sentence
                .find(REVIA)
                .map(|index| Match::new(REVIA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Tiphcha) => self
                .sentence
                .find(TIPHCHA)
                .map(|index| Match::new(TIPHCHA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.ctx == Context::Prosaic => self
                .sentence
                .find(ZARQA)
                .map(|index| Match::new(ZARQA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Pashta) if self.ctx == Context::Prosaic => self
                .sentence
                .find(PASHTA)
                .map(|index| Match::new(PASHTA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.ctx == Context::Prosaic => self
                .sentence
                .find(YETIV)
                .map(|index| Match::new(YETIV, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Tevir) if self.ctx == Context::Prosaic => self
                .sentence
                .find(TEVIR)
                .map(|index| Match::new(TEVIR, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Geresh) if self.ctx == Context::Prosaic => self
                .sentence
                .find(GERESH)
                .map(|index| Match::new(GERESH, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.ctx == Context::Prosaic => self
                .sentence
                .find(GERSHAYIM)
                .map(|index| Match::new(GERSHAYIM, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence
                    .find(PAZER)
                    .map(|index| Match::new(PAZER, index, index + ACCENT_LEN_UTF8))
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.ctx == Context::Prosaic => self
                .sentence
                .find(PAZER_GADOL)
                .map(|index| Match::new(PAZER_GADOL, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.ctx == Context::Prosaic => {
                self.sentence
                    .find(TELISHA_GEDOLA)
                    .map(|index| Match::new(TELISHA_GEDOLA, index, index + ACCENT_LEN_UTF8))
            }
            HebrewAccent::Prose(ProseAccent::Legarmeh) => {
                if let Some(outer_match) = RE_OUTER_PROSE_LEGARMEH.find(&self.sentence) {
                    println!("\n==> RE_OUTER_PROSE_LEGARMEH: FOUND!");
                    println!(
                        "OUTER MATCH:: start():{}; end():{}; asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) = RE_INNER_PROSE_LEGARMEH.find(outer_match.as_str()) {
                        println!(
                            "INNER MATCH:: start():{}; end():{}; asstr():  {}",
                            inner_match.start(),
                            inner_match.end(),
                            inner_match.as_str()
                        );
                        let absolute_inner_start = outer_start + inner_match.start();
                        let absolute_inner_end = outer_start + inner_match.end();
                        println!("Absolute start in `hay`: {}", absolute_inner_start);
                        println!("Absolute end in `hay`: {}", absolute_inner_end);
                        Some(Match::new(
                            "2CodePoints", //TODO
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("Narrow pattern not found inside the first match.");
                        None
                    }
                } else {
                    println!("No ProseAccent::Legarmeh.");
                    None
                }
            }
            // Conjunctives
            HebrewAccent::Prose(ProseAccent::Munach) if self.ctx == Context::Prosaic => {
                if let Some(outer_match) = FA_RE_OUTER_PROSE_MUNACH.find(&self.sentence).unwrap() {
                    println!("\n==> FA_RE_OUTER_COMMON_METEG: FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    Some(Match::new(MUNACH, outer_match.start(), outer_match.end()))
                } else {
                    println!("Outer pattern not found for MUNACH (Prose).");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.ctx == Context::Prosaic => self
                .sentence
                .find(MAHPAKH)
                .map(|index| Match::new(MAHPAKH, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Merkha) if self.ctx == Context::Prosaic => self
                .sentence
                .find(MERKHA)
                .map(|index| Match::new(MERKHA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.ctx == Context::Prosaic => {
                self.sentence
                    .find(MERKHA_KEFULA)
                    .map(|index| Match::new(MERKHA_KEFULA, index, index + ACCENT_LEN_UTF8))
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.ctx == Context::Prosaic => self
                .sentence
                .find(DARGA)
                .map(|index| Match::new(DARGA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Azla) if self.ctx == Context::Prosaic => self
                .sentence
                .find(QADMA)
                .map(|index| Match::new(QADMA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.ctx == Context::Prosaic => {
                self.sentence
                    .find(TELISHA_QETANA)
                    .map(|index| Match::new(TELISHA_QETANA, index, index + ACCENT_LEN_UTF8))
            }
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => self
                .sentence
                .find(GALGAL)
                .map(|index| Match::new(GALGAL, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Prose(ProseAccent::Mayela) if self.ctx == Context::Prosaic => {
                match RE_OUTER_PROSE_MEAYLA.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_PROSE_MEAYLA found");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(MEAYLA, outer_match.start(), outer_match.end()))
                    }
                    None => {
                        println!("RE_OUTER_PROSE_MEAYLA not found.");
                        None
                    }
                }
            }
            HebrewAccent::Prose(ProseAccent::Meteg) | HebrewAccent::Poetry(PoetryAccent::Meteg) => {
                if let Some(outer_match) = FA_RE_OUTER_COMMON_METEG.find(&self.sentence).unwrap() {
                    println!("\n==> FA_RE_OUTER_COMMON_METEG: FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    Some(Match::new(METEG, outer_match.start(), outer_match.end()))
                } else {
                    println!("\n==> FA_RE_OUTER_COMMON_METEG: NOT FOUND!");
                    None
                }
            }
            /* **********************************************************
             *                          POETRY
             * *********************************************************/
            // Disjunctives
            HebrewAccent::Poetry(PoetryAccent::OlehWeYored) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_OLEH_WE_YORED.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_POETRY_OLEH_WE_YORED: FOUND!");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(
                            "2CodePoints", //TODO
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("PoetryAccent::OlehWeYored not found!");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaGadol) if self.ctx == Context::Poetic => {
                find_poetry_revia_gadol(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaMugrash) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_REVIA_MUGRASH.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_POETRY_REVIA_MUGRASH: FOUND!");
                        println!(
                            "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                            outer_match.start(),
                            outer_match.end(),
                            outer_match.as_str()
                        );
                        Some(Match::new(
                            "2CodePoints", //TODO
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("PoetryAccent::ReviaMugrash NOT FOUND!");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol) if self.ctx == Context::Poetic => {
                if let Some(outer_match) = RE_OUTER_COMMON_SHALSHELET.find(&self.sentence) {
                    println!("\n==> RE_OUTER_COMMON_SHALSHELET: FOUND!");
                    println!(
                        "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) = RE_INNER_COMMON_SHALSHELET.find(outer_match.as_str())
                    {
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
                        Some(Match::new(
                            "2CodePoints", //TODO
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("PoetryAccent::ShalsheletGadol not found.");
                        None
                    }
                } else {
                    println!("PoetryAccent::ShalsheletGadol not found.");
                    None
                }
            }
            HebrewAccent::Poetry(PoetryAccent::Tsinnor) if self.ctx == Context::Poetic => self
                .sentence
                .find(ZINOR)
                .map(|index| Match::new(ZINOR, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.ctx == Context::Poetic => {
                find_poetry_revia_qaton(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.ctx == Context::Poetic => self
                .sentence
                .find(DEHI)
                .map(|index| Match::new(DEHI, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
                if self.ctx == Context::Poetic =>
            {
                match RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH found");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(
                            "2CodePoints", //TODO
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("PoetryAccent::MehuppakhLegarmeh not found");
                        None
                    }
                }
            }
            HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) if self.ctx == Context::Poetic => {
                match RE_OUTER_POETRY_AZLA_LEGARMEH.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_POETRY_AZLA_LEGARMEH found!");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(
                            "2CodePoints", //TODO
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("PoetryAccent::AzlaLegarmeh not found.");
                        None
                    }
                }
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.ctx == Context::Poetic => self
                .sentence
                .find(MUNAH)
                .map(|index| Match::new(MUNAH, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.ctx == Context::Poetic => {
                find_poetry_merkha(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.ctx == Context::Poetic => self
                .sentence
                .find(ILUY)
                .map(|index| Match::new(ILUY, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Poetry(PoetryAccent::Tarcha) => self
                .sentence
                .find(TARCHA)
                .map(|index| Match::new(TARCHA, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Poetry(PoetryAccent::Mehuppakh) if self.ctx == Context::Poetic => {
                find_poetry_mehuppakh(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Azla) if self.ctx == Context::Poetic => {
                match FA_RE_OUTER_POETRY_AZLA.find(&self.sentence).unwrap() {
                    Some(outer_match) => {
                        println!("\n==> FA_RE_OUTER_POETRY_AZLA found!");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(AZLA, outer_match.start(), outer_match.end()))
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
                if let Some(outer_match) = FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH
                    .find(&self.sentence)
                    .unwrap()
                {
                    println!("\n==> FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    Some(Match::new(
                        "TODO::Outermatch",
                        outer_match.start(),
                        outer_match.end(),
                    ))
                } else {
                    println!("Outer pattern not found for FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH.");
                    None
                }
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha) if self.ctx == Context::Poetic => {
                if let Some(outer_match) = RE_OUTER_POETRY_TSINNORIT_MERKHA.find(&self.sentence) {
                    println!("\n==> RE_OUTER_POETRY_TSINNORIT_MERKHA: found!");
                    println!(
                        "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) =
                        RE_INNER_POETRY_TSINNORIT_MERKHA.find(outer_match.as_str())
                    {
                        println!("\n==> RE_INNER_POETRY_TSINNORIT_MERKHA: found!");
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
                        Some(Match::new(
                            "2CodePoints", //TODO
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("TPoetryAccent::TsinnoritMerkha is not found (inner match");
                        None
                    }
                } else {
                    println!("PoetryAccent::TsinnoritMerkha is not found (outer match).");
                    None
                }
            }
            HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh) if self.ctx == Context::Poetic => {
                if let Some(outer_match) = RE_OUTER_POETRY_TSINNORIT_MAHPAKH.find(&self.sentence) {
                    println!("\n==> RE_OUTER_POETRY_TSINNORIT_MAHPAKH: found!");
                    println!(
                        "OUTER MATCH:: start():{}  ;end():{}  ;str():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    if let Some(inner_match) =
                        RE_INNER_POETRY_TSINNORIT_MAHPAKH.find(outer_match.as_str())
                    {
                        println!("\n==> RE_INNER_POETRY_TSINNORIT_MAHPAKH: found!");
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
                        Some(Match::new(
                            "2CodePoints", //TODO
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("PoetryAccent::TsinnoritMahpak is not found (inner match");
                        None
                    }
                } else {
                    println!("PoetryAccent::TsinnoritMahpak is not found (outer match).");
                    None
                }
            }
            /* **********************************************************
             *                          PSEUDO
             * *********************************************************/
            HebrewAccent::Pseudo(PseudoAccent::SophPasuq) => self
                .sentence
                .find(SOF_PASUQ)
                .map(|index| Match::new(SOF_PASUQ, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph) => self
                .sentence
                .find(MAQQEPH)
                .map(|index| Match::new(MAQQEPH, index, index + ACCENT_LEN_UTF8)),
            HebrewAccent::Pseudo(PseudoAccent::Paseq) => self
                .sentence
                .find(PASEQ)
                .map(|index| Match::new(PASEQ, index, index + ACCENT_LEN_UTF8)),
            _ => None,
        }
    }
}

#[cfg(test)]
#[test]
fn test_find_prose_poetry_silluq() {
    // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
    let sc = SentenceContext::new("הִי אֽוֹר׃", Context::Prosaic);
    let expected = Match {
        haystack: SILLUQ,
        start: 9,
        end: 19,
    };
    assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(expected));
    // ProseAccent, with Soph Pasuq, no Pey or Samech
    let sc = SentenceContext::new(
        "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: SILLUQ,
        start: 159,
        end: 168,
    };
    assert_eq!(sc.find_accent(ProseAccent::Silluq.into()), Some(expected));
    // ProseAccent, no Soph Paseq, with Pey
    let sc = SentenceContext::new(
        "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: SILLUQ,
        start: 165,
        end: 175,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(expected));
    // PoetryAccent with Soph Paseq and Peh
    let sc = SentenceContext::new(
        "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: SILLUQ,
        start: 159,
        end: 171,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Silluq.into()), Some(expected));
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
    let expected = Match {
        haystack: ATNACH,
        start: 52,
        end: 54,
    };
    assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), Some(expected));
    // No Atnach present
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Atnach.into()), None);
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Atnach.into()), None);
}
#[test]
fn test_find_prose_segolta() {
    let sc = SentenceContext::new(
        " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: SEGOLTA,
        start: 67,
        end: 69,
    };
    assert_eq!(sc.find_accent(ProseAccent::Segolta.into()), Some(expected));
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
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Paseq + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Vertical Bar - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Shalshelet, with Vertical Bar + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Shalshelet.into()),
        Some(expected)
    );
    // Missing Paseq or Vertical Bar
    let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Shalshelet.into()), None);
}
#[test]
fn test_find_prose_zaqeph_qaton() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: ZAQEF_QATAN,
        start: 63,
        end: 65,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::ZaqephQatan.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::ZaqephQatan.into()), None);
}
#[test]
fn test_find_prose_zaqeph_gadol() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: ZAQEF_GADOL,
        start: 48,
        end: 50,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::ZaqephGadol.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::ZaqephGadol.into()), None);
}
#[test]
fn test_find_prose_revia() {
    let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
    let expected = Match {
        haystack: REVIA,
        start: 44,
        end: 46,
    };
    assert_eq!(sc.find_accent(ProseAccent::Revia.into()), Some(expected));
    let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Revia.into()), None);
}
#[test]
fn test_find_prose_tiphcha() {
    let sc = SentenceContext::new(
        "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
        Context::Prosaic,
    );
    let expected = Match {
        haystack: TIPHCHA,
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(expected));
    let sc = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
    let expected = Match {
        haystack: TIPHCHA,
        start: 9,
        end: 11,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tiphcha.into()), Some(expected));
}
#[test]
fn test_find_prose_zarqa() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
    let expected = Match {
        haystack: ZARQA,
        start: 120,
        end: 122,
    };
    assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Zarqa.into()), None);
}
#[test]
fn test_find_prose_pashta() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: PASHTA,
        start: 44,
        end: 46,
    };
    assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Pashta.into()), None);
}
#[test]
fn test_find_prose_yetiv() {
    let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
    let expected = Match {
        haystack: YETIV,
        start: 36,
        end: 38,
    };
    assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), Some(expected));
    let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Yetiv.into()), None);
}
#[test]
fn test_find_prose_tevir() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: TEVIR,
        start: 84,
        end: 86,
    };
    assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Tevir.into()), None);
}
#[test]
fn test_find_prose_geresh() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: GERESH,
        start: 78,
        end: 80,
    };
    assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Geresh.into()), None);
}
#[test]
fn test_find_prose_gershayim() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: GERSHAYIM,
        start: 18,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::Gershayim.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Gershayim.into()), None);
}
#[test]
fn test_find_prose_pazer() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: PAZER,
        start: 99,
        end: 101,
    };
    assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Pazer.into()), None);
}
#[test]
fn test_find_prose_pazer_gadol() {
    let sc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: PAZER_GADOL,
        start: 12,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::PazerGadol.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::PazerGadol.into()), None);
}
#[test]
fn test_find_prose_telisha_gadolah() {
    let sc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: TELISHA_GEDOLA, // TODO naam niet consequent
        start: 12,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::TelishaGedolah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::TelishaGedolah.into()), None);
}
#[test]
fn test_find_prose_legarmeh() {
    // Legarmeh, with Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 52,
        end: 60,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); //  - 60
                                                                              // Legarmeh with a space + Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 52,
        end: 61,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 61
                                                                              // Legarmeh with two spaces + Paseq
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), None);
    // Legarmeh, with Vertical Bar
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 52,
        end: 59,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 59
                                                                              // Legarmeh, with space + Vertical Bar
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 52,
        end: 60,
    };
    assert_eq!(sc.find_accent(ProseAccent::Legarmeh.into()), Some(expected)); // 40 - 60
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
    let expected = Match {
        haystack: MUNACH,
        start: 28,
        end: 30,
    };
    assert_eq!(sc.find_accent(ProseAccent::Munach.into()), Some(expected));
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
    let expected = Match {
        haystack: MAHPAKH,
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), Some(expected));
    let sc = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Mahpakh.into()), None);
}
#[test]
fn test_find_prose_merkha() {
    let sc = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
    let expected = Match {
        haystack: MERKHA,
        start: 6,
        end: 8,
    };
    assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), Some(expected));
    let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Merkha.into()), None);
}
#[test]
fn test_find_prose_merkha_kephulah() {
    let sc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
    let expected = Match {
        haystack: MERKHA_KEFULA,
        start: 18,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::MerkhaKephulah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::MerkhaKephulah.into()), None);
}
#[test]
fn test_find_prose_darga() {
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: DARGA,
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(ProseAccent::Darga.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Darga.into()), None);
}
#[test]
fn test_find_prose_azla() {
    use crate::char::AZLA;
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: AZLA,
        start: 39,
        end: 41,
    };
    assert_eq!(sc.find_accent(ProseAccent::Azla.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Azla.into()), None);
}
#[test]
fn test_find_prose_telisha_qetannah() {
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: TELISHA_QETANA,
        start: 61,
        end: 63,
    };
    assert_eq!(
        sc.find_accent(ProseAccent::TelishaQetannah.into()),
        Some(expected)
    );
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::TelishaQetannah.into()), None);
}
#[test]
fn test_find_prose_galgal() {
    let sc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    let expected = Match {
        haystack: GALGAL,
        start: 23,
        end: 25,
    };
    assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), Some(expected));
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
    assert_eq!(sc.find_accent(ProseAccent::Galgal.into()), None);
}
#[test]
fn test_find_prose_meayla() {
    // Tiphcha followed by Atnach
    let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
    let expected = Match {
        haystack: MEAYLA,
        start: 48,
        end: 56,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
    // Tiphcha followed by Atnach, two words connected with a Maqqeph
    let sc = SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
    let expected = Match {
        haystack: MEAYLA,
        start: 6,
        end: 18,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
    // Tiphcha followed by silluq
    let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
    let expected = Match {
        haystack: MEAYLA,
        start: 104,
        end: 114,
    };
    assert_eq!(sc.find_accent(ProseAccent::Mayela.into()), Some(expected));
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
    let expected = Match {
        haystack: METEG,
        start: 48,
        end: 50,
    };
    assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(expected));
    // Meteg and Siluq in separate words
    let sc = SentenceContext::new(
        "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
        Context::Poetic,
    );
    let expected = Match {
        haystack: METEG,
        start: 30,
        end: 32,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(expected));
    // Only Meteg, no Silluq
    let sc = SentenceContext::new("וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Prosaic);
    let expected = Match {
        haystack: METEG,
        start: 2,
        end: 4,
    };
    assert_eq!(sc.find_accent(ProseAccent::Meteg.into()), Some(expected));
    // Two Meteg's, no Silluq
    let sc = SentenceContext::new("ום וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ", Context::Poetic);
    let expected = Match {
        haystack: METEG,
        start: 7,
        end: 9,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Meteg.into()), Some(expected));
}
/* **********************************************************
 *                          POETRY
 * *********************************************************/
#[test]
fn test_find_poetry_oleh_we_yored() {
    // OlehWeYored, one word
    let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 34,
        end: 44,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::OlehWeYored.into()),
        Some(expected)
    );
    // OlehWeYored, one word - context: Prosaic
    let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
    assert_eq!(sc.find_accent(PoetryAccent::OlehWeYored.into()), None);
    // OlehWeYored, two words
    let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 26,
        end: 37,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::OlehWeYored.into()),
        Some(expected)
    );
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
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
    // Revia followed by Oleh We Yored (1 word)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    // Revia followed by Oleh We Yored (2 words)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ReviaGadol.into()), None);
    // Revia followed by 'Oleh We Yored' (3 words)
    let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
    // Revia not directly followed by Oleh We Yored (1 word)
    let sc = SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 3,
        end: 5,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaGadol.into()),
        Some(expected)
    );
}
#[test]
fn test_find_poetry_revia_mugrash() {
    // Revia and Geresh (Ps 32:3)
    let sc = SentenceContext::new("בְּ֝שַׁאֲגָתִ֗י", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 6,
        end: 28,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaMugrash.into()),
        Some(expected)
    );
    // Revia and Geresh (Ps 110:6) - accent on a single character
    let sc = SentenceContext::new("יָדִ֣ין בַּ֭גּוֹיִם מָלֵ֣א גְוִיּ֑וֹת מָ֥חַץ רֹ֝֗אשׁ עַל־אֶ֥רֶץ רַבָּֽה׃", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 89,
        end: 93,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaMugrash.into()),
        Some(expected)
    );
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
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Paseq + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Vertical Bar - no space
    let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Shalshelet Gadol, with Vertical Bar + one space
    let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 16,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletGadol.into()),
        Some(expected)
    );
    // Missing Paseq or Vertical Bar
    let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::ShalsheletGadol.into()), None);
}
#[test]
fn test_find_poetry_tsinnor() {
    let sc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
    let expected = Match {
        haystack: ZINOR,
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Tsinnor.into()), Some(expected));
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
    let expected = Match {
        haystack: "\u{597}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaQaton.into()),
        Some(expected)
    );
    // Revia directly followed by Oleh We Yored (2 words)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{597}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ReviaQaton.into()),
        Some(expected)
    );
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
    let expected = Match {
        haystack: DEHI, // TODO naam niet consequent
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Dechi.into()), None);
}
#[test]
fn test_find_poetry_pazer() {
    let sc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
    let expected = Match {
        haystack: PAZER,
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Pazer.into()), None);
}
#[test]
fn test_find_poetry_mehuppakh_legarmeh() {
    // MehuppakhLegarmeh, with Paseq
    let sc = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 17,
        end: 22,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
        Some(expected)
    );
    // MehuppakhLegarmeh, with Vertical Bar
    let sc = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 17,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()),
        Some(expected)
    );
    // Mehuppakh only
    let sc = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::MehuppakhLegarmeh.into()), None);
}
#[test]
fn test_find_poetry_azla_legarmeh() {
    // AzlaLegarmeh, with Paseq + no space
    let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 15,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Paseq + 1 space
    let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 15,
        end: 22,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Vertical Bar + no space
    let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 15,
        end: 20,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // AzlaLegarmeh, with Vertical Bar + 1 space
    let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 15,
        end: 21,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::AzlaLegarmeh.into()),
        Some(expected)
    );
    // Azla only
    let sc = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::AzlaLegarmeh.into()), None);
}
#[test]
fn test_find_poetry_munnach() {
    let expected = Match {
        haystack: MUNAH,
        start: 12,
        end: 14,
    };
    let sc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Munach.into()), Some(expected));
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
    let expected = Match {
        haystack: "\u{5a5}",
        start: 21,
        end: 23,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
    // Tsinnorit + Merkha (1w)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Tsinnorit + Merkha (2w)
    let sc = SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Tsinnorit + Merkha (3w)
    let sc = SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a5}",
        start: 22,
        end: 24,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
    // Oleh + Merkha (1w)
    let sc = SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Oleh + Merkha (2w)
    let sc = SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), None);
    // Oleh + Merkha (3w)
    let sc = SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a5}",
        start: 22,
        end: 24,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Merkha.into()), Some(expected));
}

#[test]
fn test_find_poetry_illuy() {
    let sc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
    let expected = Match {
        haystack: ILUY,
        start: 10,
        end: 12,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Illuy.into()), None);
}
#[test]
fn test_find_poetry_tarcha() {
    let sc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
    let expected = Match {
        haystack: TARCHA, // TODO naam niet consequent
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Tarcha.into()), Some(expected));
    let sc = SentenceContext::new("את־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Tarcha.into()), None);
}
#[test]
fn test_find_poetry_galgal() {
    let sc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
    let expected = Match {
        haystack: GALGAL,
        start: 12,
        end: 14,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Galgal.into()), Some(expected));
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
    let expected = Match {
        haystack: "\u{5a4}",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Tsinnorit Mappach (one word)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Tsinnorit Mappach (two words)
    let sc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Tsinnorit Mappach (three words)
    let sc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 22,
        end: 24,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Mehuppach Legarmeh (no space)
    let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::Mehuppakh.into()), None);
    // One Mehuppach, part of Mehuppach Legarmeh (one space)
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
    let expected = Match {
        haystack: "\u{5a4}",
        start: 33,
        end: 35,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
    let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 4,
        end: 6,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
    // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
    let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
    let expected = Match {
        haystack: "\u{5a4}",
        start: 4,
        end: 6,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::Mehuppakh.into()),
        Some(expected)
    );
}

#[test]
fn test_find_poetry_azla() {
    use crate::char::AZLA;

    // contains Azla
    let sc = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
    let expected = Match {
        haystack: AZLA,
        start: 15,
        end: 17,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(expected));
    // contains Azla and Azla Legarmeh
    let sc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
    let expected = Match {
        haystack: AZLA,
        start: 5,
        end: 11,
    };
    assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(expected));
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
    let expected = Match {
        haystack: "TODO::Outermatch",
        start: 21,
        end: 23,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::ShalsheletQetannah.into()),
        Some(expected)
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
    let expected = Match {
        haystack: "2CodePoints",
        start: 4,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
    );
    // accent in a single word, without Tsinnorit
    let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in a single word, without Merkha
    let sc = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words seperated by Maqqeph
    let sc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 8,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
    );
    // accent in two words seperated by Maqqeph, without Tsinnorit
    let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words seperated by Maqqeph, without Merkha
    let sc = SentenceContext::new("את־א֘ברהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMerkha.into()), None);
    // accent in two words
    let sc = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 8,
        end: 19,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMerkha.into()),
        Some(expected)
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
    let expected = Match {
        haystack: "2CodePoints",
        start: 8,
        end: 14,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
    );
    // accent in a single word
    let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in a single word, without Tsinnorit
    let sc = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words seperated by Maqqeph, without Mahpakh
    let sc = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 4,
        end: 16,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
    );
    // accent in two words seperated by Maqqeph, without Tsinnorit
    let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words seperated by Maqqeph, without Mahpakh
    let sc = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
    assert_eq!(sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()), None);
    // accent in two words
    let sc = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
    let expected = Match {
        haystack: "2CodePoints",
        start: 8,
        end: 29,
    };
    assert_eq!(
        sc.find_accent(PoetryAccent::TsinnoritMahpakh.into()),
        Some(expected)
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

/* **********************************************************
 *                          PSEUDO
 * *********************************************************/

#[test]
fn test_find_pseudo_soph_pasuq() {
    // No Soph Pasuq
    let sc = SentenceContext::new(
        "כִּ֤י אִ֥ם בְּתוֹרַ֥ת יְהוָ֗ה חֶ֫פְצ֥וֹ וּֽבְתוֹרָת֥וֹ יֶהְגֶּ֗ה יוֹמָ֥ם וָלָֽיְלָה",
        Context::Prosaic,
    );
    assert_eq!(sc.find_accent(PseudoAccent::SophPasuq.into()), None);
    // One Soph Pasuq at the end
    let sc = SentenceContext::new(
        "כִּ֤י אִ֥ם בְּתוֹרַ֥ת יְהוָ֗ה חֶ֫פְצ֥וֹ וּֽבְתוֹרָת֥וֹ יֶהְגֶּ֗ה יוֹמָ֥ם וָלָֽיְלָה׃",
        Context::Poetic,
    );
    let expected = Match {
        haystack: SOF_PASUQ,
        start: 158,
        end: 160,
    };
    assert_eq!(
        sc.find_accent(PseudoAccent::SophPasuq.into()),
        Some(expected)
    );
    // One Soph Pasuq in the middle
    let sc = SentenceContext::new("אלהים ׃ יה֣י", Context::Poetic);
    let expected = Match {
        haystack: SOF_PASUQ,
        start: 11,
        end: 13,
    };
    assert_eq!(
        sc.find_accent(PseudoAccent::SophPasuq.into()),
        Some(expected)
    );
}

#[test]
fn test_find_pseudo_maqqeph() {
    // No Maqqeph
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PseudoAccent::Maqqeph.into()), None);
    // One Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: MAQQEPH,
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(PseudoAccent::Maqqeph.into()), Some(expected));
    // No Maqqeph
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PseudoAccent::Maqqeph.into()), None);
    // One Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: MAQQEPH,
        start: 56,
        end: 58,
    };
    assert_eq!(sc.find_accent(PseudoAccent::Maqqeph.into()), Some(expected));
}

#[test]
fn test_find_pseudo_paseq() {
    // No Maqqeph
    let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    assert_eq!(sc.find_accent(PseudoAccent::Paseq.into()), None);
    // One Maqqeph
    let sc = SentenceContext::new("ויּ֥אמר אלה֖ים׀ יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: PASEQ,
        start: 27,
        end: 29,
    };
    assert_eq!(sc.find_accent(PseudoAccent::Paseq.into()), Some(expected));
    // Two Maqqeph's
    let sc = SentenceContext::new("ויּ֥אמר׀ אלה֖ים׀ יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    let expected = Match {
        haystack: PASEQ,
        start: 14,
        end: 16,
    };
    assert_eq!(sc.find_accent(PseudoAccent::Paseq.into()), Some(expected));
    // No Maqqeph
    // let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
    // assert_eq!(sc.find_accent(PseudoAccent::Paseq.into()), None);
    // // One Maqqeph
    // let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
    // let expected = Match {
    //     haystack: PASEQ,
    //     start: 56,
    //     end: 58,
    // };
    // assert_eq!(sc.find_accent(PseudoAccent::Paseq.into()), Some(expected));
}
