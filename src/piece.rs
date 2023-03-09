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

#[derive(Debug, Hash, Clone)]
pub struct Piece {
    pub color: Color,
    pub icon: &'static str,
    // pub is_developed: bool,
    pub name: Name,
    pub point: Point,
    pub allowable_moves: Vec<Point>,
}

impl Piece {
    pub fn new(
        color: Color,
        icon: &'static str,
        name: Name,
        point: Point,
    ) -> Self {
        Piece {
            color,
            icon,
            name,
            point,
            allowable_moves: Vec::new(),
        }
    }

    // pub fn moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     match self.name {
    //         Name::PAWN => self._get_pawn_valid_moves(_board),
    //         Name::BISHOP => self._get_bishop_valid_moves(_board),
    //         Name::KING => self._get_king_valid_moves(_board),
    //         Name::KNIGHT => self._get_knight_valid_moves(_board),
    //         Name::QUEEN => self._get_queen_valid_moves(_board),
    //         Name::ROOK => self._get_rook_valid_moves(_board),
    //     }
    // }

    // pub fn get(_point: Point) -> Option<Piece> {
    //     _board.squares[_point.x][_point.y].clone()
    // }

    // pub fn is_at(_point: Point, _board: &Board) -> bool {
    //     let piece = Piece::get(_point, _board);

    //     if piece.is_some() {
    //         true
    //     } else {
    //         false
    //     }
    // }

    // pub fn is_piece(&self, _point: Point, _board: &Board) -> bool {
    //     Piece::get(_point, _board).is_some()
    // }

    // pub fn is_opponent(self: &Piece, _point: Point, _board: &Board) -> bool {
    //     let piece = Piece::get(_point, _board);

    //     if piece.is_some() {
    //         piece.unwrap().color != self.color
    //     } else {
    //         false
    //     }
    // }

    // pub fn is_personal(_self: &Piece, _point: Point, _board: &Board) -> bool {
    //     let piece = Piece::get(_point, _board);

    //     if piece.is_some() {
    //         piece.unwrap().color == _self.color
    //     } else {
    //         false
    //     }
    // }

    // pub fn is_black(&self) -> bool {
    //     self.color == Color::BLACK
    // }

    // pub fn is_white(&self) -> bool {
    //     self.color == Color::WHITE
    // }

    // pub fn is_pawn(&self) -> bool {
    //     self.name == Name::PAWN
    // }

    // pub fn is_king(&self) -> bool {
    //     self.name == Name::KING
    // }

    // pub fn can_move_up(&self) -> bool {
    //     self.point.x > ROW_8
    // }

    // pub fn can_move_up_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x >= _step
    //         && !Piece::is_personal(self, self.point.up(_step), _board)
    // }

    // pub fn can_move_down_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x + _step <= ROW_1
    //         && !Piece::is_personal(self, self.point.down(_step), _board)
    // }

    // pub fn can_move_left_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.y >= _step
    //         && !Piece::is_personal(self, self.point.left(_step), _board)
    // }

    // pub fn can_move_right_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.y + _step <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.right(_step), _board)
    // }

    // pub fn can_move_up_right_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x >= _step
    //         && self.point.y + _step <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.up_right(_step), _board)
    // }

    // pub fn can_move_up_left_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x >= _step
    //         && self.point.y >= _step
    //         && !Piece::is_personal(self, self.point.up_left(_step), _board)
    // }

    // pub fn can_move_down_right_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x + _step <= ROW_1
    //         && self.point.y + _step <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.down_right(_step), _board)
    // }

    // pub fn can_move_down_left_by(&self, _step: usize, _board: &Board) -> bool {
    //     self.point.x + _step <= ROW_1
    //         && self.point.y >= _step
    //         && !Piece::is_personal(self, self.point.down_left(_step), _board)
    // }

    // pub fn can_move_up_up_left(&self, _board: &Board) -> bool {
    //     self.point.x >= 2
    //         && self.point.y >= 1
    //         && !Piece::is_personal(self, self.point.up_up_left(), _board)
    // }

    // pub fn can_move_up_up_right(&self, _board: &Board) -> bool {
    //     self.point.x >= 2
    //         && self.point.y + 1 <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.up_up_right(), _board)
    // }

    // pub fn can_move_down_down_left(&self, _board: &Board) -> bool {
    //     self.point.x + 2 <= ROW_1
    //         && self.point.y >= 1
    //         && !Piece::is_personal(self, self.point.down_down_left(), _board)
    // }

    // pub fn can_move_down_down_right(&self, _board: &Board) -> bool {
    //     self.point.x + 2 <= ROW_1
    //         && self.point.y + 1 <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.down_down_right(), _board)
    // }

    // pub fn can_move_up_left_left(&self, _board: &Board) -> bool {
    //     self.point.x >= 1
    //         && self.point.y >= 2
    //         && !Piece::is_personal(self, self.point.up_left_left(), _board)
    // }

    // pub fn can_move_up_right_right(&self, _board: &Board) -> bool {
    //     self.point.x >= 1
    //         && self.point.y + 2 <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.up_right_right(), _board)
    // }

