#[derive(Debug,Clone)]
pub struct Cell {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub adjacent_mines: u8,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            adjacent_mines: 0,
        }
    }
}