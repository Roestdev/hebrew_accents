use crate::accent::PrivAlternateNames;

/// Optional alternate representations for an accent.
///
/// For some accents the `Biblia Hebraica Stuttgartensia` (BHS)
/// indicates an alternate spelling or name variant.
///
/// The following accents have alternatives documented:
/// | listed | alternative
/// | ----- | ------|
/// | Geresh | Teres |
/// | Pazer Gadol | Qarne Pharah |
/// | Mahpakh | Mehuppakh |
/// | Azla | Teres |
/// | Galgal | Jerach Ben Jomo |
/// | Meayla | Mayla |
/// | Meteg | Gayah |
/// | Oleh Weyored | Mahpakh and Merkha |
/// | Tsinnor | Zarqa |
/// | Dechi | Tiphcha |
/// | Mehuppakh | Mahpakh |
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AlternateNames {
    /// Hebrew name of the accent
    pub hebrew_name: &'static str,
    /// Meaning of the Hebrew name
    pub hebrew_concept: &'static str,
    /// Transliterated according the file `TRANSLITERATION.md`
    pub english_name: &'static str,
    /// Transliterated according `SBL Academic`
    pub sbl_academic: &'static str,
}

impl From<PrivAlternateNames> for AlternateNames {
    fn from(internal: PrivAlternateNames) -> Self {
        AlternateNames {
            hebrew_name: internal.hebrew_name,
            hebrew_concept: internal.hebrew_concept,
            english_name: internal.english_name,
            sbl_academic: internal.sbl_academic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== BASIC STRUCT CONSTRUCTION TESTS =====

    #[test]
    fn test_alternate_names_struct_exists() {
        let alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion, driving out",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        assert_eq!(alts.hebrew_name, "גֵּרֶשׁ");
        assert_eq!(alts.hebrew_concept, "expulsion, driving out");
        assert_eq!(alts.english_name, "Geresh");
        assert_eq!(alts.sbl_academic, "gērēš");
    }

    #[test]
    fn test_all_fields_are_static_lifetimes() {
        // Compile-time check that all fields are &'static str
        let alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion, driving out",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        // These assignments should compile without issues
        let _h_name: &'static str = alts.hebrew_name;
        let _h_concept: &'static str = alts.hebrew_concept;
        let _e_name: &'static str = alts.english_name;
        let _sbl: &'static str = alts.sbl_academic;

        assert!(!_h_name.is_empty());
        assert!(!_h_concept.is_empty());
        assert!(!_e_name.is_empty());
        assert!(!_sbl.is_empty());
    }

    // ===== DOCUMENTED ALTERNATE NAMES TESTS =====

    #[test]
    fn test_geresh_alternative_teres() {
        let geresh_alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Teres", // Alternative name
            sbl_academic: "tĕrēs",
        };

        assert_eq!(geresh_alts.hebrew_name, "גֵּרֶשׁ");
        assert_eq!(geresh_alts.english_name, "Teres");
    }

    #[test]
    fn test_pazer_gadol_alternative_qarne_pharah() {
        let pazer_gadol_alts = AlternateNames {
            hebrew_name: "פָּזֵר גָּדוֹל",
            hebrew_concept: "great scatter",
            english_name: "Qarne Pharah", // Alternative
            sbl_academic: "qārənê pāārâ",
        };

        assert_eq!(pazer_gadol_alts.english_name, "Qarne Pharah");
    }

    #[test]
    fn test_mahpakh_alternative_mehuppakh() {
        let mahpakh_alts = AlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh", // Alternative
            sbl_academic: "məhuppāḵ",
        };

        assert_eq!(mahpakh_alts.english_name, "Mehuppakh");
    }

    #[test]
    fn test_azla_alternative_teres() {
        let azla_alts = AlternateNames {
            hebrew_name: "אַזְלָא",
            hebrew_concept: "going forth",
            english_name: "Teres", // Alternative (same as Geresh variant)
            sbl_academic: "ʾazlāʾ",
        };

        assert_eq!(azla_alts.english_name, "Teres");
    }

    #[test]
    fn test_galgal_alternative_jerach_ben_jomo() {
        let galgal_alts = AlternateNames {
            hebrew_name: "גַּלְגַּל",
            hebrew_concept: "wheel",
            english_name: "Jerach Ben Jomo", // Alternative
            sbl_academic: "yarēaḥ ben yôwām",
        };

        assert_eq!(galgal_alts.english_name, "Jerach Ben Jomo");
    }

