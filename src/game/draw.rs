use crate::{asset_manager::Asset, game::Game};

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn c2(&mut self) -> Result<()> {
        self.camera.activate();
        clear_background(Color::from_hex(0x191724));
        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<()> {
        for hex in self.state.origin_hex_neighbours.iter() {
            let pos = self.config.hex_layout.hex_to_world_pos(*hex);
            draw_texture(
                self.assets.get(&Asset::Hex)?,
                pos.x - self.config.hex_layout.scale.x,
                pos.y - self.config.hex_layout.scale.y,
                WHITE,
            );
            draw_hexagon(
                pos.x,
                pos.y,
                self.config.hex_layout.scale.x - 1.,
                1.,
                true,
                GREEN,
                Color::from_rgba(0, 0, 0, 0),
            );
            let offset = (
                self.config.hex_layout.rect_size().x / 2.,
                self.config.hex_layout.scale.y / 2.,
            );
            draw_hexagon(
                pos.x - offset.0,
                pos.y - offset.1,
                self.config.hex_layout.scale.x - 1.,
                2.,
                true,
                PURPLE,
                Color::from_rgba(0, 0, 0, 0),
            );
        }

        let pos = self
            .config
            .hex_layout
            .hex_to_world_pos(self.state.hovered_hex);
        draw_hexagon(
            pos.x,
            pos.y,
            self.config.hex_layout.scale.x - 1.,
            3.,
            true,
            Color::from_rgba(0xeb, 0xbc, 0xba, 0xff),
            Color::from_rgba(0xeb, 0xbc, 0xba, 0x80),
        );

        for hex in self.state.hovered_hex.all_neighbors().iter() {
            let pos = self.config.hex_layout.hex_to_world_pos(*hex);
            draw_hexagon(
                pos.x,
                pos.y,
                self.config.hex_layout.scale.x - 1.,
                3.,
                true,
                Color::from_rgba(0xeb, 0xbc, 0xba, 0x80),
                Color::from_rgba(0x0, 0x0, 0x0, 0x00),
            );
        }

        draw_circle(0., 0., 1., RED);
        Ok(())
    }
}
