use crate::{chess::Chess, piece::Color, player::Player};

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
// ✅ Chess notations (TODO: add castling)
// Capturing
// Pawn promotion
// Castling
// Check
// Checkmate
// ✅ While loop to run the game
// ✅ Display 2 perspectives of the chessboard
fn main() {
    println!("Hello, world!");

    let mut chess = Chess::new();

    let mut player_one = Player::new("Alice", Color::WHITE);
    let mut player_two = Player::new("Bob", Color::BLACK);

    while chess.is_playing() {
        chess.board.display(&chess.get_pieces());

        player_one.move_piece(&mut chess);

        chess.board.display(&chess.get_pieces());

        player_two.move_piece(&mut chess);

        // println!("{:#?}", chess.get_pieces());
    }
}
