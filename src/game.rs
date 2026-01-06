pub mod camera_controller;
pub mod events;
pub mod game_config;
pub mod hud;
pub mod map;
pub mod theme;

use crate::game::{
    camera_controller::CameraController, events::Events, game_config::GameConfig, hud::Hud,
    map::Map, theme::Theme,
};

use anyhow::Result;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Game {
    config: GameConfig,
    events: Events,
    camera: Camera2D,
    camera_controller: CameraController,
    map: Map,
    hud: Hud,
    state: GameState,
    theme: Theme,
}

impl Game {
    pub async fn load(&mut self) -> Result<()> {
        info!("{}", self.camera.zoom);
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            clear_background(self.theme.color(theme::ThemeColor::Darker));
            let dt = get_frame_time();
            self.handle_events(dt).await?;
            self.draw().await?;
            next_frame().await;
        }
    }

    pub async fn handle_events(&mut self, dt: f32) -> Result<()> {
        match self.state {
            GameState::MapEditor => {
                self.events.update();
                self.camera_controller.handle_events(&mut self.events, dt)?;
                self.camera_controller.update(&mut self.camera, dt)?;
                self.config.handle_events(dt)?; // TODO:
                self.map.handle_events(&mut self.events, dt)?;
                self.hud.handle_events(dt)?;
            }
        }
        Ok(())
    }

    pub async fn draw(&mut self) -> Result<()> {
        match self.state {
            GameState::MapEditor => {
                set_camera(&self.camera);
                self.map.draw_map(&self.theme);
                self.map.draw_brush(&self.theme, &self.camera);
                self.map.draw_mouse_target(&self.theme, &self.camera);
                set_default_camera();
                self.hud
                    .draw(&self.theme, &self.camera, &self.camera_controller);
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
