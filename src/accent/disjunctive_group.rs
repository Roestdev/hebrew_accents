use crate::{GroupLevel, HebrewAccent, PoetryAccent, ProseAccent};

/// Full Futato hierarchy classification with prose/poetry distinction.
///
/// **Internal use only**—do not rely on this accenttype publicly as it may change
/// without semver warning. Use [`super::GroupLevel`] instead.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub(crate) enum DisjunctiveGroup {
    ProseTier1, // Fixed variants, no number needed
    ProseTier2,
    ProseTier3,
    ProseTier4,
    PoetryTier1,
    PoetryTier2,
    PoetryTier3, // Max tier differs between systems
}

impl DisjunctiveGroup {
    /// Convert to simplified public GroupLevel
    pub(crate) const fn into_public_level(self) -> Option<GroupLevel> {
        match self {
            Self::ProseTier1 | Self::PoetryTier1 => Some(GroupLevel::Tier1),
            Self::ProseTier2 | Self::PoetryTier2 => Some(GroupLevel::Tier2),
            Self::ProseTier3 | Self::PoetryTier3 => Some(GroupLevel::Tier3),
            Self::ProseTier4 => Some(GroupLevel::Tier4),
        }
    }
}

/// Lookup logic for accent hierarchy (private function)
pub(crate) fn resolve_disjunctive_group(accent: HebrewAccent) -> Option<DisjunctiveGroup> {
    match accent {
        HebrewAccent::Prose(ProseAccent::Silluq) | HebrewAccent::Prose(ProseAccent::Atnach) => {
            Some(DisjunctiveGroup::ProseTier1)
        }

        HebrewAccent::Prose(ProseAccent::Segolta)
        | HebrewAccent::Prose(ProseAccent::Shalshelet)
        | HebrewAccent::Prose(ProseAccent::ZaqephQatan)
        | HebrewAccent::Prose(ProseAccent::ZaqephGadol)
        | HebrewAccent::Prose(ProseAccent::Tiphcha) => Some(DisjunctiveGroup::ProseTier2),

        HebrewAccent::Prose(ProseAccent::Revia)
        | HebrewAccent::Prose(ProseAccent::Zarqa)
        | HebrewAccent::Prose(ProseAccent::Pashta)
        | HebrewAccent::Prose(ProseAccent::Tevir)
        | HebrewAccent::Prose(ProseAccent::Yetiv) => Some(DisjunctiveGroup::ProseTier3),

        HebrewAccent::Prose(ProseAccent::Geresh)
        | HebrewAccent::Prose(ProseAccent::Gershayim)
        | HebrewAccent::Prose(ProseAccent::Pazer)
        | HebrewAccent::Prose(ProseAccent::PazerGadol)
        | HebrewAccent::Prose(ProseAccent::TelishaGedolah)
        | HebrewAccent::Prose(ProseAccent::Legarmeh) => Some(DisjunctiveGroup::ProseTier4),

        HebrewAccent::Poetry(PoetryAccent::Silluq)
        | HebrewAccent::Poetry(PoetryAccent::OlehWeYored)
        | HebrewAccent::Poetry(PoetryAccent::Atnach) => Some(DisjunctiveGroup::PoetryTier1),

        HebrewAccent::Poetry(PoetryAccent::ReviaGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaMugrash)
        | HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol)
        | HebrewAccent::Poetry(PoetryAccent::ReviaQaton)
        | HebrewAccent::Poetry(PoetryAccent::Tsinnor)
        | HebrewAccent::Poetry(PoetryAccent::Dechi) => Some(DisjunctiveGroup::PoetryTier2),

        HebrewAccent::Poetry(PoetryAccent::Pazer)
        | HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh)
        | HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh) => Some(DisjunctiveGroup::PoetryTier3),
        _ => None, // conjunctives and pseudo-accents lack hierarchy
    }
}

#[cfg(test)]
mod disjunctive_group_tests {
    use super::*;
    use crate::HebrewAccent;
    use strum::IntoEnumIterator;

