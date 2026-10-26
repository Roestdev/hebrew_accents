mod prose_compound_type {
    use hebrew_accents::{Accent, CompoundType, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_compound_type() {
        assert_eq!(ProseAccent::Silluq.compound_type(), None);
        assert_eq!(ProseAccent::Atnach.compound_type(), None);
        assert_eq!(ProseAccent::Segolta.compound_type(), None);
        assert_eq!(
            ProseAccent::Shalshelet.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(ProseAccent::ZaqephQaton.compound_type(), None);
        assert_eq!(ProseAccent::ZaqephGadol.compound_type(), None);
        assert_eq!(ProseAccent::Revia.compound_type(), None);
        assert_eq!(ProseAccent::Tiphcha.compound_type(), None);
        assert_eq!(ProseAccent::Zarqa.compound_type(), None);
        assert_eq!(ProseAccent::Pashta.compound_type(), None);
        assert_eq!(
            ProseAccent::ShenePastin.compound_type(),
            Some(CompoundType::Standard)
        );
        assert_eq!(ProseAccent::Yetiv.compound_type(), None);
        assert_eq!(ProseAccent::Tevir.compound_type(), None);
        assert_eq!(ProseAccent::Geresh.compound_type(), None);
        assert_eq!(ProseAccent::Gershayim.compound_type(), None);
        assert_eq!(ProseAccent::Pazer.compound_type(), None);
        assert_eq!(ProseAccent::PazerGadol.compound_type(), None);
        assert_eq!(ProseAccent::TelishaGedolah.compound_type(), None);
        assert_eq!(
            ProseAccent::Legarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
    }

    #[test]
    fn test_prose_conjunctive_compound_type() {
        // Conjunctives: indices CompoundType::Primary-CompoundType::Primary, all should be None
        assert_eq!(ProseAccent::Munach.compound_type(), None);
        assert_eq!(ProseAccent::Mahpakh.compound_type(), None);
        assert_eq!(ProseAccent::Merkha.compound_type(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.compound_type(), None);
        assert_eq!(ProseAccent::Darga.compound_type(), None);
        assert_eq!(ProseAccent::Azla.compound_type(), None);
        assert_eq!(ProseAccent::TelishaQetannah.compound_type(), None);
        assert_eq!(ProseAccent::Galgal.compound_type(), None);
        assert_eq!(ProseAccent::Meayla.compound_type(), None);
        assert_eq!(ProseAccent::Meteg.compound_type(), None);
    }
}
mod poetry_compound_type {
    use hebrew_accents::{Accent, CompoundType, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_compound_type() {
        // Primary disjunctives: indices CompoundType::Primary-CompoundType::Primary, strengths CompoundType::Primary-CompoundType::Primary
        assert_eq!(PoetryAccent::Silluq.compound_type(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(PoetryAccent::Atnach.compound_type(), None);
        assert_eq!(PoetryAccent::ReviaGadol.compound_type(), None);
        assert_eq!(
            PoetryAccent::ReviaMugrash.compound_type(),
            Some(CompoundType::Standard)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(PoetryAccent::Tsinnor.compound_type(), None);
        assert_eq!(PoetryAccent::ReviaQaton.compound_type(), None);
        assert_eq!(PoetryAccent::Dechi.compound_type(), None);
        assert_eq!(PoetryAccent::Pazer.compound_type(), None);
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.compound_type(),
            Some(CompoundType::ContainsPaseq)
        );
    }

    #[test]
    fn test_poetry_conjunctive_compound_type() {
        assert_eq!(PoetryAccent::Munach.compound_type(), None);
        assert_eq!(PoetryAccent::Merkha.compound_type(), None);
        assert_eq!(PoetryAccent::Illuy.compound_type(), None);
        assert_eq!(PoetryAccent::Tarcha.compound_type(), None);
        assert_eq!(PoetryAccent::Galgal.compound_type(), None);
        assert_eq!(PoetryAccent::Mehuppakh.compound_type(), None);
        assert_eq!(PoetryAccent::Azla.compound_type(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.compound_type(), None);
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.compound_type(),
            Some(CompoundType::CanSpanTwoWords)
        );
        assert_eq!(PoetryAccent::Meteg.compound_type(), None);
    }
}
mod pseudo_compound_type {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_compound_type() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.compound_type(), None);
        assert_eq!(PseudoAccent::Maqqaph.compound_type(), None);
        assert_eq!(PseudoAccent::Paseq.compound_type(), None);
    }
}
