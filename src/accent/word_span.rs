use crate::MaxWordSpan;

/// Some compound accent may span two words.
/// For most accents the rule is one accent one word.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum WordSpan {
    OneWord,
    TwoWords,
    NotApplicable,
}

impl WordSpan {
    pub(crate) const fn to_public(self) -> Option<MaxWordSpan> {
        match self {
            WordSpan::OneWord => Some(MaxWordSpan::OneWord),
            WordSpan::TwoWords => Some(MaxWordSpan::TwoWords),
            WordSpan::NotApplicable => None,
        }
    }
}
