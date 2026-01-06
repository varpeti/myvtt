use anyhow::Result;
use macroquad::prelude::*;

use crate::game::{camera_controller::CameraController, theme::Theme};

#[derive(Debug)]
pub struct Hud {}
#[allow(clippy::derivable_impls)]
impl Default for Hud {
    fn default() -> Self {
        Self {}
    }
}

impl Hud {
    pub fn handle_events(&mut self, _dt: f32) -> Result<()> {
        Ok(())
    }

    pub fn draw(&mut self, theme: &Theme, camera: &Camera2D, camera_controller: &CameraController) {
        let (mx, my) = mouse_position();
        let camera_target_pos = camera.world_to_screen(camera_controller.to_target);

        draw_circle(
            camera_target_pos.x,
            camera_target_pos.y,
            3.,
            theme.color(crate::game::theme::ThemeColor::Normal),
        );

        draw_multiline_text(
            &format!("mouse: {:+05} {:+05}\n", mx, my,),
            12.,
            42.,
            32.,
            None,
            WHITE,
        );

        draw_fps();
    }
}
