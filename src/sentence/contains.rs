//! Implementation of contains_accent() for 'SentenceContext'

// Local modules / crate‑internal
use crate::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};

use crate::sentence::char::{
    DARGA, DECHI, ETNAHTA, GERESH, GERSHAYIM, ILUY, MAHPAKH, MAQQEPH, MERKHA, MERKHA_KEFULA, MUNAH,
    PASHTA, PAZER, QADMA, QARNEY_PARA, REVIA, SEGOL, TELISHA_GEDOLAH, TELISHA_QETANA, TEVIR,
    TIPEHA, YERAH_BEN_YOMO, YETIV, ZAQEF_GADOL, ZAQEF_QATAN, ZARQA, ZINOR,
};
use crate::sentence::context::Context;
use crate::sentence::find::{
    find_poetry_mehuppakh, find_poetry_merkha, find_poetry_revia_gadol, find_poetry_revia_qaton,
};
use crate::sentence::regex::{
    FA_RE_OUTER_COMMON_METEG, FA_RE_OUTER_COMMON_SILLUQ, FA_RE_OUTER_POETRY_AZLA,
    FA_RE_OUTER_POETRY_SHALSHELET_QETANNAH, FA_RE_OUTER_PROSE_MUNACH, RE_OUTER_COMMON_SHALSHELET,
    RE_OUTER_POETRY_AZLA_LEGARMEH, RE_OUTER_POETRY_MEHUPPAKH_LEGARMEH,
    RE_OUTER_POETRY_OLEH_WEYORED, RE_OUTER_POETRY_REVIA_MUGRASH, RE_OUTER_POETRY_TSINNORIT_MAHPAKH,
    RE_OUTER_POETRY_TSINNORIT_MERKHA, RE_OUTER_PROSE_LEGARMEH, RE_OUTER_PROSE_MEAYLA,
};
use crate::sentence::sentence_context::SentenceContext;

