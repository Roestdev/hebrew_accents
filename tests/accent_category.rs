use hebrew_accents::{Accent, AccentCategory, PoetryAccent, ProseAccent};

#[test]
fn testing_prose_accent_categories() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Atnach.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Segolta.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Shalshelet.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::ZaqephQaton.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::ZaqephGadol.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Revia.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Tiphcha.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Zarqa.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Pashta.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Yetiv.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Tevir.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Geresh.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Gershayim.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Pazer.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::PazerGadol.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::TelishaGedolah.category(),AccentCategory::Disjunctive );
    assert_eq!(ProseAccent::Legarmeh.category(),AccentCategory::Disjunctive );
    // Conjunctives
    assert_eq!(ProseAccent::Munach.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Mahpakh.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Merkha.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::MerkhaKephulah.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Darga.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Azla.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::TelishaQetannah.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Galgal.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Mayela.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Meteg.category(),AccentCategory::Conjunctive );
    assert_eq!(ProseAccent::Maqqeph.category(),AccentCategory::Conjunctive );
}


#[test]
fn testing_poetry_accent_categories() {
    // Disjunctives
    assert_eq!(PoetryAccent::Silluq.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::OlehWeYored.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::Atnach.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::ReviaGadol.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::ReviaMugrash.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::ShalsheletGadol.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::Tsinnor.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::ReviaQaton.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::Dechi.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::Pazer.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::MehuppakhLegarmeh.category(),AccentCategory::Disjunctive );
    assert_eq!(PoetryAccent::AzlaLegarmeh.category(),AccentCategory::Disjunctive );
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Merkha.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Illuy.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Tarkha.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Galgal.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Mehuppakh.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Azla.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::ShalsheletQetannah.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::TsinnoritMerkha.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::TsinnoritMahpakh.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Meteg.category(),AccentCategory::Conjunctive );
    assert_eq!(PoetryAccent::Maqqeph.category(),AccentCategory::Conjunctive );
}
