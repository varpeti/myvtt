#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use macroquad::prelude::*;

pub trait Event: Hash + Eq + Clone {}

#[derive(Debug)]
pub struct Events<E: Event> {
    pub input_map: HashMap<E, Vec<Vec<(EventS, EventT)>>>,
    events: HashSet<E>,
}

impl<E: Event> Default for Events<E> {
    fn default() -> Self {
        Self {
            input_map: HashMap::new(),
            events: HashSet::new(),
        }
    }
}

impl<E: Event, const N: usize> From<[(E, Vec<Vec<(EventS, EventT)>>); N]> for Events<E> {
    fn from(input_map: [(E, Vec<Vec<(EventS, EventT)>>); N]) -> Self {
        Self {
            input_map: HashMap::from(input_map),
            events: HashSet::default(),
        }
    }
}

impl<E: Event> Events<E> {
    pub fn update(&mut self) {
        self.events.clear();
        for (event, event_type_ors) in self.input_map.iter() {
            for event_type_ands in event_type_ors {
                let ok = event_type_ands.iter().all(|(es, et)| match et {
                    EventT::Keyboard(key_code) => match es {
                        EventS::JustPressed => is_key_pressed(*key_code),
                        EventS::IsPressed => is_key_down(*key_code),
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
                            EventS::IsPressed => is_mouse_button_down(button),
                            EventS::JustReleased => is_mouse_button_released(button),
                        }
                    }
                });
                if ok {
                    self.events.insert(event.clone());
                    break;
                }
            }
        }
    }

    pub fn contains(&self, event: &E) -> bool {
        self.events.contains(event)
    }

    pub fn pop(&mut self, event: &E) -> bool {
        self.events.remove(event)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventS {
    JustPressed,
    IsPressed,
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
