use crate::game::{map::assets::create_hexagonal_prism, theme::Theme};

use anyhow::{Result, anyhow};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub height: u8,
    pub color: Theme,
}
impl Node {
    pub fn new(height: u8, color: Theme) -> Self {
        Self { height, color }
    }

    pub fn from_str(line: &str) -> Result<(i32, i32, Self)> {
        let mut data = line.split_whitespace();
        let x = data
            .next()
            .ok_or_else(|| anyhow!("'x'"))?
            .parse::<i32>()
            .map_err(|err| anyhow!("'x' as i32: `{}`", err))?;
        let y = data
            .next()
            .ok_or_else(|| anyhow!("'y'"))?
            .parse::<i32>()
            .map_err(|err| anyhow!("'y' as i32: `{}`", err))?;
        let height = data
            .next()
            .ok_or_else(|| anyhow!("'height'"))?
            .parse::<u8>()
            .map_err(|err| anyhow!("'height' as u8: `{}`", err))?;
        let node_type = data
            .next()
            .ok_or_else(|| anyhow!("'color'"))?
            .parse::<Theme>()
            .map_err(|err| anyhow!("'color' as Theme `{}`", err))?;
        Ok((x, y, Self::new(height, node_type)))
    }

    pub fn draw_node(
        pos: Vec2,
        radius: f32,
        height: f32,
        texture: Option<Texture2D>,
        color: Color,
    ) -> Result<()> {
        let mesh = create_hexagonal_prism(
            pos,
            radius,
            height,
            color,
            Theme::PillarOffset.color(),
            texture,
        );
        draw_mesh(&mesh);
        Ok(())
    }
}
