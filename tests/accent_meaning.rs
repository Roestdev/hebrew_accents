use hebrew_accents::{Accent, PoetryAccent, ProseAccent, PseudoAccent};

#[test]
fn testing_prose_accent_meaning() {
    // Disjunctives
    assert_eq!(ProseAccent::Silluq.meaning(), "close, cessation");
    assert_eq!(ProseAccent::Atnach.meaning(), "a causing to rest");
    assert_eq!(ProseAccent::Segolta.meaning(), "a little grape-bunch");
    assert_eq!(ProseAccent::Shalshelet.meaning(), "chain or link");
    assert_eq!(ProseAccent::ZaqephQaton.meaning(), "small upright");
    assert_eq!(ProseAccent::ZaqephGadol.meaning(), "large upright");
    assert_eq!(ProseAccent::Revia.meaning(), "fourth [in a sequence]");
    assert_eq!(ProseAccent::Tiphcha.meaning(), "handbreadth or diagonal");
    assert_eq!(ProseAccent::Zarqa.meaning(), "to sprinkle, scatter");
    assert_eq!(
        ProseAccent::Pashta.meaning(),
        "extending, stretching out in length"
    );
    assert_eq!(ProseAccent::Yetiv.meaning(), "resting or sitting");
    assert_eq!(ProseAccent::Tevir.meaning(), "broken, downward tumble");
    assert_eq!(
        ProseAccent::Geresh.meaning(),
        "expulsion, driving out, divorce"
    );
    assert_eq!(
        ProseAccent::Gershayim.meaning(),
        "double of expulsion, driving out, divorce"
    );
    assert_eq!(ProseAccent::Pazer.meaning(), "lavish or scatter");
    assert_eq!(ProseAccent::PazerGadol.meaning(), "large lavish or scatter");
    assert_eq!(
        ProseAccent::TelishaGedolah.meaning(),
        "great (long) detached"
    );
    assert_eq!(
        ProseAccent::Legarmeh.meaning(),
        "for or by itself, independant"
    );
    // Conjunctives
    assert_eq!(ProseAccent::Munach.meaning(), "resting or placed");
    assert_eq!(ProseAccent::Mahpakh.meaning(), "turning round");
    assert_eq!(ProseAccent::Merkha.meaning(), "lengthener, prolonging");
    assert_eq!(ProseAccent::MerkhaKephulah.meaning(), "double lengthener");
    assert_eq!(ProseAccent::Darga.meaning(), "stairstep");
    assert_eq!(
        ProseAccent::Azla.meaning(),
        "going on (not pausing), depart"
    );
    assert_eq!(
        ProseAccent::TelishaQetannah.meaning(),
        "small (short) detached"
    );
    assert_eq!(ProseAccent::Galgal.meaning(), "wheel, circle");
    assert_eq!(ProseAccent::Mayela.meaning(), "to be raised or elevated");
    assert_eq!(ProseAccent::Meteg.meaning(), "accent or mark");
}

#[test]
fn testing_poetry_accent_meaning() {
    // Disjunctives
    assert_eq!(PoetryAccent::Silluq.meaning(), "close, cessation");
    assert_eq!(
        PoetryAccent::OlehWeYored.meaning(),
        "ascending and descending"
    );
    assert_eq!(PoetryAccent::Atnach.meaning(), "a causing to rest");
    assert_eq!(PoetryAccent::ReviaGadol.meaning(), "big fourth");
    assert_eq!(PoetryAccent::ReviaMugrash.meaning(), "exiled fourth");
    assert_eq!(
        PoetryAccent::ShalsheletGadol.meaning(),
        "large chain or link"
    );
    assert_eq!(PoetryAccent::Tsinnor.meaning(), "pipe or tube");
    assert_eq!(PoetryAccent::ReviaQaton.meaning(), "small fourth");
    assert_eq!(PoetryAccent::Dechi.meaning(), "to push or drive away");
    assert_eq!(PoetryAccent::Pazer.meaning(), "lavish or scatter");
    assert_eq!(
        PoetryAccent::MehuppakhLegarmeh.meaning(),
        "reversed to its own"
    );
    assert_eq!(PoetryAccent::AzlaLegarmeh.meaning(), "goes to its own");
    // Conjunctives
    assert_eq!(PoetryAccent::Munach.meaning(), "resting or placed");
    assert_eq!(PoetryAccent::Merkha.meaning(), "lengthener, prolonging");
    assert_eq!(PoetryAccent::Illuy.meaning(), "elevation or raising");
    assert_eq!(PoetryAccent::Tarcha.meaning(), "handbreadth or diagonal");
    assert_eq!(PoetryAccent::Galgal.meaning(), "wheel, circle");
    assert_eq!(PoetryAccent::Mehuppakh.meaning(), "reversed");
    assert_eq!(
        PoetryAccent::Azla.meaning(),
        "going on (not pausing), depart"
    );
    assert_eq!(PoetryAccent::ShalsheletQetannah.meaning(), "small chain");
    assert_eq!(
        PoetryAccent::TsinnoritMerkha.meaning(),
        "pipe of continuation"
    );
    assert_eq!(PoetryAccent::TsinnoritMahpakh.meaning(), "pipe of reversal");
    assert_eq!(PoetryAccent::Meteg.meaning(), "accent or mark");
}

#[test]
fn testing_pseudo_accent_meaning() {
    assert_eq!(PseudoAccent::SophPasuq.meaning(), "end of the verse");
    assert_eq!(PseudoAccent::Maqqeph.meaning(), "binder");
    assert_eq!(PseudoAccent::Paseq.meaning(), "binder");
}
