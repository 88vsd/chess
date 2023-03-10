use crate::{
    board::Board,
    constants::{
        Pieces, A1, A2, A7, A8, B1, B2, B7, B8, B_B_C, B_B_F, B_K_E, B_N_B,
        B_N_G, B_P_A, B_P_B, B_P_C, B_P_D, B_P_E, B_P_F, B_P_G, B_P_H, B_Q_D,
        B_R_A, B_R_H, C1, C2, C7, C8, D1, D2, D7, D8, E1, E2, E7, E8, F1, F2,
        F7, F8, G1, G2, G7, G8, H1, H2, H7, H8, PIECES_TOTAL_AMOUNT, W_B_C,
        W_B_F, W_K_E, W_N_B, W_N_G, W_P_A, W_P_B, W_P_C, W_P_D, W_P_E, W_P_F,
        W_P_G, W_P_H, W_Q_D, W_R_A, W_R_H,
    },
    piece::{Color, Name, Piece},
};

pub struct Chess {
    pub board: Board,
    pub pieces: Pieces,
}

impl Chess {
    pub fn new() -> Self {
        let pieces: Pieces = Self::_initialize_pieces();
        let board: Board = Self::_initialize_board(&pieces);

        Chess { board, pieces }
    }

    pub fn get_pieces(&self) -> &Pieces {
        &self.pieces
    }

    fn _initialize_board(_pieces: &Pieces) -> Board {
        Board::new(_pieces)
    }

    fn _initialize_pieces() -> Pieces {
        let mut pieces: [Option<Piece>; PIECES_TOTAL_AMOUNT] = [
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None,
        ];

        // Black
        pieces[B_P_A] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, A7));
        pieces[B_P_B] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, B7));
        pieces[B_P_C] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, C7));
        pieces[B_P_D] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, D7));
        pieces[B_P_E] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, E7));
        pieces[B_P_F] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, F7));
        pieces[B_P_G] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, G7));
        pieces[B_P_H] = Some(Piece::new(Color::BLACK, "♟", Name::PAWN, H7));
        pieces[B_R_A] = Some(Piece::new(Color::BLACK, "♜", Name::ROOK, A8));
        pieces[B_N_B] = Some(Piece::new(Color::BLACK, "♞", Name::KNIGHT, B8));
        pieces[B_B_C] = Some(Piece::new(Color::BLACK, "♝", Name::BISHOP, C8));
        pieces[B_Q_D] = Some(Piece::new(Color::BLACK, "♛", Name::QUEEN, D8));
        pieces[B_K_E] = Some(Piece::new(Color::BLACK, "♚", Name::KING, E8));
        pieces[B_B_F] = Some(Piece::new(Color::BLACK, "♝", Name::BISHOP, F8));
        pieces[B_N_G] = Some(Piece::new(Color::BLACK, "♞", Name::KNIGHT, G8));
        pieces[B_R_H] = Some(Piece::new(Color::BLACK, "♜", Name::ROOK, H8));
        // White
        pieces[W_P_A] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, A2));
        pieces[W_P_B] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, B2));
        pieces[W_P_C] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, C2));
        pieces[W_P_D] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, D2));
        pieces[W_P_E] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, E2));
        pieces[W_P_F] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, F2));
        pieces[W_P_G] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, G2));
        pieces[W_P_H] = Some(Piece::new(Color::WHITE, "♙", Name::PAWN, H2));
        pieces[W_R_A] = Some(Piece::new(Color::WHITE, "♖", Name::ROOK, A1));
        pieces[W_N_B] = Some(Piece::new(Color::WHITE, "♘", Name::KNIGHT, B1));
        pieces[W_B_C] = Some(Piece::new(Color::WHITE, "♗", Name::BISHOP, C1));
        pieces[W_Q_D] = Some(Piece::new(Color::WHITE, "♕", Name::QUEEN, D1));
        pieces[W_K_E] = Some(Piece::new(Color::WHITE, "♔", Name::KING, E1));
        pieces[W_B_F] = Some(Piece::new(Color::WHITE, "♗", Name::BISHOP, F1));
        pieces[W_N_G] = Some(Piece::new(Color::WHITE, "♘", Name::KNIGHT, G1));
        pieces[W_R_H] = Some(Piece::new(Color::WHITE, "♖", Name::ROOK, H1));

        // Create a new array of Pieces using map() and unwrap()
        let result: Pieces = pieces
            .iter()
            .map(|piece| piece.clone().unwrap()) // TODO: avoid clone()
            .collect::<Vec<Piece>>()
            .try_into()
            .unwrap();

        return result;
    }
}
