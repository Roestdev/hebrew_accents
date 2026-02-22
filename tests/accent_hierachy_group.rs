use hebrew_accents::{Accent, HierarchicalGroup, PoetryAccent, ProseAccent, PseudoAccent};

#[test]
fn testing_prose_accent_hierarchical_group() {
    // Disjunctives
    assert_eq!(
        ProseAccent::Silluq.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup1)
    );
    assert_eq!(
        ProseAccent::Atnach.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup1)
    );
    assert_eq!(
        ProseAccent::Segolta.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup2)
    );
    assert_eq!(
        ProseAccent::Shalshelet.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup2)
    );
    assert_eq!(
        ProseAccent::ZaqephQatan.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup2)
    );
    assert_eq!(
        ProseAccent::ZaqephGadol.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup2)
    );
    assert_eq!(
        ProseAccent::Revia.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup3)
    );
    assert_eq!(
        ProseAccent::Tiphcha.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup2)
    );
    assert_eq!(
        ProseAccent::Zarqa.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup3)
    );
    assert_eq!(
        ProseAccent::Pashta.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup3)
    );
    assert_eq!(
        ProseAccent::Yetiv.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup3)
    );
    assert_eq!(
        ProseAccent::Tevir.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup3)
    );
    assert_eq!(
        ProseAccent::Geresh.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    assert_eq!(
        ProseAccent::Gershayim.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    assert_eq!(
        ProseAccent::Pazer.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    assert_eq!(
        ProseAccent::PazerGadol.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    assert_eq!(
        ProseAccent::TelishaGedolah.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    assert_eq!(
        ProseAccent::Legarmeh.hierarchical_group(),
        Some(HierarchicalGroup::ProseGroup4)
    );
    // Conjunctives
    assert_eq!(ProseAccent::Munach.hierarchical_group(), None);
    assert_eq!(ProseAccent::Mahpakh.hierarchical_group(), None);
    assert_eq!(ProseAccent::Merkha.hierarchical_group(), None);
    assert_eq!(ProseAccent::MerkhaKephulah.hierarchical_group(), None);
    assert_eq!(ProseAccent::Darga.hierarchical_group(), None);
    assert_eq!(ProseAccent::Azla.hierarchical_group(), None);
    assert_eq!(ProseAccent::TelishaQetannah.hierarchical_group(), None);
    assert_eq!(ProseAccent::Galgal.hierarchical_group(), None);
    assert_eq!(ProseAccent::Mayela.hierarchical_group(), None);
    assert_eq!(ProseAccent::Meteg.hierarchical_group(), None);
}

#[test]
fn testing_poetry_accent_hierarchical_group() {
    // Disjunctives
    assert_eq!(
        PoetryAccent::Silluq.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup1)
    );
    assert_eq!(
        PoetryAccent::OlehWeYored.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup1)
    );
    assert_eq!(
        PoetryAccent::Atnach.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup1)
    );
    assert_eq!(
        PoetryAccent::ReviaGadol.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::ReviaMugrash.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::ShalsheletGadol.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::Tsinnor.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::ReviaQaton.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::Dechi.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup2)
    );
    assert_eq!(
        PoetryAccent::Pazer.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup3)
    );
    assert_eq!(
        PoetryAccent::MehuppakhLegarmeh.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup3)
    );
    assert_eq!(
        PoetryAccent::AzlaLegarmeh.hierarchical_group(),
        Some(HierarchicalGroup::PoetryGroup3)
    );
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Merkha.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Illuy.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Tarcha.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Galgal.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Mehuppakh.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Azla.hierarchical_group(), None);
    assert_eq!(PoetryAccent::ShalsheletQetannah.hierarchical_group(), None);
    assert_eq!(PoetryAccent::TsinnoritMerkha.hierarchical_group(), None);
    assert_eq!(PoetryAccent::TsinnoritMahpakh.hierarchical_group(), None);
    assert_eq!(PoetryAccent::Meteg.hierarchical_group(), None);
}

#[test]
fn testing_pseudo_accent_hierarchical_group() {
    assert_eq!(PseudoAccent::SophPasuq.hierarchical_group(), None);
    assert_eq!(PseudoAccent::Maqqeph.hierarchical_group(), None);
    assert_eq!(PseudoAccent::Paseq.hierarchical_group(), None);
}
