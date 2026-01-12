use std::collections::HashMap;

use macroquad::prelude::*;

#[derive(Debug)]
pub struct Entity {
    pub image: String,
    pub alpha: f32,
    pub rotation: f32,
    pub data: Option<EntityData>,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            image: String::new(),
            alpha: 1.,
            rotation: 0.,
            data: None,
        }
    }
}

impl Entity {
    pub fn new(image: String) -> Self {
        Self {
            image,
            alpha: 1.,
            rotation: 0.,
            data: None,
        }
    }

    pub fn draw(&self, pos: Vec2, size: Vec2, textures: &HashMap<String, Texture2D>) {
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
