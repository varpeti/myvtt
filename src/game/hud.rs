use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn c3(&mut self) -> Result<()> {
        set_default_camera();
        Ok(())
    }

    pub(crate) fn draw_hud(&mut self) -> Result<()> {
        let middle = vec2(screen_width() / 2., screen_height() / 2.);
        draw_hexagon(
            middle.x,
            middle.y,
            7.,
            1.,
            true,
            Color::from_rgba(0x19, 0x17, 0x24, 0x80),
            Color::from_rgba(0xe0, 0xde, 0xf4, 0x80),
        );
        draw_circle(
            middle.x,
            middle.y,
            1.,
            Color::from_rgba(0x19, 0x17, 0x24, 0x80),
        );

        draw_multiline_text(
            &format!(
                "{:?} -> {:?}\nt:{} p:{}",
                mouse_position(),
                self.state.hovered_hex,
                self.camera.camera.target,
                self.camera.camera.position,
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
