use crate::{
    piece::{Color, Piece},
    point::Point,
};

pub struct Notation<'a> {
    pub piece_icon: &'a str,
    pub starting_point: Point,
    pub is_accupying_move: bool,
    pub is_capturing_move: bool,
    pub ending_point: Point,
    pub is_check_move: bool,
    pub is_checkmate_move: bool,
}

impl Notation<'_> {
    pub fn new(_str_value: &str, _piece_color: Color) -> Notation {
        let piece_letter = &_str_value[0..1];

        let piece_icon = Piece::get_icon(piece_letter, _piece_color);

        let is_accupying_move = !_str_value.contains("x");
        let is_capturing_move = _str_value.contains("x");
        let is_checkmate_move = _str_value.contains("#");
        let is_check_move = _str_value.contains("+");

        let starting_point_str = &_str_value[1..3];
        let starting_point = Point::get(starting_point_str);

        let ending_point_str: &str;
        let ending_point: Point;

        if is_capturing_move {
            ending_point_str = &_str_value[4..6];
            ending_point = Point::get(ending_point_str);
        } else {
            ending_point_str = &_str_value[3..5];
            ending_point = Point::get(ending_point_str);
        }

        Notation {
            piece_icon,
            starting_point,
            is_accupying_move,
            is_capturing_move,
            ending_point,
            is_check_move,
            is_checkmate_move,
        }
    }
}
