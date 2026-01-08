use crate::game::{
    events::{EventS, EventT, MouseButton2},
    map::{BrushEvent, Map},
};

use std::collections::HashMap;

use hexx::{Hex, HexLayout};
use macroquad::prelude::*;

use crate::game::{
    events::Events,
    map::tiles::{Tile, TileType},
};

impl Default for Map {
    fn default() -> Self {
        let tiles = HashMap::from([
            (Hex::new(0, 0), Tile::new(TileType::Empty, 0)),
            (Hex::new(1, 0), Tile::new(TileType::Full, 0)),
            (Hex::new(-1, 2), Tile::new(TileType::Small, 0)),
            (Hex::new(0, 2), Tile::new(TileType::Small, 1)),
            (Hex::new(1, 2), Tile::new(TileType::Small, 2)),
            (Hex::new(2, 2), Tile::new(TileType::Small, 3)),
            (Hex::new(3, 2), Tile::new(TileType::Small, 4)),
            (Hex::new(4, 2), Tile::new(TileType::Small, 5)),
            (Hex::new(-2, 4), Tile::new(TileType::Half, 0)),
            (Hex::new(-1, 4), Tile::new(TileType::Half, 1)),
            (Hex::new(0, 4), Tile::new(TileType::Half, 2)),
            (Hex::new(1, 4), Tile::new(TileType::Half, 3)),
            (Hex::new(2, 4), Tile::new(TileType::Half, 4)),
            (Hex::new(3, 4), Tile::new(TileType::Half, 5)),
            (Hex::new(-3, 6), Tile::new(TileType::Large, 0)),
            (Hex::new(-2, 6), Tile::new(TileType::Large, 1)),
            (Hex::new(-1, 6), Tile::new(TileType::Large, 2)),
            (Hex::new(0, 6), Tile::new(TileType::Large, 3)),
            (Hex::new(1, 6), Tile::new(TileType::Large, 4)),
            (Hex::new(2, 6), Tile::new(TileType::Large, 5)),
        ]);

        let hex_size = 32.;
        Self {
            hex_layout: HexLayout {
                orientation: hexx::HexOrientation::Pointy,
                origin: hexx::Vec2::ZERO,
                scale: hexx::Vec2::new(hex_size, hex_size),
            },
            hex_size,
            tiles,
            brush: Tile::new(TileType::Empty, 0),
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
            ]),
        }
    }
}
