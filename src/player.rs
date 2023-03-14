use std::io;

use crate::{
    chess::Chess, constants::Pieces, notation::Notation, piece::Color,
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

    pub fn move_piece(&mut self, _chess: &mut Chess) {
        println!(
            "Player {} moves a {:?} piece...",
            self.name, self.pieces_color
        );

        let notation = self._get_notation();

        if notation.is_capturing_move {
            let captured_piece = &mut _chess.get_piece(notation.ending_point);
            captured_piece.set_is_captured();
        }

        let pieces: &Pieces = &_chess.get_pieces().clone();

        let piece = &mut _chess.get_piece(notation.starting_point);

        piece.set_point(notation.ending_point, pieces);
    }

    fn _get_notation(&self) -> Notation {
        println!("Provide a move notation: ");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Failed to read line.");

        let user_input = user_input.trim().to_string();
        // println!("{}", user_input);

        let notation = Notation::new(user_input, self.pieces_color);

        notation
    }
}
