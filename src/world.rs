extern crate rand;
use std::time::Instant;

use rand::Rng;
use rayon::prelude::*;

use crate::cell::Cell;
#[derive(Debug, Clone)]
pub struct World {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub tick_time_ms: u128,
}

const NEIGHBORS_ADDRESSES: &[(i32, i32)] = &[
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![Cell::new(false); height * width],
            width,
            height,
            tick_time_ms: 0,
        }
    }

    pub fn new_random(width: usize, height: usize) -> Self {
        let cells = (0..height * width)
            .map(|index| {
                let x = index % width;
                let y = index / width;
                if x != 0 && y != 0 && x < width - 1 && y < height - 1 {
                    Cell::new(rand::thread_rng().gen::<f64>() <= 0.25)
                } else {
                    Cell::new(false)
                }
            })
            .collect();

        Self {
            cells,
            width,
            height,
            tick_time_ms: 0,
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.cells[y * self.width + x].alive
    }

    pub fn cell_live(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && self.cells[y * self.width + x].alive
    }

    pub fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        NEIGHBORS_ADDRESSES
            .iter()
            .map(|addr| {
                let neighbour_x = x as i32 + addr.0;
                let neighbour_y = y as i32 + addr.1;
                self.is_alive(neighbour_x as usize, neighbour_y as usize)
            })
            .filter(|&value| value)
            .count()
    }

    pub fn compute_count(&self) -> Vec<usize> {
        (0..self.width * self.height)
            .into_par_iter()
            .map(|index| {
                let y = index / self.width;
                let x = index % self.width;
                if x != 0 && y != 0 && x < self.width - 1 && y < self.height - 1 {
                    self.count_alive_neighbors(x, y)
                } else {
                    0
                }
            })
            .collect()
    }

    pub fn tick(&self) -> Self {
        let _now = Instant::now();
        let counts = self.compute_count();

        let update_rule = |alive: bool, n_neigbors: usize| {
            if alive {
                !(n_neigbors < 2 || n_neigbors > 3)
            } else {
                n_neigbors == 3
            }
        };

        Self {
            width: self.width,
            height: self.height,

            cells: self
                .cells
                .par_iter()
                .zip(counts)
                .map(|(cell, n_neigbors)| Cell::new(update_rule(cell.alive, n_neigbors)))
                .collect(),
            tick_time_ms: _now.elapsed().as_millis(),
        }
    }

    pub fn clear(&self) -> Self {
        Self {
            cells: vec![Cell::new(false); self.height * self.width],
            width: self.width,
            height: self.height,
            tick_time_ms: 0,
        }
    }

    pub fn random(&self) -> Self {
        Self::new_random(self.width, self.height)
    }
}

use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board = self
            .cells
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                format!(
                    "{}{}",
                    cell,
                    if index % self.width == self.width - 1 {
                        "\n"
                    } else {
                        ""
                    },
                )
            })
            .collect::<String>();

        write!(f, "{}", board)
    }
}
