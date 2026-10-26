mod prose_hebrew_name {
    use hebrew_accents::{Accent, ProseAccent};

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_hebrew_name() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(ProseAccent::Atnach.hebrew_name(), "אַתְנַח");
        assert_eq!(ProseAccent::Segolta.hebrew_name(), "סְגֹולְתָּא");
        assert_eq!(ProseAccent::Shalshelet.hebrew_name(), "שַׁלְשֶׁלֶת");
        assert_eq!(ProseAccent::ZaqephQaton.hebrew_name(), "זָקֵף קָטוֹן");
        assert_eq!(ProseAccent::ZaqephGadol.hebrew_name(), "זָקֵף גָּדוֹל");
        assert_eq!(ProseAccent::Revia.hebrew_name(), "רְבִיעַ");
        assert_eq!(ProseAccent::Tiphcha.hebrew_name(), "טִפְחָא");
        assert_eq!(ProseAccent::Zarqa.hebrew_name(), "זַרְקָא");
        assert_eq!(ProseAccent::Pashta.hebrew_name(), "פַּשְׁטָא");
        assert_eq!(ProseAccent::ShenePastin.hebrew_name(), "שְׁנֵ֨י פַּשְׁטִין֙");
        assert_eq!(ProseAccent::Yetiv.hebrew_name(), "יְתִיב");
        assert_eq!(ProseAccent::Tevir.hebrew_name(), "תְּבִיר");
        assert_eq!(ProseAccent::Geresh.hebrew_name(), "גֵּרֵישׁ");
        assert_eq!(ProseAccent::Gershayim.hebrew_name(), "גֵּרְשַׁיִם");
        assert_eq!(ProseAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(ProseAccent::PazerGadol.hebrew_name(), "פָּזֶר גּדוֹל");
        assert_eq!(ProseAccent::TelishaGedolah.hebrew_name(), "תְּלִישָׁא גְּדוֹלָה");
        assert_eq!(ProseAccent::Legarmeh.hebrew_name(), "לְגַרְמֶהּ");
    }

    #[test]
    fn test_prose_conjunctive_hebrew_name() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(ProseAccent::Mahpakh.hebrew_name(), "מַהְפַּךְ");
        assert_eq!(ProseAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(ProseAccent::MerkhaKephulah.hebrew_name(), "מֵרְכָא כְּפוּלָה");
        assert_eq!(ProseAccent::Darga.hebrew_name(), "דַּרְגָּא");
        assert_eq!(ProseAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(ProseAccent::TelishaQetannah.hebrew_name(), "תְּלִישָא קְטַנָּה");
        assert_eq!(ProseAccent::Galgal.hebrew_name(), "גַּלְגַּל",);
        assert_eq!(ProseAccent::Meayla.hebrew_name(), "מְאַיְלָא");
        assert_eq!(ProseAccent::Meteg.hebrew_name(), "מֶתֶג");
    }
}
mod poetry_hebrew_name {
    use hebrew_accents::{Accent, PoetryAccent};
    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_hebrew_name() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.hebrew_name(), "סִלּוּק");
        assert_eq!(PoetryAccent::OlehWeYored.hebrew_name(), "עֹלֶה וְיֹרֵד");
        assert_eq!(PoetryAccent::Atnach.hebrew_name(), "אַתְנַח");
        assert_eq!(PoetryAccent::ReviaGadol.hebrew_name(), "רְבִיעַ גָּדוֹל");
        assert_eq!(PoetryAccent::ReviaMugrash.hebrew_name(), "רְבִיעַ מֻגְרָשׁ");
        assert_eq!(PoetryAccent::ShalsheletGadol.hebrew_name(), "שַׁלְשֶׁלֶת גָּדוֹל");
        assert_eq!(PoetryAccent::Tsinnor.hebrew_name(), "צִנּוֹר");
        assert_eq!(PoetryAccent::ReviaQaton.hebrew_name(), "רְבִיעַ קָטוֹן");
        assert_eq!(PoetryAccent::Dechi.hebrew_name(), "דֶּחִי");
        assert_eq!(PoetryAccent::Pazer.hebrew_name(), "פָּזֶר");
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.hebrew_name(), "מְהֻפָּךְ לְגַרְמֵהּ");
        assert_eq!(PoetryAccent::AzlaLegarmeh.hebrew_name(), "אַזְלָא לְגַרְמֶהּ");
    }

    /// Test all poetry conjunctive accents
    #[test]
    fn test_poetry_conjunctive_hebrew_name() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.hebrew_name(), "מוּנַ֣ח");
        assert_eq!(PoetryAccent::Merkha.hebrew_name(), "מֵרְכָא");
        assert_eq!(PoetryAccent::Illuy.hebrew_name(), "עִלּוּי");
        assert_eq!(PoetryAccent::Tarcha.hebrew_name(), "טַרְחָא");
        assert_eq!(PoetryAccent::Galgal.hebrew_name(), "גַּלְגַּל",);
        assert_eq!(PoetryAccent::Mehuppakh.hebrew_name(), "מְהֻפָּ֤ךְ");
        assert_eq!(PoetryAccent::Azla.hebrew_name(), "אַזְלָא");
        assert_eq!(PoetryAccent::ShalsheletQetannah.hebrew_name(), "שַׁלְשֶׁלֶת קְטַנָּה");
        assert_eq!(PoetryAccent::TsinnoritMerkha.hebrew_name(), "צִנּוֹרִת מֵרְכָא");
        assert_eq!(PoetryAccent::TsinnoritMahpakh.hebrew_name(), "צִנּוֹרִת מַהְפַּךְ");
        assert_eq!(PoetryAccent::Meteg.hebrew_name(), "מֶתֶג");
    }
}
mod pseudo_hebrew_name {
    use hebrew_accents::{Accent, PseudoAccent};
    #[test]
    fn test_pseudo_accent_hebrew_name() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.hebrew_name(), "סוֹף פָּסוּק");
        assert_eq!(PseudoAccent::Maqqaph.hebrew_name(), "מַקָּף");
        assert_eq!(PseudoAccent::Paseq.hebrew_name(), "פָּסֵק");
    }
}
