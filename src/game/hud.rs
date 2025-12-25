use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn c3(&mut self) -> Result<()> {
        set_default_camera();
        Ok(())
    }

    pub(crate) fn draw_hud(&mut self) -> Result<()> {
        draw_text(
            &format!(
                "Hello World! {:?} -> {:?} | {}",
                mouse_position(),
                self.state.hovered_hex,
                self.camera.camera.target,
            ),
            12.,
            42.,
            32.,
            WHITE,
        );
        draw_fps();
        Ok(())
    }
}
