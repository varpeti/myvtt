pub mod camera;
pub mod game_config;
pub mod game_state;
pub mod hud;
pub mod map;

use crate::game::{
    camera::RPGCamera, game_config::GameConfig, game_state::GameState, hud::Hud, map::Map,
};

use anyhow::Result;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Game {
    config: GameConfig,
    state: GameState,
    camera: RPGCamera,
    map: Map,
    hud: Hud,
}

impl Game {
    pub async fn load(&mut self) -> Result<()> {
        self.map.load("assets/map/001.map").await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        while !self.state.should_exit {
            clear_background(Color::from_hex(0x191724));
            self.handle_events()?;
            self.draw()?;
            next_frame().await;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<()> {
        self.camera.handle_events()?;
        self.config.handle_events()?;
        self.state.handle_events()?;
        self.hud.handle_events()?;
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.camera.activate()?;
        self.map.draw(&self.camera)?;
        set_default_camera();
        self.hud.draw(&self.camera)?;
        Ok(())
    }
}
