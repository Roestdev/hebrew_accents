//! Main entry point for Hebrew Accent information

use crate::codepoints::TraditionNames;
// Crate‑internal (local modules)
use crate::codepoints::CodePointPosition;
use crate::AccentCategory;
use crate::AccentKind;
use crate::AccentWordStress;
use crate::GroupLevel;
use crate::HebrewAccent;
use crate::PoetryAccent;
use crate::ProseAccent;

/// Contains (non)technical details of a Hebrew Accent
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AccentMetaData {
    /// Official Hebrew name of the accent according to BHS
    pub(crate) hebrew_name: &'static str,
    /// Semantic meaning of the Hebrew term
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according `SLB Simplified`
    pub(crate) sbl_simplified_name: &'static str,
    /// Transliterated according `SLB academic`
    pub(crate) sbl_academic: &'static str,
    /// Associated Cantillation Symbol
    pub(crate) cantillation_symbol: CantillationSymbol,
    /// Optional alternate identifiers for hebrew_name, hebrew_concept, sbl_simplified_name
    pub(crate) alternate_names: Option<AlternateNames>,
    /// Indicates the accent accenttype (Primary, Secondary),
    pub(crate) kind: Kind,
    /// Indicates the accent category (Disjunctive, Conjunctive)
    pub(crate) accent_category: Category,
    /// Indicates if the accent is on the stressed syllable
    pub(crate) word_stress: WordStress,
    /// Tradition-specific naming information
    pub(crate) traditions: TraditionNames,
    /// Contextual notes or scholarly commentary
    pub(crate) notes: Option<&'static str>,
    /// Maximum word span of the accent
    pub(crate) max_word_span: Option<MaxWordSpan>,
}

/// Some compound accent may span two words.
/// For most accents the rule is one acccent one word.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum MaxWordSpan {
    OneWord,
    TwoWords,
    None,
}

/// Optional alternate representations for an accent.
/// As indicted in the BHS
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AlternateNames {
    /// Hebrew name of the accent
    pub(crate) hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub(crate) hebrew_concept: &'static str,
    /// Transliterated according `SLB Simplified``
    pub(crate) sbl_simplified_name: &'static str,
    /// Transliterated according `SLB Academic``
    pub(crate) sbl_academic: &'static str,
}

/// Struct containing the cantillation symbol of a Hebrew Accent
/// Which may consist of one or (max) two cantillation marks
/// One cantilation mark is one UTF8 code point
/// A cantillation mark is a diacritical symbol attached to a letter or a word.
/// It is characterised by its shape and its position relative to the letter or
/// word it is attached to.
///
/// A cantillation symbol consists of one or two cantillation marks.
/// It is characterised by its meaning and by the rules for its usage in a given context.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CantillationSymbol {
    /// Primary UTF-8 code point, the one that is encountered first
    pub(crate) primary_mark: &'static Utf8CodePoint,
    /// Secondary UTF-8 code point, if applicable
    pub(crate) secondary_mark: Option<&'static Utf8CodePoint>,
}

/// Details on a specific UTF-8 Unicode code-point
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Utf8CodePoint {
    /// UTF-8 code-point id, e.g. U+0591
    pub(crate) code_point_value: &'static str,
    /// The hex value of the UTF-8 code-point
    pub(crate) hex_bytes: &'static str,
    /// The name of the UTF-8 code-point as mentioned in the UTF-8 code tables
    pub(crate) canonical_name: &'static str,
    /// The symbol of the UTF-8 code-point
    pub(crate) symbol: &'static str,
    /// The position of the code-point in relation to the consonant
    pub(crate) position: CodePointPosition,
}

/// Hebrew Accent category (either Conjunctive or Disjunctive)
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Category {
    /// accents that connect words
    Conjunctive,
    #[default]
    /// accents that separate words
    Disjunctive,
    /// Only applicable for PseudoAccent's
    None,
}

impl Category {
    pub(crate) const fn to_public(self) -> Option<AccentCategory> {
        match self {
            Category::Conjunctive => Some(AccentCategory::Conjunctive),
            Category::Disjunctive => Some(AccentCategory::Disjunctive),
            Category::None => None,
        }
    }
}

