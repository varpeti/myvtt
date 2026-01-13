use std::{
    collections::{HashMap, HashSet},
    f32,
};

use hexx::Hex;
use macroquad::prelude::*;

use crate::game::{
    entities::{Entities, entity::Entity},
    events::{Event, EventS, EventT, Events, MouseButton2},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum EntityEvent {
    Drag,
    Drop,
    RotateClockwise,
    RotateAntiClockwise,
    SizeUp,
    SizeDown,
    DuplicateDrag,
    Remove,
    ToggleEntityEsp,
}

impl Event for EntityEvent {}

impl Default for Entities {
    fn default() -> Self {
        // TODO: Remove
        let todo_remove = HashMap::from([
            (
                Hex::new(0, 0),
                vec![Entity::new("FullTransparentGreen".to_string())],
            ),
            (
                Hex::new(0, 1),
                vec![Entity::new("Token_Template.png".to_string())],
            ),
            (
                Hex::new(0, 2),
                vec![Entity::new("dragon, amethyst.png".to_string())],
            ),
            (
                Hex::new(0, 3),
                vec![Entity::new("bandit001.png".to_string())],
            ),
            (
                Hex::new(0, 4),
                vec![Entity::new("bandit002.png".to_string())],
            ),
            (
                Hex::new(0, 4),
                vec![Entity::new_with_size("red_dragon.png".to_string(), 3.)],
            ),
        ]);
        Self {
            entities: todo_remove, //HashMap::new(),
            selected_entities: HashSet::new(),
            grabbed_entities: HashSet::new(),

            entity_esp: false,

            rotation_delta: f32::consts::FRAC_PI_6,
            size_gamma: 1.25,

            textures: HashMap::from([(
                "FullTransparentGreen".to_string(),
                Texture2D::from_image(&Image::gen_image_color(8, 8, GREEN.with_alpha(0.75))),
            )]),
            entity_events: Events::from([
                (
                    EntityEvent::Drag,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    EntityEvent::Drop,
                    vec![
                        vec![(EventS::JustReleased, EventT::Mouse(MouseButton2::LeftClick))],
                        vec![(
                            EventS::JustReleased,
                            EventT::Mouse(MouseButton2::MiddleClick),
                        )],
                    ],
                ),
                (
                    EntityEvent::SizeUp,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))]],
                ),
                (
                    EntityEvent::SizeDown,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))]],
                ),
                (
                    EntityEvent::RotateClockwise,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::WheelUp),
                    )]],
                ),
                (
                    EntityEvent::RotateAntiClockwise,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::WheelDown),
                    )]],
                ),
                (
                    EntityEvent::DuplicateDrag,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::MiddleClick),
                    )]],
                ),
                (
                    EntityEvent::Remove,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::RightClick),
                    )]],
                ),
                (
                    EntityEvent::ToggleEntityEsp,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Keyboard(KeyCode::Space),
                    )]],
                ),
            ]),
        }
    }
}
