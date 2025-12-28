use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn draw_hud(&mut self) -> Result<()> {
        set_default_camera();
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
                "mouse: {:?}\ncamera: {} {} {}",
                mouse_position(),
                self.camera.get_target(),
                self.camera.get_zoom(),
                self.camera.get_rotation(),
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
