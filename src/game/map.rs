#![allow(dead_code)]

mod default;
mod load_save;
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

pub struct Map {
    hex_layout: HexLayout,
    hex_size: f32,
    tiles: HashMap<Hex, Tile>,
    brush: Tile,
    brush_size: u32,
    pub current_map_file: String,

    brush_events: Events<BrushEvent>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BrushEvent {
    PickEmpty,
    PickSmall,
    PickHalf,
    PickLarge,
    PickFull,
    RotateClockwise,
    RotateAntiClockwise,
    CloneTile,
    Draw,
    Remove,
    SizeUp,
    SizeDown,
    InsertWalls,
}

impl Event for BrushEvent {}

impl Map {
    pub async fn handle_events(&mut self, camera: &mut Camera2D) -> Result<()> {
        self.brush_events.update();

        if self.brush_events.pop(&BrushEvent::PickEmpty) {
            self.brush.tile_type = TileType::Empty;
        }
        if self.brush_events.pop(&BrushEvent::PickSmall) {
            self.brush.tile_type = TileType::Small;
        }
        if self.brush_events.pop(&BrushEvent::PickHalf) {
            self.brush.tile_type = TileType::Half;
        }
        if self.brush_events.pop(&BrushEvent::PickLarge) {
            self.brush.tile_type = TileType::Large;
        }
        if self.brush_events.pop(&BrushEvent::PickFull) {
            self.brush.tile_type = TileType::Full;
        }
        if self.brush_events.pop(&BrushEvent::RotateClockwise) {
            self.brush.rotate(1);
        }
        if self.brush_events.pop(&BrushEvent::RotateAntiClockwise) {
            self.brush.rotate(-1);
        }
        if self.brush_events.pop(&BrushEvent::SizeUp) {
            self.brush_size = self.brush_size.saturating_add(1);
        }
        if self.brush_events.pop(&BrushEvent::SizeDown) {
            self.brush_size = self.brush_size.saturating_sub(1);
        }

        if !self.brush.is_empty_or_full() {
            self.brush_size = 0;
        }

        let hoovered_hex = self
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if self.brush_events.pop(&BrushEvent::CloneTile)
            && let Some(tile) = self.tiles.get(&hoovered_hex)
        {
            self.brush = *tile;
        }
        if self.brush_events.pop(&BrushEvent::Draw) {
            if self.brush.is_empty_or_full() {
                for hex in hoovered_hex.range(self.brush_size) {
                    self.tiles.insert(hex, self.brush);
                }
            } else {
                self.tiles.insert(hoovered_hex, self.brush);
            }
            self.save_map().await?;
        }
        if self.brush_events.pop(&BrushEvent::Remove) {
            if self.brush.is_empty_or_full() {
                for hex in hoovered_hex.range(self.brush_size) {
                    self.tiles.remove(&hex);
                }
            } else {
                self.tiles.remove(&hoovered_hex);
            }
            self.save_map().await?;
        }

        if self.brush_events.pop(&BrushEvent::InsertWalls) {
            self.insert_walls();
            self.save_map().await?;
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
        let hoovered_hex = self
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if self.brush.is_empty_or_full() {
            for hex in hoovered_hex.range(self.brush_size) {
                let pos = h2q(self.hex_layout.hex_to_world_pos(hex));
                self.brush.draw(
                    pos,
                    self.hex_size,
                    theme.color(ThemeColor::Light).with_alpha(0.5),
                    theme.color(ThemeColor::Normal).with_alpha(0.5),
                );
            }
        } else {
            let pos = h2q(self.hex_layout.hex_to_world_pos(hoovered_hex));
            self.brush.draw(
                pos,
                self.hex_size,
                theme.color(ThemeColor::Light).with_alpha(0.5),
                theme.color(ThemeColor::Normal).with_alpha(0.5),
            );
        }
    }

    pub fn insert_walls(&mut self) {
        let mc = self.tiles.clone();
        for (hex, tile) in mc.iter() {
            if tile.tile_type != TileType::Empty {
                continue;
            }
            for n0 in hex.all_neighbors() {
                if self.tiles.contains_key(&n0) {
                    continue;
                }

                let mut ns = [false; 6];
                for (v, n1) in n0.all_neighbors().into_iter().enumerate() {
                    if let Some(tile) = self.tiles.get(&n1)
                        && tile.tile_type == TileType::Empty
                    {
                        ns[v] = true;
                    }
                }

                let tile = match ns {
                    // 6
                    [true, true, true, true, true, true] => Tile::new(TileType::Empty, 0),

                    // 5
                    [true, true, true, true, true, false] => Tile::new(TileType::Empty, 0),
                    [true, true, true, true, false, true] => Tile::new(TileType::Empty, 0),
                    [true, true, true, false, true, true] => Tile::new(TileType::Empty, 0),
                    [true, true, false, true, true, true] => Tile::new(TileType::Empty, 0),
                    [true, false, true, true, true, true] => Tile::new(TileType::Empty, 0),
                    [false, true, true, true, true, true] => Tile::new(TileType::Empty, 0),

                    // 4
                    [true, true, true, true, false, false] => Tile::new(TileType::Small, 3),
                    [false, true, true, true, true, false] => Tile::new(TileType::Small, 4),
                    [false, false, true, true, true, true] => Tile::new(TileType::Small, 5),
                    [true, false, false, true, true, true] => Tile::new(TileType::Small, 0),
                    [true, true, false, false, true, true] => Tile::new(TileType::Small, 1),
                    [true, true, true, false, false, true] => Tile::new(TileType::Small, 2),

                    // 3
                    [true, true, true, false, false, false] => Tile::new(TileType::Half, 2),
                    [false, true, true, true, false, false] => Tile::new(TileType::Half, 3),
                    [false, false, true, true, true, false] => Tile::new(TileType::Half, 4),
                    [false, false, false, true, true, true] => Tile::new(TileType::Half, 5),
                    [true, false, false, false, true, true] => Tile::new(TileType::Half, 0),
                    [true, true, false, false, false, true] => Tile::new(TileType::Half, 1),

                    // 2
                    [true, true, false, false, false, false] => Tile::new(TileType::Large, 1),
                    [false, true, true, false, false, false] => Tile::new(TileType::Large, 2),
                    [false, false, true, true, false, false] => Tile::new(TileType::Large, 3),
                    [false, false, false, true, true, false] => Tile::new(TileType::Large, 4),
                    [false, false, false, false, true, true] => Tile::new(TileType::Large, 5),
                    [true, false, false, false, false, true] => Tile::new(TileType::Large, 0),

                    // 1
                    [true, false, false, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, true, false, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, true, false, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, true, false, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, false, true, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, false, false, true] => Tile::new(TileType::Full, 0),

                    // 0
                    [false, false, false, false, false, false] => Tile::new(TileType::Full, 0),

                    _ => continue,
                };
                self.tiles.insert(n0, tile);
            }
        }
    }
}

pub fn h2q(v: hexx::Vec2) -> Vec2 {
    vec2(v.x, v.y)
}

pub fn q2h(v: Vec2) -> hexx::Vec2 {
    hexx::Vec2::new(v.x, v.y)
}
