// TODO implementation is not ready yet
mod prose_alternate_names {
    use hebrew_accents::{Accent, AlternateNames, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_alternate_names() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.alternate_names(), None);
        assert_eq!(ProseAccent::Atnach.alternate_names(), None);
        assert_eq!(ProseAccent::Segolta.alternate_names(), None);
        assert_eq!(ProseAccent::Shalshelet.alternate_names(), None);
        assert_eq!(ProseAccent::ZaqephQatan.alternate_names(), None);
        assert_eq!(ProseAccent::ZaqephGadol.alternate_names(), None);
        assert_eq!(ProseAccent::Revia.alternate_names(), None);
        assert_eq!(ProseAccent::Tiphcha.alternate_names(), None);
        assert_eq!(ProseAccent::Zarqa.alternate_names(), None);
        assert_eq!(ProseAccent::Pashta.alternate_names(), None);
        assert_eq!(ProseAccent::Yetiv.alternate_names(), None);
        assert_eq!(ProseAccent::Tevir.alternate_names(), None);
        assert_eq!(
            ProseAccent::Geresh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "טֶרֶס",
                hebrew_concept: "a boundary or border",
                english_name: "Teres",
                sbl_academic: "ṭeres",
            })
        );
        assert_eq!(ProseAccent::Gershayim.alternate_names(), None);
        assert_eq!(ProseAccent::Pazer.alternate_names(), None);
        assert_eq!(
            ProseAccent::PazerGadol.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "קַרְנֵי פָרָה",
                hebrew_concept: "horns of a cow",
                english_name: "Qarne Pharah",
                sbl_academic: "qarnê pārâ",
            })
        );
        assert_eq!(ProseAccent::TelishaGedolah.alternate_names(), None);
        assert_eq!(ProseAccent::Legarmeh.alternate_names(), None);
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_alternate_names() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.alternate_names(), None);
        assert_eq!(
            ProseAccent::Mahpakh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(ProseAccent::Merkha.alternate_names(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.alternate_names(), None);
        assert_eq!(ProseAccent::Darga.alternate_names(), None);
        assert_eq!(ProseAccent::Azla.alternate_names(), None);
        assert_eq!(ProseAccent::TelishaQetannah.alternate_names(), None);
        assert_eq!(
            ProseAccent::Galgal.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(ProseAccent::Meayla.alternate_names(), None);
        assert_eq!(ProseAccent::Meteg.alternate_names(), None);
    }
}
mod poetry_alternate_names {
    use hebrew_accents::{Accent, AlternateNames, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_alternate_names() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.alternate_names(), None);
        assert_eq!(
            PoetryAccent::OlehWeYored.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::Atnach.alternate_names(), None);
        assert_eq!(PoetryAccent::ReviaGadol.alternate_names(), None);
        assert_eq!(PoetryAccent::ReviaMugrash.alternate_names(), None);
        assert_eq!(PoetryAccent::ShalsheletGadol.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Tsinnor.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::ReviaQaton.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Dechi.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::Pazer.alternate_names(), None);
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.alternate_names(), None);
        assert_eq!(PoetryAccent::AzlaLegarmeh.alternate_names(), None);
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_alternate_names() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.alternate_names(), None);
        assert_eq!(PoetryAccent::Merkha.alternate_names(), None);
        assert_eq!(PoetryAccent::Illuy.alternate_names(), None);
        assert_eq!(PoetryAccent::Tarcha.alternate_names(), None);
        assert_eq!(
            PoetryAccent::Galgal.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(
            PoetryAccent::Mehuppakh.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(
            PoetryAccent::Azla.alternate_names(),
            Some(AlternateNames {
                hebrew_name: "",
                hebrew_concept: "",
                sbl_academic: "",
                english_name: "",
            })
        );
        assert_eq!(PoetryAccent::ShalsheletQetannah.alternate_names(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.alternate_names(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.alternate_names(), None);
        assert_eq!(PoetryAccent::Meteg.alternate_names(), None);
    }
}
mod pseudo_alternate_names {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_alternate_names() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.alternate_names(), None);
        assert_eq!(PseudoAccent::Maqqaph.alternate_names(), None);
        assert_eq!(PseudoAccent::Paseq.alternate_names(), None);
    }
}
