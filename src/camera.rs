#![allow(dead_code)]
use std::f32;

use macroquad::prelude::*;

pub struct Camera {
    pub camera: Camera2D,
    zoom: f32,
    pub target_pos: Vec2,
    target_rotation: f32,
    target_zoom: f32,
}

impl Camera {
    pub fn new(pos: Vec2, zoom: f32, rotation: f32) -> Self {
        Self {
            camera: Camera2D {
                rotation,
                zoom: Vec2::new(zoom / screen_width(), zoom / screen_height()),
                target: pos,
                offset: Vec2::new(0., 0.),
                render_target: None,
                viewport: None,
            },
            zoom,
            ..Default::default()
        }
    }

    pub fn activate(&mut self) {
        self.camera.zoom = Vec2::new(self.zoom / screen_width(), self.zoom / screen_height());
        set_camera(&self.camera);
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.camera.target = pos;
        self.target_pos = pos;
    }

    pub fn move_pos(&mut self, pos: Vec2) {
        self.camera.target += pos;
        self.target_pos += pos;
    }

    pub fn move_to_direction(&mut self, direction: Direction, distance: f32) {
        let rotation = -self.camera.rotation.to_radians();
        let movement = match direction {
            Direction::Up => Vec2::from_angle(rotation - std::f32::consts::FRAC_PI_2),
            Direction::Right => Vec2::from_angle(rotation),
            Direction::Down => Vec2::from_angle(rotation + std::f32::consts::FRAC_PI_2),
            Direction::Left => -Vec2::from_angle(rotation),
        };
        self.camera.target += movement * (distance / self.zoom) * get_frame_time();
    }

    pub fn smooth_move_to_direction(&mut self, direction: Direction, distance: f32) {
        let rotation = -self.camera.rotation.to_radians();
        let movement = match direction {
            Direction::Up => Vec2::from_angle(rotation - std::f32::consts::FRAC_PI_2),
            Direction::Right => Vec2::from_angle(rotation),
            Direction::Down => Vec2::from_angle(rotation + std::f32::consts::FRAC_PI_2),
            Direction::Left => -Vec2::from_angle(rotation),
        };
        self.target_pos += movement * (distance / self.zoom) * get_frame_time();
    }

    pub fn smooth_move_to_position(&mut self, target_position: Vec2) {
        self.target_pos = target_position;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.camera.rotation = rotation;
        self.target_rotation = rotation;
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.camera.rotation += rotation;
        self.target_rotation += rotation;
    }

    pub fn smooth_rotate(&mut self, target_rotation_delta: f32) {
        self.target_rotation += target_rotation_delta;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.target_zoom = zoom;
        self.camera.zoom.x = zoom / screen_width();
        self.camera.zoom.y = zoom / screen_height();
    }

    pub fn zoom(&mut self, speed: f32) {
        self.zoom *= speed;
        self.target_zoom *= speed;
        self.camera.zoom.x = self.zoom / screen_width();
        self.camera.zoom.y = self.zoom / screen_height();
    }

    pub fn smooth_zoom(&mut self, speed: f32) {
        self.target_zoom *= speed;
    }

    pub fn smooth_update(
        &mut self,
        smoothing_factor_pos: f32,
        smoothing_factor_rotation: f32,
        smoothing_factor_zoom: f32,
    ) {
        self.camera.target +=
            (self.target_pos - self.camera.target) * smoothing_factor_pos * get_frame_time();

        self.camera.rotation += (self.target_rotation - self.camera.rotation)
            * smoothing_factor_rotation
            * get_frame_time();

        self.zoom += (self.target_zoom - self.zoom) * smoothing_factor_zoom * get_frame_time();
        self.camera.zoom.x = self.zoom / screen_width();
        self.camera.zoom.y = self.zoom / screen_height();
    }

    pub fn screen_to_world(&mut self, point: Vec2) -> Vec2 {
        self.camera.screen_to_world(point)
    }

    pub fn world_to_screen(&mut self, point: Vec2) -> Vec2 {
        self.camera.world_to_screen(point)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let zoom = 2.;
        Self {
            camera: Camera2D {
                target: Vec2::default(),
                zoom: Vec2::new(zoom / screen_width(), zoom / screen_height()),
                ..Default::default()
            },
            zoom,
            target_pos: Vec2::default(),
            target_rotation: 0.,
            target_zoom: zoom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
