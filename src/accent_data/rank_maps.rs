use crate::api::{PoetryAccent, ProseAccent};

/// Sentinel value indicating "no strength hierarchy".
///
/// Stored internally in the rank maps as `u8::MAX` and converted back to
/// `None` by [`relative_strength`](crate::accent::Accent::relative_strength),
/// so users never see this value.
pub(crate) const STRENGTH_NONE: u8 = u8::MAX;

/// Index corresponds to ProseAccent enum variant order (discriminant)
pub(crate) const BHS_PROSE_RANK_MAP: [u8; ProseAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES (strength 1–18) ===
    1,  // 0:  Silluq
    2,  // 1:  Atnach
    3,  // 2:  Segolta
    4,  // 3:  Shalshelet
    5,  // 4:  ZaqephQatan
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
    // === CONJUNCTIVES (sentinel = u8::MAX) ===
    STRENGTH_NONE, // 18: Munach
    STRENGTH_NONE, // 19: Mahpakh
    STRENGTH_NONE, // 20: Merkha
    STRENGTH_NONE, // 21: MerkhaKephulah
    STRENGTH_NONE, // 22: Darga
    STRENGTH_NONE, // 23: Azla
    STRENGTH_NONE, // 24: TelishaQetannah
    STRENGTH_NONE, // 25: Galgal
    STRENGTH_NONE, // 26: Meayla
    STRENGTH_NONE, // 27: Meteg
];

/// Index corresponds to PoetryAccent enum variant order (discriminant)
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::LEN] = [
    // === PRIMARY DISJUNCTIVES (strength 1–12) ===
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
    // === CONJUNCTIVES (sentinel = u8::MAX) ===
    STRENGTH_NONE, // 12: Munach
    STRENGTH_NONE, // 13: Merkha
    STRENGTH_NONE, // 14: Illuy
    STRENGTH_NONE, // 15: Tarcha
    STRENGTH_NONE, // 16: Galgal
    STRENGTH_NONE, // 17: Mehuppakh
    STRENGTH_NONE, // 18: Azla
    STRENGTH_NONE, // 19: ShalsheletQetannah
    STRENGTH_NONE, // 20: TsinnoritMerkha
    STRENGTH_NONE, // 21: TsinnoritMahpakh
    STRENGTH_NONE, // 22: Meteg
];

// // ── Compile-time guards ────────────────────────────────────────────────

// /// Ensures no valid strength rank collides with the sentinel.
// /// Valid ranks are 1–23; `u8::MAX` (255) is reserved.
// const _: () = assert!(
//     BHS_PROSE_RANK_MAP.iter().all(|&v| v == STRENGTH_NONE || (1..=23).contains(&v)),
//     "Prose rank map contains out-of-range strengths"
// );

// const _: () = assert!(
//     BHS_POETRY_RANK_MAP.iter().all(|&v| v == STRENGTH_NONE || (1..=23).contains(&v)),
//     "Poetry rank map contains out-of-range strengths"
// );


