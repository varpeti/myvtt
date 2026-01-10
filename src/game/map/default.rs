use crate::game::{
    events::{EventS, EventT, MouseButton2},
    map::{BrushEvent, Map},
};

use std::collections::HashMap;

use hexx::HexLayout;
use macroquad::prelude::*;

use crate::game::{
    events::Events,
    map::tiles::{Tile, TileType},
};

impl Default for Map {
    fn default() -> Self {
        let hex_size = 32.;
        Self {
            hex_layout: HexLayout {
                orientation: hexx::HexOrientation::Pointy,
                origin: hexx::Vec2::ZERO,
                scale: hexx::Vec2::new(hex_size, hex_size),
            },
            hex_size,
            tiles: HashMap::new(),
            brush: Tile::new(TileType::Empty, 0),
            brush_size: 0,
            current_map_file: "assets/map/001".to_string(),

            brush_events: Events::from([
                (
                    BrushEvent::PickEmpty,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key1))]],
                ),
                (
                    BrushEvent::PickSmall,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key2))]],
                ),
                (
                    BrushEvent::PickHalf,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key3))]],
                ),
                (
                    BrushEvent::PickLarge,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key4))]],
                ),
                (
                    BrushEvent::PickFull,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Key5))]],
                ),
                (
                    BrushEvent::RotateClockwise,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))],
                    ],
                ),
                (
                    BrushEvent::RotateAntiClockwise,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))],
                    ],
                ),
                (
                    BrushEvent::SizeUp,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelUp))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::Q))],
                    ],
                ),
                (
                    BrushEvent::SizeDown,
                    vec![
                        vec![(EventS::JustPressed, EventT::Mouse(MouseButton2::WheelDown))],
                        vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::E))],
                    ],
                ),
                (
                    BrushEvent::CloneTile,
                    vec![vec![(
                        EventS::JustPressed,
                        EventT::Mouse(MouseButton2::MiddleClick),
                    )]],
                ),
                (
                    BrushEvent::Draw,
                    vec![vec![(
                        EventS::IsDown,
                        EventT::Mouse(MouseButton2::LeftClick),
                    )]],
                ),
                (
                    BrushEvent::Remove,
                    vec![vec![(
                        EventS::IsDown,
                        EventT::Mouse(MouseButton2::RightClick),
                    )]],
                ),
                (
                    BrushEvent::InsertWalls,
                    vec![vec![(EventS::JustPressed, EventT::Keyboard(KeyCode::I))]],
                ),
            ]),
        }
    }
}
