mod cell;

use std::fmt::{Display, Write};

use cell::Cell;

use rand::{thread_rng, Rng};

pub enum GuessResult {
    Mine,
    NoMine,
}

#[derive(Debug)]
pub struct Minesweeper {
    pub size: usize,
    pub grid: Vec<Vec<Cell>>,
}

impl Minesweeper {
    pub fn new(size: usize, grid: Vec<Vec<Cell>>) -> Minesweeper {
        Minesweeper {
            size,
            grid,
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
        grid
    }

    pub fn place_mines(grid: &mut Vec<Vec<Cell>>, amount: usize) {
        let mut rng = thread_rng();
        for _ in 0..amount {
            let row = rng.gen_range(0..grid.len());
            let col = rng.gen_range(0..grid[0].len());
            while grid[row][col].has_mine {
                let row = rng.gen_range(0..grid.len());
                let col = rng.gen_range(0..grid[0].len());
            }
            grid[row][col].set_has_mine();
        }
    }

    pub fn calculate_adjacent_mines(grid: &Vec<Vec<Cell>>, row: usize, col: usize) -> usize {
        let mut count = 0;
        
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                let ni = row as isize + di;
                let nj = col as isize + dj;
                if ni >= 0 && nj >= 0 && ni < grid.len() as isize && nj < grid[0].len() as isize {
                    if grid[ni as usize][nj as usize].has_mine {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    pub fn guess_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) -> Option<GuessResult> {
        let cell = &mut grid[row][col];
        if cell.flagged || cell.revealed {
            return None;
        }

        cell.set_revealed();
        if cell.has_mine{
            Some(GuessResult::Mine)
        }  else {
            Some(GuessResult::NoMine)
        }
        
    }

    pub fn flag_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize) {
        let cell = &mut grid[row][col];
        if cell.revealed{
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
                    if self.grid[i][j].flagged{
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟦 ")?;
                    }
                } else if self.grid[i][j].has_mine && self.grid[i][j].revealed{
                    f.write_str("💣 ")?;
                } else {
                    write!(f, " {} ", Minesweeper::calculate_adjacent_mines(&self.grid, i, j))?;
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
        println!("test started");
        let grid = Minesweeper::create_grid(10);
        let mut ms = Minesweeper::new(10, grid);
        Minesweeper::place_mines(&mut ms.grid, 10);
        Minesweeper::guess_cell(&mut ms.grid, 4, 4);
        Minesweeper::flag_cell(&mut ms.grid, 4, 4);
        print!("{}", ms);
    }
}