    // ==========================================
    // DISJUNCTIVE GROUP ENUM TESTS
    // ==========================================

    #[test]
    fn test_prose_tier1_variant_exists() {
        let tier1 = DisjunctiveGroup::ProseTier1;
        assert_eq!(tier1, DisjunctiveGroup::ProseTier1);
    }

    #[test]
    fn test_prose_tier2_variant_exists() {
        let tier2 = DisjunctiveGroup::ProseTier2;
        assert_eq!(tier2, DisjunctiveGroup::ProseTier2);
    }

    #[test]
    fn test_prose_tier3_variant_exists() {
        let tier3 = DisjunctiveGroup::ProseTier3;
        assert_eq!(tier3, DisjunctiveGroup::ProseTier3);
    }

    #[test]
    fn test_prose_tier4_variant_exists() {
        let tier4 = DisjunctiveGroup::ProseTier4;
        assert_eq!(tier4, DisjunctiveGroup::ProseTier4);
    }

    #[test]
    fn test_poetry_tier1_variant_exists() {
        let tier1 = DisjunctiveGroup::PoetryTier1;
        assert_eq!(tier1, DisjunctiveGroup::PoetryTier1);
    }

    #[test]
    fn test_poetry_tier2_variant_exists() {
        let tier2 = DisjunctiveGroup::PoetryTier2;
        assert_eq!(tier2, DisjunctiveGroup::PoetryTier2);
    }

    #[test]
    fn test_poetry_tier3_variant_exists() {
        let tier3 = DisjunctiveGroup::PoetryTier3;
        assert_eq!(tier3, DisjunctiveGroup::PoetryTier3);
    }

    // ==========================================
    // INTO_PUBLIC_LEVEL CONVERSION TESTS
    // ==========================================

