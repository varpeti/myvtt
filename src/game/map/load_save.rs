use std::{
    cmp::Ordering,
    fs,
    io::{BufWriter, Write},
};

use crate::game::map::{
    Map,
    tiles::{Tile, TileType},
};

use anyhow::{Result, anyhow};
use hexx::Hex;
use macroquad::prelude::*;

impl Map {
    pub async fn load_map(&mut self) -> Result<()> {
        self.tiles.clear();
        let content = load_string(&self.current_map_file).await?;
        for (i, line) in content.lines().enumerate() {
            let (hex, tile) = parse_line(line)
                .map_err(|err| anyhow!("Error: #{} line: `{}`: Expected {}", i, line, err))?;
            self.tiles.insert(hex, tile);
        }

        Ok(())
    }

    pub async fn save_map(&self) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        return Ok(());

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

        for (hex, tile) in tiles {
            writeln!(
                file,
                "{:+03} {:+03} {} {}",
                hex.x,
                hex.y,
                tile.tile_type.as_ref(),
                tile.rotation()
            )?;
        }

        Ok(())
    }
}

fn parse_line(line: &str) -> Result<(Hex, Tile)> {
    let mut data = line.split_whitespace();

    let x = data
        .next()
        .ok_or_else(|| anyhow!("#1 x as isize"))?
        .parse::<i32>()
        .map_err(|err| anyhow!("#1 x as i32: {}", err))?;

    let y = data
        .next()
        .ok_or_else(|| anyhow!("#2 y as isize"))?
        .parse::<i32>()
        .map_err(|err| anyhow!("#2 x as i32: {}", err))?;

    let tile_type = data
        .next()
        .ok_or_else(|| anyhow!("#3 tile_type as TileType"))?
        .parse::<TileType>()
        .map_err(|err| anyhow!("#3 tile_type as TileType: {}", err))?;

    let rotation = data
        .next()
        .ok_or_else(|| anyhow!("#4 rotation as u8 between 0..6"))?
        .parse::<u8>()
        .map_err(|err| anyhow!("#4 rotation as u8 between 0..6: {}", err))?
        % 6;

    Ok((Hex::new(x, y), Tile::new(tile_type, rotation)))
}
