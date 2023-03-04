use crate::piece::Color;

pub struct Player {
    pub name: &'static str,
    pub pieces_color: Color,
}

impl Player {
    pub fn new(_name: &'static str, _pieces_color: Color) -> Player {
        Player { name: _name, pieces_color: _pieces_color }
    }
}
