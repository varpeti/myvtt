pub mod camera;
pub mod game_config;
pub mod hud;
pub mod map;
pub mod theme;

use crate::game::{camera::RPGCamera, game_config::GameConfig, hud::Hud, map::Map, theme::Theme};

use anyhow::Result;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Game {
    config: GameConfig,
    camera: RPGCamera,
    map: Map,
    hud: Hud,
    state: GameState,
}

impl Game {
    pub async fn load(&mut self) -> Result<()> {
        self.map.load("assets/map/001").await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            clear_background(Theme::Background.color());
            self.handle_events().await?;
            self.draw().await?;
            next_frame().await;
        }
    }

    pub async fn handle_events(&mut self) -> Result<()> {
        match self.state {
            GameState::MapEditor => {
                self.camera.handle_events()?;
                self.map.handle_events(&self.camera).await?;
                self.config.handle_events()?;
                self.hud.handle_events()?;
            }
        }
        Ok(())
    }

    pub async fn draw(&mut self) -> Result<()> {
        match self.state {
            GameState::MapEditor => {
                self.map.draw(&self.camera)?;
                self.hud.draw(&self.camera, &self.map)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    //MainMenu,
    #[default]
    MapEditor,
    //MapPlayer,
    //ExitMenu,
    //Exiting,
}
