mod prose_tier {
    use hebrew_accents::{Accent, GroupLevel, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_tier() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.tier(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Atnach.tier(), Some(GroupLevel::Tier1));
        assert_eq!(ProseAccent::Segolta.tier(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::Shalshelet.tier(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::ZaqephQaton.tier(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::ZaqephGadol.tier(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::Revia.tier(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tiphcha.tier(), Some(GroupLevel::Tier2));
        assert_eq!(ProseAccent::Zarqa.tier(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Pashta.tier(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Yetiv.tier(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Tevir.tier(), Some(GroupLevel::Tier3));
        assert_eq!(ProseAccent::Geresh.tier(), Some(GroupLevel::Tier4));
        assert_eq!(ProseAccent::Gershayim.tier(), Some(GroupLevel::Tier4));
        assert_eq!(ProseAccent::Pazer.tier(), Some(GroupLevel::Tier4));
        assert_eq!(ProseAccent::PazerGadol.tier(), Some(GroupLevel::Tier4));
        assert_eq!(ProseAccent::TelishaGedolah.tier(), Some(GroupLevel::Tier4));
        assert_eq!(ProseAccent::Legarmeh.tier(), Some(GroupLevel::Tier4));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_tier() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.tier(), None);
        assert_eq!(ProseAccent::Mahpakh.tier(), None);
        assert_eq!(ProseAccent::Merkha.tier(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.tier(), None);
        assert_eq!(ProseAccent::Darga.tier(), None);
        assert_eq!(ProseAccent::Azla.tier(), None);
        assert_eq!(ProseAccent::TelishaQetannah.tier(), None);
        assert_eq!(ProseAccent::Galgal.tier(), None);
        assert_eq!(ProseAccent::Meayla.tier(), None);
        assert_eq!(ProseAccent::Meteg.tier(), None);
    }
}
mod poetry_tier {
    use hebrew_accents::{Accent, GroupLevel, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_tier() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.tier(), Some(GroupLevel::Tier1));
        assert_eq!(PoetryAccent::OlehWeYored.tier(), Some(GroupLevel::Tier1));
        assert_eq!(PoetryAccent::Atnach.tier(), Some(GroupLevel::Tier1));
        assert_eq!(PoetryAccent::ReviaGadol.tier(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::ReviaMugrash.tier(), Some(GroupLevel::Tier2));
        assert_eq!(
            PoetryAccent::ShalsheletGadol.tier(),
            Some(GroupLevel::Tier2)
        );
        assert_eq!(PoetryAccent::Tsinnor.tier(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::ReviaQaton.tier(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::Dechi.tier(), Some(GroupLevel::Tier2));
        assert_eq!(PoetryAccent::Pazer.tier(), Some(GroupLevel::Tier3));
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.tier(),
            Some(GroupLevel::Tier3)
        );
        assert_eq!(PoetryAccent::AzlaLegarmeh.tier(), Some(GroupLevel::Tier3));
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_tier() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.tier(), None);
        assert_eq!(PoetryAccent::Merkha.tier(), None);
        assert_eq!(PoetryAccent::Illuy.tier(), None);
        assert_eq!(PoetryAccent::Tarcha.tier(), None);
        assert_eq!(PoetryAccent::Galgal.tier(), None);
        assert_eq!(PoetryAccent::Mehuppakh.tier(), None);
        assert_eq!(PoetryAccent::Azla.tier(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.tier(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.tier(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.tier(), None);
        assert_eq!(PoetryAccent::Meteg.tier(), None);
    }
}
mod pseudo_tier {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_tier() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.tier(), None);
        assert_eq!(PseudoAccent::Maqqaph.tier(), None);
        assert_eq!(PseudoAccent::Paseq.tier(), None);
    }
}
