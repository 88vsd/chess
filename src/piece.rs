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
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub icon: &'static str,
    pub name: Name,
    pub position: Position,
}

trait Pawn {
    fn _get_pawn_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>>;
    fn _get_diagonal_moves(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _get_one_step_move(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _get_two_step_move(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    );
    fn _get_piece(
        &self,
        _row: usize,
        _col: usize,
        _board: &Board,
    ) -> Option<Piece>;
    fn _is_black_pawn(&self) -> bool;
    fn _is_white_pawn(&self) -> bool;
}

trait Rook {
    fn _get_rook_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>>;
}

impl Pawn for Piece {
    fn _get_two_step_move(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        // Define whether a black pawn has been moved by the player. If so than disable the pawn two squares move.
        // Define whether a white pawn has been moved by the player. If so than disable the pawn two squares move.

        let is_undeveloped_black_pawn = _current_row == 1;
        let is_undeveloped_white_pawn = _current_row == 6;

        if self._is_black_pawn() {
            if is_undeveloped_black_pawn {
                if _current_row < 6 {
                    let piece = _board.squares[_current_row + 2][_current_col];

                    if piece.is_none() {
                        if _board.squares[_current_row + 1][_current_col]
                            .is_none()
                        {
                            _moves.push(vec![_current_row + 2, _current_col]); // HERE
                        }
                    } else if piece.unwrap().color != self.color {
                        _moves.push(vec![_current_row + 2, _current_col]); // HERE
                    }
                }
            }
        }

        if self._is_white_pawn() {
            if is_undeveloped_white_pawn {
                if _current_row > 1 {
                    let piece = _board.squares[_current_row - 2][_current_col];

                    if piece.is_none() {
                        if _board.squares[_current_row - 1][_current_col]
                            .is_none()
                        {
                            _moves.push(vec![_current_row - 2, _current_col]); //HERE
                        }
                    } else if piece.unwrap().color != self.color {
                        _moves.push(vec![_current_row - 2, _current_col]); // HERE
                    }
                }
            }
        }
    }

    fn _get_one_step_move(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        // Define the last row index for black pawns in order to perform the "pawn promotion".
        // Define the last row index for white pawns in order to perform the "pawn promotion".

        const LAST_ROW_FOR_BLACK_PAWN: usize = 7;
        const LAST_ROW_FOR_WHITE_PAWN: usize = 0;

        if self._is_black_pawn() {
            if _current_row < LAST_ROW_FOR_BLACK_PAWN {
                let piece = _board.squares[_current_row + 1][_current_col];

                if piece.is_none() {
                    _moves.push(vec![_current_row + 1, _current_col]); // HERE
                }
            }
        }

        if self._is_white_pawn() {
            if _current_row > LAST_ROW_FOR_WHITE_PAWN {
                let piece = _board.squares[_current_row - 1][_current_col];

                if piece.is_none() {
                    _moves.push(vec![_current_row - 1, _current_col]); // HERE
                }
            }
        }
    }

    fn _get_diagonal_moves(
        &self,
        _current_row: usize,
        _current_col: usize,
        _board: &Board,
        _moves: &mut Vec<Vec<usize>>,
    ) {
        let up = _current_row - 1;
        let down = _current_row + 1;
        let left = if _current_col != 0 {
            _current_col - 1
        } else {
            _current_col
        }; // TODO: solve issue with underflow.
        let right = _current_col + 1;

        let can_move_left = _current_col > 0;
        let can_move_right = _current_col < 7;

        if self._is_black_pawn() {
            // First we check for the down-right side.
            if can_move_right {
                let piece = _board.squares[down][right];

                if piece.is_some() {
                    if piece.unwrap().color != self.color {
                        _moves.push(vec![down, right]); // HERE
                    }
                }
            }

            // Then we check for the down-left side.
            if can_move_left {
                let piece = _board.squares[down][left];

                if piece.is_some() {
                    if piece.unwrap().color != self.color {
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
                    if piece.unwrap().color != self.color {
                        _moves.push(vec![up, right]); // HERE
                    }
                }
            }

            // Then we check for the up-left side.
            if can_move_right {
                let piece = _board.squares[up][left];

                if piece.is_some() {
                    if piece.unwrap().color != self.color {
                        _moves.push(vec![up, left]); // HERE
                    }
                }
            }
        }
    }

    fn _get_pawn_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>> {
        // Instantiate a vector of all valid moves that apply to the piece.
        let mut moves = Vec::new();

        // Define a current row index of the piece on the board.
        // Define a current column index of the piece on the board.

        let current_row = self.position.row;
        let current_col = self.position.col;

        self._get_two_step_move(current_row, current_col, _board, &mut moves);
        self._get_one_step_move(current_row, current_col, _board, &mut moves);
        self._get_diagonal_moves(current_row, current_col, _board, &mut moves);

        moves
    }

    fn _get_piece(
        &self,
        _row: usize,
        _col: usize,
        _board: &Board,
    ) -> Option<Piece> {
        _board.squares[_row][_col]
    }

    fn _is_black_pawn(&self) -> bool {
        self.color == Color::BLACK
    }

    fn _is_white_pawn(&self) -> bool {
        self.color == Color::WHITE
    }
}

impl Rook for Piece {
    fn _get_rook_valid_moves(&self, _board: &Board) -> Vec<Vec<usize>> {
        let mut moves = Vec::new();

        let current_row = self.position.row;
        let current_col = self.position.col;

        // UP
        for row in (0..current_row).rev() {
            println!("{}", row);
            let piece = _board.squares[row][current_col];

            if piece.is_none() {
                moves.push(vec![row, current_col]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![row, current_col]);
                    break;
                } else {
                    break;
                }
            }
        }

        // DOWN
        for row in (current_row + 1)..8 {
            let piece = _board.squares[row][current_col];

            if piece.is_none() {
                moves.push(vec![row, current_col]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![row, current_col]);
                    break;
                } else {
                    break;
                }
            }
        }

        // LEFT
        for col in (0..current_col).rev() {
            let piece = _board.squares[current_row][col];

            if piece.is_none() {
                moves.push(vec![col, current_row]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![current_row, col]);
                    break;
                } else {
                    break;
                }
            }
        }

        // RIGHT
        for col in (current_col + 1)..8 {
            let piece = _board.squares[current_row][col];

            if piece.is_none() {
                moves.push(vec![current_row, col]);
            }

            if piece.is_some() {
                if piece.unwrap().color != self.color {
                    moves.push(vec![current_row, col]);
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
        position: Position,
    ) -> Self {
        Piece { color, icon, name, position }
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
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 0 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 1 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 2 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 3 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 4 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 5 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 6 },
            ),
            Piece::new(
                Color::BLACK,
                "♟",
                Name::PAWN,
                Position { row: 1, col: 7 },
            ),
            Piece::new(
                Color::BLACK,
                "♜",
                Name::ROOK,
                Position { row: 0, col: 0 },
            ),
            Piece::new(
                Color::BLACK,
                "♞",
                Name::KNIGHT,
                Position { row: 0, col: 1 },
            ),
            Piece::new(
                Color::BLACK,
                "♝",
                Name::BISHOP,
                Position { row: 0, col: 2 },
            ),
            Piece::new(
                Color::BLACK,
                "♛",
                Name::QUEEN,
                Position { row: 0, col: 3 },
            ),
            Piece::new(
                Color::BLACK,
                "♚",
                Name::KING,
                Position { row: 0, col: 4 },
            ),
            Piece::new(
                Color::BLACK,
                "♝",
                Name::BISHOP,
                Position { row: 0, col: 5 },
            ),
            Piece::new(
                Color::BLACK,
                "♞",
                Name::KNIGHT,
                Position { row: 0, col: 6 },
            ),
            Piece::new(
                Color::BLACK,
                "♜",
                Name::ROOK,
                Position { row: 0, col: 7 },
            ),
        ];

        pieces
    }

