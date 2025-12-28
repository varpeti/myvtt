use std::collections::HashMap;

use anyhow::{Result, anyhow};
use macroquad::prelude::*;

pub struct AssetManager {
    texture2d: HashMap<Asset, Texture2D>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            texture2d: HashMap::new(),
        }
    }

    pub async fn load(&mut self, asset: Asset, path: &str) -> Result<()> {
        self.texture2d.insert(asset, load_texture(path).await?);
        Ok(())
    }

    pub fn get(&mut self, asset: &Asset) -> Result<&Texture2D> {
        self.texture2d
            .get(asset)
            .ok_or_else(|| anyhow!("Asset `{:?}` not loaded!", asset))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Asset {
    Hex,
}
