use crate::{grid::Grid, window::MooreNeighborhood};
use std::mem;

#[derive(Debug)]
pub struct CA<T> {
    t: u32,
    pub grid: Grid<T>,
    scratch: Vec<T>,
}

impl<T> CA<T>
where
    T: PartialEq + Copy,
{
    pub fn new(grid: Grid<T>) -> Self {
        let t = 0;
        let len = grid.width * grid.height;
        let scratch = vec![grid.default; len];
        CA { t, grid, scratch }
    }

    pub fn advance<F: Fn(&MooreNeighborhood<'_, T>) -> T>(&mut self, ruleset: F) {
        self.t += 1;

        // prevent too much going on at the start
        if self.t > 1 {
            for (x, y, window) in self.grid.iter() {
                self.scratch[self.grid.idx(x, y)] = ruleset(&window);
            }
            mem::swap(&mut self.scratch, &mut self.grid.data);
        }
    }
}
