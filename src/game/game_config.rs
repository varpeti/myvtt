use anyhow::Result;
use macroquad::prelude::*;

#[derive(Debug)]
pub struct GameConfig {
    pub fullscreen: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for GameConfig {
    fn default() -> Self {
        Self { fullscreen: false }
    }
}

impl GameConfig {
    pub fn handle_events(&mut self) -> Result<()> {
        if is_key_pressed(KeyCode::F) {
            self.fullscreen = !self.fullscreen;
            set_fullscreen(self.fullscreen);
        }
        Ok(())
    }
}
