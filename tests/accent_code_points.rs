use hebrew_accents::{PoetryAccent, ProseAccent, PseudoAccent, Accent};

#[test]
fn testing_prose_accent_code_points() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.code_points(), 1);
    assert_eq!(ProseAccent::Atnach.code_points(), 1);
    assert_eq!(ProseAccent::Segolta.code_points(), 1);
    assert_eq!(ProseAccent::Shalshelet.code_points(), 2);
    assert_eq!(ProseAccent::ZaqephQaton.code_points(), 1);
    assert_eq!(ProseAccent::ZaqephGadol.code_points(), 1);
    assert_eq!(ProseAccent::Revia.code_points(), 1);
    assert_eq!(ProseAccent::Tiphcha.code_points(), 1);
    assert_eq!(ProseAccent::Zarqa.code_points(), 1);
    assert_eq!(ProseAccent::Pashta.code_points(), 1);
    assert_eq!(ProseAccent::Yetiv.code_points(), 1);
    assert_eq!(ProseAccent::Tevir.code_points(), 1);
    assert_eq!(ProseAccent::Geresh.code_points(), 1);
    assert_eq!(ProseAccent::Gershayim.code_points(), 1);
    assert_eq!(ProseAccent::Pazer.code_points(), 1);
    assert_eq!(ProseAccent::PazerGadol.code_points(), 1);
    assert_eq!(ProseAccent::TelishaGedolah.code_points(), 1);
    assert_eq!(ProseAccent::Legarmeh.code_points(), 2);
    // Conjunctives
    assert_eq!(ProseAccent::Munach.code_points(), 1);
    assert_eq!(ProseAccent::Mahpakh.code_points(), 1);
    assert_eq!(ProseAccent::Merkha.code_points(), 1);
    assert_eq!(ProseAccent::MerkhaKephulah.code_points(), 1);
    assert_eq!(ProseAccent::Darga.code_points(), 1);
    assert_eq!(ProseAccent::Azla.code_points(), 1);
    assert_eq!(ProseAccent::TelishaQetannah.code_points(), 1);
    assert_eq!(ProseAccent::Galgal.code_points(), 1);
    assert_eq!(ProseAccent::Mayela.code_points(), 1);
    assert_eq!(ProseAccent::Meteg.code_points(), 1);
}

#[test]
fn testing_poetry_accent_code_points() {
    // Disjunctives
    assert_eq!(PoetryAccent::Silluq.code_points(), 1);
    assert_eq!(PoetryAccent::OlehWeYored.code_points(), 2,);
    assert_eq!(PoetryAccent::Atnach.code_points(), 1);
    assert_eq!(PoetryAccent::ReviaGadol.code_points(), 1);
    assert_eq!(PoetryAccent::ReviaMugrash.code_points(), 2);
    assert_eq!(PoetryAccent::ShalsheletGadol.code_points(), 2);
    assert_eq!(PoetryAccent::Tsinnor.code_points(), 1);
    assert_eq!(PoetryAccent::ReviaQaton.code_points(), 1);
    assert_eq!(PoetryAccent::Dechi.code_points(), 1);
    assert_eq!(PoetryAccent::Pazer.code_points(), 1);
    assert_eq!(PoetryAccent::MehuppakhLegarmeh.code_points(), 2);
    assert_eq!(PoetryAccent::AzlaLegarmeh.code_points(), 2);
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.code_points(), 1);
    assert_eq!(PoetryAccent::Merkha.code_points(), 1);
    assert_eq!(PoetryAccent::Illuy.code_points(), 1);
    assert_eq!(PoetryAccent::Tarcha.code_points(), 1);
    assert_eq!(PoetryAccent::Galgal.code_points(), 1);
    assert_eq!(PoetryAccent::Mehuppakh.code_points(), 1);
    assert_eq!(PoetryAccent::Azla.code_points(), 1);
    assert_eq!(PoetryAccent::ShalsheletQetannah.code_points(), 1);
    assert_eq!(PoetryAccent::TsinnoritMerkha.code_points(), 2);
    assert_eq!(PoetryAccent::TsinnoritMahpakh.code_points(), 2);
    assert_eq!(PoetryAccent::Meteg.code_points(), 1);
}

#[test]
fn testing_pseudo_accent_code_points() {
    assert_eq!(PseudoAccent::SophPasuq.code_points(), 1);
    assert_eq!(PseudoAccent::Maqqeph.code_points(), 1,);
    assert_eq!(PseudoAccent::Paseq.code_points(), 1,);
}
