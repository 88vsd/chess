use crate::{piece::Piece, point::Point};

pub const COLS: usize = 8;
pub const ROWS: usize = 8;
pub const AMOUNT: usize = 16;
pub const PIECES_TOTAL_AMOUNT: usize = 32;

pub const ROW_1: usize = 7;
pub const ROW_2: usize = 6;
pub const ROW_3: usize = 5;
pub const ROW_4: usize = 4;
pub const ROW_5: usize = 3;
pub const ROW_6: usize = 2;
pub const ROW_7: usize = 1;
pub const ROW_8: usize = 0;

pub const ROWS_TOTAL_AMOUNT: usize = 8;
pub const COLUMNS_TOTAL_AMOUNT: usize = 8;

pub const COLUMN_A: usize = 0;
pub const COLUMN_B: usize = 1;
pub const COLUMN_C: usize = 2;
pub const COLUMN_D: usize = 3;
pub const COLUMN_E: usize = 4;
pub const COLUMN_F: usize = 5;
pub const COLUMN_G: usize = 6;
pub const COLUMN_H: usize = 7;

// Row 1
pub const A1: Point = Point { x: ROW_1, y: COLUMN_A };
pub const B1: Point = Point { x: ROW_1, y: COLUMN_B };
pub const C1: Point = Point { x: ROW_1, y: COLUMN_C };
pub const D1: Point = Point { x: ROW_1, y: COLUMN_D };
pub const E1: Point = Point { x: ROW_1, y: COLUMN_E };
pub const F1: Point = Point { x: ROW_1, y: COLUMN_F };
pub const G1: Point = Point { x: ROW_1, y: COLUMN_G };
pub const H1: Point = Point { x: ROW_1, y: COLUMN_H };
// Row 2
pub const A2: Point = Point { x: ROW_2, y: COLUMN_A };
pub const B2: Point = Point { x: ROW_2, y: COLUMN_B };
pub const C2: Point = Point { x: ROW_2, y: COLUMN_C };
pub const D2: Point = Point { x: ROW_2, y: COLUMN_D };
pub const E2: Point = Point { x: ROW_2, y: COLUMN_E };
pub const F2: Point = Point { x: ROW_2, y: COLUMN_F };
pub const G2: Point = Point { x: ROW_2, y: COLUMN_G };
pub const H2: Point = Point { x: ROW_2, y: COLUMN_H };
// Row 3
pub const A3: Point = Point { x: ROW_3, y: COLUMN_A };
pub const B3: Point = Point { x: ROW_3, y: COLUMN_B };
pub const C3: Point = Point { x: ROW_3, y: COLUMN_C };
pub const D3: Point = Point { x: ROW_3, y: COLUMN_D };
pub const E3: Point = Point { x: ROW_3, y: COLUMN_E };
pub const F3: Point = Point { x: ROW_3, y: COLUMN_F };
pub const G3: Point = Point { x: ROW_3, y: COLUMN_G };
pub const H3: Point = Point { x: ROW_3, y: COLUMN_H };
// Row 4
pub const A4: Point = Point { x: ROW_4, y: COLUMN_A };
pub const B4: Point = Point { x: ROW_4, y: COLUMN_B };
pub const C4: Point = Point { x: ROW_4, y: COLUMN_C };
pub const D4: Point = Point { x: ROW_4, y: COLUMN_D };
pub const E4: Point = Point { x: ROW_4, y: COLUMN_E };
pub const F4: Point = Point { x: ROW_4, y: COLUMN_F };
pub const G4: Point = Point { x: ROW_4, y: COLUMN_G };
pub const H4: Point = Point { x: ROW_4, y: COLUMN_H };
// Row 5
pub const A5: Point = Point { x: ROW_5, y: COLUMN_A };
pub const B5: Point = Point { x: ROW_5, y: COLUMN_B };
pub const C5: Point = Point { x: ROW_5, y: COLUMN_C };
pub const D5: Point = Point { x: ROW_5, y: COLUMN_D };
pub const E5: Point = Point { x: ROW_5, y: COLUMN_E };
pub const F5: Point = Point { x: ROW_5, y: COLUMN_F };
pub const G5: Point = Point { x: ROW_5, y: COLUMN_G };
pub const H5: Point = Point { x: ROW_5, y: COLUMN_H };
// Row 6
pub const A6: Point = Point { x: ROW_6, y: COLUMN_A };
pub const B6: Point = Point { x: ROW_6, y: COLUMN_B };
pub const C6: Point = Point { x: ROW_6, y: COLUMN_C };
pub const D6: Point = Point { x: ROW_6, y: COLUMN_D };
pub const E6: Point = Point { x: ROW_6, y: COLUMN_E };
pub const F6: Point = Point { x: ROW_6, y: COLUMN_F };
pub const G6: Point = Point { x: ROW_6, y: COLUMN_G };
pub const H6: Point = Point { x: ROW_6, y: COLUMN_H };
// Row 7
pub const A7: Point = Point { x: ROW_7, y: COLUMN_A };
pub const B7: Point = Point { x: ROW_7, y: COLUMN_B };
pub const C7: Point = Point { x: ROW_7, y: COLUMN_C };
pub const D7: Point = Point { x: ROW_7, y: COLUMN_D };
pub const E7: Point = Point { x: ROW_7, y: COLUMN_E };
pub const F7: Point = Point { x: ROW_7, y: COLUMN_F };
pub const G7: Point = Point { x: ROW_7, y: COLUMN_G };
pub const H7: Point = Point { x: ROW_7, y: COLUMN_H };
// Row 8
pub const A8: Point = Point { x: ROW_8, y: COLUMN_A };
pub const B8: Point = Point { x: ROW_8, y: COLUMN_B };
pub const C8: Point = Point { x: ROW_8, y: COLUMN_C };
pub const D8: Point = Point { x: ROW_8, y: COLUMN_D };
pub const E8: Point = Point { x: ROW_8, y: COLUMN_E };
pub const F8: Point = Point { x: ROW_8, y: COLUMN_F };
pub const G8: Point = Point { x: ROW_8, y: COLUMN_G };
pub const H8: Point = Point { x: ROW_8, y: COLUMN_H };
// Black Pawn|Rook|kNight|Bishop|Queen|King on Column A|B|C|D|E|F|G|H
pub const B_P_A: usize = 0;
pub const B_P_B: usize = 1;
pub const B_P_C: usize = 2;
pub const B_P_D: usize = 3;
pub const B_P_E: usize = 4;
pub const B_P_F: usize = 5;
pub const B_P_G: usize = 6;
pub const B_P_H: usize = 7;
pub const B_R_A: usize = 8;
pub const B_N_B: usize = 9;
pub const B_B_C: usize = 10;
pub const B_Q_D: usize = 11;
pub const B_K_E: usize = 12;
pub const B_B_F: usize = 13;
pub const B_N_G: usize = 14;
pub const B_R_H: usize = 15;
// White Pawn|Rook|kNight|Bishop|Queen|King on Column A|B|C|D|E|F|G|H
pub const W_P_A: usize = 16;
pub const W_P_B: usize = 17;
pub const W_P_C: usize = 18;
pub const W_P_D: usize = 19;
pub const W_P_E: usize = 20;
pub const W_P_F: usize = 21;
pub const W_P_G: usize = 22;
pub const W_P_H: usize = 23;
pub const W_R_A: usize = 24;
pub const W_N_B: usize = 25;
pub const W_B_C: usize = 26;
pub const W_Q_D: usize = 27;
pub const W_K_E: usize = 28;
pub const W_B_F: usize = 29;
pub const W_N_G: usize = 30;
pub const W_R_H: usize = 31;

pub type Pieces = [Piece; PIECES_TOTAL_AMOUNT];
