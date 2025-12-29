use anyhow::Result;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct GameState {
    pub should_exit: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for GameState {
    fn default() -> Self {
        Self { should_exit: false }
    }
}

impl GameState {
    pub fn handle_events(&mut self) -> Result<()> {
        // if is_key_pressed(KeyCode::Escape) {
        //     self.should_exit = true;
        // }
        Ok(())
    }
}
