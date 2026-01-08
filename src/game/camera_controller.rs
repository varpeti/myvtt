#![allow(dead_code)]

mod default;

use std::f32;

use anyhow::Result;
use macroquad::prelude::*;

use crate::game::events::{Event, Events};

pub struct CameraController {
    target: Vec2,
    zoom: f32,
    rotation: f32,

    pub movement_speed: f32,
    pub rotation_delta: f32,
    pub zoom_gamma: f32,

    pub to_target: Vec2,
    pub to_zoom: f32,
    pub to_rotation: f32,

    pub smoothing_factor: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,

    events: Events<CameraEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CameraEvent {
    Up,
    Right,
    Down,
    Left,
    ZoomIn,
    ZoomOut,
    RotateClockwise,
    RotateAntiClockwise,
}

impl Event for CameraEvent {}

impl CameraController {
    pub fn handle_events(&mut self, dt: f32) -> Result<()> {
        self.events.update();
        if self.events.pop(&CameraEvent::Up) {
            self.move_to_direction(Direction::Up, self.movement_speed, dt);
        }
        if self.events.pop(&CameraEvent::Down) {
            self.move_to_direction(Direction::Down, self.movement_speed, dt);
        }
        if self.events.pop(&CameraEvent::Right) {
            self.move_to_direction(Direction::Right, self.movement_speed, dt);
        }
        if self.events.pop(&CameraEvent::Left) {
            self.move_to_direction(Direction::Left, self.movement_speed, dt);
        }

        if self.events.pop(&CameraEvent::ZoomIn) {
            self.zoom(self.zoom_gamma);
        }
        if self.events.pop(&CameraEvent::ZoomOut) {
            self.zoom(1. / self.zoom_gamma);
        }

        if self.events.pop(&CameraEvent::RotateClockwise) {
            self.rotate(self.rotation_delta);
        }
        if self.events.pop(&CameraEvent::RotateAntiClockwise) {
            self.rotate(-self.rotation_delta);
        }

        Ok(())
    }

    pub fn update(&mut self, camera: &mut Camera2D, dt: f32) -> Result<()> {
        let d = self.to_target - self.target;
        if d.length() > 0.01 {
            self.target += d * self.smoothing_factor * dt;
        }
        let d = self.to_zoom - self.zoom;
        if d.abs() > 0.01 {
            self.zoom += d * self.smoothing_factor * dt;
        }
        let d = self.to_rotation - self.rotation;
        if d.abs() > 0.01 {
            self.rotation += d * self.smoothing_factor * dt;
        }

        camera.target = self.target;
        camera.zoom = vec2(2. / screen_width(), 2. / screen_height()) * self.zoom;
        camera.rotation = self.rotation.to_degrees();
        Ok(())
    }

    pub fn move_to_target(&mut self, to_target: Vec2) {
        self.to_target = to_target;
    }

    pub fn move_to_direction(&mut self, direction: Direction, distance_per_second: f32, dt: f32) {
        self.to_target +=
            direction.get_movement(self.rotation) * distance_per_second * (1. / self.zoom) * dt;
    }

    pub fn rotate(&mut self, rotation_delta: f32) {
        self.to_rotation += rotation_delta;
    }

    pub fn zoom(&mut self, zoom_gamma: f32) {
        self.to_zoom *= zoom_gamma;
        if self.to_zoom < self.min_zoom {
            self.to_zoom = self.min_zoom;
        } else if self.to_zoom > self.max_zoom {
            self.to_zoom = self.max_zoom;
        }
    }

    pub fn get_target(&self) -> Vec2 {
        self.target
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
    pub fn get_movement(&self, rotation: f32) -> Vec2 {
        let angle = match self {
            Direction::Down => 0.,
            Direction::Right => 1.,
            Direction::Up => 2.,
            Direction::Left => 3.,
        } * f32::consts::FRAC_PI_2
            + rotation;

        vec2(angle.sin(), angle.cos())
    }
}
