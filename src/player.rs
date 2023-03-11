use std::io;

use crate::{
    constants::Pieces,
    piece::{Color, Piece},
    point::Point,
};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub name: &'static str,
    pub pieces_color: Color,
}

impl Player {
    pub fn new(_name: &'static str, _pieces_color: Color) -> Player {
        Player { name: _name, pieces_color: _pieces_color }
    }

    pub fn move_piece(
        &mut self,
        _piece: &mut Piece,
        _point: Point,
        _pieces: &Pieces,
    ) {
        // let mut user_input = String::new();

        // io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        // let user_input = user_input.trim();

        let mut piece = _piece.clone();
        let available_moves = piece.moves(_pieces);

        if available_moves.contains(&_point) {
            if !piece.is_developed() {
                piece.set_is_developed();
            }

            *_piece = Piece::new(piece.color, piece.icon, piece.name, _point);
        } else {
            panic!("Provided move is not allowable!");
        }
    }
}
