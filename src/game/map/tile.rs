use std::{
    collections::HashMap,
    f32::{self, consts::FRAC_PI_6},
};

use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Small { rotation: u8 },
    Half { rotation: u8 },
    Large { rotation: u8 },
    Full,
}

impl Tile {
    pub fn draw(
        &self,
        pos: Vec2,
        size: f32,
        empty_color: Color,
        full_color: Color,
        tile_variants: &HashMap<Tile, (Vec<Vec2>, Vec<Vec2>)>,
    ) {
        if let Some((empty_v, full_v)) = tile_variants.get(self) {
            Self::draw_vertecies(pos, size, empty_color, empty_v);
            Self::draw_vertecies(pos, size, full_color, full_v);
        }
        draw_circle(pos.x, pos.y, 3., full_color);
    }

    pub fn draw_vertecies(pos: Vec2, size: f32, color: Color, vertecies: &[Vec2]) {
        if vertecies.len() >= 3 {
            for i in 1..vertecies.len() - 1 {
                draw_triangle(
                    vertecies[0] * size + pos,
                    vertecies[i] * size + pos,
                    vertecies[i + 1] * size + pos,
                    color,
                );
            }
        }
    }

    pub fn rotate(&mut self, delta: i8) {
        match self {
            Tile::Small { rotation } | Tile::Half { rotation } | Tile::Large { rotation } => {
                *rotation = (*rotation as i8 + delta).rem_euclid(6) as u8;
            }
            _ => (),
        }
    }
    pub fn is_empty_or_full(&self) -> bool {
        matches!(self, Tile::Empty | Tile::Full)
    }

    pub fn split_to_tile_variant(split: [VertexType; 6]) -> (Vec<Vec2>, Vec<Vec2>) {
        let mut empty = Vec::with_capacity(6);
        let mut full = Vec::with_capacity(6);
        for (i, vertex_type) in split.into_iter().enumerate() {
            let v = Vec2::from_angle(i as f32 * f32::consts::FRAC_PI_3 + FRAC_PI_6);
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
        (empty, full)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexType {
    Empty,
    Both,
    Full,
}
