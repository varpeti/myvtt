use std::{collections::HashMap, f32, fs};

use anyhow::{Result, anyhow};
use hexx::{HexLayout, HexOrientation};
use three_d::*;

pub struct Map {
    hex_layout: HexLayout,
    map: Vec<Vec<Node>>,
    geometries: HashMap<GeometryName, Mesh>,
    materials: HashMap<MaterialName, ColorMaterial>,
}

impl Map {
    pub fn load_models(&mut self, context: &Context) -> Result<()> {
        self.geometries.insert(
            GeometryName::Hex,
            create_hex_mesh(
                context,
                self.hex_layout.scale.x,
                self.hex_layout.scale.x,
                Srgba::new(196, 167, 231, 255),
                Srgba::new(25, 23, 36, 0),
            ),
        );

        // let hex_material = PhysicalMaterial::new_opaque(
        //     context,
        //     &CpuMaterial {
        //         emissive: Srgba::new(196, 167, 231, 255),
        //         ..Default::default()
        //     },
        // );
        self.materials
            .insert(MaterialName::NormalGround, ColorMaterial::default());
        Ok(())
    }
    pub fn handle_events(&mut self, frame_input: &mut FrameInput) -> Result<()> {
        Ok(())
    }

    pub fn draw(&mut self, frame_input: &mut FrameInput, camera: &Camera) -> Result<()> {
        let mut objects = Vec::new();

        let hex = self.get_gm(&GeometryName::Hex, &MaterialName::NormalGround)?;
        objects.push(hex);

        frame_input.screen().render(camera, objects, &[]);
        Ok(())
    }

    fn get_gm(
        &self,
        geometry: &GeometryName,
        material: &MaterialName,
    ) -> Result<Gm<&Mesh, &ColorMaterial>> {
        let geometry = self
            .geometries
            .get(geometry)
            .ok_or_else(|| anyhow!("Geometry `{:?}` not loaded!", geometry))?;
        let material = self
            .materials
            .get(material)
            .ok_or_else(|| anyhow!("Material `{:?}` not loaded!", material))?;
        Ok(Gm::new(geometry, material))
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        let content = fs::read_to_string(file_path)?;
        for (i, line) in content.lines().enumerate() {
            Node::from_str(line)
                .map_err(|err| anyhow!("Expected {} | line: @{}: `{}`", err, i + 1, line))?;
        }
        Ok(())
    }
}

impl Default for Map {
    fn default() -> Self {
        let size = 32.;
        Self {
            hex_layout: HexLayout {
                scale: hexx::Vec2::new(size, size),
                orientation: HexOrientation::Pointy,
                origin: hexx::Vec2::new(0., 0.),
            },
            map: Vec::new(),
            geometries: HashMap::new(),
            materials: HashMap::new(),
        }
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

fn create_hex_mesh(
    context: &Context,
    raduis: f32,
    height: f32,
    top_color: Srgba,
    bottom_color: Srgba,
) -> Mesh {
    let mut positions = Vec::with_capacity(12);
    for i in 0..6 {
        let angle = f32::consts::FRAC_PI_3 * i as f32;
        let (x, y) = (angle.cos() * raduis, angle.sin() * raduis);
        let l_point = vec3(x, y, 0.);
        positions.push(l_point);
        let h_point = vec3(x, y, height);
        positions.push(h_point);
    }
    let indices = vec![
        // Bottom face triangles (first 6 vertices are bottom face)
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
    ];

    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        indices: Indices::U8(indices),
        colors: Some(Vec::from([
            bottom_color,
            top_color,
            bottom_color,
            top_color,
            bottom_color,
            top_color,
            bottom_color,
            top_color,
            bottom_color,
            top_color,
            bottom_color,
            top_color,
        ])),
        ..Default::default()
    };
    Mesh::new(context, &cpu_mesh)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GeometryName {
    Hex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MaterialName {
    NormalGround,
}
