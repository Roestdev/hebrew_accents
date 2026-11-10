
mod prose_cantillation_symbol {
    use hebrew_accents::{ProseAccent, cantillation_example};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(cantillation_example(ProseAccent::Silluq.into()), "אםֽת");
        assert_eq!(cantillation_example(ProseAccent::Atnach.into()), "אם֑ת");
        assert_eq!(cantillation_example(ProseAccent::Segolta.into()), "אםת֒");
        assert_eq!(cantillation_example(ProseAccent::Shalshelet.into()), "אם֓ת ׀");
        assert_eq!(cantillation_example(ProseAccent::ZaqephQaton.into()), "אם֔ת");
        assert_eq!(cantillation_example(ProseAccent::ZaqephGadol.into()), "אם֕ת");
        assert_eq!(cantillation_example(ProseAccent::Revia.into()), "אם֗ת");
        assert_eq!(cantillation_example(ProseAccent::Tiphcha.into()), "אם֖ת");
        assert_eq!(cantillation_example(ProseAccent::Zarqa.into()), "אםת֮");
        assert_eq!(cantillation_example(ProseAccent::Pashta.into()), "אםת֙");
        assert_eq!(cantillation_example(ProseAccent::ShenePastin.into()), "א֨םת֙");
        assert_eq!(cantillation_example(ProseAccent::Yetiv.into()), "א֚םת");
        assert_eq!(cantillation_example(ProseAccent::Tevir.into()), "אם֛ת");
        assert_eq!(cantillation_example(ProseAccent::Geresh.into()), "אם֜ת");
        assert_eq!(cantillation_example(ProseAccent::Gershayim.into()), "אם֞ת");
        assert_eq!(cantillation_example(ProseAccent::Pazer.into()), "אם֡ת");
        assert_eq!(cantillation_example(ProseAccent::PazerGadol.into()), "אם֟ת");
        assert_eq!(cantillation_example(ProseAccent::TelishaGedolah.into()), "א֠םת");
        assert_eq!(cantillation_example(ProseAccent::Legarmeh.into()), "אם֣ת ׀");
    }

    #[test]
    fn test_prose_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(cantillation_example(ProseAccent::Munach.into()), "אם֣ת");
        assert_eq!(cantillation_example(ProseAccent::Mahpakh.into()), "אם֤ת");
        assert_eq!(cantillation_example(ProseAccent::Merkha.into()), "אם֥ת");
        assert_eq!(cantillation_example(ProseAccent::MerkhaKephulah.into()), "אם֦ת");
        assert_eq!(cantillation_example(ProseAccent::Darga.into()), "אם֧ת");
        assert_eq!(cantillation_example(ProseAccent::Azla.into()), "אם֨ת");
        assert_eq!(cantillation_example(ProseAccent::TelishaQetannah.into()), "אםת֩");
        assert_eq!(cantillation_example(ProseAccent::Galgal.into()), "אם֪ת");
        assert_eq!(cantillation_example(ProseAccent::Meayla.into()), "אם֖ת");
        assert_eq!(cantillation_example(ProseAccent::Meteg.into()), "אםֽת");
    }
}
mod poetry_cantillation_symbol {
    use hebrew_accents::{PoetryAccent, cantillation_example};
    #[test]
    fn test_poetry_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(cantillation_example(PoetryAccent::Silluq.into()), "אםֽת");
        assert_eq!(cantillation_example(PoetryAccent::OlehWeYored.into()), "אם֫ת אם֥ת");
        assert_eq!(cantillation_example(PoetryAccent::Atnach.into()), "אם֑ת");
        assert_eq!(cantillation_example(PoetryAccent::ReviaGadol.into()), "אם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::ReviaMugrash.into()), "א֝ם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::ShalsheletGadol.into()), "אם֓ת ׀");
        assert_eq!(cantillation_example(PoetryAccent::Tsinnor.into()), "אםת֮");
        assert_eq!(cantillation_example(PoetryAccent::ReviaQaton.into()), "אם֗ת");
        assert_eq!(cantillation_example(PoetryAccent::Dechi.into()), "אם֭ת");
        assert_eq!(cantillation_example(PoetryAccent::Pazer.into()), "אם֡ת");
        assert_eq!(cantillation_example(
            PoetryAccent::MehuppakhLegarmeh.into()),
            "אם֤ת ׀"
        );
        assert_eq!(cantillation_example(PoetryAccent::AzlaLegarmeh.into()), "אם֨ת ׀");
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(cantillation_example(PoetryAccent::Munach.into()), "אם֣ת");
        assert_eq!(cantillation_example(PoetryAccent::Merkha.into()), "אם֥ת");
        assert_eq!(cantillation_example(PoetryAccent::Illuy.into()), "אם֬ת");
        assert_eq!(cantillation_example(PoetryAccent::Tarcha.into()), "אם֖ת");
        assert_eq!(cantillation_example(PoetryAccent::Galgal.into()), "אם֪ת");
        assert_eq!(cantillation_example(PoetryAccent::Mehuppakh.into()), "אם֤ת");
        assert_eq!(cantillation_example(PoetryAccent::Azla.into()), "אם֨ת");
        assert_eq!(cantillation_example(
            PoetryAccent::ShalsheletQetannah.into()),
            "אם֓ת"
        );
        assert_eq!(cantillation_example(
            PoetryAccent::TsinnoritMerkha.into()),
            "אם֘ת אם֥ת"
        );
        assert_eq!(cantillation_example(
            PoetryAccent::TsinnoritMahpakh.into()),
            "אם֘ת אם֤ת"
        );
        assert_eq!(cantillation_example(PoetryAccent::Meteg.into()), "אםֽת");
    }
}
mod pseudo_cantillation_symbol {
    use hebrew_accents::{PseudoAccent, cantillation_example};
    #[test]
    fn test_pseudo_accent_cantillation_symbol() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(cantillation_example(PseudoAccent::SophPasuq.into()), "אםת ׃");
        assert_eq!(cantillation_example(PseudoAccent::Maqqaph.into()), "אםת־אםת");
        assert_eq!(cantillation_example(PseudoAccent::Paseq.into()), "אםת ׀");
    }
}
