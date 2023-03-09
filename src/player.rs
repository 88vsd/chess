use crate::piece::{Color, Piece, Point};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub name: &'static str,
    pub pieces_color: Color,
}

impl Player {
    pub fn new(_name: &'static str, _pieces_color: Color) -> Player {
        Player { name: _name, pieces_color: _pieces_color }
    }

    pub fn move_piece(&mut self, _piece: &mut Option<Piece>, _point: Point) {
        let piece = _piece.clone().unwrap();

        *_piece = Some(Piece::new(piece.color, piece.icon, piece.name, _point));
    }
}
