mod prose_category {
    use hebrew_accents::{Accent, AccentCategory, ProseAccent};

    #[test]
    fn test_prose_disjunctive_category() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Atnach.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Segolta.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Shalshelet.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Revia.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Tiphcha.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Zarqa.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Pashta.category(), // TODO add ShenePastin
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::ShenePastin.category(), // TODO add ShenePastin
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Yetiv.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Tevir.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Geresh.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Gershayim.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Pazer.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::PazerGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            ProseAccent::Legarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
    }

    #[test]
    fn test_prose_conjunctive_category() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Mahpakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Merkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Darga.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Azla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Galgal.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Meayla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            ProseAccent::Meteg.category(),
            Some(AccentCategory::Conjunctive)
        );
    }
}
mod poetry_category {
    use hebrew_accents::{Accent, AccentCategory, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_category() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Atnach.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Tsinnor.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Dechi.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::Pazer.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.category(),
            Some(AccentCategory::Disjunctive)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_category() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Merkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Illuy.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Tarcha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Galgal.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Azla.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.category(),
            Some(AccentCategory::Conjunctive)
        );
        assert_eq!(
            PoetryAccent::Meteg.category(),
            Some(AccentCategory::Conjunctive)
        );
    }
}
mod pseudo_category {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_category() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.category(), None);
        assert_eq!(PseudoAccent::Maqqaph.category(), None);
        assert_eq!(PseudoAccent::Paseq.category(), None);
    }
}
