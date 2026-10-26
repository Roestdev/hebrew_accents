mod prose_kind {
    use hebrew_accents::{Accent, AccentKind, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_kind() {
        assert_eq!(ProseAccent::Silluq.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Atnach.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Segolta.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Shalshelet.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::ZaqephQaton.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::ZaqephGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Revia.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Tiphcha.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Zarqa.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Pashta.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::ShenePastin.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Yetiv.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Tevir.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Geresh.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Gershayim.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Pazer.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::PazerGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::TelishaGedolah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Legarmeh.kind(), Some(AccentKind::Primary));
    }

    #[test]
    fn test_prose_conjunctive_kind() {
        // Conjunctives: indices AccentKind::Primary-AccentKind::Primary, all should be None
        assert_eq!(ProseAccent::Munach.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Mahpakh.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Merkha.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::MerkhaKephulah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Darga.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Azla.kind(), Some(AccentKind::Primary));
        assert_eq!(
            ProseAccent::TelishaQetannah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(ProseAccent::Galgal.kind(), Some(AccentKind::Primary));
        assert_eq!(ProseAccent::Meayla.kind(), Some(AccentKind::Secondary));
        assert_eq!(ProseAccent::Meteg.kind(), Some(AccentKind::Secondary));
    }
}
mod poetry_kind {
    use hebrew_accents::{Accent, AccentKind, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_kind() {
        // Primary disjunctives: indices AccentKind::Primary-AccentKind::Primary, strengths AccentKind::Primary-AccentKind::Primary
        assert_eq!(PoetryAccent::Silluq.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::OlehWeYored.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Atnach.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaGadol.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaMugrash.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::ShalsheletGadol.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::Tsinnor.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::ReviaQaton.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Dechi.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Pazer.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.kind(), Some(AccentKind::Primary));
    }

    #[test]
    fn test_poetry_conjunctive_kind() {
        assert_eq!(PoetryAccent::Munach.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Merkha.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Illuy.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Tarcha.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Galgal.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Mehuppakh.kind(), Some(AccentKind::Primary));
        assert_eq!(PoetryAccent::Azla.kind(), Some(AccentKind::Primary));
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.kind(),
            Some(AccentKind::Primary)
        );
        assert_eq!(PoetryAccent::Meteg.kind(), Some(AccentKind::Secondary));
    }
}
mod pseudo_kind {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_kind() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.kind(), None);
        assert_eq!(PseudoAccent::Maqqaph.kind(), None);
        assert_eq!(PseudoAccent::Paseq.kind(), None);
    }
}
