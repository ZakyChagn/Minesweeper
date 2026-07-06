use super::cell::Cell;

pub struct Board {
    pub width: usize,
    pub height: usize,
     cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![Cell::new(); width]; height];
        Board { width, height, cells }
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
        if let Some(cell) = self.get_cell_mut(x, y) {
            cell.is_revealed = true;
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