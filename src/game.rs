use crate::{board::Board, piece::Color, player::Player};

pub struct Game {
    pub board: Board,
    pub player_one: Player,
    pub player_two: Player,
    pub turn: Color,
    pub winner: Option<Player>,
}

impl Game {
    pub fn new(_player_one: Player, _player_two: Player) -> Game {
        Game {
            player_one: _player_one,
            player_two: _player_two,
            board: Board::new(),
            winner: None,
            turn: Color::WHITE,
        }
    }

    pub fn play(self) {
        println!("Playing...");
        self.board.display();
    }
}
