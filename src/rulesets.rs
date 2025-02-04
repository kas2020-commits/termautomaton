use crate::{states::BasicCellState, windows::MooreNeighborhood};

pub fn conways_game_of_life(window: &MooreNeighborhood<'_, BasicCellState>) -> BasicCellState {
    match (window.center, window.count(&BasicCellState::Alive)) {
        // rule 1
        (BasicCellState::Alive, x) if (x < 2) => BasicCellState::Dead,

        // rule 2
        (BasicCellState::Alive, x) if (x == 2 || x == 3) => BasicCellState::Alive,

        // rule 3
        (BasicCellState::Alive, x) if (x > 3) => BasicCellState::Dead,

        // rule 4
        (BasicCellState::Dead, x) if (x == 3) => BasicCellState::Alive,

        _ => BasicCellState::Dead,
    }
}
