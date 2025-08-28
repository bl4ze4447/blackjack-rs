use rand::rng;
use rand::seq::SliceRandom;
use crate::card::Card;
use crate::card::card_suit::CardSuit;

pub struct GameDeck {
    cards: Vec<Card>
}

impl GameDeck {
    //52 cards
    pub fn new(number_of_decks: i8) -> Self {
        let mut cards: Vec<Card> = Vec::new();

        for number in 2..=14 {
            for _ in 0..number_of_decks {
                cards.push(Card::new(number, CardSuit::Hearts));
                cards.push(Card::new(number, CardSuit::Diamonds));
                cards.push(Card::new(number, CardSuit::Clubs));
                cards.push(Card::new(number, CardSuit::Spades));
            }
        }

        GameDeck {
            cards
        }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rng());
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
