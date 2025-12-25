use crate::{asset_manager::Asset, game::Game};

use anyhow::Result;

impl Game {
    pub(super) async fn load(&mut self) -> Result<()> {
        self.assets.load(Asset::Hex, "assets/hex.png").await?;
        Ok(())
    }
}
