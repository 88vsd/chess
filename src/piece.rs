use std::fmt::Display;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Color {
    BLACK,
    WHITE,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Name {
    BISHOP,
    KING,
    KNIGHT,
    PAWN,
    QUEEN,
    ROOK,
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub icon: &'static str,
    pub name: Name,
    pub position: Position,
}

pub trait PieceTrait {
    fn new(
        color: Color,
        icon: &'static str,
        name: Name,
        position: Position,
    ) -> Self;
    //fn icon(&self) -> &'static str;
    //fn color(&self) -> &'static Color;
}

impl PieceTrait for Piece {
    fn new(
        color: Color,
        icon: &'static str,
        name: Name,
        position: Position,
    ) -> Self {
        Piece { color, icon, name, position }
    }
}
