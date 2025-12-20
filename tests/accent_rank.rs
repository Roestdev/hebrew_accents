//use hebrew_accents::{PoetryAccent, ProseAccent};
use hebrew_accents::{PoetryAccent, ProseAccent};
#[test]
fn testing_prose_accent_relative_strengths() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.relative_strength(), 1);
    assert_eq!(ProseAccent::Atnach.relative_strength(), 2);
    assert_eq!(ProseAccent::Segolta.relative_strength(), 3);
    assert_eq!(ProseAccent::Shalshelet.relative_strength(), 4);
    assert_eq!(ProseAccent::ZaqephQaton.relative_strength(), 5);
    assert_eq!(ProseAccent::ZaqephGadol.relative_strength(), 6);
    assert_eq!(ProseAccent::Revia.relative_strength(), 7);
    assert_eq!(ProseAccent::Tiphcha.relative_strength(), 8);
    assert_eq!(ProseAccent::Zarqa.relative_strength(), 9);
    assert_eq!(ProseAccent::Pashta.relative_strength(), 10);
    assert_eq!(ProseAccent::Yetiv.relative_strength(), 11);
    assert_eq!(ProseAccent::Tevir.relative_strength(), 12);
    assert_eq!(ProseAccent::Geresh.relative_strength(), 13);
    assert_eq!(ProseAccent::Gershayim.relative_strength(), 14);
    assert_eq!(ProseAccent::Pazer.relative_strength(), 15);
    assert_eq!(ProseAccent::PazerGadol.relative_strength(), 16);
    assert_eq!(ProseAccent::TelishaGedolah.relative_strength(), 17);
    assert_eq!(ProseAccent::Legarmeh.relative_strength(), 18);
    // Conjunctives
    assert_eq!(ProseAccent::Munach.relative_strength(), 19);
    assert_eq!(ProseAccent::Mahpakh.relative_strength(), 20);
    assert_eq!(ProseAccent::Merkha.relative_strength(), 21);
    assert_eq!(ProseAccent::MerkhaKephulah.relative_strength(), 22);
    assert_eq!(ProseAccent::Darga.relative_strength(), 23);
    assert_eq!(ProseAccent::Azla.relative_strength(), 24);
    assert_eq!(ProseAccent::TelishaQetannah.relative_strength(), 25);
    assert_eq!(ProseAccent::Galgal.relative_strength(), 26);
    assert_eq!(ProseAccent::Mayela.relative_strength(), 27);
    assert_eq!(ProseAccent::Meteg.relative_strength(), 28);
    assert_eq!(ProseAccent::Maqqeph.relative_strength(), 29);
}

#[test]
fn testing_poetry_accent_relative_strengths() {
    // Disjunctives
    assert_eq!(PoetryAccent::Silluq.relative_strength(), 1);
    assert_eq!(PoetryAccent::OlehWeYored.relative_strength(), 2,);
    assert_eq!(PoetryAccent::Atnach.relative_strength(), 3);
    assert_eq!(PoetryAccent::ReviaGadol.relative_strength(), 4);
    assert_eq!(PoetryAccent::ReviaMugrash.relative_strength(), 5);
    assert_eq!(PoetryAccent::ShalsheletGadol.relative_strength(), 6);
    assert_eq!(PoetryAccent::Tsinnor.relative_strength(), 7);
    assert_eq!(PoetryAccent::ReviaQaton.relative_strength(), 8);
    assert_eq!(PoetryAccent::Dechi.relative_strength(), 9);
    assert_eq!(PoetryAccent::Pazer.relative_strength(), 10);
    assert_eq!(PoetryAccent::MehuppakhLegarmeh.relative_strength(), 11);
    assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), 12);
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.relative_strength(), 13);
    assert_eq!(PoetryAccent::Merkha.relative_strength(), 14);
    assert_eq!(PoetryAccent::Illuy.relative_strength(), 15);
    assert_eq!(PoetryAccent::Tarkha.relative_strength(), 16);
    assert_eq!(PoetryAccent::Galgal.relative_strength(), 17);
    assert_eq!(PoetryAccent::Mehuppakh.relative_strength(), 18);
    assert_eq!(PoetryAccent::Azla.relative_strength(), 19);
    assert_eq!(PoetryAccent::ShalsheletQetannah.relative_strength(), 20);
    assert_eq!(PoetryAccent::TsinnoritMerkha.relative_strength(), 21);
    assert_eq!(PoetryAccent::TsinnoritMahpakh.relative_strength(), 21);
    assert_eq!(PoetryAccent::Meteg.relative_strength(), 22);
    assert_eq!(PoetryAccent::Maqqeph.relative_strength(), 23);
}
