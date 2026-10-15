mod prose_hebrew_concept {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_hebrew_concept() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.hebrew_concept(), "close, cessation");
        assert_eq!(ProseAccent::Atnach.hebrew_concept(), "a causing to rest");
        assert_eq!(
            ProseAccent::Segolta.hebrew_concept(),
            "a little grape-bunch"
        );
        assert_eq!(ProseAccent::Shalshelet.hebrew_concept(), "chain or link");
        assert_eq!(
            ProseAccent::ZaqephQatan.hebrew_concept(),
            "small upright or small standing one"
        );
        assert_eq!(ProseAccent::ZaqephGadol.hebrew_concept(), "large upright");
        assert_eq!(
            ProseAccent::Revia.hebrew_concept(),
            "fourth [in a sequence]"
        );
        assert_eq!(
            ProseAccent::Tiphcha.hebrew_concept(),
            "handbreadth or diagonal"
        );
        assert_eq!(
            ProseAccent::Zarqa.hebrew_concept(),
            "to sprinkle, to scatter"
        );
        assert_eq!(
            ProseAccent::Pashta.hebrew_concept(),
            "extending, stretching out in length"
        );
        assert_eq!(ProseAccent::Yetiv.hebrew_concept(), "resting or sitting");
        assert_eq!(
            ProseAccent::Tevir.hebrew_concept(),
            "broken, downward tumble"
        );
        assert_eq!(
            ProseAccent::Geresh.hebrew_concept(),
            "expulsion, driving out, divorce"
        );
        assert_eq!(
            ProseAccent::Gershayim.hebrew_concept(),
            "double of expulsion, driving out, divorce"
        );
        assert_eq!(ProseAccent::Pazer.hebrew_concept(), "lavish or scatter");
        assert_eq!(
            ProseAccent::PazerGadol.hebrew_concept(),
            "large lavish or scatter"
        );
        assert_eq!(
            ProseAccent::TelishaGedolah.hebrew_concept(),
            "great (long) detached"
        );
        assert_eq!(
            ProseAccent::Legarmeh.hebrew_concept(),
            "for or by itself, independent"
        );
    }

    #[test]
    fn test_prose_conjunctive_hebrew_concept() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.hebrew_concept(), "resting or placed");
        assert_eq!(ProseAccent::Mahpakh.hebrew_concept(), "turning round");
        assert_eq!(
            ProseAccent::Merkha.hebrew_concept(),
            "lengthener, prolonging"
        );
        assert_eq!(
            ProseAccent::MerkhaKephulah.hebrew_concept(),
            "double lengthener"
        );
        assert_eq!(ProseAccent::Darga.hebrew_concept(), "step, stair-step");
        assert_eq!(
            ProseAccent::Azla.hebrew_concept(),
            "going on (not pausing), depart"
        );
        assert_eq!(
            ProseAccent::TelishaQetannah.hebrew_concept(),
            "small (short) detached"
        );
        assert_eq!(ProseAccent::Galgal.hebrew_concept(), "wheel, circle");
        assert_eq!(ProseAccent::Meayla.hebrew_concept(), "upper, above, higher");
        assert_eq!(
            ProseAccent::Meteg.hebrew_concept(),
            "to bridle, check, control"
        );
    }
}
mod poetry_hebrew_concept {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_hebrew_concept() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.hebrew_concept(), "close, cessation");
        assert_eq!(
            PoetryAccent::OlehWeYored.hebrew_concept(),
            "ascending and descending"
        );
        assert_eq!(PoetryAccent::Atnach.hebrew_concept(), "a causing to rest");
        assert_eq!(PoetryAccent::ReviaGadol.hebrew_concept(), "big fourth");
        assert_eq!(PoetryAccent::ReviaMugrash.hebrew_concept(), "exiled fourth");
        assert_eq!(
            PoetryAccent::ShalsheletGadol.hebrew_concept(),
            "large chain or link"
        );
        assert_eq!(PoetryAccent::Tsinnor.hebrew_concept(), "pipe or tube");
        assert_eq!(PoetryAccent::ReviaQaton.hebrew_concept(), "small fourth");
        assert_eq!(
            PoetryAccent::Dechi.hebrew_concept(),
            "to push or drive away"
        );
        assert_eq!(PoetryAccent::Pazer.hebrew_concept(), "lavish or scatter");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.hebrew_concept(),
            "reversed to its own"
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.hebrew_concept(),
            "goes to its own"
        );
    }

    /// Test all poetry conjunctive accents (should return "
    #[test]
    fn test_poetry_conjunctive_hebrew_concept() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.hebrew_concept(), "resting or placed");
        assert_eq!(
            PoetryAccent::Merkha.hebrew_concept(),
            "lengthener, prolonging"
        );
        assert_eq!(PoetryAccent::Illuy.hebrew_concept(), "elevation or raising");
        assert_eq!(
            PoetryAccent::Tarcha.hebrew_concept(),
            "trouble, difficulty, hardship, toil"
        );
        assert_eq!(PoetryAccent::Galgal.hebrew_concept(), "wheel, circle");
        assert_eq!(PoetryAccent::Mehuppakh.hebrew_concept(), "reversed");
        assert_eq!(
            PoetryAccent::Azla.hebrew_concept(),
            "going on (not pausing), depart"
        );
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.hebrew_concept(),
            "small chain"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.hebrew_concept(),
            "pipe of continuation"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.hebrew_concept(),
            "pipe of reversal"
        );
        assert_eq!(
            PoetryAccent::Meteg.hebrew_concept(),
            "to bridle, check, control"
        );
    }
}
mod pseudo_hebrew_concept {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_hebrew_concept() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.hebrew_concept(), "end of verse");
        assert_eq!(PseudoAccent::Maqqaph.hebrew_concept(), "binder");
        assert_eq!(
            PseudoAccent::Paseq.hebrew_concept(),
            "to pause, to stop or to interrupt"
        );
    }
}
