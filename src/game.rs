use std::io::{stdin, stdout, Write};
use std::num::ParseIntError;
use std::thread::sleep;
use std::time::Duration;
use crate::card::Card;
use crate::game_deck::GameDeck;

enum PlayerType {
    Dealer,
    Player,
}

#[derive(PartialEq)]
enum HandType {
    Simple,
    Blackjack,
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
    ended: bool,
    balance: i64,
    bet_value: i32,
    last_payout: i32,
    player_hand: HandType,
    dealer_hand: HandType
}

const NUMBER_OF_DECKS: i8 = 6;
const SHUFFLE_TIMES: i8 = 16;

impl Game {
    fn get_bet() -> Result<i32, ParseIntError> {
        Self::get_choice().parse::<i32>()
    }

    pub fn start(balance: i64) -> Self {
        let mut game_deck = GameDeck::new(NUMBER_OF_DECKS);

        for _ in 0..SHUFFLE_TIMES {
            game_deck.shuffle();
        }

        let bet: i32 = loop {
            println!("> Current balance: {balance} lei");
            print!("> Place a bet (minimum 1 leu, maximul 25000 lei): ");
            if let Ok(bet_value) = Self::get_bet() {
                if bet_value < 1 {
                    println!("Bet value is too small!");
                    continue;
                }

                if bet_value as i64 > balance {
                    println!("Bet value exceeds current balance!");
                    continue;
                }

                if bet_value as i64 > 25000 {
                    println!("Bet value exceeds max bet value!");
                    continue;
                }

                break bet_value;
            }

            println!("> Invalid bet value!");
        };

        let mut player_score: i8 = 0;
        let mut dealer_score: i8 = 0;

        let mut player_hand = HandType::Simple;
        let dealer_hand = HandType::Simple;

        Self::handle_card_deal(&mut game_deck, &mut player_score, PlayerType::Player);
        sleep(Duration::from_millis(1000));

        Self::handle_card_deal(&mut game_deck, &mut dealer_score, PlayerType::Dealer);
        sleep(Duration::from_millis(1000));

        Self::handle_card_deal(&mut game_deck, &mut player_score, PlayerType::Player);

        // check if current hand is blackjack
        if player_score == 21 {
            player_hand = HandType::Blackjack;
        }

        let hidden_card = game_deck.deal_card().unwrap(); // safe to unwrap

        Game {
            dealer_score,
            player_score,
            hidden_card,
            game_deck,
            player_hand,
            dealer_hand,
            balance: balance - bet as i64,
            bet_value: bet,
            turn: PlayerType::Player,
            ended: false,
            used_hidden_card: false,
            rounds_lost: 0,
            rounds_won: 0,
            last_payout: 0,
        }
    }

    pub fn reset(&mut self) {
        let rounds_won = self.rounds_won;
        let rounds_lost = self.rounds_lost;

        *self = Self::start(self.balance);
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
            sleep(Duration::from_secs(3));
            println!("New round started!");
            self.reset();
            return true;
        }

        match self.turn {
            PlayerType::Player => {
                // ask for input
                print!("> It's your turn, choose an option: (h)it / (s)tand / (q)uit: ");
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

                if choice == "q" {
                    self.ended = true;
                    return false;
                }

                println!("Please (h)it or (s)tand!");
                true
            }

            PlayerType::Dealer => {
                if self.used_hidden_card == false {
                    Self::handle_card_deal_by_card(&mut self.hidden_card, &mut self.dealer_score, PlayerType::Dealer);
                    sleep(Duration::from_millis(1000));

                    if self.dealer_score == 21 {
                        self.dealer_hand = HandType::Blackjack;
                    }

                    self.used_hidden_card = true;
                    return true;
                }

                if self.dealer_end_check() == true {
                    return true;
                }

                Self::handle_card_deal(&mut self.game_deck, &mut self.dealer_score, PlayerType::Dealer);
                sleep(Duration::from_millis(1000));

                true
            }
        }
    }

    fn player_end_check(&mut self) {
        if self.player_score > 21 {
            println!(">> You busted! <<");
            self.rounds_lost += 1;

            self.handle_payout(PlayerType::Dealer, false);

            self.show_round_score();
            self.show_balance();
            self.ended = true;
        }
    }

    fn dealer_end_check(&mut self) -> bool {
        if self.player_hand == HandType::Blackjack {
            if self.dealer_hand == HandType::Blackjack {
                self.handle_draw();
            } else {
                self.handle_player_win();
            }

            self.ended = true;
            self.show_round_score();
            self.show_balance();

            return self.ended;
        }

        if self.dealer_hand == HandType::Blackjack {
            self.handle_dealer_win();
            self.ended = true;
            self.show_round_score();
            self.show_balance();

            return self.ended;
        }

        if self.dealer_score >= 17 {
            if self.player_score > self.dealer_score || self.dealer_score > 21 {
                self.handle_player_win();
            } else if self.player_score < self.dealer_score {
                self.handle_dealer_win();
            } else {
                self.handle_draw();
            }

            self.show_round_score();
            self.show_balance();
            self.ended = true;
        }

        self.ended
    }

    fn handle_draw(&mut self) {
        println!(">> WOW, this round ended in a draw! <<");
        self.handle_payout(PlayerType::Dealer, true);
    }

    fn handle_player_win(&mut self) {
        println!(">> You won this round! <<");
        self.rounds_won += 1;
        self.handle_payout(PlayerType::Player, false);
    }

    fn handle_dealer_win(&mut self) {
        println!(">> The dealer won this round! <<");
        self.rounds_lost += 1;
        self.handle_payout(PlayerType::Dealer, false);
    }

    fn handle_payout(&mut self, winner: PlayerType, draw: bool) {
        if draw == true {
            self.last_payout = self.bet_value;
            self.balance += self.bet_value as i64;
            return;
        }

        match winner {
            PlayerType::Dealer => {
                self.last_payout = -self.bet_value;
            }

            PlayerType::Player => {
                if self.player_hand == HandType::Blackjack {
                    self.balance += (self.bet_value * 3 / 2) as i64;
                    self.last_payout = self.bet_value * 3 / 2;
                    return;
                }

                self.balance += (self.bet_value * 2) as i64;
                self.last_payout = self.bet_value * 2;
            }
        }
    }

    fn show_round_score(&self) {
        println!("> You won: {} round/s\n> The dealer won: {} round/s", self.rounds_won, self.rounds_lost);
    }

    fn show_balance(&self) {
        println!("> Payout: {} lei!\n", self.last_payout);
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
        let (message, hand_message, border) = match player {
            PlayerType::Dealer => ("The dealer got",    "Dealer's hand",    "--------------------------------"),
            PlayerType::Player => ("You've been dealt", "Your hand",        "================================")
        };

        println!("{border}");
        println!("{message} {card}!");
        println!("{hand_message}: {score}");
        println!("{border}");
    }
}