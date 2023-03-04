use crate::piece::{Color, Piece, Position};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub pieces_color: Color,
}

impl Player {
    pub fn new(_name: &'static str, _pieces_color: Color) -> Player {
        Player { name: _name, pieces_color: _pieces_color }
    }

    pub fn move_piece(self, _piece: &mut Piece, _position: Position) {
        _piece.position.row = _position.row;
        _piece.position.col = _position.col;
    }
}
