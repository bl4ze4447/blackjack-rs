pub enum CardSuit {
    Heart,
    Diamond,
    Club,
    Spade,
}

impl CardSuit {
    pub fn to_string(&self) -> &str {
        match self {
            CardSuit::Heart => "Heart",
            CardSuit::Diamond => "Diamond",
            CardSuit::Club => "Club",
            CardSuit::Spade => "Spade"
        }
    }
}