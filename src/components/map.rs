use std::fs;

use anyhow::{Result, anyhow};
use hexx::{HexLayout, HexOrientation};

#[derive(Debug)]
pub struct Map {
    hex_layout: HexLayout,
    map: Vec<Vec<Node>>,
}

impl Map {
    pub fn new(size: f32) -> Self {
        Self {
            hex_layout: HexLayout {
                scale: hexx::Vec2::new(size, size),
                orientation: HexOrientation::Pointy,
                origin: hexx::Vec2::new(0., 0.),
            },
            map: Vec::new(),
        }
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        for (i, line) in content.lines().enumerate() {
            Node::from_str(line)
                .map_err(|err| anyhow!("Expected {} | line: @{}: `{}`", err, i + 1, line));
        }
        Ok(())
    }
}

impl Default for Map {
    fn default() -> Self {
        Self::new(64.)
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    height: u8,
    node_type: NodeType,
}
impl Node {
    pub fn new(height: u8, node_type: NodeType) -> Self {
        Self { height, node_type }
    }

    pub fn from_str(line: &str) -> Result<(usize, usize, Self)> {
        let mut data = line.split_whitespace();
        let x = data
            .next()
            .ok_or_else(|| anyhow!("'x'"))?
            .parse::<usize>()
            .map_err(|err| anyhow!("'x' as usize: `{}`", err))?;
        let y = data
            .next()
            .ok_or_else(|| anyhow!("'y'"))?
            .parse::<usize>()
            .map_err(|err| anyhow!("'y' as usize: `{}`", err))?;
        let height = data
            .next()
            .ok_or_else(|| anyhow!("'height'"))?
            .parse::<u8>()
            .map_err(|err| anyhow!("'height' as u8: `{}`", err))?;
        let node_type = NodeType::from_str(data.next().ok_or_else(|| anyhow!("'node_type'"))?)
            .map_err(|err| anyhow!("'node_type' as NodeType `{}`", err))?;
        Ok((x, y, Self::new(height, node_type)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Ground,
    Water,
    WallVertical,
    WallHorizontal,
    WallDiagonalUpRight,
    WallDiagonalDownRight,
}
impl NodeType {
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "Ground" => Self::Ground,
            "Water" => Self::Water,
            "WallVertical" => Self::WallVertical,
            "WallHorizontal" => Self::WallHorizontal,
            "WallDiagonalUpRight" => Self::WallDiagonalUpRight,
            "WallDiagonalDownRight" => Self::WallDiagonalDownRight,
            oth => return Err(anyhow!("Invalid NodeType: `{}`", oth)),
        })
    }
}
