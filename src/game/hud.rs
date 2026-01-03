use anyhow::Result;
use macroquad::prelude::*;

use crate::game::{camera::RPGCamera, map::Map, theme::Theme};

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

    pub fn draw(&mut self, camera: &RPGCamera, map: &Map) -> Result<()> {
        set_default_camera();

        let (mx, my) = mouse_position();
        let (hhex, hnode) = map.hoovered_node();
        let target = camera.get_target();

        draw_multiline_text(
            &format!(
                "mouse: {:+05} {:+05}\nhoovered: {:+05} {:+05} {:?}\nbrush: {:?}\ncamera: {:+5} {:+5} {} {:.3}",
                mx,
                my,
                hhex.x,
                hhex.y,
                hnode,
                map.brush_node(),
                camera.get_target().x as i32,
                camera.get_target().y as i32,
                camera.get_zoom() as i32,
                camera.get_rotation(),
            ),
            12.,
            42.,
            32.,
            None,
            Theme::LotusCyan.color(),
        );

        draw_fps();
        Ok(())
    }
}
