use crate::{
    board::Board,
    piece::{Color, Piece, Point},
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

    let player_one = Player::new("Alice", Color::BLACK);
    let mut player_two = Player::new("Bob", Color::WHITE);

    board.display(&black_pieces, &white_pieces);

    // player_one.move_piece(&mut black_pieces[8], Point { x: 5, y: 0 });
    // player_one.move_piece(&mut black_pieces[10], Point { x: 3, y: 2 });
    // player_one.move_piece(&mut black_pieces[11], Point { x: 3, y: 6 });
    player_one.move_piece(&mut black_pieces[0], Point { x: 3, y: 5 });
    // player_two.move_piece(&mut white_pieces[0], Point { x: 5, y: 6 });
    // player_two.move_piece(&mut white_pieces[8], Point { x: 5, y: 5 });
    // player_two.move_piece(&mut white_pieces[12], Point { x: 7, y: 0 });
    // player_one.move_piece(&mut black_pieces[1], Point { x: 3, y: 1 });
    // player_two.move_piece(&mut white_pieces[1], Point { x: 4, y: 1 });

    // println!("{:?}", black_pieces[0].position);
    board.update(&black_pieces, &white_pieces);
    board.display(&black_pieces, &white_pieces);

    // let rook_valid_moves = &black_pieces[8].moves(&board);
    // let bishop_valid_moves = &black_pieces[10].moves(&board);
    // let queen_valid_moves = &black_pieces[11].moves(&board);
    // let king_valid_moves = &white_pieces[12].moves(&board);
    // println!("{:?}", rook_valid_moves);
    // println!("{:?}", bishop_valid_moves);
    // println!("{:?}", queen_valid_moves);
    // println!("{:?}", king_valid_moves);
    // let pawn_2_valid_moves = white_pieces[1].moves(&board); //get_valid_moves(&board);
    let pawn_1_valid_moves = black_pieces[0].moves(&board);
    // println!("{:?}", pawn_2_valid_moves);
    // println!("{:?}", pawn_1_valid_moves);
    println!("{:?}", white_pieces[5].moves(&board));
    // println!("{:?}", black_pieces[1].moves(&board));

    // board.update(&black_pieces, &white_pieces);
    // board.display(&black_pieces, &white_pieces);

    // let game = Game::new(player_one, player_two);

    // game.play();
}
