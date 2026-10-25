mod prose_primary_cantillation_mark {
    use hebrew_accents::{
        Accent, CantillationMark, CantillationMarkPlacement, CantillationMarkStressPosition,
        ProseAccent,
    };

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_primary_cantillation_mark() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(
            ProseAccent::Silluq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Atnach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{591}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Segolta.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{592}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Shalshelet.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::ZaqephQatan.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{594}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{595}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Revia.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Tiphcha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        // TODO ZArqa
        assert_eq!(
            ProseAccent::Zarqa.primary_cantillation_mark(),
            // encoding HEBREW ACCENT ZINOR
            CantillationMark {
                symbol: '\u{05AE}', 
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Pashta.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{599}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::ShenePastin.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Yetiv.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59a}',
                placement: CantillationMarkPlacement::BelowRight,
                stress_position: Some(CantillationMarkStressPosition::Prepositive)
            }
        );
        assert_eq!(
            ProseAccent::Tevir.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59b}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Geresh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59c}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Gershayim.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59e}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Pazer.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a1}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::PazerGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59f}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a0}',
                placement: CantillationMarkPlacement::AboveRight,
                stress_position: Some(CantillationMarkStressPosition::Prepositive)
            }
        );
        assert_eq!(
            ProseAccent::Legarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_primary_cantillation_mark() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(
            ProseAccent::Munach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Mahpakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Merkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a6}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Darga.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a7}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Azla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a9}',
                placement: CantillationMarkPlacement::AboveLeft,
                stress_position: Some(CantillationMarkStressPosition::Postpositive)
            }
        );
        assert_eq!(
            ProseAccent::Galgal.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5aa}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Meayla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            ProseAccent::Meteg.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }
}

mod poetry_primary_cantillation_mark {
    use hebrew_accents::{
        Accent, CantillationMark, CantillationMarkPlacement, CantillationMarkStressPosition,
        PoetryAccent,
    };
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_primary_cantillation_mark() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(
            PoetryAccent::Silluq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::OlehWeYored.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ab}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Atnach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{591}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{59c}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Tsinnor.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ae}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ReviaQaton.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{597}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Dechi.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ad}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Pazer.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a1}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_primary_cantillation_mark() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(
            PoetryAccent::Munach.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a3}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Merkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a5}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Illuy.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5ac}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Tarcha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{596}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Galgal.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5aa}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a4}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Azla.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5a8}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{593}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{598}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{598}',
                placement: CantillationMarkPlacement::AboveCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
        assert_eq!(
            PoetryAccent::Meteg.primary_cantillation_mark(),
            CantillationMark {
                symbol: '\u{5bd}',
                placement: CantillationMarkPlacement::BelowCenter,
                stress_position: Some(CantillationMarkStressPosition::Impositive)
            }
        );
    }
}

mod pseudo_primary_cantillation_mark {
    use hebrew_accents::{Accent, CantillationMark, CantillationMarkPlacement, PseudoAccent};
    #[test]
    fn test_pseudo_accent_primary_cantillation_mark() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(
            PseudoAccent::SophPasuq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '׃',
                placement: CantillationMarkPlacement::SofPasuq,
                stress_position: None
            }
        );
        assert_eq!(
            PseudoAccent::Maqqaph.primary_cantillation_mark(),
            CantillationMark {
                symbol: '־',
                placement: CantillationMarkPlacement::Maqqaph,
                stress_position: None
            }
        );
        assert_eq!(
            PseudoAccent::Paseq.primary_cantillation_mark(),
            CantillationMark {
                symbol: '׀',
                placement: CantillationMarkPlacement::Paseq,
                stress_position: None
            }
        );
    }
}
