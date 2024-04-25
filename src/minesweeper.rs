mod cell;

use std::fmt::{Display, Write};

use cell::Cell;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub enum GuessResult {
    Mine,
    NoMine,
}

#[derive(Debug)]
pub struct Minesweeper {
    pub size: usize,
    pub grid: Vec<Vec<Cell>>,
    pub state: bool,
}

impl Minesweeper {
    pub fn new(size: usize, grid: Vec<Vec<Cell>>) -> Minesweeper {
        Minesweeper {
            size,
            grid,
            state: true,
        }
        
    }

    fn place_mines(grid: &mut Vec<Vec<Cell>>, num_bombs: usize) {
        let mut rng = thread_rng();
        let mut cells: Vec<(usize, usize)> = (0..grid.len())
            .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
            .collect();
        cells.shuffle(&mut rng);
        for (i, j) in cells.into_iter().take(num_bombs) {
            grid[i][j].has_mine = true;
        }
    }

    pub fn create_grid(size: usize) -> Vec<Vec<Cell>> {
        let mut grid = vec![vec![Cell::default(); size]; size];
        for i in 0..size {
            for j in 0..size {
                grid[i][j].row = i;
                grid[i][j].col = j;
            }
        }
        Minesweeper::place_mines(&mut grid, 10);
        grid
    }


    pub fn calculate_adjacent_mines(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                let ni = row as isize + di;
                let nj = col as isize + dj;
                if ni >= 0 && nj >= 0 && ni < self.grid.len() as isize && nj < self.grid[0].len() as isize {
                    if self.grid[ni as usize][nj as usize].has_mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn guess_cell(&mut self, row: usize, col: usize) -> Option<GuessResult> {
        let cell = &mut self.grid[row][col];
        if self.state == false ||cell.flagged || cell.revealed {
            return None;
        }

        cell.set_revealed();
        if cell.has_mine{
            self.state = false;
            Some(GuessResult::Mine)
        }  else {
            let count = self.calculate_adjacent_mines(row, col);
            if count == 0 {
                for &di in &[-1, 0, 1] {
                    for &dj in &[-1, 0, 1] {
                        let ni = row as isize + di;
                        let nj = col as isize + dj;
                        if ni >= 0 && nj >= 0 && ni < self.grid.len() as isize && nj < self.grid[0].len() as isize {
                            self.guess_cell(ni as usize, nj as usize);
                        }
                    }
                }
            }
            Some(GuessResult::NoMine)
        }
        
    }

    pub fn flag_cell(&mut self, row: usize, col: usize) {
        let cell = &mut self.grid[row][col];
        if self.state == false || cell.revealed{
            return;
        }

        if cell.flagged{
            cell.flag_state(false);
        } else {
            cell.flag_state(true);
        }
    }
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.grid[i][j].revealed{
                    if !self.state && self.grid[i][j].has_mine{
                        f.write_str("💣 ")?;
                    }else if self.grid[i][j].flagged{
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.grid[i][j].has_mine && self.grid[i][j].revealed{
                    f.write_str("💣 ")?;
                } else {
                    write!(f, " {} ", self.calculate_adjacent_mines(i, j))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use minesweeper::Minesweeper;
    use crate::minesweeper;

    #[test]
    fn test() {
        let grid = Minesweeper::create_grid(10);
        let mut ms = Minesweeper::new(10, grid);
        ms.guess_cell(4, 4);
        ms.flag_cell(3, 3);
        print!("{}", ms);
    }
}

