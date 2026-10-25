mod prose_accents_relative_strength {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_strength() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(ProseAccent::Atnach.relative_strength(), Some(2));
        assert_eq!(ProseAccent::Segolta.relative_strength(), Some(3));
        assert_eq!(ProseAccent::Shalshelet.relative_strength(), Some(4));
        assert_eq!(ProseAccent::ZaqephQatan.relative_strength(), Some(5));
        assert_eq!(ProseAccent::ZaqephGadol.relative_strength(), Some(6));
        assert_eq!(ProseAccent::Revia.relative_strength(), Some(7));
        assert_eq!(ProseAccent::Tiphcha.relative_strength(), Some(8));
        assert_eq!(ProseAccent::Zarqa.relative_strength(), Some(9));
        assert_eq!(ProseAccent::Pashta.relative_strength(), Some(10));
        assert_eq!(ProseAccent::ShenePastin.relative_strength(), Some(10));
        assert_eq!(ProseAccent::Yetiv.relative_strength(), Some(11));
        assert_eq!(ProseAccent::Tevir.relative_strength(), Some(12));
        assert_eq!(ProseAccent::Geresh.relative_strength(), Some(13));
        assert_eq!(ProseAccent::Gershayim.relative_strength(), Some(14));
        assert_eq!(ProseAccent::Pazer.relative_strength(), Some(15));
        assert_eq!(ProseAccent::PazerGadol.relative_strength(), Some(16));
        assert_eq!(ProseAccent::TelishaGedolah.relative_strength(), Some(17));
        assert_eq!(ProseAccent::Legarmeh.relative_strength(), Some(18));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_strength() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.relative_strength(), None);
        assert_eq!(ProseAccent::Mahpakh.relative_strength(), None);
        assert_eq!(ProseAccent::Merkha.relative_strength(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.relative_strength(), None);
        assert_eq!(ProseAccent::Darga.relative_strength(), None);
        assert_eq!(ProseAccent::Azla.relative_strength(), None);
        assert_eq!(ProseAccent::TelishaQetannah.relative_strength(), None);
        assert_eq!(ProseAccent::Galgal.relative_strength(), None);
        assert_eq!(ProseAccent::Meayla.relative_strength(), None);
        assert_eq!(ProseAccent::Meteg.relative_strength(), None);
    }
}
mod poetry_accents_relative_strength {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_strength() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(PoetryAccent::OlehWeYored.relative_strength(), Some(2));
        assert_eq!(PoetryAccent::Atnach.relative_strength(), Some(3));
        assert_eq!(PoetryAccent::ReviaGadol.relative_strength(), Some(4));
        assert_eq!(PoetryAccent::ReviaMugrash.relative_strength(), Some(5));
        assert_eq!(PoetryAccent::ShalsheletGadol.relative_strength(), Some(6));
        assert_eq!(PoetryAccent::Tsinnor.relative_strength(), Some(7));
        assert_eq!(PoetryAccent::ReviaQaton.relative_strength(), Some(8));
        assert_eq!(PoetryAccent::Dechi.relative_strength(), Some(9));
        assert_eq!(PoetryAccent::Pazer.relative_strength(), Some(10));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.relative_strength(),
            Some(11)
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), Some(12));
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_strength() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.relative_strength(), None);
        assert_eq!(PoetryAccent::Merkha.relative_strength(), None);
        assert_eq!(PoetryAccent::Illuy.relative_strength(), None);
        assert_eq!(PoetryAccent::Tarcha.relative_strength(), None);
        assert_eq!(PoetryAccent::Galgal.relative_strength(), None);
        assert_eq!(PoetryAccent::Mehuppakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Azla.relative_strength(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Meteg.relative_strength(), None);
    }
}
mod pseudo_accents_relative_strength {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_strength() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.relative_strength(), None);
        assert_eq!(PseudoAccent::Maqqaph.relative_strength(), None);
        assert_eq!(PseudoAccent::Paseq.relative_strength(), None);
    }
}
