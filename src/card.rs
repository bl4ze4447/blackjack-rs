pub mod card_suit;

use std::fmt;
use std::fmt::Formatter;
use crate::card::card_suit::CardSuit;

pub struct Card {
    number: i8,
    suit: CardSuit
}

impl Card {
    pub fn new(number: i8, suit: CardSuit) -> Self {
        Card {
            number,
            suit
        }
    }

    pub fn get_rank(&self) -> String {
        match self.number {
            2..=10 => self.number.to_string(),
            11 | 1 => "A".to_string(),
            12 => "J".to_string(),
            13 => "Q".to_string(),
            14 => "K".to_string(),
            _ => {
                eprintln!("Unknown rank was used: {}", self.number);
                "Unknown".to_string()
            }
        }
    }

    pub fn value(&self, score: i8) -> i8 {
        match self.number {
            2..=10 => self.number,
            11 | 1 => if score + 11 <= 21 { 11i8 } else { 1i8 },
            12 | 13 | 14 => 10i8,
            _ => {
                eprintln!("Unknown number was used: {}", self.number);
                -1i8
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} of {}", self.get_rank(), self.suit)
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::card_suit::CardSuit;

    #[test]
    fn test_get_rank() {
        let card: Card = Card::new(1, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "A");

        let card: Card = Card::new(11, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "A");

        let card: Card = Card::new(12, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "J");

        let card: Card = Card::new(13, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "Q");

        let card: Card = Card::new(14, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "K");

        let card: Card = Card::new(2, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "2");

        let card: Card = Card::new(3, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "3");

        let card: Card = Card::new(4, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "4");

        let card: Card = Card::new(5, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "5");

        let card: Card = Card::new(6, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "6");

        let card: Card = Card::new(7, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "7");

        let card: Card = Card::new(8, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "8");

        let card: Card = Card::new(9, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "9");

        let card: Card = Card::new(100, CardSuit::Clubs);
        assert_eq!(card.get_rank(), "Unknown");
    }

    #[test]
    fn test_get_value() {
        for i in 2..=10 {
            let card: Card = Card::new(i, CardSuit::Diamonds);
            assert_eq!(card.value(0), i);
        }

        for i in 12..=14 {
            let card: Card = Card::new(i, CardSuit::Diamonds);
            assert_eq!(card.value(0), 10);
        }

        let card: Card = Card::new(1, CardSuit::Diamonds);
        assert_eq!(card.value(0), 11);

        let card: Card = Card::new(1, CardSuit::Diamonds);
        assert_eq!(card.value(10), 11);

        let card: Card = Card::new(1, CardSuit::Diamonds);
        assert_eq!(card.value(11), 1);

        let card: Card = Card::new(11, CardSuit::Diamonds);
        assert_eq!(card.value(0), 11);

        let card: Card = Card::new(11, CardSuit::Diamonds);
        assert_eq!(card.value(10), 11);

        let card: Card = Card::new(11, CardSuit::Diamonds);
        assert_eq!(card.value(11), 1);

        let card: Card = Card::new(100, CardSuit::Diamonds);
        assert_eq!(card.value(11), -1);
    }
}