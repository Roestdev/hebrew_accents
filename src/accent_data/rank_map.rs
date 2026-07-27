use crate::{PoetryAccent, ProseAccent};

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variantextend this
/// array accordingly – the `static_assertions` check below will remind you.
///
/// Scholarly strength ranking for prose accents (Futato classification)
/// Lower values = stronger disjunctive function (longer pause)
/// Values 1-18: disjunctive accents ranked by hierarchical strength
/// Values 255+: conjunctive and pseudo-accents (no disjunctive function)
///
/// Index corresponds to ProseAccent enum variant order (discriminant)
pub(crate) const BHS_PROSE_RANK_MAP: [u8; ProseAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES ===
    1,  // 0:  Silluq
    2,  // 1:  Atnach
    3,  // 2:  Segolta
    4,  // 3:  Shalshelet
    5,  // 4:  ZaqephQatan
    5,  // 5:  ZaqephGadol
    6,  // 6:  Revia
    7,  // 7:  Tiphcha
    8,  // 8:  Zarqa
    8,  // 9:  Pashta
    9,  // 10: Yetiv
    10, // 11: Tevir
    11, // 12: Geresh
    11, // 13: Gershayim
    12, // 14: Pazer
    13, // 15: PazerGadol
    14, // 16: TelishaGedolah
    14, // 17: Legarmeh
    // === CONJUNCTIVES ===
    255, // 18: Munach
    255, // 19: Mahpakh
    255, // 20: Merkha
    255, // 21: MerkhaKephulah
    255, // 22: Darga
    255, // 23: Azla
    255, // 24: TelishaQetannah
    255, // 25: Galgal
    255, // 26: Meayla
    255, // 27: Meteg
];

/// Scholarly strength ranking for poetry accents (BHS / Futato classification)
/// Lower values = stronger disjunctive function (longer pause)
/// Values 1-12: disjunctive accents ranked by hierarchical strength
/// Value 255: conjunctive accents (no disjunctive function)
///
/// Index corresponds to PoetryAccent enum variant order (discriminant)
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES  ===
    1,  // 0:  Silluq
    2,  // 1:  OlehWeYored
    3,  // 2:  Atnach
    4,  // 3:  ReviaGadol
    5,  // 4:  ReviaMugrash
    6,  // 5:  ShalsheletGadol
    6,  // 6:  Tsinnor
    7,  // 7:  ReviaQaton
    8,  // 8:  Dechi
    9,  // 9:  Pazer
    9,  // 10: MehuppakhLegarmeh
    10, // 11: AzlaLegarmeh
    // === CONJUNCTIVES ===
    255, // 12: Munach
    255, // 13: Merkha
    255, // 14: Illuy
    255, // 15: Tarcha
    255, // 16: Galgal
    255, // 17: Mehuppakh
    255, // 18: Azla
    255, // 19: ShalsheletQetannah
    255, // 20: TsinnoritMerkha
    255, // 21: TsinnoritMahpakh
    255, // 22: Meteg
];
