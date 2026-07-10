/// Accents names according one of four Hebrew Traditions
///
/// Biblical Hebrew does not have a single, universal pronunciation.
/// It has been transmitted through four principal reading traditions,
/// each of which handles stress placement differently.
///
/// The four Traditions:
/// - Ashkenazi (the Eastern European tradition)
/// - Sephardi (the Iberian and North African tradition)
/// - Yemenite (the most archaic, from the Yemenite Jewish community)
/// - Italian (the tradition of the Italian Jewish community)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub(crate) enum Tradition {
    /// Naming of the accent according Ashkenazi tradition
    Ashkenazi {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Sephardi tradition
    Sephardi {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Italian tradition
    Italian {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
    /// Naming of the accent according Yemenite tradition
    Yemenite {
        /// Hebrew name of the accent
        hebrew_name: &'static str,
        /// Transliterated English name
        english_name: &'static str,
    },
}
