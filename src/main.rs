use crate::{
    board::Board,
    piece::{Color, Piece, Position},
    player::Player,
};

pub mod board;
pub mod piece;
pub mod player;

fn main() {
    println!("Hello, world!");

    let mut black_pieces = Piece::initialize_black_pieces();
    let mut white_pieces = Piece::initialize_white_pieces();

    let mut board = Board::new(&black_pieces, &white_pieces);

    let mut player_one = Player::new("Alice", Color::BLACK);
    let mut player_two = Player::new("Bob", Color::WHITE);

    board.display(&black_pieces, &white_pieces);

    player_one.move_piece(&mut black_pieces[0], Position { row: 4, col: 4 });

    println!("{:?}", black_pieces[0].position);

    board.update(&black_pieces, &white_pieces);
    board.display(&black_pieces, &white_pieces);

    // let game = Game::new(player_one, player_two);

    // game.play();
}
