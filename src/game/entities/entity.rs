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
    image: String,
    alpha: f32,
    rotation: f32,
    size: f32,

    pub smoothing_factor: f32,
    pub to_alpha: f32,
    pub to_rotation: f32,
    pub to_size: f32,
}

impl Entity {
    pub fn new(hex: Hex, image: String) -> Self {
        Self::new_with_size(hex, image, 1.)
    }

    pub fn new_with_size(hex: Hex, image: String, size: f32) -> Self {
        Self {
            hex,
            image,
            alpha: 1.,
            rotation: 0.,
            size,

            smoothing_factor: 5.,
            to_alpha: 1.,
            to_rotation: 0.,
            to_size: size,
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
                theme.color(ThemeColor::Normal).with_alpha(0.25),
            );
            if (self.size - 1.).abs() > f32::EPSILON {
                draw_circle_lines(
                    pos.x,
                    pos.y,
                    size / 2.,
                    2.,
                    theme.color(ThemeColor::Normal).with_alpha(0.25),
                );
            }
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

    pub fn update(&mut self, dt: f32) {
        let d = self.to_alpha - self.alpha;
        if d.abs() > 0.01 {
            self.alpha += d * self.smoothing_factor * dt;
        }
        let d = self.to_rotation - self.rotation;
        if d.abs() > 0.01 {
            self.rotation += d * self.smoothing_factor * dt;
        }
        let d = self.to_size - self.size;
        if d.abs() > 0.01 {
            self.size += d * self.smoothing_factor * dt;
        }
    }
}
