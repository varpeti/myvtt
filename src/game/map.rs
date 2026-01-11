#![allow(dead_code)]

mod load_save;
pub mod tiles;

use std::collections::HashMap;

use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    map::tiles::Tile,
    theme::{Theme, ThemeColor},
};

pub struct Map {
    pub hex_layout: HexLayout,
    pub hex_size: f32,
    pub tiles: HashMap<Hex, Tile>,
    pub current_map_file: String,

    pub mouse_target: Vec2,
    pub smoothing_factor: f32,
}

impl Default for Map {
    fn default() -> Self {
        let hex_size = 32.;
        Self {
            hex_layout: HexLayout {
                orientation: hexx::HexOrientation::Pointy,
                origin: hexx::Vec2::ZERO,
                scale: hexx::Vec2::new(hex_size, hex_size),
            },
            hex_size,
            tiles: HashMap::new(),
            current_map_file: "assets/map/001".to_string(),

            mouse_target: Vec2::new(screen_width() / 2., screen_height() / 2.),
            smoothing_factor: 28.,
        }
    }
}

impl Map {
    pub fn draw(&mut self, theme: &Theme) {
        for (hex, tile) in self.tiles.iter() {
            let pos = h2q(self.hex_layout.hex_to_world_pos(*hex));
            tile.draw(
                pos,
                self.hex_size,
                theme.color(ThemeColor::Lighter),
                theme.color(ThemeColor::Dark),
            );
        }
    }

    // pub fn update(&mut self, camera: &Camera2D, dt: f32) -> Result<()> {
    //     let mouse_pos = h2q(self.hex_layout.hex_to_world_pos(
    //         self.hex_layout
    //             .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into()))),
    //     ));
    //
    //     let d = mouse_pos - self.mouse_target;
    //     if d.length() > 0.01 {
    //         self.mouse_target += d * self.smoothing_factor * dt;
    //     }
    //
    //     Ok(())
    // }
    //
    // pub fn draw_mouse_target(&self, theme: &Theme) {
    //     let hoovered_hex = self.hex_layout.world_pos_to_hex(q2h(self.mouse_target));
    //     let offset = q2h(self.mouse_target) - self.hex_layout.hex_to_world_pos(hoovered_hex);
    //
    //     let pos = self.hex_layout.hex_to_world_pos(hoovered_hex) + offset;
    //     draw_hexagon(
    //         pos.x,
    //         pos.y,
    //         self.hex_size,
    //         5.,
    //         true,
    //         theme.color(ThemeColor::Normal),
    //         Color::default().with_alpha(0.),
    //     );
    // }
}

#[inline]
pub fn h2q(v: hexx::Vec2) -> Vec2 {
    vec2(v.x, v.y)
}

#[inline]
pub fn q2h(v: Vec2) -> hexx::Vec2 {
    hexx::Vec2::new(v.x, v.y)
}
