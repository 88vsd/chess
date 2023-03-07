use crate::piece::{Piece, Point};

pub const COLS: usize = 8;
pub const ROWS: usize = 8;
pub const AMOUNT: usize = 16;

pub const ROW_1: usize = 7;
pub const ROW_2: usize = 6;
pub const ROW_3: usize = 5;
pub const ROW_4: usize = 4;
pub const ROW_5: usize = 3;
pub const ROW_6: usize = 2;
pub const ROW_7: usize = 1;
pub const ROW_8: usize = 0;

pub const ROWS_TOTAL_AMOUNT: usize = 8;
pub const COLUMNS_TOTAL_AMOUNT: usize = 8;

pub const COLUMN_A: usize = 0;
pub const COLUMN_B: usize = 1;
pub const COLUMN_C: usize = 2;
pub const COLUMN_D: usize = 3;
pub const COLUMN_E: usize = 4;
pub const COLUMN_F: usize = 5;
pub const COLUMN_G: usize = 6;
pub const COLUMN_H: usize = 7;

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; COLS]; ROWS],
}

impl Board {
    pub fn new(
        _black_pieces: &[Piece; 16],
        _white_pieces: &[Piece; 16],
    ) -> Self {
        //let black_pieces: [Piece; AMOUNT] = Self::_initialize_black_pieces();
        // let white_pieces: [Piece; AMOUNT] = Self::_initialize_white_pieces();

        let squares = Self::_initialize(_black_pieces, _white_pieces);

        Board { squares }
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

        let first_y = 0;
        let last_y = 7;

        print!("\n\n\n");

        println!("   a b c d e f g h");
        println!();

        for x in 0..ROWS {
            for y in 0..COLS {
                // let piece = Piece::get(Point { x, y }, self);
                //self.squares[x][y];

                if y == last_y {
                    if Piece::get(Point { x, y }, self).is_some() {
                        println!(
                            "{}| {}",
                            Piece::get(Point { x, y }, self).unwrap().icon,
                            chess_notations_right.pop().unwrap()
                        );
                    } else {
                        println!(
                            // Print a space and the last `|` and after an empty square on the opposite side followed by a x number.
                            " | {}",
                            chess_notations_right.pop().unwrap()
                        );
                    }
                    // match piece {
                    //     Some(value) => {
                    //         // Print a x number and the last `|`. After that a piece icon on the opposite side.
                    //         println!(
                    //             "{}| {}",
                    //             value.icon,
                    //             chess_notations_right.pop().unwrap()
                    //         );
                    //     }
                    //     None => println!(
                    //         // Print a space and the last `|` and after an empty square on the opposite side followed by a x number.
                    //         " | {}",
                    //         chess_notations_right.pop().unwrap()
                    //     ),
                    // }
                }

                if y != last_y {
                    if Piece::get(Point { x, y }, self).is_some() {
                        if y == first_y {
                            // Print a x number and the `|` on the opposite side.
                            print!("{} |", chess_notations_left.pop().unwrap());
                        }

                        // Print a piece icon followed by the `|`.
                        print!(
                            "{}|",
                            Piece::get(Point { x, y }, self).unwrap().icon
                        )
                    } else {
                        if y == first_y {
                            // Print a x number and on the `|` on the opposite side.
                            print!("{} |", chess_notations_left.pop().unwrap());
                        };

                        // Print a space followed by the `|`.
                        print!(" |")
                    }
                    // match piece.is_some() {
                    //     Some(_) => {
                    //         if y == first_y {
                    //             // Print a x number and the `|` on the opposite side.
                    //             print!(
                    //                 "{} |",
                    //                 chess_notations_left.pop().unwrap()
                    //             );
                    //         }

                    //         // Print a piece icon followed by the `|`.
                    //         // print!("{}|", value.icon)
                    //     }
                    //     None => {
                    //         if y == first_y {
                    //             // Print a x number and on the `|` on the opposite side.
                    //             print!(
                    //                 "{} |",
                    //                 chess_notations_left.pop().unwrap()
                    //             );
                    //         };

                    //         // Print a space followed by the `|`.
                    //         print!(" |")
                    //     }
                    // }
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
        let mut squares: [[Option<Piece>; COLS]; ROWS] = [
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
        ];

        // Loop through each black piece, retrieve its board points and place it on the board.
        for i in 0..black_pieces.len() {
            let x = usize::try_from(black_pieces[i].point.x).unwrap();
            let y = usize::try_from(black_pieces[i].point.y).unwrap();

            squares[x][y] = Some(black_pieces[i].clone());
            println!("{:?}", &squares[x][y]);
        }

        // Loop through each white piece, retrieve its board points and place it on the board.
        for i in 0..white_pieces.len() {
            let x = usize::try_from(white_pieces[i].point.x).unwrap();
            let y = usize::try_from(white_pieces[i].point.y).unwrap();

            squares[x][y] = Some(white_pieces[i].clone());
            println!("{:?}", &squares[x][y]);
        }

        // Return the squares with the pieces on it.
        squares
    }

    pub fn clean(&mut self) {
        // Initialize the board with 64 None values as empty squares.
        self.squares = [
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

    fn place(&mut self, pieces: &[Piece; AMOUNT]) {
        for i in 0..pieces.len() {
            let x = usize::try_from(pieces[i].point.x).unwrap(); // Here we are just parsing an (unsinged) integer to usize.
            let y = usize::try_from(pieces[i].point.y).unwrap(); // Here we are just parsing an (unsinged) integer to usize.

            self.squares[x][y] = Some(pieces[i].clone()); // As array indexes do not support integers we pass usize values in the form of a `x` and `y`.
        }
    }
}
