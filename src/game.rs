use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use crate::card::Card;
use crate::game_deck::GameDeck;

enum PlayerType {
    Dealer,
    Player,
}

pub struct Game {
    dealer_score: i8,
    player_score: i8,
    rounds_won: i32,
    rounds_lost: i32,
    hidden_card: Card,
    used_hidden_card: bool,
    game_deck: GameDeck,
    turn: PlayerType,
    ended: bool
}

const NUMBER_OF_DECKS: i8 = 6;
const SHUFFLE_TIMES: i8 = 16;

impl Game {
    pub fn start() -> Self {
        let mut game_deck = GameDeck::new(NUMBER_OF_DECKS);

        for _ in 0..SHUFFLE_TIMES {
            game_deck.shuffle();
        }

        let mut player_score: i8 = 0;
        let mut dealer_score: i8 = 0;

        Self::handle_card_deal(&mut game_deck, &mut player_score, PlayerType::Player);
        sleep(Duration::from_millis(500));

        Self::handle_card_deal(&mut game_deck, &mut dealer_score, PlayerType::Dealer);
        sleep(Duration::from_millis(500));

        Self::handle_card_deal(&mut game_deck, &mut player_score, PlayerType::Player);

        let hidden_card = game_deck.deal_card().unwrap(); // safe to unwrap

        Game {
            dealer_score,
            player_score,
            hidden_card,
            game_deck,
            turn: PlayerType::Player,
            ended: false,
            used_hidden_card: false,
            rounds_lost: 0,
            rounds_won: 0
        }
    }

    pub fn reset(&mut self) {
        let rounds_won = self.rounds_won;
        let rounds_lost = self.rounds_lost;

        *self = Self::start();
        self.rounds_lost = rounds_lost;
        self.rounds_won = rounds_won;
    }

    fn get_choice() -> String {
        stdout().flush().unwrap_or_else( | error | {
            eprintln!("App could not flush stdout: {}", error.to_string());
        });

        let mut line = String::new();
        stdin().read_line(&mut line).unwrap_or_else(| error | {
            eprintln!("Failed to read line: {}", error.to_string());
            0usize
        });
        let choice = line.trim().to_lowercase();

        choice
    }

    pub fn next(&mut self) -> bool {
        if self.ended == true {
            print!("The round ended, do you wish to play again? (y)es or (n)o: ");
            let choice = Game::get_choice();

            if choice == "y" {
                println!("\nNew round started!");
                self.reset();
                return true;
            }

            if choice == "n" {
                return false;
            }

            println!("Please type 'y' for Yes or 'n' for No!");
            return true;
        }

        match self.turn {
            PlayerType::Player => {
                // ask for input
                print!("It's your turn, choose an option: (h)it or (s)tand: ");
                let choice = Self::get_choice();

                if choice == "h" {
                    Self::handle_card_deal(&mut self.game_deck, &mut self.player_score, PlayerType::Player);
                    Self::player_end_check(self);
                    return true;
                }

                if choice == "s" {
                    self.turn = PlayerType::Dealer;
                    return true;
                }

                println!("Please (h)it or (s)tand!");
                true
            }

            PlayerType::Dealer => {
                if self.used_hidden_card == false {
                    Self::handle_card_deal_by_card(&mut self.hidden_card, &mut self.dealer_score, PlayerType::Dealer);
                    sleep(Duration::from_millis(500));

                    self.used_hidden_card = true;
                    return true;
                }

                if self.dealer_end_check() == true {
                    return true;
                }

                Self::handle_card_deal(&mut self.game_deck, &mut self.dealer_score, PlayerType::Dealer);
                sleep(Duration::from_millis(500));

                true
            }
        }
    }

    fn player_end_check(&mut self) {
        if self.player_score > 21 {
            println!(">> You busted! <<");
            self.rounds_lost += 1;

            self.show_round_score();
            self.ended = true;
        }
    }

    fn dealer_end_check(&mut self) -> bool {
        if self.dealer_score >= 17 {
            if self.player_score > self.dealer_score || self.dealer_score > 21 {
                println!(">> You won this round! <<");
                self.rounds_won += 1;
            } else if self.player_score < self.dealer_score {
                println!(">> The dealer won this round! <<");
                self.rounds_lost += 1;
            } else {
                println!(">> WOW, this round ended in a draw! <<");
            }

            self.show_round_score();
            self.ended = true;
            return true;
        }

        false
    }

    fn show_round_score(&self) {
        println!("> You won: {} round/s\n> The dealer won: {} round/s\n", self.rounds_won, self.rounds_lost);
    }

    fn handle_card_deal(game_deck: &mut GameDeck, score: &mut i8, player_type: PlayerType) {
        match game_deck.deal_card() {
            Some(card) => {
                *score += card.value(*score);
                Self::announce_card_dealt(&card, player_type, *score);
            }
            None => {
                *game_deck = GameDeck::new(NUMBER_OF_DECKS);
                Self::handle_card_deal(game_deck, score, player_type);
            }
        }
    }

    fn handle_card_deal_by_card(card: &mut Card, score: &mut i8, player_type: PlayerType) {
        *score += card.value(*score);
        Self::announce_card_dealt(&card, player_type, *score);
    }

    fn announce_card_dealt(card: &Card, player: PlayerType, score: i8) {
        let (message, hand_message) = match player {
            PlayerType::Dealer => ("The dealer got", "Dealer's hand"),
            PlayerType::Player => ("You've been dealt", "Your hand")
        };

        println!("--------------------------------\n{message} {card}!\n{hand_message}: {score}\n--------------------------------\n");
    }
}