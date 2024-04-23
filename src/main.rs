use rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
struct Cell {
    row: usize,
    col: usize,
    has_mine: bool,
    revealed: bool,
    flagged: bool,
}

impl Cell  {
    fn set_revealed(&mut self){
        self.revealed = true;
    }

    fn set_has_mine(&mut self){
        self.has_mine = true;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            row:0,
            col:0,
            has_mine: false,
            revealed: false,
            flagged: false,
        }
    }
}

fn create_grid(size: usize) -> Vec<Vec<Cell>> {
    let mut grid = vec![vec![Cell::default(); size]; size];
    for i in 0..size {
        for j in 0..size {
            grid[i][j].row = i;
            grid[i][j].col = j;
        }
    }
    grid
}
#[derive(Debug)]
struct Minesweeper {
    size: usize,
    state: bool,
    grid: Vec<Vec<Cell>>,
}

impl Minesweeper {
    pub fn new(size: usize, state: bool) -> Minesweeper {
        Minesweeper {
            size,
            state,
            grid: create_grid(size),
        }
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

    fn calculate_adjacent_mines(grid: &Vec<Vec<Cell>>, row: usize, col: usize) -> usize {
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

    fn guess_cell(grid: &mut Vec<Vec<Cell>>, row: usize, col: usize, state: &mut bool) {
        let cell = &mut grid[row][col];
        cell.set_revealed();
        if cell.has_mine{
            *state = false;
        }
        
    }
}


fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut ms = Minesweeper::new(3,true);
        Minesweeper::place_mines(&mut ms.grid, 3);
        Minesweeper::guess_cell(&mut ms.grid, 0, 0,&mut ms.state);
        print!("{:?}", ms);
    }

}
