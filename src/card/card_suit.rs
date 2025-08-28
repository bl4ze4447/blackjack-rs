use std::fmt;
use std::fmt::Formatter;

pub enum CardSuit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CardSuit::Heart => write!(f, "Heart"),
            CardSuit::Diamond => write!(f, "Diamond"),
            CardSuit::Club => write!(f, "Club"),
            CardSuit::Spade => write!(f, "Spade")
        }
    }
}