use super::board::Board;

pub struct Game {
   pub board: Board,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
        }
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        self.board.reveal_cell(x, y);
    }

    pub fn flag_cell(&mut self, x: usize, y: usize) {
        self.board.flag_cell(x, y);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&super::cell::Cell> {
        self.board.get_cell(x, y)
    }
}