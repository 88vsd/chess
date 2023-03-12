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
// Chess notations
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

    chess.board.display(&chess.pieces);

    player_one.move_piece("PE7E5", &mut chess);
    player_one.move_piece("PA7A5", &mut chess);
    player_one.move_piece("PB7B5", &mut chess);
    player_one.move_piece("NB8A6", &mut chess);

    chess.board.display(&chess.pieces);

    // println!("{:#?}", chess.get_pieces());
}
