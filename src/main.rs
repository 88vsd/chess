use crate::{
    chess::Chess,
    constants::{Pieces, A4, B4, W_P_A, W_P_B},
    notation::Notation,
    piece::Color,
    player::Player,
};

pub mod board;
pub mod chess;
pub mod constants;
pub mod notation;
pub mod piece;
pub mod player;
pub mod point;

// Implement:
// ✅ Allowable moves for each piece
// ✅ Display chessboard based on each piece's location
// ✅ Chess notations
// Capturing
// Pawn promotion
// Castling
// Check
// Checkmate
// While loop to run the game
// Turn chessboard after each move around for player
fn main() {
    println!("Hello, world!");

    let mut chess = Chess::new();
    let mut player_one = Player::new("Alice", Color::WHITE);
    let mut player_two = Player::new("Bob", Color::BLACK);

    let is_playing = true;

    while is_playing {
        chess.board.display(&chess.pieces);

        player_one.move_piece(&mut chess);

        chess.board.display(&chess.pieces);

        player_two.move_piece(&mut chess);

        // println!("{:#?}", chess.get_pieces());
    }
}
