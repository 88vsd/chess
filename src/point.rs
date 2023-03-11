use crate::constants::{
    A1, A2, A3, A4, A5, A6, A7, A8, B1, B2, B3, B4, B5, B6, B7, B8, C1, C2, C3,
    C4, C5, C6, C7, C8, D1, D2, D3, D4, D5, D6, D7, D8, E1, E2, E3, E4, E5, E6,
    E7, E8, F1, F2, F3, F4, F5, F6, F7, F8, G1, G2, G3, G4, G5, G6, G7, G8, H1,
    H2, H3, H4, H5, H6, H7, H8,
};

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct Point {
    /// Row
    pub x: usize,
    /// Column
    pub y: usize,
}

impl Point {
    pub fn new(_x: usize, _y: usize) -> Point {
        Point { x: _x, y: _y }
    }

    pub fn down(&self, _n: usize) -> Point {
        Point { x: self.x + _n, y: self.y }
    }

    pub fn up(&self, _n: usize) -> Point {
        Point { x: self.x - _n, y: self.y }
    }

    pub fn left(&self, _n: usize) -> Point {
        Point { x: self.x, y: self.y - _n }
    }

    pub fn right(&self, _n: usize) -> Point {
        Point { x: self.x, y: self.y + _n }
    }

    pub fn up_left(&self, _n: usize) -> Point {
        Point { x: self.x - _n, y: self.y - _n }
    }

    pub fn up_right(&self, _n: usize) -> Point {
        Point { x: self.x - _n, y: self.y + _n }
    }

    pub fn down_left(&self, _n: usize) -> Point {
        Point { x: self.x + _n, y: self.y - _n }
    }

    pub fn down_right(&self, _n: usize) -> Point {
        Point { x: self.x + _n, y: self.y + _n }
    }

    pub fn up_up_left(&self) -> Point {
        Point { x: self.x - 2, y: self.y - 1 }
    }

    pub fn up_up_right(&self) -> Point {
        Point { x: self.x - 2, y: self.y + 1 }
    }

    pub fn down_down_left(&self) -> Point {
        Point { x: self.x + 2, y: self.y - 1 }
    }

    pub fn down_down_right(&self) -> Point {
        Point { x: self.x + 2, y: self.y + 1 }
    }

    pub fn up_left_left(&self) -> Point {
        Point { x: self.x - 1, y: self.y - 2 }
    }

    pub fn up_right_right(&self) -> Point {
        Point { x: self.x - 1, y: self.y + 2 }
    }

    pub fn down_left_left(&self) -> Point {
        Point { x: self.x + 1, y: self.y - 2 }
    }

    pub fn down_right_right(&self) -> Point {
        Point { x: self.x + 1, y: self.y + 2 }
    }

    pub fn get_str(_point: Point) -> &'static str {
        match _point {
            // Row 1
            A1 => "A1",
            B1 => "B1",
            C1 => "C1",
            D1 => "D1",
            E1 => "E1",
            F1 => "F1",
            G1 => "G1",
            H1 => "H1",
            // Row 2
            A2 => "A2",
            B2 => "B2",
            C2 => "C2",
            D2 => "D2",
            E2 => "E2",
            F2 => "F2",
            G2 => "G2",
            H2 => "H2",
            // Row 3
            A3 => "A3",
            B3 => "B3",
            C3 => "C3",
            D3 => "D3",
            E3 => "E3",
            F3 => "F3",
            G3 => "G3",
            H3 => "H3",
            // Row 4
            A4 => "A4",
            B4 => "B4",
            C4 => "C4",
            D4 => "D4",
            E4 => "E4",
            F4 => "F4",
            G4 => "G4",
            H4 => "H4",
            // Row 5
            A5 => "A5",
            B5 => "B5",
            C5 => "C5",
            D5 => "D5",
            E5 => "E5",
            F5 => "F5",
            G5 => "G5",
            H5 => "H5",
            // Row 6
            A6 => "A6",
            B6 => "B6",
            C6 => "C6",
            D6 => "D6",
            E6 => "E6",
            F6 => "F6",
            G6 => "G6",
            H6 => "H6",
            // Row 7
            A7 => "A7",
            B7 => "B7",
            C7 => "C7",
            D7 => "D7",
            E7 => "E7",
            F7 => "F7",
            G7 => "G7",
            H7 => "H7",
            // Row 8
            A8 => "A8",
            B8 => "B8",
            C8 => "C8",
            D8 => "D8",
            E8 => "E8",
            F8 => "F8",
            G8 => "G8",
            H8 => "H8",
            _ => panic!("Error while getting square string."),
        }
    }

    pub fn get(_square: &str) -> Point {
        match _square {
            // Row 1
            "A1" => A1,
            "B1" => B1,
            "C1" => C1,
            "D1" => D1,
            "E1" => E1,
            "F1" => F1,
            "G1" => G1,
            "H1" => H1,
            // Row 2
            "A2" => A2,
            "B2" => B2,
            "C2" => C2,
            "D2" => D2,
            "E2" => E2,
            "F2" => F2,
            "G2" => G2,
            "H2" => H2,
            // Row 3
            "A3" => A3,
            "B3" => B3,
            "C3" => C3,
            "D3" => D3,
            "E3" => E3,
            "F3" => F3,
            "G3" => G3,
            "H3" => H3,
            // Row 4
            "A4" => A4,
            "B4" => B4,
            "C4" => C4,
            "D4" => D4,
            "E4" => E4,
            "F4" => F4,
            "G4" => G4,
            "H4" => H4,
            // Row 5
            "A5" => A5,
            "B5" => B5,
            "C5" => C5,
            "D5" => D5,
            "E5" => E5,
            "F5" => F5,
            "G5" => G5,
            "H5" => H5,
            // Row 6
            "A6" => A6,
            "B6" => B6,
            "C6" => C6,
            "D6" => D6,
            "E6" => E6,
            "F6" => F6,
            "G6" => G6,
            "H6" => H6,
            // Row 7
            "A7" => A7,
            "B7" => B7,
            "C7" => C7,
            "D7" => D7,
            "E7" => E7,
            "F7" => F7,
            "G7" => G7,
            "H7" => H7,
            // Row 8
            "A8" => A8,
            "B8" => B8,
            "C8" => C8,
            "D8" => D8,
            "E8" => E8,
            "F8" => F8,
            "G8" => G8,
            "H8" => H8,
            _ => panic!("Error while getting point."),
        }
    }
}
