use std::{collections::HashMap, f32};

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
        let todo_remove = vec![
            Entity::new(Hex::new(0, 0), "FullTransparentGreen".to_string()),
            Entity::new(Hex::new(1, 0), "Token_Template.png".to_string()),
            Entity::new_with_size(Hex::new(3, -2), "dragon, amethyst.png".to_string(), 2.),
            Entity::new(Hex::new(3, 0), "bandit001.png".to_string()),
            Entity::new(Hex::new(4, 0), "bandit002.png".to_string()),
            Entity::new_with_size(Hex::new(4, 2), "red_dragon.png".to_string(), 3.),
            Entity::new(Hex::new(6, 0), "This will be an error circle!".to_string()),
        ];
        Self {
            entities: todo_remove,
            grabbed_entity: None,

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