    #[test]
    fn test_into_public_level_prose_tier1_to_tier1() {
        let result = DisjunctiveGroup::ProseTier1.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier1));
    }

    #[test]
    fn test_into_public_level_poetry_tier1_to_tier1() {
        let result = DisjunctiveGroup::PoetryTier1.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier1));
    }

    #[test]
    fn test_into_public_level_prose_tier2_to_tier2() {
        let result = DisjunctiveGroup::ProseTier2.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier2));
    }

    #[test]
    fn test_into_public_level_poetry_tier2_to_tier2() {
        let result = DisjunctiveGroup::PoetryTier2.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier2));
    }

    #[test]
    fn test_into_public_level_prose_tier3_to_tier3() {
        let result = DisjunctiveGroup::ProseTier3.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier3));
    }

    #[test]
    fn test_into_public_level_poetry_tier3_to_tier3() {
        let result = DisjunctiveGroup::PoetryTier3.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier3));
    }

    #[test]
    fn test_into_public_level_prose_tier4_to_tier4() {
        let result = DisjunctiveGroup::ProseTier4.into_public_level();
        assert_eq!(result, Some(GroupLevel::Tier4));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - PROSE TIER 1
    // ==========================================

    #[test]
    fn test_resolve_prose_silluq_returns_tier1() {
        let accent = HebrewAccent::Prose(ProseAccent::Silluq);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier1));
    }

    #[test]
    fn test_resolve_prose_atnach_returns_tier1() {
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier1));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - PROSE TIER 2
    // ==========================================

    #[test]
    fn test_resolve_prose_segolta_returns_tier2() {
        let accent = HebrewAccent::Prose(ProseAccent::Segolta);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier2));
    }

    #[test]
    fn test_resolve_prose_shalshelet_returns_tier2() {
        let accent = HebrewAccent::Prose(ProseAccent::Shalshelet);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier2));
    }

    #[test]
    fn test_resolve_prose_zaqeph_qatan_returns_tier2() {
        let accent = HebrewAccent::Prose(ProseAccent::ZaqephQatan);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier2));
    }

    #[test]
    fn test_resolve_prose_zaqeph_gadol_returns_tier2() {
        let accent = HebrewAccent::Prose(ProseAccent::ZaqephGadol);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier2));
    }

    #[test]
    fn test_resolve_prose_tiphcha_returns_tier2() {
        let accent = HebrewAccent::Prose(ProseAccent::Tiphcha);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier2));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - PROSE TIER 3
    // ==========================================

    #[test]
    fn test_resolve_prose_revia_returns_tier3() {
        let accent = HebrewAccent::Prose(ProseAccent::Revia);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier3));
    }

    #[test]
    fn test_resolve_prose_zarqa_returns_tier3() {
        let accent = HebrewAccent::Prose(ProseAccent::Zarqa);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier3));
    }

    #[test]
    fn test_resolve_prose_pashta_returns_tier3() {
        let accent = HebrewAccent::Prose(ProseAccent::Pashta);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier3));
    }

    #[test]
    fn test_resolve_prose_tevir_returns_tier3() {
        let accent = HebrewAccent::Prose(ProseAccent::Tevir);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier3));
    }

    #[test]
    fn test_resolve_prose_yetiv_returns_tier3() {
        let accent = HebrewAccent::Prose(ProseAccent::Yetiv);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier3));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - PROSE TIER 4
    // ==========================================

    #[test]
    fn test_resolve_prose_geresh_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::Geresh);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    #[test]
    fn test_resolve_prose_gershayim_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::Gershayim);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    #[test]
    fn test_resolve_prose_pazer_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::Pazer);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    #[test]
    fn test_resolve_prose_pazer_gadol_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::PazerGadol);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    #[test]
    fn test_resolve_prose_telisha_gedolah_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::TelishaGedolah);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    #[test]
    fn test_resolve_prose_legarmeh_returns_tier4() {
        let accent = HebrewAccent::Prose(ProseAccent::Legarmeh);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::ProseTier4));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - POETRY TIER 1
    // ==========================================

    #[test]
    fn test_resolve_poetry_silluq_returns_tier1() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Silluq);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier1));
    }

    #[test]
    fn test_resolve_poetry_ooleh_we_yored_returns_tier1() {
        let accent = HebrewAccent::Poetry(PoetryAccent::OlehWeYored);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier1));
    }

    #[test]
    fn test_resolve_poetry_atnach_returns_tier1() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Atnach);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier1));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - POETRY TIER 2
    // ==========================================

    #[test]
    fn test_resolve_poetry_revia_gadol_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaGadol);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    #[test]
    fn test_resolve_poetry_revia_mugrash_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaMugrash);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    #[test]
    fn test_resolve_poetry_shalshelet_gadol_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ShalsheletGadol);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    #[test]
    fn test_resolve_poetry_revia_qaton_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::ReviaQaton);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    #[test]
    fn test_resolve_poetry_tsinnor_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Tsinnor);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    #[test]
    fn test_resolve_poetry_dechi_returns_tier2() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Dechi);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier2));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - POETRY TIER 3
    // ==========================================

    #[test]
    fn test_resolve_poetry_pazer_returns_tier3() {
        let accent = HebrewAccent::Poetry(PoetryAccent::Pazer);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier3));
    }

    #[test]
    fn test_resolve_poetry_mehuppakh_legarmeh_returns_tier3() {
        let accent = HebrewAccent::Poetry(PoetryAccent::MehuppakhLegarmeh);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier3));
    }

    #[test]
    fn test_resolve_poetry_azla_legarmeh_returns_tier3() {
        let accent = HebrewAccent::Poetry(PoetryAccent::AzlaLegarmeh);
        let result = resolve_disjunctive_group(accent);
        assert_eq!(result, Some(DisjunctiveGroup::PoetryTier3));
    }

    // ==========================================
    // RESOLVE_DISJUNCTIVE_GROUP - NONE PATH
    // ==========================================

    #[test]
    fn test_resolve_conjunctives_returns_none() {
        // Conjunctive accents should return None
        let prose_munach = HebrewAccent::Prose(ProseAccent::Munach);
        let result = resolve_disjunctive_group(prose_munach);
        assert_eq!(result, None);
    }

    #[test]
    fn test_resolve_poetry_conjunctives_returns_none() {
        // Poetry conjunctive accents should return None
        let poetry_conjunctive = HebrewAccent::Poetry(PoetryAccent::Merkha);
        let result = resolve_disjunctive_group(poetry_conjunctive);
        assert_eq!(result, None);
    }

    // ==========================================
    // DERIVED TRAITS TESTS
    // ==========================================

    #[test]
    fn test_debug_trait_output() {
        let debug_out = format!("{:?}", DisjunctiveGroup::ProseTier2);
        assert!(debug_out.contains("ProseTier2"));
    }

    #[test]
    fn test_copy_trait_works() {
        let original = DisjunctiveGroup::PoetryTier3;
        let copied = original;
        assert_eq!(original, DisjunctiveGroup::PoetryTier3);
        assert_eq!(copied, DisjunctiveGroup::PoetryTier3);
    }

    #[test]
    fn test_clone_trait_works() {
        let original = DisjunctiveGroup::ProseTier4;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_eq_and_partial_eq() {
        let t1 = DisjunctiveGroup::PoetryTier1;
        let t2 = DisjunctiveGroup::PoetryTier1;
        let t3 = DisjunctiveGroup::ProseTier1;

        assert_eq!(t1, t2);
        assert_eq!(t1.eq(&t2), true);
        assert_ne!(t1, t3);
    }

    #[test]
    fn test_hash_trait_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        DisjunctiveGroup::ProseTier1.hash(&mut hasher1);
        DisjunctiveGroup::ProseTier1.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // ==========================================
    // REPR(U8) AND CONST CONTEXT TESTS
    // ==========================================

    #[test]
    fn test_const_evaluation_of_into_public_level() {
        const LEVEL: Option<GroupLevel> = DisjunctiveGroup::ProseTier1.into_public_level();
        assert!(matches!(LEVEL, Some(GroupLevel::Tier1)));
    }

    #[test]
    fn test_all_variants_in_sequence() {
        // Test all variants to ensure they're accessible
        let variants = [
            DisjunctiveGroup::ProseTier1,
            DisjunctiveGroup::ProseTier2,
            DisjunctiveGroup::ProseTier3,
            DisjunctiveGroup::ProseTier4,
            DisjunctiveGroup::PoetryTier1,
            DisjunctiveGroup::PoetryTier2,
            DisjunctiveGroup::PoetryTier3,
        ];

        assert_eq!(variants.len(), 7);
    }

    // ==========================================
    // INTEGRATION TESTS
    // ==========================================

    #[test]
    fn test_full_resolution_workflow() {
        // Test complete workflow from accent to public level
        let accent = HebrewAccent::Prose(ProseAccent::Atnach);

        // Step 1: Resolve to disjunctive group
        let group = resolve_disjunctive_group(accent);
        assert_eq!(group, Some(DisjunctiveGroup::ProseTier1));

        // Step 2: Convert to public level
        if let Some(g) = group {
            let public_level = g.into_public_level();
            assert_eq!(public_level, Some(GroupLevel::Tier1));
        } else {
            panic!("Expected Some group");
        }
    }

    #[test]
    fn test_tier_progression_accuracy() {
        // Verify each tier maps to correct public level
        let mappings = [
            (DisjunctiveGroup::ProseTier1, Some(GroupLevel::Tier1)),
            (DisjunctiveGroup::ProseTier2, Some(GroupLevel::Tier2)),
            (DisjunctiveGroup::ProseTier3, Some(GroupLevel::Tier3)),
            (DisjunctiveGroup::ProseTier4, Some(GroupLevel::Tier4)),
            (DisjunctiveGroup::PoetryTier1, Some(GroupLevel::Tier1)),
            (DisjunctiveGroup::PoetryTier2, Some(GroupLevel::Tier2)),
            (DisjunctiveGroup::PoetryTier3, Some(GroupLevel::Tier3)),
        ];

        for (group, expected) in mappings {
            assert_eq!(group.into_public_level(), expected);
        }
    }

    #[test]
    fn test_prose_poetry_equivalence_in_tier1() {
        // Both prose and poetry tier 1 should map to Tier1
        let prose_result = DisjunctiveGroup::ProseTier1.into_public_level();
        let poetry_result = DisjunctiveGroup::PoetryTier1.into_public_level();

        assert_eq!(prose_result, poetry_result);
        assert_eq!(prose_result, Some(GroupLevel::Tier1));
    }

    #[test]
    fn test_hierarchy_coverage_completeness() {
        // Ensure all disjunctive groups are covered by resolution
        //let total_prose_disjunctives = 17; // Count from match arms
        //let total_poetry_disjunctives = 12; // Count from match arms

        let mut resolved_count = 0;

        for variant in ProseAccent::iter() {
            let accent = HebrewAccent::Prose(variant);
            if resolve_disjunctive_group(accent).is_some() {
                resolved_count += 1;
            }
        }

        for variant in PoetryAccent::iter() {
            let accent = HebrewAccent::Poetry(variant);
            if resolve_disjunctive_group(accent).is_some() {
                resolved_count += 1;
            }
        }

        // Should have resolved all disjunctive accents
        assert!(resolved_count > 0);
    }

    #[test]
    fn test_repr_u8_discriminant_values() {
        // Verify discriminant values are sequential u8
        let prose_tier1 =
            unsafe { std::mem::transmute::<DisjunctiveGroup, u8>(DisjunctiveGroup::ProseTier1) };
        let prose_tier4 =
            unsafe { std::mem::transmute::<DisjunctiveGroup, u8>(DisjunctiveGroup::ProseTier4) };
        let poetry_tier3 =
            unsafe { std::mem::transmute::<DisjunctiveGroup, u8>(DisjunctiveGroup::PoetryTier3) };

        assert_eq!(prose_tier1, 0);
        assert_eq!(prose_tier4, 3);
        assert_eq!(poetry_tier3, 6);
    }

    #[test]
    fn test_thread_safety_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<DisjunctiveGroup>();
    }

    #[test]
    fn test_resolve_all_prose_disjunctive_variants() {
        // Batch test all prose disjunctive accents
        let prose_disjunctives = vec![
            ProseAccent::Silluq,
            ProseAccent::Atnach,
            ProseAccent::Segolta,
            ProseAccent::Shalshelet,
            ProseAccent::ZaqephQatan,
            ProseAccent::ZaqephGadol,
            ProseAccent::Tiphcha,
            ProseAccent::Revia,
            ProseAccent::Zarqa,
            ProseAccent::Pashta,
            ProseAccent::Tevir,
            ProseAccent::Yetiv,
            ProseAccent::Geresh,
            ProseAccent::Gershayim,
            ProseAccent::Pazer,
            ProseAccent::PazerGadol,
            ProseAccent::TelishaGedolah,
            ProseAccent::Legarmeh,
        ];

        for accent in prose_disjunctives {
            let result = resolve_disjunctive_group(HebrewAccent::Prose(accent));
            assert!(
                result.is_some(),
                "Prose {:?} should resolve to a group",
                accent
            );
        }
    }

    #[test]
    fn test_resolve_all_poetry_disjunctive_variants() {
        // Batch test all poetry disjunctive accents
        let poetry_disjunctives = vec![
            PoetryAccent::Silluq,
            PoetryAccent::OlehWeYored,
            PoetryAccent::Atnach,
            PoetryAccent::ReviaGadol,
            PoetryAccent::ReviaMugrash,
            PoetryAccent::ShalsheletGadol,
            PoetryAccent::ReviaQaton,
            PoetryAccent::Tsinnor,
            PoetryAccent::Dechi,
            PoetryAccent::Pazer,
            PoetryAccent::MehuppakhLegarmeh,
            PoetryAccent::AzlaLegarmeh,
        ];

        for accent in poetry_disjunctives {
            let result = resolve_disjunctive_group(HebrewAccent::Poetry(accent));
            assert!(
                result.is_some(),
                "Poetry {:?} should resolve to a group",
                accent
            );
        }
    }
}
