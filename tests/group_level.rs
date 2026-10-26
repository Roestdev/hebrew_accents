mod prose_group_level {
    use hebrew_accents::{Accent, GroupLevel, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_group_level() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Atnach.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Segolta.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(
            ProseAccent::Shalshelet.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            ProseAccent::ZaqephQaton.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            ProseAccent::ZaqephGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(ProseAccent::Revia.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tiphcha.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::Zarqa.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Pashta.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Yetiv.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tevir.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Geresh.group_level(), Some(GroupLevel::Tier4));
        assert_eq!(
            ProseAccent::Gershayim.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(ProseAccent::Pazer.group_level(), Some(GroupLevel::Tier4));
        assert_eq!(
            ProseAccent::PazerGadol.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.group_level(),
            Some(GroupLevel::Tier4)
        );
        assert_eq!(ProseAccent::Legarmeh.group_level(), Some(GroupLevel::Tier4));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_group_level() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.group_level(), None);
        assert_eq!(ProseAccent::Mahpakh.group_level(), None);
        assert_eq!(ProseAccent::Merkha.group_level(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.group_level(), None);
        assert_eq!(ProseAccent::Darga.group_level(), None);
        assert_eq!(ProseAccent::Azla.group_level(), None);
        assert_eq!(ProseAccent::TelishaQetannah.group_level(), None);
        assert_eq!(ProseAccent::Galgal.group_level(), None);
        assert_eq!(ProseAccent::Meayla.group_level(), None);
        assert_eq!(ProseAccent::Meteg.group_level(), None);
    }
}
mod poetry_group_level {
    use hebrew_accents::{Accent, GroupLevel, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_group_level() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(
            PoetryAccent::OlehWeYored.group_level(),
            Some(GroupLevel::Tier1)
        );
        assert_eq!(PoetryAccent::Atnach.group_level(), Some(GroupLevel::Tier1));
        assert_eq!(
            PoetryAccent::ReviaGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            PoetryAccent::ReviaMugrash.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(PoetryAccent::Tsinnor.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(
            PoetryAccent::ReviaQaton.group_level(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(PoetryAccent::Dechi.group_level(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::Pazer.group_level(), Some(GroupLevel::Tier3));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.group_level(),
            Some(GroupLevel::Tier3)
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.group_level(),
            Some(GroupLevel::Tier3)
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_group_level() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.group_level(), None);
        assert_eq!(PoetryAccent::Merkha.group_level(), None);
        assert_eq!(PoetryAccent::Illuy.group_level(), None);
        assert_eq!(PoetryAccent::Tarcha.group_level(), None);
        assert_eq!(PoetryAccent::Galgal.group_level(), None);
        assert_eq!(PoetryAccent::Mehuppakh.group_level(), None);
        assert_eq!(PoetryAccent::Azla.group_level(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.group_level(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.group_level(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.group_level(), None);
        assert_eq!(PoetryAccent::Meteg.group_level(), None);
    }
}
mod pseudo_group_level {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_group_level() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.group_level(), None);
        assert_eq!(PseudoAccent::Maqqaph.group_level(), None);
        assert_eq!(PseudoAccent::Paseq.group_level(), None);
    }
}
