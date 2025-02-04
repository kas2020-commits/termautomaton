use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub txt_grid: Option<PathBuf>,
}
