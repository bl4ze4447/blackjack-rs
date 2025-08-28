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
            11 => "A".to_string(),
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
            11 => if score + 11 <= 21 { 11i8 } else { 1i8 },
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