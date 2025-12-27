use std::f32;

use crate::game::Game;

use anyhow::Result;
use macroquad::prelude::*;

impl Game {
    pub(crate) fn c2(&mut self) -> Result<()> {
        clear_background(Color::from_hex(0x191724));
        self.camera.activate(5., 5., 5.);
        Ok(())
    }

    pub(crate) fn draw(&mut self) -> Result<()> {
        draw_sphere(vec3(0., 0., 0.), 1., None, BLUE);
        draw_sphere(vec3(2., 0., 0.), 1., None, GREEN);
        draw_sphere(vec3(0., 2., 0.), 1., None, RED);

        draw_hexagonal_prism(
            vec3(0., 0., 0.),
            64.,
            32.,
            Color::from_hex(0xebbcba),
            Color::from_hex(0x191724),
        );

        Ok(())
    }
}

fn draw_hexagonal_prism(pos: Vec3, size: f32, height: f32, top_color: Color, bottom_color: Color) {
    let mut mesh = Mesh {
        vertices: vec![],
        indices: vec![
            // // Top face triangles (first 6 vertices are bottom face)
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
        let angle = f32::consts::FRAC_PI_3 * i as f32;
        let point = Vec2::from_angle(angle) * size;
        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, pos.z),
            color: bottom_color.into(),
            uv: Vec2::default(),
            normal: Vec4::default(),
        });

        mesh.vertices.push(Vertex {
            position: vec3(point.x, point.y, pos.z - height),
            color: top_color.into(),
            uv: Vec2::default(),
            normal: Vec4::default(),
        })
    }

    draw_mesh(&mesh);
}
