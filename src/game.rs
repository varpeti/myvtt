use anyhow::Result;
use hexx::{Hex, HexOrientation};
use macroquad::prelude::*;

use crate::{
    asset_manager::{Asset, AssetManager},
    camera::{Camera, Direction},
    game_config::GameConfig,
    game_state::GameState,
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

    async fn load(&mut self) -> Result<()> {
        self.assets.load(Asset::Hex, "assets/hex.png").await?;
        Ok(())
    }

    fn c1(&mut self) -> Result<()> {
        Ok(())
    }

    fn inputs(&mut self) -> Result<()> {
        let (x, y) = mouse_position();

        let mouse_pos_in_world = self.camera.screen_to_world(Vec2::new(x, y));
        self.state.hovered_hex = self
            .config
            .hex_layout
            .world_pos_to_hex(hexx::Vec2::new(mouse_pos_in_world.x, mouse_pos_in_world.y));

        let camera_speed = 4.8;
        let rotation_speed = 1.;
        let zoom_speed = 1.1;

        if is_key_down(KeyCode::W) {
            self.camera
                .smooth_move_to_direction(Direction::Up, camera_speed);
        }
        if is_key_down(KeyCode::A) {
            self.camera
                .smooth_move_to_direction(Direction::Left, camera_speed);
        }
        if is_key_down(KeyCode::S) {
            self.camera
                .smooth_move_to_direction(Direction::Down, camera_speed);
        }
        if is_key_down(KeyCode::D) {
            self.camera
                .smooth_move_to_direction(Direction::Right, camera_speed);
        }
        self.camera.smooth_update();

        if is_key_down(KeyCode::R) {
            if is_key_down(KeyCode::LeftShift) {
                self.camera.rotate(-rotation_speed);
            } else {
                self.camera.rotate(rotation_speed);
            }
        }
        let mouse_wheel_y = mouse_wheel().1;
        if mouse_wheel_y > 0.0 {
            self.camera.zoom(1., zoom_speed);
        } else if mouse_wheel_y < 0.0 {
            self.camera.zoom(-1., zoom_speed);
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            self.state.should_exit = true;
        }

        if is_key_pressed(KeyCode::F) {
            self.config.fullscreen = !self.config.fullscreen;
            set_fullscreen(self.config.fullscreen);
        }
        Ok(())
    }

    fn c2(&mut self) -> Result<()> {
        self.camera.activate();
        clear_background(Color::from_hex(0x191724));
        draw_circle(0., 0., 1., RED);
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        for hex in self.state.origin_hex_neighbours.iter() {
            let pos = self.config.hex_layout.hex_to_world_pos(*hex);
            draw_texture(
                self.assets.get(&Asset::Hex)?,
                pos.x - self.config.hex_layout.scale.x,
                pos.y - self.config.hex_layout.scale.y,
                WHITE,
            );
            draw_hexagon(
                pos.x,
                pos.y,
                self.config.hex_layout.scale.x - 1.,
                2.,
                self.config.hex_layout.orientation == HexOrientation::Pointy,
                GRAY,
                Color::from_rgba(0, 0, 0, 0),
            );
        }

        for x in -50..50 {
            for y in -50 - x..50 - x {
                let hex = Hex::new(x, y);
                let pos = self.config.hex_layout.hex_to_world_pos(hex);
                draw_hexagon(
                    pos.x,
                    pos.y,
                    self.config.hex_layout.scale.x - 1.,
                    2.,
                    self.config.hex_layout.orientation == HexOrientation::Pointy,
                    GRAY,
                    Color::from_rgba(0, 0, 0, 0),
                );
            }
        }

        Ok(())
    }

    fn c3(&mut self) -> Result<()> {
        set_default_camera();
        Ok(())
    }

    fn draw_hud(&mut self) -> Result<()> {
        draw_text(
            &format!(
                "Hello World! {:?} -> {:?} | {}",
                mouse_position(),
                self.state.hovered_hex,
                self.camera.camera.target
            ),
            12.,
            42.,
            32.,
            WHITE,
        );
        Ok(())
    }
}
