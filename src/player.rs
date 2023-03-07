use crate::piece::{Color, Piece, Point};

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub name: &'static str,
    pub pieces_yor: Color,
}

impl Player {
    pub fn new(_name: &'static str, _pieces_yor: Color) -> Player {
        Player { name: _name, pieces_yor: _pieces_yor }
    }

    pub fn move_piece(self, _piece: &mut Piece, _point: Point) {
        _piece.point.x = _point.x;
        _piece.point.y = _point.y;
    }

    // Use x notation: Pe4xPd5
    fn capture() {}

    // 0-0 if from the King side.
    // 0-0-0 if from the Queen side.
    fn castle() {}
}