/// Internal type — has None variant for pseudo accents
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Kind {
    #[default]
    Primary,
    Secondary,
    None,
}

impl Kind {
    pub(crate) const fn to_public(self) -> Option<AccentKind> {
        match self {
            Kind::Primary => Some(AccentKind::Primary),
            Kind::Secondary => Some(AccentKind::Secondary),
            Kind::None => None,
        }
    }
}

/// WordStress, indicating the location of the accent in relation to the consonant
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum WordStress {
    #[default]
    /// ImPositive: The accent is located above the stressed syllable
    Im,
    /// PostPositive: The accent is NOT located above the stressed syllable, but at the very end of the word
    Post,
    /// PrePositive: Accent is NOT located above the stressed syllable, but at the very beginning of the word
    Pre,
    /// internal use only
    None,
}

impl WordStress {
    pub(crate) const fn to_public(self) -> Option<AccentWordStress> {
        match self {
            WordStress::Im => Some(AccentWordStress::ImPositive),
            WordStress::Post => Some(AccentWordStress::PostPositive),
            WordStress::Pre => Some(AccentWordStress::PrePositive),
            WordStress::None => None,
        }
    }
}

/// Full Futato hierarchy classification with prose/poetry distinction.
///
/// **Internal use only**—do not rely on this accenttype publicly as it may change
/// without semver warning. Use [`super::GroupLevel`] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum DisjunctiveGroup {
    ProseLevel1, // Fixed variants, no number needed
    ProseLevel2,
    ProseLevel3,
    ProseLevel4,
    PoetryLevel1,
    PoetryLevel2,
    PoetryLevel3, // Max tier differs between systems
}

impl DisjunctiveGroup {
    /// Convert to simplified public GroupLevel
    pub(crate) const fn into_public_level(self) -> Option<GroupLevel> {
        match self {
            Self::ProseLevel1 | Self::PoetryLevel1 => Some(GroupLevel::Level1),
            Self::ProseLevel2 | Self::PoetryLevel2 => Some(GroupLevel::Level2),
            Self::ProseLevel3 | Self::PoetryLevel3 => Some(GroupLevel::Level3),
            Self::ProseLevel4 => Some(GroupLevel::Level4),
        }
    }
}

