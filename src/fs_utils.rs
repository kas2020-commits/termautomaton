use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::{grid::Grid, states::BasicCellState};

pub fn load_ascii_grid_from_file(
    file_path: &Path,
    min_width: u16,
    min_height: u16,
) -> Result<Grid<BasicCellState>, io::Error> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize an empty grid
    let mut data = Vec::new();

    let mut is_first = true;
    let mut width = min_width as usize;
    let mut height = 0 as usize;

    // Iterate through each line in the file
    for line in reader.lines() {
        let line = line?;

        // Skip starting comments
        if is_first && (line.starts_with('#') || line.starts_with('!') || line.trim().is_empty()) {
            continue;
        }

        let curr_width = line.chars().count();

        // you're allowed to exceed the set width only once, during the first
        // non-comment line
        if curr_width > width && is_first {
            width = curr_width;
        }

        // now that the edge case is handled, all widths should fit within the
        // existing choice of width
        assert!(curr_width <= width);

        for char in line.chars() {
            match char {
                '*' | '#' | 'O' | 'X' => data.push(BasicCellState::Alive),
                _ => data.push(BasicCellState::Dead),
            }
        }

        // pad the rest of the line with default values
        for _ in curr_width..width {
            data.push(BasicCellState::Dead);
        }

        // prepare for next iter
        height += 1;
        if is_first {
            is_first = false;
        }
    }

    while height < min_height as usize {
        for _ in 0..width {
            data.push(BasicCellState::Dead);
        }
        height += 1;
    }

    // Return the grid as a result
    Ok(Grid::new(data, width, height, BasicCellState::Dead))
}