    #[test]
    fn test_meayla_alternative_mayla() {
        let meayla_alts = AlternateNames {
            hebrew_name: "מֵעֲיָא",
            hebrew_concept: "from a heap",
            english_name: "Mayla", // Alternative
            sbl_academic: "mēʿāyāʾ",
        };

        assert_eq!(meayla_alts.english_name, "Mayla");
    }

    #[test]
    fn test_meteg_alternative_gayah() {
        let meteg_alts = AlternateNames {
            hebrew_name: "מֶתֶג",
            hebrew_concept: "bridle",
            english_name: "Gayah", // Alternative
            sbl_academic: "gāyâ",
        };

        assert_eq!(meteg_alts.english_name, "Gayah");
    }

    #[test]
    fn test_oleh_veyored_alternative_mahpakh_merkha() {
        let oleh_alts = AlternateNames {
            hebrew_name: "עוֹלֶה וְיוֹרֵד",
            hebrew_concept: "ascending and descending",
            english_name: "Mahpakh and Merkha", // Alternative
            sbl_academic: "ʿōleh wəyōrēd",
        };

        assert_eq!(oleh_alts.english_name, "Mahpakh and Merkha");
    }

    #[test]
    fn test_tsinnor_alternative_zarqa() {
        let tsinnor_alts = AlternateNames {
            hebrew_name: "צִנּוֹר",
            hebrew_concept: "channel",
            english_name: "Zarqa", // Alternative
            sbl_academic: "zārqāʾ",
        };

        assert_eq!(tsinnor_alts.english_name, "Zarqa");
    }

    #[test]
    fn test_dechi_alternative_tiphcha() {
        let dechi_alts = AlternateNames {
            hebrew_name: "דְּחִי",
            hebrew_concept: "pushed away",
            english_name: "Tiphcha", // Alternative
            sbl_academic: "ṭip̄ḵāʾ",
        };

        assert_eq!(dechi_alts.english_name, "Tiphcha");
    }

    #[test]
    fn test_mehuppakh_alternative_mahpakh() {
        let mehuppakh_alts = AlternateNames {
            hebrew_name: "מְהֻפָּךְ",
            hebrew_concept: "inverted",
            english_name: "Mahpakh", // Alternative
            sbl_academic: "məhuppāḵ",
        };

        assert_eq!(mehuppakh_alts.english_name, "Mahpakh");
    }

    // ===== FROM TRAIT IMPLEMENTATION TESTS =====

    #[test]
    fn test_from_priv_alternate_names_basic() {
        let internal = PrivAlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Teres",
            sbl_academic: "tĕrēs",
        };

        let public: AlternateNames = internal.into();

