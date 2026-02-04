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
    /// # Example // TODO
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
                    Some(Match::new(
                        "TODO::Outermatch",
                        outer_match.start(),
                        outer_match.end(),
                    ))
                } else {
                    println!("Outer pattern not found for SILLUQ.");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Atnach)
            | HebrewAccent::Poetry(PoetryAccent::Atnach) => {
                self.sentence.find(ETNAHTA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ETNAHTA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Segolta) if self.ctx == Context::Prosaic => {
                self.sentence.find(SEGOL).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + SEGOL.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Shalshelet) if self.ctx == Context::Prosaic => {
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
                        println!("\n==> RE_INNER_COMMON_SHALSHELET: found!");
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
                            "TODO::2CodePoints",
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("Shalshalet is not found (inner match");
                        None
                    }
                } else {
                    println!("Shalshalet is not found (outer match).");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::ZaqephQaton) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZAQEF_QATAN).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ZAQEF_QATAN.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::ZaqephGadol) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZAQEF_GADOL).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ZAQEF_GADOL.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Revia) if self.ctx == Context::Prosaic => {
                self.sentence.find(REVIA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + REVIA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Tiphcha)
            | HebrewAccent::Poetry(PoetryAccent::Tarcha) => {
                self.sentence.find(TIPEHA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + TIPEHA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Zarqa) if self.ctx == Context::Prosaic => {
                self.sentence.find(ZARQA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ZARQA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Pashta) if self.ctx == Context::Prosaic => {
                self.sentence.find(PASHTA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + PASHTA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Yetiv) if self.ctx == Context::Prosaic => {
                self.sentence.find(YETIV).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + YETIV.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Tevir) if self.ctx == Context::Prosaic => {
                self.sentence.find(TEVIR).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + TEVIR.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Geresh) if self.ctx == Context::Prosaic => {
                self.sentence.find(GERESH).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + GERESH.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Gershayim) if self.ctx == Context::Prosaic => {
                self.sentence.find(GERSHAYIM).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + GERSHAYIM.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Pazer) | HebrewAccent::Poetry(PoetryAccent::Pazer) => {
                self.sentence.find(PAZER).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + PAZER.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::PazerGadol) if self.ctx == Context::Prosaic => {
                self.sentence.find(QARNEY_PARA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + QARNEY_PARA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::TelishaGedolah) if self.ctx == Context::Prosaic => {
                self.sentence.find(TELISHA_GEDOLA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + TELISHA_GEDOLA.len_utf8(),
                    )
                })
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
                            "TODO::Inner Match",
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
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
                if let Some(outer_match) = FA_RE_OUTER_PROSE_MUNACH.find(&self.sentence).unwrap() {
                    println!("\n==> FA_RE_OUTER_COMMON_METEG: FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    Some(Match::new(
                        "TODO::Outer Match",
                        outer_match.start(),
                        outer_match.end(),
                    ))
                } else {
                    println!("Outer pattern not found for MUNACH (Prose).");
                    None
                }
            }
            HebrewAccent::Prose(ProseAccent::Mahpakh) if self.ctx == Context::Prosaic => {
                self.sentence.find(MAHPAKH).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + MAHPAKH.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Merkha) if self.ctx == Context::Prosaic => {
                self.sentence.find(MERKHA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + MERKHA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::MerkhaKephulah) if self.ctx == Context::Prosaic => {
                self.sentence.find(MERKHA_KEFULA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + MERKHA_KEFULA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Darga) if self.ctx == Context::Prosaic => {
                self.sentence.find(DARGA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + DARGA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Azla) if self.ctx == Context::Prosaic => {
                self.sentence.find(QADMA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + QADMA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::TelishaQetannah) if self.ctx == Context::Prosaic => {
                self.sentence.find(TELISHA_QETANA).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + TELISHA_QETANA.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Galgal)
            | HebrewAccent::Poetry(PoetryAccent::Galgal) => {
                self.sentence.find(YERAH_BEN_YOMO).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + YERAH_BEN_YOMO.len_utf8(),
                    )
                })
            }
            HebrewAccent::Prose(ProseAccent::Mayela) if self.ctx == Context::Prosaic => {
                match RE_OUTER_PROSE_MEAYLA.find(&self.sentence) {
                    Some(outer_match) => {
                        println!("\n==> RE_OUTER_PROSE_MEAYLA found");
                        println!("Matched text: {}", outer_match.as_str());
                        println!("Starts at byte index: {}", outer_match.start());
                        println!("Ends at byte index: {}", outer_match.end());
                        Some(Match::new(
                            "TODO::Outer Match",
                            outer_match.start(),
                            outer_match.end(),
                        ))
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
                    Some(Match::new(
                        "TODO::Outer Match",
                        outer_match.start(),
                        outer_match.end(),
                    ))
                } else {
                    println!("\n==> FA_RE_OUTER_COMMON_METEG: NOT FOUND!");
                    None
                }
            }
            HebrewAccent::Pseudo(PseudoAccent::Maqqeph) => {
                self.sentence.find(MAQQEPH).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + MAQQEPH.len_utf8(),
                    )
                })
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
                            "TODO::Outer Match",
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("\n==> RE_OUTER_POETRY_OLEH_WE_YORED: NOT FOUND!");
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
                            "TODO::2CodePoints",
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("\n==> RE_OUTER_POETRY_REVIA_MUGRASH: NOT FOUND!");
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
                            "TODO::2CodePoints",
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
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
                self.sentence.find(ZINOR).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ZINOR.len_utf8(),
                    )
                })
            }
            HebrewAccent::Poetry(PoetryAccent::ReviaQaton) if self.ctx == Context::Poetic => {
                find_poetry_revia_qaton(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Dechi) if self.ctx == Context::Poetic => {
                self.sentence.find(DEHI).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + DEHI.len_utf8(),
                    )
                })
            }
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
                            "TODO::Outermatch",
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("No match found for RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH.");
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
                            "TODO::Outermatch",
                            outer_match.start(),
                            outer_match.end(),
                        ))
                    }
                    None => {
                        println!("No match found for RE_OUTER_POETRY_AZLA_LEGARMEH.");
                        None
                    }
                }
            }
            // Conjunctives
            HebrewAccent::Poetry(PoetryAccent::Munach) if self.ctx == Context::Poetic => {
                self.sentence.find(MUNAH).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + MUNAH.len_utf8(),
                    )
                })
            }
            HebrewAccent::Poetry(PoetryAccent::Merkha) if self.ctx == Context::Poetic => {
                find_poetry_merkha(&self.sentence)
            }
            HebrewAccent::Poetry(PoetryAccent::Illuy) if self.ctx == Context::Poetic => {
                self.sentence.find(ILUY).map(|index| {
                    Match::new(
                        "TODO: insert single character",
                        index,
                        index + ILUY.len_utf8(),
                    )
                })
            }
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
                        Some(Match::new(
                            "TODO::Outermatch",
                            outer_match.start(),
                            outer_match.end(),
                        ))
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
                            "TODO::2CodePoints",
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("Tsinnorit Merkha is not found (inner match");
                        None
                    }
                } else {
                    println!("Tsinnorit Merkha is not found (outer match).");
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
                            "TODO::2CodePoints",
                            absolute_inner_start,
                            absolute_inner_end,
                        ))
                    } else {
                        println!("Tsinnorit Merkha is not found (inner match");
                        None
                    }
                } else {
                    println!("Tsinnorit Merkha is not found (outer match).");
                    None
                }
            }
            _ => None,
        }
    }
}
