#![allow(dead_code)]

mod default;
mod load_save;
mod tiles;

use std::{cmp::Ordering, collections::HashMap};

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

        let hoovered_hex = self
            .hex_layout
            .world_pos_to_hex(q2h(camera.screen_to_world(mouse_position().into())));

        if self.brush_events.pop(&BrushEvent::CloneTile)
            && let Some(tile) = self.tiles.get(&hoovered_hex)
        {
            self.brush = *tile;
        }
        if self.brush_events.pop(&BrushEvent::Draw) {
            self.tiles.insert(hoovered_hex, self.brush);
            self.save_map().await?;
        }
        if self.brush_events.pop(&BrushEvent::Remove) {
            self.tiles.remove(&hoovered_hex);
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
