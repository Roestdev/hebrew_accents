// Standard library

// External crates

// Local modules / crate‑internal
use crate::char::{
    DARGA, DEHI, ETNAHTA, GERESH, GERSHAYIM, ILUY, MAHPAKH, MAQQEPH, MERKHA, MERKHA_KEFULA, MUNAH,
    PASHTA, PAZER, QADMA, QARNEY_PARA, REVIA, SEGOL, TELISHA_GEDOLA, TELISHA_QETANA, TEVIR, TIPEHA,
    YERAH_BEN_YOMO, YETIV, ZAQEF_GADOL, ZAQEF_QATAN, ZARQA, ZINOR,
};
use crate::sentence_ctx_regex::{
    FA_RE_OUTER_COMMON_METEG, FA_RE_OUTER_COMMON_SILLUQ, FA_RE_OUTER_POETRY_AZLA,
    FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH, FA_RE_OUTER_PROSE_MUNACH, RE_INNER_COMMON_SHALSHELET,
    RE_INNER_PROSE_LEGARMEH, RE_OUTER_COMMON_SHALSHELET, RE_OUTER_POETRY_AZLA_LEGARMEH,
    RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH, RE_OUTER_POETRY_OLEH_WE_YORED,
    RE_OUTER_POETRY_REVIA_MUGRASH, RE_OUTER_POETRY_TSINNORIT_MAHPAKH,
    RE_OUTER_POETRY_TSINNORIT_MERKHA, RE_OUTER_PROSE_LEGARMEH, RE_OUTER_PROSE_MEAYLA,
};
use crate::{Context, HebrewAccent, PoetryAccent, ProseAccent, SentenceContext};

impl SentenceContext {
    // TODO examples
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
                match FA_RE_OUTER_POETRY_AZLA.find(&self.sentence).unwrap() {
                    Some(m) => {
                        println!("Found FA_RE_OUTER_POETRY_AZLA!");
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
                if let Some(outer_match) = FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH
                    .find(&self.sentence)
                    .unwrap()
                {
                    println!("FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH FOUND!");
                    println!(
                        "OUTER MATCH--start():{}‑-end():{}‑-asstr():  {}",
                        outer_match.start(),
                        outer_match.end(),
                        outer_match.as_str()
                    );
                    let outer_start = outer_match.start();
                    Some(outer_start)
                } else {
                    println!("Outer pattern not found for FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH.");
                    None
                }
                //Some(10000) // TODO
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
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(15));
        // contains Azla and Azla Legarmeh
        let sc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        assert_eq!(sc.find_accent(PoetryAccent::Azla.into()), Some(5));
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
            Some(21)
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
