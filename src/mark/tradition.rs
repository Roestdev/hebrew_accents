//! Accents names according to one of four Hebrew Traditions
//!
//! Biblical Hebrew does not have a single, universal pronunciation.
//! It has been transmitted through four principal reading traditions,
//! each of which handles stress placement differently.
//!
//! The four Traditions:
//! - Ashkenazi (the Eastern European tradition)
//! - Sephardi (the Iberian and North African tradition)
//! - Yemenite (the most archaic, from the Yemenite Jewish community)
//! - Italian (the tradition of the Italian Jewish community)

/// A specific accent name in one tradition
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct AccentName {
    pub(crate) hebrew_name: &'static str,
    pub(crate) sbl_academic: &'static str,
    pub(crate) english_name: &'static str,
}

/// Names for an accent across all four Hebrew traditions
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub(crate) struct TraditionNames {
    pub(crate) ashkenazi: Option<AccentName>,
    pub(crate) sephardi: Option<AccentName>,
    pub(crate) italian: Option<AccentName>,
    pub(crate) yemenite: Option<AccentName>,
}

impl TraditionNames {
    /// Create uniform names across all four traditions
    pub(crate) const fn uniform(
        hebrew: &'static str,
        sbl: &'static str,
        english: &'static str,
    ) -> Self {
        Self {
            ashkenazi: Some(AccentName {
                hebrew_name: hebrew,
                sbl_academic: sbl,
                english_name: english,
            }),
            sephardi: Some(AccentName {
                hebrew_name: hebrew,
                sbl_academic: sbl,
                english_name: english,
            }),
            italian: Some(AccentName {
                hebrew_name: hebrew,
                sbl_academic: sbl,
                english_name: english,
            }),
            yemenite: Some(AccentName {
                hebrew_name: hebrew,
                sbl_academic: sbl,
                english_name: english,
            }),
        }
    }

    // Get name for a specific tradition
    // pub(crate) const fn get(&self, tradition: Tradition) -> Option<AccentName> {
    //     match tradition {
    //         Tradition::Ashkenazi => self.ashkenazi,
    //         Tradition::Sephardi => self.sephardi,
    //         Tradition::Italian => self.italian,
    //         Tradition::Yemenite => self.yemenite,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uniform_sets_all_traditions_identically() {
        let names = TraditionNames::uniform("דְּכִי", "deḥî", "Dehi");

        let expected = AccentName {
            hebrew_name: "דְּכִי",
            sbl_academic: "deḥî",
            english_name: "Dehi",
        };

        assert_eq!(names.ashkenazi, Some(expected));
        assert_eq!(names.sephardi, Some(expected));
        assert_eq!(names.italian, Some(expected));
        assert_eq!(names.yemenite, Some(expected));
    }

    #[test]
    fn uniform_individual_fields_are_correct() {
        let names = TraditionNames::uniform("זָקֵף", "zāqēp̄", "Zaquph");

        // Verify each field independently rather than relying solely on struct equality
        for tradition in [
            &names.ashkenazi,
            &names.sephardi,
            &names.italian,
            &names.yemenite,
        ] {
            let name = tradition.expect("tradition should be Some");
            assert_eq!(name.hebrew_name, "זָקֵף");
            assert_eq!(name.sbl_academic, "zāqēp̄");
            assert_eq!(name.english_name, "Zaquph");
        }
    }

    #[test]
    fn uniform_with_empty_strings() {
        // Edge case: all empty strings are still valid
        let names = TraditionNames::uniform("", "", "");

        let expected = AccentName {
            hebrew_name: "",
            sbl_academic: "",
            english_name: "",
        };

        assert_eq!(names.ashkenazi, Some(expected));
        assert_eq!(names.sephardi, Some(expected));
        assert_eq!(names.italian, Some(expected));
        assert_eq!(names.yemenite, Some(expected));
    }

    #[test]
    fn default_produces_all_none() {
        let names = TraditionNames::default();

        assert_eq!(names.ashkenazi, None);
        assert_eq!(names.sephardi, None);
        assert_eq!(names.italian, None);
        assert_eq!(names.yemenite, None);
    }

    #[test]
    fn accent_name_equality_and_inequality() {
        let a = AccentName {
            hebrew_name: "מֶרְכָּא",
            sbl_academic: "merka",
            english_name: "Merka",
        };
        let b = AccentName {
            hebrew_name: "מֶרְכָּא",
            sbl_academic: "merka",
            english_name: "Merka",
        };
        let c = AccentName {
            hebrew_name: "מָהִיר",
            sbl_academic: "mahir",
            english_name: "Mahir",
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn tradition_names_equality() {
        let a = TraditionNames::uniform("תְּבִיר", "təbîr", "Tevir");
        let b = TraditionNames::uniform("תְּבִיר", "təbîr", "Tevir");

        assert_eq!(a, b);
    }

    #[test]
    fn tradition_names_inequality_when_default() {
        let uniform = TraditionNames::uniform("פַּסְתָּא", "pasta", "Pasta");
        let default = TraditionNames::default();

        assert_ne!(uniform, default);
    }

    #[test]
    fn accent_name_copy_clone() {
        let original = AccentName {
            hebrew_name: "גַּעְיָא",
            sbl_academic: "gaʿya",
            english_name: "Gaaya",
        };

        // Copy semantics: assigning should clone, not move
        let copied = original;
        // If Copy isn't derived, this line wouldn't compile
        let _also_copied = original;

        assert_eq!(copied, original);
    }

    #[test]
    fn accent_name_hash_consistency() {
        use std::collections::HashMap;

        let name = AccentName {
            hebrew_name: "שׁוֹפָר",
            sbl_academic: "šop̄ar",
            english_name: "Shofar",
        };

        let mut map: HashMap<AccentName, u8> = HashMap::new();
        map.insert(name, 1);

        // Same value should retrieve from the map (exercises Hash + Eq)
        let key = AccentName {
            hebrew_name: "שׁוֹפָר",
            sbl_academic: "šop̄ar",
            english_name: "Shofar",
        };

        assert_eq!(map.get(&key), Some(&1));
    }
}
