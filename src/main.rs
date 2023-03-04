use crate::{game::Game, piece::Color, player::Player};

pub mod board;
pub mod game;
pub mod piece;
pub mod player;

fn main() {
    println!("Hello, world!");

    let player_one = Player::new("Alice", Color::BLACK);
    let player_two = Player::new("Bob", Color::WHITE);

    let game = Game::new(player_one, player_two);

    game.play();
}
