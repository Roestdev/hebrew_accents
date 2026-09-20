use crate::accent_data::STRENGTH_NONE;

/// Converts an internal raw strength value into a user-facing `Option<u8>`.
///
/// The sentinel `STRENGTH_NONE` becomes `None`; any real rank passes through.
#[inline]
pub(crate) const fn resolve_relative_strength(accent_index: u8) -> Option<u8> {
    if accent_index == STRENGTH_NONE {
        None
    } else {
        Some(accent_index)
    }
}