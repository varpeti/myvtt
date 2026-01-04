#![allow(dead_code)]

mod assets;
mod node;

use std::io::Write;

use anyhow::{Result, anyhow};
use hexx::{Hex, HexLayout, HexOrientation};
use indexmap::IndexMap;
use macroquad::prelude::*;

use crate::game::{
    camera::RPGCamera,
    map::{
        assets::{Assets, TextureName},
        node::Node,
    },
    theme::Theme,
};

pub struct Map {
    radius: f32,
    base_height: f32,
    hex_layout: HexLayout,
    map: IndexMap<Hex, Node>,
    assets: Assets,

    current_map: String,
    hoovered_hex: Hex,
    brush_node: Node,
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
            map: IndexMap::new(),
            assets: Assets::new(),
            current_map: String::new(),
            hoovered_hex: Hex::default(),
            brush_node: Node::new(1, Theme::SpringGreen),
        }
    }
}
impl Map {
    pub async fn load(&mut self, file_path: &str) -> Result<()> {
        let hex_prism_texture = load_texture("assets/img/hex_prism.png").await?;
        self.assets
            .textures
            .insert(TextureName::HexagonalPrims, hex_prism_texture.clone());

        self.load_from_file(file_path).await?;
        self.current_map = file_path.to_string();

        Ok(())
    }

