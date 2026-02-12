use hebrew_accents::{Accent, PoetryAccent, ProseAccent, PseudoAccent};

#[test]
fn testing_prose_accent_english_name() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.english_name(), "Silluq");
    assert_eq!(ProseAccent::Atnach.english_name(), "Atnach");
    assert_eq!(ProseAccent::Segolta.english_name(), "Segolta");
    assert_eq!(ProseAccent::Shalshelet.english_name(), "Shalshelet");
    assert_eq!(ProseAccent::ZaqephQatan.english_name(), "Zaqeph Qaton");
    assert_eq!(ProseAccent::ZaqephGadol.english_name(), "Zaqeph Gadol");
    assert_eq!(ProseAccent::Revia.english_name(), "Revia");
    assert_eq!(ProseAccent::Tiphcha.english_name(), "Tiphcha");
    assert_eq!(ProseAccent::Zarqa.english_name(), "Zarqa");
    assert_eq!(ProseAccent::Pashta.english_name(), "Pashta");
    assert_eq!(ProseAccent::Yetiv.english_name(), "Yetiv");
    assert_eq!(ProseAccent::Tevir.english_name(), "Tevir");
    assert_eq!(ProseAccent::Geresh.english_name(), "Geresh");
    assert_eq!(ProseAccent::Gershayim.english_name(), "Gershayim");
    assert_eq!(ProseAccent::Pazer.english_name(), "Pazer");
    assert_eq!(ProseAccent::PazerGadol.english_name(), "Pazer Gadol");
    assert_eq!(
        ProseAccent::TelishaGedolah.english_name(),
        "Telisha Gedolah"
    );
    assert_eq!(ProseAccent::Legarmeh.english_name(), "Legarmeh");
    // Conjunctives
    assert_eq!(ProseAccent::Munach.english_name(), "Munach");
    assert_eq!(ProseAccent::Mahpakh.english_name(), "Mahpakh");
    assert_eq!(ProseAccent::Merkha.english_name(), "Merkha");
    assert_eq!(
        ProseAccent::MerkhaKephulah.english_name(),
        "Merkha Kephulah"
    );
    assert_eq!(ProseAccent::Darga.english_name(), "Darga");
    assert_eq!(ProseAccent::Azla.english_name(), "Azla");
    assert_eq!(
        ProseAccent::TelishaQetannah.english_name(),
        "Telisha Qetannah"
    );
    assert_eq!(ProseAccent::Galgal.english_name(), "Galgal");
    assert_eq!(ProseAccent::Mayela.english_name(), "Mayela");
    assert_eq!(ProseAccent::Meteg.english_name(), "Meteg");
}

#[test]
fn testing_poetry_accent_english_name() {
    // Disjunctives
    assert_eq!(PoetryAccent::Silluq.english_name(), "Silluq");
    assert_eq!(PoetryAccent::OlehWeYored.english_name(), "Oleh We Yored");
    assert_eq!(PoetryAccent::Atnach.english_name(), "Atnach");
    assert_eq!(PoetryAccent::ReviaGadol.english_name(), "Revia Gadol");
    assert_eq!(PoetryAccent::ReviaMugrash.english_name(), "Revia Mugrash");
    assert_eq!(
        PoetryAccent::ShalsheletGadol.english_name(),
        "Shalshelet Gadol"
    );
    assert_eq!(PoetryAccent::Tsinnor.english_name(), "Tsinnor");
    assert_eq!(PoetryAccent::ReviaQaton.english_name(), "Revia Qaton");
    assert_eq!(PoetryAccent::Dechi.english_name(), "Dechi");
    assert_eq!(PoetryAccent::Pazer.english_name(), "Pazer");
    assert_eq!(
        PoetryAccent::MehuppakhLegarmeh.english_name(),
        "Mehuppakh Legarmeh"
    );
    assert_eq!(PoetryAccent::AzlaLegarmeh.english_name(), "Azla Legarmeh");
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.english_name(), "Munach");
    assert_eq!(PoetryAccent::Merkha.english_name(), "Merkha");
    assert_eq!(PoetryAccent::Illuy.english_name(), "Illuy");
    assert_eq!(PoetryAccent::Tarcha.english_name(), "Tarcha");
    assert_eq!(PoetryAccent::Galgal.english_name(), "Galgal");
    assert_eq!(PoetryAccent::Mehuppakh.english_name(), "Mehuppakh");
    assert_eq!(PoetryAccent::Azla.english_name(), "Azla");
    assert_eq!(
        PoetryAccent::ShalsheletQetannah.english_name(),
        "Shalshelet Qetannah"
    );
    assert_eq!(
        PoetryAccent::TsinnoritMerkha.english_name(),
        "Tsinnorit Merkha"
    );
    assert_eq!(
        PoetryAccent::TsinnoritMahpakh.english_name(),
        "Tsinnorit Mahpakh"
    );
    assert_eq!(PoetryAccent::Meteg.english_name(), "Meteg");
}

#[test]
fn testing_pseudo_accent_english_name() {
    assert_eq!(PseudoAccent::SophPasuq.english_name(), "Soph Pasuq");
    assert_eq!(PseudoAccent::Maqqeph.english_name(), "Maqqeph");
    assert_eq!(PseudoAccent::Paseq.english_name(), "Paseq");
}
