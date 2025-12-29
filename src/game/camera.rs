#![allow(dead_code)]

use std::f32;

use anyhow::Result;
use macroquad::prelude::*;

pub struct RPGCamera {
    // Data
    camera: Camera3D,
    rotation: f32,
    zoom: f32,

    to_target: Vec3,
    to_rotation: f32,
    to_zoom: f32,

    // Config
    camera_speed: f32,
    rotation_speed: f32,
    zoom_speed: f32,

    min_zoom: f32,
    max_zoom: f32,

    smoothing_factor_target: f32,
    smoothing_factor_rotation: f32,
    smoothing_factor_zoom: f32,
}

impl Default for RPGCamera {
    fn default() -> Self {
        // Forward -Y; Backward +Y; Left -X; Right +X; Up +Z; Down -Z;
        let target = vec3(0., 0., 0.);
        let rotation = 0.;
        let zoom = 512.;
        let position = calculate_position(target, rotation, zoom);
        let camera = Camera3D {
            target,
            position,
            aspect: None,
            up: vec3(0., 0., 1.),
            fovy: 45.0_f32.to_radians(),
            projection: Projection::Perspective,
            render_target: None,
            viewport: None,
            z_near: 0.1,
            z_far: 4096.,
        };
        Self {
            camera,
            rotation,
            zoom,
            to_target: target,
            to_rotation: rotation,
            to_zoom: zoom,
            camera_speed: 2.,
            rotation_speed: f32::consts::FRAC_PI_6,
            zoom_speed: 32.,
            min_zoom: 8. * 16.,
            max_zoom: 2048.,
            smoothing_factor_target: 5.,
            smoothing_factor_rotation: 5.,
            smoothing_factor_zoom: 5.,
        }
    }
}

impl RPGCamera {
    pub fn handle_events(&mut self) -> Result<()> {
        if is_key_down(KeyCode::W) {
            self.move_to_direction(Direction::Up, self.camera_speed);
        }
        if is_key_down(KeyCode::A) {
            self.move_to_direction(Direction::Left, self.camera_speed);
        }
        if is_key_down(KeyCode::S) {
            self.move_to_direction(Direction::Down, self.camera_speed);
        }
        if is_key_down(KeyCode::D) {
            self.move_to_direction(Direction::Right, self.camera_speed);
        }

        let mouse_wheel_y = mouse_wheel().1;
        if mouse_wheel_y > 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.rotate(-self.rotation_speed);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.zoom(-self.zoom_speed);
            }
        } else if mouse_wheel_y < 0.0 {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.rotate(self.rotation_speed);
            }
            if is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl) {
                self.zoom(self.zoom_speed);
            }
        }

        self.update();
        Ok(())
    }

    fn update(&mut self) {
        let dt = get_frame_time();

        let d = self.to_target - self.camera.target;
        if d.length() > 0.01 {
            self.camera.target += d * self.smoothing_factor_target * dt;
        }
        let d = self.to_zoom - self.zoom;
        if d.abs() > 0.01 {
            self.zoom += d * self.smoothing_factor_zoom * dt;
        }
        let d = self.to_rotation - self.rotation;
        if d.abs() > 0.01 {
            self.rotation += d * self.smoothing_factor_rotation * dt;
        }

        self.camera.position = calculate_position(self.camera.target, self.rotation, self.zoom);
    }

    pub fn activate(&mut self) -> Result<()> {
        set_camera(&self.camera);
        Ok(())
    }

    pub fn move_to_target(&mut self, to_target: Vec3) {
        self.to_target = to_target;
    }

    pub fn move_to_direction(&mut self, direction: Direction, distance_per_second: f32) {
        self.to_target += direction.get_movement(self.rotation)
            * distance_per_second
            * self.zoom
            * get_frame_time();
    }

    pub fn rotate(&mut self, to_rotation_delta: f32) {
        self.to_rotation += to_rotation_delta;
    }

    pub fn zoom(&mut self, to_zoom_delta: f32) {
        self.to_zoom += to_zoom_delta;
        if self.to_zoom < self.min_zoom {
            self.to_zoom = self.min_zoom
        } else if self.to_zoom > self.max_zoom {
            self.to_zoom = self.max_zoom
        }
    }

    // pub fn screen_to_world(&mut self, point: Vec2) -> Vec3 {
    //     self.camera.screen_to_world(point)
    // }

    //
    // pub fn world_to_screen(&mut self, point: Vec3) -> Vec2 {
    //     todo!()
    //     //self.camera.world_to_screen(point)
    // }

    pub fn get_target(&self) -> Vec3 {
        self.camera.target
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn get_movement(&self, rotation: f32) -> Vec3 {
        let angle = match self {
            Direction::Up => 0.,
            Direction::Right => 1.,
            Direction::Down => 2.,
            Direction::Left => 3.,
        } * f32::consts::FRAC_PI_2
            + rotation;

        -vec3(angle.sin(), angle.cos(), 0.)
    }
}

fn calculate_position(target: Vec3, rotation: f32, zoom: f32) -> Vec3 {
    vec3(
        target.x + rotation.sin() * zoom,
        target.y + rotation.cos() * zoom,
        zoom,
    )
}
