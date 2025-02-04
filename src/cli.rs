use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// pass an initial grid state as an ASCII file
    #[arg(short, long, value_name = "FILE")]
    pub ascii_grid: Option<PathBuf>,
}
