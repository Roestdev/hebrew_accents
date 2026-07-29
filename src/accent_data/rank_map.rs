use crate::api::{PoetryAccent, ProseAccent};

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variantextend this
/// array accordingly – the `static_assertions` check below will remind you.
///
/// Index corresponds to ProseAccent enum variant order (discriminant)
pub(crate) const BHS_PROSE_RANK_MAP: [u8; ProseAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES ===
    1,  // 0:  Silluq
    2,  // 1:  Atnach
    3,  // 2:  Segolta
    4,  // 3:  Shalshelet
    5,  // 4:  ZaqephQatan TODO Zaqeph Qaton (Little Zaqeph)
    6,  // 5:  ZaqephGadol
    7,  // 6:  Revia
    8,  // 7:  Tiphcha
    9,  // 8:  Zarqa
    10, // 9:  Pashta
    11, // 10: Yetiv
    12, // 11: Tevir
    13, // 12: Geresh
    14, // 13: Gershayim
    15, // 14: Pazer
    16, // 15: PazerGadol
    17, // 16: TelishaGedolah
    18, // 17: Legarmeh
    // === CONJUNCTIVES ===
    19, // 18: Munach
    20, // 19: Mahpakh
    21, // 20: Merkha
    22, // 21: MerkhaKephulah
    23, // 22: Darga
    24, // 23: Azla
    25, // 24: TelishaQetannah
    26, // 25: Galgal
    27, // 26: Meayla
    28, // 27: Meteg
];

/// Index corresponds to PoetryAccent enum variant order (discriminant)
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES  ===
    1,  // 0:  Silluq
    2,  // 1:  OlehWeYored
    3,  // 2:  Atnach
    4,  // 3:  ReviaGadol
    5,  // 4:  ReviaMugrash
    6,  // 5:  ShalsheletGadol
    7,  // 6:  Tsinnor
    8,  // 7:  ReviaQaton
    9,  // 8:  Dechi
    10, // 9:  Pazer
    11, // 10: MehuppakhLegarmeh
    12, // 11: AzlaLegarmeh
    // === CONJUNCTIVES ===
    13, // 12: Munach
    14, // 13: Merkha
    15, // 14: Illuy
    16, // 15: Tarcha
    17, // 16: Galgal
    18, // 17: Mehuppakh
    19, // 18: Azla
    20, // 19: ShalsheletQetannah
    21, // 20: TsinnoritMerkha
    22, // 21: TsinnoritMahpakh
    23, // 22: Meteg
];
