#![allow(dead_code)]

use std::f32;

use macroquad::prelude::*;

pub struct Camera {
    pub camera: Camera3D,
    rotation: f32,
    zoom: f32,
    to_target: Vec3,
    to_rotation: f32,
    to_zoom: f32,
}

impl Camera {
    pub fn new(target: Vec3, rotation: f32, zoom: f32) -> Self {
        // Forward -Y; Backward +Y; Left -X; Right +X; Up -Z; Down +Z;
        let mut position = target;
        position.z += zoom;
        position.y -= zoom;
        let camera = Camera3D {
            target,
            position,
            aspect: None,
            up: vec3(0., 0., -1.),
            fovy: 45.0_f32.to_radians(),
            projection: Projection::Perspective,
            ..Default::default()
        };
        Self {
            camera,
            rotation,
            zoom,
            to_target: target,
            to_rotation: rotation,
            to_zoom: zoom,
        }
    }

    pub fn activate(
        &mut self,
        smoothing_factor_pos: f32,
        smoothing_factor_rotation: f32,
        smoothing_factor_zoom: f32,
    ) {
        let dt = get_frame_time();
        let to_move = (self.to_target - self.camera.target) * smoothing_factor_pos * dt;
        if to_move.length() > 0.01 {
            self.camera.target += to_move;
            self.camera.position += to_move;
        }

        //
        // self.rotation +=
        //     (self.to_rotation - self.rotation) * smoothing_factor_rotation * get_frame_time();

        self.zoom += (self.to_zoom - self.zoom) * smoothing_factor_zoom * dt;

        self.camera.position = {
            let mut position = self.camera.target;
            position.z += self.zoom;
            position.y -= self.zoom;
            position
        };

        set_camera(&self.camera);
    }

    // pub fn smooth_move_to_direction(&mut self, direction: Direction, distance: f32) {
    //     let rotation = -self.camera.rotation.to_radians();
    //     let movement = match direction {
    //         Direction::Up => Vec2::from_angle(rotation - std::f32::consts::FRAC_PI_2),
    //         Direction::Right => Vec2::from_angle(rotation),
    //         Direction::Down => Vec2::from_angle(rotation + std::f32::consts::FRAC_PI_2),
    //         Direction::Left => -Vec2::from_angle(rotation),
    //     };
    //     self.to_target += movement * (distance / self.zoom) * get_frame_time();
    // }

    pub fn move_to_target(&mut self, to_target: Vec3) {
        self.to_target = to_target;
    }

    pub fn rotate(&mut self, to_rotation_delta: f32) {
        self.to_rotation += to_rotation_delta;
    }

    pub fn zoom(&mut self, to_zoom_delta: f32) {
        self.to_zoom += to_zoom_delta;
        if self.to_zoom > 0. {
            self.to_zoom = 0.
        }
    }

    // pub fn screen_to_world(&mut self, point: Vec2) -> Vec3 {
    //     todo!()
    //     //self.camera.screen_to_world(point)
    // }
    //
    // pub fn world_to_screen(&mut self, point: Vec3) -> Vec2 {
    //     todo!()
    //     //self.camera.world_to_screen(point)
    // }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(Vec3::new(0., 0., 0.), 0., -70.)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
