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
            Color::from_rgba(0xe0, 0xde, 0xf4, 0xff),
            Color::from_rgba(0xe0, 0xde, 0xf4, 0x80),
        );

        for hex in self.state.hovered_hex.all_neighbors().iter() {
            let pos = self.config.hex_layout.hex_to_world_pos(*hex);
            draw_hexagon(
                pos.x,
                pos.y,
                self.config.hex_layout.scale.x - 1.,
                3.,
                true,
                Color::from_rgba(0xe0, 0xde, 0xf4, 0x80),
                Color::from_rgba(0x0, 0x0, 0x0, 0x00),
            );
        }

        let offset = (
            self.config.hex_layout.rect_size().x / 2.,
            self.config.hex_layout.scale.y / 2.,
        );
        let hex = self
            .config
            .hex_layout
            .world_pos_to_hex(hexx::Vec2::new(0., 0.));
        let pos = self.config.hex_layout.hex_to_world_pos(hex);
        let size = self.config.hex_layout.rect_size();
        draw_texture_ex(
            self.assets.get(&Asset::Hex)?,
            pos.x - size.x / 2. + offset.0,
            pos.y - size.y / 2. + offset.1,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(size.x, size.y)),
                source: None,
                rotation: 0.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        for hex in hex.all_neighbors().iter() {
            let pos = self.config.hex_layout.hex_to_world_pos(*hex);
            draw_texture_ex(
                self.assets.get(&Asset::Hex)?,
                pos.x - size.x / 2. + offset.0,
                pos.y - size.y / 2. + offset.1,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(size.x, size.y)),
                    source: None,
                    rotation: 0.,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }

        draw_circle(0., 0., 3., RED);

        Ok(())
    }
}
