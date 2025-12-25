#![allow(dead_code)]
use macroquad::prelude::*;

pub struct Camera {
    pub camera: Camera2D,
    zoom: f32,
    zoom_speed: f32,
    rotation_speed: f32,
    pos_speed: Vec2,
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
            zoom_speed: 1.0,
            rotation_speed: 0.0,
            pos_speed: Vec2::default(),
        }
    }

    pub fn activate(&mut self) {
        self.camera.zoom = Vec2::new(self.zoom / screen_width(), self.zoom / screen_height());
        set_camera(&self.camera);
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.camera.target = pos;
    }

    pub fn move_pos(&mut self, pos: Vec2) {
        self.camera.target += pos;
    }

    pub fn move_to_direction(&mut self, direction: Direction, speed: f32) {
        let rotation = -self.camera.rotation.to_radians();
        let movement = match direction {
            Direction::Up => Vec2::from_angle(rotation - std::f32::consts::FRAC_PI_2),
            Direction::Right => Vec2::from_angle(rotation),
            Direction::Down => Vec2::from_angle(rotation + std::f32::consts::FRAC_PI_2),
            Direction::Left => -Vec2::from_angle(rotation),
        };
        self.camera.target += movement * (speed / self.zoom);
    }

    pub fn smooth_move_to_direction(&mut self, direction: Direction, force: f32) {
        let rotation = -self.camera.rotation.to_radians();
        let movement = match direction {
            Direction::Up => Vec2::from_angle(rotation - std::f32::consts::FRAC_PI_2),
            Direction::Right => Vec2::from_angle(rotation),
            Direction::Down => Vec2::from_angle(rotation + std::f32::consts::FRAC_PI_2),
            Direction::Left => -Vec2::from_angle(rotation),
        };
        self.pos_speed += movement * force;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.camera.rotation = rotation;
    }

    pub fn rotate(&mut self, rotation: f32) {
        self.camera.rotation += rotation;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
        self.camera.zoom.x = zoom / screen_width();
        self.camera.zoom.y = zoom / screen_height();
    }

    pub fn zoom(&mut self, zoom: f32, speed: f32) {
        self.zoom *= speed.powf(zoom);
        self.camera.zoom.x = self.zoom / screen_width();
        self.camera.zoom.y = self.zoom / screen_height();
    }

    pub fn smooth_update(&mut self) {
        self.pos_speed *= 0.9;
        if self.pos_speed.length() < 3. {
            self.pos_speed.x = 0.0;
            self.pos_speed.y = 0.0;
        }
        self.camera.target += self.pos_speed;
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
        Self {
            camera: Camera2D {
                target: Vec2::new(screen_width() / 2., screen_height() / 2.),
                zoom: Vec2::new(2. / screen_width(), 2. / screen_height()),
                ..Default::default()
            },
            zoom: 2.,
            zoom_speed: 1.0,
            rotation_speed: 0.0,
            pos_speed: Vec2::default(),
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
