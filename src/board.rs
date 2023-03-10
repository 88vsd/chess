use crate::{
    constants::{
        A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2,
        C3, C4, C5, C6, C7, C8, D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4,
        E5, E6, E7, E8, F1, F2, F3, F4, F5, F6, F7, F8, G1, G2, G3, G4, G5, G6,
        G7, G8, H1, H2, H3, H4, H5, H6, H7, H8,
    },
    piece::{Piece, Point},
};

#[derive(Debug, Clone)]
pub struct Board;

impl Board {
    pub fn new(_pieces: &[Piece; 32]) -> Self {
        Board
    }

    pub fn display(&self, _pieces: &[Piece; 32]) {
        println!(
            r#"
           a b c d e f g h

        8 |{}|{}|{}|{}|{}|{}|{}|{}| 8
        7 |{}|{}|{}|{}|{}|{}|{}|{}| 7
        6 |{}|{}|{}|{}|{}|{}|{}|{}| 6
        5 |{}|{}|{}|{}|{}|{}|{}|{}| 5
        4 |{}|{}|{}|{}|{}|{}|{}|{}| 4
        3 |{}|{}|{}|{}|{}|{}|{}|{}| 3
        2 |{}|{}|{}|{}|{}|{}|{}|{}| 2
        1 |{}|{}|{}|{}|{}|{}|{}|{}| 1

           a b c d e f g h
    "#,
            // Row 8
            self._get_icon(_pieces, A8),
            self._get_icon(_pieces, B8),
            self._get_icon(_pieces, C8),
            self._get_icon(_pieces, D8),
            self._get_icon(_pieces, E8),
            self._get_icon(_pieces, F8),
            self._get_icon(_pieces, G8),
            self._get_icon(_pieces, H8),
            // Row 7
            self._get_icon(_pieces, A7),
            self._get_icon(_pieces, B7),
            self._get_icon(_pieces, C7),
            self._get_icon(_pieces, D7),
            self._get_icon(_pieces, E7),
            self._get_icon(_pieces, F7),
            self._get_icon(_pieces, G7),
            self._get_icon(_pieces, H7),
            // Row 6
            self._get_icon(_pieces, A6),
            self._get_icon(_pieces, B6),
            self._get_icon(_pieces, C6),
            self._get_icon(_pieces, D6),
            self._get_icon(_pieces, E6),
            self._get_icon(_pieces, F6),
            self._get_icon(_pieces, G6),
            self._get_icon(_pieces, H6),
            // Row 5
            self._get_icon(_pieces, A5),
            self._get_icon(_pieces, B5),
            self._get_icon(_pieces, C5),
            self._get_icon(_pieces, D5),
            self._get_icon(_pieces, E5),
            self._get_icon(_pieces, F5),
            self._get_icon(_pieces, G5),
            self._get_icon(_pieces, H5),
            // Row 4
            self._get_icon(_pieces, A4),
            self._get_icon(_pieces, B4),
            self._get_icon(_pieces, C4),
            self._get_icon(_pieces, D4),
            self._get_icon(_pieces, E4),
            self._get_icon(_pieces, F4),
            self._get_icon(_pieces, G4),
            self._get_icon(_pieces, H4),
            // Row 3
            self._get_icon(_pieces, A3),
            self._get_icon(_pieces, B3),
            self._get_icon(_pieces, C3),
            self._get_icon(_pieces, D3),
            self._get_icon(_pieces, E3),
            self._get_icon(_pieces, F3),
            self._get_icon(_pieces, G3),
            self._get_icon(_pieces, H3),
            // Row 2
            self._get_icon(_pieces, A2),
            self._get_icon(_pieces, B2),
            self._get_icon(_pieces, C2),
            self._get_icon(_pieces, D2),
            self._get_icon(_pieces, E2),
            self._get_icon(_pieces, F2),
            self._get_icon(_pieces, G2),
            self._get_icon(_pieces, H2),
            // Row 1
            self._get_icon(_pieces, A1),
            self._get_icon(_pieces, B1),
            self._get_icon(_pieces, C1),
            self._get_icon(_pieces, D1),
            self._get_icon(_pieces, E1),
            self._get_icon(_pieces, F1),
            self._get_icon(_pieces, G1),
            self._get_icon(_pieces, H1),
        );
    }

    fn _get_icon(&self, _pieces: &[Piece; 32], _point: Point) -> &str {
        _pieces
            .iter()
            .find_map(|piece| {
                if piece.point == _point {
                    Some(piece.icon)
                } else {
                    None
                }
            })
            .unwrap_or(" ")
    }
}
