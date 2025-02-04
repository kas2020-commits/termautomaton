use crate::{grid::Grid, windows::MooreNeighborhood};
use std::mem;

#[derive(Debug)]
pub struct Automata<T> {
    t: u32,
    initial: Vec<T>,
    scratch: Vec<T>,

    pub grid: Grid<T>,
}

impl<T> Automata<T>
where
    T: PartialEq + Copy,
{
    pub fn new(grid: Grid<T>) -> Self {
        let t = 0;
        let len = grid.width * grid.height;
        let scratch = vec![grid.default; len];
        let initial = grid.data.clone();
        Automata {
            t,
            grid,
            scratch,
            initial,
        }
    }

    pub fn reset(&mut self) {
        self.t = 0;
        self.grid.data.copy_from_slice(&self.initial);
    }

    pub fn advance<F: Fn(&MooreNeighborhood<'_, T>) -> T>(&mut self, ruleset: F) {
        // prevent too much going on at the start
        if self.t > 1 {
            for (x, y, window) in self.grid.iter(Grid::window_moore_nowrap) {
                self.scratch[self.grid.idx(x, y)] = ruleset(&window);
            }
            mem::swap(&mut self.scratch, &mut self.grid.data);
        }

        self.t += 1;
    }
}
