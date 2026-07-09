use super::board::Board;

pub struct Game {
   pub board: Board,
   pub is_game_started: bool,
   pub is_game_over: bool,
   pub is_game_lost: bool,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height), // Place mines in 1/6 of the cells
            is_game_started: false,
            is_game_over: false,
            is_game_lost: false,
        }
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        if self.is_game_over {
            return;
        }
        if !self.is_game_started {
            self.board.place_mines(self.board.mines_placed, x, y); // Place mines in 1/6 of the cells
            self.board.calculate_mines_numbers();
            self.is_game_started = true;
        }
        if self.board.reveal_cell(x, y) {
            self.is_game_over = true;
            self.is_game_lost = true;
        }
        if self.board.amount_to_reveal == 0 {
            self.is_game_over = true;
        }
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if self.is_game_over {
            return;
        }
        self.board.flag_cell(x, y);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&super::cell::Cell> {
        self.board.get_cell(x, y)
    }
}