use super::board::Board;

pub struct Game {
   pub board: Board,
   pub is_game_started: bool,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height, (width * height) / 6), // Place mines in 1/6 of the cells
            is_game_started: false
        }
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        if !self.is_game_started {
            self.board.place_mines((self.board.width * self.board.height) / 6,x, y); // Place mines in 1/6 of the cells
            self.board.calculate_mines_numbers();
            self.is_game_started = true;
        }
        self.board.reveal_cell(x, y);
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        self.board.flag_cell(x, y);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&super::cell::Cell> {
        self.board.get_cell(x, y)
    }
}