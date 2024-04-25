#[derive(Clone, Debug)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub has_mine: bool,
    pub revealed: bool,
    pub flagged: bool,
}

impl Cell  {
    pub fn set_revealed(&mut self){
        self.revealed = true;
    }

    pub fn set_has_mine(&mut self){
        self.has_mine = true;
    }

    pub fn flag_state(&mut self, b: bool){
        self.flagged = b;
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