use std::collections::HashMap;

use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    map::h2q,
    theme::{Theme, ThemeColor},
};

#[derive(Debug, Clone)]
pub struct Entity {
    pub hex: Hex,
    pub image: String,
    pub alpha: f32,
    pub rotation: f32,
    pub size: f32,
    pub data: Option<EntityData>,
}

impl Entity {
    pub fn new(pos: Hex, image: String) -> Self {
        Self {
            hex: pos,
            image,
            alpha: 1.,
            rotation: 0.,
            size: 1.,
            data: None,
        }
    }

    pub fn new_with_size(pos: Hex, image: String, size: f32) -> Self {
        Self {
            hex: pos,
            image,
            alpha: 1.,
            rotation: 0.,
            size,
            data: None,
        }
    }

    pub fn draw_to(
        &self,
        pos: Vec2,
        hex_inradius_size: f32,
        textures: &HashMap<String, Texture2D>,
        esp: bool,
        theme: &Theme,
    ) {
        let size = self.size * hex_inradius_size;
        if let Some(texture) = textures.get(&self.image) {
            draw_texture_ex(
                texture,
                pos.x - size / 2.,
                pos.y - size / 2.,
                WHITE.with_alpha(self.alpha),
                DrawTextureParams {
                    dest_size: Some(Vec2::new(size, size)),
                    rotation: self.rotation,
                    ..Default::default()
                },
            );
        } else {
            draw_circle(pos.x, pos.y, size / 2., PINK.with_alpha(0.75));
        }

        if esp {
            draw_circle_lines(
                pos.x,
                pos.y,
                hex_inradius_size / 2.,
                2.,
                theme.color(ThemeColor::Normal).with_alpha(0.75),
            );
        }
    }

    pub fn draw(
        &self,
        hex_layout: &HexLayout,
        hex_inradius_size: f32,
        textures: &HashMap<String, Texture2D>,
        esp: bool,
        theme: &Theme,
    ) {
        let pos = h2q(hex_layout.hex_to_world_pos(self.hex));
        self.draw_to(pos, hex_inradius_size, textures, esp, theme);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityData {}
