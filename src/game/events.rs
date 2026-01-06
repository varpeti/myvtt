#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    CameraUp,
    CameraRight,
    CameraDown,
    CameraLeft,
    CameraZoomIn,
    CameraZoomOut,
    CameraRotateClockwise,
    CameraRotateAntiClockwise,

    BrushPickEmpty,
    BrushPickSmall,
    BrushPickHalf,
    BrushPickFull,
    BrushRotateClockwise,
    BrushRotateAntiClockwise,
}

#[derive(Debug)]
pub struct Events {
    pub input_map: HashMap<Event, Vec<Vec<(EventS, EventT)>>>,
    events: HashSet<Event>,
}

impl Default for Events {
    fn default() -> Self {
        let input_map = HashMap::from([
            (
                Event::CameraUp,
                vec![
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::W))],
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::Up))],
                ],
            ),
            (
                Event::CameraRight,
                vec![
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::D))],
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::Right))],
                ],
            ),
            (
                Event::CameraDown,
                vec![
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::S))],
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::Down))],
                ],
            ),
            (
                Event::CameraLeft,
                vec![
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::A))],
                    vec![(EventS::IsDown, EventT::Keyboard(KeyCode::Left))],
                ],
            ),
            (
                Event::CameraZoomIn,
                vec![
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::LeftControl)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                    ],
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::RightControl)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                    ],
                ],
            ),
            (
                Event::CameraZoomOut,
                vec![
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::LeftControl)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                    ],
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::RightControl)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                    ],
                ],
            ),
            (
                Event::CameraRotateClockwise,
                vec![
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::LeftShift)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                    ],
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::RightShift)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp)),
                    ],
                ],
            ),
            (
                Event::CameraRotateAntiClockwise,
                vec![
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::LeftShift)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                    ],
                    vec![
                        (EventS::IsDown, EventT::Keyboard(KeyCode::RightShift)),
                        (EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown)),
                    ],
                ],
            ),
            (
                Event::BrushPickEmpty,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key1))]],
            ),
            (
                Event::BrushPickSmall,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key2))]],
            ),
            (
                Event::BrushPickHalf,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key3))]],
            ),
            (
                Event::BrushPickFull,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key4))]],
            ),
            (
                Event::BrushRotateClockwise,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))]],
            ),
            (
                Event::BrushRotateAntiClockwise,
                vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))]],
            ),
        ]);

        Self {
            input_map,
            events: HashSet::new(),
        }
    }
}

impl Events {
    pub fn update(&mut self) {
        self.events.clear();
        for (event, event_type_ors) in self.input_map.iter() {
            for event_type_ands in event_type_ors {
                let ok = event_type_ands.iter().all(|(es, et)| match et {
                    EventT::Keyboard(key_code) => match es {
                        EventS::JustPressed => is_key_pressed(*key_code),
                        EventS::IsDown => is_key_down(*key_code),
                        EventS::JustReleased => is_key_released(*key_code),
                    },
                    EventT::Mouse(mouse_button) => {
                        let button = match mouse_button {
                            MouseButton2::LeftClick => MouseButton::Left,
                            MouseButton2::RightClick => MouseButton::Right,
                            MouseButton2::MiddleClick => MouseButton::Middle,
                            MouseButton2::WheelUp => return mouse_wheel().1 > 0.,
                            MouseButton2::WheelDown => return mouse_wheel().1 < 0.,
                        };
                        match es {
                            EventS::JustPressed => is_mouse_button_pressed(button),
                            EventS::IsDown => is_mouse_button_down(button),
                            EventS::JustReleased => is_mouse_button_down(button),
                        }
                    }
                });
                if ok {
                    self.events.insert(*event);
                    break;
                }
            }
        }
    }

    pub fn contains(&self, event: &Event) -> bool {
        self.events.contains(event)
    }

    pub fn pop(&mut self, event: &Event) -> bool {
        self.events.remove(event)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventS {
    JustPressed,
    IsDown,
    JustReleased,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventT {
    Keyboard(KeyCode),
    Mouse(MouseButton2),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton2 {
    LeftClick,
    RightClick,
    MiddleClick,
    WheelUp,
    WheelDown,
}
