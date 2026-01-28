use hebrew_accents::{Accent, AccentType, PoetryAccent, ProseAccent, PseudoAccent};

#[test]
fn testing_prose_accent_types() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Atnach.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::Segolta.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        ProseAccent::Shalshelet.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        ProseAccent::ZaqephQaton.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        ProseAccent::ZaqephGadol.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Revia.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::Tiphcha.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Zarqa.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Pashta.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Yetiv.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Tevir.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Geresh.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::Gershayim.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Pazer.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::PazerGadol.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        ProseAccent::TelishaGedolah.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        ProseAccent::Legarmeh.accent_type(),
        Some(AccentType::Primary)
    );
    // Conjunctives
    assert_eq!(ProseAccent::Munach.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::Mahpakh.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Merkha.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::MerkhaKephulah.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Darga.accent_type(), Some(AccentType::Primary));
    assert_eq!(ProseAccent::Azla.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::TelishaQetannah.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(ProseAccent::Galgal.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        ProseAccent::Mayela.accent_type(),
        Some(AccentType::Secondary)
    );
    assert_eq!(
        ProseAccent::Meteg.accent_type(),
        Some(AccentType::Secondary)
    );
}

#[test]
fn testing_poetry_accent_types() {
    // Disjunctives
    assert_eq!(
        PoetryAccent::Silluq.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::OlehWeYored.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Atnach.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::ReviaGadol.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::ReviaMugrash.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::ShalsheletGadol.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Tsinnor.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::ReviaQaton.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(PoetryAccent::Dechi.accent_type(), Some(AccentType::Primary));
    assert_eq!(PoetryAccent::Pazer.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        PoetryAccent::MehuppakhLegarmeh.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::AzlaLegarmeh.accent_type(),
        Some(AccentType::Primary)
    );
    // Conjunctives
    assert_eq!(
        PoetryAccent::Munach.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Merkha.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(PoetryAccent::Illuy.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        PoetryAccent::Tarkha.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Galgal.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Mehuppakh.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(PoetryAccent::Azla.accent_type(), Some(AccentType::Primary));
    assert_eq!(
        PoetryAccent::ShalsheletQetannah.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::TsinnoritMerkha.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::TsinnoritMahpakh.accent_type(),
        Some(AccentType::Primary)
    );
    assert_eq!(
        PoetryAccent::Meteg.accent_type(),
        Some(AccentType::Secondary)
    );
}

#[test]
fn testing_pseudo_accent_types() {
    assert_eq!(PseudoAccent::SophPasuq.accent_type(), None);
    assert_eq!(PseudoAccent::Maqqeph.accent_type(), None);
    assert_eq!(PseudoAccent::Paseq.accent_type(), None);
}
