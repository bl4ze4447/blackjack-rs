mod card;
mod game_deck;
mod game;

use crate::game::Game;

fn main() {
    let mut game: Game = Game::start(100);
    while game.next() { }
}