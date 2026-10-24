use crate::api::{Accent, CantillationMarkPlacement, CompoundType, HebrewAccent};
use crate::mark::MarkMetaData;

const SPACE: char = '\u{0020}';
const ALEPH: char = 'א';
const MEM: char = 'ם';
const TAV: char = 'ת';
const PASEQ: char = '\u{05C0}';

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static MarkMetaData,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static MarkMetaData>,
}

// TODO text
/// Displays cantillation symbol
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
    let primary_placement = accent.primary_cantillation_mark().placement;
    if accent.compound_type().is_none() {
        // Non-compound: single mark - most common one!
        match primary_placement {
            CantillationMarkPlacement::AboveLeft => {
                // always on the last letter
                format!("{}{}{}{}", ALEPH, MEM, TAV, primary_symbol)
            }
            CantillationMarkPlacement::AboveCenter | CantillationMarkPlacement::BelowCenter => {
                format!("{}{}{}{}", ALEPH, MEM, primary_symbol, TAV)
            }
            CantillationMarkPlacement::AboveRight | CantillationMarkPlacement::BelowRight => {
                //always on the first letter
                format!("{}{}{}{}", ALEPH, primary_symbol, MEM, TAV)
            }
            CantillationMarkPlacement::SofPasuq | CantillationMarkPlacement::Paseq => {
                format!("{}{}{}{}{}", ALEPH, MEM, TAV, SPACE, primary_symbol)
            }
            CantillationMarkPlacement::Maqqaph => {
                format!(
                    "{}{}{}{}{}{}{}",
                    ALEPH, MEM, TAV, primary_symbol, ALEPH, MEM, TAV
                )
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
                    ALEPH, primary_symbol, MEM, secondary_symbol, TAV
                )
            }
            CompoundType::ContainsPaseq => {
                // Position of the primary mark dependant of its stress-position
                // All applicable primary marks are 'Impositive'
                // Position of the secondary mark (=Paseq) is always after the word
                format!(
                    "{}{}{}{}{}{}",
                    ALEPH, MEM, primary_symbol, TAV, SPACE, PASEQ
                )
            }
            CompoundType::CanSpanTwoWords => {
                // all applicable marks are 'Impositive'
                format!(
                    "{}{}{}{}{}{}{}{}{}",
                    ALEPH, MEM, primary_symbol, TAV, SPACE, ALEPH, MEM, secondary_symbol, TAV
                )
            }
        }
    }
}
#[cfg(test)]
mod tests {}
