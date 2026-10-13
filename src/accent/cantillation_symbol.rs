use crate::accent_mark::Utf8CodePoint;
use crate::api::{Accent, CompoundType, HebrewAccent};
use crate::CantillationMarkStressPosition;

const DOTTED_CIRCLE: char = '\u{25CC}';
const SPACE: char = '\u{0020}';

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePoint,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePoint>,
}

/// Displays cantillation symbol with dotted circles indicating syllable positions.
///
/// Pattern conventions:
/// - Single accent: `{char}◌` (1 circle)
/// - Compound + twowords: `◌{sec}◌ _ ◌{primary_symbol}◌` (4 circles, spaced)
/// - Compound + paseq: `{sec} ◌{primary_symbol}◌◌` (3 circles)  
/// - Compound + nopaseq: `{sec}◌{primary_symbol}◌◌` (3 circles)
///

pub(crate) fn display_cantillation_symbol(accent: HebrewAccent) -> String {
    //get primary cantillationmark info
    let primary_symbol = accent.primary_cantillation_mark().symbol;
    let primary_stress_pos = accent.primary_cantillation_mark().stress_position.unwrap();
    if accent.compound_type().is_none() {
        // Non-compound: single mark - most common one!
        match primary_stress_pos {
            CantillationMarkStressPosition::Postpositive => {
                return format!(
                    "{}{}{}{}",
                    primary_symbol, DOTTED_CIRCLE, DOTTED_CIRCLE, DOTTED_CIRCLE
                );
            }
            CantillationMarkStressPosition::Impositive => {
                return format!(
                    "{}{}{}{}",
                    DOTTED_CIRCLE, primary_symbol, DOTTED_CIRCLE, DOTTED_CIRCLE
                );
            }
            CantillationMarkStressPosition::Prepositive => {
                return format!(
                    "{}{}{}{}",
                    DOTTED_CIRCLE, DOTTED_CIRCLE, primary_symbol, DOTTED_CIRCLE
                );
            }
        }
    } else {
        // found compound accent - only 11 present
        // get secondary cantillationmark info
        let secondary_symbol = accent.secondary_cantillation_mark().expect("//TODO").symbol;

        let compound_type = accent.compound_type().unwrap();
        match compound_type {
            CompoundType::Standard => {
                // Compound (two marks, one word, no Paseq)
                // three dotted circles
                format!(
                    "{}{}{}{}{}",
                    DOTTED_CIRCLE, secondary_symbol, DOTTED_CIRCLE, primary_symbol, DOTTED_CIRCLE
                )
            }
            CompoundType::ContainsPaseq => {
                // Position of the primary mark dependant of its stress-position
                // All applicable primary marks are 'Impositive'
                // Position of the secondary mark (=Paseq) is always after the word
                return format!(
                    "{}{}{}{}{}{}",
                    secondary_symbol,
                    SPACE,
                    DOTTED_CIRCLE,
                    primary_symbol,
                    DOTTED_CIRCLE,
                    DOTTED_CIRCLE
                );
            }
            CompoundType::CanSpanTwoWords => {
                // all applicable marks are 'Impositive'
                return format!(
                    "{}{}{}{}{}{}{}{}{}",
                    DOTTED_CIRCLE,
                    secondary_symbol,
                    DOTTED_CIRCLE,
                    DOTTED_CIRCLE,
                    SPACE,
                    DOTTED_CIRCLE,
                    primary_symbol,
                    DOTTED_CIRCLE,
                    DOTTED_CIRCLE
                );
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HebrewAccent, PoetryAccent, ProseAccent};

    #[test]
    fn test_twowords_has_four_circles_and_space() {
        let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
        let result = display_cantillation_symbol(accent);

        assert_eq!(result.matches(DOTTED_CIRCLE).count(), 4);
        assert!(result.contains(SPACE));
    }

    #[test]
    fn test_paseq_has_three_circles_with_space() {
        let accent = HebrewAccent::Prose(ProseAccent::Shalshelet);
        let result = display_cantillation_symbol(accent);

        assert_eq!(result.matches(DOTTED_CIRCLE).count(), 2);
        assert!(result.contains(SPACE));
    }

    #[test]
    fn test_single_accent_has_one_circle() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let result = display_cantillation_symbol(accent);

        assert_eq!(result.matches(DOTTED_CIRCLE).count(), 1);
        assert!(!result.contains(SPACE));
    }
}
