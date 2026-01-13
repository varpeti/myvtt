use std::collections::HashMap;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Entity {
    pub image: String,
    pub alpha: f32,
    pub rotation: f32,
    pub size: f32,
    pub data: Option<EntityData>,
}

impl Entity {
    pub fn new(image: String) -> Self {
        Self {
            image,
            alpha: 1.,
            rotation: 0.,
            size: 1.,
            data: None,
        }
    }

    pub fn new_with_size(image: String, size: f32) -> Self {
        Self {
            image,
            alpha: 1.,
            rotation: 0.,
            size,
            data: None,
        }
    }

    pub fn draw(&self, pos: Vec2, hex_inradius_size: Vec2, textures: &HashMap<String, Texture2D>) {
        let size = self.size * hex_inradius_size;
        if let Some(texture) = textures.get(&self.image) {
            draw_texture_ex(
                texture,
                pos.x - size.x / 2.,
                pos.y - size.y / 2.,
                WHITE.with_alpha(self.alpha),
                DrawTextureParams {
                    dest_size: Some(size),
                    rotation: self.rotation,
                    ..Default::default()
                },
            );
        } else {
            draw_circle(pos.x, pos.y, 16., PINK.with_alpha(0.75));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityData {}
