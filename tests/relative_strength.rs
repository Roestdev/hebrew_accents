use hebrew_accents::{ProseAccent,PoetryAccent,PseudoAccent,Accent};

    // ═══════════════════════════════════════════════════════════════
    // PROSE ACCENT TESTS
    // ═══════════════════════════════════════════════════════════════

    /// Test all prose disjunctive accents (should return Some(strength))
    #[test]
    fn test_prose_disjunctive_strength() {
        // Primary disjunctives: indices 0-17, strengths 1-18
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(ProseAccent::Atnach.relative_strength(), Some(2));
        assert_eq!(ProseAccent::Segolta.relative_strength(), Some(3));
        assert_eq!(ProseAccent::Shalshelet.relative_strength(), Some(4));
        assert_eq!(ProseAccent::ZaqephQatan.relative_strength(), Some(5));
        assert_eq!(ProseAccent::ZaqephGadol.relative_strength(), Some(6));
        assert_eq!(ProseAccent::Revia.relative_strength(), Some(7));
        assert_eq!(ProseAccent::Tiphcha.relative_strength(), Some(8));
        assert_eq!(ProseAccent::Zarqa.relative_strength(), Some(9));
        assert_eq!(ProseAccent::Pashta.relative_strength(), Some(10));
        assert_eq!(ProseAccent::Yetiv.relative_strength(), Some(11));
        assert_eq!(ProseAccent::Tevir.relative_strength(), Some(12));
        assert_eq!(ProseAccent::Geresh.relative_strength(), Some(13));
        assert_eq!(ProseAccent::Gershayim.relative_strength(), Some(14));
        assert_eq!(ProseAccent::Pazer.relative_strength(), Some(15));
        assert_eq!(ProseAccent::PazerGadol.relative_strength(), Some(16));
        assert_eq!(ProseAccent::TelishaGedolah.relative_strength(), Some(17));
        assert_eq!(ProseAccent::Legarmeh.relative_strength(), Some(18));
    }

    /// Test all prose conjunctive accents (should return None)
    #[test]
    fn test_prose_conjunctive_strength() {
        // Conjunctives: indices 18-27, all should be None
        assert_eq!(ProseAccent::Munach.relative_strength(), None);
        assert_eq!(ProseAccent::Mahpakh.relative_strength(), None);
        assert_eq!(ProseAccent::Merkha.relative_strength(), None);
        assert_eq!(ProseAccent::MerkhaKephulah.relative_strength(), None);
        assert_eq!(ProseAccent::Darga.relative_strength(), None);
        assert_eq!(ProseAccent::Azla.relative_strength(), None);
        assert_eq!(ProseAccent::TelishaQetannah.relative_strength(), None);
        assert_eq!(ProseAccent::Galgal.relative_strength(), None);
        assert_eq!(ProseAccent::Meayla.relative_strength(), None);
        assert_eq!(ProseAccent::Meteg.relative_strength(), None);
    }

    // ═══════════════════════════════════════════════════════════════
    // POETRY ACCENT TESTS
    // ═══════════════════════════════════════════════════════════════

    /// Test all poetry disjunctive accents (should return Some(strength))
    #[test]
    fn test_poetry_disjunctive_strength() {
        // Primary disjunctives: indices 0-11, strengths 1-12
        assert_eq!(PoetryAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(PoetryAccent::OlehWeYored.relative_strength(), Some(2));
        assert_eq!(PoetryAccent::Atnach.relative_strength(), Some(3));
        assert_eq!(PoetryAccent::ReviaGadol.relative_strength(), Some(4));
        assert_eq!(PoetryAccent::ReviaMugrash.relative_strength(), Some(5));
        assert_eq!(PoetryAccent::ShalsheletGadol.relative_strength(), Some(6));
        assert_eq!(PoetryAccent::Tsinnor.relative_strength(), Some(7));
        assert_eq!(PoetryAccent::ReviaQaton.relative_strength(), Some(8));
        assert_eq!(PoetryAccent::Dechi.relative_strength(), Some(9));
        assert_eq!(PoetryAccent::Pazer.relative_strength(), Some(10));
        assert_eq!(PoetryAccent::MehuppakhLegarmeh.relative_strength(), Some(11));
        assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), Some(12));
    }

    /// Test all poetry conjunctive accents (should return None)
    #[test]
    fn test_poetry_conjunctive_strength() {
        // Conjunctives: indices 12-22, all should be None
        assert_eq!(PoetryAccent::Munach.relative_strength(), None);
        assert_eq!(PoetryAccent::Merkha.relative_strength(), None);
        assert_eq!(PoetryAccent::Illuy.relative_strength(), None);
        assert_eq!(PoetryAccent::Tarcha.relative_strength(), None);
        assert_eq!(PoetryAccent::Galgal.relative_strength(), None);
        assert_eq!(PoetryAccent::Mehuppakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Azla.relative_strength(), None);
        assert_eq!(PoetryAccent::ShalsheletQetannah.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMerkha.relative_strength(), None);
        assert_eq!(PoetryAccent::TsinnoritMahpakh.relative_strength(), None);
        assert_eq!(PoetryAccent::Meteg.relative_strength(), None);
    }

    // ═══════════════════════════════════════════════════════════════
    // PSEUDO ACCENT TESTS
    // ═══════════════════════════════════════════════════════════════

    /// Test all pseudo accents (should always return None)
    #[test]
    fn test_pseudo_accent_strength() {
        // Pseudo accents never have strength hierarchy
        assert_eq!(PseudoAccent::SophPasuq.relative_strength(), None);
        assert_eq!(PseudoAccent::Maqqaph.relative_strength(), None);
        assert_eq!(PseudoAccent::Paseq.relative_strength(), None);
    }

    // ═══════════════════════════════════════════════════════════════
    // COMPREHENSIVE ITERATION TESTS
    // ═══════════════════════════════════════════════════════════════

    /// Exhaustively test all prose accents using iterator
    #[test]
    fn test_all_prose_accents_via_iterator() {
        use strum::IntoEnumIterator;

        for (expected_strength, accent) in ProseAccent::iter().enumerate() {
            let index = expected_strength;
            let actual = accent.relative_strength();

            // Indices 0-17 are disjunctives: expect Some(index + 1)
            // Indices 18-27 are conjunctives: expect None
            if index < 18 {
                assert_eq!(actual, Some((index + 1) as u8),
                    "ProseAccent {:?} at index {} should have strength {}",
                    accent, index, index + 1
                );
            } else {
                assert_eq!(actual, None,
                    "ProseAccent {:?} at index {} should have no strength",
                    accent, index
                );
            }
        }
    }

    /// Exhaustively test all poetry accents using iterator
    #[test]
    fn test_all_poetry_accents_via_iterator() {
        use strum::IntoEnumIterator;

        for (expected_strength, accent) in PoetryAccent::iter().enumerate() {
            let index = expected_strength;
            let actual = accent.relative_strength();

            // Indices 0-11 are disjunctives: expect Some(index + 1)
            // Indices 12-22 are conjunctives: expect None
            if index < 12 {
                assert_eq!(actual, Some((index + 1) as u8),
                    "PoetryAccent {:?} at index {} should have strength {}",
                    accent, index, index + 1
                );
            } else {
                assert_eq!(actual, None,
                    "PoetryAccent {:?} at index {} should have no strength",
                    accent, index
                );
            }
        }
    }

    /// Exhaustively test all pseudo accents using iterator
    #[test]
    fn test_all_pseudo_accents_via_iterator() {
        use strum::IntoEnumIterator;

        for accent in PseudoAccent::iter() {
            assert_eq!(accent.relative_strength(), None,
                "PseudoAccent {:?} should always have no strength",
                accent
            );
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // CROSS-SYSTEM VALIDATION TESTS
    // ═══════════════════════════════════════════════════════════════

    /// Verify strength ordering: lower number = stronger accent
    #[test]
    fn test_strength_ordering_semantics() {
        // Silluq (strength 1) should be stronger than Atnach (strength 2)
        assert!(
            ProseAccent::Silluq.relative_strength() < ProseAccent::Atnach.relative_strength(),
            "Silluq should have lower (stronger) rank than Atnach"
        );

        // Verify hierarchy integrity: Atnach > Segolta > Shalshelet
        assert_eq!(ProseAccent::Atnach.relative_strength(), Some(2));
        assert_eq!(ProseAccent::Segolta.relative_strength(), Some(3));
        assert_eq!(ProseAccent::Shalshelet.relative_strength(), Some(4));
    }

    /// Ensure conjunctive accents are incomparable with disjunctives
    #[test]
    fn test_conjunctive_vs_disjunctive_none_semantics() {
        // Conjunctives should return None regardless of comparison
        let conjunctive_none = ProseAccent::Munach.relative_strength();
        let disjunctive_some = ProseAccent::Silluq.relative_strength();

        assert_eq!(conjunctive_none, None);
        assert_eq!(disjunctive_some, Some(1));
    }

    /// Verify shared accent names may have different strengths across systems
    #[test]
    fn test_shared_accent_name_strength_differences() {
        // Silluq has strength 1 in both systems
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(PoetryAccent::Silluq.relative_strength(), Some(1));

        // Atnach has strength 2 in Prose, strength 3 in Poetry
        assert_eq!(ProseAccent::Atnach.relative_strength(), Some(2));
        assert_eq!(PoetryAccent::Atnach.relative_strength(), Some(3));

        // Munach returns None in both systems (conjunctive)
        assert_eq!(ProseAccent::Munach.relative_strength(), None);
        assert_eq!(PoetryAccent::Munach.relative_strength(), None);

        // Pazer: strength 15 in Prose, strength 10 in Poetry
        assert_eq!(ProseAccent::Pazer.relative_strength(), Some(15));
        assert_eq!(PoetryAccent::Pazer.relative_strength(), Some(10));
    }

    /// Validate strength range bounds
    #[test]
    fn test_strength_range_bounds() {
        // Prose: minimum strength = 1, maximum = 18
        assert_eq!(ProseAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(ProseAccent::Legarmeh.relative_strength(), Some(18));

        // Poetry: minimum strength = 1, maximum = 12
        assert_eq!(PoetryAccent::Silluq.relative_strength(), Some(1));
        assert_eq!(PoetryAccent::AzlaLegarmeh.relative_strength(), Some(12));

        // No accent should exceed u8::MAX (our sentinel)
        assert!(
            !crate::Accent::relative_strength(ProseAccent::Meteg)
                .map_or(false, |s| s == u8::MAX),
            "Sentinel value should not leak to users"
        );
    }