    // pub fn can_move_down_left_left(&self, _board: &Board) -> bool {
    //     self.point.x + 1 <= ROW_1
    //         && self.point.y >= 2
    //         && !Piece::is_personal(self, self.point.down_left_left(), _board)
    // }

    // pub fn can_move_down_right_right(&self, _board: &Board) -> bool {
    //     self.point.x + 1 <= ROW_1
    //         && self.point.y + 2 <= COLUMN_H
    //         && !Piece::is_personal(self, self.point.down_right_right(), _board)
    // }

    // pub fn is_on_column_a(&self) -> bool {
    //     self.point.y == COLUMN_A
    // }

    // pub fn is_on_column_h(&self) -> bool {
    //     self.point.y == COLUMN_H
    // }

    // pub fn is_on_row_1(&self) -> bool {
    //     self.point.x == ROW_1
    // }

    // pub fn is_on_row_8(&self) -> bool {
    //     // self.point.x > 0 // TODO: try using ==
    //     self.point.x == ROW_8 // TODO: try using ==
    // }

    // pub fn is_undeveloped(&self) -> bool {
    //     const BLACK_PAWN_STARTING_POINT: usize = ROW_7;
    //     const WHITE_PAWN_STARTING_POINT: usize = ROW_2;

    //     match self.name {
    //         Name::PAWN => match self.color {
    //             Color::BLACK => self.point.x == BLACK_PAWN_STARTING_POINT,
    //             Color::WHITE => self.point.x == WHITE_PAWN_STARTING_POINT,
    //         },
    //         Name::BISHOP => todo!(),
    //         Name::KING => todo!(),
    //         Name::KNIGHT => todo!(),
    //         Name::QUEEN => todo!(),
    //         Name::ROOK => todo!(),
    //     }
    // }

