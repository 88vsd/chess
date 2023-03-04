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
            Name::PAWN => return self._get_rook_valid_moves(_board),
            Name::ROOK => return self._get_rook_valid_moves(_board),
            Name::KNIGHT => return self._get_rook_valid_moves(_board),
            Name::BISHOP => return self._get_rook_valid_moves(_board),
            Name::QUEEN => return self._get_rook_valid_moves(_board),
            Name::KING => return self._get_rook_valid_moves(_board),
        }
    }

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
