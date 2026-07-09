use std::usize;

use rand::seq::SliceRandom;

use super::cell::Cell;

pub struct Board {
    pub width: usize,
    pub height: usize,
     cells: Vec<Vec<Cell>>,
    pub amount_to_reveal: usize,
    pub mines_placed: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells: Vec<Vec<Cell>> = vec![vec![Cell::new(); width]; height];
        let mines_ammount = (width * height) / 6;
        let board = Self {
            width,
            height,
            cells,
            mines_placed: mines_ammount, //Nombre de mines arbitraire pour l'instant. Ce nombre sera défini dans les paramètres de parties
            amount_to_reveal: (width * height) - mines_ammount,
        };

        board
    }

    pub fn place_mines(&mut self, mines: usize, x: usize, y: usize) {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        for by in 0..self.height {
            for bx in 0..self.width {
                if (bx == x && by == y) ||
                   (bx == x - 1 && by == y) ||
                   (bx == x + 1 && by == y) ||
                   (bx == x && by == y - 1) ||
                   (bx == x && by == y + 1) ||
                   (bx == x - 1 && by == y - 1) ||
                   (bx == x + 1 && by == y - 1) ||
                   (bx == x - 1 && by == y + 1) ||
                   (bx == x + 1 && by == y + 1) {
                    continue; // Skip the first revealed cell
                }
                positions.push((bx, by));
            }            
        }
        positions.shuffle(&mut rand::thread_rng());

        for &(x, y) in positions.iter().take(mines) {
            self.cells[y][x].is_mine = true;
        }
        self.mines_placed = mines;
    }

    pub fn calculate_mines_numbers(&mut self) {
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

    pub fn reveal_cell(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
             return false;
        }

        if self.cells[y][x].is_revealed || self.cells[y][x].is_flagged {
            return false;
        }

        self.cells[y][x].is_revealed = true;
        self.amount_to_reveal -= 1;
        if self.cells[y][x].is_mine {
            return true;
        }

        if self.cells[y][x].adjacent_mines > 0 {
            false;
            return false;
        }

        for (nx, ny) in self.neighbors(x, y) {
            self.reveal_cell(nx, ny);
        }
        return false;
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