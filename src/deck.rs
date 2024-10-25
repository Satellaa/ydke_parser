/// Represents a YGOPro deck with main deck, extra deck, and side deck.
#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    pub main: Vec<u32>,
    pub extra: Vec<u32>,
    pub side: Vec<u32>,
}