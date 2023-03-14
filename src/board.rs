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
        let wa8 = self._get_icon(_pieces, A8);
        let wb8 = self._get_icon(_pieces, B8);
        let wc8 = self._get_icon(_pieces, C8);
        let wd8 = self._get_icon(_pieces, D8);
        let we8 = self._get_icon(_pieces, E8);
        let wf8 = self._get_icon(_pieces, F8);
        let wg8 = self._get_icon(_pieces, G8);
        let wh8 = self._get_icon(_pieces, H8);
        // Row 7
        let wa7 = self._get_icon(_pieces, A7);
        let wb7 = self._get_icon(_pieces, B7);
        let wc7 = self._get_icon(_pieces, C7);
        let wd7 = self._get_icon(_pieces, D7);
        let we7 = self._get_icon(_pieces, E7);
        let wf7 = self._get_icon(_pieces, F7);
        let wg7 = self._get_icon(_pieces, G7);
        let wh7 = self._get_icon(_pieces, H7);
        // Row 6
        let wa6 = self._get_icon(_pieces, A6);
        let wb6 = self._get_icon(_pieces, B6);
        let wc6 = self._get_icon(_pieces, C6);
        let wd6 = self._get_icon(_pieces, D6);
        let we6 = self._get_icon(_pieces, E6);
        let wf6 = self._get_icon(_pieces, F6);
        let wg6 = self._get_icon(_pieces, G6);
        let wh6 = self._get_icon(_pieces, H6);
        // Row 5
        let wa5 = self._get_icon(_pieces, A5);
        let wb5 = self._get_icon(_pieces, B5);
        let wc5 = self._get_icon(_pieces, C5);
        let wd5 = self._get_icon(_pieces, D5);
        let we5 = self._get_icon(_pieces, E5);
        let wf5 = self._get_icon(_pieces, F5);
        let wg5 = self._get_icon(_pieces, G5);
        let wh5 = self._get_icon(_pieces, H5);
        // Row 4
        let wa4 = self._get_icon(_pieces, A4);
        let wb4 = self._get_icon(_pieces, B4);
        let wc4 = self._get_icon(_pieces, C4);
        let wd4 = self._get_icon(_pieces, D4);
        let we4 = self._get_icon(_pieces, E4);
        let wf4 = self._get_icon(_pieces, F4);
        let wg4 = self._get_icon(_pieces, G4);
        let wh4 = self._get_icon(_pieces, H4);
        // Row 3
        let wa3 = self._get_icon(_pieces, A3);
        let wb3 = self._get_icon(_pieces, B3);
        let wc3 = self._get_icon(_pieces, C3);
        let wd3 = self._get_icon(_pieces, D3);
        let we3 = self._get_icon(_pieces, E3);
        let wf3 = self._get_icon(_pieces, F3);
        let wg3 = self._get_icon(_pieces, G3);
        let wh3 = self._get_icon(_pieces, H3);
        // Row 2
        let wa2 = self._get_icon(_pieces, A2);
        let wb2 = self._get_icon(_pieces, B2);
        let wc2 = self._get_icon(_pieces, C2);
        let wd2 = self._get_icon(_pieces, D2);
        let we2 = self._get_icon(_pieces, E2);
        let wf2 = self._get_icon(_pieces, F2);
        let wg2 = self._get_icon(_pieces, G2);
        let wh2 = self._get_icon(_pieces, H2);
        // Row 1
        let wa1 = self._get_icon(_pieces, A1);
        let wb1 = self._get_icon(_pieces, B1);
        let wc1 = self._get_icon(_pieces, C1);
        let wd1 = self._get_icon(_pieces, D1);
        let we1 = self._get_icon(_pieces, E1);
        let wf1 = self._get_icon(_pieces, F1);
        let wg1 = self._get_icon(_pieces, G1);
        let wh1 = self._get_icon(_pieces, H1);

        // Row 1
        let ba1 = self._get_icon(_pieces, A1);
        let bb1 = self._get_icon(_pieces, B1);
        let bc1 = self._get_icon(_pieces, C1);
        let bd1 = self._get_icon(_pieces, D1);
        let be1 = self._get_icon(_pieces, E1);
        let bf1 = self._get_icon(_pieces, F1);
        let bg1 = self._get_icon(_pieces, G1);
        let bh1 = self._get_icon(_pieces, H1);
        // Row 2
        let ba2 = self._get_icon(_pieces, A2);
        let bb2 = self._get_icon(_pieces, B2);
        let bc2 = self._get_icon(_pieces, C2);
        let bd2 = self._get_icon(_pieces, D2);
        let be2 = self._get_icon(_pieces, E2);
        let bf2 = self._get_icon(_pieces, F2);
        let bg2 = self._get_icon(_pieces, G2);
        let bh2 = self._get_icon(_pieces, H2);
        // Row 3
        let ba3 = self._get_icon(_pieces, A3);
        let bb3 = self._get_icon(_pieces, B3);
        let bc3 = self._get_icon(_pieces, C3);
        let bd3 = self._get_icon(_pieces, D3);
        let be3 = self._get_icon(_pieces, E3);
        let bf3 = self._get_icon(_pieces, F3);
        let bg3 = self._get_icon(_pieces, G3);
        let bh3 = self._get_icon(_pieces, H3);
        // Row 4
        let ba4 = self._get_icon(_pieces, A4);
        let bb4 = self._get_icon(_pieces, B4);
        let bc4 = self._get_icon(_pieces, C4);
        let bd4 = self._get_icon(_pieces, D4);
        let be4 = self._get_icon(_pieces, E4);
        let bf4 = self._get_icon(_pieces, F4);
        let bg4 = self._get_icon(_pieces, G4);
        let bh4 = self._get_icon(_pieces, H4);
        // Row 5
        let ba5 = self._get_icon(_pieces, A5);
        let bb5 = self._get_icon(_pieces, B5);
        let bc5 = self._get_icon(_pieces, C5);
        let bd5 = self._get_icon(_pieces, D5);
        let be5 = self._get_icon(_pieces, E5);
        let bf5 = self._get_icon(_pieces, F5);
        let bg5 = self._get_icon(_pieces, G5);
        let bh5 = self._get_icon(_pieces, H5);
        // Row 6
        let ba6 = self._get_icon(_pieces, A6);
        let bb6 = self._get_icon(_pieces, B6);
        let bc6 = self._get_icon(_pieces, C6);
        let bd6 = self._get_icon(_pieces, D6);
        let be6 = self._get_icon(_pieces, E6);
        let bf6 = self._get_icon(_pieces, F6);
        let bg6 = self._get_icon(_pieces, G6);
        let bh6 = self._get_icon(_pieces, H6);
        // Row 7
        let ba7 = self._get_icon(_pieces, A7);
        let bb7 = self._get_icon(_pieces, B7);
        let bc7 = self._get_icon(_pieces, C7);
        let bd7 = self._get_icon(_pieces, D7);
        let be7 = self._get_icon(_pieces, E7);
        let bf7 = self._get_icon(_pieces, F7);
        let bg7 = self._get_icon(_pieces, G7);
        let bh7 = self._get_icon(_pieces, H7);
        // Row 8
        let ba8 = self._get_icon(_pieces, A8);
        let bb8 = self._get_icon(_pieces, B8);
        let bc8 = self._get_icon(_pieces, C8);
        let bd8 = self._get_icon(_pieces, D8);
        let be8 = self._get_icon(_pieces, E8);
        let bf8 = self._get_icon(_pieces, F8);
        let bg8 = self._get_icon(_pieces, G8);
        let bh8 = self._get_icon(_pieces, H8);

        println!(
            r#"
           a b c d e f g h                 h g f e d c b a

        8 |{wa8}|{wb8}|{wc8}|{wd8}|{we8}|{wf8}|{wg8}|{wh8}| 8           1 |{bh1}|{bg1}|{bf1}|{be1}|{bd1}|{bc1}|{bb1}|{ba1}| 1
        7 |{wa7}|{wb7}|{wc7}|{wd7}|{we7}|{wf7}|{wg7}|{wh7}| 7           2 |{bh2}|{bg2}|{bf2}|{be2}|{bd2}|{bc2}|{bb2}|{ba2}| 2
        6 |{wa6}|{wb6}|{wc6}|{wd6}|{we6}|{wf6}|{wg6}|{wh6}| 6           3 |{bh3}|{bg3}|{bf3}|{be3}|{bd3}|{bc3}|{bb3}|{ba3}| 3
        5 |{wa5}|{wb5}|{wc5}|{wd5}|{we5}|{wf5}|{wg5}|{wh5}| 5           4 |{bh4}|{bg4}|{bf4}|{be4}|{bd4}|{bc4}|{bb4}|{ba4}| 4
        4 |{wa4}|{wb4}|{wc4}|{wd4}|{we4}|{wf4}|{wg4}|{wh4}| 4           5 |{bh5}|{bg5}|{bf5}|{be5}|{bd5}|{bc5}|{bb5}|{ba5}| 5
        3 |{wa3}|{wb3}|{wc3}|{wd3}|{we3}|{wf3}|{wg3}|{wh3}| 3           6 |{bh6}|{bg6}|{bf6}|{be6}|{bd6}|{bc6}|{bb6}|{ba6}| 6
        2 |{wa2}|{wb2}|{wc2}|{wd2}|{we2}|{wf2}|{wg2}|{wh2}| 2           7 |{bh7}|{bg7}|{bf7}|{be7}|{bd7}|{bc7}|{bb7}|{ba7}| 7
        1 |{wa1}|{wb1}|{wc1}|{wd1}|{we1}|{wf1}|{wg1}|{wh1}| 1           8 |{bh8}|{bg8}|{bf8}|{be8}|{bd8}|{bc8}|{bb8}|{ba8}| 8

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
