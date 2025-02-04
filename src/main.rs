mod app;
mod automata;
mod cli;
mod fs_utils;
mod grid;
mod rulesets;
mod states;
mod windows;

use cli::Cli;
use fs_utils::load_ascii_grid_from_file;
use grid::Grid;
use rand::{distr::Uniform, prelude::*};
use states::BasicCellState;
use std::io;

use clap::Parser;

unsafe fn gen_rand_vec(len: usize) -> Vec<BasicCellState> {
    let mut rng = rand::rng();

    let mut data = Vec::with_capacity(len);

    let range = Uniform::new(0 as f64, 1 as f64).unwrap();

    for _ in 0..len {
        let val = if range.sample(&mut rng) > 0.75 {
            BasicCellState::Alive
        } else {
            BasicCellState::Dead
        };
        data.push(val);
    }

    data
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let terminal = ratatui::init();
    let initial_size = terminal.size()?;

    if let Some(ascii_grid_path) = cli.ascii_grid.as_deref() {
        let grid =
            load_ascii_grid_from_file(ascii_grid_path, initial_size.width, initial_size.height)?;
        let app_result = app::TerminalApp::new(grid, initial_size).run(terminal);
        ratatui::restore();
        app_result
    } else {
        let data = unsafe { gen_rand_vec((initial_size.width * initial_size.height) as usize * 4) };
        let grid = Grid::new(
            data,
            initial_size.width as usize * 2,
            initial_size.height as usize * 2,
            BasicCellState::Dead,
        );
        let app_result = app::TerminalApp::new(grid, initial_size).run(terminal);
        ratatui::restore();
        app_result
    }
}
