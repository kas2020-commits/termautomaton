use crate::{state::State, window::MooreNeighborhood};

pub fn conways_game_of_life(window: &MooreNeighborhood<'_, State>) -> State {
    match (window.center, window.count(&State::Alive)) {
        // rule 1
        (State::Alive, x) if (x < 2) => State::Dead,

        // rule 2
        (State::Alive, x) if (x == 2 || x == 3) => State::Alive,

        // rule 3
        (State::Alive, x) if (x > 3) => State::Dead,

        // rule 4
        (State::Dead, x) if (x == 3) => State::Alive,

        _ => State::Dead,
    }
}
