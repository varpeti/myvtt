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
            min_zoom: 16. * 16.,
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

    pub fn activate(&self) -> Result<()> {
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

    pub fn world_to_screen(&self, point: Vec3) -> Option<Vec2> {
        let view_projection = self.camera.matrix();
        let clip_space = view_projection * point.extend(1.);

        if clip_space.w <= 0. {
            return None; // Behind camera
        }

        let ndc = clip_space.truncate() / clip_space.w;

        if ndc.x < -1. || ndc.x > 1. || ndc.y < -1. || ndc.y > 1. {
            return None; // Outside of camera
        }

        Some(vec2(
            (ndc.x + 1.) * 0.5 * screen_width(),
            (1. - ndc.y) * 0.5 * screen_height(),
        ))
    }

    pub fn screen_to_world_ray(&self, point: Vec2) -> (Vec3, Vec3) {
        let inverse_view_porjection = self.camera.matrix().inverse();
        let ndc = vec2(
            (point.x / screen_width()) * 2. - 1.,
            1. - (point.y / screen_height()) * 2.,
        );

        let near_point = inverse_view_porjection * vec4(ndc.x, ndc.y, -1., 1.);
        let near_point = near_point.truncate() / near_point.w;
        let far_point = inverse_view_porjection * vec4(ndc.x, ndc.y, 1., 1.);
        let far_point = far_point.truncate() / far_point.w;

        let ray_origin = near_point;
        let ray_direction = (far_point - near_point).normalize();

        (ray_origin, ray_direction)
    }

    pub fn get_target(&self) -> Vec3 {
        self.camera.target
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_z_far(&self) -> f32 {
        self.camera.z_far
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
