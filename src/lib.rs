mod minesweeper;

use std::cell::RefCell;
use minesweeper::*;
use wasm_bindgen::prelude::*;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, Minesweeper::create_grid(10)));
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

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