use crate::api::CompoundType;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum InternalCompoundType {
    /// Indicates a normal compound accent
    /// Meaning it consists of two accent marks and sits on one word only
    Standard,
    /// Indicates that the compound accent contains a Paseq
    /// Meaning the second accent mark is a Paseq
    ContainsPaseq,
    /// Indicates that the compound accent can span two words (or one word)
    CanSpanTwoWords,
}
impl From<InternalCompoundType> for CompoundType {
    fn from(internal: InternalCompoundType) -> Self {
        match internal {
            InternalCompoundType::Standard => CompoundType::Standard,
            InternalCompoundType::ContainsPaseq => CompoundType::ContainsPaseq,
            InternalCompoundType::CanSpanTwoWords => CompoundType::CanSpanTwoWords,
        }
    }
}
