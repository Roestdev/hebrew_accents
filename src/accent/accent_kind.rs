use crate::api::AccentKind;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InternalAccentKind {
    /// Primary Hebrew accent type
    Primary,
    /// Secondary Hebrew accent type
    Secondary,
}

impl From<InternalAccentKind> for AccentKind {
    fn from(internal: InternalAccentKind) -> Self {
        match internal {
            InternalAccentKind::Primary => AccentKind::Primary,
            InternalAccentKind::Secondary => AccentKind::Secondary,
        }
    }
}
