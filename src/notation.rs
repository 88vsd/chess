use crate::{
    constants::E4,
    piece::{Color, Piece},
    point::Point,
};

pub struct Notation<'a>(
    pub &'a str,
    pub Point,
    bool,
    bool,
    pub Point,
    bool,
    bool,
);

impl Notation<'_> {
    pub fn new(_str_value: &str, _piece_color: Color) -> Notation {
        let piece_letter = &_str_value[0..1];

        let piece_icon = Piece::get_icon(piece_letter, _piece_color);

        let is_accupying_move = !_str_value.contains("x");
        let is_capturing_move = _str_value.contains("x");
        let is_checkmate_move = _str_value.contains("#");
        let is_check_move = _str_value.contains("+");

        let starting_point_str = &_str_value[1..2];
        let starting_point = Point::get("E4");

        let ending_point_str: &str;
        let ending_point: Point;

        if is_capturing_move {
            ending_point_str = &_str_value[5..6];
            ending_point = Point::get("F6");
        } else {
            ending_point_str = &_str_value[4..5];
            ending_point = Point::get("F6");
        }

        Notation(
            piece_icon,
            starting_point,
            is_accupying_move,
            is_capturing_move,
            ending_point,
            is_check_move,
            is_checkmate_move,
        )
    }
}
