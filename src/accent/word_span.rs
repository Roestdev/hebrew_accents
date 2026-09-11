use crate::WordSpan;

/// Some compound accent may span two words.
/// For most accents the rule is one accent one word.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum PrivateWordSpan {
    OneWord,
    OneOrTwoWords,
    NotApplicable,
}

impl PrivateWordSpan {
    pub(crate) const fn to_public(self) -> Option<WordSpan> {
        match self {
            PrivateWordSpan::OneWord => Some(WordSpan::OneWord),
            PrivateWordSpan::OneOrTwoWords => Some(WordSpan::OneOrTwoWords),
            PrivateWordSpan::NotApplicable => None,
        }
    }
}
