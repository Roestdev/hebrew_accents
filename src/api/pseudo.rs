//! # Hebrew Pseudo-Accents
//!
//! Enumerates syntactic markers associated with biblical Hebrew cantillation that
//! are **distinct from true cantillation accents** (ta'amim).
//!
//! ## Purpose
//!
//! Pseudo-accents represent structural symbols that influence accent placement,
//! word grouping, and verse boundaries within the Masoretic text tradition.
//! Unlike true accents, they do not carry independent melodic contours.
//!
//! ## Variants
//!
//! | Variant | Hebrew | Function |
//! |---------|--------|----------|
//! | [`SophPasuq`] | סוֹף פָּסוּק | Marks verse end |
//! | [`Maqqeph`] | מַקָּף | Joins words (hyphen) |
//! | [`Paseq`] | פָּשְׁק | Separates adjacent accents |
//!
//! ## Usage
//!
//! ```ignore
//! use crate::api::PseudoAccent;
//! use crate::Accent;
//!
//! let mark = PseudoAccent::Maqqeph;
//! println!("{}", mark); // "Maqqeph (מַקָּף), meaning: hyphen"
//! ```

use crate::Accent;
use strum_macros::{EnumCount, EnumIter};

/// Represents a syntactic marker associated with Hebrew cantillation.
///
/// `PseudoAccent` values are structural symbols that influence accent placement
/// and word grouping without carrying independent melodic contour. They govern
/// phrase boundaries, word joining, and disambiguation within the Masoretic
/// text tradition.
///
/// # Distinction from True Accents
///
/// | Property | True Accents ([`ProseAccent`](crate::ProseAccent)/[`PoetryAccent`](crate::PoetryAccent)) | Pseudo-Accents |
/// |----------|--------------------------------------------------------------------------------------------------|----------------|
/// | Melodic contour | Yes — each has a unique chant melody | No — structural only |
/// | Disjunctive/conjunctive role | Yes — phrases are built around them | No — modifies accent behavior |
/// | `relative_strength()` | Returns `Some(u8)` | Always returns `None` |
/// | `group_level()` | Returns `Some` for disjunctives | Always returns `None` |
///
/// # Representation
///
/// - `#[repr(u8)]` — Each variant is stored as a single byte.
/// - `EnumCount` / `EnumIter` — Auto-derived count and iteration.
/// - Discriminant values are **explicit and consecutive**.
///
/// # Example
///
/// ```ignore
/// use crate::api::PseudoAccent;
/// use crate::Accent;
///
/// let mark = PseudoAccent::SophPasuq;
/// assert_eq!(mark.relative_strength(), None);
/// assert_eq!(mark.group_level(), None);
/// println!("{}", mark); // "Soph Pasuq (סוֹף פָּסוּק), meaning: end of verse"
/// ```
#[repr(u8)]
#[derive(EnumCount, EnumIter, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum PseudoAccent {
    /// **Soph Pasuq** (סוֹף פָּסוּק) — "end of a verse/sentence"
    ///
    /// A terminal punctuation mark resembling a large colon (׃) that marks the
    /// end of a biblical verse. It functions as the structural boundary marker
    /// for verse divisions.
    ///
    /// # Relationship to Silluq
    ///
    /// While `SophPasuq` visually marks verse endings, it is the cantillation
    /// accent **Silluq** (not `SophPasuq`) that officially designates verse
    /// endings in standard BHS (Biblia Hebraica Stuttgartensia) texts.
    /// `SophPasuq` may be absent even at valid verse boundaries in rare cases.
    ///
    /// # Usage Notes
    ///
    /// - Always appears at the very end of a verse string
    /// - Does not carry an independent melody
    /// - Affects accent parsing: the word before `SophPasuq` typically
    ///   receives the Silluq accent
    ///
    /// # Example
    ///
    /// ```text
    /// ... וְאֵ֥ת הָאָֽרֶץ׃  ׃ פ
    ///                   ↑ SophPasuq (׃)
    /// ```
    #[default]
    SophPasuq = 0,

    /// **Maqqeph** (מַקָּף) — "hyphen, joiner"
    ///
    /// A connecting line (־) that joins multiple Hebrew words into a single
    /// phonological unit. Functions analogously to a hyphen in English.
    ///
    /// # Accent Behavior
    ///
    /// When words are joined by Maqqeph:
    ///
    /// 1. Independent accents on joined words are suppressed
    /// 2. The combined unit receives a single accent (typically on the rightmost
    ///    constituent)
    /// 3. Stress shifts to the final word of the joined group
    ///
    /// # Example
    ///
    /// ```text
    /// בְּרֵאשִׁ֖ית  →  בְּרֵאשִׁ֖ית־בָּרָ֣א
    /// (two accented words)  (one accented unit via Maqqeph)
    /// ```
    Maqqeph = 1,

    /// **Paseq** (פָּשְׁק) — "separator, divider"
    ///
    /// A vertical line (׀) inserted between adjacent cantillation marks to
    /// prevent conflation of neighboring accents where disambiguation is required.
    ///
    /// # When It Appears
    ///
    /// Paseq is used when two accents that could be confused appear adjacently.
    /// It functions purely as a visual separator and never carries melodic
    /// content.
    ///
    /// # Example
    ///
    /// ```text
    /// וַיֹּ֙אמֶר֙ ׀ יְהוָ֔ה
    ///              ↑ Paseq (׀) separates adjacent accents
    /// ```
    ///
    /// # Note
    ///
    /// Paseq does not function as a standalone accent. It only appears
    /// between other cantillation marks and does not affect the melodic
    /// chanting of the verse.
    Paseq = 2,
}

impl PseudoAccent {
    /// The total number of pseudo-accent variants.
    pub const LEN: usize = <Self as strum::EnumCount>::COUNT;

    /// Returns the discriminant as `usize`, suitable for direct table indexing.
    ///
    /// This is safe to use with `PSEUDO_ACCENT_TABLE` because the enum's
    /// discriminant values are guaranteed to be consecutive starting at 0.
    #[inline]
    pub const fn as_index(self) -> usize {
        self as usize
    }
}

impl std::fmt::Display for PseudoAccent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}), meaning: {}",
            self.english_name(),
            self.hebrew_name(),
            self.hebrew_concept()
        )
    }
}

// ── Compile-time discriminant guards ────────────────────────────────────

/// Verifies that the last discriminant + 1 equals LEN.
///
/// If a variant is inserted, removed, or reordered, this const assertion
/// will fail at compile time.
const _: () = {
    const LAST_DISCRIMINANT: u8 = PseudoAccent::Paseq as u8;
    assert!((LAST_DISCRIMINANT + 1) as usize == PseudoAccent::LEN);
};
