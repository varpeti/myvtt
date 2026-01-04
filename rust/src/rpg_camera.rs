use core::f32;

use godot::{
    classes::{Camera3D, ICamera3D, Input},
    prelude::*,
};

#[derive(GodotClass, Debug)]
#[class(base=Camera3D)]
struct RPGCamera {
    base: Base<Camera3D>,

    #[export]
    /// Test descrition
    to_target: Vector2,
    // Test descrition2
    #[export]
    to_zoom: f32,
    #[export]
    to_rotation: f32,

    #[export]
    move_speed: f32,
    #[export]
    zoom_speed: f32,
    #[export]
    min_zoom: f32,
    #[export]
    max_zoom: f32,
    #[export]
    rotation_speed: f32,
    #[export]
    smoothness: f32,

    s_to_target: Vector2,
    s_to_zoom: f32,
    s_to_rotation: f32,
}

#[godot_api]
impl ICamera3D for RPGCamera {
    fn init(base: Base<Camera3D>) -> Self {
        let to_target = Vector2::new(0., 0.);
        let to_zoom = 15.;
        let to_rotation = 0.;
        Self {
            base,
            to_target,
            to_zoom,
            to_rotation,
            move_speed: 3.,
            zoom_speed: 2.,
            min_zoom: 7.5,
            max_zoom: 150.,
            rotation_speed: f32::consts::FRAC_PI_6,
            smoothness: 5.,
            s_to_target: to_target,
            s_to_zoom: to_zoom,
            s_to_rotation: to_rotation,
        }
    }

    fn ready(&mut self) {
        self.s_to_target = self.to_target;
        self.s_to_zoom = self.to_zoom;
        self.s_to_rotation = self.to_rotation;
        self.calculate_position_and_rotation(1.0);
        self.base_mut()
            .set_rotation(Vector3::new(f32::to_radians(-45.), 0., 0.));
    }

    fn process(&mut self, dt: f32) {
        self.handle_inputs(dt);
        self.calculate_position_and_rotation(self.smoothness * dt);
    }
}

#[godot_api]
impl RPGCamera {
    #[func]
    fn handle_inputs(&mut self, dt: f32) {
        let input = Input::singleton();

        if input.is_action_pressed("move_forward") {
            self.to_target += get_movement(Direction::Forward, self.to_rotation)
                * self.move_speed
                * self.to_zoom
                * dt;
        }
        if input.is_action_pressed("move_left") {
            self.to_target += get_movement(Direction::Left, self.to_rotation)
                * self.move_speed
                * self.to_zoom
                * dt;
        }
        if input.is_action_pressed("move_backward") {
            self.to_target += get_movement(Direction::Backward, self.to_rotation)
                * self.move_speed
                * self.to_zoom
                * dt;
        }
        if input.is_action_pressed("move_right") {
            self.to_target += get_movement(Direction::Right, self.to_rotation)
                * self.move_speed
                * self.to_zoom
                * dt;
        }

        if input.is_action_just_pressed("zoom_in") {
            self.to_zoom = (self.to_zoom - self.zoom_speed).max(self.min_zoom);
        }

        if input.is_action_just_pressed("zoom_out") {
            self.to_zoom = (self.to_zoom + self.zoom_speed).min(self.max_zoom);
        }

        if input.is_action_just_pressed("rotation_clockwise") {
            self.to_rotation += self.rotation_speed;
        }
        if input.is_action_just_pressed("rotation_anticlockwise") {
            self.to_rotation -= self.rotation_speed;
        }
    }

    #[func]
    fn calculate_position_and_rotation(&mut self, smoothness_dt: f32) {
        self.s_to_target.x += lerp(self.s_to_target.x, self.to_target.x, smoothness_dt);
        self.s_to_target.y += lerp(self.s_to_target.y, self.to_target.y, smoothness_dt);
        self.s_to_zoom += lerp(self.s_to_zoom, self.to_zoom, smoothness_dt);
        self.s_to_rotation += lerp(self.s_to_rotation, self.to_rotation, smoothness_dt);

        let position = Vector3::new(
            self.s_to_target.x + self.s_to_rotation.sin() * self.s_to_zoom,
            self.s_to_zoom,
            self.s_to_target.y + self.s_to_rotation.cos() * self.s_to_zoom,
        );
        self.base_mut().set_position(position);
        let mut rotation = self.base().get_rotation();
        rotation.y = self.s_to_rotation;
        self.base_mut().set_rotation(rotation);
    }
}

fn lerp(current: f32, target: f32, smoothness: f32) -> f32 {
    (target - current) * smoothness
}

fn get_movement(direction: Direction, rotation: f32) -> Vector2 {
    let angle = match direction {
        Direction::Backward => 0.,
        Direction::Right => 1.,
        Direction::Forward => 2.,
        Direction::Left => 3.,
    } * f32::consts::FRAC_PI_2
        + rotation;
    Vector2::new(angle.sin(), angle.cos())
}

enum Direction {
    Backward,
    Right,
    Forward,
    Left,
}
