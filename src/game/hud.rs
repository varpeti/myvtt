use anyhow::Result;
use macroquad::prelude::*;

use crate::game::{camera::RPGCamera, theme::Theme};

#[derive(Debug)]
pub struct Hud {}
#[allow(clippy::derivable_impls)]
impl Default for Hud {
    fn default() -> Self {
        Self {}
    }
}

impl Hud {
    pub fn handle_events(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn draw(&mut self, camera: &RPGCamera) -> Result<()> {
        draw_multiline_text(
            &format!(
                "mouse: {:?}\ncamera: {} {} {}\norigo on screen: {:?}",
                mouse_position(),
                camera.get_target(),
                camera.get_zoom(),
                camera.get_rotation(),
                camera.world_to_screen(vec3(0., 0., 0.)),
            ),
            12.,
            42.,
            32.,
            None,
            Theme::Text.color(),
        );

        draw_fps();
        Ok(())
    }
}
