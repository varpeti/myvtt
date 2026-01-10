use std::f32::{self, consts::FRAC_PI_6};

use macroquad::prelude::*;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub tile_type: TileType,
    rotation: u8,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Empty,
            rotation: 0,
        }
    }
}

impl Tile {
    pub fn new(tile_type: TileType, rotation: u8) -> Self {
        Self {
            tile_type,
            rotation: rotation % 6,
        }
    }

    pub fn draw(&self, pos: Vec2, size: f32, empty_color: Color, full_color: Color) {
        let mut split = [VertexType::Empty; 6];
        match self.tile_type {
            TileType::Empty => (),
            TileType::Small => {
                let v = self.rotation as usize;
                split[v % 6] = VertexType::Both;
                split[(v + 1) % 6] = VertexType::Full;
                split[(v + 2) % 6] = VertexType::Both;
            }
            TileType::Half => {
                let v = self.rotation as usize;
                split[v % 6] = VertexType::Both;
                split[(v + 1) % 6] = VertexType::Full;
                split[(v + 2) % 6] = VertexType::Full;
                split[(v + 3) % 6] = VertexType::Both;
            }
            TileType::Large => {
                let v = self.rotation as usize;
                split[v % 6] = VertexType::Both;
                for i in 1..=3 {
                    split[(v + i) % 6] = VertexType::Full;
                }
                split[(v + 4) % 6] = VertexType::Both;
            }
            TileType::Full => split = [VertexType::Full; 6],
        }

        let mut empty = Vec::with_capacity(6);
        let mut full = Vec::with_capacity(6);
        for (i, vertex_type) in split.into_iter().enumerate() {
            let v = Vec2::from_angle(i as f32 * f32::consts::FRAC_PI_3 + FRAC_PI_6) * size + pos;
            match vertex_type {
                VertexType::Empty => empty.push(v),
                VertexType::Both => {
                    empty.push(v);
                    full.push(v);
                }
                VertexType::Full => {
                    full.push(v);
                }
            }
        }

        if empty.len() >= 3 {
            for i in 1..empty.len() - 1 {
                draw_triangle(empty[0], empty[i], empty[i + 1], empty_color);
            }
        }
        if full.len() >= 3 {
            for i in 1..full.len() - 1 {
                draw_triangle(full[0], full[i], full[i + 1], full_color);
            }
        }

        draw_circle(pos.x, pos.y, 2., full_color);
    }

    pub fn rotate(&mut self, delta: i8) {
        self.rotation = (self.rotation as i8 + delta).rem_euclid(6) as u8;
    }

    pub fn rotation(&self) -> u8 {
        self.rotation
    }

    pub fn is_empty_or_full(&self) -> bool {
        self.tile_type == TileType::Empty || self.tile_type == TileType::Full
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, AsRefStr)]
pub enum TileType {
    Empty,
    Small,
    Half,
    Large,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VertexType {
    Empty,
    Both,
    Full,
}
