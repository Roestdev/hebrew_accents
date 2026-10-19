use crate::mark::{MarkPlacement, StressPosition, TraditionNames};

/// Details on a specific cantellation mark
///
/// This is an internal structure combining:
///   - Cantillation Mark (symbol, placement, stress_position)
///   - Naming of the cantillation mark for different traditions
///   - Technical information (code_point_value,hex_bytes,unicode_name)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct MarkMetaData {
    /// The symbol of the cantillation mark
    pub(crate) symbol: char,
    /// The placement of the cantillation mark relative to the consonant
    pub(crate) position: MarkPlacement,
    /// Stress relation (linguistic/metrical) of the cantillation mark
    pub(crate) stress_position: StressPosition,
    /// UTF-8 code-point value, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_bytes: &'static str,
    /// The name of the UTF-8 code-point for the cantillation mark
    pub(crate) unicode_name: &'static str,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,
}
