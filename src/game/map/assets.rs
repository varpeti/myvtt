use std::{collections::HashMap, f32};

use macroquad::prelude::*;

use crate::game::map::node::Node;

pub struct Assets {
    pub meshes: HashMap<MeshName, Mesh>,
    pub materials: HashMap<MaterialName, Material>,
    pub textures: HashMap<TextureName, Texture2D>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            meshes: HashMap::new(),
            materials: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeshName {
    Hex { node: Node },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MaterialName {
    Edge,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextureName {
    HexagonalPrims,
}

pub fn create_hexagonal_prism(
    pos: Vec2,
    radius: f32,
    height: f32,
    top_color: Color,
    bottom_color: Color,
    texture: Option<Texture2D>,
) -> Mesh {
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
        texture,
    };

    for i in 0..6 {
        let angle = std::f32::consts::FRAC_PI_3 * i as f32 + f32::consts::FRAC_PI_6;
        let direction = Vec2::from_angle(angle);
        let point = direction * radius + pos;
        let uv_bottom = (direction + vec2(1., 1.)) * 0.5;
        let uv_top = uv_bottom;
        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, 0.),
            color: bottom_color.into(),
            uv: uv_bottom,
            normal: Vec4::default(),
        });
        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, height),
            color: top_color.into(),
            uv: uv_top,
            normal: Vec4::default(),
        })
    }
    mesh
}
