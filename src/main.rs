mod app;
mod automata;
mod grid;
mod rulesets;
mod state;
mod window;

use grid::Grid;
use rand::{distr::Uniform, prelude::*};
use state::State;
use std::io;

unsafe fn gen_rand_vec(len: usize) -> Vec<State> {
    let mut rng = rand::rng();

    let mut data = Vec::with_capacity(len);

    let range = Uniform::new(0 as f64, 1 as f64).unwrap();

    for _ in 0..len {
        let val = if range.sample(&mut rng) > 0.75 {
            State::Alive
        } else {
            State::Dead
        };
        data.push(val);
    }

    data
}

fn main() -> io::Result<()> {
    let terminal = ratatui::init();

    let true_size = terminal.size()?;

    let data = unsafe { gen_rand_vec((true_size.width * true_size.height) as usize) };

    let grid = Grid::new(
        data,
        true_size.width as usize,
        true_size.height as usize,
        State::Dead,
    );

    let app_result = app::App::new(grid).run(terminal);
    ratatui::restore();
    app_result
}
