mod prose_secondary_cantillation_mark {
    use hebrew_accents::{Accent, CantillationMark, CantillationMarkPlacement, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_secondary_cantillation_mark() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Atnach.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Segolta.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::Shalshelet.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(ProseAccent::ZaqephQaton.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::ZaqephGadol.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Revia.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Tiphcha.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Zarqa.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Pashta.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::ShenePastin.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '֙',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(hebrew_accents::CantillationMarkStressPosition::Postpositive)
            })
        );

        assert_eq!(ProseAccent::Yetiv.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Tevir.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Geresh.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Gershayim.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Pazer.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::PazerGadol.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::TelishaGedolah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(
            ProseAccent::Legarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_secondary_cantillation_mark() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Mahpakh.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Merkha.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::MerkhaKephulah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(ProseAccent::Darga.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Azla.secondary_cantillation_mark(), None);
        assert_eq!(
            ProseAccent::TelishaQetannah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(ProseAccent::Galgal.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Meayla.secondary_cantillation_mark(), None);
        assert_eq!(ProseAccent::Meteg.secondary_cantillation_mark(), None);
    }
}
mod poetry_secondary_cantillation_mark {
    use hebrew_accents::{
        Accent, CantillationMark, CantillationMarkPlacement, CantillationMarkStressPosition,
        PoetryAccent,
    };
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_secondary_cantillation_mark() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(PoetryAccent::Atnach.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::ReviaGadol.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::ReviaMugrash.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(PoetryAccent::Tsinnor.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::ReviaQaton.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Dechi.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Pazer.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            })
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_secondary_cantillation_mark() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Merkha.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Illuy.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Tarcha.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Galgal.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Mehuppakh.secondary_cantillation_mark(), None);
        assert_eq!(PoetryAccent::Azla.secondary_cantillation_mark(), None);
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.secondary_cantillation_mark(),
            None
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.secondary_cantillation_mark(),
            Some(CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            })
        );
        assert_eq!(PoetryAccent::Meteg.secondary_cantillation_mark(), None);
    }
}
mod pseudo_secondary_cantillation_mark {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_secondary_cantillation_mark() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.secondary_cantillation_mark(), None);
        assert_eq!(PseudoAccent::Maqqaph.secondary_cantillation_mark(), None);
        assert_eq!(PseudoAccent::Paseq.secondary_cantillation_mark(), None);
    }
}
