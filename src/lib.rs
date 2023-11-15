mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut res: String = "Hello, ".to_owned();
    res.push_str(name);
    res.push_str(", from wasm-game-of-life!");
    alert(&res);
}


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
    generation: u32,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
  pub fn tick(&mut self) {
    let mut next = self.cells.clone();
    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[self.get_index(row, col)];
        let cell_state = match (cell, self.live_neighbor_count(row, col)) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise
        };
        next[idx] = cell_state;
      }
    }
    self.cells = next;
    self.generation += 1;
  }

  pub fn new() -> Universe {
    let width = 64;
    let height = 64;
    let generation = 0;

    let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();

    Universe {
        width,
        height,
        cells,
        generation,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn generation(&self) -> u32 {
    self.generation
  }

  pub fn live_cells(&self) -> u32 {
    self.cells.iter().fold(0 as u32, |a, b: &Cell| a + *b as u32 )
  }
}


use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let live_cells = self.cells.iter().fold(0, |a, b: &Cell| a + *b as u8 );
      write!(f, "Generation: {}, live cells: {}\n", self.generation, live_cells)?;

      for line in self.cells.as_slice().chunks(self.width as usize) {
          for &cell in line {
              let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
              write!(f, "{}", symbol)?;
          }
          write!(f, "\n")?;
      }

      Ok(())
    }
}