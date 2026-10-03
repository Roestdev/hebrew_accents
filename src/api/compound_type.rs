/// Indicates the compound type of a HebrewAccent (absence is expressed via `Option<T>`)
///
/// when anaccent consists of two cantillation marks it is called a `compound accent`
///
/// Three types are distinguished:
///   **Standard**
///   **CanSpanTwowords**
/// - Oleh WeYored (עוֹלֶה וְיוֹרֵד)
/// - Tsinnorit Merkha (צִנּוֹרִית מֵרכָּא)
/// - Tsinnorit Mahpakh (צִנּוֹרִית מַהְפָּךְ)
///   **ContainsPaseq
///
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CompoundType {
    /// Indicates a normal compound accent
    /// Meaning it consists of two accent marks and sits on one word only
    Standard,
    /// Indicates that the compound accent contains a Paseq
    /// Meaning the second accent mark is a Paseq
    ContainsPaseq,
    /// Indicates that the compound accent can span two words (or one word)
    CanSpanTwoWords,
}
