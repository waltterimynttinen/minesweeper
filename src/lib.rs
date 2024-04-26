mod minesweeper;

use std::cell::RefCell;
use minesweeper::*;
use wasm_bindgen::prelude::*;


// creation of the game with a 10x10 board and 15 bombs
thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, Minesweeper::create_grid(10,15)));
}

// fetch the grid for visual representation in wasm
#[wasm_bindgen(js_name = getGrid)]
pub fn get_grid() -> String{
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen]
pub fn guess(x: usize, y: usize) {
    MINESWEEPER.with(|ms|{
        ms.borrow_mut().guess_cell(x, y);
    });
}

#[wasm_bindgen]
pub fn flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms|{
        ms.borrow_mut().flag_cell(x, y);
    });
}