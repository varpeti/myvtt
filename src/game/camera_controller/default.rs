use crate::game::camera_controller::{CameraController, CameraEvent};

use std::f32;

use macroquad::prelude::*;

use crate::game::events::{EventS, EventT, Events, MouseButton2};

impl Default for CameraController {
    fn default() -> Self {
        let target = Vec2::ZERO;
        let zoom = 1.;
        let rotation = 0.;

        Self {
            target,
            zoom,
            rotation,

            to_target: target,
            to_zoom: zoom,
            to_rotation: rotation,

            movement_speed: 512.,
            rotation_delta: f32::consts::FRAC_PI_6,
            zoom_gamma: 1.1,

            smoothing_factor: 5.,
            min_zoom: 0.125,
            max_zoom: 8.,

            events: Events::from([
                (
                    CameraEvent::Up,
                    vec![
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::W))],
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::Up))],
                    ],
                ),
                (
                    CameraEvent::Right,
                    vec![
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::D))],
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::Right))],
                    ],
                ),
                (
                    CameraEvent::Down,
                    vec![
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::S))],
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::Down))],
                    ],
                ),
                (
                    CameraEvent::Left,
                    vec![
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::A))],
                        vec![(EventS::IsPressed, EventT::Keyboard(KeyCode::Left))],
                    ],
                ),
                (
                    CameraEvent::ZoomIn,
                    vec![
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::LeftControl)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                        ],
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::RightControl)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                        ],
                    ],
                ),
                (
                    CameraEvent::ZoomOut,
                    vec![
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::LeftControl)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                        ],
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::RightControl)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                        ],
                    ],
                ),
                (
                    CameraEvent::RotateClockwise,
                    vec![
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::LeftShift)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                        ],
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::RightShift)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                        ],
                    ],
                ),
                (
                    CameraEvent::RotateAntiClockwise,
                    vec![
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::LeftShift)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                        ],
                        vec![
                            (EventS::IsPressed, EventT::Keyboard(KeyCode::RightShift)),
                            (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                        ],
                    ],
                ),
            ]),
        }
    }
}
