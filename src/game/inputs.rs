use crate::{camera::Direction, game::Game};

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(super) fn c1(&mut self) -> Result<()> {
        Ok(())
    }

    pub(super) fn inputs(&mut self) -> Result<()> {
        if is_key_down(KeyCode::W) {
            self.camera.move_to_target(vec3(0., -10., 0.));
        }
        if is_key_down(KeyCode::A) {
            self.camera.move_to_target(vec3(-10., 0., 0.));
        }
        if is_key_down(KeyCode::S) {
            self.camera.move_to_target(vec3(0., 10., 0.));
        }
        if is_key_down(KeyCode::D) {
            self.camera.move_to_target(vec3(10., 0., 0.));
        }

        // let mouse_pos_in_world = self.camera.screen_to_world(Vec2::from(mouse_position()));
        // self.state.hovered_hex = self
        //     .config
        //     .hex_layout
        //     .world_pos_to_hex(hexx::Vec2::new(mouse_pos_in_world.x, mouse_pos_in_world.y));

        // if is_mouse_button_pressed(MouseButton::Middle) {
        //     self.camera.smooth_move_to_target(mouse_pos_in_world);
        // }

        let rotation = 30.;
        let zoom = 10.;
        let mouse_wheel_y = mouse_wheel().1;
        if mouse_wheel_y > 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.camera.rotate(rotation);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.camera.zoom(zoom);
            }
        } else if mouse_wheel_y < 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.camera.rotate(-rotation);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.camera.zoom(-zoom);
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            self.state.should_exit = true;
        }

        if is_key_pressed(KeyCode::F) {
            self.config.fullscreen = !self.config.fullscreen;
            set_fullscreen(self.config.fullscreen);
        }

        Ok(())
    }
}
