mod prose_cantillation_symbol {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.cantillation_symbol(), "אםֽת");
        assert_eq!(ProseAccent::Atnach.cantillation_symbol(), "אם֑ת");
        assert_eq!(ProseAccent::Segolta.cantillation_symbol(), "אםת֒");
        assert_eq!(ProseAccent::Shalshelet.cantillation_symbol(), "אם֓ת ׀");
        assert_eq!(ProseAccent::ZaqephQatan.cantillation_symbol(), "אם֔ת");
        assert_eq!(ProseAccent::ZaqephGadol.cantillation_symbol(), "אם֕ת");
        assert_eq!(ProseAccent::Revia.cantillation_symbol(), "אם֗ת");
        assert_eq!(ProseAccent::Tiphcha.cantillation_symbol(), "אם֖ת");
        assert_eq!(ProseAccent::Zarqa.cantillation_symbol(), "אםת֮");
        assert_eq!(ProseAccent::Pashta.cantillation_symbol(), "אםת֙");
        assert_eq!(ProseAccent::Yetiv.cantillation_symbol(), "א֚םת");
        assert_eq!(ProseAccent::Tevir.cantillation_symbol(), "אם֛ת");
        assert_eq!(ProseAccent::Geresh.cantillation_symbol(), "אם֜ת");
        assert_eq!(ProseAccent::Gershayim.cantillation_symbol(), "אם֞ת");
        assert_eq!(ProseAccent::Pazer.cantillation_symbol(), "אם֡ת");
        assert_eq!(ProseAccent::PazerGadol.cantillation_symbol(), "אם֟ת");
        assert_eq!(ProseAccent::TelishaGedolah.cantillation_symbol(), "א֠םת");
        assert_eq!(ProseAccent::Legarmeh.cantillation_symbol(), "אם֣ת ׀");
    }

    #[test]
    fn test_prose_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.cantillation_symbol(), "אם֣ת");
        assert_eq!(ProseAccent::Mahpakh.cantillation_symbol(), "אם֤ת");
        assert_eq!(ProseAccent::Merkha.cantillation_symbol(), "אם֥ת");
        assert_eq!(ProseAccent::MerkhaKephulah.cantillation_symbol(), "אם֦ת");
        assert_eq!(ProseAccent::Darga.cantillation_symbol(), "אם֧ת");
        assert_eq!(ProseAccent::Azla.cantillation_symbol(), "אם֨ת");
        assert_eq!(ProseAccent::TelishaQetannah.cantillation_symbol(), "אםת֩");
        assert_eq!(ProseAccent::Galgal.cantillation_symbol(), "אם֪ת");
        assert_eq!(ProseAccent::Meayla.cantillation_symbol(), "אם֖ת");
        assert_eq!(ProseAccent::Meteg.cantillation_symbol(), "אםֽת");
    }
}
mod poetry_cantillation_symbol {
    use hebrew_accents::{Accent, PoetryAccent};
    #[test]
    fn test_poetry_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.cantillation_symbol(), "אםֽת");
        assert_eq!(PoetryAccent::OlehWeYored.cantillation_symbol(), "אם֫ת אם֥ת");
        assert_eq!(PoetryAccent::Atnach.cantillation_symbol(), "אם֑ת");
        assert_eq!(PoetryAccent::ReviaGadol.cantillation_symbol(), "אם֗ת");
        assert_eq!(PoetryAccent::ReviaMugrash.cantillation_symbol(), "א֜ם֗ת");
        assert_eq!(PoetryAccent::ShalsheletGadol.cantillation_symbol(), "אם֓ת ׀");
        assert_eq!(PoetryAccent::Tsinnor.cantillation_symbol(), "אםת֮");
        assert_eq!(PoetryAccent::ReviaQaton.cantillation_symbol(), "אם֗ת");
        assert_eq!(PoetryAccent::Dechi.cantillation_symbol(), "אם֭ת");
        assert_eq!(PoetryAccent::Pazer.cantillation_symbol(), "אם֡ת");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.cantillation_symbol(),
            "אם֤ת ׀"
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.cantillation_symbol(), "אם֨ת ׀");
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.cantillation_symbol(), "אם֣ת");
        assert_eq!(PoetryAccent::Merkha.cantillation_symbol(), "אם֥ת");
        assert_eq!(PoetryAccent::Illuy.cantillation_symbol(), "אם֬ת");
        assert_eq!(PoetryAccent::Tarcha.cantillation_symbol(), "אם֖ת");
        assert_eq!(PoetryAccent::Galgal.cantillation_symbol(), "אם֪ת");
        assert_eq!(PoetryAccent::Mehuppakh.cantillation_symbol(), "אם֤ת");
        assert_eq!(PoetryAccent::Azla.cantillation_symbol(), "אם֨ת");
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.cantillation_symbol(),
            "אם֓ת"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.cantillation_symbol(),
            "אם֘ת אם֥ת"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.cantillation_symbol(),
            "אם֘ת אם֤ת"
        );
        assert_eq!(PoetryAccent::Meteg.cantillation_symbol(), "אםֽת");
    }
}
mod pseudo_cantillation_symbol {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_cantillation_symbol() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.cantillation_symbol(), "אםת ׃");
        assert_eq!(PseudoAccent::Maqqaph.cantillation_symbol(), "אםת־אםת");
        assert_eq!(PseudoAccent::Paseq.cantillation_symbol(), "אםת ׀");
    }
}
