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

#[cfg(test)]
mod tests {
    use crate::card::card_suit::CardSuit;

    #[test]
    fn test_to_string() {
        let h = CardSuit::Hearts;
        let d = CardSuit::Diamonds;
        let c = CardSuit::Clubs;
        let s = CardSuit::Spades;

        assert_eq!(h.to_string(), "Hearts");
        assert_eq!(d.to_string(), "Diamonds");
        assert_eq!(c.to_string(), "Clubs");
        assert_eq!(s.to_string(), "Spades");
    }
}