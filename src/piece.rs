use crate::board::Board;

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

#[derive(Debug, Hash, Copy, Clone)]
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

    pub fn down(&self) -> Point {
        Point { x: self.x + 1, y: self.y }
    }

    pub fn up(&self) -> Point {
        Point { x: self.x - 1, y: self.y }
    }

    pub fn left(&self) -> Point {
        Point { x: self.x, y: self.y - 1 }
    }

    pub fn right(&self) -> Point {
        Point { x: self.x, y: self.y + 1 }
    }

    pub fn up_left(&self) -> Point {
        Point { x: self.x - 1, y: self.y - 1 } // TODO: solve problem with usize underflow.
    }

    pub fn up_right(&self) -> Point {
        Point { x: self.x - 1, y: self.y + 1 }
    }

    pub fn down_left(&self) -> Point {
        Point { x: self.x + 1, y: self.y - 1 }
    }

    pub fn down_right(&self) -> Point {
        Point { x: self.x + 1, y: self.y + 1 }
    }
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub icon: &'static str,
    pub name: Name,
    pub point: Point,
}

trait Pawn {
    fn _get_pawn_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>>;
    fn _get_diagonal_moves(&self, _board: &Board, _moves: &mut Vec<Vec<usize>>);
    fn _get_one_step_move(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _get_two_step_move(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _add_move_if_peace_is_none_at(
        &self,
        _point: Point,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _add_move_if_piece_has_opposite_color_at(
        &self,
        _point: Point,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
}

trait Rook {
    fn _get_rook_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>>;
}

impl Pawn for Piece {
    fn _get_two_step_move(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        if self.is_black() {
            // Define whether a black pawn has been moved by the player. If so than disable the pawn two squares move.
            // Define whether a white pawn has been moved by the player. If so than disable the pawn two squares move.
            if self.is_undeveloped() {
                if _current_x < 6 {
                    let piece = _board.squares[_current_x + 2][_current_y];

                    if piece.is_none() {
                        if _board.squares[_current_x + 1][_current_y].is_none()
                        {
                            _moves.push(vec![_current_x + 2, _current_y]); // HERE
                        }
                    } else if piece.unwrap().color != self.color {
                        _moves.push(vec![_current_x + 2, _current_y]); // HERE
                    }
                }
            }
        }

        if self.is_white() {
            // Define whether a black pawn has been moved by the player. If so than disable the pawn two squares move.
            // Define whether a white pawn has been moved by the player. If so than disable the pawn two squares move.
            if self.is_undeveloped() {
                if _current_x > 1 {
                    let piece = _board.squares[_current_x - 2][_current_y];

                    if piece.is_none() {
                        if _board.squares[_current_x - 1][_current_y].is_none()
                        {
                            _moves.push(vec![_current_x - 2, _current_y]); //HERE
                        }
                    } else if piece.unwrap().color != self.color {
                        _moves.push(vec![_current_x - 2, _current_y]); // HERE
                    }
                }
            }
        }
    }

    fn _get_one_step_move(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        if self.is_black() {
            if !self.is_on_row_1() {
                self._add_move_if_peace_is_none_at(
                    self.point.down(),
                    _board,
                    _moves,
                );
            }
        }

        if self.is_white() {
            if !self.is_on_row_8() {
                self._add_move_if_peace_is_none_at(
                    self.point.up(),
                    _board,
                    _moves,
                );
            }
        }
    }

    fn _get_diagonal_moves(
        &self,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        if self.is_black() {
            if self.can_move_right() {
                self._add_move_if_piece_has_opposite_color_at(
                    self.point.down_right(),
                    _board,
                    _moves,
                );
            }

            if self.can_move_left() {
                self._add_move_if_piece_has_opposite_color_at(
                    self.point.down_left(),
                    _board,
                    _moves,
                );
            }
        }

        if self.is_white() {
            if self.can_move_right() {
                self._add_move_if_piece_has_opposite_color_at(
                    self.point.up_right(),
                    _board,
                    _moves,
                );
            }

            if self.can_move_right() {
                self._add_move_if_piece_has_opposite_color_at(
                    self.point.up_left(),
                    _board,
                    _moves,
                );
            }
        }
    }

    fn _get_pawn_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>> {
        // Instantiate a vector of all valid moves that apply to the piece.
        let mut moves = Vec::new();

        // Define a current x index of the piece on the board.
        // Define a current yumn index of the piece on the board.

        let current_x = self.point.x;
        let current_y = self.point.y;

        self._get_two_step_move(current_x, current_y, _board, &mut moves);
        self._get_one_step_move(current_x, current_y, _board, &mut moves);
        self._get_diagonal_moves(_board, &mut moves);

        moves
    }

    fn _add_move_if_piece_has_opposite_color_at(
        &self,
        _point: Point,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        let piece = Piece::get(_point, _board);

        if piece.is_some() {
            if piece.unwrap().color != self.color {
                _moves.push(vec![_point.x, _point.y]); // HERE
            }
        }
    }

    fn _add_move_if_peace_is_none_at(
        &self,
        _point: Point,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        let piece = Piece::get(_point, _board);

        if piece.is_none() {
            _moves.push(vec![_point.x, _point.y]); // HERE
        }
    }
}

impl Rook for Piece {
    fn _get_rook_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>> {
        let mut moves = Vec::new();

        let current_x = self.point.x;
        let current_y = self.point.y;

        // UP
        for x in (0..current_x).rev() {
            println!("{}", x);
            let piece = _board.squares[x][current_y];

            if piece.is_none() {
                moves.push(vec![x, current_y]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![x, current_y]);
                    break;
                } else {
                    break;
                }
            }
        }

        // DOWN
        for x in (current_x + 1)..8 {
            let piece = _board.squares[x][current_y];

            if piece.is_none() {
                moves.push(vec![x, current_y]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![x, current_y]);
                    break;
                } else {
                    break;
                }
            }
        }

        // LEFT
        for y in (0..current_y).rev() {
            let piece = _board.squares[current_x][y];

            if piece.is_none() {
                moves.push(vec![y, current_x]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![current_x, y]);
                    break;
                } else {
                    break;
                }
            }
        }

        // RIGHT
        for y in (current_y + 1)..8 {
            let piece = _board.squares[current_x][y];

            if piece.is_none() {
                moves.push(vec![current_x, y]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![current_x, y]);
                    break;
                } else {
                    break;
                }
            }
        }

        moves
    }
}

