use std::{
    cmp::Ordering,
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
};

use crate::game::map::{Map, tile::Tile};

use anyhow::Result;
use hexx::Hex;
use macroquad::prelude::*;

impl Map {
    pub async fn load_map(&mut self) -> Result<()> {
        self.tiles.clear();
        let data = load_string(&self.current_map_file).await?;
        let tiles = ron::from_str::<Vec<(Hex, Tile)>>(&data)?;
        self.tiles = HashMap::from_iter(tiles);
        Ok(())
    }

    pub async fn save_map(&self) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        {}

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut file = BufWriter::new(
                fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&self.current_map_file)?,
            );
            let mut tiles = Vec::from_iter(self.tiles.iter());
            tiles.sort_by(|a, b| match a.0.x.cmp(&b.0.x) {
                Ordering::Equal => a.0.y.cmp(&b.0.y),
                o => o,
            });

            let data = ron::ser::to_string_pretty(
                &tiles,
                ron::ser::PrettyConfig::default().compact_structs(true),
            )?;
            file.write_all(data.as_bytes())?;
        }

        Ok(())
    }
}
