mod draw;
mod hud;
mod inputs;
mod load;

use anyhow::Result;
use macroquad::prelude::*;

use crate::{
    asset_manager::AssetManager, camera::Camera, game_config::GameConfig, game_state::GameState,
};

pub struct Game {
    assets: AssetManager,
    config: GameConfig,
    state: GameState,
    camera: Camera,
}

impl Game {
    pub fn new(game_config: GameConfig) -> Self {
        Self {
            assets: AssetManager::new(),
            config: game_config,
            state: GameState::new(),
            camera: Camera::default(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        eprint!("{}", self.config.hex_layout.rect_size());
        self.load().await?;
        #[allow(clippy::while_immutable_condition)]
        while !self.state.should_exit {
            self.c1()?;
            self.inputs()?;
            self.c2()?;
            self.draw()?;
            self.c3()?;
            self.draw_hud()?;
            next_frame().await;
        }
        Ok(())
    }
}