/// Lookup logic for accent hierarchy (private function)
pub(crate) fn resolve_disjunctive_group(accent: HebrewAccent) -> Option<DisjunctiveGroup> {
    match accent {
        HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Prose(ProseAccent::Atnach) => {
            Some(DisjunctiveGroup::ProseLevel1)
        }

        HebrewAccent::Prose(ProseAccent::Segolta)
        | HebrewAccent::Prose(ProseAccent::Shalshelet)
        | HebrewAccent::Prose(ProseAccent::ZaqephQatan)
        | HebrewAccent::Prose(ProseAccent::ZaqephGadol)
        | HebrewAccent::Prose(ProseAccent::Tiphcha) => Some(DisjunctiveGroup::ProseLevel2),

        HebrewAccent::Prose(ProseAccent::Revia)
        | HebrewAccent::Prose(ProseAccent::Zarqa)
        | HebrewAccent::Prose(ProseAccent::Pashta)
        | HebrewAccent::Prose(ProseAccent::Tevir)
        | HebrewAccent::Prose(ProseAccent::Yetiv) => Some(DisjunctiveGroup::ProseLevel3),

        HebrewAccent::Prose(ProseAccent::Geresh)
        | HebrewAccent::Prose(ProseAccent::Gershayim)
        | HebrewAccent::Prose(ProseAccent::Pazer)
        | HebrewAccent::Prose(ProseAccent::PazerGadol)
        | HebrewAccent::Prose(ProseAccent::TelishaGedolah)
        | HebrewAccent::Prose(ProseAccent::Legarmeh) => Some(DisjunctiveGroup::ProseLevel4),

        HebrewAccent::Poetry(PoetryAccent::Silluq)
        | HebrewAccent::Poetry(PoetryAccent::OlehWeYored)
        | HebrewAccent::Poetry(PoetryAccent::Atnach) => Some(DisjunctiveGroup::PoetryLevel1),

        HebrewAccent::Poetry(PoetryAccent::ReviaGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)
        | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaQaton)
        | HebrewAccent::Poetry(PoetryAccent::Tsinnor)
        | HebrewAccent::Poetry(PoetryAccent::Dechi) => Some(DisjunctiveGroup::PoetryLevel2),

        HebrewAccent::Poetry(PoetryAccent::Pazer)
        | HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
        | HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => Some(DisjunctiveGroup::PoetryLevel3),
        _ => None, // conjunctives and pseudo-accents lack hierarchy
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HebrewAccent, PoetryAccent, ProseAccent};

    // ── Category::to_public() exhaustive tests ──────────────────────

    #[test]
    fn category_conjunctive_converts_to_public_conjunctive() {
        let cat = Category::Conjunctive;
        assert_eq!(cat.to_public(), Some(AccentCategory::Conjunctive));
    }

    #[test]
    fn category_disjunctive_converts_to_public_disjunctive() {
        let cat = Category::Disjunctive;
        assert_eq!(cat.to_public(), Some(AccentCategory::Disjunctive));
    }

    #[test]
    fn category_none_converts_to_public_none() {
        let cat = Category::None;
        assert_eq!(cat.to_public(), None);
    }

    #[test]
    fn category_default_is_disjunctive() {
        let default_cat = Category::default();
        assert_eq!(default_cat, Category::Disjunctive);
    }

    // ── Kind::to_public() exhaustive tests ──────────────────────────

    #[test]
    fn kind_primary_converts_to_public_primary() {
        let kind = Kind::Primary;
        assert_eq!(kind.to_public(), Some(AccentKind::Primary));
    }

    #[test]
    fn kind_secondary_converts_to_public_secondary() {
        let kind = Kind::Secondary;
        assert_eq!(kind.to_public(), Some(AccentKind::Secondary));
    }

    #[test]
    fn kind_none_converts_to_public_none() {
        let kind = Kind::None;
        assert_eq!(kind.to_public(), None);
    }

    #[test]
    fn kind_default_is_primary() {
        let default_kind = Kind::default();
        assert_eq!(default_kind, Kind::Primary);
    }

    // ── WordStress::to_public() exhaustive tests ────────────────────

    #[test]
    fn word_stress_im_converts_to_public_im_positive() {
        let ws = WordStress::Im;
        assert_eq!(ws.to_public(), Some(AccentWordStress::ImPositive));
    }

    #[test]
    fn word_stress_post_converts_to_public_post_positive() {
        let ws = WordStress::Post;
        assert_eq!(ws.to_public(), Some(AccentWordStress::PostPositive));
    }

    #[test]
    fn word_stress_pre_converts_to_public_pre_positive() {
        let ws = WordStress::Pre;
        assert_eq!(ws.to_public(), Some(AccentWordStress::PrePositive));
    }

    #[test]
    fn word_stress_none_converts_to_public_none() {
        let ws = WordStress::None;
        assert_eq!(ws.to_public(), None);
    }

    #[test]
    fn word_stress_default_is_im() {
        let default_ws = WordStress::default();
        assert_eq!(default_ws, WordStress::Im);
    }

    // ── DisjunctiveGroup::into_public_level() exhaustive tests ──────

    #[test]
    fn disjunctive_group_prose_level1_converts_to_level1() {
        let g = DisjunctiveGroup::ProseLevel1;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level1));
    }

    #[test]
    fn disjunctive_group_poetry_level1_converts_to_level1() {
        let g = DisjunctiveGroup::PoetryLevel1;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level1));
    }

    #[test]
    fn disjunctive_group_prose_level2_converts_to_level2() {
        let g = DisjunctiveGroup::ProseLevel2;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level2));
    }

    #[test]
    fn disjunctive_group_poetry_level2_converts_to_level2() {
        let g = DisjunctiveGroup::PoetryLevel2;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level2));
    }

    #[test]
    fn disjunctive_group_prose_level3_converts_to_level3() {
        let g = DisjunctiveGroup::ProseLevel3;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level3));
    }

    #[test]
    fn disjunctive_group_poetry_level3_converts_to_level3() {
        let g = DisjunctiveGroup::PoetryLevel3;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level3));
    }

    #[test]
    fn disjunctive_group_prose_level4_converts_to_level4() {
        let g = DisjunctiveGroup::ProseLevel4;
        assert_eq!(g.into_public_level(), Some(GroupLevel::Level4));
    }

    // ── resolve_disjunctive_group() variant-specific tests ──────────

    #[test]
    fn resolve_disjunctive_group_prose_silluq_returns_prose_level1() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Silluq));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel1));
    }

    #[test]
    fn resolve_disjunctive_group_prose_atnach_returns_prose_level1() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Atnach));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel1));
    }

    #[test]
    fn resolve_disjunctive_group_prose_segolta_returns_prose_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Segolta));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_prose_shalshelet_returns_prose_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Shalshelet));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_prose_zaqeph_qatan_returns_prose_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::ZaqephQatan));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_prose_zaqeph_gadol_returns_prose_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::ZaqephGadol));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_prose_tiphcha_returns_prose_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Tiphcha));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_prose_revia_returns_prose_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Revia));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_prose_zarqa_returns_prose_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Zarqa));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_prose_pashta_returns_prose_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Pashta));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_prose_tevir_returns_prose_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Tevir));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_prose_yetiv_returns_prose_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Yetiv));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_prose_geresh_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Geresh));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_prose_gershayim_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Gershayim));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_prose_pazer_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Pazer));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_prose_pazer_gadol_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::PazerGadol));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_prose_telisha_gedolah_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::TelishaGedolah));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_prose_legarmeh_returns_prose_level4() {
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Legarmeh));
        assert_eq!(result, Some(DisjunctiveGroup::ProseLevel4));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_silluq_returns_poetry_level1() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Silluq));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel1));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_oleh_we_yored_returns_poetry_level1() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::OlehWeYored));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel1));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_atnach_returns_poetry_level1() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Atnach));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel1));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_revia_gadol_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::ReviaGadol));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_revia_mugrash_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::ReviaMugrash));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_shalshelet_gadol_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_revia_qaton_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::ReviaQaton));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_tsinnor_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Tsinnor));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_dechi_returns_poetry_level2() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Dechi));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel2));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_pazer_returns_poetry_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Pazer));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_mehuppakh_legarmeh_returns_poetry_level3() {
        let result =
            resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_poetry_azla_legarmeh_returns_poetry_level3() {
        let result = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh));
        assert_eq!(result, Some(DisjunctiveGroup::PoetryLevel3));
    }

    #[test]
    fn resolve_disjunctive_group_conjunctives_return_none() {
        // Conjunctive accents fall through to _ => None
        let result = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Munach));
        assert_eq!(result, None);
    }

    #[test]
    fn resolve_disjunctive_group_pseudo_accents_return_none() {
        // Pseudo accents fall through to _ => None
        use crate::PseudoAccent;
        let result = resolve_disjunctive_group(HebrewAccent::Pseudo(PseudoAccent::SophPasuq));
        assert_eq!(result, None);
    }

    // ── MaxWordSpan tests (currently unused but defined) ────────────

    #[test]
    fn max_word_span_one_word_constructs() {
        let _span = MaxWordSpan::OneWord;
    }

    #[test]
    fn max_word_span_two_words_constructs() {
        let _span = MaxWordSpan::TwoWords;
    }

    #[test]
    fn max_word_span_none_constructs() {
        let _span = MaxWordSpan::None;
    }

    #[test]
    fn max_word_span_variants_are_distinct() {
        assert_ne!(MaxWordSpan::OneWord, MaxWordSpan::TwoWords);
        assert_ne!(MaxWordSpan::OneWord, MaxWordSpan::None);
        assert_ne!(MaxWordSpan::TwoWords, MaxWordSpan::None);
    }

    // ── AlternateNames struct tests ─────────────────────────────────

    #[test]
    fn alternate_names_can_be_constructed() {
        let alt = AlternateNames {
            hebrew_name: "שילוש",
            hebrew_concept: "threefold chain",
            sbl_simplified_name: "Shalshelet",
            sbl_academic: "Shalshelet",
        };
        assert_eq!(alt.hebrew_name, "שילוש");
        assert_eq!(alt.sbl_simplified_name, "Shalshelet");
    }

    #[test]
    fn alternate_names_copy_clone_eq_hash_traits() {
        let alt1 = AlternateNames {
            hebrew_name: "Test",
            hebrew_concept: "Concept",
            sbl_simplified_name: "English",
            sbl_academic: "SBL",
        };
        let alt2 = alt1; // Copy
        let alt3 = alt1.clone(); // Clone
        assert_eq!(alt1, alt2);
        assert_eq!(alt1, alt3);
    }

    // ── Utf8CodePoint tests ─────────────────────────────────────────

    #[test]
    fn utf8_code_point_can_be_constructed() {
        use crate::codepoints::CodePointPosition;
        let cp = Utf8CodePoint {
            code_point_value: "U+0591",
            hex_bytes: "0591",
            canonical_name: "HEBREW ACCENT ETNAHTA",
            symbol: "֑",
            position: CodePointPosition::Above,
        };
        assert_eq!(cp.code_point_value, "U+0591");
        assert_eq!(cp.symbol, "֑");
    }

    #[test]
    fn utf8_code_point_copy_clone_eq_hash_traits() {
        use crate::codepoints::CodePointPosition;
        let cp1 = Utf8CodePoint {
            code_point_value: "U+0591",
            hex_bytes: "0591",
            canonical_name: "Test",
            symbol: "֑",
            position: CodePointPosition::Above,
        };
        let cp2 = cp1;
        let cp3 = cp1.clone();
        assert_eq!(cp1, cp2);
        assert_eq!(cp1, cp3);
    }

    // ── CantillationSymbol tests ────────────────────────────────────

    #[test]
    fn cantillation_symbol_simple_constructs_with_single_mark() {
        use crate::codepoints::CodePointPosition;
        let sym = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+0591",
                hex_bytes: "0591",
                canonical_name: "ETNAHTA",
                symbol: "֑",
                position: CodePointPosition::Above,
            },
            secondary_mark: None,
        };
        assert!(sym.secondary_mark.is_none());
    }

    #[test]
    fn cantillation_symbol_compound_constructs_with_two_marks() {
        use crate::codepoints::CodePointPosition;
        let sym = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+05A0",
                hex_bytes: "05A0",
                canonical_name: "TELISHA GEDOLA",
                symbol: "֠",
                position: CodePointPosition::Above,
            },
            secondary_mark: Some(&Utf8CodePoint {
                code_point_value: "U+05B0",
                hex_bytes: "05B0",
                canonical_name: "SHEVA",
                symbol: "ְ",
                position: CodePointPosition::Under,
            }),
        };
        assert!(sym.secondary_mark.is_some());
        assert_eq!(sym.secondary_mark.unwrap().symbol, "ְ");
    }

    #[test]
    fn cantillation_symbol_copy_clone_eq_hash_traits() {
        use crate::codepoints::CodePointPosition;
        let sym1 = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+0591",
                hex_bytes: "0591",
                canonical_name: "Test",
                symbol: "֑",
                position: CodePointPosition::Above,
            },
            secondary_mark: None,
        };
        let sym2 = sym1;
        let sym3 = sym1.clone();
        assert_eq!(sym1, sym2);
        assert_eq!(sym1, sym3);
    }

    // ── AccentMetaData struct tests ─────────────────────────────────

    #[test]
    fn accent_meta_data_can_be_constructed() {
        use crate::codepoints::CodePointPosition;
        let sym = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+0591",
                hex_bytes: "0591",
                canonical_name: "ETNAHTA",
                symbol: "֑",
                position: CodePointPosition::Above,
            },
            secondary_mark: None,
        };
        let meta = AccentMetaData {
            hebrew_name: "אתנחתא",
            hebrew_concept: "rest",
            sbl_simplified_name: "Atnach",
            sbl_academic: "Athnah",
            cantillation_symbol: sym,
            alternate_names: None,
            kind: Kind::Primary,
            accent_category: Category::Disjunctive,
            word_stress: WordStress::Im,
            traditions: TraditionNames::default(),
            notes: None,
            max_word_span: None,
        };
        assert_eq!(meta.sbl_simplified_name, "Atnach");
    }

    #[test]
    fn accent_meta_data_with_alternate_names() {
        use crate::codepoints::CodePointPosition;
        let sym = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+0591",
                hex_bytes: "0591",
                canonical_name: "ETNAHTA",
                symbol: "֑",
                position: CodePointPosition::Above,
            },
            secondary_mark: None,
        };
        let alt = AlternateNames {
            hebrew_name: "Alternative Hebrew",
            hebrew_concept: "Alt Concept",
            sbl_simplified_name: "Alternative English",
            sbl_academic: "SBL Alt",
        };
        let meta = AccentMetaData {
            hebrew_name: "אתנחתא",
            hebrew_concept: "rest",
            sbl_simplified_name: "Atnach",
            sbl_academic: "Athnah",
            cantillation_symbol: sym,
            alternate_names: Some(alt),
            kind: Kind::Primary,
            accent_category: Category::Disjunctive,
            word_stress: WordStress::Im,
            traditions: TraditionNames::default(),
            notes: Some("Test note"),
            max_word_span: Some(MaxWordSpan::OneWord),
        };
        assert!(meta.alternate_names.is_some());
        assert!(meta.notes.is_some());
        assert!(meta.max_word_span.is_some());
    }

    #[test]
    fn accent_meta_data_copy_clone_eq_hash_traits() {
        use crate::codepoints::CodePointPosition;
        let sym = CantillationSymbol {
            primary_mark: &Utf8CodePoint {
                code_point_value: "U+0591",
                hex_bytes: "0591",
                canonical_name: "ETNAHTA",
                symbol: "֑",
                position: CodePointPosition::Above,
            },
            secondary_mark: None,
        };
        let meta1 = AccentMetaData {
            hebrew_name: "Test",
            hebrew_concept: "Concept",
            sbl_simplified_name: "English",
            sbl_academic: "SBL",
            cantillation_symbol: sym,
            alternate_names: None,
            kind: Kind::Primary,
            accent_category: Category::Disjunctive,
            word_stress: WordStress::Im,
            traditions: TraditionNames::default(),
            notes: None,
            max_word_span: None,
        };
        let meta2 = meta1;
        let meta3 = meta1.clone();
        assert_eq!(meta1, meta2);
        assert_eq!(meta1, meta3);
    }

    // ── Integration: Resolve group then convert to public level ─────

    #[test]
    fn resolve_then_convert_prose_silluq_via_disjunctive_group() {
        let group = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Silluq));
        assert_eq!(group, Some(DisjunctiveGroup::ProseLevel1));
        if let Some(g) = group {
            assert_eq!(g.into_public_level(), Some(GroupLevel::Level1));
        } else {
            panic!("Expected Some(DisjunctiveGroup)");
        }
    }

    #[test]
    fn resolve_then_convert_poetry_dechi_via_disjunctive_group() {
        let group = resolve_disjunctive_group(HebrewAccent::Poetry(PoetryAccent::Dechi));
        assert_eq!(group, Some(DisjunctiveGroup::PoetryLevel2));
        if let Some(g) = group {
            assert_eq!(g.into_public_level(), Some(GroupLevel::Level2));
        } else {
            panic!("Expected Some(DisjunctiveGroup)");
        }
    }

    #[test]
    fn resolve_conjunctive_then_convert_returns_none() {
        let group = resolve_disjunctive_group(HebrewAccent::Prose(ProseAccent::Munach));
        assert_eq!(group, None);
    }
}