impl SentenceContext {
    /// Returns true if the accent is present in the sentence
    /// taking into account the context
    ///  
    /// # Example
    /// ```
    /// use hebrew_accents::{SentenceContext,Context,HebrewAccent,ProseAccent,PoetryAccent};
    ///
    /// let sentence_context = SentenceContext::new("וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",Context::Prosaic,);
    /// let binding = sentence_context.unwrap();
    /// assert!(binding.contains_accent(ProseAccent::Silluq.into()));
    /// assert!(!binding.contains_accent(ProseAccent::Segolta.into()));
    /// assert!(!binding.contains_accent(PoetryAccent::ReviaGadol.into()));
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
                self.sentence.contains(TELISHA_GEDOLAH)
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
            HebrewAccent::Prose(ProseAccent::Meayla) if self.ctx == Context::Prosaic => {
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
                RE_OUTER_POETRY_OLEH_WEYORED.is_match(&self.sentence)
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
                self.sentence.contains(DECHI)
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
    use crate::sentence::context::Context;
    use crate::sentence::sentence_context::SentenceContext;
    use crate::{PoetryAccent, ProseAccent, PseudoAccent};
    #[test]
    fn test_contains_prose_poetry_silluq() {
        //let sc = SentenceContext::new("test", Context::Prosaic);
        // We can't easily create an unknown variant, but we can test that
        // all known variants are handled and the default returns None
        //let binding = sc.unwrap();
        //let result = binding.find_accent(HebrewAccent::Prose(ProseAccent::Silluq));

        // ProseAccent, with Soph Pasuq and Meteg, no Pey or Samech
        let sc = SentenceContext::new(" וַיֹּ֥אמֶר אֱלֹהִ֖ים יְהִ֣י א֑וֹר וַֽיְהִי־אֽוֹר׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Silluq.into()));
        // ProseAccent, with Soph Paseq, no Pey or Samech
        let sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Silluq.into()));
        // ProseAccent, no Soph Paseq, with Pey
        let sc = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ס ",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Silluq.into()));
        // PoetryAccent with Soph Paseq and Peh
        let sc = SentenceContext::new(
            "וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵֽׁם׃ ׃ פ",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Silluq.into()));
        // Meteg not in the last word of the sentence
        let sc = SentenceContext::new(
            "ס ס וַיַּעַשׂ֩ יְהוָ֨ה אֱלֹהִ֜ים לְאָדָ֧ם וּלְאִשְׁתּ֛וֹ כָּתְנ֥וֹת ע֖וֹר וַיַּלְבִּשֵׁם׃ ס ",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Silluq.into()));
        // Meteg followed by Maqqeph (\u{05BE}) (meaning no Meteg in the last word)
        let sc = SentenceContext::new("וַ וַיִּצֹ֥ק שֶׁ֖מֶן עַֽל־עַל־רֹאשׁהּ׃ ׃ פ", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Silluq.into()));
    }
    #[test]
    fn test_contains_prose_poetry_atnach() {
        // Atnach present
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Atnach.into()));
        // No Atnach present
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Atnach.into()));
    }
    #[test]
    fn test_contains_prose_segolta() {
        let sc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Segolta.into()));
        let sc = SentenceContext::new(
            " וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֨יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Segolta.into()));
    }
    #[test]
    fn test_contains_prose_shalshelet() {
        // Shalshelet, with Paseq - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Paseq + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Vertical Bar - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Shalshelet.into()));
        // Shalshelet, with Vertical Bar + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Shalshelet.into()));
        // Missing Paseq or Vertical Bar
        let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Shalshelet.into()));
    }
    #[test]
    fn test_contains_prose_zaqeph_qaton() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֔ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::ZaqephQatan.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::ZaqephQatan.into()));
    }
    #[test]
    fn test_contains_prose_zaqeph_gadol() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹ֕הִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::ZaqephGadol.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::ZaqephGadol.into()));
    }
    #[test]
    fn test_contains_prose_revia() {
        let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּ֗ל בּ֤ין", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Revia.into()));
        let sc = SentenceContext::new("אלהים֮ את־הרקיע֒ ויּבדּל בּ֤ין", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Revia.into()));
    }
    #[test]
    fn test_contains_prose_tiphcha() {
        let sc = SentenceContext::new(
            "ויּ֣ר֖א עשׂ֔ו כּ֥י רע֖ות בּנ֣ות כּ֖נ֑ען בּעינ֖י יצח֥ק א֖בֽיו׃",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Tiphcha.into()));
        let sc = SentenceContext::new("אתך ר֖בך֑ אתך ו֖המֽים׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Tiphcha.into()));
    }
    #[test]
    fn test_contains_prose_zarqa() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶ֘ץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Zarqa.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Zarqa.into()));
    }
    #[test]
    fn test_contains_prose_pashta() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱ֙לֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Pashta.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Pashta.into()));
    }
    #[test]
    fn test_contains_prose_yetiv() {
        let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח א֚תו֙", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Yetiv.into()));
        let sc = SentenceContext::new("אֽת־יעקב֒ ושׁלּ֤ח אתו֙", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Yetiv.into()));
    }
    #[test]
    fn test_contains_prose_tevir() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמצ֛יִם ד֛דד הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Tevir.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמציִם דדד הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Tevir.into()));
    }
    #[test]
    fn test_contains_prose_geresh() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁ֜מַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Geresh.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Geresh.into()));
    }
    #[test]
    fn test_contains_prose_gershayim() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֞ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Gershayim.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Gershayim.into()));
    }
    #[test]
    fn test_contains_prose_pazer() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְא֡ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Pazer.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Pazer.into()));
    }
    #[test]
    fn test_contains_prose_pazer_gadol() {
        let sc = SentenceContext::new("בְּרֵא֟שִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::PazerGadol.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::PazerGadol.into()));
    }
    #[test]
    fn test_contains_prose_telisha_gadolah() {
        let sc = SentenceContext::new("בְּרֵא֠ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::TelishaGedolah.into()));
        let sc = SentenceContext::new("בְּרֵאית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::TelishaGedolah.into()));
    }
    #[test]
    fn test_contains_prose_legarmeh() {
        // Legarmeh, with Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh with a space + Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh with two spaces + Paseq
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  ׀  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים|  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with space + Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Legarmeh, with two spaces + Vertical Bar
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִים  |  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Legarmeh.into()));
        // Paseq or Vertical Bar is missing
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֣ים  אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Legarmeh.into()));
    }
    // Conjunctives
    #[test]
    fn test_contains_prose_munnach() {
        // Single Munach
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (Paseq)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (space + Paseq)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים ׀  את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (Vertical Bar)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים|  את השּׁמים ואת הארץ׃׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Munach.into()));
        // Munach part of Legarmeh (space +Vertical Bar)
        let sc = SentenceContext::new("בּראשׁית בּרא א֣להים  |  את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Munach.into()));
    }
    #[test]
    fn test_contains_prose_mahpakh() {
        let sc = SentenceContext::new("בּאשׁ֤ית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Mahpakh.into()));
        let sc = SentenceContext::new("בּאשׁית בּא אלֹהִים אֵת הַשָּׁמַיִם וְאת האץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Mahpakh.into()));
    }
    #[test]
    fn test_contains_prose_merkha() {
        let sc = SentenceContext::new("מזמ֥ור לדו֑ד יהו֥ה ר֝ע֗י ל֣א אחסֽר׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Merkha.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵת הַשָּׁמַ֖יִם וְאֵת הָאָֽרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Merkha.into()));
    }
    #[test]
    fn test_contains_prose_merkha_kephulah() {
        let sc = SentenceContext::new("בְּרֵאשִׁ֦ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::MerkhaKephulah.into()));
        let sc = SentenceContext::new("בְּרֵאשִׁית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָרֶץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::MerkhaKephulah.into()));
    }
    #[test]
    fn test_contains_prose_darga() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים֧ ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Darga.into()));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Darga.into()));
    }
    #[test]
    fn test_contains_prose_azla() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֨ת השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Azla.into()));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Azla.into()));
    }
    #[test]
    fn test_contains_prose_telisha_qetannah() {
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֩ת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::TelishaQetannah.into()));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::TelishaQetannah.into()));
    }
    #[test]
    fn test_contains_prose_galgal() {
        let sc = SentenceContext::new("בּראשׁית בּר֪א אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Galgal.into()));
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Galgal.into()));
    }
    #[test]
    fn test_contains_prose_meayla() {
        // Tiphcha followed by Atnach
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹ֖הִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meayla.into()));
        // Tiphcha followed by Atnach, two words connected with a Maqqeph
        let sc = SentenceContext::new("ויּ֖צא־נ֑ח וּבנ֛יו ואשׁתּ֥ו וּנשֽׁי־בנ֖יו אתּֽו׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meayla.into()));
        // Tiphcha followed by silluq
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵת הַשָּׁמַיִם וְאֵת הָ֖אָֽרֶץ", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meayla.into()));
        // only Tiphcha
        let sc = SentenceContext::new("וְבְּרֵאשִׁית בָּרָא אֱלֹהִ֑ים אֵ֖ת הַשָּׁמַיִם וְאֵת הָאָֽרֶץ", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Meayla.into()));
    }
    #[test]
    fn test_contains_prose_meteg() {
        // Only Silluq, No Meteg
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(ProseAccent::Meteg.into()));
        // Meteg and Siluq, separated by a Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meteg.into()));
        // Meteg and Siluq in separate words
        let sc = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meteg.into()));
        // Only Meteg, no Silluq
        let sc = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Prosaic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(ProseAccent::Meteg.into()));
    }
    #[test]
    fn test_contains_prose_maqqeph() {
        // No Maqqeph
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PseudoAccent::Maqqeph.into()));
        // One Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PseudoAccent::Maqqeph.into()));
    }
    /* **********************************************************
     *                          POETRY
     * *********************************************************/
    #[test]
    fn test_contains_poetry_oleh_we_yored() {
        // OlehWeYored, one word
        let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, one word - context: Prosaic
        let sc = SentenceContext::new("בְּרֵעַֽל־פַּלְגֵ֫ימָ֥יִ", Context::Prosaic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, two words
        let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָ֥יִם וְעָלֵ֥הוּ ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::OlehWeYored.into()));
        // OlehWeYored, three words
        let sc = SentenceContext::new("ועַֽל־פַּלְגֵ֫י מָיִם וְעָ֥לֵ֥הוּ ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::OlehWeYored.into()));
    }
    #[test]
    fn test_contains_poetry_revia_gadol() {
        // No Revia at all
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Two Revia's
        let sc = SentenceContext::new("בּר֗אשׁית בּרא אלהים את השּׁ֗מים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by Oleh Weyored (1 word)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫ר֥א אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by Oleh Weyored (2 words)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלה֥ים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia followed by 'Oleh Weyored' (3 words)
        let sc = SentenceContext::new("בּר֗אשׁית בּ֫רא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaGadol.into()));
        // Revia not directly followed by Oleh Weyored (1 word)
        let sc = SentenceContext::new("בּר֗אשׁית בּרא אלה֫י֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaGadol.into()));
    }
    #[test]
    fn test_contains_poetry_revia_mugrash() {
        // Revia and Geresh
        let sc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaMugrash.into()));
        // Only Revia
        let sc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מא֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaMugrash.into()));
        // Only Geresh
        let sc = SentenceContext::new(
            " שׁ֗יר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝אין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaMugrash.into()));
    }
    #[test]
    fn test_contains_poetry_shalshelet_gadol() {
        // ShalsheletGadol, with Paseq - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓׀ אֵ֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Paseq + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ ׀ אֵ֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Vertical Bar - no space
        let sc = SentenceContext::new("בְּהִ֑ים֓| אֵ֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // ShalsheletGadol, with Vertical Bar + one space
        let sc = SentenceContext::new("בְּהִ֑ים֓ | אֵ֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ShalsheletGadol.into()));
        // Missing Paseq or Vertical Bar
        let sc = SentenceContext::new("בְּהִ֑ים֓ אֵ֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ShalsheletGadol.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnor() {
        let sc = SentenceContext::new("את־אבר֮הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Tsinnor.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Tsinnor.into()));
    }
    #[test]
    fn test_contains_poetry_revia_qaton() {
        // No revia at all
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia, not followed by OleWe Yored
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by Oleh Weyored (1 word)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמי֥ם ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by Oleh Weyored (2 words)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia directly followed by 'Oleh Weyored' (3 words)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֗ת ה֫שּׁמים ואת האר֥ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia NOT directly followed by Oleh Weyored (2 words)
        let sc = SentenceContext::new("בּראשׁית בּרא א֗להים א֓ת ה֫שּׁמים וא֥ת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaQaton.into()));
        // Revia is part of Revia Mugrash
        let sc = SentenceContext::new(
            " שׁיר לֽמּ֫על֥ות אשּׂ֣א ע֭יני אל־ההר֑ים מ֝א֗ין יב֥א עזרֽי׃",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ReviaQaton.into()));
    }
    #[test]
    fn test_contains_poetry_dechi() {
        let sc = SentenceContext::new("את־אבר֭הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Dechi.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Dechi.into()));
    }
    #[test]
    fn test_contains_poetry_pazer() {
        let sc = SentenceContext::new("את־אבר֡הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Pazer.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Pazer.into()));
    }
    #[test]
    fn test_contains_poetry_mehuppakh_legarmeh() {
        // MehuppakhLegarmeh, with Paseq
        let sc = SentenceContext::new(" את־אברהם֤ ׀ מזמ֗ור", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
        // MehuppakhLegarmeh, with Vertical Bar
        let sc = SentenceContext::new(" את־אברהם֤ | מזמ֗ור", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
        // Mehuppakh only
        let sc = SentenceContext::new(" את־אברהם֤ מזמ֗ור", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::MehuppakhLegarmeh.into()));
    }
    #[test]
    fn test_contains_poetry_azla_legarmeh() {
        // AzlaLegarmeh, with Paseq + no space
        let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Paseq + 1 space
        let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Vertical Bar + no space
        let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // AzlaLegarmeh, with Vertical Bar + 1 space
        let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
        // Azla only
        let sc = SentenceContext::new(" את־אברה֨ם  א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::AzlaLegarmeh.into()));
    }
    #[test]
    fn test_contains_poetry_munnach() {
        let sc = SentenceContext::new("את־אבר֣הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Munach.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Munach.into()));
    }
    #[test]
    fn test_contains_poetry_merkha() {
        // No Merkha
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Merkha.into()));
        // One Merkha
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (1w)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (2w)
        let sc = SentenceContext::new("בּראשׁית בּרא אל֘הים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Merkha.into()));
        // Tsinnorit + Merkha (3w)
        let sc = SentenceContext::new("בּראשׁית בּר֘א אלהים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (1w)
        let sc = SentenceContext::new("בּראשׁית בּרא א֫להי֥ם את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (2w)
        let sc = SentenceContext::new("בּראשׁית בּרא אלה֫ים א֥ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Merkha.into()));
        // Oleh + Merkha (3w)
        let sc = SentenceContext::new("בּראשׁית בּר֫א אלהים א֥ת השּׁ֥מים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Merkha.into()));
    }

    #[test]
    fn test_contains_poetry_illuy() {
        let sc = SentenceContext::new("את־אב֬רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Illuy.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Illuy.into()));
    }
    #[test]
    fn test_contains_poetry_tarcha() {
        let sc = SentenceContext::new("את־אבר֖הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Tarcha.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Tarcha.into()));
    }
    #[test]
    fn test_contains_poetry_galgal() {
        let sc = SentenceContext::new("את־אבר֪הם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Galgal.into()));
        let sc = SentenceContext::new("את־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Galgal.into()));
    }
    #[test]
    fn test_contains_poetry_mehuppakh() {
        // No Mehuppach
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (one word)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להי֤ם את השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (two words)
        let sc = SentenceContext::new("בּראשׁית בּרא א֘להים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Tsinnorit Mappach (three words)
        let sc = SentenceContext::new("בּראשׁית בּ֘רא אלהים א֤ת השּׁמים ואת הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Mehuppakh.into()));

        // One Mehuppach, part of Mehuppach Legarmeh (no space) TODO
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת׀ הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space) TODO
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת ׀ הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (no space - vertical line)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת| הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space - vertical line)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת | הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of 'Mehuppach Legarmeh' (too many spaces)
        let sc = SentenceContext::new("בּראשׁית בּרא אלהים את השּׁמים וא֤ת    ׀ הארץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        //One Mehuppach, part of Mehuppach Legarmeh (no space), followed with a Mehuppach
        let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת׀ האר֤ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Mehuppakh.into()));
        // One Mehuppach, part of Mehuppach Legarmeh (one space), followed with a Mehuppach
        let sc = SentenceContext::new("בּרא֤שׁית בּרא אלהים את השּׁמים וא֤ת ׀ האר֤ץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Mehuppakh.into()));
    }

    #[test]
    fn test_contains_poetry_azla() {
        // contains Azla
        let sc = SentenceContext::new(" את־אברה֨ם א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Azla.into()));
        // contains Azla and Azla Legarmeh
        let sc = SentenceContext::new(" אה֨ת־אברה֨ם ׀ א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with space + Paseq
        let sc = SentenceContext::new(" את־אברה֨ם ׀ א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with Paseq
        let sc = SentenceContext::new(" את־אברה֨ם׀ א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with space + Vertical Bar
        let sc = SentenceContext::new(" את־אברה֨ם | א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Azla.into()));
        // Azla Legarmeh, with Vertical Bar
        let sc = SentenceContext::new(" את־אברה֨ם| א־אם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Azla.into()));
    }
    #[test]
    fn test_contains_poetry_shalshelet_qetannah() {
        // Shalshelet
        let sc = SentenceContext::new("יצחק אל־יעק֓ב ויברך", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
        // Shalshelet Gadol, with Paseq
        let sc = SentenceContext::new("יצחק אל־יעק֓ב ׀ ויברך", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
        // Shalshelet Gadol, with Vertical Bar
        let sc = SentenceContext::new("יצחק אל־יעק֓ב | ויברך", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::ShalsheletQetannah.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_merkha() {
        // accent in a single word
        let sc = SentenceContext::new("אא֘תאב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in a single word, without Tsinnorit
        let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in a single word, without Merkha
        let sc = SentenceContext::new("אא֘ת־אברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph
        let sc = SentenceContext::new("את־א֘ב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sc = SentenceContext::new("את־אב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words seperated by Maqqeph, without Merkha
        let sc = SentenceContext::new("את־א֘ברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words
        let sc = SentenceContext::new("את־א֘בם ב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words, without Tsinnorit
        let sc = SentenceContext::new("את־א֘בם ברהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in two words, without Merkha
        let sc = SentenceContext::new("את־אבם ב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
        // accent in three words
        let sc = SentenceContext::new("את־א֘בם הם ב֥רהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMerkha.into()));
    }
    #[test]
    fn test_contains_poetry_tsinnorit_mahpakh() {
        // accent in a single word
        let sc = SentenceContext::new("את־א֘ב֤רהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in a single word
        let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in a single word, without Tsinnorit
        let sc = SentenceContext::new("את־א֘ברהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sc = SentenceContext::new("אא֘ת־אב֤רהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Tsinnorit
        let sc = SentenceContext::new("את־אב֤רהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words seperated by Maqqeph, without Mahpakh
        let sc = SentenceContext::new("אא֘ת־אברהם אהם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words
        let sc = SentenceContext::new("את־א֘ברהם אהאב֤ם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words, without Tsinnorit
        let sc = SentenceContext::new("את־אברהם אהאב֤ם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in two words, without Mahpakh
        let sc = SentenceContext::new("את־א֘ברהם אהאבם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
        // accent in three words
        let sc = SentenceContext::new("את־א֘ב רהם אהאב֤ם", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::TsinnoritMahpakh.into()));
    }

    #[test]
    fn test_contains_poetry_meteg() {
        // Only Silluq, No Meteg
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PoetryAccent::Meteg.into()));
        // Meteg and Siluq, separated by a Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Meteg.into()));
        // Meteg and Siluq in separate words
        let sc = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁנֽי׃ פ",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Meteg.into()));
        // Only Meteg, no Silluq
        let sc = SentenceContext::new(
            "ויּקר֧א אלה֛ים לֽרק֖יע שׁמ֑ים וֽיהי־ע֥רב וֽיהי־ב֖קר י֥ום שׁני׃ פ",
            Context::Poetic,
        );
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PoetryAccent::Meteg.into()));
    }
    #[test]
    fn test_contains_poetry_maqqeph() {
        // No Maqqeph
        let sc = SentenceContext::new("בּראשׁ֖ית בּר֣א אלה֑ים א֥ת השּׁמ֖ים וא֥ת האֽרץ׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(!binding.contains_accent(PseudoAccent::Maqqeph.into()));
        // One Maqqeph
        let sc = SentenceContext::new("ויּ֥אמר אלה֖ים יה֣י א֑ור וֽיהי־אֽור׃", Context::Poetic);
        let binding = sc.unwrap();
        assert!(binding.contains_accent(PseudoAccent::Maqqeph.into()));
    }
}
