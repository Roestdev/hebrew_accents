mod prose_sbl_academic_name {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_sbl_academic_name() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.sbl_academic_name(), "sillûq");
        assert_eq!(ProseAccent::Atnach.sbl_academic_name(), "ʾatnaḥ");
        assert_eq!(ProseAccent::Segolta.sbl_academic_name(), "səgôlətāʾ");
        assert_eq!(ProseAccent::Shalshelet.sbl_academic_name(), "šalšelet");
        assert_eq!(ProseAccent::ZaqephQatan.sbl_academic_name(), "zāqēp qāṭôn");
        assert_eq!(ProseAccent::ZaqephGadol.sbl_academic_name(), "zāqēp gādôl");
        assert_eq!(ProseAccent::Revia.sbl_academic_name(), "rəbîaʿ");
        assert_eq!(ProseAccent::Tiphcha.sbl_academic_name(), "ṭipḥāʾ");
        assert_eq!(ProseAccent::Zarqa.sbl_academic_name(), "zarqāʾ");
        assert_eq!(ProseAccent::Pashta.sbl_academic_name(), "pašṭāʾ");
        assert_eq!(ProseAccent::Yetiv.sbl_academic_name(), "yətîb");
        assert_eq!(ProseAccent::Tevir.sbl_academic_name(), "təbîr");
        assert_eq!(ProseAccent::Geresh.sbl_academic_name(), "gērêš");
        assert_eq!(ProseAccent::Gershayim.sbl_academic_name(), "gērəšayīm");
        assert_eq!(ProseAccent::Pazer.sbl_academic_name(), "pāzer");
        assert_eq!(ProseAccent::PazerGadol.sbl_academic_name(), "pāzer gdôl");
        assert_eq!(
            ProseAccent::TelishaGedolah.sbl_academic_name(),
            "təlîšāʾ gədôlâ"
        );
        assert_eq!(ProseAccent::Legarmeh.sbl_academic_name(), "ləgarmeh");
    }

    #[test]
    fn test_prose_conjunctive_sbl_academic_name() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.sbl_academic_name(), "mûnaḥ");
        assert_eq!(ProseAccent::Mahpakh.sbl_academic_name(), "mahpak");
        assert_eq!(ProseAccent::Merkha.sbl_academic_name(), "mērəkāʾ");
        assert_eq!(
            ProseAccent::MerkhaKephulah.sbl_academic_name(),
            "mērəkāʾ kəpûlâ"
        );
        assert_eq!(ProseAccent::Darga.sbl_academic_name(), "dargāʾ");
        assert_eq!(ProseAccent::Azla.sbl_academic_name(), "ʾazlāʾ");
        assert_eq!(
            ProseAccent::TelishaQetannah.sbl_academic_name(),
            "təlîšāʾ qəṭannâ"
        );
        assert_eq!(ProseAccent::Galgal.sbl_academic_name(), "galgal");
        assert_eq!(ProseAccent::Meayla.sbl_academic_name(), "məʾaylāʾ");
        assert_eq!(ProseAccent::Meteg.sbl_academic_name(), "meteg");
    }
}
mod poetry_sbl_academic_name {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    /// Silluq, Atnach,pazer zie prose // TODO
    #[test]
    fn test_poetry_disjunctive_sbl_academic_name() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.sbl_academic_name(), "sillûq");
        assert_eq!(
            PoetryAccent::OlehWeYored.sbl_academic_name(),
            "ʿōleh wəyōrēd"
        );
        assert_eq!(PoetryAccent::Atnach.sbl_academic_name(), "ʾatnaḥ");
        assert_eq!(PoetryAccent::ReviaGadol.sbl_academic_name(), "rəbîaʿ gādôl");
        assert_eq!(
            PoetryAccent::ReviaMugrash.sbl_academic_name(),
            "rəbîaʿ mugrāš"
        );
        assert_eq!(
            PoetryAccent::ShalsheletGadol.sbl_academic_name(),
            "šalšelet gādôl"
        );
        assert_eq!(PoetryAccent::Tsinnor.sbl_academic_name(), "ṣinnôr");
        assert_eq!(PoetryAccent::ReviaQaton.sbl_academic_name(), "rəbîaʿ qāṭôn");
        assert_eq!(PoetryAccent::Dechi.sbl_academic_name(), "deḥî");
        assert_eq!(PoetryAccent::Pazer.sbl_academic_name(), "pāzer");
        assert_eq!(
            PoetryAccent::MehuppakhLegarmeh.sbl_academic_name(),
            "məhuppāk ləgarmēh"
        );
        assert_eq!(
            PoetryAccent::AzlaLegarmeh.sbl_academic_name(),
            "ʾazlāʾ ləgarmēh"
        );
    }

    /// Test all poetry conjunctive accents (should return None)
    /// // TODO common: munach,merkha,galgal,azla,meteg
    #[test]
    fn test_poetry_conjunctive_sbl_academic_name() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.sbl_academic_name(), "mûnaḥ");
        assert_eq!(PoetryAccent::Merkha.sbl_academic_name(), "mērəkāʾ");
        assert_eq!(PoetryAccent::Illuy.sbl_academic_name(), "ʿillûy");
        assert_eq!(PoetryAccent::Tarcha.sbl_academic_name(), "ṭarḥāʾ");
        assert_eq!(PoetryAccent::Galgal.sbl_academic_name(), "galgal");
        assert_eq!(PoetryAccent::Mehuppakh.sbl_academic_name(), "məhuppāk");
        assert_eq!(PoetryAccent::Azla.sbl_academic_name(), "ʾazlāʾ");
        assert_eq!(
            PoetryAccent::ShalsheletQetannah.sbl_academic_name(),
            "šalšelet qəṭannâ"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMerkha.sbl_academic_name(),
            "ṣinnôrīt mērəkāʾ"
        );
        assert_eq!(
            PoetryAccent::TsinnoritMahpakh.sbl_academic_name(),
            "ṣinnôrīt mahpak"
        );
        assert_eq!(PoetryAccent::Meteg.sbl_academic_name(), "meteg");
    }
}
mod pseudo_sbl_academic_name {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_sbl_academic_name() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.sbl_academic_name(), "sôp pāsûq");
        assert_eq!(PseudoAccent::Maqqaph.sbl_academic_name(), "maqqēp");
        assert_eq!(PseudoAccent::Paseq.sbl_academic_name(), "pāsēq");
    }
}
