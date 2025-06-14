#![allow(unused)]

use crate::HebrewAccent;
use crate::PoetryAccent;
use crate::ProseAccent;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] //,Display
pub enum Context {
    Poetry,
    #[default]
    Prose,
}
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)] // Copy,Display
pub struct SentenceContext {
    sentence: String,
    context: Context,
}

impl SentenceContext {
    #[allow(dead_code)]
    fn new(sentence: &str, context: Context) -> SentenceContext {
        SentenceContext {
            sentence: sentence.to_string(),
            context,
        }
    }

    fn contains(&self, accent: HebrewAccent) -> bool {
        match accent {
            // Prose Disjunctives
            HebrewAccent::TwentyOne(ProseAccent::Silluq) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Atnach) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Segolta) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Shalshelet) if self.context == Context::Prose => {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephQatan) if self.context == Context::Prose => {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephGadol) if self.context == Context::Prose => {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::Revia) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Tiphcha) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Zarqa) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Pashta) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Yetiv) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Tevir) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Geresh) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Gershayim) if self.context == Context::Prose => {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::Pazer) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::PazerGadol) if self.context == Context::Prose => {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::TelishaGedolah)
                if self.context == Context::Prose =>
            {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::Legarmeh) if self.context == Context::Prose => {
                true
            }
            // Prose Conjunctives
            HebrewAccent::TwentyOne(ProseAccent::Munnach) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Mahpakh) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Merkha) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prose =>
            {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::Darga) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Azla) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::TelishaQetannah)
                if self.context == Context::Prose =>
            {
                true
            }
            HebrewAccent::TwentyOne(ProseAccent::Galgal) if self.context == Context::Prose => true,
            HebrewAccent::TwentyOne(ProseAccent::Meayela) if self.context == Context::Prose => true,
            // Poetry Disjunctives
            HebrewAccent::Three(PoetryAccent::Silluq) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::OleWeYored) if self.context == Context::Poetry => {
                true
            }
            HebrewAccent::Three(PoetryAccent::Atnach) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::ReviaGadol) if self.context == Context::Poetry => {
                true
            }
            HebrewAccent::Three(PoetryAccent::ReviaMugrash) if self.context == Context::Poetry => {
                true
            }
            HebrewAccent::Three(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetry =>
            {
                true
            }
            HebrewAccent::Three(PoetryAccent::Tsinnor) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::ReviaQaton) if self.context == Context::Poetry => {
                true
            }
            HebrewAccent::Three(PoetryAccent::Dechi) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Pazer) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetry =>
            {
                true
            }
            HebrewAccent::Three(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetry => {
                true
            }
            // Poetry Conjunctives
            HebrewAccent::Three(PoetryAccent::Munnach) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Merkha) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Illuy) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Tarkha) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Galgal) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Mehuppakh) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::Azla) if self.context == Context::Poetry => true,
            HebrewAccent::Three(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetry =>
            {
                true
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetry =>
            {
                true
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetry =>
            {
                true
            }
            _ => false,
        }
    }

    fn find(&self, accent: HebrewAccent) -> Option<usize> {
        match accent {
            // Prose Disjunctives
            HebrewAccent::TwentyOne(ProseAccent::Silluq) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Atnach) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Segolta) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Shalshelet) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephQatan) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::ZaqephGadol) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Revia) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Tiphcha) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Zarqa) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Pashta) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Yetiv) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Tevir) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Geresh) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Gershayim) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Pazer) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::PazerGadol) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::TelishaGedolah)
                if self.context == Context::Prose =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Legarmeh) if self.context == Context::Prose => {
                Some(1)
            }
            // Prose Conjunctives
            HebrewAccent::TwentyOne(ProseAccent::Munnach) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Mahpakh) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Merkha) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::MerkhaKephulah)
                if self.context == Context::Prose =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Darga) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Azla) if self.context == Context::Prose => Some(1),
            HebrewAccent::TwentyOne(ProseAccent::TelishaQetannah)
                if self.context == Context::Prose =>
            {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Galgal) if self.context == Context::Prose => {
                Some(1)
            }
            HebrewAccent::TwentyOne(ProseAccent::Meayela) if self.context == Context::Prose => {
                Some(1)
            }
            // Poetry Disjunctives
            HebrewAccent::Three(PoetryAccent::Silluq) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::OleWeYored) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Atnach) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::ReviaGadol) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ReviaMugrash) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ShalsheletGadol)
                if self.context == Context::Poetry =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Tsinnor) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::ReviaQaton) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Dechi) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::Pazer) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::MehuppakhLegarmeh)
                if self.context == Context::Poetry =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::AzlaLegarmeh) if self.context == Context::Poetry => {
                Some(1)
            }
            // Poetry Conjunctives
            HebrewAccent::Three(PoetryAccent::Munnach) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Merkha) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::Illuy) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::Tarkha) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::Galgal) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::Mehuppakh) if self.context == Context::Poetry => {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::Azla) if self.context == Context::Poetry => Some(1),
            HebrewAccent::Three(PoetryAccent::ShalsheletQetannah)
                if self.context == Context::Poetry =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMerkha)
                if self.context == Context::Poetry =>
            {
                Some(1)
            }
            HebrewAccent::Three(PoetryAccent::TsinnoritMahpakh)
                if self.context == Context::Poetry =>
            {
                Some(1)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain() {
        let newsc = SentenceContext::new("gad", Context::Prose);
        assert!(newsc.contains(HebrewAccent::TwentyOne(ProseAccent::Atnach)));
        assert!(newsc.contains(HebrewAccent::TwentyOne(ProseAccent::Meayela)));
    }
    #[test]
    fn test_find() {
        let newsc = SentenceContext::new("gad", Context::Prose);
        assert_eq!(
            newsc.find(HebrewAccent::TwentyOne(ProseAccent::Galgal)),
            Some(1)
        );
        assert_eq!(
            newsc.find(HebrewAccent::TwentyOne(ProseAccent::Atnach)),
            Some(1)
        );
        assert_eq!(
            newsc.find(HebrewAccent::Three(PoetryAccent::AzlaLegarmeh)),
            None
        );
    }
}
