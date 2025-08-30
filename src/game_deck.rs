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

#[cfg(test)]
mod tests {
    use crate::game_deck::GameDeck;
    const CARDS_PER_DECK: usize = 52;

    #[test]
    fn test_ctor() {
        let gd = GameDeck::new(1);
        assert_eq!(gd.cards.len(), CARDS_PER_DECK);

        let gd = GameDeck::new(6);
        assert_eq!(gd.cards.len(), CARDS_PER_DECK * 6);

        let gd = GameDeck::new(0);
        assert_eq!(gd.cards.len(), 0);

        let gd = GameDeck::new(-20);
        assert_eq!(gd.cards.len(), 0);
    }

    #[test]
    fn test_deal_card() {
        let mut gd = GameDeck::new(1);
        let card = gd.deal_card();
        assert!(card.is_some());

        let mut gd = GameDeck::new(0);
        let card = gd.deal_card();
        assert_eq!(card.is_some(), false);

        let mut gd = GameDeck::new(-20);
        let card = gd.deal_card();
        assert_eq!(card.is_some(), false);
    }
}