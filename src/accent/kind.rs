use crate::AccentKind;

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
