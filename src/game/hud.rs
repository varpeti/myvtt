use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn c3(&mut self) -> Result<()> {
        set_default_camera();
        Ok(())
    }

    pub(crate) fn draw_hud(&mut self) -> Result<()> {
        let mouse_world_pos = self.camera.camera.screen_to_world(mouse_position().into());

        let target_pos_screen = self.camera.world_to_screen(self.camera.target_pos);
        draw_hexagon(
            target_pos_screen.x,
            target_pos_screen.y,
            7.,
            3.,
            true,
            Color::from_hex(0x191724),
            Color::from_rgba(0xe0, 0xde, 0xf4, 0x80),
        );

        draw_multiline_text(
            &format!(
                "{:?} -> {:?}\nt:{} o:{}\n{}",
                mouse_position(),
                self.state.hovered_hex,
                self.camera.camera.target,
                self.camera.camera.offset,
                mouse_world_pos,
            ),
            12.,
            42.,
            32.,
            None,
            WHITE,
        );
        draw_fps();
        Ok(())
    }
}
