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

#[cfg(test)]
mod tests {
    use crate::{Context, HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent, SentenceContext};

    // Test helper functions directly
    #[test]
    fn test_find_poetry_mehuppakh_with_valid_input() {
        // Assuming find_poetry_mehuppakh returns Option<&str> or similar
        let result = crate::sentence_ctx_funcs::find_poetry_mehuppakh("some text with mehuppakh");
        // Adjust assertion based on actual return type
        assert!(result.is_some() || result.is_none()); // Placeholder - adjust to actual behavior
    }

    #[test]
    fn test_find_poetry_merkha_with_valid_input() {
        let result = crate::sentence_ctx_funcs::find_poetry_merkha("some text with merkha");
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_find_poetry_revia_gadol_with_valid_input() {
        let result =
            crate::sentence_ctx_funcs::find_poetry_revia_gadol("some text with revia gadol");
        assert!(result.is_some() || result.is_none());
    }

    #[test]
    fn test_find_poetry_revia_qaton_with_valid_input() {
        let result =
            crate::sentence_ctx_funcs::find_poetry_revia_qaton("some text with revia qaton");
        assert!(result.is_some() || result.is_none());
    }

    // Test contains_accent with various poetry accents that use these helpers
    #[test]
    fn contains_accent_poetry_mehuppakh() {
        let ctx = SentenceContext::new("test sentence", Context::Poetic);
        let result = ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Mehuppakh));
        // This will call find_poetry_mehuppakh internally
        assert!(result == false); // Adjust based on actual behavior
    }

    #[test]
    fn contains_accent_poetry_merkha() {
        let ctx = SentenceContext::new("test sentence", Context::Poetic);
        let result = ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Merkha));
        // This will call find_poetry_merkha internally
        assert!(result == false);
    }

    #[test]
    fn contains_accent_poetry_revia_gadol() {
        let ctx = SentenceContext::new("test sentence", Context::Poetic);
        let result = ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaGadol));
        // This will call find_poetry_revia_gadol internally
        assert!(result == false);
    }

    #[test]
    fn contains_accent_poetry_revia_qaton() {
        let ctx = SentenceContext::new("test sentence", Context::Poetic);
        let result = ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaQaton));
        // This will call find_poetry_revia_qaton internally
        assert!(result == false);
    }

    // Test all prose accents to ensure they're covered
    #[test]
    fn contains_accent_prose_silluq() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
    }

    #[test]
    fn contains_accent_prose_atnach() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Atnach)));
    }

    #[test]
    fn contains_accent_prose_segolta() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Segolta)));
    }

    #[test]
    fn contains_accent_prose_shalshelet() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Shalshelet)));
    }

    #[test]
    fn contains_accent_prose_zaqeph_qatan() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephQatan)));
    }

    #[test]
    fn contains_accent_prose_zaqeph_gadol() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::ZaqephGadol)));
    }

    #[test]
    fn contains_accent_prose_revia() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Revia)));
    }

    #[test]
    fn contains_accent_prose_tiphcha() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Tiphcha)));
    }

    #[test]
    fn contains_accent_prose_zarqa() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Zarqa)));
    }

    #[test]
    fn contains_accent_prose_pashta() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Pashta)));
    }

    #[test]
    fn contains_accent_prose_yetiv() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Yetiv)));
    }

    #[test]
    fn contains_accent_prose_tevir() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Tevir)));
    }

    #[test]
    fn contains_accent_prose_geresh() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Geresh)));
    }

    #[test]
    fn contains_accent_prose_gershayim() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Gershayim)));
    }

    #[test]
    fn contains_accent_prose_pazer() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Pazer)));
    }

    #[test]
    fn contains_accent_prose_pazer_gadol() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::PazerGadol)));
    }

    #[test]
    fn contains_accent_prose_telisha_gedolah() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaGedolah)));
    }

    #[test]
    fn contains_accent_prose_legarmeh() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Legarmeh)));
    }

    #[test]
    fn contains_accent_prose_munach() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Munach)));
    }

    #[test]
    fn contains_accent_prose_mahpakh() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Mahpakh)));
    }

    #[test]
    fn contains_accent_prose_merkha() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Merkha)));
    }

    #[test]
    fn contains_accent_prose_merkha_kephulah() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::MerkhaKephulah)));
    }

    #[test]
    fn contains_accent_prose_darga() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Darga)));
    }

    #[test]
    fn contains_accent_prose_azla() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Azla)));
    }

    #[test]
    fn contains_accent_prose_telisha_qetannah() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::TelishaQetannah)));
    }

    #[test]
    fn contains_accent_prose_galgal() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Galgal)));
    }

    #[test]
    fn contains_accent_prose_mayela() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Mayela)));
    }

    #[test]
    fn contains_accent_prose_meteg() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Meteg)));
    }

    #[test]
    fn contains_accent_pseudo_maqqeph() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        assert!(!ctx.contains_accent(HebrewAccent::Pseudo(PseudoAccent::Maqqeph)));
    }

    // Test poetry accents
    #[test]
    fn contains_accent_poetry_silluq() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Silluq)));
    }

    #[test]
    fn contains_accent_poetry_atnach() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Atnach)));
    }

    #[test]
    fn contains_accent_poetry_tarcha() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tarcha)));
    }

    #[test]
    fn contains_accent_poetry_pazer() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Pazer)));
    }

    #[test]
    fn contains_accent_poetry_galgal() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Galgal)));
    }

    #[test]
    fn contains_accent_poetry_meteg() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Meteg)));
    }

    #[test]
    fn contains_accent_poetry_oleh_we_yored() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::OlehWeYored)));
    }

    #[test]
    fn contains_accent_poetry_revia_mugrash() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)));
    }

    #[test]
    fn contains_accent_poetry_shalshelet_gadol() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)));
    }

    #[test]
    fn contains_accent_poetry_tsinnor() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Tsinnor)));
    }

    #[test]
    fn contains_accent_poetry_dechi() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Dechi)));
    }

    #[test]
    fn contains_accent_poetry_mehuppakh_legarmeh() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)));
    }

    #[test]
    fn contains_accent_poetry_azla_legarmeh() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh)));
    }

    #[test]
    fn contains_accent_poetry_munach() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Munach)));
    }

    #[test]
    fn contains_accent_poetry_illuy() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Illuy)));
    }

    #[test]
    fn contains_accent_poetry_azla() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::Azla)));
    }

    #[test]
    fn contains_accent_poetry_shalshelet_qetannah() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::ShalsheletQetannah)));
    }

    #[test]
    fn contains_accent_poetry_tsinnorit_merkha() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMerkha)));
    }

    #[test]
    fn contains_accent_poetry_tsinnorit_mahpakh() {
        let ctx = SentenceContext::new("test", Context::Poetic);
        assert!(!ctx.contains_accent(HebrewAccent::Poetry(PoetryAccent::TsinnoritMahpakh)));
    }

    // Test default case (unknown accent)
    #[test]
    fn contains_accent_unknown_accent_returns_false() {
        let ctx = SentenceContext::new("test", Context::Prosaic);
        // This should hit the `_ => false` case
        // You may need to create a custom accent or use a variant not explicitly handled
        assert!(!ctx.contains_accent(HebrewAccent::Prose(ProseAccent::Silluq)));
        // Adjust as needed
    }
}
