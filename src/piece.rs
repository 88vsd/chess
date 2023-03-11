use crate::{
    constants::{Pieces, COLUMN_A, COLUMN_H, ROW_1, ROW_2, ROW_7, ROW_8},
    point::Point,
};

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

#[derive(Debug, Hash, Clone)]
pub struct Piece {
    pub color: Color,
    pub icon: &'static str,
    pub is_captured: bool,
    pub is_developed: bool,
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
            is_captured: false,
            is_developed: false,
            name,
            point,
            allowable_moves: Vec::new(),
        }
    }

    pub fn set_point(&mut self, _point: Point, _pieces: &Pieces) {
        let allowable_moves = self.moves(_pieces);

        if allowable_moves.contains(&_point) {
            if !self.is_developed() {
                self.set_is_developed();
            }

            self.point = _point;
        } else {
            println!("You can't move to {}", Point::get_str(_point));
        }
    }

    pub fn moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        match self.name {
            Name::PAWN => self._get_pawn_valid_moves(_pieces),
            Name::BISHOP => self._get_bishop_valid_moves(_pieces),
            Name::KING => self._get_king_valid_moves(_pieces),
            Name::KNIGHT => self._get_knight_valid_moves(_pieces),
            Name::QUEEN => self._get_queen_valid_moves(_pieces),
            Name::ROOK => self._get_rook_valid_moves(_pieces),
        }
    }

    pub fn get(_point: Point, _pieces: &Pieces) -> Option<&Piece> {
        // _pieces.iter().find(|piece| piece.point == point)
        _pieces.iter().find(|piece| piece.point == _point).map(|piece| piece)
    }

    pub fn get_icon(_piece_letter: &str, _piece_color: Color) -> &str {
        if _piece_color == Color::BLACK {
            match _piece_letter {
                "K" => "♚",
                "Q" => "♛",
                "R" => "♜",
                "B" => "♝",
                "N" => "♞",
                "P" => "♟",
                _ => panic!("Invalid piece"),
            }
        } else {
            match _piece_letter {
                "K" => "♔",
                "Q" => "♕",
                "R" => "♖",
                "B" => "♗",
                "N" => "♘",
                "P" => "♙",
                _ => panic!("Invalid piece"),
            }
        }
    }

    pub fn is_at(_point: Point, _pieces: &Pieces) -> bool {
        let piece = Piece::get(_point, _pieces);

        if piece.is_some() {
            true
        } else {
            false
        }
    }

    pub fn is_piece(&self, _point: Point, _pieces: &Pieces) -> bool {
        Piece::get(_point, _pieces).is_some()
    }

    pub fn is_opponent(self: &Piece, _point: Point, _pieces: &Pieces) -> bool {
        let piece = Piece::get(_point, _pieces);

        if piece.is_some() {
            piece.unwrap().color != self.color
        } else {
            false
        }
    }

    pub fn is_personal(_self: &Piece, _point: Point, _pieces: &Pieces) -> bool {
        let piece = Piece::get(_point, _pieces);

        if piece.is_some() {
            piece.unwrap().color == _self.color
        } else {
            false
        }
    }

    pub fn is_black(&self) -> bool {
        self.color == Color::BLACK
    }

    pub fn is_white(&self) -> bool {
        self.color == Color::WHITE
    }

    pub fn is_pawn(&self) -> bool {
        self.name == Name::PAWN
    }

    pub fn is_king(&self) -> bool {
        self.name == Name::KING
    }

    pub fn can_move_up(&self) -> bool {
        self.point.x > ROW_8
    }

    pub fn can_move_up_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.x >= _step
            && !Piece::is_personal(self, self.point.up(_step), _pieces)
    }

    pub fn can_move_down_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.x + _step <= ROW_1
            && !Piece::is_personal(self, self.point.down(_step), _pieces)
    }

    pub fn can_move_left_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.y >= _step
            && !Piece::is_personal(self, self.point.left(_step), _pieces)
    }

    pub fn can_move_right_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.y + _step <= COLUMN_H
            && !Piece::is_personal(self, self.point.right(_step), _pieces)
    }

    pub fn can_move_up_right_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.x >= _step
            && self.point.y + _step <= COLUMN_H
            && !Piece::is_personal(self, self.point.up_right(_step), _pieces)
    }

    pub fn can_move_up_left_by(&self, _step: usize, _pieces: &Pieces) -> bool {
        self.point.x >= _step
            && self.point.y >= _step
            && !Piece::is_personal(self, self.point.up_left(_step), _pieces)
    }

    pub fn can_move_down_right_by(
        &self,
        _step: usize,
        _pieces: &Pieces,
    ) -> bool {
        self.point.x + _step <= ROW_1
            && self.point.y + _step <= COLUMN_H
            && !Piece::is_personal(self, self.point.down_right(_step), _pieces)
    }

    pub fn can_move_down_left_by(
        &self,
        _step: usize,
        _pieces: &Pieces,
    ) -> bool {
        self.point.x + _step <= ROW_1
            && self.point.y >= _step
            && !Piece::is_personal(self, self.point.down_left(_step), _pieces)
    }

    pub fn can_move_up_up_left(&self, _pieces: &Pieces) -> bool {
        self.point.x >= 2
            && self.point.y >= 1
            && !Piece::is_personal(self, self.point.up_up_left(), _pieces)
    }

    pub fn can_move_up_up_right(&self, _pieces: &Pieces) -> bool {
        self.point.x >= 2
            && self.point.y + 1 <= COLUMN_H
            && !Piece::is_personal(self, self.point.up_up_right(), _pieces)
    }

    pub fn can_move_down_down_left(&self, _pieces: &Pieces) -> bool {
        self.point.x + 2 <= ROW_1
            && self.point.y >= 1
            && !Piece::is_personal(self, self.point.down_down_left(), _pieces)
    }

    pub fn can_move_down_down_right(&self, _pieces: &Pieces) -> bool {
        self.point.x + 2 <= ROW_1
            && self.point.y + 1 <= COLUMN_H
            && !Piece::is_personal(self, self.point.down_down_right(), _pieces)
    }

    pub fn can_move_up_left_left(&self, _pieces: &Pieces) -> bool {
        self.point.x >= 1
            && self.point.y >= 2
            && !Piece::is_personal(self, self.point.up_left_left(), _pieces)
    }

    pub fn can_move_up_right_right(&self, _pieces: &Pieces) -> bool {
        self.point.x >= 1
            && self.point.y + 2 <= COLUMN_H
            && !Piece::is_personal(self, self.point.up_right_right(), _pieces)
    }

    pub fn can_move_down_left_left(&self, _pieces: &Pieces) -> bool {
        self.point.x + 1 <= ROW_1
            && self.point.y >= 2
            && !Piece::is_personal(self, self.point.down_left_left(), _pieces)
    }

    pub fn can_move_down_right_right(&self, _pieces: &Pieces) -> bool {
        self.point.x + 1 <= ROW_1
            && self.point.y + 2 <= COLUMN_H
            && !Piece::is_personal(self, self.point.down_right_right(), _pieces)
    }

    pub fn is_on_column_a(&self) -> bool {
        self.point.y == COLUMN_A
    }

    pub fn is_on_column_h(&self) -> bool {
        self.point.y == COLUMN_H
    }

    pub fn is_on_row_1(&self) -> bool {
        self.point.x == ROW_1
    }

    pub fn is_on_row_8(&self) -> bool {
        // self.point.x > 0 // TODO: try using ==
        self.point.x == ROW_8 // TODO: try using ==
    }

    pub fn is_developed(&self) -> bool {
        self.is_developed
    }

    pub fn is_undeveloped(&self) -> bool {
        const BLACK_PAWN_STARTING_POINT: usize = ROW_7;
        const WHITE_PAWN_STARTING_POINT: usize = ROW_2;

        match self.name {
            Name::PAWN => match self.color {
                Color::BLACK => self.point.x == BLACK_PAWN_STARTING_POINT,
                Color::WHITE => self.point.x == WHITE_PAWN_STARTING_POINT,
            },
            Name::BISHOP => todo!(),
            Name::KING => todo!(),
            Name::KNIGHT => todo!(),
            Name::QUEEN => todo!(),
            Name::ROOK => todo!(),
        }
    }

    pub fn set_is_developed(&mut self) {
        self.is_developed = true;
    }

    fn _add_capture_move(&mut self, _point: Point, _pieces: &Pieces) -> bool {
        let piece = Piece::get(_point, _pieces);

        if piece.is_some() {
            if piece.unwrap().color != self.color {
                self._add_allowable_move(_point);

                return true;
            } else {
                // Returns true if the piece belongs to the same color.
                return true;
            }
        }

        return false;
    }

    fn _add_allowable_move(&mut self, _point: Point) {
        self.allowable_moves.push(_point);
    }

    fn _add_occupy_move(&mut self, _point: Point, _pieces: &Pieces) {
        let piece = Piece::get(_point, _pieces);

        if piece.is_none() {
            self._add_allowable_move(_point);
        }
    }

    fn _add_move(&mut self, _point: Point, _pieces: &Pieces) -> bool {
        if !Piece::is_at(_point, _pieces) {
            self._add_occupy_move(_point, _pieces);

            // Return true to stop iteration as king can go only 1 step.
            if self.is_king() {
                return true;
            }

            return false;
        } else {
            if Piece::is_personal(self, _point, _pieces) {
                return true;
            }

            if Piece::is_opponent(self, _point, _pieces) {
                self._add_capture_move(_point, _pieces);

                return true;
            }
        }

        panic!("Not able to validate move.");
    }

    fn _add_up_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_up_by(steps, _pieces) {
            let point = self.point.up(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_down_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_down_by(steps, _pieces) {
            let point = self.point.down(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_left_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_left_by(steps, _pieces) {
            let point = self.point.left(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_right_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_right_by(steps, _pieces) {
            let point = self.point.right(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_up_right_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_up_right_by(steps, _pieces) {
            let point = self.point.up_right(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_up_left_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_up_left_by(steps, _pieces) {
            let point = self.point.up_left(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_down_right_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_down_right_by(steps, _pieces) {
            let point = self.point.down_right(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _add_down_left_moves(&mut self, _pieces: &Pieces) {
        let mut steps = 1;

        while self.can_move_down_left_by(steps, _pieces) {
            let point = self.point.down_left(steps);

            if self._add_move(point, _pieces) {
                break;
            }

            steps += 1;
        }
    }

    fn _get_rook_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        self._add_up_moves(_pieces);
        self._add_down_moves(_pieces);
        self._add_left_moves(_pieces);
        self._add_right_moves(_pieces);

        &self.allowable_moves
    }

    fn _get_bishop_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        self._add_up_right_moves(_pieces);
        self._add_up_left_moves(_pieces);
        self._add_down_right_moves(_pieces);
        self._add_down_left_moves(_pieces);

        &self.allowable_moves
    }

    fn _get_queen_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        self._add_up_moves(_pieces);
        self._add_down_moves(_pieces);
        self._add_left_moves(_pieces);
        self._add_right_moves(_pieces);

        self._add_up_right_moves(_pieces);
        self._add_up_left_moves(_pieces);
        self._add_down_right_moves(_pieces);
        self._add_down_left_moves(_pieces);

        &self.allowable_moves
    }

    fn _get_king_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        self._add_up_moves(_pieces);
        self._add_down_moves(_pieces);
        self._add_left_moves(_pieces);
        self._add_right_moves(_pieces);

        self._add_up_right_moves(_pieces);
        self._add_up_left_moves(_pieces);
        self._add_down_right_moves(_pieces);
        self._add_down_left_moves(_pieces);

        &self.allowable_moves
    }

    fn _get_pawn_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        if self.is_black() {
            if self.can_move_down_left_by(1, _pieces) {
                self._add_capture_move(self.point.down_left(1), _pieces);
            }

            if self.can_move_down_right_by(1, _pieces) {
                self._add_capture_move(self.point.down_right(1), _pieces);
            }

            if !Piece::is_at(self.point.down(1), _pieces) {
                self._add_occupy_move(self.point.down(1), _pieces);
            }

            if !self.is_developed()
                && !Piece::is_at(self.point.down(2), _pieces)
            {
                self._add_occupy_move(self.point.down(2), _pieces);
            }
        }

        if self.is_white() {
            if self.can_move_up_left_by(1, _pieces) {
                self._add_capture_move(self.point.up_left(1), _pieces);
            }

            if self.can_move_up_right_by(1, _pieces) {
                self._add_capture_move(self.point.up_right(1), _pieces);
            }

            if !Piece::is_at(self.point.up(1), _pieces) {
                self._add_occupy_move(self.point.up(1), _pieces);
            }

            if !self.is_developed() && !Piece::is_at(self.point.up(2), _pieces)
            {
                self._add_occupy_move(self.point.up(2), _pieces);
            }
        }

        &self.allowable_moves
    }

    fn _get_knight_valid_moves(&mut self, _pieces: &Pieces) -> &Vec<Point> {
        // -.
        //  |
        //  |
        if self.can_move_up_up_left(_pieces) {
            self._add_move(self.point.up_up_left(), _pieces);
        }

        //  .-
        //  |
        //  |
        if self.can_move_up_up_right(_pieces) {
            self._add_move(self.point.up_up_right(), _pieces);
        }

        //  |
        //  |
        // -.
        if self.can_move_down_down_left(_pieces) {
            self._add_move(self.point.down_down_left(), _pieces);
        }

        //  |
        //  |
        //  .-
        if self.can_move_down_down_right(_pieces) {
            self._add_move(self.point.down_down_right(), _pieces);
        }

        //  ------.
        //        |
        if self.can_move_up_left_left(_pieces) {
            self._add_move(self.point.up_left_left(), _pieces);
        }

        //  .------
        //  |
        if self.can_move_up_right_right(_pieces) {
            self._add_move(self.point.up_right_right(), _pieces);
        }

        //       |
        // ------.
        if self.can_move_down_left_left(_pieces) {
            self._add_move(self.point.down_left_left(), _pieces);
        }

        // |
        // .------
        if self.can_move_down_right_right(_pieces) {
            self._add_move(self.point.down_right_right(), _pieces);
        }

        &self.allowable_moves
    }
}
