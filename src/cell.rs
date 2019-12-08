#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Self {
        Self { alive }
    }
}

use std::fmt;
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let character = if self.alive { '⬛' } else { '⬜' };
        write!(f, "{}", character)
    }
}
