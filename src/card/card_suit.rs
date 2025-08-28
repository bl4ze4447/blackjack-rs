use std::fmt;
use std::fmt::Formatter;

pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            CardSuit::Hearts => write!(f, "Hearts"),
            CardSuit::Diamonds => write!(f, "Diamonds"),
            CardSuit::Clubs => write!(f, "Clubs"),
            CardSuit::Spades => write!(f, "Spades")
        }
    }
}