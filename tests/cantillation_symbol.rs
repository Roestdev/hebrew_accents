mod prose_cantillation_symbol {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Atnach.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Segolta.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Shalshelet.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::ZaqephQatan.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::ZaqephGadol.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Revia.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Tiphcha.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Zarqa.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Pashta.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Yetiv.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Tevir.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Geresh.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Gershayim.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Pazer.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::PazerGadol.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::TelishaGedolah.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Legarmeh.cantillation_symbol(), "// TODO");
    }

    #[test]
    fn test_prose_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Mahpakh.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Merkha.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::MerkhaKephulah.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Darga.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Azla.cantillation_symbol(), "// TODO");
        assert_eq!(
            ProseAccent::TelishaQetannah.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(ProseAccent::Galgal.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Meayla.cantillation_symbol(), "// TODO");
        assert_eq!(ProseAccent::Meteg.cantillation_symbol(), "// TODO");
    }
}
mod poetry_cantillation_symbol {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Silluq, Atnach,pazer zie prose // TODO
    #[test]
    fn test_poetry_disjunctive_cantillation_symbol() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::OlehWeYored.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Atnach.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaGadol.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaMugrash.cantillation_symbol(), "// TODO");
        assert_eq!(
            PoetryAccent::ShalsheletGadol.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(PoetryAccent::Tsinnor.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::ReviaQaton.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Dechi.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Pazer.cantillation_symbol(), "// TODO");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.cantillation_symbol(), "// TODO");
    }

    /// Test all poetry conjunctive accents (should return None)
    /// // TODO common: munach,merkha,galgal,azla,meteg
    #[test]
    fn test_poetry_conjunctive_cantillation_symbol() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Merkha.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Illuy.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Tarcha.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Galgal.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Mehuppakh.cantillation_symbol(), "// TODO");
        assert_eq!(PoetryAccent::Azla.cantillation_symbol(), "// TODO");
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.cantillation_symbol(),
            "// TODO"
        );
        assert_eq!(PoetryAccent::Meteg.cantillation_symbol(), "// TODO");
    }
}
mod pseudo_cantillation_symbol {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_cantillation_symbol() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.cantillation_symbol(), "// TODO");
        assert_eq!(PseudoAccent::Maqqaph.cantillation_symbol(), "// TODO");
        assert_eq!(PseudoAccent::Paseq.cantillation_symbol(), "// TODO");
    }
}
