use crate::accent::STRENGTH_NONE;

/// Converts an internal raw strength value into a user-facing `Option<u8>`.
///
/// The sentinel `STRENGTH_NONE` becomes `None`; any real rank passes through.
#[inline]
pub(crate) const fn resolve_rank(accent_index: u8) -> Option<u8> {
    if accent_index == STRENGTH_NONE {
        None
    } else {
        Some(accent_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_value() {
        assert_eq!(STRENGTH_NONE, 255);
        assert_eq!(STRENGTH_NONE, u8::MAX);
        assert_eq!(resolve_rank(STRENGTH_NONE), None);
    }

    #[test]
    fn all_valid_ranks() {
        for raw in 0..=254u8 {
            assert_eq!(resolve_rank(raw), Some(raw), "rank {raw} corrupted");
        }
    }

    #[test]
    fn test_invalid_rank() {
        let invalid_rank = resolve_rank(255u8);
        assert_eq!(invalid_rank, None, "rank for 255u8 corrupted");
    }
}
