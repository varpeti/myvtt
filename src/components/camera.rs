#![allow(dead_code)]

use std::f32;

use anyhow::Result;
use three_d::*;

pub struct RPGCamera {
    camera: Camera,
    rotation: f32,
    zoom: f32,
    to_target: Vec3,
    to_target_gamma: f32,
    to_move_to_direction: [bool; 4],
    to_rotation: f32,
    to_rotation_delta: f32,
    to_zoom: f32,
    to_zoom_delta: f32,
    smoothing_factor_target: f32,
    smoothing_factor_rotation: f32,
    smoothing_factor_zoom: f32,
}

impl Default for RPGCamera {
    fn default() -> Self {
        // Forward -Y; Backward +Y; Left -X; Right +X; Up -Z; Down +Z;
        let target = vec3(0., 0., 0.);
        let rotation = 0.0f32;
        let zoom = 512.;
        Self {
            camera: Camera::new_perspective(
                Viewport::new_at_origo(0, 0),
                vec3(
                    rotation.cos() * zoom,
                    rotation.sin() * zoom,
                    target.z + zoom,
                ),
                target,
                vec3(0., 0., 1.),
                degrees(45.),
                0.1,
                2048.,
            ),
            rotation,
            zoom,
            to_target: target,
            to_target_gamma: 1.,
            to_move_to_direction: [false; 4],
            to_rotation: rotation,
            to_rotation_delta: f32::consts::FRAC_PI_3,
            to_zoom: zoom,
            to_zoom_delta: 16.,
            smoothing_factor_target: 5.,
            smoothing_factor_rotation: 5.,
            smoothing_factor_zoom: 5.,
        }
    }
}

impl RPGCamera {
    pub fn handle_events(&mut self, frame_input: &mut FrameInput) -> Result<()> {
        for event in frame_input.events.iter() {
            match event {
                Event::MouseWheel {
                    delta,
                    position: _,
                    modifiers,
                    handled: _,
                } => {
                    if delta.1 > 0. {
                        if modifiers.ctrl {
                            self.zoom(-self.to_zoom_delta);
                        }
                        if modifiers.shift {
                            self.rotate(self.to_rotation_delta)
                        }
                    } else if delta.1 < 0. {
                        if modifiers.ctrl {
                            self.zoom(self.to_zoom_delta);
                        }
                        if modifiers.shift {
                            self.rotate(-self.to_rotation_delta)
                        }
                    }
                }
                Event::KeyPress {
                    kind,
                    modifiers: _,
                    handled: _,
                } => match kind {
                    Key::W | Key::K | Key::ArrowUp => self.to_move_to_direction[0] = true,
                    Key::A | Key::H | Key::ArrowLeft => self.to_move_to_direction[1] = true,
                    Key::S | Key::J | Key::ArrowDown => self.to_move_to_direction[2] = true,
                    Key::D | Key::L | Key::ArrowRight => self.to_move_to_direction[3] = true,
                    _ => (),
                },
                Event::KeyRelease {
                    kind,
                    modifiers: _,
                    handled: _,
                } => match kind {
                    Key::W | Key::K | Key::ArrowUp => self.to_move_to_direction[0] = false,
                    Key::A | Key::H | Key::ArrowLeft => self.to_move_to_direction[1] = false,
                    Key::S | Key::J | Key::ArrowDown => self.to_move_to_direction[2] = false,
                    Key::D | Key::L | Key::ArrowRight => self.to_move_to_direction[3] = false,
                    _ => (),
                },
                _ => (),
            }
        }

        self.update(frame_input)?;
        Ok(())
    }

    fn update(&mut self, frame_input: &mut FrameInput) -> Result<()> {
        self.camera.set_viewport(frame_input.viewport);
        let dt = (frame_input.elapsed_time / 1000.) as f32;
        let mut target = self.camera.target();

        if self.to_move_to_direction[0] {
            self.move_to_direction(Direction::Up, self.to_target_gamma, dt);
        }
        if self.to_move_to_direction[1] {
            self.move_to_direction(Direction::Left, self.to_target_gamma, dt);
        }
        if self.to_move_to_direction[2] {
            self.move_to_direction(Direction::Down, self.to_target_gamma, dt);
        }
        if self.to_move_to_direction[3] {
            self.move_to_direction(Direction::Right, self.to_target_gamma, dt);
        }

        target += (self.to_target - target) * self.smoothing_factor_target * dt;
        self.zoom += (self.to_zoom - self.zoom) * self.smoothing_factor_zoom * dt;
        self.rotation += (self.to_rotation - self.rotation) * self.smoothing_factor_rotation * dt;

        let position = {
            let mut position = target;
            position.z += self.zoom;
            position.x += self.rotation.cos() * self.zoom;
            position.y -= self.rotation.sin() * self.zoom;
            position
        };

        let up = self.camera.up();
        self.camera.set_view(position, target, up);
        Ok(())
    }

    pub fn move_to_target(&mut self, to_target: Vec3) {
        self.to_target = to_target;
    }

    pub fn move_to_direction(&mut self, direction: Direction, distance_per_second: f32, dt: f32) {
        self.to_target +=
            direction.get_vec3_rotation(self.rotation) * distance_per_second * -self.zoom * dt;
    }

    pub fn rotate(&mut self, to_rotation_delta: f32) {
        self.to_rotation += to_rotation_delta;
    }

    pub fn zoom(&mut self, to_zoom_delta: f32) {
        self.to_zoom += to_zoom_delta;
    }

    // pub fn screen_to_world(&mut self, point: Vec2) -> Vec3 {
    //     todo!()
    //     self.camera.screen_to_world(point)
    // }
    //
    // pub fn world_to_screen(&mut self, point: Vec3) -> Vec2 {
    //     todo!()
    //     //self.camera.world_to_screen(point)
    // }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_target(&self) -> Vec3 {
        self.camera.target()
    }

    pub fn get_position(&self) -> Vec3 {
        self.camera.position()
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
    pub fn get_vec2_rotation(&self, rotation: f32) -> Vec2 {
        let angle = match self {
            Direction::Up => 0.,
            Direction::Right => 1.,
            Direction::Down => 2.,
            Direction::Left => 3.,
        } * f32::consts::FRAC_PI_2
            + rotation;
        vec2(angle.cos(), angle.sin())
    }
    pub fn get_vec3_rotation(&self, rotation: f32) -> Vec3 {
        let xy = self.get_vec2_rotation(rotation);
        vec3(xy.x, -xy.y, 0.)
    }
}
