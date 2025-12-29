use anyhow::Result;
use macroquad::prelude::*;

use crate::game::camera::RPGCamera;

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
        // let middle = vec2(screen_width() / 2., screen_height() / 2.);
        // draw_hexagon(
        //     middle.x,
        //     middle.y,
        //     7.,
        //     1.,
        //     true,
        //     Color::from_rgba(0x19, 0x17, 0x24, 0x80),
        //     Color::from_rgba(0xe0, 0xde, 0xf4, 0x80),
        // );
        // draw_circle(
        //     middle.x,
        //     middle.y,
        //     1.,
        //     Color::from_rgba(0x19, 0x17, 0x24, 0x80),
        // );

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
            WHITE,
        );

        draw_fps();
        Ok(())
    }
}
