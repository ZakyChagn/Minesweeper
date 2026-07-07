use rand::seq::SliceRandom;

use super::cell::Cell;

pub struct Board {
    pub width: usize,
    pub height: usize,
     cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize, mines : usize) -> Self {
        let cells = vec![vec![Cell::new(); width]; height];
        let mut board = Self {
            width,
            height,
            cells,
        };
        board.place_mines(mines);
        board.calculate_numbers();

        board
    }

    pub fn place_mines(&mut self, mines: usize) {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                positions.push((x, y));
            }            
        }
        positions.shuffle(&mut rand::thread_rng());

        for &(x, y) in positions.iter().take(mines) {
            self.cells[y][x].is_mine = true;
        }
    }

    pub fn calculate_numbers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].is_mine {
                    continue;
                }
                let mut count = 0;
                for (nx, ny) in self.neighbors(x, y) {
                    if self.cells[ny][nx].is_mine {
                        count += 1;
                    }
                }
                self.cells[y][x].adjacent_mines = count;
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.width && y < self.height {
            Some(&self.cells[y][x])
        } else {
            None
        }
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.width && y < self.height {
            Some(&mut self.cells[y][x])
        } else {
            None
        }
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            return;
        }

        if self.cells[y][x].is_revealed || self.cells[y][x].is_flagged {
            return;
        }

        self.cells[y][x].is_revealed = true;

        if self.cells[x][y].is_mine {
            return;
        }

        if self.cells[y][x].adjacent_mines > 0 {
            return;
        }

        for (nx, ny) in self.neighbors(x, y) {
            self.reveal_cell(nx, ny);
        }

    }
    
    pub fn flag_cell(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_cell_mut(x, y) {
            cell.is_flagged = !cell.is_flagged;
        }
    }
    
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    neighbors.push((nx as usize, ny as usize));
                }
            }
        }
        neighbors
    }
}