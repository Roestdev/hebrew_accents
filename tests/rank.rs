mod prose_accents_rank {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_rank() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.rank(), Some(1));
        assert_eq!(ProseAccent::Atnach.rank(), Some(2));
        assert_eq!(ProseAccent::Segolta.rank(), Some(3));
        assert_eq!(ProseAccent::Shalshelet.rank(), Some(4));
        assert_eq!(ProseAccent::ZaqephQaton.rank(), Some(5));
        assert_eq!(ProseAccent::ZaqephGadol.rank(), Some(6));
        assert_eq!(ProseAccent::Revia.rank(), Some(7));
        assert_eq!(ProseAccent::Tiphcha.rank(), Some(8));
        assert_eq!(ProseAccent::Zarqa.rank(), Some(9));
        assert_eq!(ProseAccent::Pashta.rank(), Some(10));
        assert_eq!(ProseAccent::ShenePastin.rank(), Some(10));
        assert_eq!(ProseAccent::Yetiv.rank(), Some(11));
        assert_eq!(ProseAccent::Tevir.rank(), Some(12));
        assert_eq!(ProseAccent::Geresh.rank(), Some(13));
        assert_eq!(ProseAccent::Gershayim.rank(), Some(14));
        assert_eq!(ProseAccent::Pazer.rank(), Some(15));
        assert_eq!(ProseAccent::PazerGadol.rank(), Some(16));
        assert_eq!(ProseAccent::TelishaGedolah.rank(), Some(17));
        assert_eq!(ProseAccent::Legarmeh.rank(), Some(18));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_rank() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.rank(), None);
        assert_eq!(ProseAccent::Mahpakh.rank(), None);
        assert_eq!(ProseAccent::Merkha.rank(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.rank(), None);
        assert_eq!(ProseAccent::Darga.rank(), None);
        assert_eq!(ProseAccent::Azla.rank(), None);
        assert_eq!(ProseAccent::TelishaQetannah.rank(), None);
        assert_eq!(ProseAccent::Galgal.rank(), None);
        assert_eq!(ProseAccent::Meayla.rank(), None);
        assert_eq!(ProseAccent::Meteg.rank(), None);
    }
}
mod poetry_accents_rank {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_rank() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.rank(), Some(1));
        assert_eq!(PoetryAccent::OlehWeYored.rank(), Some(2));
        assert_eq!(PoetryAccent::Atnach.rank(), Some(3));
        assert_eq!(PoetryAccent::ReviaGadol.rank(), Some(4));
        assert_eq!(PoetryAccent::ReviaMugrash.rank(), Some(5));
        assert_eq!(PoetryAccent::ShalsheletGadol.rank(), Some(6));
        assert_eq!(PoetryAccent::Tsinnor.rank(), Some(7));
        assert_eq!(PoetryAccent::ReviaQaton.rank(), Some(8));
        assert_eq!(PoetryAccent::Dechi.rank(), Some(9));
        assert_eq!(PoetryAccent::Pazer.rank(), Some(10));
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.rank(), Some(11));
        assert_eq!(PoetryAccent::AzlaLegarmeh.rank(), Some(12));
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_rank() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.rank(), None);
        assert_eq!(PoetryAccent::Merkha.rank(), None);
        assert_eq!(PoetryAccent::Illuy.rank(), None);
        assert_eq!(PoetryAccent::Tarcha.rank(), None);
        assert_eq!(PoetryAccent::Galgal.rank(), None);
        assert_eq!(PoetryAccent::Mehuppakh.rank(), None);
        assert_eq!(PoetryAccent::Azla.rank(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.rank(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.rank(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.rank(), None);
        assert_eq!(PoetryAccent::Meteg.rank(), None);
    }
}
mod pseudo_accents_rank {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_rank() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.rank(), None);
        assert_eq!(PseudoAccent::Maqqaph.rank(), None);
        assert_eq!(PseudoAccent::Paseq.rank(), None);
    }
}
