use crate::{Accent, CompoundType, HebrewAccent};

const DOTTED_CIRCLE: char = '\u{25CC}';
const SPACE: char = '\u{0020}';

/// Displays cantillation symbol with dotted circles indicating syllable positions.
///
/// Pattern conventions:
/// - Single accent: `{char}◌` (1 circle)
/// - Compound + twowords: `◌{sec}◌ _ ◌{pri}◌` (4 circles, spaced)
/// - Compound + paseq: `{sec} ◌{pri}◌◌` (3 circles)  
/// - Compound + nopaseq: `{sec}◌{pri}◌◌` (3 circles)
///
/// Note: Circle count reflects word boundary markers, not mark count.
pub(crate) fn display_cantillation_symbol(accent: HebrewAccent) -> String {
    let pri = accent.primary_cantillation_mark().symbol;

    match accent.compound_type() {
        None => {
            // Non-compound: single mark
            format!("{}{}", pri, DOTTED_CIRCLE)
        }
        Some(CompoundType::Standard) => {
            // Compound (two marks, one word, no Paseq)
            let sec = accent
                .secondary_cantillation_mark()
                .expect("Standard compound must have secondary mark")
                .symbol;
            format!(
                "{}{}{}{}{}",
                sec, DOTTED_CIRCLE, pri, DOTTED_CIRCLE, DOTTED_CIRCLE
            )
        }
        Some(CompoundType::ContainsPaseq) => {
            let sec = accent
                .secondary_cantillation_mark()
                .expect("Paseq compound must have secondary mark")
                .symbol;
            format!("{}{}{}{}{}", sec, SPACE, DOTTED_CIRCLE, pri, DOTTED_CIRCLE)
        }
        Some(CompoundType::CanSpanTwoWords) => {
            let sec = accent
                .secondary_cantillation_mark()
                .expect("Twoword must have secondary mark")
                .symbol;
            format!(
                "{}{}{} {}{}{}",
                DOTTED_CIRCLE, sec, DOTTED_CIRCLE, DOTTED_CIRCLE, pri, DOTTED_CIRCLE
            )
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
