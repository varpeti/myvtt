use std::{
    any::TypeId,
    f32, fs,
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
        // let content = load_string(&self.current_map_file).await?;
        // for (i, line) in content.lines().enumerate() {
        //     let (hex, tile) = parse_line(line)
        //         .map_err(|err| anyhow!("Error: #{} line: `{}`: Expected {}", i, line, err))?;
        //     self.tiles.insert(hex, tile);
        // }

        // TODO: Remove

        for dir in 0..12 {
            let v = hexx::Vec2::from_angle(dir as f32 * f32::consts::FRAC_PI_6);
            let hs = self.hex_layout.world_pos_to_hex(v * 0. * self.hex_size);
            let he = self.hex_layout.world_pos_to_hex(v * 50. * self.hex_size);

            let line = hs.line_to(he).collect::<Vec<_>>();
            for hex in line.iter() {
                self.tiles.insert(*hex, Tile::new(TileType::Empty, 0));
                for n0 in hex.all_neighbors() {
                    self.tiles.insert(n0, Tile::new(TileType::Empty, 0));
                }
            }
        }

        for (hex, tile) in self.tiles.clone().iter() {
            if tile.tile_type != TileType::Empty {
                continue;
            }
            for n0 in hex.all_neighbors() {
                if let Some(tile) = self.tiles.get(&n0)
                    && tile.tile_type == TileType::Empty
                {
                    continue;
                }

                let mut ns = [false; 6];
                for (v, n1) in n0.all_neighbors().into_iter().enumerate() {
                    if let Some(tile) = self.tiles.get(&n1)
                        && tile.tile_type == TileType::Empty
                    {
                        ns[v] = true;
                    }
                }

                let tile = match ns {
                    // 6
                    [true, true, true, true, true, true] => Tile::new(TileType::Empty, 0),

                    // 5
                    [true, true, true, true, true, false] => Tile::new(TileType::Empty, 2),
                    [true, true, true, true, false, true] => Tile::new(TileType::Empty, 3),
                    [true, true, true, false, true, true] => Tile::new(TileType::Empty, 4),
                    [true, true, false, true, true, true] => Tile::new(TileType::Empty, 5),
                    [true, false, true, true, true, true] => Tile::new(TileType::Empty, 0),
                    [false, true, true, true, true, true] => Tile::new(TileType::Empty, 1),

                    // 4
                    [true, true, true, true, false, false] => Tile::new(TileType::Empty, 2),
                    [false, true, true, true, true, false] => Tile::new(TileType::Empty, 3),
                    [false, false, true, true, true, true] => Tile::new(TileType::Empty, 4),
                    [true, false, false, true, true, true] => Tile::new(TileType::Empty, 5),
                    [true, true, false, false, true, true] => Tile::new(TileType::Empty, 0),
                    [true, true, true, false, false, true] => Tile::new(TileType::Empty, 1),

                    // 3
                    [true, true, true, false, false, false] => Tile::new(TileType::Half, 2),
                    [false, true, true, true, false, false] => Tile::new(TileType::Half, 3),
                    [false, false, true, true, true, false] => Tile::new(TileType::Half, 4),
                    [false, false, false, true, true, true] => Tile::new(TileType::Half, 5),
                    [true, false, false, false, true, true] => Tile::new(TileType::Half, 0),
                    [true, true, false, false, false, true] => Tile::new(TileType::Half, 1),

                    // 2
                    [true, true, false, false, false, false] => Tile::new(TileType::Small, 2),
                    [false, true, true, false, false, false] => Tile::new(TileType::Small, 3),
                    [false, false, true, true, false, false] => Tile::new(TileType::Small, 4),
                    [false, false, false, true, true, false] => Tile::new(TileType::Small, 5),
                    [false, false, false, false, true, true] => Tile::new(TileType::Small, 0),
                    [true, false, false, false, false, true] => Tile::new(TileType::Small, 1),

                    // 1
                    [true, false, false, false, false, false] => Tile::new(TileType::Full, 2),
                    [false, true, false, false, false, false] => Tile::new(TileType::Full, 3),
                    [false, false, true, false, false, false] => Tile::new(TileType::Full, 4),
                    [false, false, false, true, false, false] => Tile::new(TileType::Full, 5),
                    [false, false, false, false, true, false] => Tile::new(TileType::Full, 0),
                    [false, false, false, false, false, true] => Tile::new(TileType::Full, 1),

                    // 0
                    [false, false, false, false, false, false] => Tile::new(TileType::Full, 0),

                    _ => continue,
                };
                self.tiles.insert(n0, tile);
            }
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
        for (hex, tile) in self.tiles.iter() {
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