    // pub fn initialize_black_pieces() -> [Piece; 16] {
    //     let pieces = [
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, A7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, B7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, C7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, D7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, E7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, F7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, G7),
    //         Piece::new(Color::BLACK, "♟", Name::PAWN, H7),
    //         Piece::new(Color::BLACK, "♜", Name::ROOK, A8),
    //         Piece::new(Color::BLACK, "♞", Name::KNIGHT, B8),
    //         Piece::new(Color::BLACK, "♝", Name::BISHOP, C8),
    //         Piece::new(Color::BLACK, "♛", Name::QUEEN, D8),
    //         Piece::new(Color::BLACK, "♚", Name::KING, E8),
    //         Piece::new(Color::BLACK, "♝", Name::BISHOP, F8),
    //         Piece::new(Color::BLACK, "♞", Name::KNIGHT, G8),
    //         Piece::new(Color::BLACK, "♜", Name::ROOK, H8),
    //     ];

    //     pieces
    // }

    // pub fn initialize_white_pieces() -> [Piece; 16] {
    //     let pieces = [
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, A2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, B2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, C2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, D2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, E2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, F2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, G2),
    //         Piece::new(Color::WHITE, "♙", Name::PAWN, H2),
    //         Piece::new(Color::WHITE, "♖", Name::ROOK, A1),
    //         Piece::new(Color::WHITE, "♘", Name::KNIGHT, B1),
    //         Piece::new(Color::WHITE, "♗", Name::BISHOP, C1),
    //         Piece::new(Color::WHITE, "♕", Name::QUEEN, D1),
    //         Piece::new(Color::WHITE, "♔", Name::KING, E1),
    //         Piece::new(Color::WHITE, "♗", Name::BISHOP, F1),
    //         Piece::new(Color::WHITE, "♘", Name::KNIGHT, G1),
    //         Piece::new(Color::WHITE, "♖", Name::ROOK, H1),
    //     ];

    //     pieces
    // }

    // fn _add_capture_move(&mut self, _point: Point, _board: &Board) -> bool {
    //     let piece = Piece::get(_point, _board);

    //     if piece.is_some() {
    //         if piece.unwrap().color != self.color {
    //             self._add_allowable_move(_point);

    //             return true;
    //         } else {
    //             // Returns true if the piece belongs to the same color.
    //             return true;
    //         }
    //     }

    //     return false;
    // }

    // fn _add_allowable_move(&mut self, _point: Point) {
    //     self.allowable_moves.push(_point);
    // }

    // fn _add_occupy_move(&mut self, _point: Point, _board: &Board) {
    //     let piece = Piece::get(_point, _board);

    //     if piece.is_none() {
    //         self._add_allowable_move(_point);
    //     }
    // }

    // fn _add_move(&mut self, _point: Point, _board: &Board) -> bool {
    //     if !Piece::is_at(_point, _board) {
    //         self._add_occupy_move(_point, _board);

    //         // Return true to stop iteration as king can go only 1 step.
    //         if self.is_king() {
    //             return true;
    //         }

    //         return false;
    //     } else {
    //         if Piece::is_personal(self, _point, _board) {
    //             return true;
    //         }

    //         if Piece::is_opponent(self, _point, _board) {
    //             self._add_capture_move(_point, _board);

    //             return true;
    //         }
    //     }

    //     panic!("Not able to validate move.");
    // }

    // fn _add_up_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_up_by(steps, _board) {
    //         let point = self.point.up(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_down_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_down_by(steps, _board) {
    //         let point = self.point.down(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_left_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_left_by(steps, _board) {
    //         let point = self.point.left(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_right_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_right_by(steps, _board) {
    //         let point = self.point.right(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_up_right_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_up_right_by(steps, _board) {
    //         let point = self.point.up_right(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_up_left_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_up_left_by(steps, _board) {
    //         let point = self.point.up_left(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_down_right_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_down_right_by(steps, _board) {
    //         let point = self.point.down_right(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _add_down_left_moves(&mut self, _board: &Board) {
    //     let mut steps = 1;

    //     while self.can_move_down_left_by(steps, _board) {
    //         let point = self.point.down_left(steps);

    //         if self._add_move(point, _board) {
    //             break;
    //         }

    //         steps += 1;
    //     }
    // }

    // fn _get_rook_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     self._add_up_moves(_board);
    //     self._add_down_moves(_board);
    //     self._add_left_moves(_board);
    //     self._add_right_moves(_board);

    //     &self.allowable_moves
    // }

    // fn _get_bishop_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     self._add_up_right_moves(_board);
    //     self._add_up_left_moves(_board);
    //     self._add_down_right_moves(_board);
    //     self._add_down_left_moves(_board);

    //     &self.allowable_moves
    // }

    // fn _get_queen_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     self._add_up_moves(_board);
    //     self._add_down_moves(_board);
    //     self._add_left_moves(_board);
    //     self._add_right_moves(_board);

    //     self._add_up_right_moves(_board);
    //     self._add_up_left_moves(_board);
    //     self._add_down_right_moves(_board);
    //     self._add_down_left_moves(_board);

    //     &self.allowable_moves
    // }

    // fn _get_king_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     self._add_up_moves(_board);
    //     self._add_down_moves(_board);
    //     self._add_left_moves(_board);
    //     self._add_right_moves(_board);

    //     self._add_up_right_moves(_board);
    //     self._add_up_left_moves(_board);
    //     self._add_down_right_moves(_board);
    //     self._add_down_left_moves(_board);

    //     &self.allowable_moves
    // }

    // fn _get_pawn_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     if self.is_black() {
    //         if self.can_move_down_left_by(1, _board) {
    //             self._add_capture_move(self.point.down_left(1), _board);
    //         }

    //         if self.can_move_down_right_by(1, _board) {
    //             self._add_capture_move(self.point.down_right(1), _board);
    //         }

    //         if !Piece::is_at(self.point.down(1), _board) {
    //             self._add_occupy_move(self.point.down(1), _board);
    //         }

    //         if self.is_undeveloped()
    //             && !Piece::is_at(self.point.down(2), _board)
    //         {
    //             self._add_occupy_move(self.point.down(2), _board);
    //         }
    //     }

    //     if self.is_white() {
    //         if self.can_move_up_left_by(1, _board) {
    //             self._add_capture_move(self.point.up_left(1), _board);
    //         }

    //         if self.can_move_up_right_by(1, _board) {
    //             self._add_capture_move(self.point.up_right(1), _board);
    //         }

    //         if !Piece::is_at(self.point.up(1), _board) {
    //             self._add_occupy_move(self.point.up(1), _board);
    //         }

    //         if self.is_undeveloped() && !Piece::is_at(self.point.up(2), _board)
    //         {
    //             self._add_occupy_move(self.point.up(2), _board);
    //         }
    //     }

    //     &self.allowable_moves
    // }

    // fn _get_knight_valid_moves(&mut self, _board: &Board) -> &Vec<Point> {
    //     // -.
    //     //  |
    //     //  |
    //     if self.can_move_up_up_left(_board) {
    //         self._add_move(self.point.up_up_left(), _board);
    //     }

    //     //  .-
    //     //  |
    //     //  |
    //     if self.can_move_up_up_right(_board) {
    //         self._add_move(self.point.up_up_right(), _board);
    //     }

    //     //  |
    //     //  |
    //     // -.
    //     if self.can_move_down_down_left(_board) {
    //         self._add_move(self.point.down_down_left(), _board);
    //     }

    //     //  |
    //     //  |
    //     //  .-
    //     if self.can_move_down_down_right(_board) {
    //         self._add_move(self.point.down_down_right(), _board);
    //     }

    //     //  ------.
    //     //        |
    //     if self.can_move_up_left_left(_board) {
    //         self._add_move(self.point.up_left_left(), _board);
    //     }

    //     //  .------
    //     //  |
    //     if self.can_move_up_right_right(_board) {
    //         self._add_move(self.point.up_right_right(), _board);
    //     }

    //     //       |
    //     // ------.
    //     if self.can_move_down_left_left(_board) {
    //         self._add_move(self.point.down_left_left(), _board);
    //     }

    //     // |
    //     // .------
    //     if self.can_move_down_right_right(_board) {
    //         self._add_move(self.point.down_right_right(), _board);
    //     }

    //     &self.allowable_moves
    // }
}
