use crate::accent::resolve_disjunctive_group;
use crate::accent::AccentCategory;
use crate::accent::AccentType;
use crate::accent::GroupLevel;
use crate::accent::WordStress;
use crate::accent::{HebrewAccent, PoetryAccent, ProseAccent, PseudoAccent};
use crate::data::{
    BHS_POETRY_RANK_MAP, POETRY_ACCENT_TABLE, PROSE_ACCENT_TABLE, PSEUDO_ACCENT_TABLE,
};

/// Used for retrieving information
pub trait Accent: Copy + Sized {
    /// Hebrew name of the Hebrew Accent
    fn hebrew_name(self) -> &'static str;
    /// hebrew_concept of the Hebrew name
    fn hebrew_concept(self) -> &'static str;
    /// English name of the Hebrew Accent
    fn english_name(self) -> &'static str;
    /// Hebrew Accent type
    fn accent_type(self) -> Option<AccentType>;
    /// category of the Hebrew Accent
    fn category(self) -> Option<AccentCategory>;
    /// word-stress of the Hebrew Accent
    fn word_stress(self) -> Option<WordStress>;
    /// number of UTF-8 code points of the Hebrew Accent
    fn number_of_symbols(self) -> u8;
    /// Returns any accent_meta_data notes or context about this accent, if available.
    fn notes(self) -> Option<&'static str>;
    /// Indicates the relative strength where 1 represents the strongest accent.
    fn relative_strength(self) -> u8;
    /// indicates the relative_strength of a selected accent (1 is the strongest)
    fn group_level(self) -> Option<GroupLevel>;
}

impl Accent for HebrewAccent {
    fn hebrew_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_name(),
            HebrewAccent::Poetry(p) => p.hebrew_name(),
            HebrewAccent::Pseudo(p) => p.hebrew_name(),
        }
    }

    fn hebrew_concept(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.hebrew_concept(),
            HebrewAccent::Poetry(p) => p.hebrew_concept(),
            HebrewAccent::Pseudo(p) => p.hebrew_concept(),
        }
    }

    fn english_name(self) -> &'static str {
        match self {
            HebrewAccent::Prose(p) => p.english_name(),
            HebrewAccent::Poetry(p) => p.english_name(),
            HebrewAccent::Pseudo(p) => p.english_name(),
        }
    }

    fn accent_type(self) -> Option<AccentType> {
        match self {
            HebrewAccent::Prose(p) => p.accent_type(),
            HebrewAccent::Poetry(p) => p.accent_type(),
            HebrewAccent::Pseudo(p) => p.accent_type(),
        }
    }

    fn category(self) -> Option<AccentCategory> {
        match self {
            HebrewAccent::Prose(p) => p.category(),
            HebrewAccent::Poetry(p) => p.category(),
            HebrewAccent::Pseudo(p) => p.category(),
        }
    }

    fn word_stress(self) -> Option<WordStress> {
        match self {
            HebrewAccent::Prose(p) => p.word_stress(),
            HebrewAccent::Poetry(p) => p.word_stress(),
            HebrewAccent::Pseudo(p) => p.word_stress(),
        }
    }

    fn number_of_symbols(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.number_of_symbols(),
            HebrewAccent::Poetry(p) => p.number_of_symbols(),
            HebrewAccent::Pseudo(p) => p.number_of_symbols(),
        }
    }
    fn notes(self) -> Option<&'static str> {
        match self {
            HebrewAccent::Prose(p) => p.notes(),
            HebrewAccent::Poetry(p) => p.notes(),
            HebrewAccent::Pseudo(p) => p.notes(),
        }
    }
    // #[inline]
    fn relative_strength(self) -> u8 {
        match self {
            HebrewAccent::Prose(p) => p.relative_strength(),
            HebrewAccent::Poetry(p) => p.relative_strength(),
            HebrewAccent::Pseudo(p) => p.relative_strength(),
        }
    }

    // #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self).and_then(|g| g.into_public_level())
    }
}

impl Accent for ProseAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn accent_type(self) -> Option<AccentType> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.accent_type)
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.category)
    }

    #[inline]
    fn word_stress(self) -> Option<WordStress> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.word_stress)
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        PROSE_ACCENT_TABLE.get(self as usize).map_or(1, |x| {
            if x.cantillation_symbol.secondary_mark.is_some() {
                2
            } else {
                1
            }
        })
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
}

impl Accent for PoetryAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn accent_type(self) -> Option<AccentType> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.accent_type)
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.category)
    }

    #[inline]
    fn word_stress(self) -> Option<WordStress> {
        PROSE_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.word_stress)
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        POETRY_ACCENT_TABLE.get(self as usize).map_or(1, |x| {
            if x.cantillation_symbol.secondary_mark.is_some() {
                2
            } else {
                1
            }
        })
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        POETRY_ACCENT_TABLE
            .get(self as usize)
            .map_or(None, |x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        BHS_POETRY_RANK_MAP[self as usize]
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        resolve_disjunctive_group(self.into()).and_then(|g| g.into_public_level())
    }
}

impl Accent for PseudoAccent {
    #[inline]
    fn english_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.english_name)
    }
    #[inline]
    fn hebrew_name(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_name)
    }
    #[inline]
    fn hebrew_concept(self) -> &'static str {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .map_or("UNKNOWN", |x| x.hebrew_concept)
    }
    #[inline]
    fn accent_type(self) -> Option<AccentType> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.accent_type)
    }

    #[inline]
    fn category(self) -> Option<AccentCategory> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.category)
    }

    #[inline]
    fn word_stress(self) -> Option<WordStress> {
        PSEUDO_ACCENT_TABLE
            .get(self as usize)
            .and_then(|x| x.word_stress)
    }
    #[inline]
    fn number_of_symbols(self) -> u8 {
        1
    }
    #[inline]
    fn notes(self) -> Option<&'static str> {
        PSEUDO_ACCENT_TABLE.get(self as usize).and_then(|x| x.notes)
    }
    #[inline]
    fn relative_strength(self) -> u8 {
        self as u8 + 1
    }
    #[inline]
    fn group_level(self) -> Option<GroupLevel> {
        None
    }
}
