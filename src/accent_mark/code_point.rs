use crate::accent_mark::{CodePointPosition, StressPosition};

/// Details on a specific UTF-8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Utf8CodePoint {
    /// The symbol of the UTF-8 code-point
    pub(crate) symbol: char,
    /// The position of the code-point in relation to the consonant
    pub(crate) position: CodePointPosition,
    /// Stress relation (linguistic/metrical)
    pub(crate) stress_position: StressPosition,
    /// UTF-8 code-point id, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_bytes: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub(crate) canonical_name: &'static str,
}
