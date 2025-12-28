mod draw;
mod hud;
mod inputs;
mod load;

use anyhow::Result;
use macroquad::prelude::*;

use crate::{
    components::{asset_manager::AssetManager, camera::Camera, map::Map},
    game_config::GameConfig,
    game_state::GameState,
};

pub struct Game {
    assets: AssetManager,
    config: GameConfig,
    state: GameState,
    camera: Camera,
    map: Map,
}

impl Game {
    pub fn new(game_config: GameConfig) -> Self {
        Self {
            assets: AssetManager::new(),
            config: game_config,
            state: GameState::new(),
            camera: Camera::default(),
            map: Map::default(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.load().await?;
        #[allow(clippy::while_immutable_condition)]
        while !self.state.should_exit {
            clear_background(Color::from_hex(0x191724));
            self.inputs()?;
            self.draw()?;
            self.draw_hud()?;
            next_frame().await;
        }
        Ok(())
    }
}
