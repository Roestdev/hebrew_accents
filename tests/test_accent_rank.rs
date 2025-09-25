use hebrew_accents::{PoetryAccent, ProseAccent};

fn main() {
    assert_eq!(ProseAccent::Silluq.rank(), 1);
    assert_eq!(ProseAccent::Atnach.rank(), 2);
    assert_eq!(ProseAccent::Segolta.rank(), 3);
    assert_eq!(ProseAccent::Shalshelet.rank(), 4);
    assert_eq!(ProseAccent::ZaqephQaton.rank(), 5);
    assert_eq!(ProseAccent::ZaqephGadol.rank(), 6);
    assert_eq!(ProseAccent::Revia.rank(), 7);
    assert_eq!(ProseAccent::Tiphcha.rank(), 8);
    assert_eq!(ProseAccent::Zarqa.rank(), 9);
    assert_eq!(ProseAccent::Pashta.rank(), 10);
    assert_eq!(ProseAccent::Yetiv.rank(), 11);
    assert_eq!(ProseAccent::Tevir.rank(), 12);
    assert_eq!(ProseAccent::Geresh.rank(), 13);
    assert_eq!(ProseAccent::Gershayim.rank(), 14);
    assert_eq!(ProseAccent::Pazer.rank(), 15);
    assert_eq!(ProseAccent::PazerGadol.rank(), 16);
    assert_eq!(ProseAccent::TelishaGedolah.rank(), 17);
    assert_eq!(ProseAccent::Legarmeh.rank(), 18);
    // Conjunctives
    assert_eq!(ProseAccent::Munach.rank(), 19);
    assert_eq!(ProseAccent::Mahpakh.rank(), 20);
    assert_eq!(ProseAccent::Merkha.rank(), 21);
    assert_eq!(ProseAccent::MerkhaKephulah.rank(), 22);
    assert_eq!(ProseAccent::Darga.rank(), 23);
    assert_eq!(ProseAccent::Azla.rank(), 24);
    assert_eq!(ProseAccent::TelishaQetannah.rank(), 25);
    assert_eq!(ProseAccent::Galgal.rank(), 26);
    assert_eq!(ProseAccent::Mayela.rank(), 27);
    assert_eq!(ProseAccent::Meteg.rank(), 28);
    assert_eq!(ProseAccent::Maqqeph.rank(), 29);


                assert_eq!(PoetryAccent::Silluq.rank(), 1);
            assert_eq!(PoetryAccent::OlehWeYored.rank(), 2,);
            assert_eq!(PoetryAccent::Atnach.rank(), 3);
            assert_eq!(PoetryAccent::ReviaGadol.rank(), 4);
            assert_eq!(PoetryAccent::ReviaMugrash.rank(), 5);
            assert_eq!(PoetryAccent::ShalsheletGadol.rank(), 6);
            assert_eq!(PoetryAccent::Tsinnor.rank(), 7);
            assert_eq!(PoetryAccent::ReviaQaton.rank(), 8);
            assert_eq!(PoetryAccent::Dechi.rank(), 9);
            assert_eq!(PoetryAccent::Pazer.rank(), 10);
            assert_eq!(PoetryAccent::MehuppakhLegarmeh.rank(), 11);
            assert_eq!(PoetryAccent::AzlaLegarmeh.rank(), 12);
            // Conjunctives
            assert_eq!(PoetryAccent::Munach.rank(), 13);
            assert_eq!(PoetryAccent::Merkha.rank(), 14);
            assert_eq!(PoetryAccent::Illuy.rank(), 15);
            assert_eq!(PoetryAccent::Tarkha.rank(), 16);
            assert_eq!(PoetryAccent::Galgal.rank(), 17);
            assert_eq!(PoetryAccent::Mehuppakh.rank(), 18);
            assert_eq!(PoetryAccent::Azla.rank(), 19);
            assert_eq!(PoetryAccent::ShalsheletQetannah.rank(), 20);
            assert_eq!(PoetryAccent::TsinnoritMerkha.rank(), 21);
            assert_eq!(PoetryAccent::TsinnoritMahpakh.rank(), 21);
            assert_eq!(PoetryAccent::Meteg.rank(), 22);
            assert_eq!(PoetryAccent::Maqqeph.rank(), 23);
}
