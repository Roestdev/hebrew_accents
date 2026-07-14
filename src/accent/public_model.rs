use crate::codepoints::CodePointPosition;


/// Hebrew Accent kind — (absence is expressed via Option<T>)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentKind {
    #[default]
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

/// Hebrew Accent category — (absence is expressed via Option<T>)
///
/// # Examples
///
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, AccentCategory};
///
/// // Disjunctive accents return Some(disjunctive)
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.category(), Some(AccentCategory::Disjunctive));
///
/// // Conjunctive accents return Some(conjunctive)
/// let munach: HebrewAccent = ProseAccent::Munach.into();
/// assert_eq!(munach.category(), Some(AccentCategory::Conjunctive));
/// ```
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentCategory {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
}

/// Hebrew Accent wordstress — (absence is expressed via Option<T>)
///
/// 'WordStress', indicating the location of the accent in relation to the consonant
///
/// # Examples
///
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, AccentWordStress};
///
/// // Check where the stress is located
/// let accent = HebrewAccent::Prose(ProseAccent::Silluq);
///
/// match accent.word_stress() {
///     Some(AccentWordStress::ImPositive) => {
///         println!("Stress is on the syllable");
///     }
///     Some(AccentWordStress::PostPositive) => {
///         println!("Stress is at word end");
///     }
///     Some(AccentWordStress::PrePositive) => {
///         println!("Stress is at word beginning");
///     }
///     None => {
///         println!("No word stress information");
///     }
/// }
/// ```
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AccentWordStress {
    #[default]
    /// ImPositive: The accent is located above the stressed syllable
    ImPositive,
    /// PostPositive: The accent is NOT located above the stressed syllable, but at the very end of the word
    PostPositive,
    /// PrePositive: Accent is NOT located above the stressed syllable, but at the very beginning of the word
    PrePositive,
}

/// Public-facing representation of a cantillation codepoint
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct CantillationMark {
    /// Unicode codepoint value, e.g. "U+0597"
    pub unicode_value: &'static str,
    /// Hex byte representation, e.g. "0xd6 0x97"
    pub hex_bytes: &'static str,
    /// The actual Hebrew character/symbol, e.g. "֗"
    pub symbol: &'static str,
    /// Canonical name from UTF-8 character tables
    pub canonical_name: &'static str,
    /// Position relative to the consonant
    pub position: CodePointPosition,
}

/// Disjunctive accent hierarchy level following Futato's classification system.
///
/// Ranges from 1 (strongest pause/break) to higher numbers (weaker pauses).
/// Conjunctive accents and pseudo-accent markers return `None` as they lack
/// hierarchical disjunctive function.
///
/// # Example
/// ```
/// use hebrew_accents::{Accent, HebrewAccent, ProseAccent, GroupLevel};
///
/// let silluq = HebrewAccent::Prose(ProseAccent::Silluq);
/// assert_eq!(silluq.group_level(), Some(GroupLevel::Level1));
///
/// let conjunctive = HebrewAccent::Prose(ProseAccent::Munach);
/// assert_eq!(conjunctive.group_level(), None);
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum GroupLevel {
    /// Primary disjunctive tier — creates major clause/phrasal breaks
    Level1 = 1, // value represents group number for extension in future
    /// Secondary disjunctive tier — subordinate phrase boundaries
    Level2,
    /// Tertiary disjunctive tier — minor phrasal divisions
    Level3,
    /// Quaternary disjunctive tier — fine-grained subdivisions
    Level4,
}

impl GroupLevel {
    /// Raw numeric strength value (1 = strongest disjunctive)
    pub const fn value(self) -> u8 {
        match self {
            Self::Level1 => 1,
            Self::Level2 => 2,
            Self::Level3 => 3,
            Self::Level4 => 4,
        }
    }

    /// Human-readable description of hierarchy tier
    pub const fn description(self) -> &'static str {
        match self {
            Self::Level1 => "Primary disjunctive (major clause break)",
            Self::Level2 => "Secondary disjunctive (phrase boundary)",
            Self::Level3 => "Tertiary disjunctive (minor division)",
            Self::Level4 => "Quaternary disjunctive (fine subdivision)",
        }
    }
}

impl std::fmt::Display for GroupLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Level1 => write!(f, "Tier 1 (Primary disjunctive)"),
            Self::Level2 => write!(f, "Tier 2 (Secondary disjunctive)"),
            Self::Level3 => write!(f, "Tier 3 (Tertiary disjunctive)"),
            Self::Level4 => write!(f, "Tier 4 (Quaternary disjunctive)"),
        }
    }
}
