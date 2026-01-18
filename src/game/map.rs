#![allow(dead_code)]

mod load_save;
pub mod tile;

use std::collections::HashMap;

use anyhow::Result;
use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    map::tile::{Tile, VertexType},
    theme::{Theme, ThemeColor},
};

pub struct Map {
    pub hex_layout: HexLayout,
    pub hex_size: f32,
    pub tile_variants: HashMap<Tile, (Vec<Vec2>, Vec<Vec2>)>,
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
            tile_variants: HashMap::new(),
            tiles: HashMap::new(),
            current_map_file: "assets/map/001".to_string(),

            mouse_target: Vec2::new(screen_width() / 2., screen_height() / 2.),
            smoothing_factor: 28.,
        }
    }
}

impl Map {
    pub async fn load_tile_variants(&mut self) -> Result<()> {
        self.tile_variants.insert(
            Tile::Empty,
            Tile::split_to_tile_variant([VertexType::Empty; 6]),
        );

        for v in 0..6 {
            {
                let mut split = [VertexType::Empty; 6];
                split[v % 6] = VertexType::Both;
                split[(v + 1) % 6] = VertexType::Full;
                split[(v + 2) % 6] = VertexType::Both;
                self.tile_variants.insert(
                    Tile::Small { rotation: v as u8 },
                    Tile::split_to_tile_variant(split),
                );
            }
            {
                let mut split = [VertexType::Empty; 6];
                split[v % 6] = VertexType::Both;
                split[(v + 1) % 6] = VertexType::Full;
                split[(v + 2) % 6] = VertexType::Full;
                split[(v + 3) % 6] = VertexType::Both;
                self.tile_variants.insert(
                    Tile::Half { rotation: v as u8 },
                    Tile::split_to_tile_variant(split),
                );
            }
            {
                let mut split = [VertexType::Full; 6];
                split[v % 6] = VertexType::Both;
                split[(v + 1) % 6] = VertexType::Empty;
                split[(v + 2) % 6] = VertexType::Both;
                self.tile_variants.insert(
                    Tile::Large { rotation: v as u8 },
                    Tile::split_to_tile_variant(split),
                );
            }
        }

        self.tile_variants.insert(
            Tile::Full,
            Tile::split_to_tile_variant([VertexType::Full; 6]),
        );
        Ok(())
    }
    pub fn draw(&mut self, theme: &Theme) {
        for (hex, tile) in self.tiles.iter() {
            let pos = h2q(self.hex_layout.hex_to_world_pos(*hex));
            tile.draw(
                pos,
                self.hex_size,
                theme.color(ThemeColor::Lighter),
                theme.color(ThemeColor::Dark),
                &self.tile_variants,
            );
        }
    }
}

#[inline]
pub fn h2q(v: hexx::Vec2) -> Vec2 {
    vec2(v.x, v.y)
}

#[inline]
pub fn q2h(v: Vec2) -> hexx::Vec2 {
    hexx::Vec2::new(v.x, v.y)
}
