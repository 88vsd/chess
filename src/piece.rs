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

#[derive(Debug, Hash, Copy, Clone)]
pub struct Piece {
    pub yor: Color,
    pub icon: &'static str,
    pub name: Name,
    pub point: Point,
}

trait Pawn {
    fn _get_pawn_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>>;
    fn _get_diagonal_moves(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
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
    fn _get_piece(&self, _x: usize, _y: usize, _board: &Board)
        -> Option<Piece>;
    fn _is_black_pawn(&self) -> bool;
    fn _is_white_pawn(&self) -> bool;
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
        // Define whether a black pawn has been moved by the player. If so than disable the pawn two squares move.
        // Define whether a white pawn has been moved by the player. If so than disable the pawn two squares move.

        let is_undeveloped_black_pawn = _current_x == 1;
        let is_undeveloped_white_pawn = _current_x == 6;

        if self._is_black_pawn() {
            if is_undeveloped_black_pawn {
                if _current_x < 6 {
                    let piece = _board.squares[_current_x + 2][_current_y];

                    if piece.is_none() {
                        if _board.squares[_current_x + 1][_current_y].is_none()
                        {
                            _moves.push(vec![_current_x + 2, _current_y]); // HERE
                        }
                    } else if piece.unwrap().yor != self.yor {
                        _moves.push(vec![_current_x + 2, _current_y]); // HERE
                    }
                }
            }
        }

        if self._is_white_pawn() {
            if is_undeveloped_white_pawn {
                if _current_x > 1 {
                    let piece = _board.squares[_current_x - 2][_current_y];

                    if piece.is_none() {
                        if _board.squares[_current_x - 1][_current_y].is_none()
                        {
                            _moves.push(vec![_current_x - 2, _current_y]); //HERE
                        }
                    } else if piece.unwrap().yor != self.yor {
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
        // Define the last x index for black pawns in order to perform the "pawn promotion".
        // Define the last x index for white pawns in order to perform the "pawn promotion".

        const LAST_ROW_FOR_BLACK_PAWN: usize = 7;
        const LAST_ROW_FOR_WHITE_PAWN: usize = 0;

        if self._is_black_pawn() {
            if _current_x < LAST_ROW_FOR_BLACK_PAWN {
                let piece = _board.squares[_current_x + 1][_current_y];

                if piece.is_none() {
                    _moves.push(vec![_current_x + 1, _current_y]); // HERE
                }
            }
        }

        if self._is_white_pawn() {
            if _current_x > LAST_ROW_FOR_WHITE_PAWN {
                let piece = _board.squares[_current_x - 1][_current_y];

                if piece.is_none() {
                    _moves.push(vec![_current_x - 1, _current_y]); // HERE
                }
            }
        }
    }

    fn _get_diagonal_moves(
        &self,
        _current_x: usize,
        _current_y: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        let up = _current_x - 1;
        let down = _current_x + 1;
        let left = if _current_y != 0 {
            _current_y - 1
        } else {
            _current_y
        }; // TODO: solve issue with underflow.
        let right = _current_y + 1;

        let can_move_left = _current_y > 0;
        let can_move_right = _current_y < 7;

        if self._is_black_pawn() {
            // First we check for the down-right side.
            if can_move_right {
                let piece = _board.squares[down][right];

                if piece.is_some() {
                    if piece.unwrap().yor != self.yor {
                        _moves.push(vec![down, right]); // HERE
                    }
                }
            }

            // Then we check for the down-left side.
            if can_move_left {
                let piece = _board.squares[down][left];

                if piece.is_some() {
                    if piece.unwrap().yor != self.yor {
                        _moves.push(vec![down, left]); // HERE
                    }
                }
            }
        }

        if self._is_white_pawn() {
            // First we check for the up-right side.
            if can_move_right {
                let piece = _board.squares[up][right];

                if piece.is_some() {
                    if piece.unwrap().yor != self.yor {
                        _moves.push(vec![up, right]); // HERE
                    }
                }
            }

            // Then we check for the up-left side.
            if can_move_right {
                let piece = _board.squares[up][left];

                if piece.is_some() {
                    if piece.unwrap().yor != self.yor {
                        _moves.push(vec![up, left]); // HERE
                    }
                }
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
        self._get_diagonal_moves(current_x, current_y, _board, &mut moves);

        moves
    }

    fn _get_piece(
        &self,
        _x: usize,
        _y: usize,
        _board: &Board,
    ) -> Option<Piece> {
        _board.squares[_x][_y]
    }

    fn _is_black_pawn(&self) -> bool {
        self.yor == Color::BLACK
    }

    fn _is_white_pawn(&self) -> bool {
        self.yor == Color::WHITE
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
                if piece.unwrap().yor != self.yor {
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
                if piece.unwrap().yor != self.yor {
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
                if piece.unwrap().yor != self.yor {
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
                if piece.unwrap().yor != self.yor {
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
        yor: Color,
        icon: &'static str,
        name: Name,
        point: Point,
    ) -> Self {
        Piece { yor, icon, name, point }
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