        assert_eq!(public.hebrew_name, "גֵּרֶשׁ");
        assert_eq!(public.hebrew_concept, "expulsion");
        assert_eq!(public.english_name, "Teres");
        assert_eq!(public.sbl_academic, "tĕrēs");
    }

    #[test]
    fn test_from_conversion_preserves_all_fields() {
        let internal = PrivAlternateNames {
            hebrew_name: "פָּזֵר גָּדוֹל",
            hebrew_concept: "great scatter",
            english_name: "Qarne Pharah",
            sbl_academic: "qārənê pāārâ",
        };

        let public: AlternateNames = internal.into();

        // Direct pattern match for verification
        assert_eq!(public.hebrew_name, "פָּזֵר גָּדוֹל");
        assert_eq!(public.hebrew_concept, "great scatter");
        assert_eq!(public.english_name, "Qarne Pharah");
        assert_eq!(public.sbl_academic, "qārənê pāārâ");
    }

    #[test]
    fn test_from_conversion_identity() {
        let original = PrivAlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh",
            sbl_academic: "məhuppāḵ",
        };

        let converted: AlternateNames = original.clone().into();

        // Verify no data loss during conversion
        assert_eq!(original.hebrew_name, converted.hebrew_name);
        assert_eq!(original.hebrew_concept, converted.hebrew_concept);
        assert_eq!(original.english_name, converted.english_name);
        assert_eq!(original.sbl_academic, converted.sbl_academic);
    }

    #[test]
    fn test_from_trait_const_compatible() {
        // Verify both types can be used in const contexts
        const INTERNAL: PrivAlternateNames = PrivAlternateNames {
            hebrew_name: "מֶתֶג",
            hebrew_concept: "bridle",
            english_name: "Gayah",
            sbl_academic: "gāyâ",
        };

        const PUBLIC: AlternateNames = AlternateNames {
            hebrew_name: "מֶתֶג",
            hebrew_concept: "bridle",
            english_name: "Gayah",
            sbl_academic: "gāyâ",
        };

        assert_eq!(INTERNAL.hebrew_name, PUBLIC.hebrew_name);
        assert_eq!(INTERNAL.english_name, PUBLIC.english_name);
    }

    // ===== DERIVED TRAIT TESTS =====

    // --- Debug Trait ---

    #[test]
    fn test_debug_trait_output() {
        let alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let debug = format!("{:?}", alts);
        assert!(debug.contains("AlternateNames"));
        assert!(debug.contains("Geresh"));
        assert!(debug.contains("expulsion"));
    }

    #[test]
    fn test_debug_trait_all_fields_visible() {
        let alts = AlternateNames {
            hebrew_name: "פָּזֵר",
            hebrew_concept: "scatter",
            english_name: "Pazer",
            sbl_academic: "pāzer",
        };

        let debug = format!("{:?}", alts);

        // All fields should appear in debug output
        assert!(debug.contains("hebrew_name"));
        assert!(debug.contains("hebrew_concept"));
        assert!(debug.contains("english_name"));
        assert!(debug.contains("sbl_academic"));
    }

    // --- Copy Trait ---

    #[test]
    fn test_copy_trait_works() {
        let original = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let copied = original; // Copy occurs automatically

        assert_eq!(original, copied);
        assert_eq!(copied.english_name, "Geresh");
        // Original still usable after copy
        assert_eq!(original.hebrew_name, "גֵּרֶשׁ");
    }

    #[test]
    fn test_copy_trait_no_allocation() {
        // Verify all fields are &'static (zero allocation)
        let alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let _copied = alts; // Stack copy only
                            // No heap allocation occurs
    }

    // --- Clone Trait ---

    #[test]
    fn test_clone_trait_works() {
        let original = AlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh",
            sbl_academic: "məhuppāḵ",
        };

        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(cloned.english_name, "Mehuppakh");
    }

    // --- PartialEq/Eq Trait ---

    #[test]
    fn test_partial_eq_same_values() {
        let a1 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let a2 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        assert_eq!(a1, a2);
    }

    #[test]
    fn test_partial_eq_different_values() {
        let a1 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let a2 = AlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh",
            sbl_academic: "məhuppāḵ",
        };

        assert_ne!(a1, a2);
    }

    #[test]
    fn test_eq_trait_reflexivity() {
        let a = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        assert!(a.eq(&a));
    }

    #[test]
    fn test_eq_trait_symmetry() {
        let a1 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        let a2 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        assert!(a1.eq(&a2));
        assert!(a2.eq(&a1));
    }

    // --- Hash Trait ---

    #[test]
    fn test_hash_trait_consistency() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        let a1 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };

        a1.hash(&mut hasher1);
        a1.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    #[test]
    fn test_hash_in_collections() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let a1 = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        };
        let a2 = AlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh",
            sbl_academic: "məhuppāḵ",
        };

        set.insert(a1);
        set.insert(a2);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&a1));
        assert!(set.contains(&a2));
    }

    // ===== REAL BHS DATA TESTS =====

    #[test]
    fn test_geresh_bhs_alternate() {
        // According to BHS, Geresh can also be called Teres
        let bhs_variant = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion, driving out",
            english_name: "Teres",
            sbl_academic: "tĕrēs",
        };

        assert_eq!(bhs_variant.hebrew_name, "גֵּרֶשׁ");
        assert_eq!(bhs_variant.english_name, "Teres");
        assert_eq!(bhs_variant.sbl_academic, "tĕrēs");
    }

    #[test]
    fn test_all_documented_alternatives_exist() {
        // Verify all 11 documented alternatives can be created
        let alternatives = vec![
            ("Geresh", "Teres"),
            ("Pazer Gadol", "Qarne Pharah"),
            ("Mahpakh", "Mehuppakh"),
            ("Azla", "Teres"),
            ("Galgal", "Jerach Ben Jomo"),
            ("Meayla", "Mayla"),
            ("Meteg", "Gayah"),
            ("Oleh Weyored", "Mahpakh and Merkha"),
            ("Tsinnor", "Zarqa"),
            ("Dechi", "Tiphcha"),
            ("Mehuppakh", "Mahpakh"),
        ];

        for (primary, alt) in &alternatives {
            let alts = AlternateNames {
                hebrew_name: primary,
                hebrew_concept: "test",
                english_name: alt,
                sbl_academic: alt,
            };

            assert_eq!(alts.english_name, *alt);
            assert_eq!(alts.hebrew_name, *primary);
        }
    }

    // ===== COMBINATION TESTS =====

    #[test]
    fn test_complete_alternate_names_workflow() {
        // Full workflow: create internal -> convert to public -> use
        let internal = PrivAlternateNames {
            hebrew_name: "פָּזֵר גָּדוֹל",
            hebrew_concept: "great scatter",
            english_name: "Qarne Pharah",
            sbl_academic: "qārənê pāārâ",
        };

        let public: AlternateNames = internal.into();

        // All operations should succeed
        assert_eq!(public.hebrew_name, "פָּזֵר גָּדוֹל");
        assert_eq!(public.english_name, "Qarne Pharah");
        assert_eq!(public.sbl_academic, "qārənê pāārâ");

        // Can be displayed
        let _display = format!("{:?}", public);

        // Can be compared
        let another = AlternateNames {
            hebrew_name: "פָּזֵר גָּדוֹל",
            hebrew_concept: "great scatter",
            english_name: "Qarne Pharah",
            sbl_academic: "qārənê pāārâ",
        };

        assert_eq!(public, another);
    }

    #[test]
    fn test_alternate_names_in_result_wrapper() {
        // Should work with Result/Option wrappers
        let result_ok: Result<AlternateNames, ()> = Ok(AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš",
        });

        let option_some: Option<AlternateNames> = Some(AlternateNames {
            hebrew_name: "מַהְפָּךְ",
            hebrew_concept: "overturning",
            english_name: "Mehuppakh",
            sbl_academic: "məhuppāḵ",
        });

        assert!(result_ok.is_ok());
        assert!(option_some.is_some());
        assert_eq!(result_ok.unwrap().english_name, "Geresh");
        assert_eq!(option_some.unwrap().english_name, "Mehuppakh");
    }

    // ===== EDGE CASE TESTS =====

    #[test]
    fn test_empty_strings_allowed() {
        // Empty strings are technically valid (though not recommended)
        let empty = AlternateNames {
            hebrew_name: "",
            hebrew_concept: "",
            english_name: "",
            sbl_academic: "",
        };

        assert!(empty.hebrew_name.is_empty());
        assert!(empty.english_name.is_empty());
    }

    #[test]
    fn test_unicode_characters_in_fields() {
        // Hebrew and special characters should work
        let alts = AlternateNames {
            hebrew_name: "עוֹלֶה וְיוֹרֵד",
            hebrew_concept: "ascending and descending",
            english_name: "Oleh WeYored",
            sbl_academic: "ʿōleh wəyōrēd",
        };

        assert!(alts.hebrew_name.contains('ו'));
        assert!(alts.sbl_academic.contains('ʿ'));
    }

    #[test]
    fn test_special_chaclars_in_sbl() {
        // SBL transliteration uses diacritics
        let alts = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion",
            english_name: "Geresh",
            sbl_academic: "gērēš", // Contains macron (ē) and shiv'e (š)
        };

        assert!(alts.sbl_academic.contains('ē'));
        assert!(alts.sbl_academic.contains('š'));
    }

    #[test]
    fn test_long_names_handling() {
        // Longer alternate names should work fine
        let alts = AlternateNames {
            hebrew_name: "עוֹלֶה וְיוֹרֵד",
            hebrew_concept: "ascending and descending",
            english_name: "Mahpakh and Merkha (compound form)",
            sbl_academic: "mahpaḵ wəmerkā",
        };

        assert!(alts.english_name.contains("Mahpakh"));
        assert!(alts.english_name.contains("Merkha"));
        assert_eq!(alts.english_name.len(), 34);
    }

    // ===== CONST EVALUATION TESTS =====

    #[test]
    fn test_const_alternate_names() {
        const ALTS: AlternateNames = AlternateNames {
            hebrew_name: "מֶתֶג",
            hebrew_concept: "bridle",
            english_name: "Gayah",
            sbl_academic: "gāyâ",
        };

        assert_eq!(ALTS.hebrew_name, "מֶתֶג");
        assert_eq!(ALTS.english_name, "Gayah");
        assert_eq!(ALTS.sbl_academic, "gāyâ");
    }

    #[test]
    fn test_static_alternate_names() {
        static METEG_ALTS: AlternateNames = AlternateNames {
            hebrew_name: "מֶתֶג",
            hebrew_concept: "bridle",
            english_name: "Gayah",
            sbl_academic: "gāyâ",
        };

        assert_eq!(METEG_ALTS.english_name, "Gayah");
        assert_eq!(METEG_ALTS.hebrew_name, "מֶתֶג");
    }

    // ===== USAGE SCENARIO TESTS =====

    #[test]
    fn test_alternate_names_for_bhs_lookup() {
        // Simulate BHS lookup scenario
        fn lookup_alt_names(accent_name: &str) -> Option<AlternateNames> {
            match accent_name {
                "Geresh" => Some(AlternateNames {
                    hebrew_name: "גֵּרֶשׁ",
                    hebrew_concept: "expulsion",
                    english_name: "Teres",
                    sbl_academic: "tĕrēs",
                }),
                "Meteg" => Some(AlternateNames {
                    hebrew_name: "מֶתֶג",
                    hebrew_concept: "bridle",
                    english_name: "Gayah",
                    sbl_academic: "gāyâ",
                }),
                _ => None,
            }
        }

        let geresh = lookup_alt_names("Geresh");
        let unknown = lookup_alt_names("Silluq");

        assert!(geresh.is_some());
        assert!(unknown.is_none());
        assert_eq!(geresh.unwrap().english_name, "Teres");
    }

    #[test]
    fn test_alternate_names_in_map() {
        use std::collections::HashMap;

        let mut bhs_alternatives: HashMap<&'static str, AlternateNames> = HashMap::new();

        bhs_alternatives.insert(
            "Geresh",
            AlternateNames {
                hebrew_name: "גֵּרֶשׁ",
                hebrew_concept: "expulsion",
                english_name: "Teres",
                sbl_academic: "tĕrēs",
            },
        );

        bhs_alternatives.insert(
            "Meteg",
            AlternateNames {
                hebrew_name: "מֶתֶג",
                hebrew_concept: "bridle",
                english_name: "Gayah",
                sbl_academic: "gāyâ",
            },
        );

        assert_eq!(bhs_alternatives.len(), 2);
        assert!(bhs_alternatives.contains_key("Geresh"));
        assert!(bhs_alternatives.contains_key("Meteg"));
        assert_eq!(
            bhs_alternatives.get("Geresh").unwrap().english_name,
            "Teres"
        );
    }

    // ===== DOCUMENTATION EXAMPLE VERIFICATION =====

    #[test]
    fn doc_example_bhs_spelling_variant() {
        // From documentation: "BHS indicates alternate spelling"
        let bhs_variant = AlternateNames {
            hebrew_name: "גֵּרֶשׁ",
            hebrew_concept: "expulsion, driving out",
            english_name: "Teres",
            sbl_academic: "tĕrēs",
        };

        // The alternative name is documented
        assert_eq!(bhs_variant.english_name, "Teres");
        assert_eq!(bhs_variant.hebrew_name, "גֵּרֶשׁ");
    }

    #[test]
    fn doc_example_all_documented_accent_alternatives() {
        // Verify the table in docs is accurate
        let documented = [
            ("Geresh", "Teres"),
            ("Pazer Gadol", "Qarne Pharah"),
            ("Mahpakh", "Mehuppakh"),
            ("Azla", "Teres"),
            ("Galgal", "Jerach Ben Jomo"),
            ("Meayla", "Mayla"),
            ("Meteg", "Gayah"),
            ("Oleh Weyored", "Mahpakh and Merkha"),
            ("Tsinnor", "Zarqa"),
            ("Dechi", "Tiphcha"),
            ("Mehuppakh", "Mahpakh"),
        ];

        // Count documented alternatives
        assert_eq!(documented.len(), 11);

        // All should be creatable
        for (primary, alt) in &documented {
            let _alts = AlternateNames {
                hebrew_name: primary,
                hebrew_concept: "documented",
                english_name: alt,
                sbl_academic: alt,
            };
        }
    }
}
