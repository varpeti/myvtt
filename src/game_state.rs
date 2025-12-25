use hexx::Hex;
use macroquad::prelude::*;

#[derive(Debug, Default)]
pub struct GameState {
    pub should_exit: bool,
    pub origin_hex_neighbours: [Hex; 6],
    pub hovered_hex: Hex,
}

impl GameState {
    pub fn new() -> Self {
        let origin = Hex::new(0, 0);
        Self {
            should_exit: false,
            origin_hex_neighbours: origin.all_neighbors(),
            hovered_hex: origin,
        }
    }
}
