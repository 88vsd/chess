use crate::piece::Piece;

const COLS: usize = 8;
const ROWS: usize = 8;
const AMOUNT: usize = 16;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: [[Option<Piece>; COLS]; ROWS],
}

impl Board {
    pub fn new(
        _black_pieces: &[Piece; 16],
        _white_pieces: &[Piece; 16],
    ) -> Self {
        //let black_pieces: [Piece; AMOUNT] = Self::_initialize_black_pieces();
        // let white_pieces: [Piece; AMOUNT] = Self::_initialize_white_pieces();

        let board = Self::_initialize(_black_pieces, _white_pieces);

        Board { board }
    }

    pub fn display(
        &self,
        black_pieces: &[Piece; 16],
        white_pieces: &[Piece; 16],
    ) {
        let mut chess_notations_left =
            vec!["1", "2", "3", "4", "5", "6", "7", "8"];
        let mut chess_notations_right =
            vec!["1", "2", "3", "4", "5", "6", "7", "8"];

        let first_col = 0;
        let last_col = 7;

        print!("\n\n\n");

        println!("   a b c d e f g h");
        println!();

        for row in 0..ROWS {
            for col in 0..COLS {
                let piece = self.board[row][col];

                if col == last_col {
                    match piece {
                        Some(value) => {
                            // Print a row number and the last `|`. After that a piece icon on the opposite side.
                            println!(
                                "{}| {}",
                                value.icon,
                                chess_notations_right.pop().unwrap()
                            );
                        }
                        None => println!(
                            // Print a space and the last `|` and after an empty square on the opposite side followed by a row number.
                            " | {}",
                            chess_notations_right.pop().unwrap()
                        ),
                    }
                }

                if col != last_col {
                    match piece {
                        Some(value) => {
                            if col == first_col {
                                // Print a row number and the `|` on the opposite side.
                                print!(
                                    "{} |",
                                    chess_notations_left.pop().unwrap()
                                );
                            }

                            // Print a piece icon followed by the `|`.
                            print!("{}|", value.icon)
                        }
                        None => {
                            if col == first_col {
                                // Print a row number and on the `|` on the opposite side.
                                print!(
                                    "{} |",
                                    chess_notations_left.pop().unwrap()
                                );
                            };

                            // Print a space followed by the `|`.
                            print!(" |")
                        }
                    }
                }
            }
        }

        println!();
        println!("   a b c d e f g h");
    }

    fn _initialize(
        black_pieces: &[Piece; 16],
        white_pieces: &[Piece; 16],
    ) -> [[Option<Piece>; COLS]; ROWS] {
        // Initialize the board with 64 None values as empty squares.
        let mut board: [[Option<Piece>; COLS]; ROWS] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        // Loop through each black piece, retrieve its board positions and place it on the board.
        for i in 0..black_pieces.len() {
            let row = usize::try_from(black_pieces[i].position.row).unwrap();
            let col = usize::try_from(black_pieces[i].position.col).unwrap();

            board[row][col] = Some(black_pieces[i]);
            println!("{:?}", &board[row][col]);
        }

        // Loop through each white piece, retrieve its board positions and place it on the board.
        for i in 0..white_pieces.len() {
            let row = usize::try_from(white_pieces[i].position.row).unwrap();
            let col = usize::try_from(white_pieces[i].position.col).unwrap();

            board[row][col] = Some(white_pieces[i]);
            println!("{:?}", &board[row][col]);
        }

        // Return the board with the pieces on it.
        board
    }

    pub fn clean(&mut self) {
        // Initialize the board with 64 None values as empty squares.
        self.board = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];
    }

    pub fn update(
        &mut self,
        _black_pieces: &[Piece; AMOUNT],
        _white_pieces: &[Piece; AMOUNT],
    ) {
        self.clean();

        self.place(_black_pieces);
        self.place(_white_pieces);
    }

    fn place(&mut self, pieces: &[Piece; 16]) {
        for i in 0..pieces.len() {
            let row = usize::try_from(pieces[i].position.row).unwrap(); // Here we are just parsing an (unsinged) integer to usize.
            let col = usize::try_from(pieces[i].position.col).unwrap(); // Here we are just parsing an (unsinged) integer to usize.

            self.board[row][col] = Some(pieces[i]); // As array indexes do not support integers we pass usize values in the form of a `row` and `col`.
        }
    }
}
