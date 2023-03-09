use crate::{
    chess::Chess,
    constants::{A4, B4, W_P_A, W_P_B},
    piece::Color,
    player::Player,
};

pub mod board;
pub mod chess;
pub mod constants;
pub mod piece;
pub mod player;

// Implement:
// Capturing
// Pawn promotion
// Castling
// Check
// Checkmate
fn main() {
    println!("Hello, world!");

    let mut chess = Chess::new();
    let mut player_one = Player::new("Alice", Color::BLACK);

    chess.board.display(&chess.pieces);

    // chess.pieces[W_P_A] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, B5));
    player_one.move_piece(&mut chess.pieces[W_P_A], A4);
    player_one.move_piece(&mut chess.pieces[W_P_B], B4);

    chess.board.display(&chess.pieces);
}