impl Piece {
    pub fn new(
        color: Color,
        icon: &'static str,
        name: Name,
        point: Point,
    ) -> Self {
        Piece { color, icon, name, point }
    }

    pub fn get_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>> {
        match self.name {
            Name::PAWN => return self._get_pawn_valid_moves(_board),
            Name::ROOK => return self._get_rook_valid_moves(_board),
            Name::KNIGHT => return self._get_rook_valid_moves(_board),
            Name::BISHOP => return self._get_rook_valid_moves(_board),
            Name::QUEEN => return self._get_rook_valid_moves(_board),
            Name::KING => return self._get_rook_valid_moves(_board),
        }
    }

    pub fn get(_point: Point, _board: &Board) -> Option<Piece> {
        _board.squares[_point.x][_point.y]
    }

    pub fn is_black(&self) -> bool {
        self.color == Color::BLACK
    }

    pub fn is_white(&self) -> bool {
        self.color == Color::WHITE
    }

    pub fn can_move_left(&self) -> bool {
        self.point.y > 0
    }

    pub fn can_move_right(&self) -> bool {
        self.point.y < 7
    }

    pub fn is_on_row_1(&self) -> bool {
        //self.point.x < 7 // TODO: try using ==
        self.point.x == 7
    }

    pub fn is_on_row_8(&self) -> bool {
        // self.point.x > 0 // TODO: try using ==
        self.point.x == 0 // TODO: try using ==
    }

    pub fn is_undeveloped(&self) -> bool {
        const BLACK_PAWN_STARTING_POINT: usize = 1;
        const WHITE_PAWN_STARTING_POINT: usize = 6;

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

    pub fn initialize_black_pieces() -> [Piece; 16] {
        let pieces = [
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 0 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 1 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 2 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 3 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 4 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 5 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 6 }),
            Piece::new(Color::BLACK, "♟", Name::PAWN, Point { x: 1, y: 7 }),
            Piece::new(Color::BLACK, "♜", Name::ROOK, Point { x: 0, y: 0 }),
            Piece::new(Color::BLACK, "♞", Name::KNIGHT, Point { x: 0, y: 1 }),
            Piece::new(Color::BLACK, "♝", Name::BISHOP, Point { x: 0, y: 2 }),
            Piece::new(Color::BLACK, "♛", Name::QUEEN, Point { x: 0, y: 3 }),
            Piece::new(Color::BLACK, "♚", Name::KING, Point { x: 0, y: 4 }),
            Piece::new(Color::BLACK, "♝", Name::BISHOP, Point { x: 0, y: 5 }),
            Piece::new(Color::BLACK, "♞", Name::KNIGHT, Point { x: 0, y: 6 }),
            Piece::new(Color::BLACK, "♜", Name::ROOK, Point { x: 0, y: 7 }),
        ];

        pieces
    }

    pub fn initialize_white_pieces() -> [Piece; 16] {
        let pieces = [
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 0 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 1 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 2 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 3 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 4 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 5 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 6 }),
            Piece::new(Color::WHITE, "♙", Name::PAWN, Point { x: 6, y: 7 }),
            Piece::new(Color::WHITE, "♖", Name::ROOK, Point { x: 7, y: 0 }),
            Piece::new(Color::WHITE, "♘", Name::KNIGHT, Point { x: 7, y: 1 }),
            Piece::new(Color::WHITE, "♗", Name::BISHOP, Point { x: 7, y: 2 }),
            Piece::new(Color::WHITE, "♕", Name::QUEEN, Point { x: 7, y: 3 }),
            Piece::new(Color::WHITE, "♔", Name::KING, Point { x: 7, y: 4 }),
            Piece::new(Color::WHITE, "♗", Name::BISHOP, Point { x: 7, y: 5 }),
            Piece::new(Color::WHITE, "♘", Name::KNIGHT, Point { x: 7, y: 6 }),
            Piece::new(Color::WHITE, "♖", Name::ROOK, Point { x: 7, y: 7 }),
        ];

        pieces
    }
}
