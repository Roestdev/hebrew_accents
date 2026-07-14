use crate::accent::PoetryAccent;

/// Mapping from the enum discriminant (as `usize`) to the logical relative_strength.
///
/// The order **must** correspond exactly to the order of the variants
/// declared in `PoetryAccent`.  If you add a new variantextend this
/// array accordingly – the `static_assertions` check below will remind you.
pub(crate) const BHS_POETRY_RANK_MAP: [u8; PoetryAccent::LEN] = [
    // Disjunctives
    /* 0 */ 1, // Silluq
    /* 1 */ 2, // OlehWeYored
    /* 2 */ 3, // Atnach
    /* 3 */ 4, // ReviaGadol
    /* 4 */ 5, // ReviaMugrash
    /* 5 */ 6, // ShalsheletGadol
    /* 6 */ 7, // Tsinnor
    /* 7 */ 8, // ReviaQaton
    /* 8 */ 9, // Dechi
    /* 9 */ 10, // Pazer
    /*10 */ 11, // MehuppakhLegarmeh
    /*11 */ 12, // AzlaLegarmeh
    // Conjunctives
    /*12 */ 13, // Munach
    /*13 */ 14, // Merkha
    /*14 */ 15, // Illuy
    /*15 */ 16, // Tarcha
    /*16 */ 17, // Galgal
    /*17 */ 18, // Mehuppakh
    /*18 */ 19, // Azla
    /*19 */ 20, // ShalsheletQetannah
    /*20 */ 21, // TsinnoritMerkha - same rank as TsinnoritMahpakh
    /*21 */ 21, // TsinnoritMahpakh - same rank as TsinnoritMerkha
    /*22 */ 22, // Meteg
];
