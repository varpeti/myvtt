#![allow(dead_code)]

mod tiles;

use std::collections::HashMap;

use anyhow::Result;
use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    events::{Event, Events},
    map::tiles::{Tile, TileType},
    theme::{Theme, ThemeColor},
};

#[derive(Debug)]
pub struct Map {
    hex_layout: HexLayout,
    hex_size: f32,
    tiles: HashMap<Hex, Tile>,
    brush: Tile,
}

impl Default for Map {
    fn default() -> Self {
        let tiles = HashMap::from([
            (Hex::new(0, 0), Tile::new(TileType::Empty, 0)),
            (Hex::new(-1, 2), Tile::new(TileType::Small, 0)),
            (Hex::new(0, 2), Tile::new(TileType::Small, 1)),
            (Hex::new(1, 2), Tile::new(TileType::Small, 2)),
            (Hex::new(2, 2), Tile::new(TileType::Small, 3)),
            (Hex::new(3, 2), Tile::new(TileType::Small, 4)),
            (Hex::new(4, 2), Tile::new(TileType::Small, 5)),
            (Hex::new(-2, 4), Tile::new(TileType::Half, 0)),
            (Hex::new(-1, 4), Tile::new(TileType::Half, 1)),
            (Hex::new(0, 4), Tile::new(TileType::Half, 2)),
            (Hex::new(1, 4), Tile::new(TileType::Half, 3)),
            (Hex::new(2, 4), Tile::new(TileType::Half, 4)),
            (Hex::new(3, 4), Tile::new(TileType::Half, 5)),
            (Hex::new(-3, 6), Tile::new(TileType::Full, 0)),
        ]);

        let hex_size = 32.;
        Self {
            hex_layout: HexLayout {
                orientation: hexx::HexOrientation::Pointy,
                origin: hexx::Vec2::ZERO,
                scale: hexx::Vec2::new(hex_size, hex_size),
            },
            hex_size,
            tiles,
            brush: Tile::new(TileType::Empty, 0),
        }
    }
}

impl Map {
    pub fn handle_events(&mut self, events: &mut Events, camera: &mut Camera2D) -> Result<()> {
        if events.pop(&Event::BrushPickEmpty) {
            self.brush.tile_type = TileType::Empty;
        }
        if events.pop(&Event::BrushPickSmall) {
            self.brush.tile_type = TileType::Small;
        }
        if events.pop(&Event::BrushPickHalf) {
            self.brush.tile_type = TileType::Half;
        }
        if events.pop(&Event::BrushPickFull) {
            self.brush.tile_type = TileType::Full;
        }
        if events.pop(&Event::BrushRotateClockwise) {
            self.brush.rotation(1);
        }
        if events.pop(&Event::BrushRotateAntiClockwise) {
            self.brush.rotation(-1);
        }

        let hoovered_hex = self
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if events.pop(&Event::BrushCloneTile)
            && let Some(tile) = self.tiles.get(&hoovered_hex)
        {
            self.brush = *tile;
        }
        if events.pop(&Event::BrushDraw) {
            self.tiles.insert(hoovered_hex, self.brush);
        }
        if events.pop(&Event::BrushRemove) {
            self.tiles.remove(&hoovered_hex);
        }

        Ok(())
    }

    pub fn draw_map(&mut self, theme: &Theme) {
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

    pub fn draw_camera_target(&self, theme: &Theme, camera: &Camera2D) {
        let pos = self
            .hex_layout
            .hex_to_world_pos(self.hex_layout.world_pos_to_hex(q2h(camera.target)));

        draw_hexagon(
            pos.x,
            pos.y,
            self.hex_size,
            2.,
            true,
            theme.color(ThemeColor::Normal),
            Color::default().with_alpha(0.),
        );
    }

    pub fn draw_mouse_target(&self, theme: &Theme, camera: &Camera2D) {
        let pos = self.hex_layout.hex_to_world_pos(
            self.hex_layout
                .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into()))),
        );

        draw_hexagon(
            pos.x,
            pos.y,
            self.hex_size,
            5.,
            true,
            theme.color(ThemeColor::Normal),
            Color::default().with_alpha(0.),
        );
    }

    pub fn draw_brush(&self, theme: &Theme, camera: &Camera2D) {
        let pos = h2q(self.hex_layout.hex_to_world_pos(
            self.hex_layout
                .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into()))),
        ));

        self.brush.draw(
            pos,
            self.hex_size,
            theme.color(ThemeColor::Lighter).with_alpha(0.5),
            theme.color(ThemeColor::Dark).with_alpha(0.5),
        );
    }
}

pub fn h2q(v: hexx::Vec2) -> Vec2 {
    vec2(v.x, v.y)
}

pub fn q2h(v: Vec2) -> hexx::Vec2 {
    hexx::Vec2::new(v.x, v.y)
}
