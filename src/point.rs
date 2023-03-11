#[derive(Debug, Hash, Copy, Clone, PartialEq)]
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
}
