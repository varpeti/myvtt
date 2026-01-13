pub mod brush;
pub mod camera_controller;
pub mod entities;
pub mod events;
pub mod game_state;
pub mod hud;
pub mod map;
pub mod theme;

use crate::game::{
    brush::Brush,
    camera_controller::CameraController,
    entities::Entities,
    game_state::{GameState, Mode},
    hud::Hud,
    map::Map,
    theme::Theme,
};

use anyhow::Result;
use macroquad::prelude::*;

#[derive(Default)]
pub struct Game {
    brush: Brush,
    camera: Camera2D,
    camera_controller: CameraController,
    entities: Entities,
    hud: Hud,
    map: Map,
    state: GameState,
    theme: Theme,
}

impl Game {
    pub async fn load(&mut self) -> Result<()> {
        self.map.load_map().await?;
        self.entities.load_textures().await?;
        self.entities.load_entities().await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        while self.state.mode != Mode::Exiting {
            clear_background(self.theme.color(theme::ThemeColor::Darker));
            let dt = get_frame_time();
            self.handle_events(dt).await?;
            self.draw().await?;
            next_frame().await;
        }
        Ok(())
    }

    pub async fn handle_events(&mut self, dt: f32) -> Result<()> {
        self.state.handle_events(dt)?;
        if is_quit_requested() {
            self.map.save_map().await?;
            self.state.mode = Mode::Exiting;
            return Ok(());
        }

        match self.state.mode {
            Mode::Normal => {
                self.camera_controller.handle_events(dt)?;
                self.camera_controller.update(&mut self.camera, dt)?;
                self.hud.handle_events(dt)?;
                self.entities.update(dt)?;
                self.entities
                    .handle_events(&self.map.hex_layout, &self.camera, dt)?;
            }
            Mode::MapEditor => {
                self.camera_controller.handle_events(dt)?;
                self.camera_controller.update(&mut self.camera, dt)?;
                self.brush
                    .handle_events(&mut self.map, &self.camera)
                    .await?;
                self.brush.update(&self.map, &self.camera, dt)?;
                self.hud.handle_events(dt)?;
            }
            Mode::Exiting => (),
        }
        Ok(())
    }

    pub async fn draw(&mut self) -> Result<()> {
        match self.state.mode {
            Mode::Normal => {
                set_camera(&self.camera);
                self.map.draw(&self.theme);
                self.entities
                    .draw(&self.theme, &self.map.hex_layout, &self.camera);
                set_default_camera();
                self.hud
                    .draw(&self.theme, &self.camera, &self.camera_controller);
            }
            Mode::MapEditor => {
                set_camera(&self.camera);
                self.map.draw(&self.theme);
                self.brush.draw(&self.map, &self.theme);
                self.entities
                    .draw(&self.theme, &self.map.hex_layout, &self.camera);
                set_default_camera();
                self.hud
                    .draw(&self.theme, &self.camera, &self.camera_controller);
            }
            Mode::Exiting => (),
        }
        Ok(())
    }
}
