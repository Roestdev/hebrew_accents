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
    pub(crate) const fn uniform(hebrew: &'static str, sbl: &'static str ,english: &'static str) -> Self {
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

// /// Which Hebrew reading tradition
//#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
//#[non_exhaustive]
// pub(crate) enum Tradition {
//     Ashkenazi,
//     Sephardi,
//     Italian,
//     Yemenite,
//}