    async fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        let content = load_string(file_path).await?;
        for (i, line) in content.lines().enumerate() {
            let (x, y, node) = Node::from_str(line)
                .map_err(|err| anyhow!("Expected {} | line: @{}: `{}`", err, i + 1, line))?;
            self.map.insert(Hex::new(x, y), node);
        }
        Ok(())
    }

    async fn save(&self, file_path: &str) -> Result<()> {
        #[cfg(target_arch = "wasm32")]
        {
            // File saving not implemented yet for wasm32
            return Ok(());
        }

        let mut file = std::fs::File::create(file_path)?;
        for (hex, node) in self.map.iter() {
            let color: &'static str = node.color.into();
            let line = format!("{:+2} {:+2} {:2} {}\n", hex.x, hex.y, node.height, color);
            file.write_all(line.as_bytes())?;
        }
        Ok(())
    }

    pub async fn handle_events(&mut self, camera: &RPGCamera) -> Result<()> {
        let (hex, _node) = self.screen_to_node(mouse_position().into(), camera);
        self.hoovered_hex = hex;

        if let Some(hoovered_node) = self.map.get_mut(&self.hoovered_hex) {
            if is_mouse_button_down(MouseButton::Left) {
                // Replace
                self.map.insert(self.hoovered_hex, self.brush_node);
                self.save(&self.current_map).await?;
            } else if is_mouse_button_down(MouseButton::Right) {
                // Remove
                self.map.swap_remove(&self.hoovered_hex);
                self.save(&self.current_map).await?;
            } else if is_mouse_button_pressed(MouseButton::Middle) {
                // Copy
                self.brush_node.height = hoovered_node.height;
                self.brush_node.color = hoovered_node.color;
            }
        } else {
            // Add
            if is_mouse_button_down(MouseButton::Left) {
                self.map.insert(self.hoovered_hex, self.brush_node);
                self.save(&self.current_map).await?;
            }
        }

        if is_key_pressed(KeyCode::Q) {
            self.brush_node.height = self.brush_node.height.saturating_add(1);
        }
        if is_key_pressed(KeyCode::E) {
            self.brush_node.height = self.brush_node.height.saturating_sub(1);
        }

        if !is_key_down(KeyCode::LeftShift)
            && !is_key_down(KeyCode::RightShift)
            && !is_key_down(KeyCode::LeftControl)
            && !is_key_down(KeyCode::RightControl)
        {
            let my = mouse_wheel().1;
            if my > 0. {
                self.brush_node.color = self.brush_node.color.next();
            } else if my < 0. {
                self.brush_node.color = self.brush_node.color.previous();
            }
        }

        Ok(())
    }

    pub fn draw(&self, camera: &RPGCamera) -> Result<()> {
        camera.activate()?;

        let hexagonal_prims_texture = self
            .assets
            .textures
            .get(&TextureName::HexagonalPrims)
            .cloned();

        self.draw_map(&hexagonal_prims_texture)?;

        let pos = self.hex_layout.hex_to_world_pos(self.hoovered_hex);
        Node::draw_node(
            vec2(pos.x, pos.y),
            self.radius / 1.2,
            self.brush_node.height as f32 * self.base_height + 1.,
            hexagonal_prims_texture.clone(),
            self.brush_node.color.color(),
        )?;

        //TODO: Remove
        let pos = self.hex_layout.hex_to_world_pos(Hex::new(-2, 0));
        let pos = vec3(pos.x, pos.y, self.base_height * 5.);
        let rotation = camera.get_rotation();
        let half_width = self.radius / 3.;
        let height = self.radius;
        let size = vec2(rotation.cos(), -rotation.sin()) * half_width;
        let color = Theme::LotusYellow4.color();

        let vertices = vec![
            Vertex::new2(pos + vec3(-size.x, -size.y, 0.), vec2(0., 0.), color),
            Vertex::new2(pos + vec3(-size.x, -size.y, height), vec2(0., 1.), color),
            Vertex::new2(pos + vec3(size.x, size.y, height), vec2(1., 1.), color),
            Vertex::new2(pos + vec3(size.x, size.y, 0.), vec2(1., 0.), color),
        ];
        let mesh = Mesh {
            vertices,
            indices: vec![0, 1, 2, 0, 3, 2],
            texture: None,
        };
        draw_mesh(&mesh);

        Ok(())
    }

    fn draw_map(&self, hexagonal_prims_texture: &Option<Texture2D>) -> Result<()> {
        for (&hex, &node) in self.map.iter() {
            let pos = {
                let pos = self.hex_layout.hex_to_world_pos(hex);
                vec2(pos.x, pos.y)
            };
            let radius = if hex == self.hoovered_hex {
                if self.brush_node.height < node.height {
                    self.radius / 1.3
                } else {
                    self.radius / 1.1
                }
            } else {
                self.radius
            };
            Node::draw_node(
                pos,
                radius,
                node.height as f32 * self.base_height,
                hexagonal_prims_texture.clone(),
                node.color.color(),
            )?;
        }
        Ok(())
    }

    pub fn screen_to_hex(&self, point: Vec2, camera: &RPGCamera) -> Hex {
        let (ray_origin, ray_direction) = camera.screen_to_world_ray(point);
        self.ray_to_hex(ray_origin, ray_direction)
    }

    pub fn ray_to_hex(&self, ray_origin: Vec3, ray_direction: Vec3) -> Hex {
        // TODO: check if ray ever hit z=0. It fails if not.
        let t = -ray_origin.z / ray_direction.z;
        let intersection = ray_origin + ray_direction * t;
        self.hex_layout
            .world_pos_to_hex(hexx::Vec2::new(intersection.x, intersection.y))
    }

    pub fn screen_to_node(&self, point: Vec2, camera: &RPGCamera) -> (Hex, Option<&Node>) {
        let (ray_origin, ray_direction) = camera.screen_to_world_ray(point);
        self.ray_to_node(ray_origin, ray_direction)
    }

    pub fn ray_to_node(&self, ray_origin: Vec3, ray_direction: Vec3) -> (Hex, Option<&Node>) {
        // TODO: check if ray ever hit z=0. It will stuck if not
        let mut t = 0.;
        let step = f32::min(self.base_height / 2., self.radius);
        loop {
            let point = ray_origin + ray_direction * t;
            let hex = self
                .hex_layout
                .world_pos_to_hex(hexx::Vec2::new(point.x, point.y));
            if point.z <= 0. {
                // Map does not have anything below 0.
                return (hex, None);
            }
            if let Some(node) = self.map.get(&hex)
                && point.z <= (node.height as f32 * self.base_height)
            {
                return (hex, Some(node));
            }
            t += step;
        }
    }

    pub fn hoovered_hex(&self) -> Hex {
        self.hoovered_hex
    }

    pub fn hoovered_node(&self) -> (Hex, Option<&Node>) {
        (self.hoovered_hex, self.map.get(&self.hoovered_hex))
    }

    pub fn brush_node(&self) -> Node {
        self.brush_node
    }
}
