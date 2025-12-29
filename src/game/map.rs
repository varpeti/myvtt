use std::{
    collections::HashMap,
    f32::{self, consts::FRAC_PI_6},
    fs,
};

use anyhow::{Result, anyhow};
use hexx::{Hex, HexLayout, HexOrientation};
use macroquad::prelude::*;

pub struct Map {
    radius: f32,
    base_height: f32,
    hex_layout: HexLayout,
    map: HashMap<(i32, i32), Node>,
    meshes: HashMap<MeshName, Mesh>,
}

impl Default for Map {
    fn default() -> Self {
        let radius = 32.;
        Self {
            radius,
            base_height: 16.,
            hex_layout: HexLayout {
                scale: hexx::Vec2::new(radius, radius),
                orientation: HexOrientation::Pointy,
                origin: hexx::Vec2::new(0., 0.),
            },
            map: HashMap::new(),
            meshes: HashMap::new(),
        }
    }
}

impl Map {
    pub async fn load(&mut self, file_path: &str) -> Result<()> {
        for i in 1..9 {
            self.meshes.insert(
                MeshName::Hex {
                    node: Node::new(i, NodeType::Ground),
                },
                create_hexagonal_prism(
                    self.radius,
                    self.base_height * i as f32,
                    Color::from_hex(0xebbcba),
                    Color::from_hex(0x191724),
                ),
            );
        }

        self.load_from_file(file_path)?;
        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        draw_sphere(vec3(0., 0., self.base_height + 1.), 1., None, WHITE);
        draw_sphere(vec3(2., 0., self.base_height + 1.), 1., None, RED);
        draw_sphere(vec3(0., 2., self.base_height + 1.), 1., None, GREEN);
        draw_sphere(vec3(0., 0., self.base_height + 3.), 1., None, BLUE);

        self.draw_map()?;

        Ok(())
    }

    fn draw_map(&self) -> Result<()> {
        for ((x, y), &node) in self.map.iter() {
            let mesh = self
                .meshes
                .get(&MeshName::Hex { node })
                .ok_or_else(|| anyhow!("Node not found!"))?;

            let hex = Hex::new(*x, *y);
            let pos = {
                let pos = self.hex_layout.hex_to_world_pos(hex);
                vec2(pos.x, pos.y)
            };

            let mesh = modify_hexagonal_prism(mesh, pos);
            draw_mesh(&mesh);
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        for (i, line) in content.lines().enumerate() {
            let (x, y, node) = Node::from_str(line)
                .map_err(|err| anyhow!("Expected {} | line: @{}: `{}`", err, i + 1, line))?;
            self.map.insert((x, y), node);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    height: u8,
    node_type: NodeType,
}
impl Node {
    pub fn new(height: u8, node_type: NodeType) -> Self {
        Self { height, node_type }
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
        let node_type = NodeType::from_str(data.next().ok_or_else(|| anyhow!("'node_type'"))?)
            .map_err(|err| anyhow!("'node_type' as NodeType `{}`", err))?;
        Ok((x, y, Self::new(height, node_type)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MeshName {
    Hex { node: Node },
}

fn create_hexagonal_prism(radius: f32, height: f32, top_color: Color, bottom_color: Color) -> Mesh {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![
            // Top face triangles (first 6 vertices are bottom face)
            // 0, 2, 4, // First bottom triangle
            // 0, 4, 6, // Second bottom triangle
            // 0, 6, 8, // Third bottom triangle
            // 0, 8, 10, // Fourth bottom triangle
            // Top face triangles (last 6 vertices are top face)
            1, 3, 5, // First top triangle
            1, 5, 7, // Second top triangle
            1, 7, 9, // Third top triangle
            1, 9, 11, // Fourth top triangle
            // Side face triangles (connecting bottom and top vertices)
            0, 1, 2, // Side 1, triangle 1
            2, 1, 3, // Side 1, triangle 2
            2, 3, 4, // Side 2, triangle 1
            4, 3, 5, // Side 2, triangle 2
            4, 5, 6, // Side 3, triangle 1
            6, 5, 7, // Side 3, triangle 2
            6, 7, 8, // Side 4, triangle 1
            8, 7, 9, // Side 4, triangle 2
            8, 9, 10, // Side 5, triangle 1
            10, 9, 11, // Side 5, triangle 2
            10, 11, 0, // Side 6, triangle 1
            0, 11, 1, // Side 6, triangle 2
        ],
        texture: None,
    };

    for i in 0..6 {
        let angle = f32::consts::FRAC_PI_3 * i as f32 + f32::consts::FRAC_PI_6;
        let point = Vec2::from_angle(angle) * radius;
        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, 0.),
            color: bottom_color.into(),
            uv: Vec2::default(),
            normal: Vec4::default(),
        });

        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, height),
            color: top_color.into(),
            uv: Vec2::default(),
            normal: Vec4::default(),
        })
    }
    mesh
}

fn modify_hexagonal_prism(mesh: &Mesh, pos: Vec2) -> Mesh {
    let mut mesh = Mesh {
        vertices: mesh.vertices.clone(),
        indices: mesh.indices.clone(),
        texture: mesh.texture.clone(),
    };
    for v in mesh.vertices.iter_mut() {
        v.position.x += pos.x;
        v.position.y += pos.y;
    }
    mesh
}