    pub fn initialize_white_pieces() -> [Piece; 16] {
        let pieces = [
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 0 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 1 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 2 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 3 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 4 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 5 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 6 },
            ),
            Piece::new(
                Color::WHITE,
                "♙",
                Name::PAWN,
                Position { row: 6, col: 7 },
            ),
            Piece::new(
                Color::WHITE,
                "♖",
                Name::ROOK,
                Position { row: 7, col: 0 },
            ),
            Piece::new(
                Color::WHITE,
                "♘",
                Name::KNIGHT,
                Position { row: 7, col: 1 },
            ),
            Piece::new(
                Color::WHITE,
                "♗",
                Name::BISHOP,
                Position { row: 7, col: 2 },
            ),
            Piece::new(
                Color::WHITE,
                "♕",
                Name::QUEEN,
                Position { row: 7, col: 3 },
            ),
            Piece::new(
                Color::WHITE,
                "♔",
                Name::KING,
                Position { row: 7, col: 4 },
            ),
            Piece::new(
                Color::WHITE,
                "♗",
                Name::BISHOP,
                Position { row: 7, col: 5 },
            ),
            Piece::new(
                Color::WHITE,
                "♘",
                Name::KNIGHT,
                Position { row: 7, col: 6 },
            ),
            Piece::new(
                Color::WHITE,
                "♖",
                Name::ROOK,
                Position { row: 7, col: 7 },
            ),
        ];

        pieces
    }
}
