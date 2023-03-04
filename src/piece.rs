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
    pub row: u8,
    pub col: u8,
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
