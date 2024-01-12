mod utils;

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

                if self.cells[self.get_index(row_iter_index as u32, column_iter_index as u32)] == Cell::Alive {
                    live_count += 1;
                }
            }
        }

        live_count
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
            cells: vec![Cell::Dead; 9],
        };

        assert_eq!(universe.get_index(0, 0), 0);
        assert_eq!(universe.get_index(0, 1), 1);
        assert_eq!(universe.get_index(1, 0), 3);
        assert_eq!(universe.get_index(1, 1), 4);
    }

    #[test]
    fn test_get_cell_live_neighbors() {
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: vec![Cell::Dead; 9],
        };

        let mut index = universe.get_index(0, 0);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(0, 1);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(0, 2);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(1, 0);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(1, 1);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(1, 2);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(2, 0);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(2, 1);
        universe.cells[index] = Cell::Alive;
        index = universe.get_index(2, 2);
        universe.cells[index] = Cell::Alive;

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
}
