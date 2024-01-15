mod utils;

extern crate fixedbitset;

use wasm_bindgen::prelude::*;

use fixedbitset::FixedBitSet;
use rand::Rng;
use std::convert::TryInto;
use std::fmt;

// #[wasm_bindgen]
// #[repr(u8)]
// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Cell {
//     Dead = 0,
//     Alive = 1,
// }

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = FixedBitSet::with_capacity((width * height) as usize);

        for index in 0..(width * height) {
            let cell = if rng.gen_range(0..100) < 54 {
                cells.set(index as usize, true);
            } else {
                cells.set(index as usize, false);
            };
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column).try_into().unwrap()
    }

    fn get_cell_live_neighbors(&self, row: u32, column: u32) -> u8 {
        let mut live_count = 0;

        // 0 0 0
        // 0 0 0
        // 0 0 0
        for row_iter in row as i64 - 1..=row as i64 + 1 {
            for column_iter in column as i64 - 1..=column as i64 + 1 {
                if row_iter == row as i64 && column_iter == column as i64 {
                    continue;
                }

                let mut row_iter_index = row_iter % self.height as i64;
                let mut column_iter_index = column_iter % self.width as i64;

                if row_iter < 0 {
                    row_iter_index = self.height as i64 + row_iter;
                }

                if column_iter < 0 {
                    column_iter_index = self.width as i64 + column_iter;
                }

                if self.cells[self.get_index(row_iter_index as u32, column_iter_index as u32)]
                    == true
                {
                    live_count += 1;
                }
            }
        }

        live_count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let neighbor_live_count = self.get_cell_live_neighbors(row, column);

                let next_cell = match (cell, neighbor_live_count) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => false,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                let symbol = if *cell == false { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: FixedBitSet::with_capacity(9),
        };

        assert_eq!(universe.get_index(0, 0), 0);
        assert_eq!(universe.get_index(0, 1), 1);
        assert_eq!(universe.get_index(1, 0), 3);
        assert_eq!(universe.get_index(1, 1), 4);
    }

    #[test]
    fn test_all_cells_alive() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: FixedBitSet::with_capacity(9),
        };

        for index in 0..9 {
            universe.cells.set(index, true);
        }

        // 1 1 1
        // 1 1 1
        // 1 1 1

        assert_eq!(universe.get_cell_live_neighbors(0, 0), 8);
        assert_eq!(universe.get_cell_live_neighbors(0, 1), 8);
        assert_eq!(universe.get_cell_live_neighbors(0, 2), 8);
        assert_eq!(universe.get_cell_live_neighbors(1, 0), 8);
        assert_eq!(universe.get_cell_live_neighbors(1, 1), 8);
        assert_eq!(universe.get_cell_live_neighbors(1, 2), 8);
        assert_eq!(universe.get_cell_live_neighbors(2, 0), 8);
        assert_eq!(universe.get_cell_live_neighbors(2, 1), 8);
        assert_eq!(universe.get_cell_live_neighbors(2, 2), 8);
    }

    #[test]
    fn test_all_cells_dead() {
        let universe = Universe {
            width: 3,
            height: 3,
            cells: FixedBitSet::with_capacity(9),
        };

        // 0 0 0
        // 0 0 0
        // 0 0 0

        assert_eq!(universe.get_cell_live_neighbors(0, 0), 0);
        assert_eq!(universe.get_cell_live_neighbors(0, 1), 0);
        assert_eq!(universe.get_cell_live_neighbors(0, 2), 0);
        assert_eq!(universe.get_cell_live_neighbors(1, 0), 0);
        assert_eq!(universe.get_cell_live_neighbors(1, 1), 0);
        assert_eq!(universe.get_cell_live_neighbors(1, 2), 0);
        assert_eq!(universe.get_cell_live_neighbors(2, 0), 0);
        assert_eq!(universe.get_cell_live_neighbors(2, 1), 0);
        assert_eq!(universe.get_cell_live_neighbors(2, 2), 0);
    }

    #[test]
    fn test_single_alive_middle() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: FixedBitSet::with_capacity(9),
        };

        let index = universe.get_index(1, 1);
        universe.cells[index] = true;

        // 0 0 0
        // 0 1 0
        // 0 0 0

        assert_eq!(universe.get_cell_live_neighbors(0, 0), 1);
        assert_eq!(universe.get_cell_live_neighbors(0, 1), 1);
        assert_eq!(universe.get_cell_live_neighbors(0, 2), 1);
        assert_eq!(universe.get_cell_live_neighbors(1, 0), 1);
        assert_eq!(universe.get_cell_live_neighbors(1, 1), 0);
        assert_eq!(universe.get_cell_live_neighbors(1, 2), 1);
        assert_eq!(universe.get_cell_live_neighbors(2, 0), 1);
        assert_eq!(universe.get_cell_live_neighbors(2, 1), 1);
        assert_eq!(universe.get_cell_live_neighbors(2, 2), 1);
    }

    #[test]
    fn test_diagnal_alive() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: FixedBitSet::with_capacity(9),
        };

        let mut index = universe.get_index(0, 0);
        universe.cells[index] = true;
        index = universe.get_index(1, 1);
        universe.cells[index] = true;
        index = universe.get_index(2, 2);
        universe.cells[index] = true;

        // 1 0 0
        // 0 1 0
        // 0 0 1

        assert_eq!(universe.get_cell_live_neighbors(0, 0), 2);
        assert_eq!(universe.get_cell_live_neighbors(0, 1), 3);
        assert_eq!(universe.get_cell_live_neighbors(0, 2), 3);
        assert_eq!(universe.get_cell_live_neighbors(1, 0), 3);
        assert_eq!(universe.get_cell_live_neighbors(1, 1), 2);
        assert_eq!(universe.get_cell_live_neighbors(1, 2), 3);
        assert_eq!(universe.get_cell_live_neighbors(2, 0), 3);
        assert_eq!(universe.get_cell_live_neighbors(2, 1), 3);
        assert_eq!(universe.get_cell_live_neighbors(2, 2), 2);
    }
}
