use crate::{
    constants::{
        A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2,
        C3, C4, C5, C6, C7, C8, D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4,
        E5, E6, E7, E8, F1, F2, F3, F4, F5, F6, F7, F8, G1, G2, G3, G4, G5, G6,
        G7, G8, H1, H2, H3, H4, H5, H6, H7, H8, PIECES_TOTAL_AMOUNT,
    },
    piece::Piece,
    point::Point,
};

#[derive(Debug, Clone)]
pub struct Board;

impl Board {
    pub fn new(_pieces: &[Piece; PIECES_TOTAL_AMOUNT]) -> Self {
        Board
    }

    pub fn display(&self, _pieces: &[Piece; PIECES_TOTAL_AMOUNT]) {
        // Row 8
        let a8 = self._get_icon(_pieces, A8);
        let b8 = self._get_icon(_pieces, B8);
        let c8 = self._get_icon(_pieces, C8);
        let d8 = self._get_icon(_pieces, D8);
        let e8 = self._get_icon(_pieces, E8);
        let f8 = self._get_icon(_pieces, F8);
        let g8 = self._get_icon(_pieces, G8);
        let h8 = self._get_icon(_pieces, H8);
        // Row 7
        let a7 = self._get_icon(_pieces, A7);
        let b7 = self._get_icon(_pieces, B7);
        let c7 = self._get_icon(_pieces, C7);
        let d7 = self._get_icon(_pieces, D7);
        let e7 = self._get_icon(_pieces, E7);
        let f7 = self._get_icon(_pieces, F7);
        let g7 = self._get_icon(_pieces, G7);
        let h7 = self._get_icon(_pieces, H7);
        // Row 6
        let a6 = self._get_icon(_pieces, A6);
        let b6 = self._get_icon(_pieces, B6);
        let c6 = self._get_icon(_pieces, C6);
        let d6 = self._get_icon(_pieces, D6);
        let e6 = self._get_icon(_pieces, E6);
        let f6 = self._get_icon(_pieces, F6);
        let g6 = self._get_icon(_pieces, G6);
        let h6 = self._get_icon(_pieces, H6);
        // Row 5
        let a5 = self._get_icon(_pieces, A5);
        let b5 = self._get_icon(_pieces, B5);
        let c5 = self._get_icon(_pieces, C5);
        let d5 = self._get_icon(_pieces, D5);
        let e5 = self._get_icon(_pieces, E5);
        let f5 = self._get_icon(_pieces, F5);
        let g5 = self._get_icon(_pieces, G5);
        let h5 = self._get_icon(_pieces, H5);
        // Row 4
        let a4 = self._get_icon(_pieces, A4);
        let b4 = self._get_icon(_pieces, B4);
        let c4 = self._get_icon(_pieces, C4);
        let d4 = self._get_icon(_pieces, D4);
        let e4 = self._get_icon(_pieces, E4);
        let f4 = self._get_icon(_pieces, F4);
        let g4 = self._get_icon(_pieces, G4);
        let h4 = self._get_icon(_pieces, H4);
        // Row 3
        let a3 = self._get_icon(_pieces, A3);
        let b3 = self._get_icon(_pieces, B3);
        let c3 = self._get_icon(_pieces, C3);
        let d3 = self._get_icon(_pieces, D3);
        let e3 = self._get_icon(_pieces, E3);
        let f3 = self._get_icon(_pieces, F3);
        let g3 = self._get_icon(_pieces, G3);
        let h3 = self._get_icon(_pieces, H3);
        // Row 2
        let a2 = self._get_icon(_pieces, A2);
        let b2 = self._get_icon(_pieces, B2);
        let c2 = self._get_icon(_pieces, C2);
        let d2 = self._get_icon(_pieces, D2);
        let e2 = self._get_icon(_pieces, E2);
        let f2 = self._get_icon(_pieces, F2);
        let g2 = self._get_icon(_pieces, G2);
        let h2 = self._get_icon(_pieces, H2);
        // Row 1
        let a1 = self._get_icon(_pieces, A1);
        let b1 = self._get_icon(_pieces, B1);
        let c1 = self._get_icon(_pieces, C1);
        let d1 = self._get_icon(_pieces, D1);
        let e1 = self._get_icon(_pieces, E1);
        let f1 = self._get_icon(_pieces, F1);
        let g1 = self._get_icon(_pieces, G1);
        let h1 = self._get_icon(_pieces, H1);

        println!(
            r#"
           a b c d e f g h                 h g f e d c b a

        8 |{a8}|{b8}|{c8}|{d8}|{e8}|{f8}|{g8}|{h8}| 8           1 |{h1}|{g1}|{f1}|{e1}|{d1}|{c1}|{b1}|{a1}| 1
        7 |{a7}|{b7}|{c7}|{d7}|{e7}|{f7}|{g7}|{h7}| 7           2 |{h2}|{g2}|{f2}|{e2}|{d2}|{c2}|{b2}|{a2}| 2
        6 |{a6}|{b6}|{c6}|{d6}|{e6}|{f6}|{g6}|{h6}| 6           3 |{h3}|{g3}|{f3}|{e3}|{d3}|{c3}|{b3}|{a3}| 3
        5 |{a5}|{b5}|{c5}|{d5}|{e5}|{f5}|{g5}|{h5}| 5           4 |{h4}|{g4}|{f4}|{e4}|{d4}|{c4}|{b4}|{a4}| 4
        4 |{a4}|{b4}|{c4}|{d4}|{e4}|{f4}|{g4}|{h4}| 4           5 |{h5}|{g5}|{f5}|{e5}|{d5}|{c5}|{b5}|{a5}| 5
        3 |{a3}|{b3}|{c3}|{d3}|{e3}|{f3}|{g3}|{h3}| 3           6 |{h6}|{g6}|{f6}|{e6}|{d6}|{c6}|{b6}|{a6}| 6
        2 |{a2}|{b2}|{c2}|{d2}|{e2}|{f2}|{g2}|{h2}| 2           7 |{h7}|{g7}|{f7}|{e7}|{d7}|{c7}|{b7}|{a7}| 7
        1 |{a1}|{b1}|{c1}|{d1}|{e1}|{f1}|{g1}|{h1}| 1           8 |{h8}|{g8}|{f8}|{e8}|{d8}|{c8}|{b8}|{a8}| 8

           a b c d e f g h                 h g f e d c b a
    "#
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
