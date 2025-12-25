use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(super) fn c1(&mut self) -> Result<()> {
        Ok(())
    }

    pub(super) fn inputs(&mut self) -> Result<()> {
        let mouse_pos_in_world = self.camera.screen_to_world(Vec2::from(mouse_position()));
        self.state.hovered_hex = self
            .config
            .hex_layout
            .world_pos_to_hex(hexx::Vec2::new(mouse_pos_in_world.x, mouse_pos_in_world.y));

        if is_mouse_button_pressed(MouseButton::Middle) {
            self.camera.smooth_move_to_position(mouse_pos_in_world);
        }
        self.camera.smooth_move_update(5.0);

        let rotation = 30.;
        let zoom = 1.1;
        let mouse_wheel_y = mouse_wheel().1;
        if mouse_wheel_y > 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.camera.smooth_rotate(rotation);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.camera.smooth_zoom(zoom);
            }
        } else if mouse_wheel_y < 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.camera.smooth_rotate(-rotation);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.camera.smooth_zoom(1. / zoom);
            }
        }
        self.camera.smooth_zoom_update(5.0);
        self.camera.smooth_rotate_update(5.0);

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
