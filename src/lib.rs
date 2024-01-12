mod utils;

use std::cmp;
use std::convert::TryInto;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column).try_into().unwrap()
    }

    fn get_cell_live_neighbors(&self, row: u32, column: u32) -> u8 {
        let mut live_count = 0;

        // 0 0 0
        // 0 0 0
        // 0 0 0
        for row_iter in row - 1..=row + 1 {
            for column_iter in column - 1..=column + 1 {
                if row_iter == row && column_iter == column {
                    continue;
                }

                let mut row_iter_index = row_iter % self.height;
                let mut column_iter_index = column_iter % self.width;

                if row_iter < 0 {
                    row_iter_index = self.height + row_iter;
                }

                if column_iter < 0 {
                    column_iter_index = self.width + column_iter;
                }

                if self.cells[self.get_index(row_iter_index, column_iter_index)] == Cell::Alive {
                    live_count += 1;
                }
            }
        }

        live_count
    }
}
